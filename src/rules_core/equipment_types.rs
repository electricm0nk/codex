//! Equipment `TYPE:` tags per record identity, from `data/equipment_types.json`.
//!
//! Written by `gen_equipment_types` (crates/codex-ingest) from the pinned PCGen oracle. The
//! catalog's own category is four-way; these tags (`["Magic","Wand","Combat Gear"]`) are what let
//! the equipment picker offer real kinds. An identity absent here is simply untyped: callers show
//! it as uncategorized rather than guessing.

use std::collections::BTreeMap;

use crate::support::paths::repo_root;

/// Path of the sidecar under the data root (bundled as a resource in a packaged app).
pub const EQUIPMENT_TYPES_PATH: &str = "data/equipment_types.json";

/// The generated document's `types` object as `identity -> tags`. Missing or malformed input is an
/// error naming the problem, never an empty map standing in for "no data".
pub fn parse_equipment_types(json: &str) -> Result<BTreeMap<String, Vec<String>>, String> {
    let document: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("{EQUIPMENT_TYPES_PATH} is not valid JSON: {e}"))?;
    let types = document
        .get("types")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| format!("{EQUIPMENT_TYPES_PATH} has no `types` object"))?;
    types
        .iter()
        .map(|(key, tags)| {
            let tags = tags
                .as_array()
                .ok_or_else(|| format!("{EQUIPMENT_TYPES_PATH}: `{key}` is not an array of tags"))?
                .iter()
                .map(|tag| {
                    tag.as_str()
                        .map(str::to_string)
                        .ok_or_else(|| format!("{EQUIPMENT_TYPES_PATH}: `{key}` has a non-string tag"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok((key.clone(), tags))
        })
        .collect()
}

/// The sidecar, loaded once per process from [`repo_root`] (the packaged resource root in an
/// installed app).
pub fn equipment_types() -> &'static Result<BTreeMap<String, Vec<String>>, String> {
    static LOADED: std::sync::OnceLock<Result<BTreeMap<String, Vec<String>>, String>> = std::sync::OnceLock::new();
    LOADED.get_or_init(|| {
        let path = repo_root().join(EQUIPMENT_TYPES_PATH);
        let text = std::fs::read_to_string(&path).map_err(|e| format!("could not read {}: {e}", path.display()))?;
        parse_equipment_types(&text)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_generated_document_shape() {
        let json = r#"{"oracle_sha":"x","types":{"Longsword":["Weapon","Melee"],"Gem Of X":["Gem"]}}"#;
        let parsed = parse_equipment_types(json).unwrap();
        assert_eq!(parsed["Longsword"], vec!["Weapon".to_string(), "Melee".to_string()]);
        assert_eq!(parsed.len(), 2);
    }

    #[test]
    fn a_document_without_a_types_object_is_an_error_not_an_empty_map() {
        assert!(parse_equipment_types(r#"{"oracle_sha":"x"}"#).is_err());
        assert!(parse_equipment_types("not json").is_err());
    }

    #[test]
    fn the_committed_sidecar_loads_and_names_real_kinds() {
        let types = equipment_types().as_ref().expect("data/equipment_types.json loads");
        assert!(types.len() > 9000, "expected thousands of typed identities, got {}", types.len());
        assert_eq!(types["Staff of Scorching"][..2], ["Magic".to_string(), "Staff".to_string()]);
        assert_eq!(types["Longsword"][0], "Weapon");
    }
}
