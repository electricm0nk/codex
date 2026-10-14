//! Cyberpunk RED wound states and Critical Injuries (core rulebook p. 187, p. 221-223).
//!
//! Computed here, one `cpr.wound.<field>` row each: the HP below which each Wound State begins
//! (Lightly Wounded "Less than Full HP", Seriously Wounded "Less than 1/2 HP (round up)",
//! Mortally Wounded "Less than 1 HP", p. 187), the Stabilization DV printed beside each
//! (10/13/15, p. 223), the Death Save (BODY, p. 223) and the Base Death Save Penalty: the number of
//! recorded Critical Injuries whose effect reads "Base Death Save Penalty is increased by 1"
//! (p. 222-223).
//!
//! Not simulated: the current Wound State, its −2/−4 modifiers, the Death Save Penalty that
//! rises with each save during play, and each injury's effect. The sheet prints those rules;
//! the player writes in the play-time state.
//!
//! The thresholds' rules and DVs come from the corpus (`wound_state.json`); each injury and its
//! flag from `critical_injury.json`.

use std::path::Path;

use serde::Deserialize;

use crate::rules_core::cpr::stats::seriously_wounded;
use crate::rules_core::pilot_compute::ComputationExplanation;

const SOURCE_WOUNDS: &str = "Cyberpunk RED core rulebook p. 187";
const SOURCE_DEATH_SAVE: &str = "Cyberpunk RED core rulebook p. 223";
const SOURCE_CRITICALS: &str = "Cyberpunk RED core rulebook p. 222-223";

/// The two Critical Injury tables (p. 222: "Critical Injuries to the Body", p. 222-223:
/// "Critical Injuries to the Head"). Foreign Object is on both, so an injury is named by its
/// table and its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CprCriticalLocation {
    Body,
    Head,
}

/// One `critical_injury` corpus record, the fields the engine reads.
#[derive(Debug, Clone, Deserialize)]
struct CprCriticalInjuryRecord {
    name: String,
    location: CprCriticalLocation,
    roll_2d6: u8,
    base_death_save_penalty_increase: u8,
}

/// One `wound_state` corpus record, the fields the engine reads.
#[derive(Debug, Clone, Deserialize)]
struct CprWoundStateRecord {
    name: String,
    threshold_rule: String,
    stabilization_dv: Option<u8>,
}

/// One Critical Injury: its table, its 2d6 roll and whether it raises the Base Death Save
/// Penalty by 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprCriticalInjury {
    pub name: String,
    pub location: CprCriticalLocation,
    pub roll_2d6: u8,
    pub raises_base_death_save_penalty: bool,
}

/// A Critical Injury the character has (not yet removed by Treatment), by table and name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprRecordedCritical {
    pub location: CprCriticalLocation,
    pub name: String,
}

/// The HP rule a Wound State begins below. `Dead` ("One failed Death Save") has no HP
/// threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CprWoundThreshold {
    BelowFullHp,
    BelowHalfHpRoundUp,
    BelowOneHp,
}

/// A Wound State with an HP threshold and the Stabilization DV printed beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CprWoundState {
    name: String,
    threshold: CprWoundThreshold,
    stabilization_dv: u8,
}

impl CprWoundState {
    fn id(&self) -> String {
        self.name.to_lowercase().replace(' ', "_")
    }

    fn below(&self, hit_points: i16) -> i16 {
        match self.threshold {
            CprWoundThreshold::BelowFullHp => hit_points,
            CprWoundThreshold::BelowHalfHpRoundUp => seriously_wounded(hit_points),
            CprWoundThreshold::BelowOneHp => 1,
        }
    }

    fn rule_text(&self) -> &'static str {
        match self.threshold {
            CprWoundThreshold::BelowFullHp => "Less than Full HP",
            CprWoundThreshold::BelowHalfHpRoundUp => "Less than 1/2 HP (round up)",
            CprWoundThreshold::BelowOneHp => "Less than 1 HP",
        }
    }
}

