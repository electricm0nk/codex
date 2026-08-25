//! v0.6 alpha swarm: engine-derived per-book *content* state dump for the
//! operator's status dashboard.
//!
//! Run via `cargo run --bin v06_content_state_dump`; emits JSON on stdout.
//!
//! **Why this exists.** `v06_class_state_dump` fixed the 27-class matrix by
//! making it read the engine instead of hand-written prose. The rest of the
//! dashboard still did not: the per-book doneness matrix
//! (`races`/`classes`/`spells`/`equipment`/`feats`) was a hand-set literal in
//! `pf1e_dashboard_producer.py`, the Bestiary 1 panel emitted 41 bare monster
//! name strings with no state field at all (so the viewer rendered all 41 as
//! "Not started" — the operator's 2026-07-29 report), and the feats column
//! carried a hand-typed "16 of 185" that had gone stale by more than a
//! factor of four.
//!
//! Everything this binary emits is counted out of the real compiled tables
//! (`ClassId::ALL`, `SPELL_LIST`, `equipment_tables()`, `MonsterId::ALL`,
//! `all_feat_tables()`) or observed by running the real compute pipeline.
//! Nothing here is a literal a human has to remember to update: a book that
//! gains or loses records changes this output with no edit to this file.
//!
//! **Relationship to `corpus_ingest_diagnostic.rs`.** That module
//! (`apps/desktop/src-tauri`) already counts the same tables for the app's
//! own diagnostic surface, but it lives behind a `#[tauri::command]` in a
//! separate cargo workspace — building it to answer a dashboard tick would
//! mean building the whole desktop app. This binary is the same counting
//! discipline reachable from the `codex` crate alone, plus the two things
//! the diagnostic does not carry: the per-monster stat-block detail the
//! Bestiary panel needs, and the behavioural feat-effect probe.
//!
//! This binary is an operator/ops surface (like `gen_cache_acg` and
//! `v06_class_state_dump`), not an app runtime surface — nothing in the
//! shipped app calls it.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use codex::rules_core::character_input::{CharacterClassLevel, CharacterInput, SelectedChoice};
use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus, PilotBaseChassisComputation, build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::rules_tables::acg::{self, AcgClassId};
use codex::rules_core::rules_tables::apg::{self, ApgClassId};
use codex::rules_core::rules_tables::beastiary1::{self, MonsterId, MonsterStatBlock};
use codex::rules_core::rules_tables::crb::{
    class_tables::ClassId, equipment_tables as crb_equipment_tables, feats as crb_feats,
    race_tables::{RaceId, race_id_from_token, race_traits},
    spell_list as crb_spell_list,
};
use codex::rules_core::rules_tables::advanced_race_guide as arg;
use codex::rules_core::rules_tables::feats_all::all_feat_tables;
use codex::rules_core::rules_tables::pathfinder_unchained as pu;
use codex::rules_core::rules_tables::pathfinder_unchained::class_chassis::PuClassId;
use codex::rules_core::rules_tables::ultimate_campaign as uca;
use codex::rules_core::rules_tables::ultimate_equipment as ue;
use codex::rules_core::rules_tables::ultimate_combat as uc;
use codex::rules_core::rules_tables::ultimate_magic as um;
use codex::rules_core::rules_tables::ultimate_psionics as upsi;
use codex::rules_core::rules_tables::ultimate_wilderness as uw;
use codex::rules_core::rules_tables::ultimate_intrigue as ui;
use codex::rules_core::rules_tables::RuleSetId;

/// The shared deterministic pilot input fixture, relative to the crate root.
/// Read at runtime rather than `include_str!`ed, exactly as
/// `v06_class_state_dump` does and for the same reason (a `src/` target
/// should not bake a `tests/` asset into itself).
const FIXTURE_RELATIVE_PATH: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

/// The class postures the feat-effect probe sweeps. A feat whose bonus is
/// gated on a class resource (Extra Rage, Extra Ki, Extra Panache) can only
/// show up as a computed change on a character who *has* that resource, so a
/// single-class probe would systematically undercount. These five cover the
/// distinct resource shapes this engine models today: martial baseline
/// (fighter), rage (barbarian), ki/unarmed (monk), prepared arcane casting
/// (wizard), and panache (swashbuckler).
///
/// Deliberately a small, named set rather than all 27 classes: the probe is
/// a *lower bound* on wiring either way (see `probe_feat_effect_wiring`), and
/// each added posture multiplies the run cost that a cron-driven dashboard
/// tick has to pay.
const PROBE_CLASSES: &[&str] = &["fighter", "barbarian", "monk", "wizard", "swashbuckler"];

