use crate::clipping::clip_way_to_bbox;
use crate::coordinate_system::cartesian::{XZBBox, XZPoint};
use crate::coordinate_system::geographic::{LLBBox, LLPoint};
use crate::coordinate_system::transformation::CoordTransformer;
use crate::progress::emit_gui_progress_update;
use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

// Raw data from OSM

#[derive(Debug, Deserialize)]
struct OsmMember {
    r#type: String,
    r#ref: u64,
    r#role: String,
}

#[derive(Debug, Deserialize)]
struct OsmElement {
    pub r#type: String,
    pub id: u64,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub nodes: Option<Vec<u64>>,
    pub tags: Option<HashMap<String, String>>,
    #[serde(default)]
    pub members: Vec<OsmMember>,
}

#[derive(Debug, Deserialize)]
pub struct OsmData {
    elements: Vec<OsmElement>,
    #[serde(default)]
    pub remark: Option<String>,
}

impl OsmData {
    /// Returns true if there are no elements in the OSM data
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

struct SplitOsmData {
    pub nodes: Vec<OsmElement>,
    pub ways: Vec<OsmElement>,
    pub relations: Vec<OsmElement>,
    #[allow(dead_code)]
    pub others: Vec<OsmElement>,
}

impl SplitOsmData {
    fn total_count(&self) -> usize {
        self.nodes.len() + self.ways.len() + self.relations.len() + self.others.len()
    }
    fn from_raw_osm_data(osm_data: OsmData) -> Self {
        let mut nodes = Vec::new();
        let mut ways = Vec::new();
        let mut relations = Vec::new();
        let mut others = Vec::new();
        for element in osm_data.elements {
            match element.r#type.as_str() {
                "node" => nodes.push(element),
                "way" => ways.push(element),
                "relation" => relations.push(element),
                _ => others.push(element),
            }
        }
        SplitOsmData {
            nodes,
            ways,
            relations,
            others,
        }
    }
}

// End raw data

// Normalized data that we can use

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessedNode {
    pub id: u64,
    pub tags: HashMap<String, String>,

    // Minecraft coordinates
    pub x: i32,
    pub z: i32,
}

