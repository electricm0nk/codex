//! Weapon-group MEMBERSHIP, resolved at ingest from the oracle's own weapon-proficiency rows
//! (`epic-f-class-completion.md` §3.3, SD-36 Epic F1-3). Read-only, closure-only input over the
//! already-loaded [`PinnedTree`] -- adds no new record kind and moves no count in
//! `data/sheet_rules/_report.json`.
//!
//! PCGen resolves a `TYPE=` weapon selector by checking whether a weapon's own accumulated
//! `TYPE:` facet -- its base `*_profs_weapon.lst` row, plus every `.MOD` row that targets it by
//! name -- contains the selector's tag(s). `uc_profs_weapon.lst:100`'s `Katana.MOD
//! TYPE:Samurai.HeavyBlade` is exactly this: "Samurai" weapon membership does not exist as its
//! own record anywhere in the corpus, only as an ADDED tag on the weapons that belong to it.
//! This module reproduces that one mechanism: a single pass over every `*_profs_weapon.lst` file
//! the pinned tree already read, building `weapon name -> its full accumulated tag set`, then
//! answering "every weapon whose tag set is a superset of these tags" for a selector.
//!
//! A row's identity is its `KEY:` token when it carries one, else its own literal NAME field
//! (stripped of a trailing `.MOD`) -- the same identity the rest of the converted proficiency
//! vocabulary uses (`ProfRef::Weapon` keys by KEY when present). A `.MOD` row addresses its
//! target the same way PCGen does: by that identity, whether it is a KEY or a bare NAME (Katana's
//! `.MOD` row targets it by NAME because Katana carries no `KEY:` override; Short Sword's would
//! target it by `Sword (Short)`, its `KEY:`). Either way the `.MOD` row's tags union onto the
//! SAME entry as the base row's, matching PCGen's own accumulation (F1 adversarial finding 2).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use super::closure::{tokenize_row, PinnedTree};

/// weapon display name (as the oracle names it, e.g. `"Katana"`) -> every `TYPE:` tag it
/// carries, own row + `.MOD` rows unioned -- exactly what PCGen itself would check a `TYPE=`
/// selector against. Tags are folded to their `to_ascii_lowercase` key: PCGen's own data mixes
/// spellings of the same tag across books (`uc_profs_weapon.lst` spells it `OneHandedFireArm`
/// while `acg_abilities_class.lst`'s `AUTO:WEAPONPROF|TYPE=OnehandedFirearm` selector spells it
/// differently), and PCGen itself resolves `TYPE=` case-insensitively -- a literal-spelling match
/// here would silently drop a real, resolvable grant (F1 re-check round 1, finding 1). The
/// weapon's own display NAME (the map key) is never folded -- only the TYPE tags used for
/// matching are, so members/labels still print the oracle's literal spelling.
pub struct WeaponMembershipIndex {
    by_weapon: BTreeMap<String, BTreeSet<String>>,
}

fn base_weapon_name(name_field: &str) -> &str {
    name_field.strip_suffix(".MOD").unwrap_or(name_field).trim()
}

impl WeaponMembershipIndex {
    /// Build from every `*_profs_weapon.lst` file the pinned tree already parsed. Never reads
    /// the filesystem itself -- `tree.files` is the same parse every other closure index uses.
    /// `_pfs/` overlay files are excluded, the same B9 rule `PinnedTree::build_indexes` already
    /// applies to `mod_index`/`define_index`/`bonus_var_index`.
    pub fn build(tree: &PinnedTree) -> Self {
        let mut by_weapon: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for file in &tree.files {
            if file.is_pfs {
                continue;
            }
            let base = file.rel_path.rsplit('/').next().unwrap_or(&file.rel_path);
            if !base.to_ascii_lowercase().ends_with("profs_weapon.lst") {
                continue;
            }
            for line in &file.lines {
                let trimmed = line.trim_end_matches('\r');
                let first = trimmed.trim_start();
                if first.is_empty() || first.starts_with('#') {
                    continue;
                }
                let (name, tokens) = tokenize_row(trimmed);
                let base_name = base_weapon_name(&name);
                if base_name.is_empty() {
                    continue;
                }
                // A row's identity is its `KEY:` token when it carries one, else its own literal
                // NAME (stripped of a trailing `.MOD`) -- the same identity every other converted
                // proficiency vocabulary uses (`ProfRef::Weapon` keys by KEY when present). A
                // `.MOD` row that targets the weapon BY that KEY (PCGen's own convention: the
                // `.MOD` row's name field is the base row's KEY, not its display NAME) therefore
                // resolves to the identical identity and its tags union onto the same entry,
                // rather than starting a second, phantom one (F1 adversarial finding 2).
                let weapon = tokens.iter().find(|(k, _)| k == "KEY").map(|(_, v)| v.trim()).filter(|v| !v.is_empty()).unwrap_or(base_name);
                let entry = by_weapon.entry(weapon.to_string()).or_default();
                for (k, v) in &tokens {
                    if k != "TYPE" {
                        continue;
                    }
                    for seg in v.split('.') {
                        let seg = seg.trim();
                        if !seg.is_empty() {
                            entry.insert(seg.to_ascii_lowercase());
                        }
                    }
                }
            }
        }
        WeaponMembershipIndex { by_weapon }
    }