/// The levels each probe posture is evaluated at. Level 1 is the creation
/// posture the operator's question is usually about; level 12 is high enough
/// that level-gated feat effects (Greater Weapon Focus needs Fighter 8,
/// Greater Weapon Specialization needs Fighter 12) are actually reachable.
const PROBE_LEVELS: &[u8] = &[1, 12];

/// Generic `selection_id` values the probe pairs with a feat's own derived
/// choice-set id. Feats whose effect depends on a player-chosen target
/// (Weapon Focus, Skill Focus, Spell Focus, Improved Critical, Weapon
/// Specialization) read that target off `selected_choices`, so appending the
/// feat key alone would report every one of them as unwired.
///
/// The choice-set id is *derived by the engine's own naming rule*
/// (`choice:<snake_case(feat key)>_target`, e.g. `choice:weapon_focus_target`,
/// `choice:skill_focus_target`, `choice:greater_spell_focus_target`) rather
/// than read off a per-feat lookup table — a hand-maintained mapping here
/// would be exactly the kind of list that silently goes stale, which is the
/// whole disease this binary exists to cure. A feat that invents a new
/// choice-set naming shape falls out of the probe and is reported as
/// unwired, which is the honest failure direction.
const PROBE_SELECTIONS: &[&str] = &[
    "weapon:Longsword",
    "skill:Climb",
    "school:evocation",
    "feat:dodge",
];

/// One CRB race's real, engine-observed state.
struct RaceState {
    id: String,
    /// Every level in [`PROBE_LEVELS`] reaches `Computed` for a Fighter of
    /// this race.
    computed: bool,
    /// Real per-race explanation records this engine grounds
    /// (`race_tables::race_traits()`), never a hand-kept tally.
    trait_records: usize,
}

/// One content kind's real ingested record count for one book.
struct KindCount {
    kind: &'static str,
    ingested: u32,
}

/// One book's real ingested content, counted from its own compiled tables.
struct BookContent {
    id: &'static str,
    kinds: Vec<KindCount>,
}

/// One Bestiary 1 monster's real, engine-resolved state.
struct MonsterState {
    key: String,
    stat_block: MonsterStatBlock,
    /// The JSON corpus cache record for this monster exists on disk.
    corpus_record: bool,
}

