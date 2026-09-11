//! Core Rulebook class-skill-list data (`AT-34-E3-001`, `class_feature_
//! option_pool_record_not_held_by_engine` mechanism, cycle 7).
//!
//! Cycle 6's own "Remainder" table named "Class-skill/companion-mount
//! attribution, 13 units, unchanged from cycle 5 — all 13 carry
//! `description: null`; `skill_allocation.rs`'s own bounded 3-class/5-skill
//! posture does not cover the full-list shape these records carry" and its
//! own next-cycle plan called this "the largest genuine new-subsystem
//! investment... a real new consumer this engine does not have (a full
//! class-skill-list table wider than `skill_allocation.rs`'s own bounded
//! posture)".
//!
//! This module IS that table, for the 9 CRB base classes' own `"Class
//! Skills ~ <Class>"` internal chassis records (`CATEGORY:Internal`,
//! `CSKILL:` token) plus `"Jack of All Trades ~ Class Skills"`'s own
//! `CSKILL:ALL` grant — 10 of the 13-unit sub-cause cycle 6 named. The
//! remaining 3 (`Companion ~ Animal Companion`, `Companion ~ Special
//! Mount`, `Special Mount ~ Standard Choices`) are a DIFFERENT corpus shape
//! (`FOLLOWERS:`/`COMPANIONLIST:` tokens, not `CSKILL:`) and are left named
//! in this cycle's own remainder, not folded in here.
//!
//! **This table does not require a live consumer to close its 10 units,
//! by the same precedent `weapon_tables::CLASS_ARMOR_PROFICIENCIES`
//! (cycle 6) already established for this exact mechanism**: a record with
//! `description: null` moves bucket B ("engine does not hold this
//! record's content") to D ("engine holds it, nothing to display") once a
//! real, tested table transcribes and verifies its content byte-for-byte
//! against the live corpus — not once some OTHER subsystem starts reading
//! that table. `skill_allocation.rs`'s own bounded posture is a SEPARATE,
//! pre-existing consumer with its own SD-20 file-touch authority; widening
//! it to read this table is future work, not a precondition for this
//! table's own bucket B -> D move (`decisions.md §2`'s "a shelf, not a
//! half-fix").
//!
//! Every row below is transcribed verbatim from that class's own
//! `cr_abilities_class.lst` `CSKILL:` token (piped list, `TYPE=X` entries
//! kept literal, never expanded) and verified byte-for-byte against the
//! live corpus JSON in `class_skill_lists_match_their_own_corpus_records`
//! below — never a shape guess.

/// One CRB base class's own class-skill list, or the special "every skill"
/// grant (`Jack of All Trades`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassSkillList {
    /// The engine's class id, e.g. `"class:barbarian"`, or a non-class
    /// pseudo-id (`"class_feature:jack_of_all_trades"`) for the one
    /// class-feature-granted, not class-granted, row this table carries.
    pub owner_id: &'static str,
    /// `true` only for `"Jack of All Trades ~ Class Skills"`'s own
    /// `CSKILL:ALL` grant — every skill becomes a class skill, and
    /// `skills` is deliberately empty for this row (there is no
    /// enumerable list to transcribe; `ALL` is the record's whole
    /// content).
    pub all_skills: bool,
    /// The record's class-skill entries, in the corpus's own order, one per
    /// pipe-split `CSKILL:` element. Never expanded — expansion is a SEPARATE
    /// concern (`skill_allocation.rs`'s consumer work), not this table's own
    /// "does the engine hold this record's content" question.
    ///
    /// SD-35 `AT-35-E6-003-SWEEP` cycle 7: these used to ship as the literal
    /// token strings, so a family wildcard sat here as the ingest spelling
    /// `"TYPE=Craft"` and `decisions.md` §11 counted it on the live side. The
    /// distinction the wildcard carries is real and load-bearing, so it is
    /// typed rather than dropped — see [`ClassSkillEntry`]. The corpus
    /// verification test below still compares this field against the live
    /// record's own `CSKILL:` token, rebuilt element for element, so the
    /// typing is proved lossless by the same check that proved the
    /// transcription.
    pub skills: &'static [ClassSkillEntry],
}