    /// Every weapon whose accumulated tag set is a superset of `tags` (case-insensitive match on
    /// the oracle's own tag spelling -- PCGen itself resolves `TYPE=` case-insensitively, and the
    /// corpus spells the same tag differently across books; never a name-similarity guess),
    /// name-sorted for a deterministic converter output.
    pub fn members_with_all(&self, tags: &[String]) -> Vec<String> {
        let needles: Vec<String> = tags.iter().map(|t| t.to_ascii_lowercase()).collect();
        self.by_weapon.iter().filter(|(_, t)| needles.iter().all(|tag| t.contains(tag))).map(|(name, _)| name.clone()).collect()
    }

    /// The oracle's own spelling of a weapon-proficiency identity, matched case-insensitively
    /// (PCGen keys a proficiency name case-insensitively), or `None` when no row carries it.
    pub fn weapon_named(&self, name: &str) -> Option<&str> {
        let name = name.trim();
        self.by_weapon.keys().find(|k| k.eq_ignore_ascii_case(name)).map(String::as_str)
    }
}

/// Process-wide cache: the pinned tree is loaded once per process (`PinnedTree::load`), and
/// every `convert_record` call shares it, so this index -- itself only a read-only derivation of
/// rows the tree already holds -- is built at most once too, the same `OnceLock` shape
/// `weapon_tables.rs`'s own per-process cache uses.
static INDEX: OnceLock<WeaponMembershipIndex> = OnceLock::new();

