//! The Cyberpunk RED printed sheet lines: every line a CPR sheet carries that is not a computed
//! number, as `load_saved_character` sends it (`SheetLineDto`, `kind` per block).
//!
//! Two groups of lines:
//!
//! - **Box lines** (`kind` `cpr`): the composed sheet's own printed lines (dice as written, a
//!   weapon's or a piece of cyberware's name, "N/A", a recorded Critical Injury, a Reputation
//!   event; `compose_sheet`, `src/rules_core/cpr/sheet.rs`) and the saved Lifepath answers. The
//!   sheet places each in its box (`cyberpunkSheetModel.ts`).
//! - **Rule lines** (one `kind` per block; the sheet renders them under "Rules and features"):
//!   - `role_ability`: the Role Ability's rule text (`role_ability.json`), with every value the
//!     engine gives its Rank (`cpr.role.<role>.*` rows) printed on the line;
//!   - `cyberware`: each installed piece's effect text (`cyberware.json`) with its Humanity Loss;
//!   - `humanity` (D-6): the Humanity Loss and what it leaves: Humanity and the EMP the engine
//!     computed from it. The loss is printed, never a fixed "(-2 EMP)" (the p. 118 Media and
//!     Lawman packs print -2 for a loss of 10, which the p. 80-81 rule makes -1 from any EMP);
//!   - `weapon_special`: each carried weapon's special rules -- its alternate fire modes
//!     (Autofire, Suppressive Fire, Shotgun Shell, Arrows, Explosive) or an Exotic Weapon's own
//!     text -- with its Skill Base, ROF and, for Autofire, the Autofire Skill Base;
//!   - `netrunning` (D-2): the brain-damage rule a Netrunner's sheet prints, with the Hit Points
//!     it comes off. Changing that line's wording is an edit here and nowhere else.
//!
//! Every number on a line is read from the composed sheet's `cpr.` rows; nothing here computes a
//! rule value. A line whose record the corpus does not hold, or whose row the sheet did not
//! compose, is refused by name, never dropped. The order is fixed (box lines in the engine's
//! order, then the rule blocks in the order above), so the lines are stable for one character.

use std::path::Path;

use serde::Deserialize;

use codex::rules_core::cpr::input::{CprLifepath, CprSheetChoices};
use codex::rules_core::cpr::sheet::{CprPrintedForm, CprSheet};

use crate::character_hub::SheetLineDto;

/// The engine's printed brain-damage line (`compose_sheet`, the Netrunner block).
const BRAIN_DAMAGE_LINE: &str = "cpr.netrunning.brain_damage";
/// Further printed Netrunner rule lines the engine may add (`cpr.netrunning.printed.<i>`).
const NETRUNNING_PRINTED: &str = "cpr.netrunning.printed.";
/// The Alt. Fire Modes cell of a weapon that has none, as the weapon table prints it.
const NO_ALT_FIRE: &str = "None";

#[derive(Debug, Clone, Deserialize)]
struct RoleAbilityRecord {
    name: String,
    role: String,
    source_page: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CyberwareRecord {
    name: String,
    source_page: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct WeaponRecord {
    name: String,
    weapon_class: String,
    alt_fire_modes: Option<String>,
    source_page: String,
    text: String,
}

/// The corpus records whose rule text the sheet prints.
#[derive(Debug, Clone)]
pub(crate) struct CprPrintCatalog {
    role_abilities: Vec<RoleAbilityRecord>,
    cyberware: Vec<CyberwareRecord>,
    weapons: Vec<WeaponRecord>,
}

fn read_records<T: for<'de> Deserialize<'de>>(corpus_dir: &Path, file: &str) -> Result<Vec<T>, String> {
    let path = corpus_dir.join(file);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))
}

impl CprPrintCatalog {
    pub(crate) fn load(corpus_dir: &Path) -> Result<Self, String> {
        Ok(CprPrintCatalog {
            role_abilities: read_records(corpus_dir, "role_ability.json")?,
            cyberware: read_records(corpus_dir, "cyberware.json")?,
            weapons: read_records(corpus_dir, "weapon.json")?,
        })
    }
}

fn line(id: String, kind: &str, label: String, form: &str, value: String, also: Vec<String>, prose: String) -> SheetLineDto {
    SheetLineDto { id, kind: kind.to_owned(), label, form: form.to_owned(), value, also, prose, condition: None }
}

