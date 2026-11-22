//! Reputation (Cyberpunk RED core rulebook p. 193-194).
//!
//! - The GM awards a Reputation Level 1 to 10 for a deed; "Most Cyberpunk RED Characters start
//!   with a Reputation of 0" (p. 193). The "Who Knows About You" text of each Level is read
//!   from the `reputation_level` corpus records and printed beside the Level.
//! - "A new deed's Reputation will only replace the old one if the Level is higher" (p. 193).
//!   "Reputation received from a negative event or action can replace the Reputation of a
//!   positive event or action" (p. 193): the same higher-Level rule decides, whatever the kind.
//! - Facedown (p. 194): "COOL + Reputation* + 1d10"; "*If your defining Reputation Event is for
//!   cowardice your Reputation Level is treated as a negative number." The sheet prints the
//!   resolved additive term beside the literal die ("9 + 1d10"); the die is never rolled here.
//!
//! The recognition roll ("roll 1d10 under the Reputation Level", p. 193) is printed text.

use std::fmt;
use std::path::Path;

use serde::Deserialize;

use crate::rules_core::pilot_compute::ComputationExplanation;

/// The highest Reputation Level (p. 193 table: Level 1 to 10).
pub const MAX_LEVEL: u8 = 10;

/// What a Reputation event was for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CprReputationKind {
    /// A deed the character is known for.
    Deed,
    /// A defining event for cowardice: Facedowns treat the Level as negative (p. 194).
    Cowardice,
}

/// One Reputation award: the Level the GM assigned and what it was for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprReputationEvent {
    pub level: u8,
    pub kind: CprReputationKind,
    pub description: String,
}

/// Whether a recorded event became the character's defining Reputation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CprReputationOutcome {
    /// The event's Level is higher: it is now the defining event.
    Replaced,
    /// The event's Level is not higher: the earlier defining event stays.
    Kept,
}

/// Why a Reputation event was not recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CprReputationRefusal {
    /// The GM awards a Level from the p. 193 table, 1 to 10.
    LevelOutOfRange { level: u8, max: u8 },
}

impl fmt::Display for CprReputationRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CprReputationRefusal::LevelOutOfRange { level, max } => {
                write!(f, "Reputation Level {level} is outside the table's Level 1 to {max} (p. 193)")
            }
        }
    }
}

/// A character's Reputation: the defining event, or none (Reputation 0).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CprReputation {
    defining: Option<CprReputationEvent>,
}

impl CprReputation {
    /// The Reputation Level, 0 to 10.
    pub fn level(&self) -> u8 {
        self.defining.as_ref().map_or(0, |e| e.level)
    }

    /// The defining Reputation event, if the character has one.
    pub fn defining_event(&self) -> Option<&CprReputationEvent> {
        self.defining.as_ref()
    }

    fn is_cowardice(&self) -> bool {
        matches!(self.defining.as_ref().map(|e| e.kind), Some(CprReputationKind::Cowardice))
    }

    /// The Reputation term of a Facedown: the Level, negative when the defining event is for
    /// cowardice (p. 194).
    pub fn facedown_reputation(&self) -> i16 {
        let level = i16::from(self.level());
        if self.is_cowardice() {
            -level
        } else {
            level
        }
    }

