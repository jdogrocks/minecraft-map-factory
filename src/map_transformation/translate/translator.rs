// use super::startend_translator::StartEndTranslator;
use super::startend_translator::StartEndTranslator;
use super::vector_translator::VectorTranslator;
use super::Operator;
use crate::coordinate_system::cartesian::{XZBBox, XZVector};
use crate::osm_parser::ProcessedElement;

/// Create a translate operator (translator) from json
pub fn translator_from_json(config: &serde_json::Value) -> Result<Box<dyn Operator>, String> {
    let type_str = config
        .get("type")
        .and_then(serde_json::Value::as_str)
        .ok_or("Expected a string field 'type' in an translator dict:\n{}".to_string())?;

    let translator_config = config
        .get("config")
        .ok_or("Expected a dict field 'config' in an translator dict")?;

    let translator_result: Result<Box<dyn Operator>, String> = match type_str {
        "vector" => {
            let upper_result: Result<Box<VectorTranslator>, _> =
                serde_json::from_value(translator_config.clone())
                    .map(Box::new)
                    .map_err(|e| e.to_string());
            upper_result.map(|o| o as Box<dyn Operator>)
        }
        "startend" => {
            let upper_result: Result<Box<StartEndTranslator>, _> =
                serde_json::from_value(translator_config.clone())
                    .map(Box::new)
                    .map_err(|e| e.to_string());
            upper_result.map(|o| o as Box<dyn Operator>)
        }
        _ => Err(format!("Unrecognized translator type '{type_str}'")),
    };

    translator_result.map_err(|e| format!("Translator config format error:\n{e}"))
}

