# Rules engine

> Scope: The headless PF1 rules-computation spine — from chosen character input through the deterministic chassis engine to the boundary contract the GUI consumes.
> Last verified: 2026-08-21 against tranche/11 (SD-31 wave 26 — formula interpreter wired to its
> first production consumers). **Path correction 2026-08-22** (SD-32 closure epilogue): all 12
> src/rules_core/pilot_compute.rs cites updated to `src/rules_core/pilot_compute/mod.rs` — the
> module (still the deterministic chassis engine's own entry point) became a directory of 10 files
> during SD-31 (`bonus_stack_reader.rs`, `formula_interpreter.rs`, and others alongside `mod.rs`);
> no other content in this doc re-verified.
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

This document orients a contributor entering `src/rules_core/` cold. It describes the compute spine
end-to-end, the fail-honest convention every engine in this tree follows, and a catalog of the
per-domain engines with their entry points. It does not restate per-line rules content — read the
cited modules for that.

## The compute spine, end to end

The engine has five layers. Data flows strictly downward; nothing later in the list mutates or
re-derives what an earlier layer already produced.

### 1. `src/rules_core/character_input.rs` — chosen picks only

`crate::rules_core::character_input::CharacterInput` is the wire shape for what a player chose:
race, class levels, ability scores, selected feats, skill-rank allocations, equipment selections,
spell selections, and free-form selected choices (feat/trait/domain-style picks keyed by
`choice_set_id`/`selection_id`). The module's own header comment states the boundary precisely: it
"deliberately does not compute derived values, evaluate effects, or interpret formulas."
`ChosenCharacterState` (the payload of `CharacterInput.chosen`) is a pure data record — no method on
it computes anything.

The module also owns a plain-text fixture grammar and its loader, `load_character_input_fixture`,
which parses `key=value` lines (`race_id=`, `class_level=`, `ability=`, `feat=`, `skill=`,
`equipment=`, `choice=`, `spell=`, `provenance=`) into a `CharacterInput`. Every malformed or
missing-required-field line produces a `CharacterInputDiagnostic` with `claim_blocking: true` (see
`diagnostic()` at the bottom of the file); the loader either returns a fully valid `CharacterInput`
with zero diagnostics, or `None` with the diagnostics that explain why. This is the first appearance
of the fail-honest pattern described below — it starts at the input boundary, not just inside the
compute engine.

`EquipmentSelection.active_state: ActiveState` (`EquippedActive` / `Absent` / `SelectedInactive`) is
the field every downstream equipment-aware computation filters on; a selection that is merely
recorded but not active contributes nothing (see `src/rules_core/contract.rs`'s `to_pilot_receipt`, which filters
on exactly this field before computing equipment effects).

### 2. `src/rules_core/composed_input.rs` — bridging chosen input with corpus content

`compose(character_input: CharacterInput, corpus: SourcePackageContent) -> ComposedInputLoadResult`
in `src/rules_core/composed_input.rs` is the consumer-side bridge between what the player chose
(`CharacterInput`) and what the loaded PCGen corpus actually contains (`SourcePackageContent`,
defined in `src/rules_core/source_content.rs`). It performs exactly three checks, no rule evaluation:

1. `character_input.source_package_id` must equal `corpus.package_id` — a mismatch is an `Error`-severity `ComposedInputDiagnostic` and `compose` returns `composed: None`.
2. An empty corpus is not fatal — it produces a `Warning`-severity `EmptyCorpus` diagnostic, but `composed` is still populated (the deterministic-pilot path can still evaluate with seeded defaults).
3. Otherwise composition succeeds with zero diagnostics.

The module's doc comment explains why this lives in `rules_core` rather than `pcgen_import`: it is a
"transport-and-shape carrier" — no parsing, no include resolution, no IR conversion. Those live
upstream in `pcgen_import` (`include_resolver`, `ir_converter`, `lst_parser::*`), which
`src/rules_core/composed_input.rs` calls only through its convenience loader `load_composed_core_rulebook` (PCC
entry file → resolved include graph → per-kind LST parse → owned parser containers) and
`project_corpus_from_owned` (owned containers → a borrowed `SourcePackageContent`). The borrow
discipline is deliberate: the corpus's records borrow from the owned parser containers, so the
caller must keep `owned_inputs` alive as long as it uses the corpus — the module comment states this
was chosen over `unsafe` lifetime transmutes to keep the borrow auditable.

`ComposedCharacterInput<'a>` (the `Some` case of `composed`) is the actual input the rest of the
spine consumes: it owns the `CharacterInput` and holds the borrowed `SourcePackageContent`.

### 3. `src/rules_core/pilot_compute/mod.rs` — the deterministic chassis engine

This is the core engine, and at roughly 17,800 lines it is by a wide margin the largest file in
`rules_core`. Its structure is not a grab-bag: it is one long orchestrator function,
`compute_pilot_base_chassis`, that calls a fixed, ordered sequence of per-domain helper functions,
each of which appends to two shared accumulators (`explanations: Vec<ComputationExplanation>`,
`diagnostics: Vec<ComputationDiagnostic>`) rather than returning independent state. Reading the file
productively means reading `compute_pilot_base_chassis`'s body first (it is the table of contents)
and then jumping to the specific helper you need — not reading top to bottom.

**Entry points** (the only two `pub fn`s the module exports beyond its types):

- `compute_pilot_base_chassis(input: &CharacterInput) -> PilotBaseChassisComputation` — the core computation. Produces ability modifiers, base attack bonus, base saves, the deterministic combat baseline (melee attack bonus / armor class for a fixed Longsword + Chain Shirt + Dodge posture), total saves, selected skill modifiers, and a long tail of per-race and per-class explanation/diagnostic records, all accumulated into one `PilotBaseChassisComputation`.
- `build_pilot_headless_receipt(input: &CharacterInput) -> PilotHeadlessReceipt` — a thin wrapper that runs `compute_pilot_base_chassis` and derives one `HeadlessReceiptStatus` (`Computed` or `Blocked`) from whether any diagnostic in the result is claim-blocking. This is the receipt shape `src/rules_core/pilot_view_model.rs` and `src/rules_core/pilot_failure.rs` consume (see below); it predates and is narrower than the SD-20 boundary contract's `PilotReceipt` (§5).

**Internal organization**, in the order `compute_pilot_base_chassis` calls them:

| Stage | Representative functions | What it grounds |
|---|---|---|
| Ability modifiers | `compute_ability_modifiers` | `floor(score/2) - 5` per ability |
| Class chassis dispatch | `compute_class_chassis`, `compute_fighter_chassis`, `compute_wizard_chassis`, `compute_multiclass_base_chassis`, `has_supported_class_chassis` | Base attack bonus + base saves for the classes/levels the dispatch table recognizes; unsupported input pushes `class_chassis.unsupported` |
| Combat baseline | `compute_combat_baseline`, `unmet_combat_posture_conditions` | Baseline melee attack bonus / armor class for the exact deterministic Longsword/Chain Shirt/Dodge/no-shield posture |
| Total saves | `compute_total_saves` | Base save + relevant ability modifier, gated on `has_supported_class_chassis` |
| Selected skill modifiers | `compute_selected_skill_modifiers`, `unmet_selected_skill_posture_conditions` | Climb/Intimidate/Swim only, gated on an exact rank-1 + Chain Shirt posture |
| Per-class feature/spell explainers | `explain_fighter_class_features`, `explain_hybrid_level1_chassis`, `explain_paladin_level1_chassis_and_spell_burden_separation`, `explain_ranger_level1_chassis_and_class_feature_separation`, `explain_barbarian_level1_chassis`, `explain_monk_level1_chassis`, `explain_rogue_level1_chassis`, `explain_sorcerer_level1_spell_baseline`, `explain_wizard_level1_prepared_spell_baseline`, `explain_cleric_level1_spell_baseline`, `explain_druid_level1_spell_baseline`, `explain_bard_level1_spell_baseline` | Per-class, per-level named feature and spell-baseline explanation/diagnostic records — one function family per one of the 11 core classes |
| Per-race seams | `explain_human_pilot_race_seam`, `explain_human_trait_bundle`, `explain_dwarf_race_seam`, `explain_elf_race_seam`, `explain_gnome_race_seam`, `explain_half_elf_race_seam`, `explain_half_orc_race_seam`, `explain_halfling_race_seam` | Per-race trait recognition and (where grounded) numeric contribution — one function family per one of the 7 core races (Human's is split across two functions) |
| Cross-cutting validation | `validate_fighter_feat_choice_legality` | Input-legality checks that produce diagnostics without computing a value |

Each per-class/per-race function follows the same internal shape: a `supported_<class>_level(input)
-> Option<u8>` gate function decides whether the input's class/level combination is inside the
function's proven range, and the `explain_*`/`compute_*` function either produces real explanation
records or pushes a named claim-blocking diagnostic and stops. This gate-then-explain pairing recurs
at every level band; the file's per-function doc comments record which named sub-features are
grounded versus still claim-blocked as of the current level ceiling for that class.

**Multiclass base-chassis dispatch (SD-24 Epic 5).** `compute_multiclass_base_chassis` fires whenever
`input.chosen.class_levels.len() >= 2`; `is_supported_multiclass_mix` gates it to combinations where
every class level is individually supported — today that means Fighter + Wizard only, at any split of
total level 1-10 (deterministically proven level-by-level, both solo-to-multiclass transition
directions, in `tests/sd24_multiclass_deterministic.rs`/`tests/sd24_multiclass_integration.rs`). Base
attack bonus and saves stack per PF1's canonical additive multiclass rule: each class's own
fractional BAB/save progression is summed *before* flooring once for the total, reading the
fractional classification from `class_tables.rs`'s own `good_saves_for(ClassId) -> Option<(bool, bool,
bool)>` (`multiclass_good_saves`) rather than a second, independently-maintained copy. `fighter_level_in_mix`/
`wizard_level_in_mix` (`pilot_compute.rs`) resolve each class's own sub-level from the mix so that
class's per-level named-feature/spell-baseline explainers (e.g. `explain_wizard_level1_prepared_spell_baseline`)
keep firing once a second class joins, instead of silently going quiet the moment the build stops
being single-class. This grounds the base-chassis/explanation layer only — it does not by itself get
a Fighter+Wizard multiclass build to `HeadlessReceiptStatus::Computed` end-to-end (spellbook and
other per-domain diagnostics can still block); see [status.md](./status.md) for the current
`Computed`-reachability ceiling.

**Core output types** (`PilotBaseChassisComputation`, `ComputationExplanation`, and `ComputationDiagnostic`
are defined near the top of the file; `HeadlessReceiptStatus` and `PilotHeadlessReceipt` are defined
later, immediately before the two entry-point functions — all five precede the roughly 13,000 lines of
per-class/per-race function logic that populate them):

- `PilotBaseChassisComputation` — the aggregate struct `compute_pilot_base_chassis` returns: `ability_modifiers`, `base_attack_bonus`, `base_saves`, `baseline_melee_attack_bonus`, `baseline_armor_class`, `total_saves`, `selected_skill_modifiers`, `explanations`, `diagnostics`.
- `ComputationExplanation { id, value, detail }` — one machine-checkable record per computed value.
- `ComputationDiagnostic { id, message, claim_blocking }` — see the fail-honest pattern below.
- `HeadlessReceiptStatus` (`Computed` | `Blocked`) and `PilotHeadlessReceipt { case_id, source_package_id, status, computation }` — the receipt `build_pilot_headless_receipt` returns.

The module's own header doc comment is explicit about what it is not: "not a full rules engine" — it
names the specific PF1 mechanics still out of scope (feat/item/condition-based save modifiers,
weapon damage, active Power Attack math, initiative, general skill modifiers beyond the three
selected skills, armor-check penalties beyond the deterministic posture, feat prerequisites, oracle
parity). Several of those gaps are exactly what the later per-domain engines in the
[per-domain engine catalog](#per-domain-engine-catalog) below exist to fill, without editing this file.

**Path note (SD31-E4-F1-005, since 2026-08-20):** `pilot_compute` is now a directory module
(`src/rules_core/pilot_compute/mod.rs` holds the ~17,800-line orchestrator this section describes;
`src/rules_core/pilot_compute/class_feature_grant_consumer.rs`, `class_slayer.rs`, and
`class_ultimate_combat.rs` are per-class submodules split out as a pure code-move — same behaviour,
same call sites via `mod.rs`'s own `use super::*` re-export). This document's `pilot_compute.rs`
references above predate that split and describe `mod.rs`'s content; not fully swept to the new path
throughout this file as of this note — treat `pilot_compute.rs` and `pilot_compute/mod.rs` as the
same file wherever this document names the former.

### 3a. `src/rules_core/pilot_compute/formula_interpreter.rs` and `domain_power.rs` — the formula
interpreter (SD-31 wave 25/25b, a real architecture change, not an extension of the pattern above)

**Every function cataloged in the table above is a hand-written, bespoke Rust closed-form
expression, independently derived and verified against the corpus per feature.** That was a pinned
rule (`SD-27 decisions.md §24.1`, "No formula interpreter") until `OPERATOR-RULINGS-2026-08-21.md`
§20 overturned it for this package (folded into `docs/release/SD-31-corpus-closure-grind/decisions.md`
as Decision 20): PCGen's own `BONUS:`/`DEFINE:` LST tokens already encode this arithmetic, and
hand-transcribing it into a bespoke Rust function per feature was the direct cause of this program's
per-unit throughput cost. Two new submodules, both under `src/rules_core/pilot_compute/`, exist as of
wave 25b:

- **`formula_reproduction_harness.rs`** — mechanically enumerates the existing hand-modelled
  functions from source (>=166, a re-derivable floor, not a hand-maintained list) and defines the
  `FormulaEvaluator` trait every interpreter implementation must satisfy. A small set of its
  enumerated cases (21 as of wave 25b) exercise a real evaluator against them for agreement.
- **`formula_interpreter.rs`** (`PcgenFormulaEvaluator`) — a real recursive-descent parser/evaluator
  for the arithmetic grammar PCGen's own `BONUS:`/`DEFINE:` formula segments carry (integer/ability-
  modifier variables, `+ - * /`, `floor`/`ceil`/`abs`/`min`/`max`/`if`/`classlevel`), semantics
  re-derived from the pinned oracle's REAL resolution chain (`BonusObj.java` → `FormulaFactory.java`
  → `JEPFormula.java` → `VariableProcessor.java` → `pcgen/util/PJEP.java extends org.nfunk.jep.JEP`,
  function library `plugin/jepcommands/*Command.java` — see the module's own doc for the full chain
  and wave 25b integration's correction of an earlier version that cited the wrong PCGen subsystem).
- **`domain_power.rs`** — a narrower, independent arithmetic evaluator applied specifically to
  Cleric/Inquisitor domain-power formulas, extending `ground_or_block_cleric_domain_power`/
  `ground_or_block_inquisitor_domain_power`'s prior Good+Healing allowlist to War and Strength (wave
  25 salvage, merged wave 25b).

**What has NOT changed:** the ruling's own condition. *"Every interpreted value must clear
`derived_evaluator_fixture_check` ... An interpreted value with no fixture is not done."* Neither
`formula_interpreter.rs` nor `formula_reproduction_harness.rs` has a production consumer wired as of
wave 25b — both are `pub` infrastructure with zero non-test callers, banking zero corpus units. The
hand-written pattern the table above describes remains the shipping mechanism for every unit
currently `done`; the interpreter is additive capability, not (yet) a replacement for what already
ships. `domain_power.rs` IS wired (Cleric/Inquisitor War/Strength), because its own fixture gate
(`mod fixture_check_tests`, in-module, corpus-byte-transcription-checked) satisfies the ruling's
condition directly.

**Known, disclosed gaps as of wave 25b** (see `OPEN-ISSUES.md` rows 354-357 and the wave 25b receipt
for the full account): `classlevel("X")` does not verify its class-name argument against a bound
class context (silently wrong, not merely incomplete, for a genuinely cross-class formula — a
confirmed real corpus shape, `bestiary_3`'s `classlevel ("Magical Beast")/2-1`); comparisons do not
yet produce a reusable numeric value outside `if()`'s own condition slot, so boolean-to-int coercion
(`"1+(KineticistLVL>=15)"`) and `&&` (Sorcerer bloodline gates) both refuse rather than evaluate; the
`BONUS:<TAG>|<target>|` envelope, PRE-token gating, and `PREVARGTEQ`-embedded repeated-conditional
clauses are a different PCGen subsystem (`BonusObj`/`MultiTagBonusObj`) entirely out of scope.

**Also new in `pilot_compute/mod.rs` as of wave 25b**: a flat-override `race_trait` compute seam
(`explain_rougarou_flat_override_race_trait`, `explain_gillman_flat_override_race_trait`,
`explain_vanara_flat_override_race_trait`) — the first `race_trait` movement in six waves, grounding
a race's flat Speed/Vision/Natural-Weapon override (and, for Gillman/Vanara, the alternate-trait
`PREFACT`-gated replacement of that override) without a new subsystem, following the same
gate-then-explain shape the per-race seam table above already uses.

### 3b. Wave 26 — the interpreter gets its first production consumers, plus grammar widening

Wave 25b built `formula_interpreter.rs` and proved it against 22 hand-modelled functions but wired
zero consumers. Wave 26 plugged it in:

- **`class_feature_grant_consumer.rs`** now resolves a `class_feature` corpus record's `DESC:` `%N`
  placeholder through the interpreter (`resolve_pcgen_var_chain`, a fixed-point pass over the
  record's own same-record `BONUS:VAR` chain, seeded with the character's real class level) where
  the static, book-agnostic `class_feature_descriptions.rs` catalog (desktop app) has no character
  context to resolve it — that catalog was also fixed this wave to refuse serving a description whose
  `%N` it cannot fill, rather than silently dropping the number (a real, disclosed ~2,389-description
  reduction in raw served count, trading a subtly-wrong sentence for none). 12 class_feature records
  fixture-verified this wave; 1 (`core_rulebook:class_feature:rogue_trapfinding`) newly crosses the
  `derived`+`fixture-verified` → `done` bar. **Reachability caveat** (`OPEN-ISSUES.md` row 366): the
  new consumer row this module emits for Trapfinding is itself suppressed by the pre-existing
  `already_computed_slugs` guard (a hand-modelled `class_chassis.rogue.trapfinding` already occupies
  that slug) — the banked unit's `done` status rests on the `derived` wiring class's own bar
  (evaluator-fixture correctness), not on this new row reaching a live sheet. The value shown to the
  player was, and remains, correct via the pre-existing hand-modelled path.
- **`domain_power.rs`** (its own, separate, `i32`-typed evaluator — NOT `formula_interpreter`'s `f64`
  one; unification is an open question, `OPEN-ISSUES.md` row 368) widened Cleric's own domain
  dispatch to read the shared `DOMAIN_POWER_CATALOG` generically (previously Good/Healing-only even
  though War/Strength already existed and were already served to Inquisitor), and added two new
  catalog entries: Destruction ~ Destructive Smite (`max(DomainDestructionLVL/2,1)`) and Glory ~ Touch
  of Glory (bare `DomainGloryLVL`, no `max()` wrap). 0 board units bank from this — confirmed this
  cycle and pre-existing per `OPEN-ISSUES.md` row 360 — because `v06_work_inventory.rs`'s
  `class_feature_owner` cannot attribute ANY `Domain Power ~ X` corpus row to Cleric or Inquisitor at
  all, not even the already-shipped, already-computed Good/Healing powers.
- **`formula_interpreter.rs` grammar widened**: bare/parenthesised comparisons as first-class
  boolean-as-numeric values (`Expr::Cmp`), `&&`-chains of comparisons (`Expr::And`), and
  `skillinfo("TOTALRANK", ...)` (`Expr::SkillInfoTotalRank`) — all three derived from decompiling the
  pinned `org.scijava:jep:2.4.2` dependency jar's bytecode (`Comparative`/`Logical`/
  `SkillInfoCommand`), not guessed. `corpus_shape_coverage`'s refusal count fell from 431/2,671
  (16.1%) to 118/2,671 (4.4%).
- **`bonus_stack_reader.rs`** (new module) reads the real multi-token `PREVARGTEQ`-gated additive-
  stack shape (`witch_ward_bonus` and ~210–222 similar records/target-variable groups) — several
  `BONUS:VAR` tokens sharing one target, each individually gated, summed only over currently-
  qualifying addends per `PlayerCharacter.getTotalBonusTo`/`BonusManager.sumActiveBonusMap`. Zero
  consumers wired yet (out of this wave's scope). The wave-25-dispatch-named
  "`PREVARGTEQ`-embedded-inside-raw-formula-text" shape does not exist in the real corpus — a
  dispatch-premise correction (`OPEN-ISSUES.md` row 364); this module reads the shape that IS real.
- **`race_trait`**: a formula-shaped seam for Undine's 3 alternate racial traits was built this wave
  and its arithmetic/fixtures were independently verified sound by two separate reviews, but the
  accompanying board-credit change (adding `"undine"` to a coarse race-level allowlist) was found, on
  mutation, to award `done` credit to 11 sibling records with no consumer of any kind — marked GAMED
  and **NOT merged** to `tranche/11`. See `OPEN-ISSUES.md` row 365 for the full finding and two
  remediation paths a future wave can take to land the sound parts (the seam and its fixtures)
  without the gaming vector.

**Known interpreter gaps as of wave 26** (supersedes the wave-25b list above where noted):
`classlevel("X")` still does not verify its class-name argument (unchanged — still a hard
precondition on banking anything through it); `classlevel("X","APPLIEDAS=NONEPIC")`'s 2-arg form is a
real corpus shape, unverified, refuses cleanly; `var`/`count`/`mastervar`/`charbonusto`/`cl` (57
refusals) remain unimplemented; comparisons/`&&`/`skillinfo(TOTALRANK)` are no longer gaps (closed
this wave).

### 3c. Wave 27 — the interpreter's second consumer (ability modifiers), and the class-chassis census

Wave 27's dispatch reframed the program's remaining wall as "features for characters that cannot
exist" and asked how many of the 157 not-done `class` units are Monk-shaped — a chassis table
present, only the `table_class_id` dispatch mapping missing. **The census answer is zero**: every
class with a real chassis table anywhere in the codebase (34 total, across CRB/APG/ACG/Pathfinder
Unchained/Ultimate Combat) is already dispatched. See [status.md](./status.md)'s wave 27 section for
the full breakdown of where the remaining 157 classes actually sit (prestige entry-requirement gap,
net-new base-class tables, structurally-non-PC-class records, unstarted books).

- **`class_feature_grant_consumer.rs`'s `resolve_pcgen_var_chain` now seeds the six ability-modifier
  abbreviations** (STR/DEX/CON/INT/WIS/CHA) before its fixed-point `BONUS:VAR` pass, so a
  `class_feature` `DESC:` formula referencing a bare ability modifier (not just the character's class
  level) can resolve. Two units newly clear the `derived`+`fixture-verified` bar: Ranger ~ Master
  Hunter, Rogue ~ Master Strike — both riding on pre-existing CRB chassis dispatch, not new class
  support. **Reachability caveat, same shape as wave 26's row 366**: `already_computed_slugs`
  suppresses both new rows in production (a pre-existing hand-modelled `value:0` explanation already
  occupies each slug), so neither value newly reaches a live character sheet this wave — confirmed by
  driving `compute_pilot_base_chassis` across 165 synthetic characters and finding zero
  interpreter-resolved lines. `OPEN-ISSUES.md` row 375 names the concrete unblock
  (`pathfinder_unchained::rogue_features::master_strike_dc` already proves the pattern for the
  Unchained Rogue in the same file).
- **The flat-override `race_trait` seam grew to 5 races** (Rougarou, Gillman, Vanara, **Samsaran,
  Nagaji**), each requiring full per-record coverage of the race's reachable `computed` population
  before being added to `FLAT_OVERRIDE_RACE_TRAIT_RACES` — a direct, disclosed response to wave 26's
  Undine GAMED finding (row 365's partial-coverage shape). One real bug was caught and fixed during
  integration: Nagaji's Hypnotic Gaze is an alternate trait that replaces Serpent's Sense
  (`Nagaji_ReplaceSerpentsSense`, already registered in `race_resolver.rs`'s
  `ALTERNATE_TRAIT_REPLACE_FLAGS`), but the merged seam emitted both unconditionally — fixed by
  gating on `replaced_by_alternate_trait`, mirroring `explain_gillman_flat_override_race_trait`/
  `explain_vanara_flat_override_race_trait`. The credit mechanism itself (`is_seamed`, race-level not
  record-level) still banks all 10 Samsaran+Nagaji units regardless of this fix — see `OPEN-ISSUES.md`
  row 365/378/380 for why that coarseness was left as-is rather than patched piecemeal for one race.
- **No change to `formula_interpreter.rs`, `class_tables.rs`, or any `ClassId`-family enum this
  wave.** Both class-scoped lanes (a corpus-wide census, and a CRB-prestige-class architecture
  investigation) were comment-only diffs; zero classes were made buildable.

### 4. `src/rules_core/pilot_compute_corpus.rs` — the corpus-aware wrapping seam

`compute_pilot_with_corpus(input: &CharacterInput, corpus: &SourcePackageContent) ->
CorpusPilotReceipt` in `src/rules_core/pilot_compute_corpus.rs` wraps `compute_pilot_base_chassis`
with corpus-derived contributions **without modifying `src/rules_core/pilot_compute/mod.rs` itself** — the module doc
comment states this explicitly as a design constraint, so that every caller of the unwrapped chassis
function keeps working unchanged. `CorpusPilotReceipt { base: PilotBaseChassisComputation,
corpus_derived: CorpusDerivedSection }` is the result: `base` is the unchanged chassis computation,
`corpus_derived` adds two things the chassis alone cannot prove:

- `school_coverage: BTreeMap<Pf1SchoolId, SchoolCoverage>` — every entry in `input.chosen.spells_selected` that resolves against the corpus (via `spell_resolver::spell_id_resolve`) grouped by PF1 spell school, each carrying a `TableCellRef` when the corpus record can be pinned to a canonical Paizo table cell.
- `equipped_items: Vec<ResolvedEquipment>` — every equipment selection that resolves against the corpus (via `equipment_resolver::equipment_id_resolve`), carrying the resolved record's name/key and an (as of this module) `DerivedEquipmentStats::default()` placeholder — populating real per-item stats is explicitly out of scope here and is what `src/rules_core/equipment_effects.rs` (see the [per-domain engine catalog](#per-domain-engine-catalog) below) exists to do instead.

Resolution here is deliberately generic: it reads a resolved corpus record's own `school`/category
field rather than dispatching through per-school or per-category code, per the module's doc comment.
`TableCellRef { rule_set, table, row_key, column_key }` is the shared "this claim is anchored to a
specific Paizo source-book table cell, not just a corpus record's existence" proof type — it recurs
across `src/rules_core/pilot_compute_corpus.rs`, `src/rules_core/equipment_effects.rs`, and the support-state matrix's
grounding-reference pattern (see [support-state-matrix.md](./support-state-matrix.md)).

### 5. `src/rules_core/contract.rs` — the boundary contract and the only sanctioned exit surface

`src/rules_core/contract.rs` is the GUI-facing boundary: everything the desktop app is allowed to
render comes through this module's types, never by reaching into `src/rules_core/pilot_compute/mod.rs` or any
per-domain engine directly. Its own header doc comment names it the contract's "code-level home."

- `CharacterInputPermutation` (`BrandNew` | `MidBuild` | `Multiclass`) and `classify_character_input(input: &CharacterInput) -> CharacterInputPermutation` classify an input into one of three canonical shapes the contract documents: multiclass takes precedence over mid-build, mid-build over brand-new (see the function body for the exact precedence and thresholds).
- `PilotReceipt` is the full GUI-facing receipt. It does not duplicate `PilotBaseChassisComputation`/`CorpusPilotReceipt` — it composes with them: `chassis` is the unchanged chassis computation, `corpus_derived` is the unchanged corpus-derived section, and `diagnostics` hoists the chassis's diagnostics to the receipt's top level. On top of that it adds the real per-domain engine outputs: `skills: SkillTotals`, `spellbook: SpellbookCoverage`, `feats: Vec<ResolvedFeat>`, `equipment_effects: EquipmentEffects`, `weapon_damage: Vec<WeaponDamageBreakdown>`.
- `to_pilot_receipt(receipt: &CorpusPilotReceipt, input: &CharacterInput, corpus: &SourcePackageContent) -> PilotReceipt` is the function that actually builds a `PilotReceipt`: it resolves `input.chosen.selected_feats` against `rules_tables::crb::feats::feat_tables()` (an unmatched feat string is silently skipped, never fabricated into a category), filters `equipment_selections` to `ActiveState::EquippedActive` before computing equipment effects, and reuses that same filtered `equipped` slice and its `EquipmentEffects` result when calling `damage_total::resolve_weapon_damage_breakdown` rather than recomputing either.
- `compute_level_up_preview(character: &CharacterInput, from_level: u8, to_level: u8) -> LevelUpPlan` is a thin pass-through to `level_up::compute_level_up_grants`. It is deliberately **not** a `PilotReceipt` field — the doc comment explains that Level-Up models a level *transition* (needs two extra parameters no other `PilotReceipt` consumer has), not a current-state snapshot, so it stays a standalone function alongside `PilotReceipt` instead of contaminating it.
- `PrintedSheetCell { cell_id, source_field, value: PrintedSheetCellValue }` and `printed_sheet_cell_map(receipt: &PilotReceipt) -> Vec<PrintedSheetCell>` are the literal cells a printed PF1 character sheet renders. `PrintedSheetCellValue` is either `Number(i16)` or `Blocked` — never a third "unknown" state, and never a fabricated number standing in for a blocked one. Every cell's `source_field` names the exact `PilotReceipt` field path it renders, for auditability. Not every `PilotReceipt` field becomes a cell: `printed_sheet_cell_map`'s own doc comment records, field by field, why `spells_prepared`/`spells_known`/`school_specialization` and `EquipmentEffects.spell_failure_chance` stay reachable only via `receipt.*` directly rather than being flattened into cells that don't fit `Number(i16) | Blocked` cleanly; `PilotReceipt.weapon_damage`'s own field doc comment records the same reasoning for why the full `WeaponDamageBreakdown` structures are never flattened into cells either.

## The fail-honest pattern

This is the single most important convention for anyone touching the engine, and it holds at every
layer described above, not just inside `src/rules_core/pilot_compute/mod.rs`.

**The rule**: every computed value carries an explanation record proving how it was derived; every
diagnostic carries a `claim_blocking: bool`; a computation is blocked if and only if at least one
claim-blocking diagnostic exists in its diagnostic list. The engine never fabricates a value it
cannot prove — it either computes the value for real (with an explanation record) or it withholds it
and returns an explicit blocked posture, never a zero or a guess presented as data.

**Where to see the mechanism directly**:

- `PilotBaseChassisComputation`'s numeric fields (`base_attack_bonus`, `base_saves`, `total_saves`, `baseline_melee_attack_bonus`, `baseline_armor_class`, `selected_skill_modifiers`) are computed to their real value on the supported path, or explicitly zeroed while a claim-blocking diagnostic is pushed on the unsupported path. `compute_total_saves` (`src/rules_core/pilot_compute/mod.rs`) is a clean, short example: if `!has_supported_class_chassis(input)`, it pushes `defense.total_save.unsupported` with `claim_blocking: true` and returns `BaseSaves::default()` — it does not attempt a partial computation.
- `build_pilot_headless_receipt` derives `HeadlessReceiptStatus` purely from whether `computation.diagnostics.iter().any(|d| d.claim_blocking)` — this is the status-derivation logic in one line, and it is the canonical place to see "blocked iff any claim-blocking diagnostic exists" implemented.
- `src/rules_core/contract.rs`'s `printed_sheet_cell_map` reads the same pattern one layer up: it checks specific diagnostic ids (`CLASS_CHASSIS_UNSUPPORTED_DIAGNOSTIC_ID`, `TOTAL_SAVE_UNSUPPORTED_DIAGNOSTIC_ID`, `COMBAT_BASELINE_UNSUPPORTED_DIAGNOSTIC_ID`, via the local `diagnostic_blocking` helper) to decide, cell by cell, whether to render `PrintedSheetCellValue::Blocked` instead of the chassis's zeroed number. The module doc comment is explicit that these diagnostic ids are additive/OR'd per cell, not a single blanket check — a fully-supported chassis can still have its combat-baseline cells blocked by a more specific posture failure, and vice versa.
- Not every diagnostic is claim-blocking. `src/rules_core/skill_allocation.rs`'s cross-class max-rank-cap diagnostic (`CROSS_CLASS_MAX_RANK_EXCEEDED_ID`) is always `claim_blocking: false`, because the corresponding `SkillTotal.ranks` already carries the real, legal, capped number — the diagnostic is informational, not a block. This is the pattern's other half: a diagnostic without `claim_blocking: true` is a note, not a withheld claim.
- Absence is a third, distinct state from both "computed" and "blocked": `printed_sheet_cell_map`'s doc comment spells out that a `sheet.skill.*` cell renders `Blocked` when `input.chosen.skill_allocations` never named that skill at all — `allocate_skill_ranks` only produces an entry for skills the player actually submitted, so `Blocked` there means "no computed value exists," not "a diagnostic gated this."

**Primary-failure-owner classification** (`src/rules_core/pilot_failure.rs`):
`FailureClassifier::primary_owner(&self) -> PrimaryOwner` maps a `PilotHeadlessReceipt`'s `status`
onto one of five required owners (`ModelFlaw`, `ImporterFlaw`, `EngineFlaw`, `OracleGap`, `UiGap`).
The module's own doc comment is candid about the current classifier's narrowness: `Computed` maps to
`OracleGap` (real outputs exist but no old-vs-new comparison evidence has been claimed yet) and
`Blocked` maps to `EngineFlaw` (the engine failed to compute the required outputs). The three other
owners exist in the vocabulary but are not yet reachable from the current receipt surface — the doc
comment states future work can classify them "as the receipt surface grows to carry importer, model,
and UI signals."

**View-model projection** (`src/rules_core/pilot_view_model.rs`): `PilotViewModel::from_receipt(receipt:
&PilotHeadlessReceipt) -> Self` is the adapter that turns a `PilotHeadlessReceipt` into a
UI-consumer surface. It preserves the same discipline one layer further out: `snapshot:
Option<PilotSnapshot>` is `Some` only when `status == HeadlessReceiptStatus::Computed`; on
`Blocked`, `snapshot` is `None` and the caller has only `explanations`, `diagnostics`, and the
classified `primary_owner` to work with — there is no code path that produces a `PilotSnapshot` from
a blocked receipt.

## Per-domain engine catalog

Each of the following operates independently of `src/rules_core/pilot_compute/mod.rs`'s internals; they are called
either directly by `contract.rs::to_pilot_receipt` or by each other, and none of them edits
`src/rules_core/pilot_compute/mod.rs`.

**`src/rules_core/spellbook.rs`** (Epic 2) — `compute_spellbook_coverage(input, corpus) -> SpellbookCoverage`.
Fills a gap `src/rules_core/pilot_compute_corpus.rs`'s own doc comment names explicitly: the corpus-aware seam
proves spell *reachability* only (a spell resolves against the corpus), never slot math,
prepared/known posture, or spell save DCs. `src/rules_core/spellbook.rs` computes those, dispatching per PF1 spell
school through nine submodules under `src/rules_core/spellbook/` — `src/rules_core/spellbook/abjuration.rs`,
`src/rules_core/spellbook/conjuration.rs`, `src/rules_core/spellbook/divination.rs`, `src/rules_core/spellbook/enchantment.rs`, `src/rules_core/spellbook/evocation.rs`, `src/rules_core/spellbook/illusion.rs`, `src/rules_core/spellbook/necromancy.rs`,
`src/rules_core/spellbook/transmutation.rs`, `src/rules_core/spellbook/universal.rs` — one per PF1 strict spell school, all nine landed.
`SpellbookCoverage` carries `slots_total`/`slots_used` (`BTreeMap<u8, u8>` keyed by spell level)
and `spell_save_dc` (`BTreeMap<String, u8>` keyed by class id) among other fields;
`contract.rs::printed_sheet_cell_map` emits one dynamic sheet cell per present key in each of those
three maps rather than a fixed cell set, so a non-caster naturally produces zero spellbook cells.

**`src/rules_core/skill_allocation.rs`** (Epic 4) — `allocate_skill_ranks(input: &CharacterInput) -> SkillTotals`.
Computes, for each skill the player actually allocated ranks to, whether it is a class skill,
applies the PF1 cross-class half-cap (`ceil((character level + 1) / 2)`) when it is not, and records
untrained-use modifiers (`SkillTotals.untrained_use`) for allocated-but-zero-rank skills. It
deliberately does not enumerate the full skill universe — a skill with no allocation entry at all
has no entry in `totals`, matching the fail-honest "absence, not fabrication" discipline above.

**`src/rules_core/feat_prereqs.rs`** (Epic 3) — `evaluate_feat_prerequisites(feat: &FeatKey) ->
PrerequisiteEvaluation` and `compute_feat_effects(feat: &FeatKey) -> FeatEffects`, dispatching by
`FeatCategory` across four submodules under `src/rules_core/feat_prereqs/`: `src/rules_core/feat_prereqs/general.rs`,
`src/rules_core/feat_prereqs/combat.rs`, `src/rules_core/feat_prereqs/item_creation.rs`, `src/rules_core/feat_prereqs/metamagic.rs` — one per category in
`rules_tables::crb::feats::feat_tables()` (185 CRB feat records: 50 General, 110 Combat, 8
ItemCreation, 17 Metamagic), all four categories landed. `FeatCategory` also carries `Teamwork`
and `Panache`, which only APG/ACG records use; those two dispatch arms have no landed
evaluation path (every submodule above evaluates against the CRB catalog, which by construction
holds no record of either) and say so rather than reporting a real APG/ACG feat as unrecognized.
The book-spanning catalog the desktop Feat picker serves is
`rules_tables::feats_all::all_feat_tables()` (486 records across CRB/APG/ACG) — see
[rules-data-tables.md](./rules-data-tables.md). Ingesting those records does **not** ground their
mechanical effects: `src/rules_core/feat_effects.rs` still grounds computed effects for a small
subset of CRB feats only.

**`src/rules_core/equipment_effects.rs`** (Epic 5) + submodules + `src/rules_core/equipment_resolver.rs` + `src/rules_core/spell_resolver.rs` —
`compute_equipment_effects(equipped: &[EquipmentSelection], corpus) -> EquipmentEffects` dispatches
across category submodules under `src/rules_core/equipment_effects/`: `src/rules_core/equipment_effects/arms_armor.rs`
(AC/max-dex/spell-failure — the fields `EquipmentStatEffect` is shaped to carry),
`src/rules_core/equipment_effects/general.rs` (per-item skill-check circumstance bonuses, via
`ResolvedEquipmentEffect::skill_bonus`), `src/rules_core/equipment_effects/magic_items.rs` (per-item ability-score enhancement
bonuses, via `ResolvedEquipmentEffect::ability_bonus`), `src/rules_core/equipment_effects/equipmods.rs` (per-item weapon
to-hit/damage enhancement bonuses, via `ResolvedEquipmentEffect::weapon_enhancement_bonus`) — the original four Epic 5
categories — plus, added SD-31 wave 18 (operator ruling 2026-08-19, intelligent-item subsystem
in scope): `src/rules_core/equipment_effects/intelligent_item.rs` (a selection-scoped
Intelligence/Wisdom/Charisma/Ego/alignment contribution, via `ResolvedEquipmentEffect::intelligent_item`
— reads the CRB/Mythic Adventures `Intelligent Item ~ ...` `BONUS:VAR` chains directly; the Base
record's own `BaseCostTracker`-formula Ego contribution is honestly skipped, not fabricated; the
`Intelligent Item ~ Power/Purpose` families' own headline mechanics remain unresolved). A scoped
`WeaponEnhancementBonus::natural_attack_only` field (equipmods.rs) marks the Amulet of Mighty Fists
family's bonus as natural-attack-only; **both** live consumers — `damage_total::
resolve_weapon_enhancement_modifier` (the top-level-selection `weapon_enhancement_bonus` path) and
`equipment_effects::resolve_weapon_to_hit_bonus` (the `applied_modifiers`-attachment path the
shipped desktop app's `attach_equipment_modifier_at_root` actually uses) — check
`equipment_effects::is_natural_attack_weapon` on the specific weapon being resolved before applying
it (`OPEN-ISSUES.md` rows 309/318 — the first consumer was fixed in wave 17/18, the second leaked
until wave 18's own integration cycle closed it). Corpus identity resolution for both equipment and
spells is centralized, not duplicated per engine: `equipment_resolver::equipment_id_resolve` and
`spell_resolver::spell_id_resolve` are the sole lookup functions every corpus-aware engine
(`src/rules_core/pilot_compute_corpus.rs`, `src/rules_core/equipment_effects.rs`,
`src/rules_core/spellbook.rs`, `src/rules_core/damage_total.rs`) calls to turn a chosen
`item_id`/`spell_id` into a real corpus record plus an optional `TableCellRef`.

The intelligent-item resolver's own DTO shape has a real desktop-facing surface —
`apps/desktop/src-tauri/src/intelligent_item_catalog.rs` (SD-31 wave 18) serves 152 intelligent-item
component records (98 core_rulebook + 71 mythic_adventures, minus 17 `VISIBLE:NO` trigger rows)
grouped by family, each mechanic transcribed with a friendly label/formula/translated prerequisite —
see [desktop-app.md](./desktop-app.md). The two surfaces are independent: the catalog reads raw
corpus equipmods directly (a static component reference), not `compute_equipment_effects`'s
resolved output (a specific character's equipped item); reconciling them into one
per-character "equipped intelligent item" view is a named, un-staffed follow-on
(`OPEN-ISSUES.md` row 318's `next_lever`, `progress.md` `SD31-W18-INTEGRATE-001`).

**`src/rules_core/damage_total.rs`** (Epic 6) — a single flat file (no per-category subdirectory), because per its
own doc comment the full damage-modifier picture is one sequential computation rather than a
per-category dispatch: `resolve_base_damage_dice`, `resolve_str_damage_modifier` (full STR mod
one-handed, 1.5x two-handed, 0.5x off-hand, read from the corpus's real `WIELD:` token),
`resolve_weapon_enhancement_modifier`, `resolve_feat_damage_effect` (bounded to feats whose `BONUS:`
token is a directly-usable constant, e.g. Weapon Specialization — explicitly excludes
PCGen-formula-over-BAB feats like Power Attack), `resolve_critical_threat_range`,
`resolve_critical_multiplier`, composed by the entry point `resolve_weapon_damage_breakdown(input,
corpus, equipment_effects, str_modifier) -> Vec<WeaponDamageBreakdown>`. One `WeaponDamageBreakdown`
per equipped item identified as a weapon; a non-weapon equipped item is silently absent from the
`Vec`, never represented with `None` fields.

**`src/rules_core/level_up.rs`** (Epic 7) + 11 per-class submodules — `compute_level_up_grants(character:
&CharacterInput, from_level: u8, to_level: u8) -> LevelUpPlan`, with one submodule per core class
under `src/rules_core/level_up/`: `src/rules_core/level_up/barbarian.rs`, `src/rules_core/level_up/bard.rs`, `src/rules_core/level_up/cleric.rs`, `src/rules_core/level_up/druid.rs`, `src/rules_core/level_up/fighter.rs`,
`src/rules_core/level_up/monk.rs`, `src/rules_core/level_up/paladin.rs`, `src/rules_core/level_up/ranger.rs`, `src/rules_core/level_up/rogue.rs`, `src/rules_core/level_up/sorcerer.rs`, `src/rules_core/level_up/wizard.rs` — all 11 landed,
closing Epic 7. `LevelUpPlan`'s `automatic_features` field composes read-only with two
already-grounded sources rather than re-deriving them:
`rules_tables::crb::class_tables::class_tables()` for class-generic BAB/save progression, and
`pilot_compute::compute_pilot_base_chassis`'s own per-class `explanations` for class-specific
pillars (e.g. Barbarian Rage, Uncanny Dodge). This is a read-only composition, not a second copy of
chassis logic. Note a live dispatch limitation surfaced by SD-25's adapter work: `compute_level_up_grants` reads a single implied class off `character.chosen.class_levels` and returns an honestly-empty `LevelUpPlan::default()` for any multiclass mix (it has no per-class-delta parameter). The desktop hub-of-hubs `RuleSystemAdapter::level_up` (see [desktop-app.md](./desktop-app.md) §"Rule-system adapter seam") deliberately takes an explicit `&[ClassLevelDelta]` slice so a multiclass level-up is *expressible* at that seam; widening the free function itself to honor that shape is still open.

**`src/rules_core/encounters.rs`** (SD-22 DM-toolkit) — `Encounter::new(party: &[CharacterSnapshot], monsters:
&[MonsterRef]) -> EncounterResult`, an associated function on the unit struct `Encounter` (named
`new` to match the criterion's literal signature, not because it constructs `Self`). Computes PF1
encounter difficulty (`Difficulty::Easy/Medium/Hard/Deadly`) from the Core Rulebook's Encounter
Design / CR Equivalencies / Experience Point Awards tables, grounded against the public PRD mirror
per the module's own doc comment; an empty `monsters` slice is a direct-rule `Easy`, not derived
from an undefined EL-vs-APL comparison.

**`src/rules_core/party_cr.rs`** (SD-22 DM-toolkit) — `party_challenge_rating(party: &[CharacterSnapshot]) -> f32`
computes Average Party Level per the Core Rulebook's "Step 1 — Determine APL" rule (sum levels /
party size, rounded, then ±1 for parties of 6+ or 3-or-fewer). Returns `f32` to match the
criterion's literal signature even though the rule always yields a whole number.

## Relationship to the data layer and tests

Most engines above read static rule tables rather than embedding rule data inline (`skill_allocation.rs`
is a documented exception — its own module doc comment explains that the table store carries no
class-skill-list table yet, so its bounded Fighter class-skill set is a cited inline constant instead);
the tables live under `src/rules_core/rules_tables/` (`crb/`, `apg/`, `acg/`, `beastiary1/` — one
directory per sourcebook), each exposing a table accessor engines reference directly — typically a
`pub fn <name>_tables() -> &'static [...]` (`rules_tables::crb::feats::feat_tables()`), but not
uniformly: `rules_tables::crb::class_tables::class_tables()` returns an owned `Vec`, and
`rules_tables::crb::spell_list::SPELL_LIST` is a plain `pub const` slice, not a function.
See [rules-data-tables.md](./rules-data-tables.md) for how those tables are structured and
sourced. The convention every per-domain engine's doc comment converges on independently — noted
explicitly in `src/rules_core/feat_prereqs.rs`, `src/rules_core/spellbook.rs`, and `src/rules_core/level_up.rs` — is a direct, fully-qualified
`use` of the specific table item, never a generic `RulesTables` indirection type (which does not
exist in this repo; several module doc comments note that the doctrine docs' illustrative signatures
assumed one and that this repo's real modules deliberately dropped it).

Tests for this spine live in the repo-root `tests/` directory (integration tests against the public
API, not `#[cfg(test)]` unit tests inside `rules_core`). Representative files actually opened for
this document:

- `tests/ge06_pilot_base_computation.rs` — proves `compute_pilot_base_chassis` against the deterministic GE-06 Human Fighter level-1 fixture (`tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`), asserting ability modifiers and base chassis values only.
- `tests/sd20_tabletop_readiness_integration.rs` — the Epic 8 integration-closure test: runs the full boundary-contract pipeline (`classify_character_input` → `compute_pilot_with_corpus` → `to_pilot_receipt` → `printed_sheet_cell_map`) against a fixture and asserts every defined sheet cell is a real, non-`Blocked` number matching a golden `expected_output`.
- `tests/sd13_barbarian_level6_progression.rs` (representative of ~400 per-class/per-level widening tests) — imports `support_state_matrix::seeded_current_truth` alongside chassis assertions, so a class/level widening and its matrix-row transition are proven together, not separately.

See [testing.md](./testing.md) for the full test-organization convention.

## Where to start if you're changing X

| If you're changing... | Start here |
|---|---|
| A new class's level-1 chassis or an existing class's level ceiling | `src/rules_core/pilot_compute/mod.rs`: find the class's `supported_<class>_level` and `explain_<class>_level1_chassis` (or equivalent) functions; add the gate condition and explanation records following the existing pattern |
| A new race's trait recognition | `src/rules_core/pilot_compute/mod.rs`: the `explain_<race>_race_seam` function family |
| Spell slot/prepared/known math for a school | `src/rules_core/spellbook.rs` + the specific `spellbook/<school>.rs` submodule |
| Skill rank totals, cross-class penalties, untrained use | `src/rules_core/skill_allocation.rs` |
| Feat prerequisites or feat-granted effects | `src/rules_core/feat_prereqs.rs` + the category submodule under `feat_prereqs/` matching the feat's `FeatCategory` |
| Equipment-derived AC/skill/ability/weapon bonuses | `src/rules_core/equipment_effects.rs` + the category submodule under `equipment_effects/` |
| Weapon damage rolls (base dice, STR, enhancement, feats, crits) | `src/rules_core/damage_total.rs` |
| What grants happen on level-up for a class | `src/rules_core/level_up.rs` + the per-class submodule under `level_up/` |
| Encounter difficulty or party CR | `src/rules_core/encounters.rs` / `src/rules_core/party_cr.rs` |
| What the GUI is allowed to render, or a new sheet cell | `src/rules_core/contract.rs`: `PilotReceipt`, `to_pilot_receipt`, `printed_sheet_cell_map` |
| Corpus resolution for a chosen item/spell id | `src/rules_core/equipment_resolver.rs` / `src/rules_core/spell_resolver.rs` |
| Whether something should count as claim-blocked | Re-read "The fail-honest pattern" above before writing a diagnostic |
| Whether a capability is officially supported yet | [support-state-matrix.md](./support-state-matrix.md) |

