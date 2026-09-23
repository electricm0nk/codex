//! Shared canonical class seeds and the shared deterministic pilot input
//! fixture, factored out of `src/bin/v06_class_state_dump.rs` so more than
//! one binary can build the same production-shaped [`CharacterInput`] for a
//! class at a level without duplicating the seed table.
//!
//! # Why this exists
//!
//! `v06_class_state_dump` and `src/bin/class_census.rs` (SD-36 Epic F, batch
//! F0, step F0b) both need to answer the exact same question -- "what does
//! the real fixed loadout `apps/desktop/src-tauri/src/pf1_adapter.rs`'s
//! `compose_character_input` actually composes for a freshly created
//! character of class X at level N" -- and answering it wrong in two places
//! that drift apart is worse than answering it once. A `src/bin/*.rs` file
//! is its own separate crate target; nothing in `src/bin/class_census.rs`
//! could import a function defined only in `src/bin/v06_class_state_dump.rs`.
//! This module is the shared home both binaries import from instead.
//!
//! Moving this code changed nothing about what it computes:
//! `v06_class_state_dump`'s own JSON output is byte-identical before and
//! after this move except for the `generated_at` timestamp (see
//! `docs/release/SD-36-consolidation/artifacts/epic-f/` for the two saved
//! dumps this was verified against).

use crate::rules_core::character_input::{
    AcquisitionMode, CharacterClassLevel, CharacterInput, SelectedChoice, SpellSelection,
};

/// The shared deterministic pilot input fixture, relative to the crate root.
/// Read at runtime (rather than `include_str!`ed) so a `src/` target does not
/// bake a `tests/` asset into itself; every binary that uses this is only
/// ever run from a repo checkout, the same assumption `gen_cache_acg`/
/// `gen_cache_apg` already make about `CARGO_MANIFEST_DIR`.
pub const FIXTURE_RELATIVE_PATH: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

/// SD-36 F1c-3 (D6): the choice a Commoner records its one Simple weapon under -- the converted
/// choice id of `Single Simple Weapon Proficiency` (`cr_abilities_class.lst:2736`,
/// `CHOOSE:WEAPONPROFICIENCY|!PC[TYPE=Simple]`), the one member of the `Simple Weapon Proficiency
/// Choice` pool the Commoner's `Weapon and Armor Proficiency` picks into
/// (`cr_abilities_class.lst:2825`, linked at ingest by `pool_link.rs`).
pub const COMMONER_WEAPON_CHOICE_ID: &str = "core_rulebook:class_feature:single_simple_weapon_proficiency";

/// The Commoner's canonical Simple weapon: Club, the first Simple weapon the CRB weapon table
/// (`weapon_tables::WEAPON_TABLE`) lists that is usable in melee (the census baseline attack is a
/// melee attack; Blowgun, listed first, is ranged only). A Path-A default, not a player's pick.
pub const COMMONER_CANONICAL_WEAPON: &str = "weapon:Club";

/// The class-conditional canonical seeds `compose_character_input`
/// (`apps/desktop/src-tauri/src/pf1_adapter.rs`) applies at creation time.
/// Returned as `(selected_choices, spells_selected)`.
///
/// These are silently-applied canonical defaults in production, not real
/// in-game player choices (the pickers for them are Path B in
/// `docs/release/v0.6/choice-picker-ui-gap-scoping.md`). Mirroring them here
/// is what makes a dump answer the operator's real question rather than a
/// hypothetical bare-skeleton one.
///
/// A class name this table does not recognize (most of the untabled
/// exotic/CRB-NPC/Ultimate Combat classes F0b's census sweeps for the first
/// time) gets no seed at all (`(Vec::new(), Vec::new())`) -- the same bare
/// posture `compose_character_input` itself falls back to for a class it
/// has no canonical-choice block for. This function never invents a seed
/// for a class `pf1_adapter.rs` does not already seed.
pub fn canonical_seeds_for(class_name: &str) -> (Vec<SelectedChoice>, Vec<SpellSelection>) {
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
        // module's callers sweep
        // (`pf1_human_fighter_level1_ge06_deterministic_input.txt`) already
        // carries it, exactly as `compose_character_input`'s own fixed
        // loadout does, so the two postures really do match.
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
        //
        // SD-36 F1c-5 (D8): plus the Summoner Class Selection pick, whose Standard class carries
        // the Summoner's weapon and armor proficiency (`apg_abilities_class.lst:747`).
        "summoner" => (
            vec![
                choice(
                    "choice:summoner_eidolon_evolution",
                    "evolution:improved_natural_armor",
                ),
                choice(SUMMONER_CLASS_SELECTION_CHOICE_ID, SUMMONER_CANONICAL_CLASS_SELECTION),
            ],
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
        // pf1_adapter.rs: the Commoner Path A seed (SD-36 F1c-3, D6). The Commoner is
        // proficient with ONE Simple weapon of the player's choice; the converted record links
        // that pick to its options, and the sheet prints the weapon recorded here.
        "commoner" => (vec![choice(COMMONER_WEAPON_CHOICE_ID, COMMONER_CANONICAL_WEAPON)], Vec::new()),
        _ => (Vec::new(), Vec::new()),
    }
}

/// SD-36 F1c-5 (D8): the choice a Summoner records its Summoner Class Selection pick under -- the
/// converted id of the Summoner ability (`apg_abilities_class.lst:739`), which fills the
/// `Summoner Class Selection` pool (`apg_abilitycategories.lst:267`,
/// `POOL:Pool_Summoner_Class_Selection`) with one pick (`BONUS:VAR|Pool_Summoner_Class_Selection|1`).
pub const SUMMONER_CLASS_SELECTION_CHOICE_ID: &str = "advanced_players_guide:class_feature:summoner";

/// The Summoner's canonical Class Selection: `Summoner ~ Standard Class` (`:741`). The oracle
/// makes it the no-pick default -- `:739`'s `BONUS:VAR|StandardSummoner|1|TYPE=Base|
/// !PREABILITY:1,CATEGORY=Class,TYPE.Summoner Class Selection` makes a Summoner with no pick in
/// the pool a standard Summoner -- and it is also the pool's first member in oracle order (`:741`
/// before `pu_abilities_class.lst:117`). A Path-A default, not a player's pick.
pub const SUMMONER_CANONICAL_CLASS_SELECTION: &str = "advanced_players_guide:class_feature:summoner_standard_class";

/// Build the real production-shaped input for one class at one level:
/// `fixture` cloned, `case_id` re-stamped, the class swapped in as the sole
/// class level, and `class_name`'s canonical seeds (above) merged onto the
/// fixture's own selections. `class_name` is the bare slug (no `class:`
/// prefix), matching `ClassCensusEntry::class_id.strip_prefix("class:")`.
pub fn input_for(fixture: &CharacterInput, class_name: &str, level: u8) -> CharacterInput {
    let mut input = fixture.clone();
    input.case_id = Some(format!("class_seeds.{class_name}.level{level}"));
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: format!("class:{class_name}"),
        level,
    }];

    let (choices, spells) = canonical_seeds_for(class_name);
    input.chosen.selected_choices.extend(choices);
    input.chosen.spells_selected.extend(spells);
    input
}
