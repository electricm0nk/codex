//! v0.6 alpha swarm: engine-derived per-class state dump for the operator's
//! status dashboard.
//!
//! Run via `cargo run --bin v06_class_state_dump`; emits JSON on stdout.
//!
//! **Why this exists.** The operator's dashboard
//! (`~/swarm-observer/PF1e-dashboard.html`) used to derive its 27-class
//! status matrix by regex-scraping hand-written English prose out of
//! `docs/release/v0.6/SWARM_REPORT.md`. Prose drifts the moment somebody
//! forgets to edit it, so the matrix could never be trusted. The engine
//! already knows the truth: a class either reaches
//! `HeadlessReceiptStatus::Computed` through the real compute pipeline or
//! it does not, and when it does not, the claim-blocking diagnostics name
//! the specific remaining gap. This binary dumps exactly that, so the
//! dashboard reads code instead of prose.
//!
//! **Input posture.** Every class is run through the *real fixed loadout*
//! `apps/desktop/src-tauri/src/pf1_adapter.rs`'s `compose_character_input`
//! actually composes for a freshly created character — not a bare,
//! choice-free skeleton. That is the posture the operator's question
//! ("can a user build this class in the app yet?") is actually about.
//! Concretely, this reuses the existing, already-established construction:
//!
//! 1. Load `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
//!    — the same shared fixture `tests/v06_rogue_ui_reachability.rs` already
//!    uses for exactly this purpose, and which that test verified is
//!    byte-for-byte the shape `compose_character_input` produces (race,
//!    ability scores, feats, skill allocations, equipment selections).
//! 2. Swap in the class under test at level 1 (`apply_level_up`'s and
//!    `compose_character_input`'s own single-class shape).
//! 3. Apply the same class-conditional canonical choice/spell seeds
//!    `compose_character_input` applies — Wizard's school specialization +
//!    starter spellbook, Arcanist's Metamagic Knowledge + starter
//!    spellbook, Sorcerer's bloodline + arcane bond, Cleric's domain,
//!    Druid's nature bond, Monk's bonus feat, Cavalier's Order,
//!    Inquisitor's domain, and Oracle's Mystery + Curse. See
//!    `canonical_seeds_for` below; each seed cites the `pf1_adapter.rs`
//!    block it mirrors.
//!
//! **Level sweep.** Each class is run at every level in `1..=20`, not at a
//! single level. That is not decoration. Paladin and Ranger are blocked
//! *only* at their first level (that chassis slice defers the hybrid spell
//! burden; every later level reaches `Computed`), whereas Monk is blocked at
//! every level. Reporting one level would flatten those two very different
//! situations into the same word. A class counts as complete here only when
//! every swept level reaches `Computed`.
//!
//! Nothing here computes or fabricates a status: `build_pilot_headless_receipt`
//! does that, and this binary only serialises what it returns.
//!
//! This binary is an operator/ops surface (like `gen_cache_acg`), not an
//! app runtime surface — nothing in the shipped app calls it.

use std::path::PathBuf;
use std::process::Command;

use codex::rules_core::character_input::{
    AcquisitionMode, CharacterClassLevel, CharacterInput, SelectedChoice, SpellSelection,
    load_character_input_fixture,
};
use codex::rules_core::pilot_compute::{HeadlessReceiptStatus, build_pilot_headless_receipt};
use codex::rules_core::rules_tables::acg::AcgClassId;
use codex::rules_core::rules_tables::apg::ApgClassId;
use codex::rules_core::rules_tables::crb::class_tables::ClassId;

/// The shared deterministic pilot input fixture, relative to the crate root.
/// Read at runtime (rather than `include_str!`ed) so a `src/` target does not
/// bake a `tests/` asset into itself; this binary is only ever run from a
/// repo checkout, the same assumption `gen_cache_acg`/`gen_cache_apg` already
/// make about `CARGO_MANIFEST_DIR`.
const FIXTURE_RELATIVE_PATH: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

/// The level range every class is swept over. 20 is PF1's own class ceiling
/// and the `MAXLEVEL:20` every CRB/APG/ACG class table already declares.
const MAX_LEVEL: u8 = 20;

