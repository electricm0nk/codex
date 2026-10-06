//! SD-37 E7.1: Starfinder oracle parity (`docs/release/SD-37-starfinder-1e/epic-breakdown.md` E7.1).
//!
//! The roster is the four SD-37 seeds, a level-1 build of each of the 10 Starfinder player classes
//! (the Mechanic 1 is E5.4's parity build, with its drone) -- 14 characters and one drone. Each is
//! rendered twice: by the REAL PCGen engine at the pinned oracle
//! (`scripts/oracle_harness/sf_parity_run.sh`, one `.pcg` per build from
//! `scripts/oracle_harness/sf_parity_builds.py`, outputs committed under
//! `scripts/oracle_harness/sf_parity/`) and by this engine ([`crate::sf_adapter::compute_sheet`]
//! over the build's `CharacterInput`). Every number PCGen exports for a build is compared with the
//! engine's `sf.*` explanation row for the same total.
//!
//! A difference is either **explained** -- a row of `scripts/oracle_harness/sf_parity/explained.tsv`
//! naming the build, the field, both values and why (PCGen departs from the SRD, cited; or the engine
//! does not compute the value, with the decision that says so) -- or a failure. A ledger row whose
//! difference no longer exists is a failure too (a stale explanation hides a fix nobody checked).
//!
//! The oracle files are a fixture of a real run, not a mock: the run script re-derives every one
//! of them, and this test fails when a build's identity (class, level, the six scores, every skill's
//! ranks) in the file disagrees with the engine build, so a drift between the two sides of a build
//! cannot pass as parity.

