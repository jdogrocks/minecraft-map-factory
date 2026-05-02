/// Minecraft Java Edition datapack format (pack_format) version table.
///
/// Each entry maps a Minecraft release string to its datapack pack_format.
/// Source: <https://minecraft.wiki/w/Data_pack#Pack_format>
///
/// When a new Minecraft version ships, append the new entry here and update
/// `pack.mcmeta` via `generate_pack_mcmeta`.
const DATAPACK_FORMAT_TABLE: &[(&str, u32)] = &[
    ("1.21.4", 61),
    ("1.21.5", 71),
    ("1.22", 80),
    ("1.22.1", 84),
    ("1.23", 92),
    ("1.24", 103),
    ("1.25", 113),
    ("1.26", 122),
    ("1.26.1", 124),
    ("1.26.1.2", 124),
];

/// Look up the datapack `pack_format` for the given Minecraft version string.
///
/// Returns `None` if the version is not in the table.
pub fn datapack_format_for(mc_version: &str) -> Option<u32> {
    DATAPACK_FORMAT_TABLE
        .iter()
        .find(|(v, _)| *v == mc_version)
        .map(|(_, f)| *f)
}

/// The oldest pack_format the bundled tall-world datapack supports.
///
/// Set to 61 (MC 1.21.4) — the version the datapack was first authored for.
pub const PACK_FORMAT_MIN: u32 = 61;

/// The newest pack_format the bundled tall-world datapack has been validated against.
pub const PACK_FORMAT_MAX: u32 = 124; // MC 1.26.1.2

/// Generate the JSON content for `pack.mcmeta`.
///
/// Uses the flat `min_format`/`max_format` schema introduced in MC 26.1.2,
/// which replaces the old `pack_format`/`supported_formats` fields.
/// Derives the range from `DATAPACK_FORMAT_TABLE` automatically. Falls back to
/// `PACK_FORMAT_MIN` / `PACK_FORMAT_MAX` if the table is somehow empty.
pub fn generate_pack_mcmeta(description: &str) -> String {
    let min = DATAPACK_FORMAT_TABLE
        .first()
        .and_then(|&(v, _)| datapack_format_for(v))
        .unwrap_or(PACK_FORMAT_MIN);
    let max = DATAPACK_FORMAT_TABLE
        .last()
        .and_then(|&(v, _)| datapack_format_for(v))
        .unwrap_or(PACK_FORMAT_MAX);
    format!(
        r#"{{
  "pack": {{
    "description": "{description}",
    "min_format": {min},
    "max_format": {max}
  }}
}}"#,
        min = min,
        max = max,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_versions() {
        assert_eq!(datapack_format_for("1.21.4"), Some(61));
        assert_eq!(datapack_format_for("1.21.5"), Some(71));
        assert_eq!(datapack_format_for("1.26.1.2"), Some(124));
    }

    #[test]
    fn test_unknown_version_returns_none() {
        assert_eq!(datapack_format_for("1.99.0"), None);
        assert_eq!(datapack_format_for(""), None);
    }

    #[test]
    fn test_table_is_ascending() {
        let formats: Vec<u32> = DATAPACK_FORMAT_TABLE.iter().map(|(_, f)| *f).collect();
        for window in formats.windows(2) {
            assert!(
                window[0] <= window[1],
                "pack_format table is not non-decreasing: {} > {}",
                window[0],
                window[1]
            );
        }
    }

    #[test]
    fn test_generate_pack_mcmeta_contains_correct_range() {
        let json = generate_pack_mcmeta("test pack");
        assert!(json.contains(&format!("\"min_format\": {}", PACK_FORMAT_MIN)));
        assert!(json.contains(&format!("\"max_format\": {}", PACK_FORMAT_MAX)));
        assert!(json.contains("\"test pack\""));
        assert!(!json.contains("pack_format"), "old pack_format key must not appear");
        assert!(!json.contains("supported_formats"), "old supported_formats key must not appear");
    }

    #[test]
    fn test_pack_format_max_matches_latest_entry() {
        let latest = DATAPACK_FORMAT_TABLE.last().map(|(_, f)| *f).unwrap_or(0);
        assert_eq!(
            PACK_FORMAT_MAX, latest,
            "PACK_FORMAT_MAX must match the last entry in DATAPACK_FORMAT_TABLE"
        );
    }

    #[test]
    fn test_generate_pack_mcmeta_is_valid_json() {
        let json = generate_pack_mcmeta("MMF extended build height");
        serde_json::from_str::<serde_json::Value>(&json)
            .expect("generated pack.mcmeta is not valid JSON");
    }
}