fn crb_content() -> BookContent {
    BookContent {
        id: "core_rulebook",
        kinds: vec![
            KindCount { kind: "races", ingested: RaceId::ALL.len() as u32 },
            KindCount { kind: "classes", ingested: ClassId::ALL.len() as u32 },
            KindCount { kind: "spells", ingested: crb_spell_list::SPELL_LIST.len() as u32 },
            KindCount {
                kind: "equipment",
                ingested: crb_equipment_tables::equipment_tables().len() as u32,
            },
            KindCount { kind: "feats", ingested: crb_feats::feat_tables().len() as u32 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

fn apg_content() -> BookContent {
    BookContent {
        id: "advanced_players_guide",
        kinds: vec![
            // The APG ships no race table in this codebase; the seven CRB
            // races are the whole ingested race roster (see
            // `aggregate_races_state`'s own note in the producer). Reported
            // as a real 0 rather than omitted, so the matrix can tell
            // "nothing ingested" from "kind not applicable".
            KindCount { kind: "races", ingested: 0 },
            KindCount { kind: "classes", ingested: ApgClassId::ALL.len() as u32 },
            KindCount { kind: "spells", ingested: apg::spell_list::SPELL_LIST.len() as u32 },
            KindCount {
                kind: "equipment",
                ingested: apg::equipment_tables::EQUIPMENT_TABLE.len() as u32,
            },
            KindCount { kind: "feats", ingested: apg::feats::feat_tables().len() as u32 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

fn acg_content() -> BookContent {
    BookContent {
        id: "advanced_class_guide",
        kinds: vec![
            KindCount { kind: "races", ingested: 0 },
            KindCount { kind: "classes", ingested: AcgClassId::ALL.len() as u32 },
            KindCount { kind: "spells", ingested: acg::spell_list::SPELL_LIST.len() as u32 },
            KindCount {
                kind: "equipment",
                ingested: acg::equipment_tables::equipment_tables().len() as u32,
            },
            KindCount { kind: "feats", ingested: acg::feats::feat_tables().len() as u32 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

/// SD-27. These two were missing from the emitted list even though the engine
/// has compiled their tables since SD-27 landed, so every consumer of this
/// dump — the dashboard included — reported four books and silently omitted
/// two. Same defect shape as `v06_work_inventory::rule_set_for`: the
/// `RuleSetId` match below is exhaustive and was forced to grow, but this
/// hand-listed roster was not, and nothing failed.
fn arg_content() -> BookContent {
    BookContent {
        id: "advanced_race_guide",
        kinds: vec![
            // ARG declares zero races and zero racial defaults
            // (`decisions.md §25`) — a measured zero, asserted by
            // `race_catalog.rs`. Its race work is 153 alternate racial traits,
            // which are deliberately not race rows.
            KindCount { kind: "races", ingested: 0 },
            KindCount {
                kind: "classes",
                ingested: arg::class_spell_levels::ARG_CLASS_SPELL_LEVELS.len() as u32,
            },
            KindCount { kind: "spells", ingested: arg::spell_list::SPELL_LIST.len() as u32 },
            KindCount {
                kind: "equipment",
                ingested: arg::equipment_tables::equipment_tables().len() as u32,
            },
            KindCount { kind: "feats", ingested: arg::feats::feat_tables().len() as u32 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

fn pu_content() -> BookContent {
    BookContent {
        id: "pathfinder_unchained",
        kinds: vec![
            KindCount { kind: "races", ingested: 0 },
            KindCount { kind: "classes", ingested: PuClassId::ALL.len() as u32 },
            KindCount { kind: "spells", ingested: 0 },
            KindCount {
                kind: "equipment",
                ingested: pu::equipment_tables::equipment_tables().len() as u32,
            },
            KindCount { kind: "feats", ingested: pu::feat_tables::feat_tables().len() as u32 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

/// SD28-E13. Same hand-listed-roster shape `arg_content()`/`pu_content()`'s
/// own doc comment warns about: the `RuleSetId` match below is exhaustive
/// and was forced to grow a `Uca` arm, but this roster is not
/// compiler-enforced, so it is added explicitly here rather than trusted to
/// follow automatically.
fn uca_content() -> BookContent {
    BookContent {
        id: "ultimate_campaign",
        kinds: vec![
            KindCount { kind: "races", ingested: 0 },
            KindCount { kind: "classes", ingested: 0 },
            KindCount { kind: "spells", ingested: 0 },
            KindCount { kind: "equipment", ingested: 0 },
            KindCount { kind: "feats", ingested: uca::feat_tables::feat_tables().len() as u32 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

/// SD28-E24. Same hand-listed-roster shape `uca_content()`'s own doc
/// comment warns about -- the `RuleSetId` match below is exhaustive and
/// was forced to grow a `Ui` arm, but this roster is not
/// compiler-enforced, so it is added explicitly here rather than trusted
/// to follow automatically. Slices 1-2: feats, spells, equipment. (Caught
/// while wiring UE: this roster was never updated when slice 2 landed
/// spell/equipment -- fixed here.) `class_feature`/races/classes remain
/// real, verified zeros, not yet ingested.
fn ui_content() -> BookContent {
    BookContent {
        id: "ultimate_intrigue",
        kinds: vec![
            KindCount { kind: "races", ingested: 0 },
            KindCount { kind: "classes", ingested: 0 },
            KindCount { kind: "spells", ingested: ui::spell_list::SPELL_LIST.len() as u32 },
            KindCount {
                kind: "equipment",
                ingested: (ui::equipment_tables::equipment_tables().len()
                    + ui::equipment_tables::equipmod_tables().len()) as u32,
            },
            KindCount { kind: "feats", ingested: ui::feat_tables::feat_tables().len() as u32 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

/// SD28-E25. Same hand-listed-roster shape `ui_content()` above warns
/// about. First slice: equipment only (no feats file exists in this
/// book -- see `ultimate_equipment::equipment_tables`'s own doc comment).
fn ue_content() -> BookContent {
    BookContent {
        id: "ultimate_equipment",
        kinds: vec![
            KindCount { kind: "races", ingested: 0 },
            KindCount { kind: "classes", ingested: 0 },
            KindCount { kind: "spells", ingested: 0 },
            KindCount {
                kind: "equipment",
                ingested: (ue::equipment_tables::equipment_tables().len()
                    + ue::equipment_tables::equipmod_tables().len()) as u32,
            },
            KindCount { kind: "feats", ingested: 0 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

/// SD28-E26. Same hand-listed-roster shape the sibling `*_content()`
/// functions above warn about. First slice: feats only.
fn uw_content() -> BookContent {
    BookContent {
        id: "ultimate_wilderness",
        kinds: vec![
            KindCount { kind: "races", ingested: 0 },
            KindCount { kind: "classes", ingested: 0 },
            KindCount { kind: "spells", ingested: 0 },
            KindCount { kind: "equipment", ingested: 0 },
            KindCount { kind: "feats", ingested: uw::feat_tables::feat_tables().len() as u32 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

/// SD28-E27. Same hand-listed-roster shape the sibling `*_content()`
/// functions above warn about. First slice: feats only.
fn uc_content() -> BookContent {
    BookContent {
        id: "ultimate_combat",
        kinds: vec![
            KindCount { kind: "races", ingested: 0 },
            KindCount { kind: "classes", ingested: 0 },
            KindCount { kind: "spells", ingested: 0 },
            KindCount { kind: "equipment", ingested: 0 },
            KindCount { kind: "feats", ingested: uc::feat_tables::feat_tables().len() as u32 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

/// SD28-E28/E15. Same hand-listed-roster shape the sibling `*_content()`
/// functions above warn about. SD28-E15 adds the equipment slice (26
/// records: 24 General + 2 ArmsArmor) alongside the earlier feat catalog.
fn um_content() -> BookContent {
    BookContent {
        id: "ultimate_magic",
        kinds: vec![
            KindCount { kind: "races", ingested: 0 },
            KindCount { kind: "classes", ingested: 0 },
            KindCount { kind: "spells", ingested: 0 },
            KindCount {
                kind: "equipment",
                ingested: um::equipment_tables::equipment_tables().len() as u32,
            },
            KindCount { kind: "feats", ingested: um::feat_tables::feat_tables().len() as u32 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

/// SD28-E29/E15. Same hand-listed-roster shape the sibling `*_content()`
/// functions above warn about. SD28-E15 adds the equipment slice (552
/// records: 326 equipment + 226 equipmods) alongside the earlier feat
/// catalog.
fn upsi_content() -> BookContent {
    BookContent {
        id: "ultimate_psionics",
        kinds: vec![
            KindCount { kind: "races", ingested: 0 },
            KindCount { kind: "classes", ingested: 0 },
            KindCount { kind: "spells", ingested: 0 },
            KindCount {
                kind: "equipment",
                ingested: (upsi::equipment_tables::equipment_tables().len()
                    + upsi::equipment_tables::equipmod_tables().len())
                    as u32,
            },
            KindCount { kind: "feats", ingested: upsi::feat_tables::feat_tables().len() as u32 },
            KindCount { kind: "monsters", ingested: 0 },
        ],
    }
}

fn bestiary1_content() -> BookContent {
    BookContent {
        id: "bestiary_1",
        kinds: vec![
            // Bestiary 1 has no PC races, no PC classes, and — per
            // `beastiary1/mod.rs`'s own register A13 finding — no spell-list
            // concept at all. All three are a real, verified zero, which is
            // what lets the matrix show "O" for them instead of inheriting
            // some other book's aggregate.
            KindCount { kind: "races", ingested: 0 },
            KindCount { kind: "classes", ingested: 0 },
            KindCount { kind: "spells", ingested: 0 },
            KindCount {
                kind: "equipment",
                ingested: beastiary1::equipment_tables::EQUIPMENT_TABLE.len() as u32,
            },
            KindCount { kind: "feats", ingested: 0 },
            KindCount { kind: "monsters", ingested: MonsterId::ALL.len() as u32 },
        ],
    }
}

/// The `beastiary1:monster:<lowercase-name>` corpus key for a monster, built
/// from the stat block's own real name rather than from a second hand-kept
/// list — the same key shape `monster_key_resolve` already resolves and
/// `cache_gen::beastiary1` already writes.
fn monster_key(name: &str) -> String {
    format!(
        "beastiary1:monster:{}",
        name.to_lowercase().replace(['-', ' '], "_")
    )
}

/// Every Bestiary 1 monster's real engine state, resolved through the real
/// `monster_resolve` entry point. A monster that failed to resolve would be
/// omitted here rather than emitted with fabricated fields, so the dashboard
/// count can never exceed the number of monsters that genuinely resolve.
fn monster_states(repo_root: &Path) -> Vec<MonsterState> {
    let corpus_dir = repo_root.join("data/corpus/beastiary/monster");
    MonsterId::ALL
        .iter()
        .filter_map(|&id| {
            let stat_block = beastiary1::monster_resolve(id, RuleSetId::Bestiary1)?;
            let key = monster_key(&stat_block.name);
            let file_stem = key.rsplit(':').next().unwrap_or_default().to_string();
            let corpus_record = corpus_dir.join(format!("{file_stem}.json")).exists();
            Some(MonsterState { key, stat_block, corpus_record })
        })
        .collect()
}

/// The corpus race-id token for a `RaceId`, exhaustively matched so a race
/// added to `RaceId::ALL` cannot be silently omitted from the dashboard (the
/// same compiler-enforced discipline `v06_class_state_dump::crb_class_name`
/// uses). Round-tripped through the engine's own `race_id_from_token` in
/// [`race_states`], so a token that stopped resolving is a hard failure here
/// rather than a race that quietly reports "blocked".
fn race_token(race: RaceId) -> &'static str {
    match race {
        RaceId::Human => "race:human",
        RaceId::Dwarf => "race:dwarf",
        RaceId::Elf => "race:elf",
        RaceId::Gnome => "race:gnome",
        RaceId::HalfElf => "race:half-elf",
        RaceId::HalfOrc => "race:half-orc",
        RaceId::Halfling => "race:halfling",
    }
}

/// Every CRB race's real engine state, observed by running the real compute
/// pipeline for a Fighter of that race at each [`PROBE_LEVELS`] level.
///
/// This replaces the dashboard's previous race source, which was a regex
/// scrape of hand-written prose in `risks-and-open-questions.md` — the same
/// prose-drift disease `v06_class_state_dump` already cured for classes.
fn race_states(fixture: &CharacterInput) -> Vec<RaceState> {
    RaceId::ALL
        .iter()
        .map(|&race| {
            let token = race_token(race);
            assert_eq!(
                race_id_from_token(token),
                Some(race),
                "race token {token} must round-trip through the engine's own resolver"
            );
            let trait_records = race_traits()
                .iter()
                .filter(|t| t.race_id == race)
                .count();
            let computed = PROBE_LEVELS.iter().all(|&level| {
                let mut input = posture_input(fixture, "fighter", level);
                input.chosen.race_id = token.to_owned();
                build_pilot_headless_receipt(&input).status == HeadlessReceiptStatus::Computed
            });
            RaceState {
                id: token.trim_start_matches("race:").to_owned(),
                computed,
                trait_records,
            }
        })
        .collect()
}

/// `"Greater Weapon Focus"` -> `"choice:greater_weapon_focus_target"`.
fn derived_choice_set_id(feat_key: &str) -> String {
    let slug: String = feat_key
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect();
    format!("choice:{slug}_target")
}

/// The computed facts a feat is allowed to move, with the two fields that
/// change merely because a feat was *recorded* deliberately excluded.
///
/// `explanations` is included (a feat that adds a new explanation line has a
/// real computed consequence) but `diagnostics` is not: a feat whose
/// prerequisites this character fails makes the engine emit a diagnostic
/// without computing anything, and counting that as "wired" would inflate the
/// number with exactly the feats that do nothing.
fn observable_facts(computation: &PilotBaseChassisComputation) -> (Vec<(String, i16)>, [i16; 12]) {
    let explanations = computation
        .explanations
        .iter()
        .map(|e| (e.id.clone(), e.value))
        .collect();
    let numbers = [
        computation.ability_modifiers.strength,
        computation.ability_modifiers.dexterity,
        computation.ability_modifiers.constitution,
        computation.base_attack_bonus,
        computation.base_saves.fortitude,
        computation.base_saves.reflex,
        computation.base_saves.will,
        computation.baseline_melee_attack_bonus,
        computation.baseline_armor_class,
        computation.total_saves.fortitude,
        computation.total_saves.reflex,
        computation.total_saves.will,
    ];
    (explanations, numbers)
}

/// The real production-shaped input for one class at one level, exactly as
/// `v06_class_state_dump::input_for` builds it (the fixture's own loadout,
/// untouched) minus that binary's class-conditional canonical seeds, which
/// this binary's two probes do not need.
fn posture_input(fixture: &CharacterInput, class_name: &str, level: u8) -> CharacterInput {
    let mut input = fixture.clone();
    input.case_id = Some(format!("v06_content_state_dump.{class_name}.level{level}"));
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: format!("class:{class_name}"),
        level,
    }];
    input
}

/// The feat probe's baseline posture: [`posture_input`] with the fixture's
/// three pre-granted feats stripped.
///
/// **Why stripped.** The fixture ships `feat:power_attack`, `feat:dodge` and
/// `feat:weapon_focus` plus the `choice:human_bonus_feat` /
/// `choice:fighter_bonus_feat` selections that grant them. Probing a feat
/// whose bonus the baseline character *already has* measures nothing —
/// Dodge's +1 AC and Weapon Focus' +1 attack are both already applied and
/// neither stacks with itself — so those feats would report as unwired even
/// though this engine computes them. Clearing the grants first is what makes
/// the probe measure the feat rather than the fixture.
///
/// Used **only** by the feat probe, which compares two computations against
/// each other and so does not care whether either one reaches `Computed`.
/// The race probe deliberately keeps the full fixture loadout, because
/// stripping a Fighter's bonus-feat selection genuinely blocks the chassis
/// and would report every race as blocked for a reason that has nothing to
/// do with the race.
///
/// The stripped ids are matched by the `feat:` synthetic-id prefix and the
/// `_bonus_feat`/`_character_feat` choice-set suffix, not by name, so a
/// future fixture that grants a different feat is covered without an edit
/// here.
fn feat_probe_input(fixture: &CharacterInput, class_name: &str, level: u8) -> CharacterInput {
    let mut input = posture_input(fixture, class_name, level);
    input.chosen.selected_feats.retain(|f| !f.starts_with("feat:"));
    input.chosen.selected_choices.retain(|c| {
        !(c.choice_set_id.ends_with("_bonus_feat") || c.choice_set_id.ends_with("_character_feat"))
    });
    input
}

/// Every catalog feat key whose presence genuinely changes what this engine
/// computes, observed by running the real compute pipeline twice — once
/// without the feat and once with it — across [`PROBE_CLASSES`] x
/// [`PROBE_LEVELS`], and taking the union.
///
/// **This is a lower bound, and says so.** A feat whose effect needs a
/// posture outside the swept set (a class not in `PROBE_CLASSES`, a
/// choice-set naming shape outside `derived_choice_set_id`'s rule, an
/// opponent, an ally, or a combat action this engine does not model) reads
/// as unwired here even if some code somewhere mentions it. That is the
/// honest direction to be wrong in for a doneness dashboard, and it is the
/// direction the *previous* hand-typed "16 of 185" was wrong in too — the
/// difference is that this number re-derives itself every time the engine
/// changes, and that one did not.
fn probe_feat_effect_wiring(fixture: &CharacterInput) -> BTreeSet<String> {
    let mut wired = BTreeSet::new();
    let keys: BTreeSet<&'static str> = all_feat_tables()
        .iter()
        .flat_map(|table| table.entries.iter().map(|entry| entry.key))
        .collect();

    for class_name in PROBE_CLASSES {
        for &level in PROBE_LEVELS {
            let base_input = feat_probe_input(fixture, class_name, level);
            let baseline = observable_facts(&compute_pilot_base_chassis(&base_input));

            for &key in &keys {
                if wired.contains(key) {
                    continue;
                }
                let choice_set = derived_choice_set_id(key);
                // Variant 0 is the bare feat; the rest pair it with one
                // generic target selection each. One selection per run (not
                // all four at once) so a producer that reads "the first
                // selection in this set" cannot be handed a target it does
                // not understand and stay silent.
                for variant in 0..=PROBE_SELECTIONS.len() {
                    let mut input = base_input.clone();
                    input.chosen.selected_feats.push(key.to_string());
                    if variant > 0 {
                        input.chosen.selected_choices.push(SelectedChoice {
                            choice_set_id: choice_set.clone(),
                            selection_id: PROBE_SELECTIONS[variant - 1].to_string(),
                        });
                    }
                    let observed = observable_facts(&compute_pilot_base_chassis(&input));
                    if observed != baseline {
                        wired.insert(key.to_string());
                        break;
                    }
                }
            }
        }
    }
    wired
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn real_now_iso8601() -> String {
    let output = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("system `date` must be available to stamp generated_at");
    String::from_utf8(output.stdout)
        .expect("date output is valid UTF-8")
        .trim()
        .to_string()
}

fn main() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let fixture_text = match std::fs::read_to_string(&fixture_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("could not read {}: {e}", fixture_path.display());
            std::process::exit(1);
        }
    };
    let fixture = match load_character_input_fixture(&fixture_text).character_input {
        Some(input) => input,
        None => {
            eprintln!("fixture {} did not load", fixture_path.display());
            std::process::exit(1);
        }
    };

    let books = [
        crb_content(),
        apg_content(),
        acg_content(),
        bestiary1_content(),
        arg_content(),
        pu_content(),
        uca_content(),
        ui_content(),
        ue_content(),
        uw_content(),
        uc_content(),
        um_content(),
        upsi_content(),
    ];
    let monsters = monster_states(&repo_root);
    let races = race_states(&fixture);
    let wired = probe_feat_effect_wiring(&fixture);

    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"generated_at\": \"{}\",\n", real_now_iso8601()));
    out.push_str("  \"generated_by\": \"v06_content_state_dump\",\n");
    out.push_str("  \"schema_version\": 1,\n");

    out.push_str("  \"books\": [\n");
    for (i, book) in books.iter().enumerate() {
        out.push_str("    {\n");
        out.push_str(&format!("      \"id\": \"{}\",\n", book.id));
        out.push_str("      \"kinds\": {\n");
        for (j, kind) in book.kinds.iter().enumerate() {
            out.push_str(&format!(
                "        \"{}\": {}{}\n",
                kind.kind,
                kind.ingested,
                if j + 1 < book.kinds.len() { "," } else { "" }
            ));
        }
        out.push_str("      }\n");
        out.push_str(if i + 1 < books.len() { "    },\n" } else { "    }\n" });
    }
    out.push_str("  ],\n");

    out.push_str("  \"races\": [\n");
    for (i, r) in races.iter().enumerate() {
        out.push_str("    {\n");
        out.push_str(&format!("      \"id\": \"{}\",\n", json_escape(&r.id)));
        out.push_str(&format!("      \"computed\": {},\n", r.computed));
        out.push_str(&format!("      \"trait_records\": {}\n", r.trait_records));
        out.push_str(if i + 1 < races.len() { "    },\n" } else { "    }\n" });
    }
    out.push_str("  ],\n");

    out.push_str("  \"monsters\": [\n");
    for (i, m) in monsters.iter().enumerate() {
        let attacks: Vec<String> = m
            .stat_block
            .natural_attacks
            .iter()
            .map(|a| format!("{} {}", a.name, a.damage_dice))
            .collect();
        out.push_str("    {\n");
        out.push_str(&format!("      \"key\": \"{}\",\n", json_escape(&m.key)));
        out.push_str(&format!("      \"name\": \"{}\",\n", json_escape(&m.stat_block.name)));
        out.push_str(&format!(
            "      \"challenge_rating\": {},\n",
            m.stat_block.challenge_rating
        ));
        out.push_str(&format!("      \"size\": \"{}\",\n", json_escape(&m.stat_block.size)));
        out.push_str(&format!("      \"speed_ft\": {},\n", m.stat_block.speed_ft));
        out.push_str(&format!(
            "      \"race_type\": \"{}\",\n",
            json_escape(&m.stat_block.race_type)
        ));
        out.push_str(&format!(
            "      \"race_subtype\": {},\n",
            match &m.stat_block.race_subtype {
                Some(s) => format!("\"{}\"", json_escape(s)),
                None => "null".to_string(),
            }
        ));
        out.push_str(&format!(
            "      \"source_page\": \"{}\",\n",
            json_escape(&m.stat_block.source_page)
        ));
        out.push_str(&format!(
            "      \"natural_attacks\": [{}],\n",
            attacks
                .iter()
                .map(|a| format!("\"{}\"", json_escape(a)))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str("      \"engine_stat_block\": true,\n");
        out.push_str(&format!("      \"corpus_record\": {}\n", m.corpus_record));
        out.push_str(if i + 1 < monsters.len() { "    },\n" } else { "    }\n" });
    }
    out.push_str("  ],\n");

    out.push_str("  \"feat_effects\": {\n");
    out.push_str("    \"probe\": \"selected_feats delta over compute_pilot_base_chassis\",\n");
    out.push_str(&format!(
        "    \"probe_postures\": [{}],\n",
        PROBE_CLASSES
            .iter()
            .flat_map(|c| PROBE_LEVELS.iter().map(move |l| format!("\"{c} L{l}\"")))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str("    \"per_book\": {\n");
    let tables = all_feat_tables();
    for (i, table) in tables.iter().enumerate() {
        let book_id = match table.rule_set {
            RuleSetId::Crb => "core_rulebook",
            RuleSetId::Apg => "advanced_players_guide",
            RuleSetId::Acg => "advanced_class_guide",
            RuleSetId::Bestiary1 => "bestiary_1",
            // SD-27: `all_feat_tables()` now yields ARG and PU tables too.
            RuleSetId::Arg => "advanced_race_guide",
            RuleSetId::Pu => "pathfinder_unchained",
            RuleSetId::Uca => "ultimate_campaign",
            RuleSetId::Ui => "ultimate_intrigue",
            // UE has no feats file at all (`decisions.md`/`epic-25`'s own
            // corpus-shape note) -- this arm exists only for exhaustiveness;
            // `all_feat_tables()` never yields a `Ue` table.
            RuleSetId::Ue => "ultimate_equipment",
            RuleSetId::Uw => "ultimate_wilderness",
            RuleSetId::Uc => "ultimate_combat",
            RuleSetId::Um => "ultimate_magic",
            RuleSetId::Upsi => "ultimate_psionics",
            RuleSetId::BonusBestiary => "bonus_bestiary",
            RuleSetId::MonsterCodex => "monster_codex",
            // Inner Sea Races has no feats file this repo compiles either; the
            // arm exists for exhaustiveness, exactly as `Ue`'s above does.
            RuleSetId::Isr => "inner_sea_races",
            // Horror Adventures has no feats file this repo compiles either;
            // the arm exists for exhaustiveness, as `Isr`'s and `Ue`'s do.
            RuleSetId::Ha => "horror_adventures",
            RuleSetId::Ce => "core_essentials",
            RuleSetId::Iswg => "inner_sea_world_guide",
            RuleSetId::Botd1 => "book_of_the_damned_volume_1",
            RuleSetId::Botd2 => "book_of_the_damned_volume_2",
            // SD-29 Epic 7 (companion lane). Neither book has a feats file this
            // repo compiles; both arms exist for exhaustiveness, exactly as
            // `Ha`'s, `Isr`'s and `Ue`'s above do. This match going red is the
            // enum doing its designed job -- `decisions.md §45.2` records the
            // same thing happening when `Isr` was added.
            //
            // **They were missing on `origin/tranche/9` at `bac2f569`**, which
            // is a compile error in this bin and therefore `0 passed across 0
            // suites` for the whole `root-full` stage -- the failure mode
            // `AGENTS.md` records as "one broken bin meant 0 of 502 suites
            // ran". Both lanes found it and both wrote these arms; the merge
            // keeps one copy and both reasons (`§46.6` rule 1).
            RuleSetId::Isc => "inner_sea_combat",
            RuleSetId::Isi => "inner_sea_intrigue",
            // SD-29 Epic 7 round 2. Same disposition as `Isc`/`Isi` above: no
            // feats file this repo compiles, so these three arms exist for
            // exhaustiveness only. Written in the same commit that adds the
            // variants, because a missing arm here is a compile error in this
            // bin and therefore `0 passed across 0 suites` for the entire
            // `root-full` stage.
            RuleSetId::B5 => "bestiary_5",
            RuleSetId::B6 => "bestiary_6",
            RuleSetId::B2 => "bestiary_2",
            RuleSetId::B3 => "bestiary_3",
            RuleSetId::B4 => "bestiary_4",
            // SD-29 Epic 5 extend, round 7. No feats file this repo compiles;
            // the arm exists for exhaustiveness, as `B4`'s above does.
            RuleSetId::Isb => "inner_sea_bestiary",
            RuleSetId::Isg => "inner_sea_gods",
            // SD31-E6-F2-003. Occult Adventures has no feats file this repo
            // compiles either; the arm exists for exhaustiveness, exactly as
            // `Isg`'s and `Isb`'s above do -- `all_feat_tables()` never
            // yields an `Oa` table.
            RuleSetId::Oa => "occult_adventures",
            // SD31-E6-F2-007 -- this book's first compiled rule set of any
            // kind. Unlike `Oa` above, `all_feat_tables()` DOES yield a
            // `Mythic` table (358 gap rows from `ma_feats.lst`), so this arm
            // is real, not exhaustiveness-only.
            RuleSetId::Mythic => "mythic_adventures",
            // SD-31 wave-29 (`lane5-book-onboard` lane). Adventurer's Guide
            // has no feats file this repo compiles yet (its first record
            // family is `spell_list`, not a feat table); the arm exists
            // for exhaustiveness only, exactly as `Oa`'s above does --
            // `all_feat_tables()` never yields an `AdventurersGuide` table.
            RuleSetId::AdventurersGuide => "adventurers_guide",
            // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-
            // onboarding-precondition`, AT-32-G0-003). Inner Sea Faiths,
            // Inner Sea Magic and Inner Sea Temples have no feats file this
            // repo compiles (their first record family is `spell_list`,
            // not a feat table); those three arms exist for exhaustiveness
            // only, exactly as `AdventurersGuide`'s above does. Inner Sea
            // Taverns' first family IS feat (`istav_feats.lst`'s 9 base
            // declarations via `feat_gap_tables`), so `all_feat_tables()`
            // DOES yield an `InnerSeaTaverns` table, the same shape
            // `Mythic`'s arm above documents.
            RuleSetId::InnerSeaFaiths => "inner_sea_faiths",
            RuleSetId::InnerSeaMagic => "inner_sea_magic",
            RuleSetId::InnerSeaTaverns => "inner_sea_taverns",
            RuleSetId::InnerSeaTemples => "inner_sea_temples",
        };
        let records = table.entries.len();
        let wired_here = table
            .entries
            .iter()
            .filter(|e| wired.contains(e.key))
            .map(|e| e.key)
            .collect::<BTreeSet<_>>();
        out.push_str(&format!("      \"{book_id}\": {{\n"));
        out.push_str(&format!("        \"records\": {records},\n"));
        out.push_str(&format!("        \"effect_wired\": {},\n", wired_here.len()));
        out.push_str(&format!(
            "        \"effect_wired_keys\": [{}]\n",
            wired_here
                .iter()
                .map(|k| format!("\"{}\"", json_escape(k)))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(if i + 1 < tables.len() { "      },\n" } else { "      }\n" });
    }
    out.push_str("    }\n");
    out.push_str("  }\n");
    out.push_str("}\n");

    print!("{out}");
}
