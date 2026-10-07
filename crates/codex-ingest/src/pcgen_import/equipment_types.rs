//! Equipment `TYPE:` tags, read from PCGen `.lst` equipment files.
//!
//! The hand-authored and generated equipment tables carry only a four-way category
//! (`ArmsArmor`, `General`, `MagicItems`, `Equipmods`), so a catalog of thousands of items
//! could not be browsed by kind. PCGen's own `TYPE:` token has the real taxonomy
//! (`Magic.Wand.Combat Gear`, `Weapon.Melee.Martial`, `Gem`, `Goods.Trade`, ...). This module
//! reads it, keyed by the same record identity the equipment tables use, so a sidecar
//! (`data/equipment_types.json`, written by `gen_equipment_types`) can enrich catalog rows
//! without regenerating any table.
//!
//! Identity follows `gen_equipment_gap_tables`' own record predicate: skip comments,
//! ALL-CAPS directives, `.MOD` overlays and `CATEGORY:Internal` rows; a `.COPY=` row takes its
//! variant name; the identity is the `KEY:` token when present, else the declared name.

use std::collections::{BTreeMap, HashMap};

fn is_non_record_line(first: &str, fields: &[&str]) -> bool {
    if first.is_empty() || first.starts_with('#') {
        return true;
    }
    let is_directive = first
        .split_once(':')
        .map(|(head, _)| !head.is_empty() && head.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()))
        .unwrap_or(false);
    if is_directive && !first.starts_with("CLASS:") {
        return true;
    }
    if first.starts_with("CATEGORY=Internal|") || fields.iter().any(|f| f.trim() == "CATEGORY:Internal") {
        return true;
    }
    first.contains(".MOD")
}

fn token_value<'a>(fields: &[&'a str], token: &str) -> Option<&'a str> {
    fields.iter().find_map(|f| f.trim().strip_prefix(token))
}

/// Every `TYPE:` field on the row, unioned in declared order; `.CLEAR` is a directive, not a tag.
fn type_tags(fields: &[&str]) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    for field in fields {
        let Some(value) = field.trim().strip_prefix("TYPE:") else { continue };
        for tag in value.split('.').map(str::trim).filter(|t| !t.is_empty() && *t != "CLEAR") {
            if !tags.iter().any(|existing| existing == tag) {
                tags.push(tag.to_string());
            }
        }
    }
    tags
}

/// `(identity, tags)` for every record in one `.lst` text that carries at least one type tag.
pub fn parse_equipment_types(text: &str) -> Vec<(String, Vec<String>)> {
    // A `.COPY=<base>` row resolves `<base>` against a base row's `KEY:` when it has one, else its name.
    let mut base_tags: HashMap<String, Vec<String>> = HashMap::new();
    for line in text.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        let first = fields[0].trim();
        if is_non_record_line(first, &fields) || first.contains(".COPY=") {
            continue;
        }
        let tags = type_tags(&fields);
        if tags.is_empty() {
            continue;
        }
        base_tags.insert(first.to_string(), tags.clone());
        if let Some(key) = token_value(&fields, "KEY:") {
            base_tags.insert(key.to_string(), tags);
        }
    }

    let mut out = Vec::new();
    for line in text.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        let first = fields[0].trim();
        if is_non_record_line(first, &fields) {
            continue;
        }
        let (name, copy_base) = match first.split_once(".COPY=") {
            Some((base, variant)) => (variant.to_string(), Some(base)),
            None => (first.to_string(), None),
        };
        let key = token_value(&fields, "KEY:").map(str::to_string).unwrap_or(name);
        let mut tags = type_tags(&fields);
        if tags.is_empty()
            && let Some(inherited) = copy_base.and_then(|base| base_tags.get(base))
        {
            tags = inherited.clone();
        }
        if !tags.is_empty() {
            out.push((key, tags));
        }
    }
    out
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MergedEquipmentTypes {
    pub types: BTreeMap<String, Vec<String>>,
    /// Records whose identity was already present with a *different* tag list (the first is kept).
    pub conflicts: usize,
}

