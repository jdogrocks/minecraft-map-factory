use serde::Deserialize;
use std::collections::HashMap;

/// A theme pack defines which entities spawn in which building contexts.
#[derive(Debug, Clone, Deserialize)]
pub struct ThemePack {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub description: String,
    pub rules: HashMap<String, ContextRule>,
}

/// A rule for a building context (e.g. "residential", "commercial").
#[derive(Debug, Clone, Deserialize)]
pub struct ContextRule {
    /// Entities that can spawn in this context, with relative weights.
    pub entities: Vec<EntityEntry>,
    /// Maximum number of entities per building floor.
    pub max_per_floor: u32,
}

/// A single entity type that can be placed.
#[derive(Debug, Clone, Deserialize)]
pub struct EntityEntry {
    /// Minecraft entity ID (e.g. "minecraft:cat")
    pub id: String,
    /// Relative spawn weight (higher = more common)
    pub weight: u32,
}

impl ThemePack {
    /// Select an entity for a given context using a deterministic seed.
    pub fn select_entity(&self, context: &str, seed: u64) -> Option<&EntityEntry> {
        let rule = self.rules.get(context)?;
        if rule.entities.is_empty() {
            return None;
        }
        let total_weight: u64 = rule.entities.iter().map(|e| e.weight as u64).sum();
        if total_weight == 0 {
            return None;
        }
        let roll = seed % total_weight;
        let mut cumulative = 0u64;
        for entry in &rule.entities {
            cumulative += entry.weight as u64;
            if roll < cumulative {
                return Some(entry);
            }
        }
        rule.entities.last()
    }

    /// Get the max entities per floor for a context.
    pub fn max_per_floor(&self, context: &str) -> u32 {
        self.rules
            .get(context)
            .map(|r| r.max_per_floor)
            .unwrap_or(0)
    }
}

/// Built-in default theme pack: realistic animals and villagers.
pub fn default_theme() -> ThemePack {
    let mut rules = HashMap::new();

    rules.insert(
        "residential".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:cat".to_string(),
                    weight: 40,
                },
                EntityEntry {
                    id: "minecraft:wolf".to_string(),
                    weight: 20,
                },
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 30,
                },
                EntityEntry {
                    id: "minecraft:parrot".to_string(),
                    weight: 10,
                },
            ],
            max_per_floor: 3,
        },
    );

    rules.insert(
        "commercial".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 60,
                },
                EntityEntry {
                    id: "minecraft:cat".to_string(),
                    weight: 20,
                },
                EntityEntry {
                    id: "minecraft:iron_golem".to_string(),
                    weight: 20,
                },
            ],
            max_per_floor: 4,
        },
    );

    rules.insert(
        "public".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 50,
                },
                EntityEntry {
                    id: "minecraft:iron_golem".to_string(),
                    weight: 30,
                },
                EntityEntry {
                    id: "minecraft:cat".to_string(),
                    weight: 20,
                },
            ],
            max_per_floor: 5,
        },
    );

    rules.insert(
        "farm".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:cow".to_string(),
                    weight: 25,
                },
                EntityEntry {
                    id: "minecraft:pig".to_string(),
                    weight: 25,
                },
                EntityEntry {
                    id: "minecraft:chicken".to_string(),
                    weight: 25,
                },
                EntityEntry {
                    id: "minecraft:sheep".to_string(),
                    weight: 15,
                },
                EntityEntry {
                    id: "minecraft:horse".to_string(),
                    weight: 10,
                },
            ],
            max_per_floor: 4,
        },
    );

    rules.insert(
        "religious".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 70,
                },
                EntityEntry {
                    id: "minecraft:cat".to_string(),
                    weight: 30,
                },
            ],
            max_per_floor: 2,
        },
    );

    rules.insert(
        "industrial".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:iron_golem".to_string(),
                    weight: 50,
                },
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 50,
                },
            ],
            max_per_floor: 2,
        },
    );

    ThemePack {
        name: "default".to_string(),
        description: "Realistic animals and villagers placed contextually in buildings".to_string(),
        rules,
    }
}

/// Built-in fantasy theme pack: magical and mythical creatures.
pub fn fantasy_theme() -> ThemePack {
    let mut rules = HashMap::new();

    rules.insert(
        "residential".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:allay".to_string(),
                    weight: 30,
                },
                EntityEntry {
                    id: "minecraft:cat".to_string(),
                    weight: 30,
                },
                EntityEntry {
                    id: "minecraft:fox".to_string(),
                    weight: 20,
                },
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 20,
                },
            ],
            max_per_floor: 3,
        },
    );

    rules.insert(
        "commercial".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:wandering_trader".to_string(),
                    weight: 40,
                },
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 30,
                },
                EntityEntry {
                    id: "minecraft:allay".to_string(),
                    weight: 30,
                },
            ],
            max_per_floor: 4,
        },
    );

    rules.insert(
        "public".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:iron_golem".to_string(),
                    weight: 30,
                },
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 30,
                },
                EntityEntry {
                    id: "minecraft:allay".to_string(),
                    weight: 20,
                },
                EntityEntry {
                    id: "minecraft:snow_golem".to_string(),
                    weight: 20,
                },
            ],
            max_per_floor: 5,
        },
    );

    rules.insert(
        "farm".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:mooshroom".to_string(),
                    weight: 25,
                },
                EntityEntry {
                    id: "minecraft:bee".to_string(),
                    weight: 25,
                },
                EntityEntry {
                    id: "minecraft:fox".to_string(),
                    weight: 25,
                },
                EntityEntry {
                    id: "minecraft:rabbit".to_string(),
                    weight: 25,
                },
            ],
            max_per_floor: 4,
        },
    );

    rules.insert(
        "religious".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:allay".to_string(),
                    weight: 50,
                },
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 30,
                },
                EntityEntry {
                    id: "minecraft:snow_golem".to_string(),
                    weight: 20,
                },
            ],
            max_per_floor: 3,
        },
    );

    rules.insert(
        "industrial".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:iron_golem".to_string(),
                    weight: 60,
                },
                EntityEntry {
                    id: "minecraft:snow_golem".to_string(),
                    weight: 40,
                },
            ],
            max_per_floor: 2,
        },
    );

    ThemePack {
        name: "fantasy".to_string(),
        description: "Magical and mythical creatures placed in buildings".to_string(),
        rules,
    }
}