/// One claim-blocking diagnostic, plus exactly which levels it fires at.
struct BlockingDiagnostic {
    id: String,
    message: String,
    levels: Vec<u8>,
}

/// One class's real, engine-derived state across the whole level sweep.
struct ClassState {
    id: String,
    book: &'static str,
    /// Every level in 1..=MAX_LEVEL reaches `Computed`.
    computed: bool,
    /// The creation posture specifically -- what a freshly made character of
    /// this class does in the app today.
    computed_at_level_1: bool,
    levels_computed: Vec<u8>,
    levels_blocked: Vec<u8>,
    blocking: Vec<BlockingDiagnostic>,
}

/// Lowercase corpus name for a CRB class. `ApgClassId`/`AcgClassId` already
/// expose their own `name()`; `crb::class_tables::ClassId` does not, so this
/// exhaustive `match` supplies the same thing (and the compiler enforces that
/// a newly added CRB class cannot be silently omitted from the dashboard).
fn crb_class_name(class_id: ClassId) -> &'static str {
    match class_id {
        ClassId::Barbarian => "barbarian",
        ClassId::Bard => "bard",
        ClassId::Cleric => "cleric",
        ClassId::Druid => "druid",
        ClassId::Fighter => "fighter",
        ClassId::Monk => "monk",
        ClassId::Paladin => "paladin",
        ClassId::Ranger => "ranger",
        ClassId::Rogue => "rogue",
        ClassId::Sorcerer => "sorcerer",
        ClassId::Wizard => "wizard",
    }
}