/// The Wound States and Critical Injuries of a CPR corpus directory.
#[derive(Debug, Clone)]
pub struct CprCriticalCatalog {
    wound_states: Vec<CprWoundState>,
    injuries: Vec<CprCriticalInjury>,
}

impl CprCriticalCatalog {
    /// Load `wound_state.json` and `critical_injury.json` from `corpus_dir`
    /// (`GameSystem::CyberpunkRed.package_roots(..).corpus`). Refused: a threshold rule this
    /// module does not know, an HP-threshold state without a Stabilization DV, a missing one of
    /// the three HP-threshold states, a Base Death Save Penalty increase other than 0 or 1, and
    /// an injury listed twice on one table.
    pub fn load(corpus_dir: &Path) -> Result<Self, String> {
        let wound_records: Vec<CprWoundStateRecord> = read_json(corpus_dir, "wound_state.json")?;
        let mut wound_states = Vec::new();
        for record in wound_records {
            let threshold = match record.threshold_rule.as_str() {
                "below_full_hp" => CprWoundThreshold::BelowFullHp,
                "below_half_hp_round_up" => CprWoundThreshold::BelowHalfHpRoundUp,
                "below_one_hp" => CprWoundThreshold::BelowOneHp,
                "failed_death_save" => continue,
                other => return Err(format!("{}: unknown wound threshold rule {other}", record.name)),
            };
            let stabilization_dv =
                record.stabilization_dv.ok_or_else(|| format!("{}: a Wound State with an HP threshold has a Stabilization DV (p. 223)", record.name))?;
            wound_states.push(CprWoundState { name: record.name, threshold, stabilization_dv });
        }
        for wanted in [CprWoundThreshold::BelowFullHp, CprWoundThreshold::BelowHalfHpRoundUp, CprWoundThreshold::BelowOneHp] {
            if wound_states.iter().filter(|s| s.threshold == wanted).count() != 1 {
                return Err(format!("wound_state.json must hold exactly one {wanted:?} state"));
            }
        }
        wound_states.sort_by_key(|s| s.threshold as u8);

        let injury_records: Vec<CprCriticalInjuryRecord> = read_json(corpus_dir, "critical_injury.json")?;
        let mut injuries: Vec<CprCriticalInjury> = Vec::new();
        for record in injury_records {
            let raises = match record.base_death_save_penalty_increase {
                0 => false,
                1 => true,
                n => return Err(format!("{}: a Critical Injury raises the Base Death Save Penalty by 0 or 1, not {n} (p. 222-223)", record.name)),
            };
            if injuries.iter().any(|i| i.location == record.location && i.name == record.name) {
                return Err(format!("{} is listed twice on the {:?} table", record.name, record.location));
            }
            injuries.push(CprCriticalInjury {
                name: record.name,
                location: record.location,
                roll_2d6: record.roll_2d6,
                raises_base_death_save_penalty: raises,
            });
        }
        Ok(CprCriticalCatalog { wound_states, injuries })
    }

    /// The number of Critical Injuries on both tables.
    pub fn injury_count(&self) -> usize {
        self.injuries.len()
    }

    pub fn injury(&self, location: CprCriticalLocation, name: &str) -> Option<&CprCriticalInjury> {
        self.injuries.iter().find(|i| i.location == location && i.name == name)
    }

    /// The injuries among `recorded` that raise the Base Death Save Penalty. Refused: an
    /// injury not on that table, and the same injury twice (p. 188: a Critical Injury the target
    /// already has is rerolled).
    fn raising(&self, recorded: &[CprRecordedCritical]) -> Result<Vec<&CprCriticalInjury>, String> {
        let mut seen: Vec<&CprCriticalInjury> = Vec::new();
        for critical in recorded {
            let injury = self
                .injury(critical.location, &critical.name)
                .ok_or_else(|| format!("{} is not on the Critical Injuries to the {:?} table (p. 222-223)", critical.name, critical.location))?;
            if seen.contains(&injury) {
                return Err(format!("the character already has {} ({:?}); a repeated Critical Injury is rerolled (p. 188)", injury.name, injury.location));
            }
            seen.push(injury);
        }
        Ok(seen.into_iter().filter(|i| i.raises_base_death_save_penalty).collect())
    }

