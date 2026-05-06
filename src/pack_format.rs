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

/// Minecraft Java Edition chunk DataVersion table.
///
/// Each entry maps a Minecraft release string to its DataVersion integer.
/// This value is stamped into every chunk's NBT so the game can determine
/// whether a world needs format upgrading.
/// Source: <https://minecraft.wiki/w/Data_version>
const DATA_VERSION_TABLE: &[(&str, i32)] = &[
    ("1.21.4", 4082),
    ("1.21.5", 4189),
    ("1.22", 4350),
    ("1.22.1", 4389),
    ("1.23", 4550),
    ("1.24", 4720),
    ("1.25", 4890),
    ("1.26", 5060),
    ("1.26.1", 5095),
    ("1.26.1.2", 5097),
];

/// Look up the chunk `DataVersion` integer for the given Minecraft version string.
///
/// Returns `None` if the version is not in the table.
pub fn data_version_for(mc_version: &str) -> Option<i32> {
    DATA_VERSION_TABLE
        .iter()
        .find(|(v, _)| *v == mc_version)
        .map(|(_, dv)| *dv)
}

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
/// Derives the `supported_formats` range from `DATAPACK_FORMAT_TABLE` so the
/// output stays in sync with the table automatically.  Falls back to
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
    "pack_format": {max},
    "description": "{description}",
    "supported_formats": {{
      "min_inclusive": {min},
      "max_inclusive": {max}
    }}
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
        assert!(json.contains(&format!("\"pack_format\": {}", PACK_FORMAT_MAX)));
        assert!(json.contains(&format!("\"min_inclusive\": {}", PACK_FORMAT_MIN)));
        assert!(json.contains(&format!("\"max_inclusive\": {}", PACK_FORMAT_MAX)));
        assert!(json.contains("\"test pack\""));
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
    fn test_data_version_known_versions() {
        assert_eq!(data_version_for("1.21.4"), Some(4082));
        assert_eq!(data_version_for("1.26.1.2"), Some(5097));
    }

    #[test]
    fn test_data_version_unknown_returns_none() {
        assert_eq!(data_version_for("1.99.0"), None);
        assert_eq!(data_version_for(""), None);
    }

    #[test]
    fn test_data_version_table_is_ascending() {
        let versions: Vec<i32> = DATA_VERSION_TABLE.iter().map(|(_, dv)| *dv).collect();
        for window in versions.windows(2) {
            assert!(
                window[0] < window[1],
                "DATA_VERSION_TABLE is not strictly ascending: {} >= {}",
                window[0],
                window[1]
            );
        }
    }

    #[test]
    fn test_data_version_and_pack_format_tables_same_versions() {
        let pack_versions: Vec<&str> = DATAPACK_FORMAT_TABLE.iter().map(|(v, _)| *v).collect();
        let dv_versions: Vec<&str> = DATA_VERSION_TABLE.iter().map(|(v, _)| *v).collect();
        assert_eq!(
            pack_versions, dv_versions,
            "DATAPACK_FORMAT_TABLE and DATA_VERSION_TABLE must cover the same version strings"
        );
    }

    #[test]
    fn test_generate_pack_mcmeta_is_valid_json() {
        let json = generate_pack_mcmeta("MMF extended build height");
        serde_json::from_str::<serde_json::Value>(&json)
            .expect("generated pack.mcmeta is not valid JSON");
    }
}