/// Translate elements and bounding box by a vector
pub fn translate_by_vector(
    vector: XZVector,
    elements: &mut Vec<ProcessedElement>,
    xzbbox: &mut XZBBox,
) {
    *xzbbox += vector;

    for element in elements {
        match element {
            ProcessedElement::Node(n) => {
                n.x += vector.dx;
                n.z += vector.dz;
            }
            ProcessedElement::Way(w) => {
                for n in &mut w.nodes {
                    n.x += vector.dx;
                    n.z += vector.dz;
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordinate_system::cartesian::{XZPoint, XZVector};
    use crate::ground::Ground;
    use crate::osm_parser::{ProcessedNode, ProcessedWay};
    use crate::test_utilities::generate_default_example;
    use std::collections::HashMap;

    fn make_node(id: u64, x: i32, z: i32) -> ProcessedElement {
        ProcessedElement::Node(ProcessedNode {
            id,
            tags: HashMap::new(),
            x,
            z,
        })
    }

    fn make_way(id: u64, nodes: Vec<(i32, i32)>) -> ProcessedElement {
        ProcessedElement::Way(ProcessedWay {
            id,
            tags: HashMap::new(),
            nodes: nodes
                .into_iter()
                .enumerate()
                .map(|(i, (x, z))| ProcessedNode {
                    id: i as u64,
                    tags: HashMap::new(),
                    x,
                    z,
                })
                .collect(),
        })
    }

    // this ensures translate_by_vector function is correct
    #[test]
    fn test_translate_by_vector() {
        let dx: i32 = 123;
        let dz: i32 = -234;
        let vector = XZVector { dx, dz };

        let (xzbbox1, elements1) = generate_default_example();

        let mut xzbbox2 = xzbbox1.clone();
        let mut elements2 = elements1.clone();

        translate_by_vector(vector, &mut elements2, &mut xzbbox2);

        for (original, translated) in elements1.iter().zip(elements2.iter()) {
            match (original, translated) {
                (ProcessedElement::Node(a), ProcessedElement::Node(b)) => {
                    assert_eq!(a.id, b.id);
                    assert_eq!(a.tags, b.tags);
                    assert_eq!(b.x, a.x + dx);
                    assert_eq!(b.z, a.z + dz);
                }
                (ProcessedElement::Way(a), ProcessedElement::Way(b)) => {
                    assert_eq!(a.id, b.id);
                    assert_eq!(a.tags, b.tags);
                    for (nodea, nodeb) in a.nodes.iter().zip(b.nodes.iter()) {
                        assert_eq!(nodea.id, nodeb.id);
                        assert_eq!(nodea.tags, nodeb.tags);
                        assert_eq!(nodeb.x, nodea.x + dx);
                        assert_eq!(nodeb.z, nodea.z + dz);
                    }
                }
                (ProcessedElement::Relation(a), ProcessedElement::Relation(b)) => {
                    assert_eq!(a, b);
                }
                _ => {
                    panic!(
                        "Element type changed: original {} to {}",
                        original.kind(),
                        translated.kind()
                    );
                }
            }
        }
    }

    #[test]
    fn test_translate_by_zero_vector() {
        let vector = XZVector { dx: 0, dz: 0 };
        let mut elements = vec![make_node(1, 10, 20)];
        let mut xzbbox = XZBBox::rect_from_xz_lengths(50.0, 50.0).unwrap();
        let orig_min_x = xzbbox.min_x();

        translate_by_vector(vector, &mut elements, &mut xzbbox);

        if let ProcessedElement::Node(n) = &elements[0] {
            assert_eq!((n.x, n.z), (10, 20));
        }
        assert_eq!(xzbbox.min_x(), orig_min_x);
    }

    #[test]
    fn test_translate_by_negative_vector() {
        let vector = XZVector { dx: -5, dz: -10 };
        let mut elements = vec![make_node(1, 10, 20)];
        let mut xzbbox = XZBBox::rect_from_xz_lengths(50.0, 50.0).unwrap();

        translate_by_vector(vector, &mut elements, &mut xzbbox);

        if let ProcessedElement::Node(n) = &elements[0] {
            assert_eq!((n.x, n.z), (5, 10));
        }
    }

    #[test]
    fn test_translate_way_nodes() {
        let vector = XZVector { dx: 100, dz: 200 };
        let mut elements = vec![make_way(1, vec![(0, 0), (10, 10)])];
        let mut xzbbox = XZBBox::rect_from_xz_lengths(50.0, 50.0).unwrap();

        translate_by_vector(vector, &mut elements, &mut xzbbox);

        if let ProcessedElement::Way(w) = &elements[0] {
            assert_eq!((w.nodes[0].x, w.nodes[0].z), (100, 200));
            assert_eq!((w.nodes[1].x, w.nodes[1].z), (110, 210));
        }
    }

    #[test]
    fn test_translate_empty_elements() {
        let vector = XZVector { dx: 10, dz: 20 };
        let mut elements: Vec<ProcessedElement> = Vec::new();
        let mut xzbbox = XZBBox::rect_from_xz_lengths(50.0, 50.0).unwrap();
        let orig_min_x = xzbbox.min_x();

        translate_by_vector(vector, &mut elements, &mut xzbbox);

        assert!(elements.is_empty());
        assert_eq!(xzbbox.min_x(), orig_min_x + 10);
    }

    #[test]
    fn test_translator_from_json_vector() {
        let json: serde_json::Value =
            serde_json::from_str(r#"{"type":"vector","config":{"vector":{"dx":100,"dz":200}}}"#)
                .unwrap();
        let result = translator_from_json(&json);
        assert!(result.is_ok());
        assert!(result.unwrap().repr().contains("translate"));
    }

    #[test]
    fn test_translator_from_json_startend() {
        let json: serde_json::Value = serde_json::from_str(
            r#"{"type":"startend","config":{"start":{"x":0,"z":0},"end":{"x":10,"z":20}}}"#,
        )
        .unwrap();
        let result = translator_from_json(&json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_translator_from_json_unknown_type() {
        let json: serde_json::Value =
            serde_json::from_str(r#"{"type":"polar","config":{}}"#).unwrap();
        let result = translator_from_json(&json);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.contains("Unrecognized"));
    }

    #[test]
    fn test_translator_from_json_missing_type() {
        let json: serde_json::Value = serde_json::from_str(r#"{"config":{}}"#).unwrap();
        let result = translator_from_json(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_translator_from_json_missing_config() {
        let json: serde_json::Value = serde_json::from_str(r#"{"type":"vector"}"#).unwrap();
        let result = translator_from_json(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_translator_from_json_invalid_vector_config() {
        let json: serde_json::Value =
            serde_json::from_str(r#"{"type":"vector","config":{"wrong_field":42}}"#).unwrap();
        let result = translator_from_json(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_vector_translator_operate() {
        let vt = VectorTranslator {
            vector: XZVector { dx: 5, dz: 10 },
        };
        let mut elements = vec![make_node(1, 0, 0)];
        let mut xzbbox = XZBBox::rect_from_xz_lengths(50.0, 50.0).unwrap();
        let mut ground = Ground::new_flat(64);

        vt.operate(&mut elements, &mut xzbbox, &mut ground);

        if let ProcessedElement::Node(n) = &elements[0] {
            assert_eq!((n.x, n.z), (5, 10));
        }
    }

    #[test]
    fn test_vector_translator_repr() {
        let vt = VectorTranslator {
            vector: XZVector { dx: 100, dz: 200 },
        };
        let repr = vt.repr();
        assert!(repr.contains("translate"));
        assert!(repr.contains("diaplacement")); // preserves existing typo
    }

    #[test]
    fn test_startend_translator_operate() {
        let st = StartEndTranslator {
            start: XZPoint { x: 10, z: 20 },
            end: XZPoint { x: 30, z: 50 },
        };
        let mut elements = vec![make_node(1, 0, 0)];
        let mut xzbbox = XZBBox::rect_from_xz_lengths(50.0, 50.0).unwrap();
        let mut ground = Ground::new_flat(64);

        st.operate(&mut elements, &mut xzbbox, &mut ground);

        // Displacement = end - start = (20, 30)
        if let ProcessedElement::Node(n) = &elements[0] {
            assert_eq!((n.x, n.z), (20, 30));
        }
    }

    #[test]
    fn test_startend_translator_repr() {
        let st = StartEndTranslator {
            start: XZPoint { x: 0, z: 0 },
            end: XZPoint { x: 10, z: 20 },
        };
        let repr = st.repr();
        assert!(repr.contains("translate"));
    }
}