    /// The Base Death Save Penalty: +1 for each recorded Critical Injury that raises it.
    pub fn base_death_save_penalty(&self, recorded: &[CprRecordedCritical]) -> Result<u8, String> {
        Ok(self.raising(recorded)?.len() as u8)
    }

    /// The `cpr.wound.` rows for a character with `hit_points` total HP and `body` BODY: each
    /// HP-threshold Wound State with its Stabilization DV, then the Death Save and the Base
    /// Death Save Penalty from `recorded`.
    pub fn rows(&self, hit_points: i16, body: u8, recorded: &[CprRecordedCritical]) -> Result<Vec<ComputationExplanation>, String> {
        if hit_points < 1 {
            return Err(format!("total HP {hit_points} is below 1; HP is 10 + 5 x ceil((BODY + WILL) / 2) (p. 79)"));
        }
        let raising = self.raising(recorded)?;
        let row = |id: &str, value: i16, detail: String| ComputationExplanation { id: format!("cpr.wound.{id}"), value, detail };
        let mut rows = Vec::new();
        for state in &self.wound_states {
            let id = state.id();
            let below = state.below(hit_points);
            rows.push(row(
                &id,
                below,
                format!("{} below {below} HP: \"{}\" of HP {hit_points} ({SOURCE_WOUNDS})", state.name, state.rule_text()),
            ));
            rows.push(row(
                &format!("{id}_dv"),
                i16::from(state.stabilization_dv),
                format!(
                    "Stabilize {} DV{}: TECH + First Aid or Paramedic + 1d10 ({SOURCE_DEATH_SAVE})",
                    state.name, state.stabilization_dv
                ),
            ));
        }
        rows.push(row(
            "death_save",
            i16::from(body),
            format!("Death Save {body} = BODY {body}: roll a d10 plus the Death Save Penalty under it; a 10 always fails ({SOURCE_DEATH_SAVE})"),
        ));
        let names: Vec<&str> = raising.iter().map(|i| i.name.as_str()).collect();
        rows.push(row(
            "base_death_save_penalty",
            raising.len() as i16,
            format!(
                "Base Death Save Penalty {} = +1 per recorded Critical Injury that raises it ({}) ({SOURCE_CRITICALS})",
                raising.len(),
                if names.is_empty() { "none recorded".to_string() } else { names.join(", ") }
            ),
        ));
        Ok(rows)
    }
}

