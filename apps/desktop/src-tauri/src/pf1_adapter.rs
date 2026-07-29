//! `Pf1Adapter` — the Pathfinder 1e rule-system's adapter implementation
//! (SD-25 Epic 3 "Hub of Hubs" refactor, Criterion 3.2).
//!
//! Extracted out of `character_hub.rs`, which previously carried this
//! module's mutation logic inline alongside its Tauri command wrappers and
//! wire DTOs. `character_hub.rs` keeps its `#[tauri::command]` surface and
//! DTOs (they are also depended on by `characterHub::appendToCharacter` /
//! `characterHub::reSaveCharacter`, outside this criterion's file-touch
//! grant) and now calls into the functions this module owns; this module
//! owns the actual PF1 rules-application logic: composing a fresh
//! `CharacterInput` from a create-character request, applying one bounded
//! mutation (level up / add equipment / add spell) to an already-saved
//! character's `CharacterInput`, and the shared
//! load -> mutate -> recompute -> re-save -> return-envelope tail every
//! mutation op shares.
//!
//! `Pf1Adapter` (the zero-sized struct below) is the type this criterion's
//! own doc (`cycles/3_2.md`) names as the destination for
//! `impl RuleSystemAdapter for Pf1Adapter`. Criterion 3.1's
//! `RuleSystemAdapter` trait (`rule_system_adapter.rs`) had not landed on
//! `tranche/5-3` as of this cycle's own dispatch (parallel: yes, isolation:
//! worktree — a sibling cycle, not a prerequisite this cycle blocks on per
//! `cycles/3_2.md`'s own "Gated on: E2 complete" line) — `Pf1Adapter::level_up`
//! was written first as a real, wired, tested inherent method under the
//! trait surface's method name, ready to receive the `impl` once 3.1
//! landed. 3.1 landed mid-cycle (a later rebase during this cycle's own
//! push picked it up); the `impl RuleSystemAdapter for Pf1Adapter` block
//! below was then added in the same cycle rather than deferred, using
//! `compute_level_up_grants_for_class` (this cycle's own register-A2 fix,
//! `src/rules_core/level_up.rs`) as `level_up`'s real per-delta dispatch —
//! the fix and its production call site land together, not in two
//! separate cycles.
//!
//! ## Carry-forward register A5 (operator-confirmed, `decisions.md §11`)
//!
//! `mutate_saved_character_at_root` below now advances `envelope.revision_id`
//! on every successful call (via `next_mutation_revision_id`), not just
//! `saved_at`. Before this cycle, only `characterHub::reSaveCharacter`'s own
//! bespoke `next_revision_id` advanced the counter; every mutation routed
//! through this function (`level_up_character`, `add_equipment_selection`,
//! `add_spell_selection`, and `appendToCharacter` via
//! `characterHub::appendToCharacter`, which reuses this same function)
//! silently preserved whatever `revision_id` was already on disk, forever.
//! This is a deliberate, operator-approved behavior change, not a deferred
//! `## DISCOVERED` item.

use std::path::Path;

use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput,
    ChosenCharacterState, EquipmentSelection, SelectedChoice, SkillAllocation, SpellSelection,
};
use codex::rules_core::feat_effects;
use codex::rules_core::level_up::{compute_level_up_grants_for_class, LevelUpPlan};
use codex::rules_core::pilot_compute::{
    build_pilot_headless_receipt, compute_pilot_base_chassis, ComputationDiagnostic,
    PilotBaseChassisComputation, SelectedSkillModifiers,
};
#[cfg(test)]
use codex::rules_core::pilot_compute::HeadlessReceiptStatus;
use codex::rules_core::pilot_compute_corpus::{
    compute_combat_baseline_from_corpus, compute_pilot_with_corpus,
    compute_selected_skill_modifiers_from_corpus, CorpusPilotReceipt,
};
use codex::rules_core::pilot_view_model::{
    PilotCombatViewModel, PilotCompanionViewModel, PilotDefenseViewModel, PilotSkillViewModel,
    PilotSnapshot,
};
use codex::rules_core::source_content::SourcePackageContent;
use codex::saved_character::local_store::SavedCharacterStore;

use crate::character_hub::{
    map_chosen_feat_targets_dto, map_corpus_derived_dto, map_diagnostics_dto, map_snapshot_dto,
    map_spells_selected_dto, map_summary_dto, summarize_envelope, CreateCharacterRequest,
    CreateCharacterResponse,
    ListSavedCharactersResponse, LoadSavedCharacterResponse, HUMAN_RACE_ID, SOURCE_PACKAGE_ID,
};
use crate::characterHub::appendToCharacter::{
    append_to_character_at_root, AppendToCharacterResponse, ItemToAppendDto,
};
use crate::characterHub::recomputeCharacter::{recompute_character_at_root, RecomputeCharacterResponse};
use crate::characterHub::reSaveCharacter::{re_save_character_at_root, ReSaveCharacterResponse};
use crate::corpus_fixtures::corpus_fixture_bundle;
use crate::rule_system_adapter::{ClassLevelDelta, RuleSystemAdapter};

/// v0.6 alpha swarm: needed by `compose_character_input`'s Wizard-only
/// canonical school-choice seeding. Not re-exported from `character_hub.rs`
/// (unlike `HUMAN_RACE_ID`/`SOURCE_PACKAGE_ID`) since nothing outside this
/// file needs it yet.
const WIZARD_CLASS_ID: &str = "class:wizard";

/// v0.6 alpha swarm (bootstrap-deadlock fix): the canonical starter spell
/// seeded for every Wizard at class-acquisition time (`compose_character_input`
/// and `apply_level_up`'s new-class-entry branch) — a 0-level Evocation
/// cantrip, the specialist school, so it never trips the opposed-school
/// double slot cost, and well within the level-0 budget (3 slots) at every
/// wizard level `unmet_wizard_spellbook_conditions` currently supports (1-3).
/// The same literal already proven safe by
/// `wizard_level1_reaches_computed_once_a_real_spell_is_recorded_and_prepared`.
///
/// `"Light"` — a real `SPELL_LIST` key (`rules_tables::crb::spell_list.rs`),
/// not a synthetic placeholder: v0.6 alpha swarm's slot-budget-enforcement
/// fix taught `parse_wizard_spellbook_spell_id` (`pilot_compute.rs`) to
/// resolve real spell catalog keys directly, so production code now seeds a
/// real spell rather than the old `"evocation.0.light"` convention-only
/// value (which still resolves too, via that function's fallback, but
/// there's no reason to keep seeding a placeholder now that the real path
/// exists).
const WIZARD_STARTER_SPELL_ID: &str = "Light";

/// v0.6 alpha swarm (Path A choice-picker gap closure): needed by
/// `compose_character_input`'s Sorcerer/Cleric/Druid canonical-choice
/// seeding below, mirroring `WIZARD_CLASS_ID`'s own precedent exactly --
/// each of these three classes' engine (`pilot_compute.rs`) can reach
/// `Computed` today, but only once its own real, recognized choice is
/// present, and no picker anywhere in the creation UI can submit one (see
/// `docs/release/v0.6/choice-picker-ui-gap-scoping.md`). Not re-exported
/// from `character_hub.rs` since nothing outside this file needs them yet.
const SORCERER_CLASS_ID: &str = "class:sorcerer";
const CLERIC_CLASS_ID: &str = "class:cleric";
const DRUID_CLASS_ID: &str = "class:druid";

/// v0.6 alpha swarm (Path A choice-picker gap closure, Arcanist's own):
/// needed by `compose_character_input`'s Arcanist canonical-choice
/// seeding below. Unlike Sorcerer/Cleric/Druid (a choice alone
/// sufficient) or Wizard (a spell alone sufficient), Arcanist needs
/// BOTH: a starter spell (the same bootstrap-deadlock shape Wizard has)
/// AND a recognized Metamagic Knowledge choice (the same "no picker
/// exists for this choice set" gap Sorcerer/Cleric/Druid have) --
/// verified directly against `pilot_compute.rs`'s own
/// `single_class_arcanist_with_a_valid_spellbook_and_recognized_metamagic_knowledge_reaches_computed`
/// test that this exact combination is sufficient, with no other
/// precondition, to reach `Computed`. See
/// `docs/release/v0.6/arcanist-metamagic-knowledge-exploit-scoping.md`
/// and `risks-and-open-questions.md` for the full record of this gap.
const ARCANIST_CLASS_ID: &str = "class:arcanist";
const ARCANIST_STARTER_SPELL_ID: &str = "Light";
const ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID: &str = "choice:arcanist_metamagic_knowledge";
/// **Real bug found and fixed (2026-07-25)**: this used to be the bare
/// literal `"Empower Spell"` (zero colons), which live-testing before
/// shipping the `characterHubModel.ts` `CLASS_OPTIONS` entry caught as a
/// real save-time error -- `saved_character::local_store::validate_character_input`
/// requires every `selected_choices` entry's `selection_id` to carry at
/// least one colon to round-trip through the fixture grammar, and a real
/// feat name has none. The compute engine (`pilot_compute.rs`) now
/// expects this exact namespaced value: `ground_or_block_arcanist_metamagic_knowledge`
/// translates it back to the literal `"Empower Spell"` via
/// `arcanist_metamagic_knowledge_feat_name` before ever reaching the feat
/// catalog, so this seed must stay in sync with that translation's own
/// `metamagic:<snake_case_slug>` convention.
const EMPOWER_SPELL_METAMAGIC_SELECTION: &str = "metamagic:empower_spell";

/// v0.6 alpha swarm (Path A choice-picker gap closure, Monk's own): needed
/// by `compose_character_input`'s Monk canonical-choice seeding below.
/// Monk is the Sorcerer/Cleric/Druid shape, not Wizard's -- a single
/// recognized choice is sufficient, and no bootstrapped spell is involved.
///
/// Monk's engine already computes a complete build at every level 1-20; its
/// one remaining claim-blocking diagnostic
/// (`class_feature.monk.bounded_progression.bonus_feat.unsupported`) fires
/// purely because nothing anywhere seeds `choice:monk_bonus_feat`, so the
/// default posture never exercises a seam that already works. No picker in
/// the creation UI can submit one (Path B in
/// `docs/release/v0.6/choice-picker-ui-gap-scoping.md`).
const MONK_CLASS_ID: &str = "class:monk";
const MONK_BONUS_FEAT_CHOICE_ID: &str = "choice:monk_bonus_feat";

/// **Why Dodge, of the seven feats the corpus offers at
/// `MonkBonusFeatLVL,1`.** Verified directly against the PCGen corpus
/// (`.../core_rulebook/cr_abilities_class.lst:1263`):
/// `Monk Bonus Feat ~ Dodge ... PREVARGTEQ:MonkBonusFeatLVL,1 ...
/// ABILITY:FEAT|VIRTUAL|Dodge` -- genuinely available at level 1, alongside
/// Catch Off-Guard, Combat Reflexes, Deflect Arrows, Improved Grapple,
/// Scorpion Style, and Throw Anything (the 6th/10th-level additions are
/// gated `PREVARGTEQ:MonkBonusFeatLVL,6` / `,10` and are not options here).
///
/// Six of those seven close the burden in `pilot_compute.rs`, but they are
/// not equivalent, and Dodge is the only one that is genuinely resolved
/// under the posture this function actually composes:
///
/// - **Dodge** is the one option the engine CROSS-CHECKS: its
///   `dodge_bonus_feat_is_genuinely_active` gate closes the burden only when
///   `feat:dodge` is really present on `chosen.selected_feats`, so the
///   claimed benefit is one the character actually has. It is --
///   unconditionally, for every race, in this very function's fixed GE-06
///   loadout below. And the benefit is real and already computed:
///   `compute_combat_baseline` applies `DODGE_AC_BONUS` (+1 AC) for any
///   character carrying `feat:dodge`, regardless of which slot granted it.
///   So this seed makes the engine compute a real effect; it is not a token
///   that merely silences a diagnostic (`docs/governance/no-stub-mvp-doctrine.md`).
/// - **Catch Off-Guard / Throw Anything** close as *provably vacuous* under
///   this bounded slice (their entire benefit is improvised/splash weapons,
///   which the deterministic Longsword baseline never models). Legal, but
///   they would hand a player a canonical feat worth exactly zero.
/// - **Combat Reflexes / Scorpion Style / Improved Grapple** each ground a
///   real number, but their closures fire on the choice alone with no
///   `selected_feats` cross-check -- seeding one would claim a feat the
///   character does not carry, so the sheet's Feats tab would show the
///   player nothing for the feat they supposedly picked.
/// - **Deflect Arrows** requires `feat:deflect_arrows` on `selected_feats`
///   (its `text_complete` branch), which this loadout does not carry.
///
/// **Honest caveat.** For a Human, this function also seeds
/// `choice:human_bonus_feat -> feat:dodge`, and the corpus record above
/// carries `!PREABILITY:1,CATEGORY=FEAT,Dodge` -- so a Human Monk's
/// canonical posture names Dodge in two slots, which PF1 would treat as a
/// wasted pick. The COMPUTED result is still correct (the +1 dodge bonus is
/// applied once by `compute_combat_baseline`, never doubled), and this is
/// the same class of approximation the fixed GE-06 loadout already makes
/// everywhere else -- it seeds `choice:fighter_bonus_feat` for every class,
/// Wizard included. A real per-class bonus-feat picker (Path B) is what
/// replaces this, exactly as for the Sorcerer/Cleric/Druid seeds.
const DODGE_FEAT_SELECTION: &str = "feat:dodge";

/// v0.6 alpha swarm (Path A choice-picker gap closure, the two
/// chooser-shaped power lists): Witch's Hex and Shaman's Spirit. Both are
/// the Sorcerer/Cleric/Druid/Monk shape -- a single recognized choice is
/// sufficient and no bootstrapped spell is involved -- not Wizard's.
///
/// Each engine already computes a complete build at every level 1-20; each
/// had exactly two claim-blocking diagnostics, and BOTH were downstream of
/// the same fact: nothing anywhere seeds the class's one defining chooser,
/// so the default posture never exercised a seam that already worked. No
/// picker in the creation UI can submit one (Path B in
/// `docs/release/v0.6/choice-picker-ui-gap-scoping.md`).
const WITCH_CLASS_ID: &str = "class:witch";
const WITCH_HEX_CHOICE_ID: &str = "choice:witch_hex";
const SHAMAN_CLASS_ID: &str = "class:shaman";
const SHAMAN_SPIRIT_CHOICE_ID: &str = "choice:shaman_spirit";

/// **Why Flight, of the corpus's 53 base Witch hexes.** Verified directly
/// against the PCGen corpus
/// (`.../advanced_players_guide/apg_abilities_class.lst:892`):
/// `KEY:Witch Hex ~ Flight ... BONUS:SKILL|Swim|4|TYPE=Racial`, gated only
/// by `PREVARGTEQ:WitchHexAbilityLVL,1` -- genuinely available at level 1.
///
/// Three hexes have a grounded magnitude in `pilot_compute.rs` and any of
/// the three closes the burden, but they are not equivalent. Flight is the
/// only one whose number reaches a total this engine actually computes:
/// `compute_selected_skill_modifiers` folds it into
/// `skill.selected_modifier.swim`. Ward's deflection/resistance bonus is
/// grounded standalone (real AC/save totals exist, but Ward's magnitude is
/// not wired into either), and Cauldron's `+4` insight lands on Craft
/// (Alchemy), which is not among the three skills this engine computes. So
/// this seed makes the engine compute a real, visible effect rather than
/// handing a player a token that merely silences a diagnostic
/// (`docs/governance/no-stub-mvp-doctrine.md`).
const FLIGHT_HEX_SELECTION: &str = "hex:flight";