/// One element of a class-skill list.
///
/// The ingest format writes a whole-family grant as `TYPE=<Family>` and a
/// single skill as its bare name. This crate carries the same distinction in
/// its own vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassSkillEntry {
    /// One named skill, exactly as the record spells it — `"Acrobatics"`,
    /// `"Knowledge (Nature)"`.
    Named(&'static str),
    /// Every subskill of a family — `Family("Craft")` grants every Craft
    /// subskill. The roster a family expands to lives with the consumer
    /// (`skill_allocation::skill_family_member_ids`), never here.
    Family(&'static str),
}

use ClassSkillEntry::{Family, Named};

/// The 9 CRB base classes' own `"Class Skills ~ <Class>"` records, plus
/// `"Jack of All Trades ~ Class Skills"`. Closed list, each row
/// independently re-verified against the live corpus below — never a name
/// pattern.
pub const CLASS_SKILL_LISTS: &[ClassSkillList] = &[
    ClassSkillList {
        owner_id: "class:barbarian",
        all_skills: false,
        skills: &[
            Named("Acrobatics"), Named("Climb"), Family("Craft"), Named("Handle Animal"), Named("Intimidate"),
            Named("Knowledge (Nature)"), Named("Perception"), Named("Ride"), Named("Survival"), Named("Swim"),
        ],
    },
    ClassSkillList {
        owner_id: "class:bard",
        all_skills: false,
        skills: &[
            Named("Acrobatics"), Named("Appraise"), Named("Bluff"), Named("Climb"), Family("Craft"), Named("Diplomacy"),
            Named("Disguise"), Named("Escape Artist"), Named("Intimidate"), Family("Knowledge"), Named("Linguistics"),
            Named("Perception"), Family("Perform"), Family("Profession"), Named("Sense Motive"),
            Named("Sleight of Hand"), Named("Spellcraft"), Named("Stealth"), Named("Use Magic Device"),
        ],
    },
    ClassSkillList {
        owner_id: "class:cleric",
        all_skills: false,
        skills: &[
            Named("Appraise"), Family("Craft"), Named("Diplomacy"), Named("Heal"), Named("Knowledge (Arcana)"),
            Named("Knowledge (History)"), Named("Knowledge (Nobility)"), Named("Knowledge (Planes)"),
            Named("Knowledge (Religion)"), Named("Linguistics"), Family("Profession"), Named("Sense Motive"),
            Named("Spellcraft"),
        ],
    },
    ClassSkillList {
        owner_id: "class:druid",
        all_skills: false,
        skills: &[
            Named("Climb"), Family("Craft"), Named("Fly"), Named("Handle Animal"), Named("Heal"),
            Named("Knowledge (Geography)"), Named("Knowledge (Nature)"), Named("Perception"),
            Family("Profession"), Named("Ride"), Named("Spellcraft"), Named("Survival"), Named("Swim"),
        ],
    },
    ClassSkillList {
        owner_id: "class:fighter",
        all_skills: false,
        skills: &[
            Named("Climb"), Family("Craft"), Named("Handle Animal"), Named("Intimidate"),
            Named("Knowledge (Dungeoneering)"), Named("Knowledge (Engineering)"), Family("Profession"),
            Named("Ride"), Named("Survival"), Named("Swim"),
        ],
    },
    ClassSkillList {
        owner_id: "class:monk",
        all_skills: false,
        skills: &[
            Named("Acrobatics"), Named("Climb"), Family("Craft"), Named("Escape Artist"), Named("Intimidate"),
            Named("Knowledge (History)"), Named("Knowledge (Religion)"), Named("Perception"), Family("Perform"),
            Family("Profession"), Named("Ride"), Named("Sense Motive"), Named("Stealth"), Named("Swim"),
        ],
    },
    ClassSkillList {
        owner_id: "class:paladin",
        all_skills: false,
        skills: &[
            Family("Craft"), Named("Diplomacy"), Named("Handle Animal"), Named("Heal"), Named("Knowledge (Nobility)"),
            Named("Knowledge (Religion)"), Family("Profession"), Named("Ride"), Named("Sense Motive"), Named("Spellcraft"),
        ],
    },
    ClassSkillList {
        owner_id: "class:ranger",
        all_skills: false,
        skills: &[
            Named("Climb"), Family("Craft"), Named("Handle Animal"), Named("Heal"), Named("Intimidate"),
            Named("Knowledge (Dungeoneering)"), Named("Knowledge (Geography)"), Named("Knowledge (Nature)"),
            Named("Perception"), Family("Profession"), Named("Ride"), Named("Spellcraft"), Named("Stealth"), Named("Survival"),
            Named("Swim"),
        ],
    },
    ClassSkillList {
        owner_id: "class:rogue",
        all_skills: false,
        skills: &[
            Named("Acrobatics"), Named("Appraise"), Named("Bluff"), Named("Climb"), Family("Craft"), Named("Diplomacy"),
            Named("Disable Device"), Named("Disguise"), Named("Escape Artist"), Named("Intimidate"),
            Named("Knowledge (Dungeoneering)"), Named("Knowledge (Local)"), Named("Linguistics"), Named("Perception"),
            Family("Perform"), Family("Profession"), Named("Sense Motive"), Named("Sleight of Hand"),
            Named("Stealth"), Named("Swim"), Named("Use Magic Device"),
        ],
    },
    ClassSkillList {
        owner_id: "class_feature:jack_of_all_trades",
        all_skills: true,
        skills: &[],
    },
];

/// This owner's class-skill-list row, or `None` for an owner this table
/// does not cover. `None` means "not ingested", NOT "no class skills" —
/// same discipline as `weapon_tables::class_weapon_proficiency`.
pub fn class_skill_list(owner_id: &str) -> Option<&'static ClassSkillList> {
    CLASS_SKILL_LISTS.iter().find(|entry| entry.owner_id == owner_id)
}