/// Built-in urban_dense theme pack: city residents with no livestock.
pub fn urban_dense_theme() -> ThemePack {
    let mut rules = HashMap::new();

    rules.insert(
        "residential".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 60,
                },
                EntityEntry {
                    id: "minecraft:cat".to_string(),
                    weight: 30,
                },
                EntityEntry {
                    id: "minecraft:parrot".to_string(),
                    weight: 10,
                },
            ],
            max_per_floor: 4,
        },
    );

    rules.insert(
        "commercial".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 70,
                },
                EntityEntry {
                    id: "minecraft:iron_golem".to_string(),
                    weight: 20,
                },
                EntityEntry {
                    id: "minecraft:cat".to_string(),
                    weight: 10,
                },
            ],
            max_per_floor: 6,
        },
    );

    rules.insert(
        "public".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 70,
                },
                EntityEntry {
                    id: "minecraft:iron_golem".to_string(),
                    weight: 20,
                },
                EntityEntry {
                    id: "minecraft:cat".to_string(),
                    weight: 10,
                },
            ],
            max_per_floor: 8,
        },
    );

    rules.insert(
        "farm".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 80,
                },
                EntityEntry {
                    id: "minecraft:bee".to_string(),
                    weight: 20,
                },
            ],
            max_per_floor: 2,
        },
    );

    rules.insert(
        "religious".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 80,
                },
                EntityEntry {
                    id: "minecraft:cat".to_string(),
                    weight: 20,
                },
            ],
            max_per_floor: 3,
        },
    );

    rules.insert(
        "industrial".to_string(),
        ContextRule {
            entities: vec![
                EntityEntry {
                    id: "minecraft:iron_golem".to_string(),
                    weight: 60,
                },
                EntityEntry {
                    id: "minecraft:villager".to_string(),
                    weight: 40,
                },
            ],
            max_per_floor: 3,
        },
    );

    ThemePack {
        name: "urban_dense".to_string(),
        description: "City residents — villagers and iron golems, no livestock".to_string(),
        rules,
    }
}

/// Load a theme pack by name. Returns the built-in pack or None.
pub fn load_theme(name: &str) -> Option<ThemePack> {
    match name {
        "default" => Some(default_theme()),
        "fantasy" => Some(fantasy_theme()),
        "urban_dense" => Some(urban_dense_theme()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme_has_all_contexts() {
        let theme = default_theme();
        assert!(theme.rules.contains_key("residential"));
        assert!(theme.rules.contains_key("commercial"));
        assert!(theme.rules.contains_key("public"));
        assert!(theme.rules.contains_key("farm"));
        assert!(theme.rules.contains_key("religious"));
        assert!(theme.rules.contains_key("industrial"));
    }

    #[test]
    fn test_fantasy_theme_has_all_contexts() {
        let theme = fantasy_theme();
        assert!(theme.rules.contains_key("residential"));
        assert!(theme.rules.contains_key("commercial"));
        assert!(theme.rules.contains_key("public"));
        assert!(theme.rules.contains_key("farm"));
    }

    #[test]
    fn test_select_entity_deterministic() {
        let theme = default_theme();
        let e1 = theme.select_entity("residential", 42);
        let e2 = theme.select_entity("residential", 42);
        assert!(e1.is_some());
        assert_eq!(e1.unwrap().id, e2.unwrap().id);
    }

    #[test]
    fn test_select_entity_unknown_context() {
        let theme = default_theme();
        assert!(theme.select_entity("nonexistent", 42).is_none());
    }

    #[test]
    fn test_max_per_floor() {
        let theme = default_theme();
        assert_eq!(theme.max_per_floor("residential"), 3);
        assert_eq!(theme.max_per_floor("nonexistent"), 0);
    }

    #[test]
    fn test_load_theme() {
        assert!(load_theme("default").is_some());
        assert!(load_theme("fantasy").is_some());
        assert!(load_theme("urban_dense").is_some());
        assert!(load_theme("nonexistent").is_none());
    }

    #[test]
    fn test_urban_dense_theme_has_all_contexts() {
        let theme = urban_dense_theme();
        assert!(theme.rules.contains_key("residential"));
        assert!(theme.rules.contains_key("commercial"));
        assert!(theme.rules.contains_key("public"));
        assert!(theme.rules.contains_key("farm"));
        assert!(theme.rules.contains_key("religious"));
        assert!(theme.rules.contains_key("industrial"));
    }

    #[test]
    fn test_urban_dense_theme_no_livestock() {
        let theme = urban_dense_theme();
        let livestock = ["minecraft:cow", "minecraft:pig", "minecraft:chicken", "minecraft:sheep", "minecraft:horse"];
        for (_, rule) in &theme.rules {
            for entry in &rule.entities {
                assert!(
                    !livestock.contains(&entry.id.as_str()),
                    "urban_dense theme should not contain livestock: {}",
                    entry.id
                );
            }
        }
    }
}