/// **Why Life, of the ten primary Shaman Spirits.** Unlike Witch -- where
/// only 3 of 53 hexes have a grounded magnitude -- all ten Spirits already
/// ground their immediately-available base ability, so this picks which one
/// the default posture records, not which one works
/// (`all_ten_spirits_reach_computed_not_just_the_canonical_one` in
/// `pilot_compute.rs` pins that every one of them reaches `Computed`).
///
/// Life earns the seed on magnitude richness. Verified against
/// `.../advanced_class_guide/acg_abilities_class.lst:1600`,
/// `KEY:Life Spirit ~ Channel` carries three real formulas --
/// `BONUS:VAR|ShamanChannelTimes|1+CHA`,
/// `BONUS:VAR|ShamanChannelDice|(ShamanChannelLVL+1)/2`,
/// `BONUS:VAR|ShamanChannelDC|10+(ShamanChannelLVL/2)+CHA` -- which
/// `pilot_compute.rs` grounds as three separate explanation records. The
/// other nine each ground a single touch-attack or morale-bonus fact.
///
/// **Honest caveat.** None of the ten base abilities carries a `BONUS:`
/// landing on a computed total -- every one is a `BONUS:VAR` feeding its
/// own `DESC:` text -- so unlike Witch's Flight this seed grounds real
/// magnitudes without integrating into an existing total. The Spirit
/// abilities that DO land on real totals (Heavens' Manifestation
/// `BONUS:SAVE|ALL`, Life's own Healer's Touch `BONUS:SKILL|Heal|4`) are
/// all in the level-8+/16+ gated tiers, which stay deferred.
const LIFE_SPIRIT_SELECTION: &str = "spirit:life";

/// v0.6 alpha swarm (Path A choice-picker gap closure, the four
/// spellcasting-shaped classes): Alchemist, Investigator, Warpriest and
/// Bloodrager each reach `Computed` at every level 1-20 today, but only
/// once their own real, recognized creation-time choices are present, and
/// no picker anywhere in the creation UI can submit one -- the same gap,
/// and the same fix, as the Wizard/Arcanist/Sorcerer/Cleric/Druid/Monk
/// seeds above.
///
/// Three of the four need BOTH a spell/extract seed AND a chooser seed
/// (Arcanist's shape); Bloodrager needs only the chooser (Sorcerer's
/// shape), since its own spell posture is genuinely valid with zero known
/// spells. Every value below is verified directly against
/// `pilot_compute.rs`'s own
/// `spellcasting_shaped_class_closure_tests::all_four_spellcasting_shaped_classes_reach_computed_at_every_level`,
/// which runs this exact seed set over the whole 1-20 sweep.
const ALCHEMIST_CLASS_ID: &str = "class:alchemist";
const INVESTIGATOR_CLASS_ID: &str = "class:investigator";
const WARPRIEST_CLASS_ID: &str = "class:warpriest";
const BLOODRAGER_CLASS_ID: &str = "class:bloodrager";

/// Alchemist and Investigator share one formula list
/// (`SPELLLIST:1|Alchemist` on Investigator's own class record), so they
/// share one canonical starter extract. `"Cure Light Wounds"` is a real
/// `ALCHEMIST_SPELL_LIST` key at extract level 1 -- the only extract level
/// either class can reach at class level 1 -- so a single value is inside
/// the slot budget at every level of the sweep for both. Each is recorded
/// under its OWN `source_class_id`; the two formula books never
/// cross-satisfy.
const CANONICAL_EXTRACT_SPELL_ID: &str = "Cure Light Wounds";

/// Alchemist's canonical Discovery, one of the corpus's 35. Feral Mutagen
/// is the only one whose record carries real self-contained magnitudes
/// attaching to an already-grounded feature of this class (Mutagen).
const ALCHEMIST_DISCOVERY_CHOICE_ID: &str = "choice:alchemist_discovery";
const FERAL_MUTAGEN_DISCOVERY_SELECTION: &str = "discovery:feral_mutagen";

/// Investigator's canonical Talent. Resiliency is the one entry of her own
/// 40-record Rogue Talent whitelist this codebase grounds (task #58); the
/// slot itself does not open until investigator level 3, so the seed is
/// correctly inert at levels 1-2 and simply takes effect when it opens.
const INVESTIGATOR_TALENT_CHOICE_ID: &str = "choice:investigator_talent";
const RESILIENCY_TALENT_SELECTION: &str = "talent:resiliency";

/// Warpriest's canonical Blessing (Destruction, whose Destructive Attacks
/// minor power is grounded) plus its canonical starter spell. `"Light"` is
/// a real level-0 `SPELL_LIST` key and a real Cleric orison -- Warpriest
/// casts from `SPELLLIST:1|Cleric` -- and level-0 slots are 3+ at every
/// warpriest level, so one seed covers the whole sweep. Deliberately the
/// same literal `WIZARD_STARTER_SPELL_ID`/`ARCANIST_STARTER_SPELL_ID` use.
const WARPRIEST_BLESSING_CHOICE_ID: &str = "choice:warpriest_blessing";
const DESTRUCTION_BLESSING_SELECTION: &str = "blessing:destruction";
const WARPRIEST_STARTER_SPELL_ID: &str = "Light";

/// Bloodrager's canonical Bloodline, one of ten. Arcane keeps one
/// bloodline NAME shared with the Sorcerer seed above, but Bloodrager's
/// bloodlines are PARALLEL to Sorcerer's rather than shared with them
/// (task #59) -- the grounding underneath is Bloodrager's own separate
/// corpus records.
const BLOODRAGER_BLOODLINE_CHOICE_ID: &str = "choice:bloodrager_bloodline";
const ARCANE_BLOODRAGER_BLOODLINE_SELECTION: &str = "bloodline:arcane";

/// The Pathfinder 1e `RuleSystemAdapter` implementation. Zero-sized today —
/// every operation below is stateless (it takes the on-disk root / mutation
/// closure it needs as parameters, exactly like this crate's existing
/// `_at_root` free-function convention) — see this module's own top-of-file
/// doc comment for the trait-impl timing note.
///
/// `#[allow(dead_code)]`: not yet constructed by any `#[tauri::command]`
/// call site — criterion 3.4 (Tauri command-surface routes through the
/// hub-of-hubs) wires real callers once criterion 3.1's `RuleSystemAdapter`
/// trait has landed and this type implements it. Exercised today by this
/// module's own `#[cfg(test)]` tests (`pf1_adapter_level_up_delegates_to_the_real_implementation`),
/// which prove `level_up` below is real, wired behavior, not a stub —
/// `cargo check`'s non-test build just cannot see that caller.
#[allow(dead_code)]
pub struct Pf1Adapter;

#[allow(dead_code)]
impl Pf1Adapter {
    /// The `level_up` operation named by `RuleSystemAdapter`'s method
    /// surface (`cycles/3_1.md`). Delegates to `level_up_character_at_root`
    /// below so there is exactly one real implementation of "level up a
    /// saved character's class", not two.
    pub fn level_up(
        &self,
        root: &Path,
        class_id: &str,
        additional_choices: Vec<SelectedChoice>,
        skill_allocations: Option<Vec<SkillAllocation>>,
        saved_at: &str,
    ) -> Result<CreateCharacterResponse, String> {
        level_up_character_at_root(root, class_id, additional_choices, skill_allocations, saved_at)
    }
}

impl RuleSystemAdapter for Pf1Adapter {
    fn rule_system_id(&self) -> &'static str {
        "pf1"
    }

    fn chassis_resolve(&self, input: &CharacterInput) -> PilotBaseChassisComputation {
        compute_pilot_base_chassis(input)
    }

    /// Dispatches each `ClassLevelDelta` through `compute_level_up_grants_for_class`
    /// (this cycle's own register-A2 fix) and merges the resulting plans —
    /// so a multi-delta call (e.g. a Fighter+Wizard mix leveling both sides
    /// in one request) produces the union of every class's real grants,
    /// never the top-level `compute_level_up_grants`'s multiclass-gap empty
    /// default this criterion's carry-forward note describes.
    fn level_up(&self, character: &CharacterInput, deltas: &[ClassLevelDelta]) -> LevelUpPlan {
        let mut plan = LevelUpPlan::default();
        for delta in deltas {
            let sub = compute_level_up_grants_for_class(
                character,
                &delta.class_id,
                delta.from_level,
                delta.to_level,
            );
            plan.automatic_features.extend(sub.automatic_features);
            plan.pick_from_lists.extend(sub.pick_from_lists);
            plan.resource_pool_change.pools.extend(sub.resource_pool_change.pools);
            plan.prerequisites_added.extend(sub.prerequisites_added);
            plan.capstone_threshold = plan.capstone_threshold || sub.capstone_threshold;
        }
        plan
    }

    fn save_character(
        &self,
        root: &Path,
        expected_revision_id: &str,
        saved_at: &str,
    ) -> Result<ReSaveCharacterResponse, String> {
        re_save_character_at_root(root, expected_revision_id, saved_at)
    }

    fn append_to_character(
        &self,
        root: &Path,
        items_to_append: &[ItemToAppendDto],
        saved_at: &str,
    ) -> Result<AppendToCharacterResponse, String> {
        append_to_character_at_root(root, items_to_append, saved_at)
    }

    fn recompute(&self, root: &Path, character_id: &str) -> RecomputeCharacterResponse {
        recompute_character_at_root(root, character_id)
    }

    fn list_saved_characters(
        &self,
        characters_root: &Path,
    ) -> Result<ListSavedCharactersResponse, String> {
        let listing = SavedCharacterStore::list_all(characters_root).map_err(|err| err.message)?;
        Ok(ListSavedCharactersResponse {
            characters: listing.characters.iter().map(map_summary_dto).collect(),
            unreadable_count: listing.unreadable_entries.len(),
        })
    }

    fn load_saved_character(&self, root: &Path) -> Result<LoadSavedCharacterResponse, String> {
        let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

        let (snapshot, diagnostics, corpus_receipt) =
            match resolve_unified_pilot_snapshot(&envelope.character_input, corpus_fixture_bundle()) {
                Ok((snapshot, corpus_receipt)) => (Some(snapshot), Vec::new(), corpus_receipt),
                Err(diagnostics) => (
                    None,
                    diagnostics,
                    compute_pilot_with_corpus(&envelope.character_input, corpus_fixture_bundle()),
                ),
            };

        Ok(LoadSavedCharacterResponse {
            summary: summarize_envelope(&envelope),
            snapshot: snapshot.as_ref().map(map_snapshot_dto),
            diagnostics: map_diagnostics_dto(&diagnostics),
            corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
            selected_feats: envelope.character_input.chosen.selected_feats.clone(),
            spells_selected: map_spells_selected_dto(&envelope.character_input.chosen.spells_selected),
            chosen_feat_targets: map_chosen_feat_targets_dto(&envelope.character_input),
        })
    }
}

// ----- Pure functions (unit-testable, no AppHandle / filesystem) -----