    /// Record a GM-awarded event. It becomes the defining event only when its Level is higher
    /// than the current one (p. 193).
    pub fn record(&self, event: CprReputationEvent) -> Result<(CprReputation, CprReputationOutcome), CprReputationRefusal> {
        if !(1..=MAX_LEVEL).contains(&event.level) {
            return Err(CprReputationRefusal::LevelOutOfRange { level: event.level, max: MAX_LEVEL });
        }
        if event.level > self.level() {
            Ok((CprReputation { defining: Some(event) }, CprReputationOutcome::Replaced))
        } else {
            Ok((self.clone(), CprReputationOutcome::Kept))
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct ReputationLevelRecord {
    level: u8,
    who_knows_about_you: String,
}

/// The p. 193 "Who Knows About You" text of Reputation Level 1 to 10.
#[derive(Debug, Clone)]
pub struct CprReputationCatalog {
    who_knows: Vec<String>,
}

impl CprReputationCatalog {
    /// Load `reputation_level.json` from `corpus_dir` (`GameSystem::CyberpunkRed.package_roots(..).corpus`).
    /// Refuses a table that is short, out of order or has an empty row.
    pub fn load(corpus_dir: &Path) -> Result<Self, String> {
        let path = corpus_dir.join("reputation_level.json");
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let records: Vec<ReputationLevelRecord> =
            serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        if records.len() != usize::from(MAX_LEVEL) {
            return Err(format!("reputation_level.json: {} levels, not {MAX_LEVEL}", records.len()));
        }
        let mut who_knows = Vec::with_capacity(records.len());
        for (i, record) in records.into_iter().enumerate() {
            let level = i as u8 + 1;
            if record.level != level {
                return Err(format!("reputation_level.json: entry {} is level {}, not {level}", i + 1, record.level));
            }
            if record.who_knows_about_you.trim().is_empty() {
                return Err(format!("reputation_level.json: level {level} has no Who Knows About You text"));
            }
            who_knows.push(record.who_knows_about_you);
        }
        Ok(CprReputationCatalog { who_knows })
    }

    /// The "Who Knows About You" text of `level` (1 to 10); none for Level 0.
    pub fn who_knows(&self, level: u8) -> Option<&str> {
        if level == 0 {
            return None;
        }
        self.who_knows.get(usize::from(level - 1)).map(String::as_str)
    }
}

/// The Facedown base: COOL plus the (signed) Reputation term.
fn facedown_base(cool: u8, reputation: &CprReputation) -> i16 {
    i16::from(cool) + reputation.facedown_reputation()
}

/// The Facedown line as the sheet prints it: the resolved number and the literal die, e.g.
/// "9 + 1d10" for COOL 6 and Reputation 3 (p. 194).
pub fn facedown_line(cool: u8, reputation: &CprReputation) -> String {
    format!("{} + 1d10", facedown_base(cool, reputation))
}

/// The `cpr.reputation.level` and `cpr.reputation.facedown` rows.
pub fn reputation_rows(cool: u8, reputation: &CprReputation, catalog: &CprReputationCatalog) -> Vec<ComputationExplanation> {
    let level = reputation.level();
    let level_detail = match (reputation.defining_event(), catalog.who_knows(level)) {
        (Some(event), Some(who)) => {
            let kind = match event.kind {
                CprReputationKind::Deed => String::new(),
                CprReputationKind::Cowardice => {
                    format!("; the defining Reputation Event is for cowardice, so Facedowns treat the Level as -{level} (p. 194)")
                }
            };
            format!(
                "Reputation {level}: {who} (p. 193, Who Knows About You){kind}. When first meeting, a person rolls 1d10 and has heard of the character on a roll under {level} (p. 193)"
            )
        }
        _ => "Reputation 0: Most Cyberpunk RED Characters start with a Reputation of 0 (p. 193); no deed has been awarded a Reputation Level".to_string(),
    };
    let base = facedown_base(cool, reputation);
    let term = if reputation.is_cowardice() {
        format!("COOL {cool} - Reputation {level} (cowardice) + 1d10")
    } else {
        format!("COOL {cool} + Reputation {level} + 1d10")
    };
    vec![
        ComputationExplanation { id: "cpr.reputation.level".to_string(), value: i16::from(level), detail: level_detail },
        ComputationExplanation {
            id: "cpr.reputation.facedown".to_string(),
            value: base,
            detail: format!("Facedown {}: {term} (p. 194)", facedown_line(cool, reputation)),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn catalog() -> CprReputationCatalog {
        let corpus = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/cyberpunk-red/corpus");
        CprReputationCatalog::load(&corpus).expect("the CPR reputation_level corpus loads")
    }

    fn deed(level: u8) -> CprReputationEvent {
        CprReputationEvent { level, kind: CprReputationKind::Deed, description: format!("deed {level}") }
    }

    fn cowardice(level: u8) -> CprReputationEvent {
        CprReputationEvent { level, kind: CprReputationKind::Cowardice, description: format!("ran {level}") }
    }

    fn reputation_of(event: CprReputationEvent) -> CprReputation {
        CprReputation::default().record(event).expect("a Level 1-10 event is recorded").0
    }

    /// "Who Knows About You", Reputation Level 1 to 10, as printed on p. 193 (PDF page 194).
    const PRINTED: [&str; 10] = [
        "Anyone who was there at the time knows.",
        "Stories have gotten around to immediate friends.",
        "All your co-workers and casual acquaintances know.",
        "Stories are all over the local area.",
        "Others beyond your local area recognize your name.",
        "Others beyond your local area know you on sight.",
        "A news story or two has been written about your exploits.",
        "Your exploits regularly make the headlines and screamsheets.",
        "Your exploits always make the screamsheets and TV.",
        "You're known worldwide.",
    ];

    #[test]
    fn cool_6_and_reputation_3_face_down_at_9_plus_1d10() {
        let rep = reputation_of(deed(3));
        assert_eq!(rep.level(), 3);
        assert_eq!(rep.facedown_reputation(), 3);
        assert_eq!(facedown_line(6, &rep), "9 + 1d10");

        let rows = reputation_rows(6, &rep, &catalog());
        let ids: Vec<&str> = rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["cpr.reputation.level", "cpr.reputation.facedown"]);
        assert_eq!(rows[0].value, 3);
        assert_eq!(rows[1].value, 9);
        assert!(rows[1].detail.starts_with("Facedown 9 + 1d10: COOL 6 + Reputation 3 + 1d10"), "{}", rows[1].detail);
        assert!(rows[1].detail.contains("p. 194"), "{}", rows[1].detail);
        assert!(rows[0].detail.contains(PRINTED[2]), "{}", rows[0].detail);
    }

    #[test]
    fn a_cowardice_reputation_3_with_cool_6_faces_down_at_3_plus_1d10() {
        let rep = reputation_of(cowardice(3));
        assert_eq!(rep.level(), 3, "the Level itself stays 3");
        assert_eq!(rep.facedown_reputation(), -3, "p. 194: treated as a negative number");
        assert_eq!(facedown_line(6, &rep), "3 + 1d10");

        let rows = reputation_rows(6, &rep, &catalog());
        assert_eq!(rows[0].value, 3);
        assert_eq!(rows[1].value, 3);
        assert!(rows[1].detail.starts_with("Facedown 3 + 1d10: COOL 6 - Reputation 3 (cowardice) + 1d10"), "{}", rows[1].detail);
        assert!(rows[0].detail.contains("cowardice"), "{}", rows[0].detail);
    }

    #[test]
    fn a_negative_facedown_base_prints_with_its_sign() {
        let rep = reputation_of(cowardice(5));
        assert_eq!(facedown_line(2, &rep), "-3 + 1d10");
        assert_eq!(reputation_rows(2, &rep, &catalog())[1].value, -3);
    }

    #[test]
    fn a_new_character_starts_at_reputation_0() {
        let rep = CprReputation::default();
        assert_eq!(rep.level(), 0);
        assert_eq!(rep.facedown_reputation(), 0);
        assert!(rep.defining_event().is_none());
        assert_eq!(facedown_line(6, &rep), "6 + 1d10");
        let rows = reputation_rows(6, &rep, &catalog());
        assert_eq!((rows[0].value, rows[1].value), (0, 6));
        assert!(rows[0].detail.contains("start with a Reputation of 0"), "{}", rows[0].detail);
    }

    #[test]
    fn every_level_1_to_10_carries_the_printed_who_knows_about_you_text() {
        let catalog = catalog();
        let mut compared = 0;
        for (i, want) in PRINTED.into_iter().enumerate() {
            let level = i as u8 + 1;
            assert_eq!(catalog.who_knows(level), Some(want), "level {level}");
            let rows = reputation_rows(5, &reputation_of(deed(level)), &catalog);
            assert_eq!(rows[0].value, i16::from(level));
            assert!(rows[0].detail.contains(want), "level {level}: {}", rows[0].detail);
            assert_eq!(rows[1].value, 5 + i16::from(level));
            let rows = reputation_rows(5, &reputation_of(cowardice(level)), &catalog);
            assert_eq!(rows[1].value, 5 - i16::from(level));
            compared += 1;
        }
        assert_eq!(compared, 10);
        assert_eq!(catalog.who_knows(0), None);
        assert_eq!(catalog.who_knows(11), None);
    }

    #[test]
    fn a_new_event_replaces_the_defining_one_only_when_its_level_is_higher() {
        let rep = reputation_of(deed(3));

        let (rep, outcome) = rep.record(deed(2)).unwrap();
        assert_eq!(outcome, CprReputationOutcome::Kept);
        assert_eq!(rep.level(), 3);
        let (rep, outcome) = rep.record(deed(3)).unwrap();
        assert_eq!(outcome, CprReputationOutcome::Kept, "an equal Level does not replace");
        assert_eq!(rep.defining_event().unwrap().description, "deed 3");

        let (rep, outcome) = rep.record(deed(5)).unwrap();
        assert_eq!(outcome, CprReputationOutcome::Replaced);
        assert_eq!(rep.level(), 5);

        // A cowardice event replaces a deed under the same rule (p. 193).
        let (rep, outcome) = rep.record(cowardice(4)).unwrap();
        assert_eq!(outcome, CprReputationOutcome::Kept);
        assert_eq!(rep.facedown_reputation(), 5);
        let (rep, outcome) = rep.record(cowardice(6)).unwrap();
        assert_eq!(outcome, CprReputationOutcome::Replaced);
        assert_eq!((rep.level(), rep.facedown_reputation()), (6, -6));

        // And a higher deed replaces the cowardice.
        let (rep, _) = rep.record(deed(6)).unwrap();
        assert_eq!(rep.facedown_reputation(), -6);
        let (rep, outcome) = rep.record(deed(7)).unwrap();
        assert_eq!(outcome, CprReputationOutcome::Replaced);
        assert_eq!((rep.level(), rep.facedown_reputation()), (7, 7));
    }

    #[test]
    fn an_event_outside_level_1_to_10_is_refused() {
        for level in [0u8, 11, 255] {
            assert_eq!(
                CprReputation::default().record(deed(level)),
                Err(CprReputationRefusal::LevelOutOfRange { level, max: 10 })
            );
        }
    }
}