/// Test-only: the roster and the comparison.
#[cfg(test)]
pub(crate) mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::{Path, PathBuf};

    use codex::rules_core::character_input::{
        AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState,
        EquipmentSelection, SelectedChoice, SkillAllocation, SpellSelection,
    };
    use codex::rules_core::sheet_rule::{split_rule_id, SheetLine, SheetLineValue};

    use crate::sf_adapter::{compute_sheet, package};

    fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
    }

    pub(crate) const ORACLE_DIR: &str = "scripts/oracle_harness/sf_parity";
    pub(crate) const LEDGER: &str = "scripts/oracle_harness/sf_parity/explained.tsv";

    fn item(id: &str, active_state: ActiveState) -> EquipmentSelection {
        EquipmentSelection {
            item_id: format!("core:equipment:{id}"),
            equipped_or_active: active_state == ActiveState::EquippedActive,
            active_state,
            applied_modifiers: Vec::new(),
        }
    }

    fn choice(set: &str, selection: &str) -> SelectedChoice {
        SelectedChoice { choice_set_id: set.to_owned(), selection_id: selection.to_owned() }
    }

    /// A level-1 human of `class` (`sf_parity_builds.py` holds the same build by the oracle's
    /// names): the human's +2 on `key`, the theme, the skill ranks, worn second skin, a wielded
    /// azimuth laser pistol and a carried tactical baton, and the spells known.
    fn level_one(
        class: &str,
        scores: [i16; 6],
        theme: &str,
        key: &str,
        class_choice: Option<&str>,
        ranks: &[&str],
        spells: &[&str],
    ) -> CharacterInput {
        let [strength, dexterity, constitution, intelligence, wisdom, charisma] = scores;
        let class_id = match class {
            "biohacker" | "vanguard" | "witchwarper" => format!("character_operations_manual:class:{class}"),
            _ => format!("core:class:{class}"),
        };
        let mut choices = vec![choice("core:ability:2_racial_stat_bonus", key)];
        if let Some(c) = class_choice {
            choices.push(choice(&class_id, c));
        }
        CharacterInput {
            case_id: None,
            source_package_id: "starfinder-1e".to_owned(),
            chosen: ChosenCharacterState {
                race_id: "core:race:human".to_owned(),
                class_levels: vec![CharacterClassLevel { class_id: class_id.clone(), level: 1 }],
                ability_scores: AbilityScores { strength, dexterity, constitution, intelligence, wisdom, charisma },
                selected_feats: vec![format!("core:ability:{theme}"), "core:ability:2_racial_stat_bonus".to_owned()],
                skill_allocations: ranks.iter().map(|s| SkillAllocation { skill_id: (*s).to_owned(), ranks: 1 }).collect(),
                equipment_selections: vec![
                    item("second_skin", ActiveState::EquippedActive),
                    item("laser_pistol_azimuth", ActiveState::EquippedActive),
                    item("baton_tactical", ActiveState::SelectedInactive),
                ],
                selected_choices: choices,
                selected_traits: Vec::new(),
                spells_selected: spells
                    .iter()
                    .map(|s| SpellSelection {
                        spell_id: (*s).to_owned(),
                        source_class_id: class_id.clone(),
                        acquisition_mode: AcquisitionMode::Known,
                    })
                    .collect(),
                class_ability_activations: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    /// Every roster build: `(build id, oracle file stem, engine input)`.
    pub(crate) fn roster() -> Vec<(String, String, CharacterInput)> {
        let mut out: Vec<(String, String, CharacterInput)> = crate::sf_adapter::tests::seeds()
            .into_iter()
            .map(|(seed, _, input)| (seed.to_owned(), stem(seed), input))
            .collect();
        out.push(("SF-Mechanic-1".into(), stem("SF-Mechanic-1"), crate::sf_drone_print::tests::mechanic_1_input()));
        let l1 = [
            ("SF-Envoy-1", level_one("envoy", [10, 12, 12, 11, 10, 18], "xenoseeker", "CHA", None, &["bluff", "diplomacy", "sense_motive", "culture"], &[])),
            (
                "SF-Mystic-1",
                level_one(
                    "mystic",
                    [10, 12, 12, 10, 18, 11],
                    "priest",
                    "WIS",
                    None,
                    &["mysticism", "medicine", "perception"],
                    &["core:spell:detect_magic", "core:spell:stabilize", "core:spell:mystic_cure_level_1"],
                ),
            ),
            ("SF-Operative-1", level_one("operative", [10, 18, 12, 12, 11, 10], "ace_pilot", "DEX", None, &["acrobatics", "stealth", "piloting", "perception"], &[])),
            ("SF-Solarian-1", level_one("solarian", [12, 11, 12, 10, 10, 18], "bounty_hunter", "CHA", None, &["athletics", "mysticism", "survival"], &[])),
            ("SF-Soldier-1", level_one("soldier", [12, 18, 12, 10, 11, 10], "outlaw", "DEX", Some("DEX"), &["athletics", "piloting", "sleight_of_hand"], &[])),
            (
                "SF-Technomancer-1",
                level_one(
                    "technomancer",
                    [10, 12, 12, 18, 11, 10],
                    "spacefarer",
                    "INT",
                    None,
                    &["computers", "engineering", "physical_science"],
                    &["core:spell:detect_magic", "core:spell:energy_ray", "core:spell:magic_missile"],
                ),
            ),
            ("SF-Biohacker-1", level_one("biohacker", [10, 14, 11, 18, 10, 10], "ace_pilot", "INT", Some("INT"), &["medicine", "life_science", "computers"], &[])),
            ("SF-Vanguard-1", level_one("vanguard", [12, 13, 18, 10, 10, 10], "bounty_hunter", "CON", None, &["athletics", "acrobatics", "intimidate"], &[])),
            (
                "SF-Witchwarper-1",
                level_one(
                    "witchwarper",
                    [10, 14, 11, 10, 10, 18],
                    "icon",
                    "CHA",
                    None,
                    &["mysticism", "bluff", "physical_science"],
                    &["character_operations_manual:spell:hazard", "character_operations_manual:spell:puncture_veil"],
                ),
            ),
        ];
        out.extend(l1.into_iter().map(|(b, input)| (b.to_owned(), stem(b), input)));
        out
    }

    fn stem(build: &str) -> String {
        build.to_ascii_lowercase().replace('-', "_")
    }

    /// `+3` / `-1` / `3` / `4.3 lbs.` / `1955 cr` -> the number as written (sign dropped).
    fn number(text: &str) -> Option<String> {
        let t = text.trim().trim_end_matches(" lbs.").trim_end_matches(" cr").trim();
        let t = t.strip_prefix('+').unwrap_or(t);
        (!t.is_empty() && t.parse::<f64>().is_ok()).then(|| t.to_string())
    }

    /// The damage bonus of a PCGen damage string (`1d8+3` -> `3`, `1d4` -> `0`).
    fn damage_bonus(text: &str) -> Option<String> {
        let t = text.trim();
        let after_dice = t.find('d').map(|i| &t[i + 1..]).unwrap_or(t);
        match after_dice.find(['+', '-']) {
            Some(i) => number(&after_dice[i..]),
            None => Some("0".into()),
        }
    }

    /// The oracle file of one build, normalised to `field -> value` (module doc).
    pub(crate) fn oracle_view(text: &str) -> BTreeMap<String, String> {
        let mut out = BTreeMap::new();
        for line in text.lines() {
            let Some((key, value)) = line.split_once('=') else { continue };
            let parts: Vec<&str> = value.split('|').collect();
            if let Some(name) = key.strip_prefix("skill.") {
                let field = |p: &str| parts.iter().find_map(|x| x.strip_prefix(p)).unwrap_or("");
                let ranks = field("ranks=").trim_end_matches(".0").to_string();
                let trained_only = field("untrained=") == "NO";
                let total = if trained_only && ranks == "0" { Some("untrained".to_string()) } else { number(parts[0]) };
                if let Some(t) = total {
                    out.insert(format!("skill.{name}"), t);
                }
                if !ranks.is_empty() {
                    out.insert(format!("skill.{name}.ranks"), ranks);
                }
            } else if let Some(name) = key.strip_prefix("weapon.") {
                let name = name.trim_start_matches('*');
                if let Some(v) = number(parts[0]) {
                    out.insert(format!("weapon.{name}.attack"), v);
                }
                if let Some(d) = parts.iter().find_map(|x| x.strip_prefix("damage=")).and_then(damage_bonus) {
                    out.insert(format!("weapon.{name}.damage"), d);
                }
            } else if let Some(rest) = key.strip_prefix("spells.") {
                let Some((class, level)) = rest.rsplit_once('.') else { continue };
                let class = class.to_ascii_lowercase();
                let field = |p: &str| parts.iter().find_map(|x| x.strip_prefix(p)).unwrap_or("").to_string();
                let (per_day, known) = (field("per_day:"), field("known:"));
                // Every known spell's DC at the level (`dcs:`); a spell with no saving throw has
                // none. They agree, or the level's DC is the disagreement itself.
                let dcs: BTreeSet<String> = field("dcs:").split(',').map(str::trim).filter(|d| !d.is_empty()).map(str::to_string).collect();
                let dc = dcs.into_iter().collect::<Vec<_>>().join("|");
                if per_day == "0" && known == "0" {
                    continue;
                }
                out.insert(format!("spells.{class}.{level}.per_day"), per_day);
                out.insert(format!("spells.{class}.{level}.known"), known);
                if !dc.is_empty() {
                    out.insert(format!("spells.{class}.{level}.dc"), dc);
                }
            } else if let Some(mode) = key.strip_prefix("var.").filter(|m| matches!(*m, "walk" | "fly")) {
                // The drone's speeds (`VAR.Walk`, `VAR.Fly`).
                if let Some(v) = number(value) {
                    out.insert(format!("speed.{mode}"), v);
                }
            } else if matches!(key, "name" | "race" | "credits" | "acp") || key.starts_with("abilities.") || key.starts_with("var.") || key.starts_with("master") {
                continue;
            } else if key == "class" {
                out.insert(key.into(), value.to_ascii_lowercase());
            } else if let Some(v) = number(value) {
                out.insert(key.into(), v);
            }
        }
        out
    }

    const STATS: [(&str, &str); 6] = [
        ("STR", "strength"),
        ("DEX", "dexterity"),
        ("CON", "constitution"),
        ("INT", "intelligence"),
        ("WIS", "wisdom"),
        ("CHA", "charisma"),
    ];

    /// The engine's values for one build, keyed like [`oracle_view`].
    pub(crate) fn engine_view(input: &CharacterInput) -> BTreeMap<String, String> {
        let package = package().expect("the Starfinder package loads");
        let sheet = compute_sheet(package, input).unwrap_or_else(|r| panic!("{r:?}"));
        let rows: BTreeMap<String, i64> = sheet.explanations().into_iter().map(|e| (e.id, i64::from(e.value))).collect();
        let mut out = BTreeMap::new();
        let (class, level) = sheet.classes.first().cloned().unwrap_or_default();
        out.insert("class".into(), class.rsplit(':').next().unwrap_or("").to_string());
        out.insert("level".into(), level.to_string());
        for (short, long) in STATS {
            out.insert(format!("stat.{short}"), rows[&format!("sf.ability_score.{long}")].to_string());
        }
        for (key, row) in [
            ("hp", "sf.hit_points"),
            ("stamina", "sf.stamina"),
            ("resolve", "sf.resolve"),
            ("eac", "sf.eac"),
            ("kac", "sf.kac"),
            ("initiative", "sf.initiative"),
            ("bab", "sf.base_attack_bonus"),
            ("save.Fortitude", "sf.fortitude"),
            ("save.Reflex", "sf.reflex"),
            ("save.Will", "sf.will"),
            ("attack.melee", "sf.attack.melee"),
            ("attack.ranged", "sf.attack.ranged"),
            ("bulk", "sf.bulk"),
            ("credits.spent", "sf.credits.spent"),
        ] {
            if let Some(v) = rows.get(row) {
                out.insert(key.into(), v.to_string());
            }
        }
        for skill in &sheet.skills.skills {
            let value = match &skill.total {
                Some(t) => t.total.to_string(),
                None => "untrained".into(),
            };
            out.insert(format!("skill.{}", skill.label), value);
            let ranks = input.chosen.skill_allocations.iter().find(|a| a.skill_id == skill.skill).map_or(0, |a| a.ranks);
            out.insert(format!("skill.{}.ranks", skill.label), ranks.to_string());
        }
        for casting in &sheet.spells {
            let class = casting.class.rsplit(':').next().unwrap_or("").to_string();
            for level in &casting.levels {
                for (field, total) in [("per_day", &level.per_day), ("known", &level.known), ("dc", &level.save_dc)] {
                    if let Some(t) = total {
                        out.insert(format!("spells.{class}.{}.{field}", level.level), t.total.to_string());
                    }
                }
            }
        }
        // A weapon is keyed by its printed name (PCGen exports the same name): its rows are
        // `sf.weapon.<equipment slug>.attack` / `.damage`.
        for weapon in &sheet.attacks.weapons {
            let item = weapon.item.rsplit(':').next().unwrap_or("");
            for field in ["attack", "damage"] {
                if let Some(v) = rows.get(&format!("sf.weapon.{item}.{field}")) {
                    out.insert(format!("weapon.{}.{field}", weapon.label), v.to_string());
                }
            }
        }
        // Every other engine row under its own id: the oracle exports none of these, so they
        // are the "not in the oracle" list (`sf_oracle_parity_the_not_in_oracle_list_is_current`).
        let mapped = |id: &str| {
            id.starts_with("sf.ability_score.")
                || id.starts_with("sf.skill.")
                || id.starts_with("sf.spells.")
                || id.starts_with("sf.weapon.")
                || [
                    "sf.hit_points",
                    "sf.stamina",
                    "sf.resolve",
                    "sf.eac",
                    "sf.kac",
                    "sf.initiative",
                    "sf.base_attack_bonus",
                    "sf.fortitude",
                    "sf.reflex",
                    "sf.will",
                    "sf.attack.melee",
                    "sf.attack.ranged",
                    "sf.bulk",
                    "sf.credits.spent",
                ]
                .contains(&id)
        };
        for (id, v) in &rows {
            if !mapped(id) {
                out.insert(format!("row.{id}"), v.to_string());
            }
        }
        out
    }

    /// The drone's numbers as the engine prints them (E5.4's block, `sf_drone_print`), keyed like
    /// [`oracle_view`]: Hit Points, base attack and the three base saves from the drone class's
    /// lines; the ability scores from the chassis' base scores plus its ability increases (the
    /// chassis' `(Dex)` / `(Wis)` lines); the speeds from the race line. The engine computes no
    /// drone total (`sf_drone_print` module doc), so a total the oracle exports has no key here.
    pub(crate) fn drone_view(input: &CharacterInput) -> BTreeMap<String, String> {
        let package = package().expect("the Starfinder package loads");
        let sheet = compute_sheet(package, input).unwrap_or_else(|r| panic!("{r:?}"));
        let lines: Vec<&SheetLine> = sheet.lines.iter().filter(|l| l.kind == crate::sf_drone_print::COMPANION_KIND).collect();
        let number = |l: &SheetLine| match l.value {
            SheetLineValue::Resolved(n) => Some(i64::from(n)),
            _ => None,
        };
        let mut out = BTreeMap::new();
        for line in &lines {
            let (_, kind, _) = split_rule_id(&line.id);
            if kind == "class" && !line.id.contains('#') {
                out.insert("bab".to_string(), number(line).expect("the drone class line is its base attack").to_string());
            }
            for (suffix, key) in [
                ("(hit points)", "hp"),
                ("(base Fortitude save)", "save.Fortitude.base"),
                ("(base Reflex save)", "save.Reflex.base"),
                ("(base Will save)", "save.Will.base"),
            ] {
                if kind == "class" && line.label.ends_with(suffix) {
                    out.insert(key.to_string(), number(line).expect("a numbered class line").to_string());
                }
            }
            if kind == "race" {
                for (mode, key) in [("Walk", "speed.walk"), ("Fly", "speed.fly")] {
                    if let Some(feet) = line.prose.split(&format!("Speed: {mode} ")).nth(1).and_then(|r| r.split(' ').next()) {
                        out.insert(key.to_string(), feet.to_string());
                    }
                }
            }
            if let Some(scores) = line.prose.split("Base ability scores: ").nth(1) {
                for part in scores.split(", ") {
                    let mut words = part.split_whitespace();
                    let (Some(ability), Some(score)) = (words.next(), words.next().and_then(|w| w.parse::<i64>().ok())) else { continue };
                    // The chassis' ability increase at this drone level (`Hover (Dex)`: +0 at 1st).
                    let increase: i64 = lines
                        .iter()
                        .filter(|l| l.id.starts_with(line.id.as_str()) && l.label.ends_with(&format!("({ability})")))
                        .filter_map(|l| number(l))
                        .sum();
                    out.insert(format!("stat.{}", ability.to_ascii_uppercase()), (score + increase).to_string());
                }
            }
        }
        out
    }

    /// One `explained.tsv` row: `build \t field \t engine \t oracle \t class \t reason`.
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) struct Explained {
        pub build: String,
        pub field: String,
        pub engine: String,
        pub oracle: String,
        pub class: String,
        pub reason: String,
    }

    /// The two ways a difference is explained.
    pub(crate) const ORACLE_DEPARTS: &str = "oracle-departs-from-srd";
    pub(crate) const ENGINE_DOES_NOT_COMPUTE: &str = "engine-does-not-compute";

    pub(crate) fn ledger() -> Vec<Explained> {
        let path = repo_root().join(LEDGER);
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        text.lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .map(|l| {
                let c: Vec<&str> = l.split('\t').collect();
                assert!(c.len() == 6, "{LEDGER}: 6 tab-separated cells expected: {l:?}");
                Explained {
                    build: c[0].into(),
                    field: c[1].into(),
                    engine: c[2].into(),
                    oracle: c[3].into(),
                    class: c[4].into(),
                    reason: c[5].into(),
                }
            })
            .collect()
    }

    /// One build's comparison: every oracle field, the engine's value for it, and the engine's
    /// fields the oracle does not export.
    pub(crate) struct Compared {
        pub build: String,
        pub fields: Vec<(String, String, String)>,
        pub engine_only: Vec<String>,
    }

    pub(crate) const DRONE: &str = "SF-Mechanic-1-drone";

    /// The roster compared (module doc). Writes each build's two views to `SF_PARITY_ENGINE_OUT`
    /// when it is set (`scripts/oracle_harness/sf_parity_check.py` reads them).
    pub(crate) fn compare() -> Vec<Compared> {
        let mut out = Vec::new();
        // (build, oracle file stem, oracle view, engine view)
        type View = (String, String, BTreeMap<String, String>, BTreeMap<String, String>);
        let mut views: Vec<View> = Vec::new();
        for (build, stem, input) in roster() {
            let read = |stem: &str| {
                let path = repo_root().join(ORACLE_DIR).join(format!("{stem}.oracle.txt"));
                std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
            };
            views.push((build.clone(), stem.clone(), oracle_view(&read(&stem)), engine_view(&input)));
            if build == "SF-Mechanic-1" {
                let text = read("sf_mechanic_1_drone");
                // The drone renders only with its master loaded (party mode): the file names the
                // master and its level, which must be this build's.
                assert!(text.lines().any(|l| l == "master=sf_mechanic_1"), "the drone run's master is sf_mechanic_1");
                assert!(text.lines().any(|l| l == "master.level=1") && text.lines().any(|l| l == "master.var.drone_companion_lvl=1"), "a Mechanic 1 master");
                let mut oracle = oracle_view(&text);
                assert_eq!(oracle.remove("level").as_deref(), Some("1"), "drone level 1 (DroneLVL)");
                views.push((DRONE.to_string(), "sf_mechanic_1_drone".into(), oracle, drone_view(&input)));
            }
        }
        for (build, stem, oracle, engine) in views {
            if let Ok(dir) = std::env::var("SF_PARITY_ENGINE_OUT") {
                let dump: BTreeMap<&str, &BTreeMap<String, String>> = BTreeMap::from([("engine", &engine), ("oracle", &oracle)]);
                std::fs::create_dir_all(&dir).expect("dump dir");
                std::fs::write(Path::new(&dir).join(format!("{stem}.json")), serde_json::to_string_pretty(&dump).expect("json") + "\n").expect("writes");
            }
            let fields = oracle
                .iter()
                .map(|(field, want)| (field.clone(), engine.get(field).cloned().unwrap_or_else(|| "absent".into()), want.clone()))
                .collect();
            let engine_only = engine.keys().filter(|k| !oracle.contains_key(*k)).cloned().collect();
            out.push(Compared { build, fields, engine_only });
        }
        out
    }

    /// A build's identity -- class, level, the six scores, each skill's ranks -- is the build
    /// itself, not a total: a difference there means the two sides render different characters,
    /// and no ledger row may explain it.
    fn identity_field(field: &str) -> bool {
        matches!(field, "class" | "level") || field.starts_with("stat.") || field.ends_with(".ranks")
    }

    /// SD-37 E7.1's acceptance: over the whole roster, every difference between the oracle and the
    /// engine is an `explained.tsv` row (exact values), every ledger row is still a difference,
    /// and no difference is in a build's identity.
    #[test]
    fn sf_oracle_parity_every_difference_is_explained() {
        let compared = compare();
        assert_eq!(compared.len(), 15, "4 seeds + 10 level-1 classes + the Mechanic 1's drone");
        let diffs: BTreeSet<(String, String, String, String)> = compared
            .iter()
            .flat_map(|c| c.fields.iter().filter(|(_, e, o)| e != o).map(move |(f, e, o)| (c.build.clone(), f.clone(), e.clone(), o.clone())))
            .collect();
        let ledger = ledger();
        let explained: BTreeSet<(String, String, String, String)> =
            ledger.iter().map(|x| (x.build.clone(), x.field.clone(), x.engine.clone(), x.oracle.clone())).collect();
        for x in &ledger {
            assert!([ORACLE_DEPARTS, ENGINE_DOES_NOT_COMPUTE].contains(&x.class.as_str()), "{LEDGER}: unknown class {x:?}");
            assert!(!identity_field(&x.field), "{LEDGER}: an identity field cannot be explained: {x:?}");
            if x.class == ORACLE_DEPARTS {
                assert!(x.reason.contains("https://"), "{LEDGER}: an oracle departure cites the SRD: {x:?}");
            }
            assert!(!x.reason.trim().is_empty(), "{LEDGER}: a reason: {x:?}");
        }
        let unexplained: Vec<_> = diffs.difference(&explained).collect();
        let stale: Vec<_> = explained.difference(&diffs).collect();
        let identity: Vec<_> = diffs.iter().filter(|d| identity_field(&d.1)).collect();
        let total: usize = compared.iter().map(|c| c.fields.len()).sum();
        eprintln!("sf_oracle_parity: {} builds, {total} oracle fields compared, {} differ, {} explained", compared.len(), diffs.len(), explained.len());
        assert!(identity.is_empty(), "the two sides render different characters: {identity:#?}");
        assert!(unexplained.is_empty(), "{} unexplained differences (build, field, engine, oracle):\n{unexplained:#?}", unexplained.len());
        assert!(stale.is_empty(), "{} ledger rows no longer differ -- remove them:\n{stale:#?}", stale.len());
    }

    /// What the oracle does not contain, as the engine computes it: every engine field of every
    /// build the oracle exports no value for, committed in `not_in_oracle.tsv` and kept equal to
    /// the computation (`SF_PARITY_NOT_IN_ORACLE_WRITE=1` rewrites it).
    #[test]
    fn sf_oracle_parity_the_not_in_oracle_list_is_current() {
        let mut text = String::from("# SD-37 E7.1: engine fields the oracle exports no value for (build \\t field). Written by\n# SF_PARITY_NOT_IN_ORACLE_WRITE=1 cargo test --bin codex-desktop sf_oracle_parity_the_not_in_oracle_list_is_current\n");
        for c in compare() {
            for f in &c.engine_only {
                text.push_str(&format!("{}\t{f}\n", c.build));
            }
        }
        let path = repo_root().join(ORACLE_DIR).join("not_in_oracle.tsv");
        if std::env::var_os("SF_PARITY_NOT_IN_ORACLE_WRITE").is_some() {
            std::fs::write(&path, &text).expect("writes");
        }
        assert_eq!(std::fs::read_to_string(&path).unwrap_or_default(), text, "{} is stale; rewrite it with SF_PARITY_NOT_IN_ORACLE_WRITE=1", path.display());
    }
}