/// Build a `CharacterInput` for the requested race/class/level. Race, class,
/// and ability scores are the caller's real choices; the feat/skill/
/// equipment loadout is fixed — `unmet_combat_posture_conditions`
/// (`pilot_compute.rs`) requires this exact equipment/feat posture verbatim
/// to reach `Computed`, so widening it would not change which combinations
/// reach `Computed`. Human additionally receives its own canonical
/// choice-slot values — the ability-bonus target is the caller's real
/// choice (`request.ability_bonus_target`); every other race omits the
/// Human-only slots. For every class except Wizard, `spells_selected` is
/// *not* fixed to any hardcoded placeholder (SD-24 Criterion 7.5): no
/// `Computed`-status gate reads it for those classes, `CreateCharacterRequest`
/// collects no spell choices from the caller, and a freshly composed
/// character starts with an empty spellbook that only grows through the
/// real wired `add_spell_selection` / `appendToCharacter` command surface.
///
/// Wizard is the one exception (v0.6 alpha swarm, bootstrap-deadlock fix):
/// `unmet_wizard_spellbook_conditions` (`pilot_compute.rs`) reads
/// `spells_selected` as part of the same `Computed` gate `create_character`
/// requires before persisting, so an empty spellbook meant a freshly
/// created Wizard could never be saved at all — no command exists that can
/// grow a spellbook that was never written to disk in the first place. This
/// seeds one canonical starter spell (`WIZARD_STARTER_SPELL_ID`, a real
/// 0-level Evocation cantrip — the same identity already proven safe
/// against the budget math by `wizard_level1_reaches_computed_once_a_real_spell_is_recorded_and_prepared`)
/// as both `AcquisitionMode::Known` and `AcquisitionMode::Prepared`, the
/// same "fixed canonical default now, real player choice is separate future
/// UI work" pattern already used for the Fighter feat loadout and the
/// Wizard school-specialization choices seeded below.
///
/// Moved here verbatim from `character_hub.rs` (SD-25 Criterion 3.2
/// extraction) — re-exported at `crate::character_hub::compose_character_input`
/// so `characterHub::appendToCharacter` / `characterHub::reSaveCharacter`'s
/// existing `use crate::character_hub::compose_character_input;` import
/// paths keep resolving unchanged.
pub fn compose_character_input(request: &CreateCharacterRequest) -> CharacterInput {
    let mut selected_choices = vec![
        SelectedChoice {
            choice_set_id: "choice:level_1_character_feat".to_owned(),
            selection_id: "feat:power_attack".to_owned(),
        },
        SelectedChoice {
            choice_set_id: "choice:fighter_bonus_feat".to_owned(),
            selection_id: "feat:weapon_focus:weapon:longsword".to_owned(),
        },
    ];

    if request.race_id == HUMAN_RACE_ID {
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:human_bonus_feat".to_owned(),
            selection_id: "feat:dodge".to_owned(),
        });
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:human_ability_bonus".to_owned(),
            selection_id: format!("ability:{}", request.ability_bonus_target),
        });
    }

    // v0.6 alpha swarm: without this, `unmet_wizard_spellbook_conditions`
    // (pilot_compute.rs) unconditionally blocks a Wizard from ever reaching
    // Computed, no matter what spells a tester later selects -- it requires
    // the canonical Evocation specialization (opposed Necromancy/
    // Transmutation) before it even looks at spellbook content, and nothing
    // anywhere seeded that choice for a freshly created character. Mirrors
    // this function's own existing precedent (Fighter's fixed Power Attack/
    // Dodge/Weapon Focus loadout, Human's fixed bonus-feat/ability-bonus
    // choices): a fixed, canonical default for the one class/level range
    // this engine's chassis dispatch actually supports (Wizard 1-3's
    // spellbook grounding), not a real in-game "pick your school" choice --
    // that UI is separate, larger, out-of-scope future work.
    let mut spells_selected = Vec::new();
    if request.class_id == WIZARD_CLASS_ID {
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:wizard_school_specialization".to_owned(),
            selection_id: "school:evocation".to_owned(),
        });
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:wizard_opposed_schools".to_owned(),
            selection_id: "school:necromancy".to_owned(),
        });
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:wizard_opposed_schools".to_owned(),
            selection_id: "school:transmutation".to_owned(),
        });

        // v0.6 alpha swarm (bootstrap-deadlock fix): see this function's own
        // doc comment above for why a Wizard specifically needs a non-empty
        // spellbook to ever be saved at all.
        spells_selected.push(SpellSelection {
            spell_id: WIZARD_STARTER_SPELL_ID.to_owned(),
            source_class_id: WIZARD_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        spells_selected.push(SpellSelection {
            spell_id: WIZARD_STARTER_SPELL_ID.to_owned(),
            source_class_id: WIZARD_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });
    } else if request.class_id == ARCANIST_CLASS_ID {
        // v0.6 alpha swarm (Path A choice-picker gap closure, Arcanist's
        // own): see `ARCANIST_CLASS_ID`'s own doc comment above. Needs
        // BOTH a starter spell (Wizard's own bootstrap-deadlock shape)
        // AND a recognized Metamagic Knowledge choice (Sorcerer/Cleric/
        // Druid's own "no picker for this choice" shape) -- verified
        // together, not either alone, per
        // `single_class_arcanist_with_a_valid_spellbook_and_recognized_metamagic_knowledge_reaches_computed`.
        selected_choices.push(SelectedChoice {
            choice_set_id: ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID.to_owned(),
            selection_id: EMPOWER_SPELL_METAMAGIC_SELECTION.to_owned(),
        });
        spells_selected.push(SpellSelection {
            spell_id: ARCANIST_STARTER_SPELL_ID.to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        spells_selected.push(SpellSelection {
            spell_id: ARCANIST_STARTER_SPELL_ID.to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });
    } else if request.class_id == ALCHEMIST_CLASS_ID {
        // v0.6 alpha swarm (the four spellcasting-shaped classes): Arcanist's
        // own "chooser seed AND starter-spell seed, both required" shape.
        // Alchemist's prepared-extract validator needs at least one extract
        // recorded in the formula book AND one prepared today before it will
        // ground anything, and its Discovery chooser is the canonical
        // narrowing that resolves the remaining class-feature blocker.
        selected_choices.push(SelectedChoice {
            choice_set_id: ALCHEMIST_DISCOVERY_CHOICE_ID.to_owned(),
            selection_id: FERAL_MUTAGEN_DISCOVERY_SELECTION.to_owned(),
        });
        spells_selected.push(SpellSelection {
            spell_id: CANONICAL_EXTRACT_SPELL_ID.to_owned(),
            source_class_id: ALCHEMIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        spells_selected.push(SpellSelection {
            spell_id: CANONICAL_EXTRACT_SPELL_ID.to_owned(),
            source_class_id: ALCHEMIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });
    } else if request.class_id == INVESTIGATOR_CLASS_ID {
        // Same shape as Alchemist immediately above, and genuinely the same
        // underlying mechanism: Investigator's `SPELLLIST:1|Alchemist` token
        // makes the formula list literally shared. The two are still seeded
        // separately because each formula book is keyed on its own
        // `source_class_id` and neither satisfies the other.
        selected_choices.push(SelectedChoice {
            choice_set_id: INVESTIGATOR_TALENT_CHOICE_ID.to_owned(),
            selection_id: RESILIENCY_TALENT_SELECTION.to_owned(),
        });
        spells_selected.push(SpellSelection {
            spell_id: CANONICAL_EXTRACT_SPELL_ID.to_owned(),
            source_class_id: INVESTIGATOR_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        spells_selected.push(SpellSelection {
            spell_id: CANONICAL_EXTRACT_SPELL_ID.to_owned(),
            source_class_id: INVESTIGATOR_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });
    } else if request.class_id == WARPRIEST_CLASS_ID {
        // Warpriest needs THREE seeds, not two: a Blessing choice (its own
        // Blessing-powers blocker), and a spellbook entry recorded plus
        // prepared (its prepared-spellbook blocker). The Blessing choice
        // also resolves the class-feature blocker, the same way Cleric's
        // Good domain does.
        selected_choices.push(SelectedChoice {
            choice_set_id: WARPRIEST_BLESSING_CHOICE_ID.to_owned(),
            selection_id: DESTRUCTION_BLESSING_SELECTION.to_owned(),
        });
        spells_selected.push(SpellSelection {
            spell_id: WARPRIEST_STARTER_SPELL_ID.to_owned(),
            source_class_id: WARPRIEST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        spells_selected.push(SpellSelection {
            spell_id: WARPRIEST_STARTER_SPELL_ID.to_owned(),
            source_class_id: WARPRIEST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });
    } else if request.class_id == BLOODRAGER_CLASS_ID {
        // Sorcerer/Cleric/Druid's shape, not Arcanist's: NO spell seed. A
        // Bloodrager with zero known spells is a genuinely valid posture
        // (`unmet_bloodrager_known_spell_conditions` returns no unmet
        // conditions for an empty known list), and the class casts nothing
        // at all below level 4. Only the Bloodline chooser is needed.
        selected_choices.push(SelectedChoice {
            choice_set_id: BLOODRAGER_BLOODLINE_CHOICE_ID.to_owned(),
            selection_id: ARCANE_BLOODRAGER_BLOODLINE_SELECTION.to_owned(),
        });
    }

    // v0.6 alpha swarm (Path A choice-picker gap closure, per
    // `docs/release/v0.6/choice-picker-ui-gap-scoping.md`): Sorcerer,
    // Cleric, and Druid can each genuinely reach `Computed` today, but
    // only once their own real, recognized choice is present -- and, same
    // as Wizard's own school-specialization gap before this fix, nothing
    // in the creation UI has ever had a way to submit one (no picker, no
    // wire-contract field). Mirrors the Wizard block immediately above:
    // a fixed, canonical default, silently applied to every character of
    // that class, NOT a real in-game choice -- that picker is separate,
    // larger, out-of-scope future work (Path B in the scoping doc). Unlike
    // Wizard, none of these three classes need a bootstrapped
    // known/prepared spell to avoid a save-time deadlock (each engine's
    // own known-spell posture is genuinely valid with zero known spells,
    // proven directly by each closure's own `..._reaches_computed` test),
    // so no `spells_selected` entries are seeded here.
    if request.class_id == SORCERER_CLASS_ID {
        // Verified directly against `pilot_compute.rs`'s own
        // `single_class_sorcerer_with_arcane_bond_recognized_reaches_computed`
        // test: Arcane bloodline + a familiar Arcane Bond (chosen over
        // bonded object as the more commonly played, equally-supported
        // option -- both are recognized identically) is sufficient, with
        // no other precondition, to reach `Computed`.
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:sorcerer_bloodline".to_owned(),
            selection_id: "bloodline:arcane".to_owned(),
        });
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:sorcerer_arcane_bond".to_owned(),
            selection_id: "bond:familiar".to_owned(),
        });
    } else if request.class_id == CLERIC_CLASS_ID {
        // Verified directly against `pilot_compute.rs`'s own Cleric
        // closure: a recognized Good domain (with no Healing domain also
        // chosen, and Touch of Good correctly left inactive -- a
        // genuinely valid PF1 posture, not every Good-domain Cleric is
        // using this limited-use power at every moment) is sufficient to
        // reach `Computed`. No `class_ability_activations` entry is
        // seeded for Touch of Good -- "not currently active" is the
        // honest default, mirroring how Barbarian Rage's own "not raging"
        // default needs no activation entry either.
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:cleric_domain".to_owned(),
            selection_id: "domain:good".to_owned(),
        });
    } else if request.class_id == DRUID_CLASS_ID {
        // Verified directly against `pilot_compute.rs`'s own
        // `single_class_druid_level1_with_animal_companion_reaches_computed`
        // test: a recognized animal-companion nature bond is sufficient,
        // with no other precondition, to reach `Computed`. No species
        // choice is seeded -- Wolf is the only companion species this
        // codebase's Druid/Hunter seam ever grounds, assumed automatically
        // once the bond type is recognized (no species-selection input is
        // modeled for this class at all).
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:druid_nature_bond".to_owned(),
            selection_id: "bond:animal_companion".to_owned(),
        });
    } else if request.class_id == MONK_CLASS_ID {
        // v0.6 alpha swarm (Path A choice-picker gap closure, Monk's own) --
        // the same shape as the three above, and for the same reason: the
        // engine can already compute a complete Monk, but only once a real
        // recognized choice is present, and no picker can submit one.
        //
        // Verified directly against `pilot_compute.rs`'s own
        // `monk_with_dodge_bonus_feat_genuinely_active_does_not_trip_the_diagnostic`
        // test: a recognized `choice:monk_bonus_feat -> feat:dodge`, PLUS
        // `feat:dodge` genuinely present on `selected_feats` (which this
        // function's fixed loadout above already carries, unconditionally,
        // for every race), is sufficient with no other precondition to
        // reach `Computed`. And it is genuinely RESOLVED, not merely
        // tolerated: the engine emits a real
        // `class_feature.monk.bounded_progression.bonus_feat.dodge_active`
        // record carrying the +1 dodge AC bonus `compute_combat_baseline`
        // is already applying. See `DODGE_FEAT_SELECTION`'s own doc comment
        // for why Dodge specifically, of the seven corpus options at
        // `MonkBonusFeatLVL,1`, and for the Human double-grant caveat.
        //
        // ONE seeding site only, unlike Wizard/Arcanist: Monk's whole
        // bonus-feat seam sits behind `supported_monk_level`, which matches
        // a SINGLE-class Monk only, so `apply_level_up`'s multiclass-dip
        // branch never reaches it and needs no mirrored seed -- established
        // empirically by
        // `monk_multiclass_dip_reaches_computed_from_apply_level_up_alone`,
        // which passes without one. Leveling a Monk 1 -> 2 takes
        // `apply_level_up`'s increment-existing-level branch, so this
        // creation-time seed simply persists -- pinned all the way to the
        // PF1 cap by `monk_stays_computed_leveling_all_the_way_to_20`.
        selected_choices.push(SelectedChoice {
            choice_set_id: MONK_BONUS_FEAT_CHOICE_ID.to_owned(),
            selection_id: DODGE_FEAT_SELECTION.to_owned(),
        });
    } else if request.class_id == WITCH_CLASS_ID {
        // v0.6 alpha swarm (Path A choice-picker gap closure, the
        // chooser-shaped power lists): the same one-choice shape as
        // Sorcerer/Cleric/Druid/Monk. See `FLIGHT_HEX_SELECTION`'s own doc
        // comment for why Flight specifically, of the corpus's 53 base
        // hexes.
        //
        // Verified directly against `pilot_compute.rs`'s own
        // `single_class_witch_with_the_canonical_flight_hex_reaches_computed`
        // and `witch_with_the_canonical_flight_hex_stays_computed_at_every_level`
        // tests: a recognized `choice:witch_hex -> hex:flight` is
        // sufficient, with no other precondition, to reach `Computed` at
        // every level 1-20. No spell is seeded -- a Witch's prepared-spell
        // posture is genuinely valid with zero spells, so Wizard's
        // bootstrap-deadlock shape does not apply here.
        selected_choices.push(SelectedChoice {
            choice_set_id: WITCH_HEX_CHOICE_ID.to_owned(),
            selection_id: FLIGHT_HEX_SELECTION.to_owned(),
        });
    } else if request.class_id == SHAMAN_CLASS_ID {
        // See `LIFE_SPIRIT_SELECTION`'s own doc comment for why Life, of
        // the ten primary Spirits. Verified directly against
        // `shaman_with_the_canonical_life_spirit_stays_computed_at_every_level`.
        selected_choices.push(SelectedChoice {
            choice_set_id: SHAMAN_SPIRIT_CHOICE_ID.to_owned(),
            selection_id: LIFE_SPIRIT_SELECTION.to_owned(),
        });
    }

    CharacterInput {
        case_id: Some(request.character_id.clone()),
        source_package_id: SOURCE_PACKAGE_ID.to_owned(),
        chosen: ChosenCharacterState {
            race_id: request.race_id.clone(),
            class_levels: vec![CharacterClassLevel {
                class_id: request.class_id.clone(),
                level: request.level,
            }],
            ability_scores: AbilityScores {
                strength: request.ability_scores.strength,
                dexterity: request.ability_scores.dexterity,
                constitution: request.ability_scores.constitution,
                intelligence: request.ability_scores.intelligence,
                wisdom: request.ability_scores.wisdom,
                charisma: request.ability_scores.charisma,
            },
            selected_feats: vec![
                "feat:power_attack".to_owned(),
                "feat:dodge".to_owned(),
                "feat:weapon_focus".to_owned(),
            ],
            skill_allocations: vec![
                SkillAllocation {
                    skill_id: "skill:climb".to_owned(),
                    ranks: 1,
                },
                SkillAllocation {
                    skill_id: "skill:intimidate".to_owned(),
                    ranks: 1,
                },
                SkillAllocation {
                    skill_id: "skill:swim".to_owned(),
                    ranks: 1,
                },
            ],
            equipment_selections: vec![
                EquipmentSelection {
                    item_id: "item:longsword".to_owned(),
                    equipped_or_active: true,
                    active_state: ActiveState::EquippedActive,
                    applied_modifiers: Vec::new(),
                },
                EquipmentSelection {
                    item_id: "item:chain_shirt".to_owned(),
                    equipped_or_active: true,
                    active_state: ActiveState::EquippedActive,
                    applied_modifiers: Vec::new(),
                },
                EquipmentSelection {
                    item_id: "item:shield".to_owned(),
                    equipped_or_active: false,
                    active_state: ActiveState::Absent,
                    applied_modifiers: Vec::new(),
                },
                EquipmentSelection {
                    item_id: "power_attack".to_owned(),
                    equipped_or_active: false,
                    active_state: ActiveState::SelectedInactive,
                    applied_modifiers: Vec::new(),
                },
            ],
            selected_choices,
            spells_selected,
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

/// Computes the next `{character_id}.rev.N` revision id from the current
/// on-disk one — the same "increment the numeric suffix, restart at 1 for a
/// non-conforming legacy revision_id" logic
/// `characterHub::reSaveCharacter::next_revision_id` already established
/// (that function is private to a file outside this criterion's file-touch
/// grant, so this is a parallel, functionally-identical implementation
/// rather than a shared one — not a behavior fork, the same rule applied in
/// two call sites).
fn next_mutation_revision_id(character_id: &str, current_revision_id: &str) -> String {
    let prefix = format!("{character_id}.rev.");
    let next_n = current_revision_id
        .strip_prefix(prefix.as_str())
        .and_then(|suffix| suffix.parse::<u64>().ok())
        .map(|n| n + 1)
        .unwrap_or(1);
    format!("{prefix}{next_n}")
}

/// v0.6 alpha swarm items 1+27 sub-task 5: the single, unified
/// Computed/Blocked verdict every command below now goes through, replacing
/// the exact-posture-only headless check each used to run directly.
/// `combat.baseline_unsupported`/`skill.selected_modifier.unsupported` are
/// no longer independently claim-blocking here — the corpus-aware
/// combat-baseline/selected-skill pillars (sub-task 4,
/// `pilot_compute_corpus::compute_combat_baseline_from_corpus`/
/// `compute_selected_skill_modifiers_from_corpus`) decide those two
/// specifically, for ANY resolvable armor/shield loadout (weapon and feat
/// requirements unchanged — see that sub-task's own scope note). Every
/// OTHER headless diagnostic (chassis unsupported, spellbook unsupported,
/// etc.) still blocks exactly as before, unaffected by this replacement.
///
/// Returns the projected `PilotSnapshot` (ready for `map_snapshot_dto`) and
/// the already-computed `CorpusPilotReceipt` (ready for
/// `map_corpus_derived_dto`) together, since every caller needs both and
/// computing the corpus-aware receipt twice would be wasteful. On failure,
/// returns the full diagnostic list — the original headless diagnostics
/// with the two posture-specific ones replaced by the real corpus-aware
/// unmet reasons when either combat or selected-skill blocked, or the
/// original diagnostics verbatim when something else (chassis, spellbook,
/// etc.) blocked regardless.
pub(crate) fn resolve_unified_pilot_snapshot(
    character_input: &CharacterInput,
    corpus: &SourcePackageContent,
) -> Result<(PilotSnapshot, CorpusPilotReceipt), Vec<ComputationDiagnostic>> {
    let receipt = build_pilot_headless_receipt(character_input);
    let corpus_receipt = compute_pilot_with_corpus(character_input, corpus);

    let other_diagnostics_present = receipt.computation.diagnostics.iter().any(|diagnostic| {
        diagnostic.claim_blocking
            && diagnostic.id != "combat.baseline_unsupported"
            && diagnostic.id != "skill.selected_modifier.unsupported"
    });
    if other_diagnostics_present {
        return Err(receipt.computation.diagnostics.clone());
    }

    let combat_result =
        compute_combat_baseline_from_corpus(&receipt.computation, character_input, corpus);
    let skills_result =
        compute_selected_skill_modifiers_from_corpus(&receipt.computation, character_input, corpus);

    match (combat_result, skills_result) {
        (Ok(combat), Ok(skills)) => {
            let snapshot = PilotSnapshot {
                ability_modifiers: receipt.computation.ability_modifiers,
                base_attack_bonus: receipt.computation.base_attack_bonus,
                base_saves: receipt.computation.base_saves,
                combat: PilotCombatViewModel {
                    baseline_melee_attack_bonus: combat.melee_attack_bonus,
                },
                defense: PilotDefenseViewModel {
                    baseline_armor_class: combat.armor_class,
                    total_save: receipt.computation.total_saves,
                    damage_reduction: receipt
                        .computation
                        .explanations
                        .iter()
                        .find(|explanation| {
                            explanation.id == "class_feature.barbarian.damage_reduction"
                        })
                        .map(|explanation| explanation.value)
                        .filter(|&value| value > 0),
                },
                skill: PilotSkillViewModel {
                    selected_modifier: SelectedSkillModifiers {
                        climb: skills.climb,
                        intimidate: skills.intimidate,
                        swim: skills.swim,
                    },
                },
                // Projected through the view model's own function rather
                // than re-derived here: this snapshot is assembled by hand
                // (it substitutes corpus-resolved combat/skill values for
                // the receipt's own), and a second, drifting copy of the
                // companion projection is exactly how the DR extraction
                // just above ended up duplicated.
                companion: PilotCompanionViewModel::from_receipt(&receipt),
            };
            Ok((snapshot, corpus_receipt))
        }
        (combat_result, skills_result) => {
            let mut diagnostics: Vec<ComputationDiagnostic> = receipt
                .computation
                .diagnostics
                .iter()
                .filter(|diagnostic| {
                    diagnostic.id != "combat.baseline_unsupported"
                        && diagnostic.id != "skill.selected_modifier.unsupported"
                })
                .cloned()
                .collect();
            if let Err(unmet) = combat_result {
                diagnostics.push(ComputationDiagnostic {
                    id: "combat.baseline_unsupported".to_owned(),
                    message: format!(
                        "baseline combat totals require every equipped item to resolve \
                         against the corpus with known math (armor/shield loadout is \
                         widened; the Longsword and Dodge/Weapon Focus requirements are \
                         unchanged); unmet conditions: {}",
                        unmet.join("; ")
                    ),
                    claim_blocking: true,
                });
            }
            if let Err(unmet) = skills_result {
                diagnostics.push(ComputationDiagnostic {
                    id: "skill.selected_modifier.unsupported".to_owned(),
                    message: format!(
                        "selected skill modifiers require every equipped item to resolve \
                         against the corpus with known math; unmet conditions: {}",
                        unmet.join("; ")
                    ),
                    claim_blocking: true,
                });
            }
            Err(diagnostics)
        }
    }
}

/// Shared load -> recompute -> re-save -> return-envelope tail for every
/// `mutate_saved_character` operation: applies `mutate` to the loaded
/// envelope's `CharacterInput`, recomputes via the real engine, and either
/// re-saves and returns `Saved` or leaves the on-disk envelope untouched and
/// returns `Blocked`. Never persists an unproven build — mirrors
/// `create_character`/`clone_character`'s own invariant. `mutate` receives
/// only the `CharacterInput`, so it cannot smuggle in a different `saved_at`
/// or bypass the recompute gate.
///
/// **SD-25 carry-forward register A5** (operator-confirmed 2026-07-21,
/// `decisions.md §11`): every successful call now also advances
/// `envelope.revision_id` (and `latest_authoritative_revision_ref`) to a
/// freshly minted `{character_id}.rev.N`, not just `saved_at` — folded in
/// as part of this extraction per the operator's explicit approval. Every
/// command routed through this function (`level_up_character`,
/// `add_equipment_selection`, `add_spell_selection`, and `appendToCharacter`
/// via `characterHub::appendToCharacter`, which calls this same function)
/// now advances the counter on every mutating call, not just
/// `reSaveCharacter`.
///
/// `pub(crate)` (rather than private) so `characterHub::appendToCharacter`
/// (SD-24 Epic 7, Criterion 7.1) can reuse this module's own
/// load -> mutate -> recompute -> re-save -> return-envelope invariant
/// instead of re-deriving it a second time — re-exported at
/// `crate::character_hub::mutate_saved_character_at_root` so that file's
/// existing `character_hub::mutate_saved_character_at_root(...)` call site
/// keeps resolving unchanged.
pub(crate) fn mutate_saved_character_at_root(
    root: &Path,
    saved_at: &str,
    mutate: impl FnOnce(&mut CharacterInput),
) -> Result<CreateCharacterResponse, String> {
    let mut envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

    mutate(&mut envelope.character_input);

    let (snapshot, corpus_receipt) =
        match resolve_unified_pilot_snapshot(&envelope.character_input, corpus_fixture_bundle()) {
            Ok(result) => result,
            Err(diagnostics) => {
                return Ok(CreateCharacterResponse::Blocked {
                    diagnostics: map_diagnostics_dto(&diagnostics),
                });
            }
        };

    let next_revision_id =
        next_mutation_revision_id(&envelope.character_id, &envelope.revision_id);
    envelope.revision_id = next_revision_id.clone();
    envelope.latest_authoritative_revision_ref = next_revision_id;
    envelope.saved_at = saved_at.to_owned();

    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;

    Ok(CreateCharacterResponse::Saved {
        summary: Box::new(summarize_envelope(&envelope)),
        snapshot: map_snapshot_dto(&snapshot),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
    })
}

/// Increments `class_id`'s level by 1 on the given `CharacterInput`, or
/// adds a new level-1 entry for `class_id` if the character has none yet
/// (the multiclass "dip" case). Every other field is untouched.
pub fn apply_level_up(character_input: &mut CharacterInput, class_id: &str) {
    if let Some(class_level) = character_input
        .chosen
        .class_levels
        .iter_mut()
        .find(|class_level| class_level.class_id == class_id)
    {
        class_level.level = class_level.level.saturating_add(1);
    } else {
        character_input.chosen.class_levels.push(CharacterClassLevel {
            class_id: class_id.to_owned(),
            level: 1,
        });

        // v0.6 alpha swarm: the same fix `compose_character_input` got for
        // fresh Wizard creation, applied to the multiclass-dip path. This
        // `else` branch only runs the first time `class_id` is added to
        // `class_levels` -- if it were already Wizard, the `if` branch
        // above (increment existing level) would have fired instead -- so
        // it can never fire twice for the same character and cannot
        // duplicate the seeded choices (which would break
        // `wizard_has_canonical_specialization_selections`'s exact-2
        // opposed-schools count). Without this, multiclassing Wizard onto
        // an existing character hits the same unconditional
        // "requires the canonical Evocation specialization" block
        // `compose_character_input`'s fix already solved for creation --
        // frontend verified this live before this fix landed.
        if class_id == WIZARD_CLASS_ID {
            character_input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: "choice:wizard_school_specialization".to_owned(),
                selection_id: "school:evocation".to_owned(),
            });
            character_input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: "choice:wizard_opposed_schools".to_owned(),
                selection_id: "school:necromancy".to_owned(),
            });
            character_input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: "choice:wizard_opposed_schools".to_owned(),
                selection_id: "school:transmutation".to_owned(),
            });

            // v0.6 alpha swarm (bootstrap-deadlock fix): the same starter-spell
            // seed `compose_character_input` gets for fresh Wizard creation,
            // applied to the multiclass-dip path for the identical reason --
            // see `compose_character_input`'s own doc comment. Same
            // once-only guarantee as the choice seeding immediately above
            // (this whole block only runs the first time Wizard is added).
            character_input.chosen.spells_selected.push(SpellSelection {
                spell_id: WIZARD_STARTER_SPELL_ID.to_owned(),
                source_class_id: WIZARD_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
            character_input.chosen.spells_selected.push(SpellSelection {
                spell_id: WIZARD_STARTER_SPELL_ID.to_owned(),
                source_class_id: WIZARD_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Prepared,
            });
        } else if class_id == ARCANIST_CLASS_ID {
            // v0.6 alpha swarm (Path A choice-picker gap closure,
            // Arcanist's own): the same starter-spell-plus-Metamagic-
            // Knowledge-choice seed `compose_character_input` gets for
            // fresh Arcanist creation, applied to the multiclass-dip
            // path for the identical reason -- see `ARCANIST_CLASS_ID`'s
            // own doc comment. Same once-only guarantee as the Wizard
            // block above (this whole branch only runs the first time
            // Arcanist is added).
            character_input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID.to_owned(),
                selection_id: EMPOWER_SPELL_METAMAGIC_SELECTION.to_owned(),
            });
            character_input.chosen.spells_selected.push(SpellSelection {
                spell_id: ARCANIST_STARTER_SPELL_ID.to_owned(),
                source_class_id: ARCANIST_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
            character_input.chosen.spells_selected.push(SpellSelection {
                spell_id: ARCANIST_STARTER_SPELL_ID.to_owned(),
                source_class_id: ARCANIST_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Prepared,
            });
        }
    }
}