/// First record for an identity wins; a later one with different tags is counted, not applied.
pub fn merge_equipment_types(records: impl IntoIterator<Item = (String, Vec<String>)>) -> MergedEquipmentTypes {
    let mut merged = MergedEquipmentTypes::default();
    for (key, tags) in records {
        match merged.types.get(&key) {
            Some(existing) if *existing != tags => merged.conflicts += 1,
            Some(_) => {}
            None => {
                merged.types.insert(key, tags);
            }
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tags(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_record_with_a_key_token_is_identified_by_that_key() {
        let text = "Staff of Scorching\tKEY:Staff of Scorching (Base)\tTYPE:Magic.Staff\tCOST:4500\n";
        assert_eq!(
            parse_equipment_types(text),
            vec![("Staff of Scorching (Base)".to_string(), tags(&["Magic", "Staff"]))]
        );
    }

    #[test]
    fn without_a_key_token_the_declared_name_is_the_identity() {
        let text = "Longsword\tTYPE:Weapon.Melee.Martial.OneHanded\tCOST:15\n";
        assert_eq!(
            parse_equipment_types(text),
            vec![("Longsword".to_string(), tags(&["Weapon", "Melee", "Martial", "OneHanded"]))]
        );
    }

    #[test]
    fn a_copy_row_takes_the_variant_name_and_inherits_its_base_type_unless_it_states_one() {
        let text = "Spellbook\tTYPE:Goods.General\tCOST:15\n\
                    Spellbook.COPY=Runes of Wealth\tCOST:31365\n\
                    Spellbook.COPY=Plain Variant\tTYPE:Goods.Container\n";
        let parsed = parse_equipment_types(text);
        assert!(parsed.contains(&("Runes of Wealth".to_string(), tags(&["Goods", "General"]))), "{parsed:?}");
        assert!(parsed.contains(&("Plain Variant".to_string(), tags(&["Goods", "Container"]))), "{parsed:?}");
    }

    #[test]
    fn comments_directives_mod_overlays_and_internal_rows_are_not_records() {
        let text = "# a comment\n\
                    SOURCELONG:Core Rulebook\tSOURCESHORT:CR\n\
                    Dagger.MOD\tTYPE:Weapon\n\
                    Hidden\tCATEGORY:Internal\tTYPE:Weapon\n\
                    Real Item\tTYPE:Gem\n";
        assert_eq!(parse_equipment_types(text), vec![("Real Item".to_string(), tags(&["Gem"]))]);
    }

    #[test]
    fn a_record_with_no_type_token_yields_nothing() {
        assert!(parse_equipment_types("Mystery\tCOST:1\n").is_empty());
    }

    #[test]
    fn several_type_fields_are_unioned_in_order_and_clear_directives_are_ignored() {
        let text = "Odd\tTYPE:.CLEAR\tTYPE:Magic.Ring\tTYPE:Magic.Wondrous\n";
        assert_eq!(
            parse_equipment_types(text),
            vec![("Odd".to_string(), tags(&["Magic", "Ring", "Wondrous"]))]
        );
    }

    #[test]
    fn merging_keeps_the_first_type_for_a_key_and_counts_disagreements() {
        let merged = merge_equipment_types(vec![
            ("Holy Symbol (Silver)".to_string(), tags(&["Goods", "General"])),
            ("Holy Symbol (Silver)".to_string(), tags(&["Goods", "General"])),
            ("Holy Symbol (Silver)".to_string(), tags(&["Magic", "Wondrous"])),
            ("Longsword".to_string(), tags(&["Weapon"])),
        ]);
        assert_eq!(merged.types["Holy Symbol (Silver)"], tags(&["Goods", "General"]));
        assert_eq!(merged.types["Longsword"], tags(&["Weapon"]));
        assert_eq!(merged.conflicts, 1, "only the genuinely different duplicate is a conflict");
    }
}