/// The value of the composed sheet's row `id`; a missing row is refused by name.
fn row_value(sheet: &CprSheet, id: &str) -> Result<i16, String> {
    sheet
        .rows
        .iter()
        .find(|r| r.id == id)
        .map(|r| r.value)
        .ok_or_else(|| format!("cpr_sheet_print: the sheet has no {id} row to print"))
}

/// The resolved figure a row's derivation names: its text before the first ": "
/// ("Combat Awareness points 4").
fn row_figure(detail: &str) -> &str {
    detail.split_once(": ").map_or(detail, |(head, _)| head)
}

/// The engine's printed line `id`'s value (a weapon's or a piece's name).
fn printed_value<'a>(sheet: &'a CprSheet, id: &str) -> Option<&'a str> {
    sheet.printed.iter().find(|l| l.id == id).map(|l| l.value.as_str())
}

/// Every printed line of one composed Cyberpunk RED sheet, in a fixed order (module docs).
pub(crate) fn sheet_lines(
    catalog: &CprPrintCatalog,
    sheet: &CprSheet,
    choices: &CprSheetChoices,
    lifepath: &CprLifepath,
) -> Result<Vec<SheetLineDto>, String> {
    let mut lines: Vec<SheetLineDto> = Vec::new();
    let mut netrunning: Vec<SheetLineDto> = Vec::new();
    for printed in &sheet.printed {
        let form = match printed.form {
            CprPrintedForm::Dice => "dice",
            CprPrintedForm::Words => "words",
        };
        let dto = line(printed.id.clone(), "cpr", printed.label.clone(), form, printed.value.clone(), Vec::new(), printed.prose.clone());
        if printed.id == BRAIN_DAMAGE_LINE || printed.id.starts_with(NETRUNNING_PRINTED) {
            netrunning.push(dto);
        } else {
            lines.push(dto);
        }
    }
    // The Lifepath answers, printed as saved: words, never a number.
    lines.extend(lifepath.answers.iter().enumerate().map(|(i, answer)| {
        line(
            format!("cpr.lifepath.{i}"),
            "cpr",
            if answer.column.is_empty() || answer.column == answer.table {
                answer.table.clone()
            } else {
                format!("{}: {}", answer.table, answer.column)
            },
            "words",
            answer.text.clone(),
            Vec::new(),
            if answer.detail.is_empty() {
                format!("{} (Lifepath, Cyberpunk RED core rulebook p. 45-70)", answer.text)
            } else {
                format!("{}: {} (Lifepath, Cyberpunk RED core rulebook p. 45-70)", answer.text, answer.detail)
            },
        )
    }));
    lines.extend(role_ability_lines(catalog, sheet, choices)?);
    lines.extend(cyberware_lines(catalog, sheet)?);
    lines.extend(weapon_special_lines(catalog, sheet)?);
    lines.extend(netrunning_lines(sheet, netrunning)?);
    Ok(lines)
}

/// Each held Role's Ability rule text (earlier Roles first, p. 143) with every
/// `cpr.role.<role>.` row's figure on the line.
fn role_ability_lines(catalog: &CprPrintCatalog, sheet: &CprSheet, choices: &CprSheetChoices) -> Result<Vec<SheetLineDto>, String> {
    let mut lines = Vec::new();
    for (role, _) in choices.held_roles() {
        let record = catalog
            .role_abilities
            .iter()
            .find(|r| r.role == role.name())
            .ok_or_else(|| format!("cpr_sheet_print: role_ability.json holds no Role Ability of the {}", role.name()))?;
        let prefix = format!("cpr.role.{}.", role.id());
        let also: Vec<String> =
            sheet.rows.iter().filter(|r| r.id.starts_with(&prefix)).map(|r| row_figure(&r.detail).to_owned()).collect();
        if also.is_empty() {
            return Err(format!("cpr_sheet_print: the sheet has no {prefix}* row for the {} Role Ability", record.name));
        }
        lines.push(line(
            format!("cpr.print.role_ability.{}", role.id()),
            "role_ability",
            record.name.clone(),
            "words",
            String::new(),
            also,
            format!("{} (Cyberpunk RED core rulebook {})", record.text, record.source_page),
        ));
    }
    Ok(lines)
}