/// Applies one level-up's full choice set on top of `apply_level_up`:
/// appends any additional chosen options (e.g. a hit-die roll record or a
/// feat pick at a feat-gaining level) and, when the caller supplies a full
/// skill-allocation set, replaces `chosen.skill_allocations` wholesale.
///
/// Hit-die-roll and feat-pick choices are modeled as generic
/// `SelectedChoice { choice_set_id, selection_id }` entries — this crate's
/// existing extensible convention for player choices (see
/// `compose_character_input`'s own `choice:level_1_character_feat` /
/// `choice:human_ability_bonus` entries and `pilot_compute.rs`'s
/// `choice_selection` reader) — rather than new dedicated fields on
/// `ChosenCharacterState`. That struct is constructed as a literal at
/// dozens of call sites across this crate's own test suite; adding a
/// required field there would break every one of them for a v0.6 task
/// scoped to persistence, not a rules_core schema change.
///
/// `skill_allocations: None` leaves the character's existing allocations
/// untouched (distinct from `apply_set_skill_allocations`'s own always-
/// replace contract, since a level-up's skill-points step is optional here
/// — the caller may choose to persist skill allocations via the dedicated
/// `set_skill_allocations` command instead).
///
/// Every `SelectedChoice.choice_set_id` must have exactly one colon (two
/// colon-segments, e.g. `"choice:level_2_hit_points"`) and every
/// `selection_id` at least one colon (e.g. `"hp:average"`,
/// `"feat:cleave"`) — `SavedCharacterStore::save`'s own
/// `validate_character_input` enforces this grammar so every choice
/// round-trips through the on-disk fixture format; a caller (or its Tauri
/// wrapper) that violates it fails honestly at save time rather than
/// producing a silently-truncated record.
pub fn apply_level_up_choices(
    character_input: &mut CharacterInput,
    class_id: &str,
    additional_choices: Vec<SelectedChoice>,
    skill_allocations: Option<Vec<SkillAllocation>>,
) {
    apply_level_up(character_input, class_id);
    character_input.chosen.selected_choices.extend(additional_choices);
    if let Some(allocations) = skill_allocations {
        apply_set_skill_allocations(character_input, allocations);
    }
}

/// `level_up_character`'s real implementation: load -> mutate -> recompute
/// -> re-save -> return envelope. Split out from the `#[tauri::command]`
/// wrapper in `character_hub.rs` so it is unit-testable against a real
/// `SavedCharacterStore` fixture without an `AppHandle`.
///
/// Mirrors `create_character`/`clone_character`'s "never persist an
/// unproven build" invariant: if the leveled-up build does not reach
/// `Computed` (e.g. leveling past the range the compute engine currently
/// supports), the saved character on disk is left exactly as it was and
/// `Blocked` is returned with the real diagnostics — the mutation is never
/// silently applied on disk when the recompute fails.
pub(crate) fn level_up_character_at_root(
    root: &Path,
    class_id: &str,
    additional_choices: Vec<SelectedChoice>,
    skill_allocations: Option<Vec<SkillAllocation>>,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_level_up_choices(character_input, class_id, additional_choices, skill_allocations);
    })
}

/// Appends one entry to `chosen.equipment_selections`. `equipped_or_active`
/// is derived from `active_state` (matching `EquipmentSelection`'s own doc
/// comment: the flag is a backward-compatible projection of the state, not
/// an independent choice) — true only for `ActiveState::EquippedActive`.
/// Every other field is untouched.
pub fn apply_add_equipment_selection(
    character_input: &mut CharacterInput,
    item_id: &str,
    active_state: ActiveState,
) {
    character_input.chosen.equipment_selections.push(EquipmentSelection {
        item_id: item_id.to_owned(),
        equipped_or_active: active_state == ActiveState::EquippedActive,
        active_state,
        applied_modifiers: Vec::new(),
    });
}

/// `add_equipment_selection`'s real implementation — see
/// `mutate_saved_character_at_root` for the shared
/// load -> mutate -> recompute -> re-save -> return-envelope semantics.
pub(crate) fn add_equipment_selection_at_root(
    root: &Path,
    item_id: &str,
    active_state: ActiveState,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_add_equipment_selection(character_input, item_id, active_state);
    })
}

/// v0.6 alpha swarm items 1+27 sub-task 6: appends `modifier_item_id` onto
/// the `applied_modifiers` list of the `equipment_selections` entry whose
/// `item_id` matches `item_id` -- mirrors PCGen's own single-entry
/// `CUSTOMIZATION:EQMOD=` convention (an applied equipmod has no separate
/// top-level selection of its own), same as
/// `character_input::EquipmentSelection`'s own doc comment describes.
/// Returns `false` (no mutation) when no selection matches `item_id` --
/// the caller (`attach_equipment_modifier_at_root`) checks this before
/// charging any money, so a not-found target never silently no-ops after
/// a real charge.
pub fn apply_attach_equipment_modifier(
    character_input: &mut CharacterInput,
    item_id: &str,
    modifier_item_id: &str,
) -> bool {
    let Some(selection) = character_input
        .chosen
        .equipment_selections
        .iter_mut()
        .find(|selection| selection.item_id == item_id)
    else {
        return false;
    };
    selection.applied_modifiers.push(modifier_item_id.to_owned());
    true
}

/// Appends one entry to `chosen.spells_selected`. Every other field is
/// untouched.
pub fn apply_add_spell_selection(
    character_input: &mut CharacterInput,
    spell_id: &str,
    source_class_id: &str,
    acquisition_mode: AcquisitionMode,
) {
    character_input.chosen.spells_selected.push(SpellSelection {
        spell_id: spell_id.to_owned(),
        source_class_id: source_class_id.to_owned(),
        acquisition_mode,
    });
}

/// Appends one entry to `chosen.selected_feats`. Every other field is
/// untouched.
pub fn apply_add_feat_selection(character_input: &mut CharacterInput, feat_id: &str) {
    character_input.chosen.selected_feats.push(feat_id.to_owned());
}