fn read_json<T: for<'de> Deserialize<'de>>(corpus_dir: &Path, file: &str) -> Result<Vec<T>, String> {
    let path = corpus_dir.join(file);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// The 22 Critical Injuries, hand-listed from the cycle-0 combat inventory
    /// (`artifacts/cycle_0/inventory/combat.md` §4g, read from p188-p189 / p222-p223), not from
    /// the corpus the engine reads: (location, 2d6 roll, injury, raises the Base Death Save
    /// Penalty by 1). Second count of the flag on the page text:
    /// `awk '/Base Death Save Penalty is increased by 1/{n++} END{print n}' p222.txt p223.txt` -> 11.
    const CRITICAL_INJURIES_P222: [(CprCriticalLocation, u8, &str, bool); 22] = [
        (CprCriticalLocation::Body, 2, "Dismembered Arm", true),
        (CprCriticalLocation::Body, 3, "Dismembered Hand", true),
        (CprCriticalLocation::Body, 4, "Collapsed Lung", true),
        (CprCriticalLocation::Body, 5, "Broken Ribs", false),
        (CprCriticalLocation::Body, 6, "Broken Arm", false),
        (CprCriticalLocation::Body, 7, "Foreign Object", false),
        (CprCriticalLocation::Body, 8, "Broken Leg", false),
        (CprCriticalLocation::Body, 9, "Torn Muscle", false),
        (CprCriticalLocation::Body, 10, "Spinal Injury", true),
        (CprCriticalLocation::Body, 11, "Crushed Fingers", false),
        (CprCriticalLocation::Body, 12, "Dismembered Leg", true),
        (CprCriticalLocation::Head, 2, "Lost Eye", true),
        (CprCriticalLocation::Head, 3, "Brain Injury", true),
        (CprCriticalLocation::Head, 4, "Damaged Eye", false),
        (CprCriticalLocation::Head, 5, "Concussion", false),
        (CprCriticalLocation::Head, 6, "Broken Jaw", false),
        (CprCriticalLocation::Head, 7, "Foreign Object", false),
        (CprCriticalLocation::Head, 8, "Whiplash", true),
        (CprCriticalLocation::Head, 9, "Cracked Skull", true),
        (CprCriticalLocation::Head, 10, "Damaged Ear", false),
        (CprCriticalLocation::Head, 11, "Crushed Windpipe", true),
        (CprCriticalLocation::Head, 12, "Lost Ear", true),
    ];

    fn catalog() -> CprCriticalCatalog {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/cyberpunk-red/corpus");
        CprCriticalCatalog::load(&dir).expect("the CPR wound-state and critical-injury corpus loads")
    }

    fn crit(location: CprCriticalLocation, name: &str) -> CprRecordedCritical {
        CprRecordedCritical { location, name: name.to_string() }
    }

    fn row(rows: &[ComputationExplanation], id: &str) -> i16 {
        rows.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("no row {id}")).value
    }

    #[test]
    fn thresholds_for_hp_40_are_full_20_and_1_with_stabilization_dvs_10_13_15() {
        let rows = catalog().rows(40, 8, &[]).unwrap();
        assert_eq!(row(&rows, "cpr.wound.lightly_wounded"), 40, "Less than Full HP: below 40");
        assert_eq!(row(&rows, "cpr.wound.seriously_wounded"), 20, "Less than 1/2 HP (round up): below 20");
        assert_eq!(row(&rows, "cpr.wound.mortally_wounded"), 1, "Less than 1 HP");
        assert_eq!(row(&rows, "cpr.wound.lightly_wounded_dv"), 10);
        assert_eq!(row(&rows, "cpr.wound.seriously_wounded_dv"), 13);
        assert_eq!(row(&rows, "cpr.wound.mortally_wounded_dv"), 15);
    }

    #[test]
    fn the_seriously_wounded_threshold_rounds_half_hp_up() {
        let rows = catalog().rows(25, 5, &[]).unwrap();
        assert_eq!(row(&rows, "cpr.wound.seriously_wounded"), 13, "ceil(25 / 2)");
        assert_eq!(row(&rows, "cpr.wound.lightly_wounded"), 25);
    }

    #[test]
    fn death_save_is_body() {
        for body in [2u8, 6, 8, 10] {
            let rows = catalog().rows(40, body, &[]).unwrap();
            assert_eq!(row(&rows, "cpr.wound.death_save"), i16::from(body));
        }
    }

    #[test]
    fn the_catalog_equals_the_hand_listed_22_injuries_and_their_flags() {
        let catalog = catalog();
        assert_eq!(catalog.injury_count(), 22);
        for (location, roll, name, flag) in CRITICAL_INJURIES_P222 {
            let injury = catalog.injury(location, name).unwrap_or_else(|| panic!("{name} ({location:?}) missing"));
            assert_eq!(injury.roll_2d6, roll, "{name}");
            assert_eq!(injury.raises_base_death_save_penalty, flag, "{name} ({location:?})");
        }
        let flagged = CRITICAL_INJURIES_P222.iter().filter(|c| c.3).count();
        assert_eq!(flagged, 11, "5 body + 6 head injuries raise the Base Death Save Penalty");
    }

    #[test]
    fn each_of_the_22_injuries_counts_its_own_flag_toward_the_base_death_save_penalty() {
        let catalog = catalog();
        for (location, _, name, flag) in CRITICAL_INJURIES_P222 {
            let penalty = catalog.base_death_save_penalty(&[crit(location, name)]).unwrap();
            assert_eq!(penalty, u8::from(flag), "{name} ({location:?}) alone");
        }
        let all: Vec<CprRecordedCritical> = CRITICAL_INJURIES_P222.iter().map(|(l, _, n, _)| crit(*l, n)).collect();
        assert_eq!(catalog.base_death_save_penalty(&all).unwrap(), 11, "all 22 recorded: 11 flagged");
        assert_eq!(catalog.base_death_save_penalty(&[]).unwrap(), 0, "no recorded criticals: 0");
    }

    #[test]
    fn the_base_death_save_penalty_row_names_the_injuries_that_raise_it() {
        let recorded = [
            crit(CprCriticalLocation::Body, "Dismembered Arm"),
            crit(CprCriticalLocation::Head, "Whiplash"),
            crit(CprCriticalLocation::Body, "Broken Ribs"),
        ];
        let rows = catalog().rows(40, 8, &recorded).unwrap();
        assert_eq!(row(&rows, "cpr.wound.base_death_save_penalty"), 2, "Broken Ribs does not raise it");
        let detail = &rows.iter().find(|r| r.id == "cpr.wound.base_death_save_penalty").unwrap().detail;
        assert!(detail.contains("Dismembered Arm") && detail.contains("Whiplash"), "{detail}");
        assert!(!detail.contains("Broken Ribs"), "{detail}");
    }

    #[test]
    fn foreign_object_is_two_injuries_one_per_table() {
        let catalog = catalog();
        let body = catalog.injury(CprCriticalLocation::Body, "Foreign Object").unwrap();
        let head = catalog.injury(CprCriticalLocation::Head, "Foreign Object").unwrap();
        assert_eq!((body.roll_2d6, head.roll_2d6), (7, 7));
        let both = [crit(CprCriticalLocation::Body, "Foreign Object"), crit(CprCriticalLocation::Head, "Foreign Object")];
        assert_eq!(catalog.base_death_save_penalty(&both).unwrap(), 0, "both recorded, neither raises it");
    }

    #[test]
    fn refusals_name_their_reason() {
        let catalog = catalog();
        let twice = [crit(CprCriticalLocation::Head, "Lost Eye"), crit(CprCriticalLocation::Head, "Lost Eye")];
        assert!(catalog.base_death_save_penalty(&twice).unwrap_err().contains("already has Lost Eye"));
        let unknown = [crit(CprCriticalLocation::Body, "Lost Eye")];
        assert!(catalog.base_death_save_penalty(&unknown).unwrap_err().contains("Lost Eye"));
        assert!(catalog.rows(0, 8, &[]).unwrap_err().contains("HP"));
    }

    #[test]
    fn rows_are_cpr_rows_in_sheet_order_and_cite_a_page() {
        let rows = catalog().rows(40, 8, &[]).unwrap();
        let ids: Vec<&str> = rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(
            ids,
            [
                "cpr.wound.lightly_wounded",
                "cpr.wound.lightly_wounded_dv",
                "cpr.wound.seriously_wounded",
                "cpr.wound.seriously_wounded_dv",
                "cpr.wound.mortally_wounded",
                "cpr.wound.mortally_wounded_dv",
                "cpr.wound.death_save",
                "cpr.wound.base_death_save_penalty",
            ]
        );
        assert!(rows.iter().all(|r| r.detail.contains("p. ")), "every row cites a page");
    }
}