/// Each installed piece's effect text with its Humanity Loss, then the D-6 Humanity line.
fn cyberware_lines(catalog: &CprPrintCatalog, sheet: &CprSheet) -> Result<Vec<SheetLineDto>, String> {
    let mut lines = Vec::new();
    for i in 0.. {
        let Some(name) = printed_value(sheet, &format!("cpr.cyberware.piece.{i}.name")) else { break };
        let record = catalog
            .cyberware
            .iter()
            .find(|r| r.name == name)
            .ok_or_else(|| format!("cpr_sheet_print: '{name}' is not a cyberware.json record"))?;
        let loss = row_value(sheet, &format!("cpr.cyberware.piece.{i}.humanity_loss"))?;
        lines.push(line(
            format!("cpr.print.cyberware.{i}"),
            "cyberware",
            record.name.clone(),
            "words",
            String::new(),
            vec![format!("Humanity Loss {loss}")],
            format!("{} (Cyberpunk RED core rulebook {})", record.text, record.source_page),
        ));
    }
    if !lines.is_empty() {
        let loss = row_value(sheet, "cpr.cyberware.humanity_loss")?;
        let humanity = row_value(sheet, "cpr.derived.humanity_current")?;
        let emp = row_value(sheet, "cpr.stat.emp")?;
        let emp_now = row_value(sheet, "cpr.derived.emp_current")?;
        lines.push(line(
            "cpr.print.humanity_loss".to_owned(),
            "humanity",
            "Humanity Loss".to_owned(),
            "words",
            String::new(),
            vec![format!("Humanity Loss {loss}"), format!("Humanity {humanity}"), format!("EMP {emp_now}")],
            format!(
                "Humanity Loss {loss} from installed cyberware: Humanity {humanity} = EMP {emp} x 10 - {loss}. EMP {emp_now}: EMP is lowered by 1 every time the tens place of Humanity is lowered (Cyberpunk RED core rulebook p. 80-81)."
            ),
        ));
    }
    Ok(lines)
}

/// Each carried weapon's special rules: its alternate fire modes, or an Exotic Weapon's text.
fn weapon_special_lines(catalog: &CprPrintCatalog, sheet: &CprSheet) -> Result<Vec<SheetLineDto>, String> {
    let mut lines = Vec::new();
    for i in 0.. {
        let Some(name) = printed_value(sheet, &format!("cpr.weapon.{i}.name")) else { break };
        let record = catalog
            .weapons
            .iter()
            .find(|r| r.name == name)
            .ok_or_else(|| format!("cpr_sheet_print: '{name}' is not a weapon.json record"))?;
        // `weapon.json` holds "None" in the Alt. Fire Modes cell of a weapon the table gives none.
        let modes = record.alt_fire_modes.as_deref().filter(|m| !m.is_empty() && *m != NO_ALT_FIRE);
        let special = match (modes, record.weapon_class.as_str()) {
            (Some(modes), _) => format!("{name}: alternate fire {modes} (Cyberpunk RED core rulebook {})", record.source_page),
            (None, "exotic") => format!("{name}: {} (Cyberpunk RED core rulebook {})", record.text, record.source_page),
            _ => continue,
        };
        let mut also = Vec::new();
        for (field, label) in [("skill_base", "Skill Base"), ("rof", "ROF")] {
            let id = format!("cpr.weapon.{i}.{field}");
            if sheet.rows.iter().any(|r| r.id == id) {
                also.push(format!("{label} {}", row_value(sheet, &id)?));
            }
        }
        if modes.is_some_and(|m| m.contains("Autofire")) {
            also.push(format!("Autofire Skill Base {}", row_value(sheet, "cpr.skill.autofire")?));
        }
        lines.push(line(format!("cpr.print.weapon.{i}"), "weapon_special", record.name.clone(), "words", String::new(), also, special));
    }
    Ok(lines)
}