/// Appends a chooser feat together with the target it names, recording the
/// target as a real `SelectedChoice` under the feat's own Mechanism-B
/// contract.
///
/// Returns an error rather than silently degrading in the two cases that
/// would otherwise produce a feat the engine cannot read: a target given for
/// a feat that takes none, and a blank target. Both mean the caller and the
/// rules disagree about the feat, and quietly dropping the target would
/// leave a character that looks configured and computes as though it were
/// not.
///
/// The prefix and choice-set id are read from
/// [`feat_effects::chooser_contract_for_feat`] rather than assembled here,
/// so this cannot drift from the producers that consume the result.
pub fn resolve_feat_target_choice(
    feat_id: &str,
    target: Option<&str>,
) -> Result<Option<SelectedChoice>, String> {
    let Some(raw) = target else {
        // No target given. Legitimate for an ordinary feat, and also for a
        // chooser feat whose target has not been named yet -- the engine
        // grounds nothing for it and the sheet reports it as untargeted.
        return Ok(None);
    };
    if raw.trim().is_empty() {
        return Err(format!(
            "{feat_id} was given an empty target; a chooser feat needs a real target or none at all"
        ));
    }
    let Some(contract) = feat_effects::chooser_contract_for_feat(feat_id) else {
        return Err(format!(
            "{feat_id} takes no chosen target, but '{raw}' was supplied"
        ));
    };
    Ok(Some(SelectedChoice {
        choice_set_id: contract.choice_set_id.to_owned(),
        selection_id: format!("{}{}", contract.selection_prefix, raw.trim()),
    }))
}

/// Appends the feat and, when one was resolved, the choice recording its
/// target.
pub fn apply_add_feat_selection_with_target(
    character_input: &mut CharacterInput,
    feat_id: &str,
    target_choice: Option<SelectedChoice>,
) {
    if let Some(choice) = target_choice {
        character_input.chosen.selected_choices.push(choice);
    }
    apply_add_feat_selection(character_input, feat_id);
}

/// `add_feat_selection`'s real implementation — see
/// `mutate_saved_character_at_root` for the shared
/// load -> mutate -> recompute -> re-save -> return-envelope semantics.
pub(crate) fn add_feat_selection_at_root(
    root: &Path,
    feat_id: &str,
    target: Option<&str>,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    // Resolve before mutating: `mutate_saved_character_at_root` takes an
    // infallible closure, so a rejected target must surface as an error here
    // rather than as a silently feat-only save.
    let target_choice = resolve_feat_target_choice(feat_id, target)?;

    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_add_feat_selection_with_target(character_input, feat_id, target_choice.clone());
    })
}

/// `add_spell_selection`'s real implementation — see
/// `mutate_saved_character_at_root` for the shared
/// load -> mutate -> recompute -> re-save -> return-envelope semantics.
pub(crate) fn add_spell_selection_at_root(
    root: &Path,
    spell_id: &str,
    source_class_id: &str,
    acquisition_mode: AcquisitionMode,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_add_spell_selection(character_input, spell_id, source_class_id, acquisition_mode);
    })
}

/// Appends BOTH a `Known` and a `Prepared` entry for the same spell, in one
/// mutation (v0.6 alpha swarm, item 3 / the Wizard spellbook bootstrap fix).
///
/// `unmet_wizard_spellbook_conditions` requires a non-empty recorded
/// (`Known`) set AND a non-empty prepared (`Prepared`) set simultaneously
/// to reach `Computed`. `add_spell_selection` only ever appends one
/// `SpellSelection` (one spell, one mode) per call, and
/// `mutate_saved_character_at_root` discards any mutation that doesn't
/// independently reach `Computed` -- so a `Known`-only call is Blocked
/// (nothing `Prepared` yet) and discarded, and a `Prepared`-only call is
/// *also* Blocked (the prepared spell isn't in the still-empty recorded
/// set, since the first call never persisted) and discarded. Every UI path
/// -- creation or level-up -- was structurally stuck at zero spells
/// regardless of call order. This function breaks that bootstrap deadlock
/// by satisfying both conditions in the same atomic mutation.
///
/// Once a character has at least one spell recorded and prepared this way,
/// the plain `apply_add_spell_selection`/`add_spell_selection_at_root`
/// (either mode) work exactly as before for every subsequent spell --
/// learning more known spells or re-preparing an already-known spell
/// doesn't violate any exactness check on its own. This function exists
/// only to cross that first-spell threshold; it is not a replacement for
/// `add_spell_selection`.
pub fn apply_record_and_prepare_spell_selection(
    character_input: &mut CharacterInput,
    spell_id: &str,
    source_class_id: &str,
) {
    apply_add_spell_selection(character_input, spell_id, source_class_id, AcquisitionMode::Known);
    apply_add_spell_selection(character_input, spell_id, source_class_id, AcquisitionMode::Prepared);
}

/// `record_and_prepare_spell_selection`'s real implementation — see
/// `apply_record_and_prepare_spell_selection`'s own doc comment for why
/// this exists alongside (not instead of) `add_spell_selection_at_root`.
pub(crate) fn record_and_prepare_spell_selection_at_root(
    root: &Path,
    spell_id: &str,
    source_class_id: &str,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_record_and_prepare_spell_selection(character_input, spell_id, source_class_id);
    })
}

/// Replaces `chosen.skill_allocations` wholesale with the caller's full
/// allocation set. Unlike equipment/spell selections (which append one
/// entry at a time), the skill-allocation dialog always sends its complete
/// draft on accept (v0.6 alpha swarm, task 2), so persistence must replace
/// the whole set rather than append to it.
pub fn apply_set_skill_allocations(
    character_input: &mut CharacterInput,
    skill_allocations: Vec<SkillAllocation>,
) {
    character_input.chosen.skill_allocations = skill_allocations;
}

