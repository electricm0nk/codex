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
    /// The literal, pipe-split `CSKILL:` token entries, in the corpus's
    /// own order. A `TYPE=X` entry names a whole skill-type wildcard
    /// (e.g. `"TYPE=Craft"` means every Craft subskill), kept as the
    /// literal token text rather than expanded — expansion is a SEPARATE
    /// concern (`skill_allocation.rs`'s own future consumer work), not
    /// this table's own "does the engine hold this record's content"
    /// question.
    pub skills: &'static [&'static str],
}

/// The 9 CRB base classes' own `"Class Skills ~ <Class>"` records, plus
/// `"Jack of All Trades ~ Class Skills"`. Closed list, each row
/// independently re-verified against the live corpus below — never a name
/// pattern.
pub const CLASS_SKILL_LISTS: &[ClassSkillList] = &[
    ClassSkillList {
        owner_id: "class:barbarian",
        all_skills: false,
        skills: &[
            "Acrobatics", "Climb", "TYPE=Craft", "Handle Animal", "Intimidate",
            "Knowledge (Nature)", "Perception", "Ride", "Survival", "Swim",
        ],
    },
    ClassSkillList {
        owner_id: "class:bard",
        all_skills: false,
        skills: &[
            "Acrobatics", "Appraise", "Bluff", "Climb", "TYPE=Craft", "Diplomacy",
            "Disguise", "Escape Artist", "Intimidate", "TYPE=Knowledge", "Linguistics",
            "Perception", "TYPE=Perform", "TYPE=Profession", "Sense Motive",
            "Sleight of Hand", "Spellcraft", "Stealth", "Use Magic Device",
        ],
    },
    ClassSkillList {
        owner_id: "class:cleric",
        all_skills: false,
        skills: &[
            "Appraise", "TYPE=Craft", "Diplomacy", "Heal", "Knowledge (Arcana)",
            "Knowledge (History)", "Knowledge (Nobility)", "Knowledge (Planes)",
            "Knowledge (Religion)", "Linguistics", "TYPE=Profession", "Sense Motive",
            "Spellcraft",
        ],
    },
    ClassSkillList {
        owner_id: "class:druid",
        all_skills: false,
        skills: &[
            "Climb", "TYPE=Craft", "Fly", "Handle Animal", "Heal",
            "Knowledge (Geography)", "Knowledge (Nature)", "Perception",
            "TYPE=Profession", "Ride", "Spellcraft", "Survival", "Swim",
        ],
    },
    ClassSkillList {
        owner_id: "class:fighter",
        all_skills: false,
        skills: &[
            "Climb", "TYPE=Craft", "Handle Animal", "Intimidate",
            "Knowledge (Dungeoneering)", "Knowledge (Engineering)", "TYPE=Profession",
            "Ride", "Survival", "Swim",
        ],
    },
    ClassSkillList {
        owner_id: "class:monk",
        all_skills: false,
        skills: &[
            "Acrobatics", "Climb", "TYPE=Craft", "Escape Artist", "Intimidate",
            "Knowledge (History)", "Knowledge (Religion)", "Perception", "TYPE=Perform",
            "TYPE=Profession", "Ride", "Sense Motive", "Stealth", "Swim",
        ],
    },
    ClassSkillList {
        owner_id: "class:paladin",
        all_skills: false,
        skills: &[
            "TYPE=Craft", "Diplomacy", "Handle Animal", "Heal", "Knowledge (Nobility)",
            "Knowledge (Religion)", "TYPE=Profession", "Ride", "Sense Motive", "Spellcraft",
        ],
    },
    ClassSkillList {
        owner_id: "class:ranger",
        all_skills: false,
        skills: &[
            "Climb", "TYPE=Craft", "Handle Animal", "Heal", "Intimidate",
            "Knowledge (Dungeoneering)", "Knowledge (Geography)", "Knowledge (Nature)",
            "Perception", "TYPE=Profession", "Ride", "Spellcraft", "Stealth", "Survival",
            "Swim",
        ],
    },
    ClassSkillList {
        owner_id: "class:rogue",
        all_skills: false,
        skills: &[
            "Acrobatics", "Appraise", "Bluff", "Climb", "TYPE=Craft", "Diplomacy",
            "Disable Device", "Disguise", "Escape Artist", "Intimidate",
            "Knowledge (Dungeoneering)", "Knowledge (Local)", "Linguistics", "Perception",
            "TYPE=Perform", "TYPE=Profession", "Sense Motive", "Sleight of Hand",
            "Stealth", "Swim", "Use Magic Device",
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
                let cskill = json["data"]["raw_tokens"]
                    .as_array()
                    .expect("raw_tokens is an array")
                    .iter()
                    .find(|t| t["key"].as_str() == Some("CSKILL"))
                    .and_then(|t| t["value"].as_str())
                    .unwrap_or_default();
                let expected: Vec<&str> = cskill.split('|').collect();
                assert_eq!(row.skills, expected.as_slice(), "{class_name} CSKILL list");
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
        let cskill = json["data"]["raw_tokens"]
            .as_array()
            .expect("raw_tokens is an array")
            .iter()
            .find(|t| t["key"].as_str() == Some("CSKILL"))
            .and_then(|t| t["value"].as_str())
            .unwrap_or_default();
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