impl ProcessedNode {
    pub fn xz(&self) -> XZPoint {
        XZPoint {
            x: self.x,
            z: self.z,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessedWay {
    pub id: u64,
    pub nodes: Vec<ProcessedNode>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProcessedMemberRole {
    Outer,
    Inner,
    Part,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessedMember {
    pub role: ProcessedMemberRole,
    pub way: Arc<ProcessedWay>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessedRelation {
    pub id: u64,
    pub tags: HashMap<String, String>,
    pub members: Vec<ProcessedMember>,
}

#[derive(Debug, Clone)]
pub enum ProcessedElement {
    Node(ProcessedNode),
    Way(ProcessedWay),
    Relation(ProcessedRelation),
}

impl ProcessedElement {
    pub fn tags(&self) -> &HashMap<String, String> {
        match self {
            ProcessedElement::Node(n) => &n.tags,
            ProcessedElement::Way(w) => &w.tags,
            ProcessedElement::Relation(r) => &r.tags,
        }
    }

    pub fn id(&self) -> u64 {
        match self {
            ProcessedElement::Node(n) => n.id,
            ProcessedElement::Way(w) => w.id,
            ProcessedElement::Relation(r) => r.id,
        }
    }

    pub fn kind(&self) -> &str {
        match self {
            ProcessedElement::Node(_) => "node",
            ProcessedElement::Way(_) => "way",
            ProcessedElement::Relation(_) => "relation",
        }
    }

    pub fn nodes<'a>(&'a self) -> Box<dyn Iterator<Item = &'a ProcessedNode> + 'a> {
        match self {
            ProcessedElement::Node(node) => Box::new([node].into_iter()),
            ProcessedElement::Way(way) => Box::new(way.nodes.iter()),
            ProcessedElement::Relation(_) => Box::new([].into_iter()),
        }
    }
}

pub fn parse_osm_data(
    osm_data: OsmData,
    bbox: LLBBox,
    scale: f64,
    debug: bool,
) -> (Vec<ProcessedElement>, XZBBox) {
    println!("{} Parsing data...", "[2/7]".bold());
    println!("Bounding box: {bbox:?}");
    emit_gui_progress_update(5.0, "Parsing data...");

    // Deserialize the JSON data into the OSMData structure
    let data = SplitOsmData::from_raw_osm_data(osm_data);

    let (coord_transformer, xzbbox) = CoordTransformer::llbbox_to_xzbbox(&bbox, scale)
        .unwrap_or_else(|e| {
            eprintln!("Error in defining coordinate transformation:\n{e}");
            panic!();
        });

    if debug {
        println!("Total elements: {}", data.total_count());
        println!("Scale factor X: {}", coord_transformer.scale_factor_x());
        println!("Scale factor Z: {}", coord_transformer.scale_factor_z());
    }

    let mut nodes_map: HashMap<u64, ProcessedNode> = HashMap::new();
    let mut ways_map: HashMap<u64, Arc<ProcessedWay>> = HashMap::new();

    let mut processed_elements: Vec<ProcessedElement> = Vec::new();

    // First pass: store all nodes with Minecraft coordinates and process nodes with tags
    for element in data.nodes {
        if let (Some(lat), Some(lon)) = (element.lat, element.lon) {
            let llpoint = LLPoint::new(lat, lon).unwrap_or_else(|e| {
                eprintln!("Encountered invalid node element:\n{e}");
                panic!();
            });

            let xzpoint = coord_transformer.transform_point(llpoint);

            let processed: ProcessedNode = ProcessedNode {
                id: element.id,
                tags: element.tags.clone().unwrap_or_default(),
                x: xzpoint.x,
                z: xzpoint.z,
            };

            nodes_map.insert(element.id, processed.clone());

            // Only add tagged nodes to processed_elements if they're within or near the bbox
            // This significantly improves performance by filtering out distant nodes
            if !element.tags.as_ref().map(|t| t.is_empty()).unwrap_or(true) {
                // Node has tags, check if it's in the bbox (with some margin)
                if xzbbox.contains(&xzpoint) {
                    processed_elements.push(ProcessedElement::Node(processed));
                }
            }
        }
    }

    // Second pass: process ways and clip them to bbox
    for element in data.ways {
        let mut nodes: Vec<ProcessedNode> = vec![];
        if let Some(node_ids) = &element.nodes {
            for &node_id in node_ids {
                if let Some(node) = nodes_map.get(&node_id) {
                    nodes.push(node.clone());
                }
            }
        }

        // Clip the way to bbox to reduce node count dramatically
        let tags = element.tags.clone().unwrap_or_default();

        // Store unclipped way for relation assembly (clipping happens after ring merging)
        let way = Arc::new(ProcessedWay {
            id: element.id,
            tags,
            nodes,
        });
        ways_map.insert(element.id, Arc::clone(&way));

        // Clip way nodes for standalone way processing (not relations)
        let clipped_nodes = clip_way_to_bbox(&way.nodes, &xzbbox);

        // Skip ways that are completely outside the bbox (empty after clipping)
        if clipped_nodes.is_empty() {
            continue;
        }

        let processed: ProcessedWay = ProcessedWay {
            id: element.id,
            tags: way.tags.clone(),
            nodes: clipped_nodes,
        };

        processed_elements.push(ProcessedElement::Way(processed));
    }

    // Third pass: process relations and clip member ways
    for element in data.relations {
        let Some(tags) = &element.tags else {
            continue;
        };

        // Process multipolygons and building relations
        let relation_type = tags.get("type").map(|x: &String| x.as_str());
        if relation_type != Some("multipolygon") && relation_type != Some("building") {
            continue;
        };

        let is_building_relation = relation_type == Some("building")
            || tags.contains_key("building")
            || tags.contains_key("building:part");

        // Water relations require unclipped ways for ring merging in water_areas.rs
        // Building multipolygon relations also need unclipped ways so that
        // open outer-way segments can be merged into closed rings before clipping
        let is_water_relation = is_water_element(tags);
        let is_building_multipolygon = (tags.contains_key("building")
            || tags.contains_key("building:part"))
            && relation_type == Some("multipolygon");
        let keep_unclipped = is_water_relation || is_building_multipolygon;

        let members: Vec<ProcessedMember> = element
            .members
            .iter()
            .filter_map(|mem: &OsmMember| {
                if mem.r#type != "way" {
                    eprintln!("WARN: Unknown relation member type \"{}\"", mem.r#type);
                    return None;
                }

                let trimmed_role = mem.role.trim();
                let role = if trimmed_role.eq_ignore_ascii_case("outer")
                    || trimmed_role.eq_ignore_ascii_case("outline")
                {
                    ProcessedMemberRole::Outer
                } else if trimmed_role.eq_ignore_ascii_case("inner") {
                    ProcessedMemberRole::Inner
                } else if trimmed_role.eq_ignore_ascii_case("part") {
                    if relation_type == Some("building") {
                        // "part" role only applies to type=building relations.
                        ProcessedMemberRole::Part
                    } else {
                        // For multipolygon relations, "part" is not a valid role, skip.
                        return None;
                    }
                } else if is_building_relation {
                    ProcessedMemberRole::Outer
                } else {
                    return None;
                };

                // Check if the way exists in ways_map
                let way = match ways_map.get(&mem.r#ref) {
                    Some(w) => Arc::clone(w),
                    None => {
                        // Way was likely filtered out because it was completely outside the bbox
                        return None;
                    }
                };

                // If keep_unclipped is true (e.g., certain water or building multipolygon
                // relations), keep member ways unclipped for ring merging; otherwise clip now.
                let final_way = if keep_unclipped {
                    way
                } else {
                    let clipped_nodes = clip_way_to_bbox(&way.nodes, &xzbbox);
                    if clipped_nodes.is_empty() {
                        return None;
                    }
                    Arc::new(ProcessedWay {
                        id: way.id,
                        tags: way.tags.clone(),
                        nodes: clipped_nodes,
                    })
                };

                Some(ProcessedMember {
                    role,
                    way: final_way,
                })
            })
            .collect();

        if !members.is_empty() {
            processed_elements.push(ProcessedElement::Relation(ProcessedRelation {
                id: element.id,
                members,
                tags: tags.clone(),
            }));
        }
    }

    emit_gui_progress_update(14.0, "");

    drop(nodes_map);
    drop(ways_map);

    (processed_elements, xzbbox)
}

/// Returns true if tags indicate a water element handled by water_areas.rs.
fn is_water_element(tags: &HashMap<String, String>) -> bool {
    // Check for explicit water tag
    if tags.contains_key("water") {
        return true;
    }

    // Check for natural=water or natural=bay
    if let Some(natural_val) = tags.get("natural") {
        if natural_val == "water" || natural_val == "bay" {
            return true;
        }
    }

    // Check for waterway=dock (also handled as water area)
    if let Some(waterway_val) = tags.get("waterway") {
        if waterway_val == "dock" {
            return true;
        }
    }

    false
}

const PRIORITY_ORDER: [&str; 6] = [
    "entrance", "building", "highway", "waterway", "water", "barrier",
];

// Function to determine the priority of each element
pub fn get_priority(element: &ProcessedElement) -> usize {
    // Check each tag against the priority order
    for (i, &tag) in PRIORITY_ORDER.iter().enumerate() {
        if element.tags().contains_key(tag) {
            return i;
        }
    }
    // Return a default priority if none of the tags match
    PRIORITY_ORDER.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // ── Helper constructors ─────────────────────────────────────────

    fn tags(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    fn node(id: u64, x: i32, z: i32, tag_pairs: &[(&str, &str)]) -> ProcessedNode {
        ProcessedNode {
            id,
            tags: tags(tag_pairs),
            x,
            z,
        }
    }

    fn way(id: u64, nodes: Vec<ProcessedNode>, tag_pairs: &[(&str, &str)]) -> ProcessedWay {
        ProcessedWay {
            id,
            nodes,
            tags: tags(tag_pairs),
        }
    }

    fn relation(
        id: u64,
        members: Vec<ProcessedMember>,
        tag_pairs: &[(&str, &str)],
    ) -> ProcessedRelation {
        ProcessedRelation {
            id,
            tags: tags(tag_pairs),
            members,
        }
    }

    // ── OsmData ─────────────────────────────────────────────────────

    #[test]
    fn osm_data_is_empty_on_no_elements() {
        let data: OsmData = serde_json::from_str(r#"{"elements":[]}"#).unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn osm_data_is_not_empty_with_elements() {
        let json = r#"{"elements":[{"type":"node","id":1,"lat":54.63,"lon":9.93}]}"#;
        let data: OsmData = serde_json::from_str(json).unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    fn osm_data_remark_field_optional() {
        let data: OsmData = serde_json::from_str(r#"{"elements":[]}"#).unwrap();
        assert!(data.remark.is_none());

        let data: OsmData =
            serde_json::from_str(r#"{"elements":[],"remark":"rate limit"}"#).unwrap();
        assert_eq!(data.remark.as_deref(), Some("rate limit"));
    }

    // ── SplitOsmData ────────────────────────────────────────────────

    #[test]
    fn split_osm_data_classifies_elements_correctly() {
        let json = r#"{"elements":[
            {"type":"node","id":1,"lat":54.63,"lon":9.93},
            {"type":"node","id":2,"lat":54.64,"lon":9.94},
            {"type":"way","id":10,"nodes":[1,2]},
            {"type":"relation","id":100,"members":[]},
            {"type":"area","id":200}
        ]}"#;
        let data: OsmData = serde_json::from_str(json).unwrap();
        let split = SplitOsmData::from_raw_osm_data(data);

        assert_eq!(split.nodes.len(), 2);
        assert_eq!(split.ways.len(), 1);
        assert_eq!(split.relations.len(), 1);
        assert_eq!(split.others.len(), 1);
        assert_eq!(split.total_count(), 5);
    }

    #[test]
    fn split_osm_data_empty_input() {
        let data: OsmData = serde_json::from_str(r#"{"elements":[]}"#).unwrap();
        let split = SplitOsmData::from_raw_osm_data(data);
        assert_eq!(split.total_count(), 0);
    }

    // ── ProcessedNode ───────────────────────────────────────────────

    #[test]
    fn processed_node_xz_returns_correct_point() {
        let n = node(1, 42, -7, &[]);
        let pt = n.xz();
        assert_eq!(pt.x, 42);
        assert_eq!(pt.z, -7);
    }

    // ── ProcessedElement accessors ──────────────────────────────────

    #[test]
    fn element_tags_returns_correct_map() {
        let elem = ProcessedElement::Node(node(1, 0, 0, &[("building", "yes")]));
        assert_eq!(elem.tags().get("building").unwrap(), "yes");
    }

    #[test]
    fn element_id_returns_correct_id() {
        let elem = ProcessedElement::Node(node(42, 0, 0, &[]));
        assert_eq!(elem.id(), 42);

        let w = way(99, vec![], &[]);
        assert_eq!(ProcessedElement::Way(w).id(), 99);

        let r = relation(200, vec![], &[]);
        assert_eq!(ProcessedElement::Relation(r).id(), 200);
    }

    #[test]
    fn element_kind_returns_correct_string() {
        assert_eq!(ProcessedElement::Node(node(1, 0, 0, &[])).kind(), "node");
        assert_eq!(ProcessedElement::Way(way(1, vec![], &[])).kind(), "way");
        assert_eq!(
            ProcessedElement::Relation(relation(1, vec![], &[])).kind(),
            "relation"
        );
    }

    #[test]
    fn element_nodes_iterator_for_node() {
        let n = node(1, 10, 20, &[]);
        let elem = ProcessedElement::Node(n);
        let nodes: Vec<_> = elem.nodes().collect();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].x, 10);
    }

    #[test]
    fn element_nodes_iterator_for_way() {
        let w = way(1, vec![node(1, 0, 0, &[]), node(2, 5, 5, &[])], &[]);
        let elem = ProcessedElement::Way(w);
        let nodes: Vec<_> = elem.nodes().collect();
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn element_nodes_iterator_for_relation_is_empty() {
        let r = relation(1, vec![], &[]);
        let elem = ProcessedElement::Relation(r);
        assert_eq!(elem.nodes().count(), 0);
    }

    // ── get_priority ────────────────────────────────────────────────

    #[test]
    fn priority_entrance_is_highest() {
        let elem = ProcessedElement::Node(node(1, 0, 0, &[("entrance", "main")]));
        assert_eq!(get_priority(&elem), 0);
    }

    #[test]
    fn priority_building_is_second() {
        let elem = ProcessedElement::Node(node(1, 0, 0, &[("building", "yes")]));
        assert_eq!(get_priority(&elem), 1);
    }

    #[test]
    fn priority_highway() {
        let elem = ProcessedElement::Way(way(1, vec![], &[("highway", "residential")]));
        assert_eq!(get_priority(&elem), 2);
    }

    #[test]
    fn priority_waterway() {
        let elem = ProcessedElement::Way(way(1, vec![], &[("waterway", "river")]));
        assert_eq!(get_priority(&elem), 3);
    }

    #[test]
    fn priority_water() {
        let elem = ProcessedElement::Way(way(1, vec![], &[("water", "lake")]));
        assert_eq!(get_priority(&elem), 4);
    }

    #[test]
    fn priority_barrier() {
        let elem = ProcessedElement::Way(way(1, vec![], &[("barrier", "fence")]));
        assert_eq!(get_priority(&elem), 5);
    }

    #[test]
    fn priority_unknown_tag_returns_default() {
        let elem = ProcessedElement::Node(node(1, 0, 0, &[("amenity", "cafe")]));
        assert_eq!(get_priority(&elem), PRIORITY_ORDER.len());
    }

    #[test]
    fn priority_no_tags_returns_default() {
        let elem = ProcessedElement::Node(node(1, 0, 0, &[]));
        assert_eq!(get_priority(&elem), PRIORITY_ORDER.len());
    }

    #[test]
    fn priority_multiple_tags_uses_first_match() {
        // "building" (idx 1) should win over "barrier" (idx 5)
        let elem =
            ProcessedElement::Node(node(1, 0, 0, &[("building", "yes"), ("barrier", "wall")]));
        let p = get_priority(&elem);
        assert!(p <= 1); // building or entrance
    }

    // ── is_water_element ────────────────────────────────────────────

    #[test]
    fn water_element_with_water_tag() {
        assert!(is_water_element(&tags(&[("water", "lake")])));
    }

    #[test]
    fn water_element_natural_water() {
        assert!(is_water_element(&tags(&[("natural", "water")])));
    }

    #[test]
    fn water_element_natural_bay() {
        assert!(is_water_element(&tags(&[("natural", "bay")])));
    }

    #[test]
    fn water_element_waterway_dock() {
        assert!(is_water_element(&tags(&[("waterway", "dock")])));
    }

    #[test]
    fn not_water_element_waterway_river() {
        assert!(!is_water_element(&tags(&[("waterway", "river")])));
    }

    #[test]
    fn not_water_element_natural_wood() {
        assert!(!is_water_element(&tags(&[("natural", "wood")])));
    }

    #[test]
    fn not_water_element_empty_tags() {
        assert!(!is_water_element(&tags(&[])));
    }

    #[test]
    fn not_water_element_unrelated_tags() {
        assert!(!is_water_element(&tags(&[
            ("building", "yes"),
            ("highway", "primary"),
        ])));
    }

    // ── parse_osm_data (integration-style, no network) ──────────────

    #[test]
    fn parse_osm_data_processes_nodes_ways_relations() {
        // Build minimal OSM data with known lat/lon inside Arnis bbox
        let json = r#"{"elements":[
            {"type":"node","id":1,"lat":54.630,"lon":9.930},
            {"type":"node","id":2,"lat":54.631,"lon":9.932},
            {"type":"node","id":3,"lat":54.632,"lon":9.934,"tags":{"amenity":"bench"}},
            {"type":"way","id":10,"nodes":[1,2,3],"tags":{"highway":"footway"}},
            {"type":"relation","id":100,"members":[{"type":"way","ref":10,"role":"outer"}],"tags":{"type":"multipolygon","natural":"water"}}
        ]}"#;
        let osm_data: OsmData = serde_json::from_str(json).unwrap();
        let bbox = LLBBox::new(54.627, 9.927, 54.635, 9.938).unwrap();

        let (elements, xzbbox) = parse_osm_data(osm_data, bbox, 1.0, false);

        // Should have at least a way and possibly a tagged node and relation
        assert!(!elements.is_empty());
        // Bounding box should be valid
        assert!(xzbbox.max_x() >= xzbbox.min_x());
        assert!(xzbbox.max_z() >= xzbbox.min_z());
    }

    #[test]
    fn parse_osm_data_empty_input() {
        let osm_data: OsmData = serde_json::from_str(r#"{"elements":[]}"#).unwrap();
        let bbox = LLBBox::new(54.627, 9.927, 54.635, 9.938).unwrap();
        let (elements, _xzbbox) = parse_osm_data(osm_data, bbox, 1.0, false);
        assert!(elements.is_empty());
    }

    #[test]
    fn parse_osm_data_nodes_without_lat_lon_are_skipped() {
        // Node missing lat/lon should not panic or be included
        let json = r#"{"elements":[{"type":"node","id":1,"tags":{"amenity":"bench"}}]}"#;
        let osm_data: OsmData = serde_json::from_str(json).unwrap();
        let bbox = LLBBox::new(54.627, 9.927, 54.635, 9.938).unwrap();
        let (elements, _) = parse_osm_data(osm_data, bbox, 1.0, false);
        assert!(elements.is_empty());
    }

    #[test]
    fn parse_osm_data_relation_non_multipolygon_skipped() {
        let json = r#"{"elements":[
            {"type":"node","id":1,"lat":54.630,"lon":9.930},
            {"type":"node","id":2,"lat":54.631,"lon":9.932},
            {"type":"way","id":10,"nodes":[1,2]},
            {"type":"relation","id":100,"members":[{"type":"way","ref":10,"role":"outer"}],"tags":{"type":"route","route":"bus"}}
        ]}"#;
        let osm_data: OsmData = serde_json::from_str(json).unwrap();
        let bbox = LLBBox::new(54.627, 9.927, 54.635, 9.938).unwrap();
        let (elements, _) = parse_osm_data(osm_data, bbox, 1.0, false);
        // No relations should be in the output (route relations are ignored)
        assert!(elements.iter().all(|e| e.kind() != "relation"));
    }

    // ── ProcessedMemberRole ─────────────────────────────────────────

    #[test]
    fn member_role_variants_are_distinct() {
        assert_ne!(ProcessedMemberRole::Outer, ProcessedMemberRole::Inner);
        assert_ne!(ProcessedMemberRole::Inner, ProcessedMemberRole::Part);
        assert_ne!(ProcessedMemberRole::Outer, ProcessedMemberRole::Part);
    }

    // ── ProcessedMember / ProcessedRelation Clone + PartialEq ──────

    #[test]
    fn processed_relation_clone_and_eq() {
        let w = Arc::new(way(10, vec![node(1, 0, 0, &[])], &[]));
        let member = ProcessedMember {
            role: ProcessedMemberRole::Outer,
            way: Arc::clone(&w),
        };
        let rel = relation(100, vec![member], &[("type", "multipolygon")]);
        let rel2 = rel.clone();
        assert_eq!(rel, rel2);
    }
}