/// The engine's Netrunner rule lines (D-2: brain damage), with the Hit Points it comes off.
fn netrunning_lines(sheet: &CprSheet, printed: Vec<SheetLineDto>) -> Result<Vec<SheetLineDto>, String> {
    if printed.is_empty() {
        return Ok(printed);
    }
    let hp = row_value(sheet, "cpr.derived.hit_points")?;
    Ok(printed
        .into_iter()
        .map(|mut l| {
            l.kind = "netrunning".to_owned();
            if l.id == BRAIN_DAMAGE_LINE {
                l.also = vec![format!("Hit Points {hp}"), "armor SP does not apply".to_owned()];
            }
            l
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    use codex::rules_core::cpr::cyberware::CprCyberwareInstall;
    use codex::rules_core::cpr::input::{CprCharacterInput, CprStat, CprStats};
    use codex::rules_core::cpr::roles::CprRole;
    use codex::rules_core::cpr::sheet::{compose_sheet, CprSheetCatalogs};
    use codex::rules_core::game_system::GameSystem;

    fn corpus() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(GameSystem::CyberpunkRed.corpus_relative())
    }

    /// Each fixture character's composed sheet and its printed lines.
    fn printed_fixtures() -> Vec<(String, CprSheet, Vec<SheetLineDto>)> {
        let catalogs = CprSheetCatalogs::load(&corpus()).expect("the CPR corpus loads");
        let print = CprPrintCatalog::load(&corpus()).expect("the print catalog loads");
        crate::cpr_adapter::tests::sheet_fixture_characters()
            .into_iter()
            .map(|(character, choices)| {
                let lifepath = crate::cpr_adapter::tests::sheet_fixture_lifepath(&character.character_id).unwrap_or_default();
                let sheet = compose_sheet(&catalogs, &character.input, &choices).expect("composes");
                let lines = sheet_lines(&print, &sheet, &choices, &lifepath).expect("prints");
                (character.character_id, sheet, lines)
            })
            .collect()
    }

    fn row(sheet: &CprSheet, id: &str) -> i16 {
        row_value(sheet, id).unwrap_or_else(|e| panic!("{e}"))
    }

    fn find<'a>(lines: &'a [SheetLineDto], id: &str) -> &'a SheetLineDto {
        lines.iter().find(|l| l.id == id).unwrap_or_else(|| panic!("no printed line {id}"))
    }

    /// The criterion over the four fixture characters (`cpr_adapter::tests::sheet_fixture_characters`):
    /// the Role Ability prints its rule text with every Rank value; each installed piece of
    /// cyberware prints its effect with its Humanity Loss; the Humanity Loss line (D-6) prints the
    /// loss, Humanity and EMP as the engine's rows give them; every weapon with special rules
    /// prints them with its Skill Base and ROF; the Netrunner prints the brain-damage rule (D-2)
    /// with its Hit Points. No line carries "(-2 EMP)". With `CPR_SHEET_LINES_OUT` set, each
    /// character's lines are written there, one file per character.
    #[test]
    fn every_rule_line_prints_with_its_resolved_numbers() {
        let printed = printed_fixtures();
        assert_eq!(printed.len(), 4);
        for (id, sheet, lines) in &printed {
            // Every engine printed line is carried once; the box lines keep the engine's order and
            // its Netrunner rule lines move to the netrunning block.
            let engine: Vec<&str> = sheet.printed.iter().map(|l| l.id.as_str()).collect();
            let carried: Vec<&str> = lines.iter().map(|l| l.id.as_str()).filter(|l| engine.contains(l)).collect();
            assert_eq!(carried.iter().collect::<BTreeSet<_>>(), engine.iter().collect::<BTreeSet<_>>(), "{id}: the engine's printed lines");
            assert_eq!(carried.len(), engine.len(), "{id}: each engine line once");
            let boxed = |ids: &[&str]| -> Vec<String> {
                ids.iter().filter(|l| **l != BRAIN_DAMAGE_LINE && !l.starts_with(NETRUNNING_PRINTED)).map(|l| (*l).to_owned()).collect()
            };
            assert_eq!(boxed(&carried), boxed(&engine), "{id}: the box lines in the engine's order");
            for l in lines {
                for text in [&l.label, &l.value, &l.prose] {
                    assert!(!text.contains("(-2 EMP)") && !text.contains("-2 EMP"), "{id}: {} prints {text:?}", l.id);
                }
            }
            if let Some(dir) = std::env::var_os("CPR_SHEET_LINES_OUT") {
                let text: String = lines
                    .iter()
                    .map(|l| format!("{} | {} | {} | {} | {} | {} | {}\n", l.kind, l.id, l.label, l.form, l.value, l.prose.replace('\n', " / "), l.also.join("; ")))
                    .collect();
                std::fs::write(Path::new(&dir).join(format!("{id}.txt")), text).expect("writes the character's lines");
            }
        }
        let by_id = |want: &str| printed.iter().find(|(id, _, _)| id == want).unwrap_or_else(|| panic!("no fixture {want}"));

        // The kitted Solo: Combat Awareness at Rank 4, Grafted Muscle and Bone Lace, Humanity.
        let (_, sheet, lines) = by_id("cpr-solo-kitted");
        let role = find(lines, "cpr.print.role_ability.solo");
        assert_eq!((role.kind.as_str(), role.label.as_str()), ("role_ability", "Combat Awareness"));
        assert!(role.prose.starts_with("The Solo") || role.prose.contains("Combat Awareness"), "{}", role.prose);
        assert_eq!(
            role.also,
            [format!("Combat Awareness Rank {}", row(sheet, "cpr.role.solo.rank")), format!("Combat Awareness points {}", row(sheet, "cpr.role.solo.combat_awareness_points"))]
        );
        let piece = find(lines, "cpr.print.cyberware.0");
        assert_eq!((piece.kind.as_str(), piece.label.as_str()), ("cyberware", "Grafted Muscle and Bone Lace"));
        assert!(piece.prose.contains("BODY"), "{}", piece.prose);
        assert_eq!(piece.also, [format!("Humanity Loss {}", row(sheet, "cpr.cyberware.piece.0.humanity_loss"))]);
        let humanity = find(lines, "cpr.print.humanity_loss");
        assert_eq!(humanity.kind, "humanity");
        assert_eq!(
            humanity.also,
            [
                format!("Humanity Loss {}", row(sheet, "cpr.cyberware.humanity_loss")),
                format!("Humanity {}", row(sheet, "cpr.derived.humanity_current")),
                format!("EMP {}", row(sheet, "cpr.derived.emp_current")),
            ]
        );
        // Its Heavy Pistol has no alternate fire mode and is not Exotic: no special line.
        assert!(!lines.iter().any(|l| l.kind == "weapon_special"));

        // The arsenal: the weapons with alternate fire modes print them; a plain pistol prints none.
        let (_, sheet, lines) = by_id("cpr-solo-arsenal");
        let specials: Vec<(&str, &str)> =
            lines.iter().filter(|l| l.kind == "weapon_special").map(|l| (l.id.as_str(), l.label.as_str())).collect();
        assert_eq!(
            specials,
            [
                ("cpr.print.weapon.3", "SMG"),
                ("cpr.print.weapon.4", "Heavy SMG"),
                ("cpr.print.weapon.5", "Shotgun"),
                ("cpr.print.weapon.6", "Assault Rifle"),
                ("cpr.print.weapon.8", "Bows & Crossbows"),
                ("cpr.print.weapon.9", "Grenade Launcher"),
                ("cpr.print.weapon.10", "Rocket Launcher"),
            ]
        );
        let smg = find(lines, "cpr.print.weapon.3");
        assert!(smg.prose.contains("Autofire (3)") && smg.prose.contains("Suppressive Fire"), "{}", smg.prose);
        assert_eq!(
            smg.also,
            [
                format!("Skill Base {}", row(sheet, "cpr.weapon.3.skill_base")),
                format!("ROF {}", row(sheet, "cpr.weapon.3.rof")),
                format!("Autofire Skill Base {}", row(sheet, "cpr.skill.autofire")),
            ]
        );

        // The Netrunner: the brain-damage rule (D-2) with the Hit Points it comes off.
        let (_, sheet, lines) = by_id("cpr-netrunner-decked");
        let brain = find(lines, BRAIN_DAMAGE_LINE);
        assert_eq!(brain.kind, "netrunning");
        assert!(brain.prose.contains("Hit Points") && brain.prose.contains("armor does not reduce it"), "{}", brain.prose);
        assert_eq!(brain.also[0], format!("Hit Points {}", row(sheet, "cpr.derived.hit_points")));
        let net = find(lines, "cpr.print.role_ability.netrunner");
        assert_eq!(net.also[1], format!("NET Actions per Turn {}", row(sheet, "cpr.role.netrunner.net_actions")));

        // The bare Streetrat has no Role, cyberware or weapon: it prints no rule line.
        let (_, _, lines) = by_id("cpr-solo-streetrat");
        assert!(lines.iter().all(|l| l.kind == "cpr"), "{:?}", lines.iter().map(|l| &l.id).collect::<Vec<_>>());
    }

    /// D-6: the Media and Lawman starting cyberware packs print "10 Humanity Loss (-2 EMP)" on
    /// p. 118; the sheet prints the loss and the engine's EMP, which the p. 80-81 rule lowers by 1
    /// from any multiple of 10 (EMP 6: Humanity 60 - 10 = 50, EMP 5), never the "-2 EMP".
    #[test]
    fn a_starting_pack_prints_its_humanity_loss_and_the_engines_emp_never_minus_two() {
        #[derive(Deserialize)]
        struct Kit {
            role: String,
            cyberware: Vec<KitLine>,
            printed_emp_change: Option<String>,
        }
        #[derive(Deserialize)]
        struct KitLine {
            options: Vec<KitOption>,
        }
        #[derive(Deserialize)]
        struct KitOption {
            name: String,
        }
        let kits: Vec<Kit> = read_records(&corpus(), "starting_kit.json").expect("starting kits");
        let catalogs = CprSheetCatalogs::load(&corpus()).expect("the CPR corpus loads");
        let print = CprPrintCatalog::load(&corpus()).expect("the print catalog loads");
        let mut checked = 0;
        for role in [CprRole::Media, CprRole::Lawman] {
            let kit = kits.iter().find(|k| k.role == role.name()).unwrap_or_else(|| panic!("no {} kit", role.name()));
            assert_eq!(kit.printed_emp_change.as_deref(), Some("-2 EMP"), "{}: the book prints -2 EMP (p. 118)", role.name());
            let mut stats = CprStats::default();
            for stat in CprStat::ALL {
                stats.set(stat, 6);
            }
            let choices = CprSheetChoices {
                role: Some((role, 4)),
                cyberware: kit.cyberware.iter().map(|l| CprCyberwareInstall { name: l.options[0].name.clone(), mount: None }).collect(),
                ..CprSheetChoices::default()
            };
            let sheet = compose_sheet(&catalogs, &CprCharacterInput { stats }, &choices).expect("composes");
            let lines = sheet_lines(&print, &sheet, &choices, &CprLifepath::default()).expect("prints");
            let humanity = find(&lines, "cpr.print.humanity_loss");
            let emp_now = row(&sheet, "cpr.derived.emp_current");
            assert_eq!((row(&sheet, "cpr.cyberware.humanity_loss"), emp_now), (10, 5), "{}: HL 10 from EMP 6 leaves EMP 5", role.name());
            assert!(humanity.also.contains(&"EMP 5".to_owned()), "{}: {:?}", role.name(), humanity.also);
            for l in &lines {
                assert!(!format!("{} {} {} {}", l.label, l.value, l.prose, l.also.join(" ")).contains("-2 EMP"), "{}: {}", role.name(), l.id);
            }
            checked += 1;
        }
        assert_eq!(checked, 2);
    }

    /// The lines are stable: printing one character twice gives the same lines, every id once.
    #[test]
    fn the_printed_lines_are_stable_and_each_id_is_printed_once() {
        let first = printed_fixtures();
        let second = printed_fixtures();
        for ((id, _, a), (_, _, b)) in first.iter().zip(&second) {
            let render = |lines: &[SheetLineDto]| serde_json::to_string(lines).expect("serialises");
            assert_eq!(render(a), render(b), "{id}: printed twice, the lines differ");
            let ids: BTreeSet<&str> = a.iter().map(|l| l.id.as_str()).collect();
            assert_eq!(ids.len(), a.len(), "{id}: a line id printed twice");
        }
    }

    /// A printed piece the corpus does not hold is refused by name, never dropped.
    #[test]
    fn a_line_whose_record_the_corpus_lacks_is_refused_by_name() {
        let catalogs = CprSheetCatalogs::load(&corpus()).expect("the CPR corpus loads");
        let mut print = CprPrintCatalog::load(&corpus()).expect("the print catalog loads");
        let choices = CprSheetChoices {
            cyberware: vec![CprCyberwareInstall { name: "Biomonitor".to_owned(), mount: None }],
            ..CprSheetChoices::default()
        };
        let mut stats = CprStats::default();
        for stat in CprStat::ALL {
            stats.set(stat, 6);
        }
        let sheet = compose_sheet(&catalogs, &CprCharacterInput { stats }, &choices).expect("composes");
        print.cyberware.retain(|r| r.name != "Biomonitor");
        let error = sheet_lines(&print, &sheet, &choices, &CprLifepath::default()).unwrap_err();
        assert!(error.contains("'Biomonitor' is not a cyberware.json record"), "{error}");
    }
}