/// `set_skill_allocations`'s real implementation — see
/// `mutate_saved_character_at_root` for the shared
/// load -> mutate -> recompute -> re-save -> return-envelope semantics.
pub(crate) fn set_skill_allocations_at_root(
    root: &Path,
    skill_allocations: Vec<SkillAllocation>,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_set_skill_allocations(character_input, skill_allocations);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character_hub::AbilityScoresDto;
    use codex::saved_character::{
        SavedCharacterEnvelope, SavedCharacterRevisionKind, CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
    };
    use std::path::PathBuf;

    const TEST_SAVED_AT: &str = "2026-07-21T00:00:00Z";
    const GAME_SYSTEM_ID: &str = "pf1";
    const FIGHTER_CLASS_ID: &str = "class:fighter";
    const WIZARD_CLASS_ID: &str = "class:wizard";

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "codex-pf1-adapter-{label}-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    fn request_for(character_id: &str, level: u8) -> CreateCharacterRequest {
        CreateCharacterRequest {
            character_id: character_id.to_owned(),
            display_label: "Pf1Adapter Test Character".to_owned(),
            race_id: "race:human".to_owned(),
            class_id: FIGHTER_CLASS_ID.to_owned(),
            level,
            ability_scores: AbilityScoresDto {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
            ability_bonus_target: "strength".to_owned(),
            saved_at: TEST_SAVED_AT.to_owned(),
        }
    }

    fn wizard_request_for(character_id: &str, level: u8) -> CreateCharacterRequest {
        CreateCharacterRequest { class_id: WIZARD_CLASS_ID.to_owned(), ..request_for(character_id, level) }
    }

    fn arcanist_request_for(character_id: &str, level: u8) -> CreateCharacterRequest {
        CreateCharacterRequest { class_id: ARCANIST_CLASS_ID.to_owned(), ..request_for(character_id, level) }
    }

    fn monk_request_for(character_id: &str, level: u8) -> CreateCharacterRequest {
        CreateCharacterRequest { class_id: MONK_CLASS_ID.to_owned(), ..request_for(character_id, level) }
    }

    fn witch_request_for(character_id: &str, level: u8) -> CreateCharacterRequest {
        CreateCharacterRequest {
            class_id: WITCH_CLASS_ID.to_owned(),
            ..request_for(character_id, level)
        }
    }

    fn shaman_request_for(character_id: &str, level: u8) -> CreateCharacterRequest {
        CreateCharacterRequest {
            class_id: SHAMAN_CLASS_ID.to_owned(),
            ..request_for(character_id, level)
        }
    }

    fn seed_envelope(character_id: &str, level: u8) -> SavedCharacterEnvelope {
        let character_input = compose_character_input(&request_for(character_id, level));
        SavedCharacterEnvelope {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
            display_label: "Pf1Adapter Test Character".to_owned(),
            character_input,
        }
    }

    /// SD-25 carry-forward register A5's RED -> GREEN proof: before this
    /// cycle, `mutate_saved_character_at_root` preserved whatever
    /// `revision_id` was already on disk on every mutating call. Now every
    /// command routed through it — proven here directly for
    /// `level_up_character`, `add_equipment_selection`, and
    /// `add_spell_selection` (the three that live in this crate's own
    /// file-touch grant; `appendToCharacter` shares this exact function so
    /// it advances too, verified indirectly — its own existing tests in
    /// `characterHub::appendToCharacter`, outside this cycle's grant,
    /// continue to pass unchanged) — advances the counter.
    #[test]
    fn level_up_character_at_root_advances_revision_id() {
        let character_id = "pf1-adapter-revision-level-up";
        let root = tempdir("revision-level-up");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = level_up_character_at_root(
            &root,
            FIGHTER_CLASS_ID,
            Vec::new(),
            None,
            "2026-07-21T01:00:00Z",
        )
        .expect("level up call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("Human Fighter level 1 -> 2 must reach Computed, got: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.revision_id,
            format!("{character_id}.rev.2"),
            "level_up_character must advance revision_id, not just saved_at"
        );
        assert_eq!(
            reloaded.latest_authoritative_revision_ref,
            format!("{character_id}.rev.2")
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// v0.6 alpha swarm, task 3: level-up HP + choices persistence.
    /// `LevelUpDialog.onAccept` needs to persist a hit-die roll record, any
    /// feat picks at feat-gaining levels, and a skill-allocation update, all
    /// in the same mutation as the level increment. Hit-die/feat choices are
    /// modeled as generic `SelectedChoice` entries (this crate's existing
    /// extensible convention — see `compose_character_input`'s own
    /// `choice:level_1_character_feat` / `choice:human_ability_bonus`
    /// entries) rather than new dedicated fields, so no schema change is
    /// needed on `ChosenCharacterState`.
    #[test]
    fn level_up_character_at_root_persists_additional_choices_and_skill_allocations() {
        let character_id = "pf1-adapter-level-up-choices";
        let root = tempdir("level-up-choices");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = level_up_character_at_root(
            &root,
            FIGHTER_CLASS_ID,
            vec![
                SelectedChoice {
                    choice_set_id: "choice:level_2_hit_points".to_owned(),
                    selection_id: "hp:average".to_owned(),
                },
                SelectedChoice {
                    choice_set_id: "choice:level_2_bonus_feat".to_owned(),
                    selection_id: "feat:cleave".to_owned(),
                },
            ],
            Some(vec![
                SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 1 },
                SkillAllocation { skill_id: "skill:intimidate".to_owned(), ranks: 1 },
                SkillAllocation { skill_id: "skill:swim".to_owned(), ranks: 1 },
            ]),
            "2026-07-21T01:00:00Z",
        )
        .expect("level up with choices call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "Human Fighter level 1 -> 2 with an additional recorded choice must \
                     still reach Computed (nothing reads these choice_set_ids as a gate), \
                     got: {diagnostics:?}"
                )
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(reloaded.character_input.chosen.class_levels[0].level, 2);
        assert!(
            reloaded.character_input.chosen.selected_choices.iter().any(|choice| {
                choice.choice_set_id == "choice:level_2_hit_points"
                    && choice.selection_id == "hp:average"
            }),
            "the hit-point roll choice must be persisted as a real selected_choices entry"
        );
        assert!(
            reloaded.character_input.chosen.selected_choices.iter().any(|choice| {
                choice.choice_set_id == "choice:level_2_bonus_feat"
                    && choice.selection_id == "feat:cleave"
            }),
            "the feat pick at this feat-gaining level must be persisted as a real \
             selected_choices entry"
        );
        assert_eq!(
            reloaded
                .character_input
                .chosen
                .skill_allocations
                .iter()
                .map(|allocation| (allocation.skill_id.as_str(), allocation.ranks))
                .collect::<Vec<_>>(),
            vec![("skill:climb", 1), ("skill:intimidate", 1), ("skill:swim", 1)],
            "a supplied skill-allocation set must replace chosen.skill_allocations wholesale"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn add_equipment_selection_at_root_advances_revision_id() {
        let character_id = "pf1-adapter-revision-equipment";
        let root = tempdir("revision-equipment");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = add_equipment_selection_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-21T01:00:00Z",
        )
        .expect("add-equipment call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("adding equipment must reach Computed, got: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.revision_id,
            format!("{character_id}.rev.2"),
            "add_equipment_selection must advance revision_id, not just saved_at"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// v0.6 alpha swarm, task 2: skill-point allocation persistence.
    /// `SkillAllocationDialog.onAccept` always sends its complete draft
    /// allocation, so `set_skill_allocations_at_root` must replace
    /// `chosen.skill_allocations` wholesale (not append) while still
    /// advancing the revision counter like every other mutation op.
    #[test]
    fn set_skill_allocations_at_root_advances_revision_id() {
        let character_id = "pf1-adapter-revision-skill-allocations";
        let root = tempdir("revision-skill-allocations");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = set_skill_allocations_at_root(
            &root,
            vec![
                SkillAllocation { skill_id: "skill:swim".to_owned(), ranks: 1 },
                SkillAllocation { skill_id: "skill:intimidate".to_owned(), ranks: 1 },
                SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 1 },
            ],
            "2026-07-21T01:00:00Z",
        )
        .expect("set-skill-allocations call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("re-ordering the supported skill triple must still reach Computed, got: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.revision_id,
            format!("{character_id}.rev.2"),
            "set_skill_allocations must advance revision_id, not just saved_at"
        );
        assert_eq!(
            reloaded
                .character_input
                .chosen
                .skill_allocations
                .iter()
                .map(|allocation| allocation.skill_id.as_str())
                .collect::<Vec<_>>(),
            vec!["skill:swim", "skill:intimidate", "skill:climb"],
            "the on-disk allocation must match the caller's new set/order exactly, proving a full replace rather than an append"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// v0.6 alpha swarm: feat exposure. Appending a feat beyond the fixed
    /// Power Attack/Dodge/Weapon Focus posture must still reach Computed --
    /// `unmet_combat_posture_conditions` only requires `selected_feats` to
    /// *contain* Dodge/Weapon Focus (a `.any(...)` check), not match an
    /// exact set the way `skill_allocations` does, so an appended feat is
    /// additive and safe.
    #[test]
    fn add_feat_selection_at_root_appends_and_persists_when_computed() {
        let character_id = "pf1-adapter-revision-feat";
        let root = tempdir("revision-feat");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = add_feat_selection_at_root(&root, "feat:toughness", None, "2026-07-21T01:00:00Z")
            .expect("add-feat call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("adding a feat must reach Computed, got: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert!(
            reloaded.character_input.chosen.selected_feats.contains(&"feat:toughness".to_owned()),
            "the on-disk envelope must reflect the appended feat: {:?}",
            reloaded.character_input.chosen.selected_feats
        );
        assert_eq!(
            reloaded.revision_id,
            format!("{character_id}.rev.2"),
            "add_feat_selection must advance revision_id, not just saved_at"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- Wizard spellbook posture fix (v0.6 alpha swarm) -----
    //
    // Before this fix, `compose_character_input` never seeded the
    // `choice:wizard_school_specialization` / `choice:wizard_opposed_schools`
    // selections `unmet_wizard_spellbook_conditions` requires, for ANY class
    // -- so a freshly created Wizard character could never reach Computed no
    // matter what spells a tester later selected, since the specialization
    // check fails before the spellbook-content checks are even reached.
    // Frontend verified this live (fresh creation and multiclassing onto an
    // existing character, both correctly Blocked, never persisted) and it
    // directly blocks alpha bar item 3's "select spells at each
    // spell-gaining level" for Wizard specifically.

    #[test]
    fn compose_character_input_seeds_the_canonical_wizard_school_choices_only_for_wizard() {
        let wizard_input = compose_character_input(&wizard_request_for("wizard-school-seed", 1));
        assert!(
            wizard_input.chosen.selected_choices.iter().any(|c| c.choice_set_id
                == "choice:wizard_school_specialization"
                && c.selection_id == "school:evocation"),
            "a composed Wizard must have the canonical Evocation specialization seeded: {:?}",
            wizard_input.chosen.selected_choices
        );
        let opposed: Vec<&str> = wizard_input
            .chosen
            .selected_choices
            .iter()
            .filter(|c| c.choice_set_id == "choice:wizard_opposed_schools")
            .map(|c| c.selection_id.as_str())
            .collect();
        assert_eq!(
            opposed.len(),
            2,
            "a composed Wizard must have exactly two opposed schools seeded: {opposed:?}"
        );
        assert!(opposed.contains(&"school:necromancy"));
        assert!(opposed.contains(&"school:transmutation"));

        // A non-Wizard class must not receive Wizard-only choice seeds --
        // mirrors the existing Human-only / Fighter-only conditional seeding
        // already in this same function.
        let fighter_input = compose_character_input(&request_for("fighter-no-wizard-seed", 1));
        assert!(
            !fighter_input
                .chosen
                .selected_choices
                .iter()
                .any(|c| c.choice_set_id.starts_with("choice:wizard_")),
            "a composed Fighter must not receive Wizard-only school choices: {:?}",
            fighter_input.chosen.selected_choices
        );
    }

    /// The single most important regression guard for this fix: a real
    /// Wizard level 1 character, with one real spell both recorded
    /// (`AcquisitionMode::Known`) and prepared (`AcquisitionMode::Prepared`)
    /// within budget (a 0-level Evocation cantrip -- the specialist school,
    /// costing 1 of the level-0 budget's 3 slots, no opposed-school
    /// penalty), must reach `Computed`. Before this fix this was
    /// structurally unreachable regardless of which spell was picked, since
    /// the specialization check blocked before spellbook content was even
    /// examined. Exercises `build_pilot_headless_receipt` directly (the
    /// same compute path `add_spell_selection_at_root`'s "never persist an
    /// unproven build" gate calls) rather than two sequential
    /// `add_spell_selection_at_root` calls -- the first of those would
    /// itself return `Blocked` (only `Known`, nothing `Prepared` yet, a
    /// real and correct intermediate state) and therefore never persist,
    /// so a second call couldn't build on it; a real UI would send both
    /// selections as part of one accepted "prepare spells for the day"
    /// action, not this test's own concern.
    #[test]
    fn wizard_level1_reaches_computed_once_a_real_spell_is_recorded_and_prepared() {
        let mut character_input = compose_character_input(&wizard_request_for("wizard-spellbook", 1));
        character_input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: WIZARD_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        character_input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: WIZARD_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&character_input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Wizard level 1 with the canonical school seeded and one real spell \
             recorded+prepared within budget must reach Computed, got diagnostics: {:?}",
            receipt.computation.diagnostics
        );
    }

    // ----- Wizard bootstrap-deadlock fix (v0.6 alpha swarm, item 10) -----
    //
    // Even after the school-choice seeding above, a freshly composed Wizard
    // still could never be SAVED: `create_character` only ever persists a
    // build that independently reaches `Computed`, and a Wizard with zero
    // spells never does (`unmet_wizard_spellbook_conditions` requires a
    // non-empty spellbook). No command could grow a spellbook that was
    // never written to disk in the first place -- `add_spell_selection`
    // only sets one `AcquisitionMode` per call and the "never persist an
    // unproven build" invariant discards any call that doesn't
    // independently reach `Computed`, so a Known-only call is discarded, a
    // Prepared-only call is discarded, and neither builds toward the other
    // (proved directly by `add_spell_selection_at_root_cannot_bootstrap_a_wizard_spellbook_from_zero`
    // above). Fix: seed one canonical starter spell, Known+Prepared, in the
    // SAME mutation as the class-level add -- both at `compose_character_input`
    // (fresh creation) and `apply_level_up`'s new-class-entry branch
    // (multiclass dip) -- so a Wizard never exists in a zero-spell state
    // that would need bootstrapping in the first place.

    #[test]
    fn compose_character_input_seeds_the_canonical_wizard_starter_spell_only_for_wizard() {
        let wizard_input = compose_character_input(&wizard_request_for("wizard-starter-spell", 1));
        let known: Vec<&str> = wizard_input
            .chosen
            .spells_selected
            .iter()
            .filter(|s| {
                s.source_class_id == WIZARD_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known
            })
            .map(|s| s.spell_id.as_str())
            .collect();
        let prepared: Vec<&str> = wizard_input
            .chosen
            .spells_selected
            .iter()
            .filter(|s| {
                s.source_class_id == WIZARD_CLASS_ID
                    && s.acquisition_mode == AcquisitionMode::Prepared
            })
            .map(|s| s.spell_id.as_str())
            .collect();
        assert_eq!(
            known,
            vec![WIZARD_STARTER_SPELL_ID],
            "a composed Wizard must have the canonical starter spell recorded as Known: {:?}",
            wizard_input.chosen.spells_selected
        );
        assert_eq!(
            prepared,
            vec![WIZARD_STARTER_SPELL_ID],
            "a composed Wizard must have the canonical starter spell prepared today: {:?}",
            wizard_input.chosen.spells_selected
        );

        // A non-Wizard class must not receive the Wizard-only starter spell
        // -- mirrors the existing Human-only / Fighter-only / Wizard-choice
        // conditional seeding already in this same function.
        let fighter_input = compose_character_input(&request_for("fighter-no-starter-spell", 1));
        assert!(
            fighter_input.chosen.spells_selected.is_empty(),
            "a composed Fighter must not receive the Wizard-only starter spell: {:?}",
            fighter_input.chosen.spells_selected
        );
    }

    /// The direct proof of the fix: a freshly composed Wizard, with NO
    /// manual spell selection added by the caller (unlike the test above,
    /// which manually seeds a spell to prove the school-choice fix in
    /// isolation), must reach `Computed` purely from what
    /// `compose_character_input` itself seeds. This is the exact starting
    /// state `create_character` builds and tries to save.
    #[test]
    fn wizard_level1_reaches_computed_from_compose_character_input_alone() {
        let character_input = compose_character_input(&wizard_request_for("wizard-starter-computed", 1));

        let receipt = build_pilot_headless_receipt(&character_input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a freshly composed Wizard level 1 must reach Computed with no caller-added spells, \
             proving the starter-spell seed alone breaks the bootstrap deadlock: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Arcanist's own Path A gap closure (v0.6 alpha swarm, 2026-07-25):
    /// mirrors `compose_character_input_seeds_the_canonical_wizard_starter_spell_only_for_wizard`'s
    /// own shape, plus the Metamagic Knowledge choice. A composed
    /// Arcanist must have both the starter spell AND the canonical
    /// Metamagic Knowledge choice seeded; a non-Arcanist class must
    /// receive neither.
    #[test]
    fn compose_character_input_seeds_the_canonical_arcanist_starter_spell_and_metamagic_knowledge_only_for_arcanist()
    {
        let arcanist_input = compose_character_input(&arcanist_request_for("arcanist-starter-spell", 1));
        let known: Vec<&str> = arcanist_input
            .chosen
            .spells_selected
            .iter()
            .filter(|s| {
                s.source_class_id == ARCANIST_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known
            })
            .map(|s| s.spell_id.as_str())
            .collect();
        let prepared: Vec<&str> = arcanist_input
            .chosen
            .spells_selected
            .iter()
            .filter(|s| {
                s.source_class_id == ARCANIST_CLASS_ID
                    && s.acquisition_mode == AcquisitionMode::Prepared
            })
            .map(|s| s.spell_id.as_str())
            .collect();
        assert_eq!(known, vec![ARCANIST_STARTER_SPELL_ID], "{:?}", arcanist_input.chosen.spells_selected);
        assert_eq!(prepared, vec![ARCANIST_STARTER_SPELL_ID], "{:?}", arcanist_input.chosen.spells_selected);

        let metamagic_choice = arcanist_input
            .chosen
            .selected_choices
            .iter()
            .find(|c| c.choice_set_id == ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID)
            .expect("a composed Arcanist must have the canonical Metamagic Knowledge choice seeded");
        assert_eq!(metamagic_choice.selection_id, EMPOWER_SPELL_METAMAGIC_SELECTION);

        let fighter_input = compose_character_input(&request_for("fighter-no-arcanist-seed", 1));
        assert!(
            fighter_input.chosen.spells_selected.is_empty(),
            "a composed Fighter must not receive the Arcanist-only starter spell: {:?}",
            fighter_input.chosen.spells_selected
        );
        assert!(
            !fighter_input
                .chosen
                .selected_choices
                .iter()
                .any(|c| c.choice_set_id == ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID),
            "a composed Fighter must not receive the Arcanist-only Metamagic Knowledge choice: {:?}",
            fighter_input.chosen.selected_choices
        );
    }

    /// **The milestone test**: a freshly composed Arcanist, with NO
    /// manual spell/choice selection added by the caller, must reach
    /// `Computed` purely from what `compose_character_input` itself
    /// seeds -- mirroring `wizard_level1_reaches_computed_from_compose_character_input_alone`
    /// exactly. This is the exact starting state `create_character`
    /// builds and tries to save -- proving Arcanist is now genuinely
    /// product-reachable through the real creation flow, not just
    /// engine-Computed in a hand-built test fixture.
    #[test]
    fn arcanist_level1_reaches_computed_from_compose_character_input_alone() {
        let character_input = compose_character_input(&arcanist_request_for("arcanist-starter-computed", 1));

        let receipt = build_pilot_headless_receipt(&character_input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a freshly composed Arcanist level 1 must reach Computed with no caller-added \
             spells/choices, proving the starter-spell + Metamagic Knowledge seed together \
             break the last remaining gap: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Backlog item 9a (risks-and-open-questions.md): `spells_selected`
    /// wasn't exposed through `load_saved_character`'s response, same shape
    /// of gap as item 8's `selected_feats`. A composed Wizard already has
    /// the seeded starter spell (Known + Prepared), so this is a real
    /// non-empty round-trip proof, not just the empty case.
    #[test]
    fn load_saved_character_surfaces_persisted_spells_selected() {
        let character_id = "pf1-adapter-spells-selected-exposure";
        let root = tempdir("spells-selected-exposure");
        let envelope = SavedCharacterEnvelope {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
            display_label: "Pf1Adapter Spells Selected Exposure Test".to_owned(),
            character_input: compose_character_input(&wizard_request_for(character_id, 1)),
        };
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let loaded = Pf1Adapter
            .load_saved_character(&root)
            .expect("load_saved_character should succeed");

        assert_eq!(
            loaded.spells_selected.len(),
            2,
            "the seeded starter spell's Known + Prepared entries must both surface: {:?}",
            loaded.spells_selected
        );
        assert!(loaded.spells_selected.iter().any(|s| s.spell_id == WIZARD_STARTER_SPELL_ID
            && s.source_class_id == WIZARD_CLASS_ID
            && matches!(s.acquisition_mode, crate::character_hub::AcquisitionModeDto::Known)));
        assert!(loaded.spells_selected.iter().any(|s| s.spell_id == WIZARD_STARTER_SPELL_ID
            && s.source_class_id == WIZARD_CLASS_ID
            && matches!(s.acquisition_mode, crate::character_hub::AcquisitionModeDto::Prepared)));

        std::fs::remove_dir_all(&root).ok();
    }

    /// A minimal saved character for the chooser-target tests below.
    fn chooser_test_envelope(character_id: &str) -> SavedCharacterEnvelope {
        SavedCharacterEnvelope {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
            display_label: "Chooser Target Test".to_owned(),
            character_input: compose_character_input(&request_for("race:human", 1)),
        }
    }

    /// Every freshly created character carries the canonical Fighter
    /// bonus-feat seed (`choice:fighter_bonus_feat ->
    /// feat:weapon_focus:weapon:longsword`), so the payload must surface a
    /// real, non-empty target rather than only proving the empty case.
    #[test]
    fn load_saved_character_surfaces_resolved_chooser_feat_targets() {
        let character_id = "pf1-adapter-chooser-targets-exposure";
        let root = tempdir("chooser-targets-exposure");
        let envelope = SavedCharacterEnvelope {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
            display_label: "Pf1Adapter Chooser Targets Exposure Test".to_owned(),
            character_input: compose_character_input(&request_for("race:human", 1)),
        };
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let loaded = Pf1Adapter
            .load_saved_character(&root)
            .expect("load_saved_character should succeed");

        let weapon_focus = loaded
            .chosen_feat_targets
            .iter()
            .find(|entry| entry.feat_id.contains("weapon_focus") || entry.feat_id == "Weapon Focus")
            .unwrap_or_else(|| {
                panic!(
                    "Weapon Focus's seeded target must surface: {:?}",
                    loaded.chosen_feat_targets
                )
            });
        assert_eq!(weapon_focus.target_kind, "Weapon");
        assert_eq!(
            weapon_focus.targets,
            vec!["longsword".to_owned()],
            "the seeded legacy compound target must resolve, not read as untargeted"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// The write path this whole task exists to create: a chooser feat added
    /// with a target must persist a real `SelectedChoice`, and that target
    /// must come back out through the load payload.
    #[test]
    fn adding_a_chooser_feat_with_a_target_persists_and_surfaces_it() {
        let root = tempdir("add-chooser-feat-with-target");
        let envelope = chooser_test_envelope("add-chooser-feat-with-target");
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        add_feat_selection_at_root(&root, "Skill Focus", Some("Perception"), TEST_SAVED_AT)
            .expect("adding a chooser feat with a target should succeed");

        let loaded = Pf1Adapter
            .load_saved_character(&root)
            .expect("load_saved_character should succeed");

        let skill_focus = loaded
            .chosen_feat_targets
            .iter()
            .find(|entry| entry.feat_id == "Skill Focus")
            .unwrap_or_else(|| {
                panic!("Skill Focus must surface: {:?}", loaded.chosen_feat_targets)
            });
        assert_eq!(skill_focus.target_kind, "Skill");
        assert_eq!(skill_focus.targets, vec!["Perception".to_owned()]);

        std::fs::remove_dir_all(&root).ok();
    }

    /// A chooser feat may legitimately be added without naming its target
    /// yet. That must persist the feat and record no choice -- never a
    /// seeded default, per the same no-silent-seeding rule the producers
    /// follow.
    #[test]
    fn adding_a_chooser_feat_without_a_target_seeds_nothing() {
        let root = tempdir("add-chooser-feat-no-target");
        let envelope = chooser_test_envelope("add-chooser-feat-no-target");
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        add_feat_selection_at_root(&root, "Skill Focus", None, TEST_SAVED_AT)
            .expect("adding a chooser feat without a target should succeed");

        let loaded = Pf1Adapter
            .load_saved_character(&root)
            .expect("load_saved_character should succeed");

        let skill_focus = loaded
            .chosen_feat_targets
            .iter()
            .find(|entry| entry.feat_id == "Skill Focus")
            .expect("the feat is held, so it must be reported as untargeted");
        assert!(
            skill_focus.targets.is_empty(),
            "no target may be invented: {skill_focus:?}"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_target_for_a_feat_that_takes_none_is_rejected() {
        assert!(resolve_feat_target_choice("feat:toughness", Some("Longsword")).is_err());
        assert!(resolve_feat_target_choice("Weapon Focus", Some("   ")).is_err());
        // The legitimate shapes still pass.
        assert!(resolve_feat_target_choice("feat:toughness", None).is_ok());
        assert!(resolve_feat_target_choice("Weapon Focus", Some("Rapier")).is_ok());
    }

    /// The prefix and choice set must come from the feat's own contract, not
    /// be assembled by callers -- otherwise a caller could write a selection
    /// the producers cannot read.
    #[test]
    fn a_resolved_target_uses_the_feats_own_contract() {
        let choice = resolve_feat_target_choice("Improved Critical", Some("Rapier"))
            .expect("valid")
            .expect("a chooser feat with a target yields a choice");
        assert_eq!(choice.choice_set_id, "choice:improved_critical_target");
        assert_eq!(choice.selection_id, "weapon:Rapier");
    }

    /// The multiclass-dip mirror of the test above: leveling a Wizard class
    /// entry onto an existing character (not fresh creation) must also
    /// reach `Computed` with no manual spell selection, proving
    /// `apply_level_up`'s new-class-entry branch seeds the same starter
    /// spell `compose_character_input` does.
    #[test]
    fn wizard_multiclass_dip_reaches_computed_from_apply_level_up_alone() {
        let mut character_input = compose_character_input(&request_for("fighter-then-wizard-dip", 1));
        apply_level_up(&mut character_input, WIZARD_CLASS_ID);

        let receipt = build_pilot_headless_receipt(&character_input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "multiclassing Wizard onto an existing Fighter must reach Computed with no \
             caller-added spells, proving apply_level_up's new-class-entry branch seeds the \
             same starter spell compose_character_input does: {:?}",
            receipt.computation.diagnostics
        );
    }

    // ----- Rogue end-to-end UI reachability audit (v0.6 alpha swarm) -----
    //
    // Investigated with the same rigor as the Wizard chain: does Rogue
    // actually reach Computed through the real UI creation/level-up path,
    // not just via hand-built test fixtures? Unlike Wizard, Rogue has no
    // bespoke extra posture gate anywhere in pilot_compute.rs (no
    // equivalent of unmet_wizard_spellbook_conditions) -- the one
    // class-conditional sub-check in unmet_combat_posture_conditions is an
    // `is_some()` guard that only ADDS a requirement when Fighter is the
    // dispatch-supported class; it's skipped entirely for Rogue, never
    // blocks it. compose_character_input has no Rogue-specific branch at
    // all (only Wizard gets conditional seeding) -- Rogue receives the
    // exact same fixed loadout as every other non-Wizard class, and task
    // 4's generic chassis dispatch widening already covers it. These two
    // tests confirm empirically, through the real compose_character_input /
    // apply_level_up path (not a hand-built CharacterInput), that this
    // holds for both creation and multiclass dip -- no fix needed, Rogue
    // was already fully reachable.

    #[test]
    fn rogue_level1_reaches_computed_from_compose_character_input_alone() {
        let rogue_request = CreateCharacterRequest {
            class_id: "class:rogue".to_owned(),
            ..request_for("rogue-starter-computed", 1)
        };
        let character_input = compose_character_input(&rogue_request);

        let receipt = build_pilot_headless_receipt(&character_input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a freshly composed Rogue level 1 must reach Computed through the real UI creation \
             path, with no gap analogous to Wizard's spellbook posture: {:?}",
            receipt.computation.diagnostics
        );
    }

    #[test]
    fn rogue_multiclass_dip_reaches_computed_from_apply_level_up_alone() {
        let mut character_input = compose_character_input(&request_for("fighter-then-rogue-dip", 1));
        apply_level_up(&mut character_input, "class:rogue");

        let receipt = build_pilot_headless_receipt(&character_input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "multiclassing Rogue onto an existing Fighter must reach Computed through the real \
             UI level-up path, with no seeding fix needed (unlike Wizard): {:?}",
            receipt.computation.diagnostics
        );
    }

    // ----- Monk end-to-end UI reachability (v0.6 alpha swarm, Path A) -----
    //
    // Monk's engine can already compute a full level-1 build; its one
    // remaining claim-blocking diagnostic
    // (`class_feature.monk.bounded_progression.bonus_feat.unsupported`)
    // fires only because nothing seeds `choice:monk_bonus_feat`, exactly
    // the Sorcerer/Cleric/Druid "no picker exists for this choice set"
    // shape. See `MONK_CLASS_ID`'s own doc comment for why Dodge is the
    // canonical pick.

    /// **The milestone test**: a freshly composed Monk, with NO choice
    /// added by the caller, must reach `Computed` purely from what
    /// `compose_character_input` itself seeds -- mirroring
    /// `arcanist_level1_reaches_computed_from_compose_character_input_alone`
    /// and `wizard_level1_reaches_computed_from_compose_character_input_alone`
    /// exactly. This is the exact starting state `create_character` builds
    /// and tries to save.
    #[test]
    fn monk_level1_reaches_computed_from_compose_character_input_alone() {
        let character_input = compose_character_input(&monk_request_for("monk-starter-computed", 1));

        let receipt = build_pilot_headless_receipt(&character_input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a freshly composed Monk level 1 must reach Computed with no caller-added choices, \
             proving the canonical Dodge bonus-feat seed closes the last remaining gap: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The seed must be genuinely active, not merely recorded: the engine's
    /// `dodge_bonus_feat_is_genuinely_active` gate requires `feat:dodge` to
    /// really be present on `selected_feats` (a Monk who names Dodge in the
    /// slot but does not carry the feat still, correctly, blocks). This
    /// pins that the composed posture satisfies BOTH halves, so the +1
    /// dodge AC bonus this closure claims is one the character really has
    /// -- not a token that merely silences a diagnostic.
    #[test]
    fn composed_monk_carries_both_halves_of_the_dodge_bonus_feat_seed() {
        let character_input = compose_character_input(&monk_request_for("monk-dodge-halves", 1));

        assert!(
            character_input.chosen.selected_choices.iter().any(|c| {
                c.choice_set_id == MONK_BONUS_FEAT_CHOICE_ID
                    && c.selection_id == DODGE_FEAT_SELECTION
            }),
            "the composed Monk must carry the canonical bonus-feat choice: {:?}",
            character_input.chosen.selected_choices
        );
        assert!(
            character_input
                .chosen
                .selected_feats
                .iter()
                .any(|feat| feat == DODGE_FEAT_SELECTION),
            "the composed Monk must genuinely carry feat:dodge, or the seeded choice would be \
             an unmet precondition rather than a real grant: {:?}",
            character_input.chosen.selected_feats
        );

        let receipt = build_pilot_headless_receipt(&character_input);
        assert!(
            receipt.computation.explanations.iter().any(
                |e| e.id == "class_feature.monk.bounded_progression.bonus_feat.dodge_active"
            ),
            "the engine must emit the real dodge_active grounding record, proving the seed is \
             resolved rather than merely tolerated: {:?}",
            receipt.computation.explanations
        );
    }

    /// The composed Witch really carries the canonical Flight hex, and it
    /// really reaches `Computed` through the production compose path --
    /// both halves, so a seed that silences a diagnostic without producing
    /// a computable character would still fail here.
    #[test]
    fn a_composed_witch_gets_the_canonical_flight_hex_and_computes() {
        let witch_input = compose_character_input(&witch_request_for("witch-seed", 1));

        assert!(
            witch_input.chosen.selected_choices.iter().any(|c| {
                c.choice_set_id == WITCH_HEX_CHOICE_ID
                    && c.selection_id == FLIGHT_HEX_SELECTION
            }),
            "a composed Witch must carry the canonical Flight hex: {:?}",
            witch_input.chosen.selected_choices
        );

        let receipt = build_pilot_headless_receipt(&witch_input);
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a composed Witch must reach Computed: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The composed Shaman really carries the canonical Life Spirit and
    /// really computes.
    #[test]
    fn a_composed_shaman_gets_the_canonical_life_spirit_and_computes() {
        let shaman_input = compose_character_input(&shaman_request_for("shaman-seed", 1));

        assert!(
            shaman_input.chosen.selected_choices.iter().any(|c| {
                c.choice_set_id == SHAMAN_SPIRIT_CHOICE_ID
                    && c.selection_id == LIFE_SPIRIT_SELECTION
            }),
            "a composed Shaman must carry the canonical Life Spirit: {:?}",
            shaman_input.chosen.selected_choices
        );

        let receipt = build_pilot_headless_receipt(&shaman_input);
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a composed Shaman must reach Computed: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Both seeds are class-scoped: no other class may pick either up,
    /// mirroring `a_composed_fighter_gets_no_arcanist_seed`'s own negative
    /// check. Also pins that Witch and Shaman do NOT cross-seed each other
    /// -- their hex/spirit lists are parallel but entirely distinct corpus
    /// records (Shaman's `Shaman Hex ~ Charm` carries its own
    /// `ShamanCharmHexDuration`/`ShamanCharmHexDC`, Witch's carries
    /// `WitchCharmSteps`/`WitchHexDC_Charm`), so a shared seed would be
    /// exactly the "shared name is not a shared thing" error.
    #[test]
    fn the_witch_and_shaman_seeds_do_not_leak_across_classes() {
        let fighter_input = compose_character_input(&request_for("fighter-no-seed", 1));
        for choice_set in [WITCH_HEX_CHOICE_ID, SHAMAN_SPIRIT_CHOICE_ID] {
            assert!(
                !fighter_input
                    .chosen
                    .selected_choices
                    .iter()
                    .any(|c| c.choice_set_id == choice_set),
                "a composed Fighter must not receive {choice_set}: {:?}",
                fighter_input.chosen.selected_choices
            );
        }

        let witch_input = compose_character_input(&witch_request_for("witch-no-spirit", 1));
        assert!(
            !witch_input
                .chosen
                .selected_choices
                .iter()
                .any(|c| c.choice_set_id == SHAMAN_SPIRIT_CHOICE_ID),
            "a composed Witch must not receive the Shaman Spirit choice: {:?}",
            witch_input.chosen.selected_choices
        );

        let shaman_input = compose_character_input(&shaman_request_for("shaman-no-hex", 1));
        assert!(
            !shaman_input
                .chosen
                .selected_choices
                .iter()
                .any(|c| c.choice_set_id == WITCH_HEX_CHOICE_ID),
            "a composed Shaman must not receive the Witch Hex choice: {:?}",
            shaman_input.chosen.selected_choices
        );
    }

    /// Leveling a seeded Witch/Shaman must stay `Computed` at every step to
    /// the PF1 cap. `apply_level_up` takes the increment-existing-level
    /// branch for both, so the creation-time seed simply persists and no
    /// second seeding site is owed -- established empirically here rather
    /// than assumed, exactly as `monk_stays_computed_leveling_all_the_way_to_20`
    /// did for Monk.
    #[test]
    fn the_seeded_witch_and_shaman_stay_computed_leveling_all_the_way_to_20() {
        for (class_id, request) in [
            (WITCH_CLASS_ID, witch_request_for("witch-level-sweep", 1)),
            (SHAMAN_CLASS_ID, shaman_request_for("shaman-level-sweep", 1)),
        ] {
            let mut character_input = compose_character_input(&request);

            for expected_level in 2..=20u8 {
                apply_level_up(&mut character_input, class_id);

                assert_eq!(
                    character_input.chosen.class_levels,
                    vec![CharacterClassLevel {
                        class_id: class_id.to_owned(),
                        level: expected_level
                    }],
                    "apply_level_up must increment the existing {class_id} entry"
                );

                let receipt = build_pilot_headless_receipt(&character_input);
                assert_eq!(
                    receipt.status,
                    HeadlessReceiptStatus::Computed,
                    "{class_id} leveled to {expected_level} through the real level-up path \
                     must stay Computed: {:?}",
                    receipt.computation.diagnostics
                );
            }
        }
    }

    /// The seed is Monk-only: no other class may pick up a
    /// `choice:monk_bonus_feat` entry, mirroring
    /// `a_composed_fighter_gets_no_arcanist_seed`'s own negative check.
    #[test]
    fn a_composed_fighter_gets_no_monk_bonus_feat_seed() {
        let fighter_input = compose_character_input(&request_for("fighter-no-monk-seed", 1));

        assert!(
            !fighter_input
                .chosen
                .selected_choices
                .iter()
                .any(|c| c.choice_set_id == MONK_BONUS_FEAT_CHOICE_ID),
            "a composed Fighter must not receive the Monk-only bonus-feat choice: {:?}",
            fighter_input.chosen.selected_choices
        );
    }

    /// Leveling a seeded Monk must stay `Computed` at every step to the PF1
    /// cap: `apply_level_up` takes the increment-existing-level branch here,
    /// so the creation-time seed simply persists and no second seeding site
    /// is owed. This also pins that the further bonus-feat slots PF1 grants
    /// at Monk 2/6/10 (`choice:monk_bonus_feat_2/_3/_4`) are
    /// recognized-when-present but never claim-blocking, so widening the
    /// level range cannot silently reintroduce a blocker.
    #[test]
    fn monk_stays_computed_leveling_all_the_way_to_20() {
        let mut character_input = compose_character_input(&monk_request_for("monk-level-sweep", 1));

        for expected_level in 2..=20u8 {
            apply_level_up(&mut character_input, MONK_CLASS_ID);

            assert_eq!(
                character_input.chosen.class_levels,
                vec![CharacterClassLevel {
                    class_id: MONK_CLASS_ID.to_owned(),
                    level: expected_level
                }],
                "apply_level_up must increment the existing Monk entry, not add a second one"
            );

            let receipt = build_pilot_headless_receipt(&character_input);
            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Computed,
                "a Monk leveled up to {expected_level} through the real level-up path must stay \
                 Computed: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// The creation-side mirror: `levelOptions` in `characterHubModel.ts`
    /// feeds the create-character form's level dropdown, which composes
    /// directly at the chosen level rather than leveling up to it. The
    /// engine computes Monk at all 20 (`cargo run --bin v06_class_state_dump`
    /// reports `levels_blocked: []`), and this pins that same truth through
    /// the real production compose path -- so the UI's conservative
    /// `[1]` range is a deliberate, documented UI-verification lag, not an
    /// engine limit hiding behind it.
    #[test]
    fn monk_reaches_computed_at_every_created_level() {
        for level in 1..=20u8 {
            let character_input =
                compose_character_input(&monk_request_for("monk-created-at-level", level));
            let receipt = build_pilot_headless_receipt(&character_input);

            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Computed,
                "a freshly composed Monk at level {level} must reach Computed: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// Monk's `CLASS_OPTIONS` entry in `characterHubModel.ts` is being
    /// promoted from `human-diagnostics-only` to `full`, and that label
    /// means specifically "reaches `Computed` for ANY race in
    /// `RACE_OPTIONS`". Nothing in the Monk seam is race-gated
    /// (`supported_monk_level` matches on class levels only, and the Dodge
    /// cross-check reads `selected_feats`, which this loadout seeds for
    /// every race), but the label must be earned rather than assumed -- so
    /// this pins it against the real race roster the UI actually offers.
    #[test]
    fn monk_level1_reaches_computed_for_every_race_the_ui_offers() {
        for race_id in [
            "race:human",
            "race:dwarf",
            "race:elf",
            "race:gnome",
            "race:half-elf",
            "race:half-orc",
            "race:halfling",
        ] {
            let request = CreateCharacterRequest {
                race_id: race_id.to_owned(),
                ..monk_request_for("monk-any-race", 1)
            };
            let receipt = build_pilot_headless_receipt(&compose_character_input(&request));

            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Computed,
                "Monk level 1 must reach Computed for {race_id}, or the `full` support label in \
                 characterHubModel.ts's CLASS_OPTIONS would overclaim: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// The multiclass-dip mirror. Unlike Wizard -- whose
    /// `unmet_wizard_spellbook_conditions` gate is race/multiclass-blind and
    /// therefore genuinely needed a SECOND seeding site in
    /// `apply_level_up`'s new-class-entry branch -- Monk's whole
    /// bonus-feat seam sits behind `supported_monk_level`, which matches
    /// only a SINGLE-class Monk (`[class_level]`). A Fighter who dips Monk
    /// never reaches that seam at all. This test pins that empirically, so
    /// the "one site or two?" question is answered by the engine rather
    /// than assumed -- and it is a real Computed assertion, not merely
    /// "the Monk diagnostic is absent".
    #[test]
    fn monk_multiclass_dip_reaches_computed_from_apply_level_up_alone() {
        let mut character_input = compose_character_input(&request_for("fighter-then-monk-dip", 1));
        apply_level_up(&mut character_input, MONK_CLASS_ID);

        let receipt = build_pilot_headless_receipt(&character_input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "multiclassing Monk onto an existing Fighter must reach Computed through the real \
             UI level-up path: {:?}",
            receipt.computation.diagnostics
        );
    }

    // ----- Real spell slot-budget enforcement (v0.6 alpha swarm) -----
    //
    // Frontend found this live while chasing an unrelated cosmetic issue:
    // parse_wizard_spellbook_spell_id (pilot_compute.rs) only ever
    // recognized the synthetic `<school>.<level>.<name>` convention, so
    // real catalog spell_ids (e.g. "Magic Missile", the literal key
    // spell_catalog.rs hands the frontend picker) silently failed to parse
    // and were dropped from unmet_wizard_spellbook_conditions's slot-budget
    // consumption count -- a Wizard could add unlimited real spells with
    // zero slot enforcement. Fixed by teaching that function to resolve
    // real SPELL_LIST keys directly (no corpus needed -- it's a compiled-in
    // static table, not the same headless/corpus-aware wall that blocked
    // AC widening).
    //
    // This test reproduces frontend's exact live-verified numbers through
    // the real command surface (not a parser-only unit test, per explicit
    // instruction): a level-1 Wizard's 1st-level budget is 2 slots (base 1
    // + specialist bonus 1, Int 10 = +0 bonus). Magic Missile (Evocation,
    // cost 1) + Alarm (Abjuration, cost 1) = 2 consumed, exactly at
    // capacity -- both accepted. Grease (Conjuration, cost 1) as a third
    // prepared 1st-level spell pushes consumption to 3, over budget --
    // must be honestly Blocked and never persisted.
    #[test]
    fn wizard_spell_slot_budget_rejects_a_third_spell_that_exceeds_capacity() {
        let character_id = "pf1-adapter-wizard-slot-budget";
        let root = tempdir("wizard-slot-budget");
        let mut character_input = compose_character_input(&wizard_request_for(character_id, 1));
        character_input.chosen.spells_selected.clear();
        let envelope = SavedCharacterEnvelope {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
            display_label: "Pf1Adapter Wizard Slot Budget Test".to_owned(),
            character_input,
        };
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let first = record_and_prepare_spell_selection_at_root(
            &root,
            "Magic Missile",
            WIZARD_CLASS_ID,
            "2026-07-23T01:00:00Z",
        )
        .expect("first spell call should not error");
        assert!(
            matches!(first, CreateCharacterResponse::Saved { .. }),
            "Magic Missile alone (1 of 2 slots) must be accepted: {first:?}"
        );

        let second = record_and_prepare_spell_selection_at_root(
            &root,
            "Alarm",
            WIZARD_CLASS_ID,
            "2026-07-23T02:00:00Z",
        )
        .expect("second spell call should not error");
        assert!(
            matches!(second, CreateCharacterResponse::Saved { .. }),
            "Magic Missile + Alarm (2 of 2 slots, exactly at capacity) must be accepted: \
             {second:?}"
        );

        let third = record_and_prepare_spell_selection_at_root(
            &root,
            "Grease",
            WIZARD_CLASS_ID,
            "2026-07-23T03:00:00Z",
        )
        .expect("third spell call should not error");
        assert!(
            matches!(third, CreateCharacterResponse::Blocked { .. }),
            "a third prepared 1st-level spell (3 of 2 slots) must be honestly Blocked, proving \
             real catalog spells now count against the slot budget: {third:?}"
        );

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        let prepared: Vec<&str> = reloaded
            .character_input
            .chosen
            .spells_selected
            .iter()
            .filter(|s| s.acquisition_mode == AcquisitionMode::Prepared)
            .map(|s| s.spell_id.as_str())
            .collect();
        assert_eq!(
            prepared,
            vec!["Magic Missile", "Alarm"],
            "the Blocked third call must not have persisted Grease: {prepared:?}"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// The single most important regression guard for item 3: proves the
    /// bootstrap deadlock is real through the actual persistence command
    /// (add_spell_selection_at_root, called twice, single-mode each time,
    /// exactly how a real UI would call the existing command) and that
    /// record_and_prepare_spell_selection_at_root breaks it.
    ///
    /// A real `compose_character_input`-created Wizard no longer starts
    /// with an empty spellbook (a later fix seeds one canonical starter
    /// spell precisely to route around this deadlock at class-acquisition
    /// time, since nothing could otherwise grow a spellbook that could
    /// never be saved in the first place) -- so this test explicitly clears
    /// `spells_selected` back to empty first, to keep proving the
    /// underlying single-mode-mutation mechanism this deadlock came from,
    /// as a standing regression guard for why the starter-spell seed
    /// exists.
    #[test]
    fn add_spell_selection_at_root_cannot_bootstrap_a_wizard_spellbook_from_zero() {
        let character_id = "pf1-adapter-wizard-bootstrap-deadlock";
        let root = tempdir("wizard-bootstrap-deadlock");
        let mut character_input = compose_character_input(&wizard_request_for(character_id, 1));
        character_input.chosen.spells_selected.clear();
        let envelope = SavedCharacterEnvelope {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
            display_label: "Pf1Adapter Wizard Bootstrap Test".to_owned(),
            character_input,
        };
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let known_only = add_spell_selection_at_root(
            &root,
            "Light",
            WIZARD_CLASS_ID,
            AcquisitionMode::Known,
            "2026-07-21T01:00:00Z",
        )
        .expect("known-only call should not error");
        assert!(
            matches!(known_only, CreateCharacterResponse::Blocked { .. }),
            "recording alone (nothing prepared yet) must stay honestly Blocked, proving the \
             deadlock is real: {known_only:?}"
        );

        let prepared_only = add_spell_selection_at_root(
            &root,
            "Light",
            WIZARD_CLASS_ID,
            AcquisitionMode::Prepared,
            "2026-07-21T02:00:00Z",
        )
        .expect("prepared-only call should not error");
        assert!(
            matches!(prepared_only, CreateCharacterResponse::Blocked { .. }),
            "the first call never persisted, so this loads the original empty spellbook again \
             and is also Blocked (prepared spell isn't in the still-empty recorded set): \
             {prepared_only:?}"
        );

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert!(
            reloaded.character_input.chosen.spells_selected.is_empty(),
            "neither Blocked call should have persisted anything: {:?}",
            reloaded.character_input.chosen.spells_selected
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// The fix: one atomic call breaks the deadlock the previous test
    /// proves. Same explicit-empty-spellbook setup as that test, for the
    /// same reason (a real `compose_character_input`-created Wizard no
    /// longer starts empty since the starter-spell seed landed).
    #[test]
    fn record_and_prepare_spell_selection_at_root_breaks_the_bootstrap_deadlock() {
        let character_id = "pf1-adapter-wizard-bootstrap-fix";
        let root = tempdir("wizard-bootstrap-fix");
        let mut character_input = compose_character_input(&wizard_request_for(character_id, 1));
        character_input.chosen.spells_selected.clear();
        let envelope = SavedCharacterEnvelope {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
            display_label: "Pf1Adapter Wizard Bootstrap Fix Test".to_owned(),
            character_input,
        };
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = record_and_prepare_spell_selection_at_root(
            &root,
            "Light",
            WIZARD_CLASS_ID,
            "2026-07-21T01:00:00Z",
        )
        .expect("record-and-prepare call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "one atomic record+prepare call must reach Computed on the first spell, \
                     breaking the bootstrap deadlock, got: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        let spells = &reloaded.character_input.chosen.spells_selected;
        assert_eq!(spells.len(), 2, "must persist both the Known and Prepared entries: {spells:?}");
        assert!(spells.iter().any(|s| s.acquisition_mode == AcquisitionMode::Known));
        assert!(spells.iter().any(|s| s.acquisition_mode == AcquisitionMode::Prepared));

        // Once bootstrapped, the plain single-mode command works normally
        // for a second spell -- proves this fix doesn't change
        // add_spell_selection's own behavior, just breaks the first-spell
        // deadlock.
        let second = add_spell_selection_at_root(
            &root,
            "Light",
            WIZARD_CLASS_ID,
            AcquisitionMode::Known,
            "2026-07-21T02:00:00Z",
        )
        .expect("a second known-only call should not error");
        assert!(
            matches!(second, CreateCharacterResponse::Saved { .. }),
            "once bootstrapped, plain add_spell_selection must work normally: {second:?}"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn add_spell_selection_at_root_advances_revision_id() {
        let character_id = "pf1-adapter-revision-spell";
        let root = tempdir("revision-spell");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = add_spell_selection_at_root(
            &root,
            "Mage Armor",
            "class:wizard",
            AcquisitionMode::Known,
            "2026-07-21T01:00:00Z",
        )
        .expect("add-spell call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("adding a spell must reach Computed, got: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.revision_id,
            format!("{character_id}.rev.2"),
            "add_spell_selection must advance revision_id, not just saved_at"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// A second consecutive mutation keeps advancing (`.rev.2` -> `.rev.3`),
    /// proving the counter is real, not a second hardcoded constant —
    /// mirrors `reSaveCharacter`'s own equivalent proof test.
    #[test]
    fn mutate_saved_character_at_root_keeps_advancing_across_repeated_calls() {
        let character_id = "pf1-adapter-revision-repeated";
        let root = tempdir("revision-repeated");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        level_up_character_at_root(
            &root,
            FIGHTER_CLASS_ID,
            Vec::new(),
            None,
            "2026-07-21T01:00:00Z",
        )
        .expect("first level up should not error");
        let after_first = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(after_first.revision_id, format!("{character_id}.rev.2"));

        add_equipment_selection_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-21T02:00:00Z",
        )
        .expect("add-equipment should not error");
        let after_second = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(after_second.revision_id, format!("{character_id}.rev.3"));

        std::fs::remove_dir_all(&root).ok();
    }

    /// `Pf1Adapter::level_up` (the trait-surface-named inherent method) must
    /// delegate to the exact same real implementation, not a second
    /// parallel one.
    #[test]
    fn pf1_adapter_level_up_delegates_to_the_real_implementation() {
        let character_id = "pf1-adapter-struct-level-up";
        let root = tempdir("struct-level-up");
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let adapter = Pf1Adapter;
        let response = adapter
            .level_up(&root, FIGHTER_CLASS_ID, Vec::new(), None, "2026-07-21T01:00:00Z")
            .expect("level up call should not error");

        match response {
            CreateCharacterResponse::Saved { summary, .. } => {
                assert_eq!(summary.class_summary, "class:fighter:2");
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("Human Fighter level 1 -> 2 must reach Computed, got: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(reloaded.revision_id, format!("{character_id}.rev.2"));

        std::fs::remove_dir_all(&root).ok();
    }

    /// `impl RuleSystemAdapter for Pf1Adapter` — proves `Pf1Adapter` genuinely
    /// implements the full seven-method trait surface (criterion 3.1's
    /// `rule_system_adapter.rs`) with real, wired behavior end to end, the
    /// same way `rule_system_adapter.rs`'s own `TestPf1Delegate` test proved
    /// the trait was *implementable* — this proves the *real* extracted
    /// `Pf1Adapter` type is the implementation, not a parallel test-only one.
    #[test]
    fn pf1_adapter_implements_rule_system_adapter_end_to_end() {
        let adapter: Box<dyn RuleSystemAdapter> = Box::new(Pf1Adapter);
        assert_eq!(adapter.rule_system_id(), "pf1");

        let character_id = "pf1-adapter-trait-impl";
        let characters_root = tempdir("trait-impl-root");
        let root = characters_root.join(character_id);
        let envelope = seed_envelope(character_id, 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        // chassis_resolve: real compute over a real CharacterInput.
        let chassis = adapter.chassis_resolve(&envelope.character_input);
        assert_eq!(chassis.base_attack_bonus, 1, "human fighter level 1 chassis");

        // level_up: register A2's multiclass-safe dispatch, exercised via a
        // single-delta call here (the multiclass case is proven directly in
        // `src/rules_core/level_up.rs`'s own tests).
        let deltas = [ClassLevelDelta {
            class_id: FIGHTER_CLASS_ID.to_owned(),
            from_level: 1,
            to_level: 2,
        }];
        let plan = adapter.level_up(&envelope.character_input, &deltas);
        assert!(
            !plan.automatic_features.is_empty(),
            "Fighter 1 -> 2 should grant at least one automatic feature"
        );

        // list_saved_characters / load_saved_character: real disk I/O,
        // reusing `character_hub`'s own mapping instead of re-deriving it.
        let listing = adapter
            .list_saved_characters(&characters_root)
            .expect("list_saved_characters should succeed");
        assert!(listing.characters.iter().any(|c| c.character_id == character_id));

        let loaded = adapter
            .load_saved_character(&root)
            .expect("load_saved_character should succeed");
        assert_eq!(loaded.summary.character_id, character_id);
        assert!(loaded.snapshot.is_some());
        // Backlog item 8 (risks-and-open-questions.md): the Feat picker
        // needs a character's full persisted feat list, not just feats
        // added in the current session. Fighter's fixed loadout seeds
        // three feats at composition time, so a freshly-seeded, never-
        // mutated character is proof the load path surfaces persisted
        // feats verbatim, not merely session-appended ones.
        assert_eq!(
            loaded.selected_feats,
            vec![
                "feat:power_attack".to_owned(),
                "feat:dodge".to_owned(),
                "feat:weapon_focus".to_owned(),
            ]
        );
        // Backlog item 9a: this Fighter fixture has no spells at all --
        // proves the empty case round-trips honestly (not fabricated
        // entries). See `load_saved_character_surfaces_persisted_spells_selected`
        // below for the non-empty Wizard case.
        assert!(loaded.spells_selected.is_empty());

        // append_to_character: real corpus-validated batch append.
        let append_result = adapter
            .append_to_character(
                &root,
                &[ItemToAppendDto {
                    item_id: "Dagger (Base)".to_owned(),
                    active_state: crate::character_hub::ActiveStateDto::EquippedActive,
                }],
                "2026-07-21T01:00:00Z",
            )
            .expect("append_to_character should not error");
        assert!(append_result.success, "appending a real item should succeed: {:?}", append_result.error);

        // recompute: real read-and-recompute, no mutation.
        let recomputed = adapter.recompute(&root, character_id);
        assert!(recomputed.success, "recompute should succeed: {:?}", recomputed.error);

        // save_character: real re-save via the revision-conflict-checked path.
        let reloaded_after_append =
            SavedCharacterStore::load(&root).expect("reload after append should succeed");
        let saved = adapter
            .save_character(&root, &reloaded_after_append.revision_id, "2026-07-21T02:00:00Z")
            .expect("save_character should not error");
        assert!(saved.success, "save_character should succeed: {:?}", saved.error);

        let conflict = adapter
            .save_character(&root, "stale-revision-not-on-disk", "2026-07-21T03:00:00Z")
            .expect("save_character should not error even on a conflict rejection");
        assert!(!conflict.success);
        assert_eq!(conflict.error.as_deref(), Some("revision_conflict"));

        std::fs::remove_dir_all(&characters_root).ok();
    }
}