/// The class-conditional canonical seeds `compose_character_input`
/// (`apps/desktop/src-tauri/src/pf1_adapter.rs`) applies at creation time.
/// Returned as `(selected_choices, spells_selected)`.
///
/// These are silently-applied canonical defaults in production, not real
/// in-game player choices (the pickers for them are Path B in
/// `docs/release/v0.6/choice-picker-ui-gap-scoping.md`). Mirroring them here
/// is what makes this dump answer the operator's real question rather than a
/// hypothetical bare-skeleton one.
fn canonical_seeds_for(class_name: &str) -> (Vec<SelectedChoice>, Vec<SpellSelection>) {
    let choice = |set: &str, selection: &str| SelectedChoice {
        choice_set_id: set.to_owned(),
        selection_id: selection.to_owned(),
    };
    let spell = |class: &str, mode: AcquisitionMode| SpellSelection {
        // `WIZARD_STARTER_SPELL_ID`/`ARCANIST_STARTER_SPELL_ID`/
        // `WARPRIEST_STARTER_SPELL_ID` in `pf1_adapter.rs` are all `"Light"`.
        spell_id: "Light".to_owned(),
        source_class_id: format!("class:{class}"),
        acquisition_mode: mode,
    };
    // `CANONICAL_EXTRACT_SPELL_ID` in `pf1_adapter.rs`. Alchemist and
    // Investigator cast extracts off a formula list, not the CRB spell
    // list, so their seed is a different literal from `spell` above.
    let extract = |class: &str, mode: AcquisitionMode| SpellSelection {
        spell_id: "Cure Light Wounds".to_owned(),
        source_class_id: format!("class:{class}"),
        acquisition_mode: mode,
    };

    match class_name {
        // pf1_adapter.rs: the Wizard block (school specialization + two
        // opposed schools, plus the bootstrap starter spell known+prepared).
        "wizard" => (
            vec![
                choice("choice:wizard_school_specialization", "school:evocation"),
                choice("choice:wizard_opposed_schools", "school:necromancy"),
                choice("choice:wizard_opposed_schools", "school:transmutation"),
            ],
            vec![
                spell("wizard", AcquisitionMode::Known),
                spell("wizard", AcquisitionMode::Prepared),
            ],
        ),
        // pf1_adapter.rs: the Arcanist block (Metamagic Knowledge + the same
        // bootstrap starter-spellbook shape Wizard needs).
        "arcanist" => (
            vec![choice(
                "choice:arcanist_metamagic_knowledge",
                "metamagic:empower_spell",
            )],
            vec![
                spell("arcanist", AcquisitionMode::Known),
                spell("arcanist", AcquisitionMode::Prepared),
            ],
        ),
        // pf1_adapter.rs: the Sorcerer/Cleric/Druid Path A block. None of the
        // three need a seeded spell — each engine's known-spell posture is
        // genuinely valid with zero known spells.
        "sorcerer" => (
            vec![
                choice("choice:sorcerer_bloodline", "bloodline:arcane"),
                choice("choice:sorcerer_arcane_bond", "bond:familiar"),
            ],
            Vec::new(),
        ),
        "cleric" => (vec![choice("choice:cleric_domain", "domain:good")], Vec::new()),
        "druid" => (
            vec![choice("choice:druid_nature_bond", "bond:animal_companion")],
            Vec::new(),
        ),
        // pf1_adapter.rs: the Monk Path A seed. Same one-choice shape as
        // Sorcerer/Cleric/Druid -- no spell involved. The engine closes
        // Monk's last bonus-feat blocker only when `feat:dodge` is ALSO
        // genuinely on `selected_feats`; the shared GE-06 fixture this
        // binary sweeps (`pf1_human_fighter_level1_ge06_deterministic_input.txt`)
        // already carries it, exactly as `compose_character_input`'s own
        // fixed loadout does, so the two postures really do match.
        "monk" => (
            vec![choice("choice:monk_bonus_feat", "feat:dodge")],
            Vec::new(),
        ),
        // pf1_adapter.rs: the Witch Path A seed. Flight is the canonical
        // hex because its magnitude is the only one of the corpus's 53
        // that lands on a total this engine computes --
        // `BONUS:SKILL|Swim|4|TYPE=Racial`
        // (`advanced_players_guide/apg_abilities_class.lst:892`) flows
        // into `skill.selected_modifier.swim`. No spell is seeded: a
        // Witch's prepared-spell posture is genuinely valid with zero
        // spells (`unmet_witch_prepared_spell_conditions` iterates the
        // selections and finds nothing to reject), so Wizard's
        // bootstrap-deadlock shape does not apply.
        "witch" => (
            vec![choice("choice:witch_hex", "hex:flight")],
            Vec::new(),
        ),
        // pf1_adapter.rs: the Shaman Path A seed. Unlike Witch, all TEN
        // primary Spirits already ground their immediately-available
        // base ability, so this picks which one the default posture
        // records rather than which one works. Life earns it by
        // grounding the richest real magnitude set (Channel
        // uses-per-day, dice AND save DC).
        "shaman" => (
            vec![choice("choice:shaman_spirit", "spirit:life")],
            Vec::new(),
        ),
        // pf1_adapter.rs: the four spellcasting-shaped classes. Three of
        // them take Arcanist's own "chooser seed AND spell seed" shape;
        // Bloodrager takes Sorcerer's chooser-only shape, since a
        // Bloodrager with zero known spells is a genuinely valid posture
        // and the class casts nothing at all below level 4.
        //
        // `CANONICAL_EXTRACT_SPELL_ID` is `"Cure Light Wounds"`, a real
        // extract-level-1 entry of the shared `ALCHEMIST_SPELL_LIST`
        // (Investigator's own `SPELLLIST:1|Alchemist` token makes that
        // list literally shared with Alchemist). Each is still recorded
        // under its OWN `source_class_id`: the two formula books never
        // cross-satisfy, which `pilot_compute.rs`'s own
        // `the_two_extract_classes_do_not_cross_satisfy_each_others_formula_books`
        // pins.
        "alchemist" => (
            vec![choice("choice:alchemist_discovery", "discovery:feral_mutagen")],
            vec![
                extract("alchemist", AcquisitionMode::Known),
                extract("alchemist", AcquisitionMode::Prepared),
            ],
        ),
        "investigator" => (
            vec![choice("choice:investigator_talent", "talent:resiliency")],
            vec![
                extract("investigator", AcquisitionMode::Known),
                extract("investigator", AcquisitionMode::Prepared),
            ],
        ),
        // Warpriest needs a Blessing choice AND a spellbook entry
        // recorded plus prepared. `Light` is a real level-0 SPELL_LIST
        // key and a real Cleric orison -- Warpriest casts from
        // `SPELLLIST:1|Cleric` -- so `spell()` above already produces
        // exactly the right shape.
        "warpriest" => (
            vec![choice("choice:warpriest_blessing", "blessing:destruction")],
            vec![
                spell("warpriest", AcquisitionMode::Known),
                spell("warpriest", AcquisitionMode::Prepared),
            ],
        ),
        "bloodrager" => (
            vec![choice("choice:bloodrager_bloodline", "bloodline:arcane")],
            Vec::new(),
        ),
        // pf1_adapter.rs: the Summoner Path A seed. The Eidolon's one
        // genuinely built evolution purchase -- cost 1 out of a level-1
        // pool of 3, so it is affordable at every level in the sweep.
        "summoner" => (
            vec![choice(
                "choice:summoner_eidolon_evolution",
                "evolution:improved_natural_armor",
            )],
            Vec::new(),
        ),
        // pf1_adapter.rs: the Cavalier/Inquisitor/Oracle Path A block
        // (2026-07-29). All three are the Sorcerer/Cleric/Druid/Monk
        // shape -- a recognized chooser selection alone, no bootstrapped
        // spell (each class's own known-spell posture, where it has one,
        // is genuinely valid with zero known spells). Each seeds the ONE
        // corpus-verified option this codebase actually grounds a power
        // for; see `pilot_compute.rs`'s
        // `apg_canonical_choice_path_a_tests` for the proof that these
        // exact seeds are sufficient at every level 1-20, and that
        // nothing else reaches `Computed` off the back of them.
        "cavalier" => (
            vec![choice("choice:cavalier_order", "order:sword")],
            Vec::new(),
        ),
        "inquisitor" => (vec![choice("choice:inquisitor_domain", "domain:good")], Vec::new()),
        "oracle" => (
            vec![
                choice("choice:oracle_mystery", "mystery:life"),
                choice("choice:oracle_curse", "curse:clouded_vision"),
            ],
            Vec::new(),
        ),
        _ => (Vec::new(), Vec::new()),
    }
}