#[cfg(test)]
mod class_skill_list_tests {
    use super::*;
    use crate::pcgen_import::ingest_record;
    use std::path::PathBuf;

    /// Every base-class row's own claim, re-derived from the LIVE corpus
    /// record's own `CSKILL` token — not merely asserted in the table
    /// above. RED if the corpus record ever changes its class-skill list.
    #[test]
    fn class_skill_lists_match_their_own_corpus_records() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("data/corpus/core_rulebook/class_feature/class_skills");
        let expectations: &[(&str, &str)] = &[
            ("class:barbarian", "Barbarian"),
            ("class:bard", "Bard"),
            ("class:cleric", "Cleric"),
            ("class:druid", "Druid"),
            ("class:fighter", "Fighter"),
            ("class:monk", "Monk"),
            ("class:paladin", "Paladin"),
            ("class:ranger", "Ranger"),
            ("class:rogue", "Rogue"),
        ];
        for (owner_id, class_name) in expectations {
            let row = class_skill_list(owner_id)
                .unwrap_or_else(|| panic!("{owner_id} must be a real row in CLASS_SKILL_LISTS"));
            assert!(!row.all_skills, "{class_name} is a named list, not the ALL row");
            let mut found_file = false;
            for entry in std::fs::read_dir(&dir).expect("class_skills dir exists") {
                let entry = entry.expect("readable dir entry");
                let text = std::fs::read_to_string(entry.path()).expect("readable corpus json");
                let json: serde_json::Value =
                    serde_json::from_str(&text).expect("valid corpus json");
                let key = json["data"]["key"].as_str().unwrap_or_default();
                if key != format!("Class Skills ~ {class_name}") {
                    continue;
                }
                found_file = true;
                let cskill =
                    ingest_record::first_token_value(&json, "CSKILL").unwrap_or_default();
                let expected: Vec<&str> = cskill.split('|').collect();
                // Rebuild the record's own token spelling from the typed
                // entries. This is both the transcription check it has always
                // been and, since SD-35 `AT-35-E6-003-SWEEP` cycle 7, the proof
                // that typing the family wildcard lost nothing: a `Family`
                // entry must reproduce the record's element exactly.
                let rebuilt: Vec<String> = row
                    .skills
                    .iter()
                    .map(|entry| match entry {
                        ClassSkillEntry::Named(name) => (*name).to_string(),
                        ClassSkillEntry::Family(family) => format!("TYPE={family}"),
                    })
                    .collect();
                assert_eq!(rebuilt, expected, "{class_name} CSKILL list");
            }
            assert!(found_file, "no corpus record found for {class_name}");
        }
    }

    /// `"Jack of All Trades ~ Class Skills"`'s own `CSKILL:ALL` grant is a
    /// different shape (no enumerable list) — verified separately.
    #[test]
    fn jack_of_all_trades_is_the_all_skills_row() {
        let row = class_skill_list("class_feature:jack_of_all_trades")
            .expect("jack_of_all_trades must be a real row");
        assert!(row.all_skills);
        assert!(row.skills.is_empty());

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
            "data/corpus/core_rulebook/class_feature/jack_of_all_trades/jack_of_all_trades_class_skills.json",
        );
        let text = std::fs::read_to_string(&path).expect("readable corpus json");
        let json: serde_json::Value = serde_json::from_str(&text).expect("valid corpus json");
        let cskill = ingest_record::first_token_value(&json, "CSKILL").unwrap_or_default();
        assert_eq!(cskill, "ALL");
    }

    /// A class this table does not cover returns `None`, not a fabricated
    /// empty list — `None` must never be read as "no class skills".
    #[test]
    fn unknown_owner_returns_none() {
        assert_eq!(class_skill_list("class:sorcerer"), None);
        assert_eq!(class_skill_list("class:wizard"), None);
    }
}