pub fn index(tree: &PinnedTree) -> &'static WeaponMembershipIndex {
    INDEX.get_or_init(|| WeaponMembershipIndex::build(tree))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::closure::{file_family, LstFile};
    use std::path::PathBuf;

    fn tree_from_lines(files: Vec<(&str, Vec<&str>)>) -> PinnedTree {
        PinnedTree {
            root: PathBuf::new(),
            book_paths: BTreeMap::new(),
            files: files
                .into_iter()
                .map(|(name, lines)| LstFile {
                    rel_path: name.to_string(),
                    book: "test".to_string(),
                    family: file_family(name),
                    is_pfs: false,
                    lines: lines.into_iter().map(|s| s.to_string()).collect(),
                })
                .collect(),
            mod_index: BTreeMap::new(),
            base_index: BTreeMap::new(),
            keyed_index: BTreeMap::new(),
            define_index: BTreeMap::new(),
            bonus_var_index: BTreeMap::new(),
            class_rows: BTreeMap::new(),
            level_lines: BTreeMap::new(),
            fact_index: BTreeMap::new(),
            pfs_base_keys: BTreeSet::new(),
            ability_category_parent: BTreeMap::new(),
            ability_category_type: BTreeMap::new(),
        }
    }

    /// The Katana/Samurai shape (`uc_profs_weapon.lst:82,100`): membership is declared ONLY on
    /// the `.MOD` row, never as its own record.
    #[test]
    fn a_mod_row_tag_adds_the_weapon_to_that_groups_membership() {
        let tree = tree_from_lines(vec![(
            "uc_profs_weapon.lst",
            vec!["Katana\t\t\t\t\t\tTYPE:Eastern.Melee.Martial.OneHanded.Exotic", "Katana.MOD\t\t\t\t\tTYPE:Samurai.HeavyBlade"],
        )]);
        let idx = WeaponMembershipIndex::build(&tree);
        assert_eq!(idx.members_with_all(&["Samurai".to_string()]), vec!["Katana".to_string()]);
        // The base row's own tags are still there too (unioned, not replaced).
        assert_eq!(idx.members_with_all(&["Martial".to_string()]), vec!["Katana".to_string()]);
    }

    /// A conjunction (`Light.Martial`) matches only a weapon carrying BOTH tags, not either one.
    #[test]
    fn members_with_all_requires_every_tag() {
        let tree = tree_from_lines(vec![(
            "cr_profs_weapon.lst",
            vec!["Handaxe\t\t\t\t\t\tTYPE:Martial.Light.Melee", "Longsword\t\t\t\t\tTYPE:Martial.OneHanded.Melee", "Dagger\t\t\t\t\tTYPE:Simple.Light.Melee"],
        )]);
        let idx = WeaponMembershipIndex::build(&tree);
        assert_eq!(idx.members_with_all(&["Light".to_string(), "Martial".to_string()]), vec!["Handaxe".to_string()]);
    }

    /// F1 re-check round 1, finding 1: PCGen's own corpus spells the same weapon-proficiency tag
    /// with different capitalization across books (`uc_profs_weapon.lst` declares
    /// `TYPE:OneHandedFireArm`; `acg_abilities_class.lst`'s `AUTO:WEAPONPROF` selectors read
    /// `TYPE=OnehandedFirearm`/`TYPE=TwohandedFirearm`). A selector must resolve against a
    /// differently-cased member tag rather than silently dropping the whole grant.
    #[test]
    fn membership_matching_is_case_insensitive_on_the_oracle_tag_spelling() {
        let tree = tree_from_lines(vec![(
            "uc_profs_weapon.lst",
            vec![
                "Pistol\t\t\t\tTYPE:Martial.OneHandedFireArm",
                "Musket\t\t\t\tTYPE:Martial.TwoHandedFireArm",
            ],
        )]);
        let idx = WeaponMembershipIndex::build(&tree);
        assert_eq!(
            idx.members_with_all(&["OnehandedFirearm".to_string()]),
            vec!["Pistol".to_string()],
            "TYPE=OnehandedFirearm must resolve to the same member TYPE=OneHandedFireArm returns"
        );
        assert_eq!(
            idx.members_with_all(&["TwohandedFirearm".to_string()]),
            vec!["Musket".to_string()],
            "TYPE=TwohandedFirearm must resolve to the same member TYPE=TwoHandedFireArm returns"
        );
    }

    /// A `_pfs/` overlay row is excluded, matching every other closure index's own B9 rule.
    #[test]
    fn pfs_overlay_rows_are_excluded() {
        let mut tree = tree_from_lines(vec![("book/_pfs/pfs_profs_weapon.lst", vec!["Ghost Weapon\t\tTYPE:PfsOnly"])]);
        tree.files[0].is_pfs = true;
        let idx = WeaponMembershipIndex::build(&tree);
        assert!(idx.members_with_all(&["PfsOnly".to_string()]).is_empty());
    }

    /// A file that is not a `*_profs_weapon.lst` file contributes nothing, even if it declares a
    /// `TYPE:` token that happens to match -- membership is scoped to the one file kind PCGen
    /// itself uses to declare weapon-proficiency membership.
    #[test]
    fn a_non_proficiency_file_is_not_scanned() {
        let tree = tree_from_lines(vec![("cr_equip_arms_armor.lst", vec!["Katana\t\tTYPE:Samurai.Weapon"])]);
        let idx = WeaponMembershipIndex::build(&tree);
        assert!(idx.members_with_all(&["Samurai".to_string()]).is_empty());
    }

    /// F1 adversarial finding 2 (`cr_profs_weapon.lst:53,143`): a row that carries a `KEY:`
    /// distinct from its literal NAME is identified by that KEY -- the same identity every other
    /// converted proficiency uses (`ProfRef::Weapon("Sword (Short)")`, never `"Short Sword"`) --
    /// and a `.MOD` row addressed by that KEY unions its tags onto the SAME entry rather than
    /// starting a phantom second one under the un-keyed name.
    #[test]
    fn a_key_field_is_the_weapons_identity_and_a_mod_row_addressed_by_key_unions_onto_it() {
        let tree = tree_from_lines(vec![(
            "cr_profs_weapon.lst",
            vec![
                "Short Sword\t\t\tKEY:Sword (Short)\tTYPE:Martial.Light.Melee",
                "Sword (Short).MOD\t\tTYPE:Weapon Group Blades Light",
            ],
        )]);
        let idx = WeaponMembershipIndex::build(&tree);
        assert_eq!(
            idx.members_with_all(&["Light".to_string(), "Martial".to_string()]),
            vec!["Sword (Short)".to_string()],
            "the base row's tags must be filed under the KEY, not the literal NAME"
        );
        assert_eq!(
            idx.members_with_all(&["Weapon Group Blades Light".to_string()]),
            vec!["Sword (Short)".to_string()],
            "the .MOD row's tags (addressed by KEY) must union onto the same entry as the base row, not a separate one"
        );
        assert!(
            idx.members_with_all(&["Light".to_string()]).iter().all(|n| n != "Short Sword"),
            "the un-keyed literal NAME must never appear as a member identity once a KEY exists"
        );
    }
}