/// Build the real production-shaped input for one class at one level.
fn input_for(fixture: &CharacterInput, class_name: &str, level: u8) -> CharacterInput {
    let mut input = fixture.clone();
    input.case_id = Some(format!("v06_class_state_dump.{class_name}.level{level}"));
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: format!("class:{class_name}"),
        level,
    }];

    let (choices, spells) = canonical_seeds_for(class_name);
    input.chosen.selected_choices.extend(choices);
    input.chosen.spells_selected.extend(spells);
    input
}

/// Sweep one class across every level and collect its real engine state.
fn state_for(fixture: &CharacterInput, class_name: &str, book: &'static str) -> ClassState {
    let mut levels_computed = Vec::new();
    let mut levels_blocked = Vec::new();
    // Preserves first-seen diagnostic order rather than sorting, so the
    // dashboard reads the engine's own emission order.
    let mut blocking: Vec<BlockingDiagnostic> = Vec::new();

    for level in 1..=MAX_LEVEL {
        let input = input_for(fixture, class_name, level);

        // A panic inside the compute pipeline is itself a real, reportable
        // blocker -- an operator dashboard that crashed instead of naming it
        // would be hiding the worst kind of gap. `build_pilot_headless_receipt`
        // takes `&CharacterInput` (plain owned data, no interior mutability),
        // so there is no torn state to observe after an unwind here.
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            build_pilot_headless_receipt(&input)
        }));

        let receipt = match outcome {
            Ok(receipt) => receipt,
            Err(payload) => {
                let detail = payload
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_owned())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "<non-string panic payload>".to_owned());
                levels_blocked.push(level);
                let id = "engine.panic".to_owned();
                match blocking.iter_mut().find(|b| b.id == id) {
                    Some(existing) => existing.levels.push(level),
                    None => blocking.push(BlockingDiagnostic {
                        id,
                        message: format!(
                            "the compute pipeline PANICS for class:{class_name} at this level \
                             instead of returning a receipt, so no status can be derived: \
                             {detail}"
                        ),
                        levels: vec![level],
                    }),
                }
                continue;
            }
        };

        if receipt.status == HeadlessReceiptStatus::Computed {
            levels_computed.push(level);
        } else {
            levels_blocked.push(level);
        }
        for d in receipt.computation.diagnostics.iter().filter(|d| d.claim_blocking) {
            match blocking.iter_mut().find(|b| b.id == d.id) {
                Some(existing) => {
                    // The engine legitimately emits the same diagnostic more
                    // than once per computation; the dashboard wants the set
                    // of affected levels, not a multiset.
                    if existing.levels.last() != Some(&level) {
                        existing.levels.push(level);
                    }
                }
                None => blocking.push(BlockingDiagnostic {
                    id: d.id.clone(),
                    message: d.message.clone(),
                    levels: vec![level],
                }),
            }
        }
    }

    ClassState {
        id: class_name.to_owned(),
        book,
        computed: levels_blocked.is_empty(),
        computed_at_level_1: levels_computed.first() == Some(&1),
        levels_computed,
        levels_blocked,
        blocking,
    }
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
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_RELATIVE_PATH);
    let fixture_text = match std::fs::read_to_string(&fixture_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("could not read {}: {e}", fixture_path.display());
            std::process::exit(1);
        }
    };

    let load = load_character_input_fixture(&fixture_text);
    if !load.diagnostics.is_empty() {
        eprintln!(
            "shared deterministic fixture failed to load cleanly: {:?}",
            load.diagnostics
        );
        std::process::exit(1);
    }
    let Some(fixture) = load.character_input else {
        eprintln!("shared deterministic fixture produced no CharacterInput record");
        std::process::exit(1);
    };

    // Silence the default panic printer for the duration of the sweep: every
    // panic is caught, its payload is preserved verbatim in the class's own
    // `engine.panic` blocking diagnostic, and letting the default hook also
    // dump dozens of backtraces to stderr would bury the real signal.
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let mut states = Vec::new();
    for class_id in ClassId::ALL {
        states.push(state_for(&fixture, crb_class_name(*class_id), "CRB"));
    }
    for class_id in ApgClassId::ALL {
        states.push(state_for(&fixture, class_id.name(), "APG"));
    }
    for class_id in AcgClassId::ALL {
        states.push(state_for(&fixture, class_id.name(), "ACG"));
    }

    std::panic::set_hook(previous_hook);

    let classes: Vec<serde_json::Value> = states
        .iter()
        .map(|s| {
            serde_json::json!({
                "id": s.id,
                "book": s.book,
                "computed": s.computed,
                "status": if s.computed { "Computed" } else { "Blocked" },
                "computed_at_level_1": s.computed_at_level_1,
                "levels_computed": s.levels_computed,
                "levels_blocked": s.levels_blocked,
                "blocking_diagnostics": s
                    .blocking
                    .iter()
                    .map(|b| serde_json::json!({
                        "id": b.id,
                        "message": b.message,
                        "levels": b.levels,
                    }))
                    .collect::<Vec<_>>(),
            })
        })
        .collect();

    let computed_count = states.iter().filter(|s| s.computed).count();
    let document = serde_json::json!({
        "generated_at": real_now_iso8601(),
        "generated_by": "cargo run --bin v06_class_state_dump",
        "source_of_truth": "codex::rules_core::pilot_compute::build_pilot_headless_receipt",
        "input_posture": format!(
            "{FIXTURE_RELATIVE_PATH} with the class swapped to the class under test, swept over \
             levels 1-{MAX_LEVEL}, plus the canonical per-class choice/spell seeds \
             compose_character_input applies (pf1_adapter.rs). A class counts as computed only \
             when every level in the sweep reaches HeadlessReceiptStatus::Computed."
        ),
        "max_level": MAX_LEVEL,
        "class_count": states.len(),
        "computed_count": computed_count,
        "blocked_count": states.len() - computed_count,
        "classes": classes,
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&document).expect("dashboard document is serialisable")
    );
}
