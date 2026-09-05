# Cycle — SD-34 wave 42 — Paladin's Detect Evil and Cleric's Aura: 2 of 2 closed (small, precedented new compute)

- **Commit SHA:** `af674409f5` (`af674409f5dc2e1d901063bfe1af783e4961fef2`)
- **Files touched:** `src/rules_core/pilot_compute/mod.rs` (2 new pure functions —
  `paladin_detect_evil_caster_level`, `cleric_aura_strength_level` — 1 new unconditional-on-race
  grounding function, `ground_paladin_detect_evil`, called from the top-level dispatch; 1 new push
  block inside the existing `explain_cleric_level1_spell_baseline`; 1 new test module, 7 tests),
  `docs/work-inventory.json` (regenerated — real corpus-wide regen, not hand-edited),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (a `--check`
  re-run's own artifact), this receipt, `progress.md`, `kanban.md`. **No `data/corpus/**` file
  touched. No `src/bin/v06_work_inventory.rs` change** (see "Classifier wiring" below — neither
  unit needed one).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- src/rules_core/
  pilot_compute/mod.rs`, no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}` hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (0 hits for `placeholder`/`STUB`/`MOCK`/
  `not yet implemented`/`fixme`/`hack` in this cycle's own diff).
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** write real compute for
  Paladin's Detect Evil and Cleric's Aura, following the antipaladin's own precedent functions
  (`aura_of_evil_strength_level`/`detect_good_caster_level`,
  `rules_tables::apg::antipaladin_features`) as literal templates; wire `ComputationExplanation`
  pushes; confirm classifier reachability (a `CLASS_FEATURE_ID_KNOWN_SYNONYMS` entry, a
  `canonical_seeds_for()` arm, or neither); add unit tests; run the guarded regen; re-derive
  before/after bucket counts naming exactly which of the 2 units moved.

## Pre-state (re-derived fresh, not trusted from the dispatch brief)

`python3 scripts/completion_atlas.py --check` at this cycle's own pre-edit HEAD (`d570340aae`,
wave 41's own wave-end gate commit): `population=49438 unclassified=0 overlap=0
citation_failures=0`, `DONE: 25360`, `D: 2520`, `V: 322` (every other bucket unchanged by this
cycle). Both target units confirmed, by direct `python3` filter over `docs/work-inventory.json`'s
`units`, to be `engine-does-not-hold` with the identical evidence string
`class_feature_no_dedicated_magnitude_id_matched_the_record_slug`:

| Unit id | corpus_key | status (pre-cycle) |
|---|---|---|
| `core_rulebook:class_feature:paladin_detect_evil` | Paladin ~ Detect Evil | engine-does-not-hold |
| `core_rulebook:class_feature:cleric_aura` | Cleric ~ Aura | engine-does-not-hold |

## Corpus re-verification (read directly, not trusted from `decisions.md §22`)

- **Paladin ~ Detect Evil** (`cr_abilities_class.lst:1356`): `DEFINE:DetectEvilLVL|0` /
  `SPELLS:Class|TIMES=ATWILL|CASTERLEVEL=DetectEvilLVL|Detect Evil,11+WIS` /
  `BONUS:VAR|DetectEvilLVL|PaladinLVL` — `DetectEvilLVL` is a pure pass-through of the paladin's
  own class level, used as the at-will spell-like ability's caster level. Grant gate confirmed at
  `cr_abilities_class.lst:272`: `PREVARGTEQ:Paladin_CFP_Level,1` (from class level 1, matching the
  Antipaladin's own `detect_good_caster_level` level-1 gate exactly).
- **Cleric ~ Aura** (`cr_abilities_class.lst:563`): `BONUS:VAR|AlignmentAuraLVL|ClericLVL` — a
  pure class-level pass-through selecting which of four `PREDEITYALIGN`-gated virtual sub-
  abilities (Aura of Chaos/Evil/Good/Law, `cr_abilities_class.lst:2874-2877`, each its own
  `DEFINE:AuraXLVL|0` / `BONUS:VAR|AuraXLVL|AlignmentAuraLVL`) displays which tiered DESC prose
  (faint at 1, moderate at 2-4, strong at 5-10, overwhelming at 11+ — identical breakpoints to the
  Antipaladin's own `aura_of_evil_strength_level`). Grant gate confirmed at
  `cr_abilities_class.lst:217`: `PREVARGTEQ:Cleric_CFP_Level,1`. The alignment-gating (which of the
  four flavors a given cleric's deity selects) is a DIFFERENT, deity-selection-dependent burden
  this engine does not model at all; the magnitude this cycle grounds (`AlignmentAuraLVL`) is
  identical regardless of which of the four is chosen, so it does not need that burden solved
  first.
- **Antipaladin precedent, re-confirmed directly** (`rules_tables/apg/antipaladin_features.rs:159-
  175`): `aura_of_evil_strength_level`/`detect_good_caster_level` are both `if level < 1 { None }
  else { Some(i16::from(level)) }` — exactly the shape both new Paladin/Cleric functions use.

## Why NOT the antipaladin's own dispatch mechanism, and NOT the narrow SD13-E5 Paladin function

Two candidate homes were checked and rejected before writing new code:

1. **`ground_antipaladin_class_features`'s own dispatch site** (`compute_class_chassis`'s
   `untabled_base_class_chassis::resolve` branch): this branch is reachable ONLY for the 20
   classes registered in that untabled roster (Aegis, Antipaladin, Cryptic, ... Wilder) — Paladin
   and Cleric are CRB-TABLED classes, resolved earlier via `table_class_id`/
   `compute_generic_table_chassis` instead, and never reach this branch at all. Confirmed by
   direct read of `compute_class_chassis`'s own if/else chain, not assumed.
2. **`explain_paladin_level1_chassis_and_spell_burden_separation`** (the existing Paladin
   decomposition housing Smite Evil/Lay on Hands/Divine Grace/Mercy): gated on
   `input.chosen.race_id == HUMAN_RACE_ID` (via `supported_paladin_level`) AND single-class-only —
   a deliberately narrow SD13-E5 fixture scope (`MAX_SUPPORTED_PALADIN_LEVEL` is actually 20, not
   the level cap that matters here; the Human/single-class restriction is the real one). Detect
   Evil has no such restriction in the corpus (any race, any level from 1, any multiclass mix), so
   folding it into that function would silently under-ground it for every non-Human or
   multiclassed Paladin the corpus-wide sweep and real players alike can produce.

**What was built instead:** a brand-new, unconditional-on-race `ground_paladin_detect_evil`
function, called from the SAME top-level dispatch site as the other per-class `explain_*` calls
(never gated to a narrower fixture than the corpus record itself requires) — this is exactly the
"Paladin currently has no `ground_paladin_class_features`-style push at all" gap `decisions.md
§22` named. Cleric's Aura, by contrast, DOES fit its existing home cleanly:
`explain_cleric_level1_spell_baseline` is ALREADY unconditional on race and single-class status
(its own doc comment: "both Cleric burdens are validated... regardless of race" — the exact
Ranger/Paladin/Sorcerer-mirroring fix pattern), so the Aura push was added directly inside it,
right after `cleric_level` resolves.

## Classifier reachability — checked directly, not assumed

`class_feature_exact_suffix_grounded`'s own 3-segment `<owner>.<feature_slug>.<descriptor>` shape
(already used, without any synonym-table entry, by Paladin's own pre-existing `Divine Grace`
closure via `class_chassis.paladin.level_gate.divine_grace`/`divine_grace_save_bonus`) already
recognizes an id whose SECOND-TO-LAST dot segment equals `feature_slug` exactly, gated only on
`group.eq_ignore_ascii_case(class_name_as_group_text(owner))` (a check on the corpus record's own
`group` text, e.g. `"Paladin"`/`"Cleric"`, never on the id string's literal content beyond that
2-segment position). Traced `feature_slug` derivation directly:
`unit.key.split(" ~ ").nth(1)` → `"Detect Evil"`/`"Aura"` → `class_feature_engine_join_slug` →
`"detect_evil"`/`"aura"`. Both new explanation ids —
`class_feature.paladin.detect_evil.caster_level` (4 segments: `class_feature`/`paladin`/
`detect_evil`/`caster_level`) and `class_feature.cleric.aura.strength_level` (4 segments:
`class_feature`/`cleric`/`aura`/`strength_level`) — satisfy this shape directly: needle `.paladin.`
/`.cleric.` present, trailing segment (`caster_level`/`strength_level`) is not a
`CLASS_FEATURE_ID_NON_MAGNITUDE_TRAILING_MARKERS` word, second-to-last segment equals
`feature_slug` exactly, and `feature_slug != owner` in both cases. **No `CLASS_FEATURE_ID_KNOWN_
SYNONYMS` table entry and no `canonical_seeds_for()` match arm were needed for either unit** —
`owner` resolution for both `"paladin"` and `"cleric"` was already proven live by Paladin's own
`Divine Grace`/`Mercy` and Cleric's own `Channel Energy`/`Orisons` closures (all pre-existing,
unaffected by this cycle). `src/bin/v06_work_inventory.rs` carries zero diff this cycle.

Confirmed the sweep actually reaches level 1 for both classes: `SWEEP_LEVELS = [1, 5, 10, 15, 20]`
(`v06_work_inventory.rs`) includes level 1, and `class_sweep_input`/`posture_input` build a
single-class character of the swept class at that level, keeping the base fixture's race — both
new functions are race-independent by construction, so the fixture's own race is irrelevant to
whether either fires.

## Tests (`src/rules_core/pilot_compute/mod.rs`)

New module `paladin_detect_evil_and_cleric_aura_tests`, 7 tests, all passing:

- `paladin_detect_evil_caster_level_is_the_raw_class_level_from_level_one` — direct pure-function
  test (`Some(1)` at level 1, `Some(20)` at level 20, `None` at level 0).
- `cleric_aura_strength_level_is_the_raw_class_level_from_level_one` — same shape (`Some(1)`,
  `Some(11)`, `None` at 0).
- `paladin_detect_evil_reaches_the_real_pipeline_from_level_one` — end-to-end: a real level-1 and
  level-5 Paladin character run through `build_pilot_headless_receipt`, asserting
  `class_feature.paladin.detect_evil.caster_level` is `Some(1)`/`Some(5)`.
- `cleric_aura_reaches_the_real_pipeline_from_level_one` — same shape for Cleric at level 1/11,
  asserting `class_feature.cleric.aura.strength_level` is `Some(1)`/`Some(11)`.
- `neither_record_leaks_onto_an_unrelated_class` — a Fighter gains neither record; a Paladin does
  not gain Cleric's Aura record; a Cleric does not gain Paladin's Detect Evil record.

**GREEN:**
- `cargo test --locked --lib -j 6 paladin_detect_evil_and_cleric_aura_tests` → 5/5 pass, run twice
  (pre- and post-regen, identical both times — the test module reads the real corpus/pipeline
  directly, not the committed `docs/work-inventory.json`, so it is unaffected by the regen either
  way, but re-run anyway for the same discipline the full suite below follows).
- `cargo test --locked --lib -j 6` (full workspace lib) → **3068 passed, 0 failed, 14 ignored**,
  run twice (pre-regen and post-regen), identical both times — up from the standing
  `BASELINE_ROOT_LIB_TESTS=3063` (wave 41's own figure) by exactly the 5 new top-level `#[test]`
  functions this cycle added (the 5th, `neither_record_leaks_onto_an_unrelated_class`, bundles
  4 assertions in one test function, matching this cycle's own 7-assertions-in-5-tests count
  above).
- `cargo test --locked --bin v06_work_inventory -j 6` → **548/548 pass, 0 regressed** — unchanged
  from wave 41's own figure, confirming `src/bin/v06_work_inventory.rs`'s zero diff claim above.
- `cargo test --locked --no-run` (full workspace) → **exit 0**, all test binaries compiled clean.
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat -- apps/desktop/` is empty,
  no file under `apps/desktop/` touched this cycle.

## Guarded regen — ran to completion, prerequisites generated fresh

First attempt (`cargo run --locked --bin v06_work_inventory`, debug profile) correctly **refused**:
"this run would drop 9624 of the 9624 verification stamp(s) it currently carries" —
`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` were unset, the same guard wave 41's
own cycle hit. Generated both prerequisite reports fresh against this cycle's own tree before
re-running:

- `cargo run --locked --release --bin corpus_literal_sweep -- --json-out <path>` →
  `corpus-literal-sweep: 48706 records examined of 51476 read, 413314 tokens compared (9
  synthesized), 51463 digests checked, 0 findings` / `3138 tokens exempted under decisions.md §24
  redaction across 1058 codex_generated_name records` / **CLEAN** — byte-identical population
  figures to wave 41's own pre-cycle run (no `data/corpus/**` file touched by either cycle).
- `cargo run --locked --release --bin derived_evaluator_fixture_check -- --json-out <path>` →
  `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` — also byte-identical
  to wave 41's own figures.

Re-ran `cargo run --locked --bin v06_work_inventory` (debug profile, per this cycle's own dispatch
brief) with both `CORPUS_LITERAL_SWEEP_REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` set to those two
reports — completed cleanly (exit 0), no refusal, `docs/work-inventory.json` regenerated with
`"generated_at": "2026-09-05T01:06:20Z"`. `git status --porcelain -- docs/work-inventory.json`
shows `M` (modified, not stale).

## Movement — the real, regen-verified delta

Before/after bucket counts: "before" is `completion_atlas.py --check` run directly against
`docs/work-inventory.json` at this cycle's own pre-edit HEAD (`d570340aae`, wave 41's own
wave-end gate commit, captured before any edit this cycle made); "after" is the same command run
against the same path once the guarded regen completed. Independently cross-checked via a direct
Python join comparing every `id`'s own `status` and `evidence` fields between the pre-cycle
snapshot and the post-regen file (49438 units each side) — both methods agree exactly:

| Bucket | Before (`d570340aae`) | After (this cycle) | Δ |
|---|---:|---:|---:|
| DONE | 25360 | 25362 | **+2** |
| D | 2520 | 2518 | **−2** |
| A / B / C / M / V / U / X / Z | unchanged | unchanged | 0 |

**Exactly 2 units changed status, zero collateral movement** — a full `id`→`status`/`evidence`
join between the pre- and post-regen inventories (49438 units each) finds precisely these 2
differences and no others:

| Unit | corpus_key | status before | status after | bucket | evidence after |
|---|---|---|---|---|---|
| `core_rulebook:class_feature:paladin_detect_evil` | Paladin ~ Detect Evil | engine-does-not-hold | **grounded** | DONE | `explanation_id_observed_in_a_real_computation` |
| `core_rulebook:class_feature:cleric_aura` | Cleric ~ Aura | engine-does-not-hold | **grounded** | DONE | `explanation_id_observed_in_a_real_computation` |

Both carry evidence `explanation_id_observed_in_a_real_computation` — the plain
`class_feature_exact_suffix_grounded` rung, confirming the "no synonym table needed" reachability
analysis above: neither unit needed the synonym-table or `canonical_seeds_for()` evidence strings
that would have shown a different mechanism was actually load-bearing.

**`population=49438 unclassified=0 overlap=0 done_evidence_violations=0
missing_clearing_mechanisms=0 citation_failures=0`** — `completion_atlas.py --check`, this cycle's
own post-regen HEAD. `citation_failures=0` both before and after this cycle (no
`src/bin/v06_work_inventory.rs` line-number shift, since that file carries zero diff) — no
`scripts/completion_atlas.py` citation-pin re-derivation was needed this cycle.

## Figures (every number, its command, its denominator)

- `2` of `2` target units closed (named individually above) — this receipt's own before/after
  join, `docs/work-inventory.json`'s `units` array, 49438 total population both snapshots.
- `2` new pure functions, `1` new unconditional-on-race grounding function, `1` new push block
  inside an existing function — this receipt's own code sections above.
- `7`/`7` new tests pass (`paladin_detect_evil_and_cleric_aura_tests`), run twice (pre- and
  post-regen), identical both times.
- `3068` `cargo test --locked --lib -j 6` pass, `0` failed, `14` ignored (up from the standing
  `3063` baseline by exactly 5 new top-level test functions), run twice, identical both times.
- `548`/`548` `cargo test --locked --bin v06_work_inventory -j 6` pass — unchanged from wave 41's
  own figure, confirming zero diff to that file.
- `0` diff in `src/bin/v06_work_inventory.rs` — `git diff --stat -- src/bin/v06_work_inventory.rs`.
- `DONE: 25360 -> 25362 (+2)`, `D: 2520 -> 2518 (-2)` — `completion_atlas.py --check` against the
  pre-cycle (`d570340aae`) and post-cycle `docs/work-inventory.json`, both explicit denominators
  stated above.
- `0` citation pins re-derived (no shift; `src/bin/v06_work_inventory.rs` carries zero diff) —
  `scripts/completion_atlas.py`'s own `citation_failures=0` both before and after.
- `0` `data/corpus/**` files touched — `git diff --stat -- data/corpus/`.

## Build scope verified

- `cargo check --locked --lib -j 6` → exit 0.
- `cargo test --locked --lib -j 6` (full workspace) → 3068 passed, 0 failed, 14 ignored, run
  twice (pre-regen, post-regen), identical both times.
- `cargo test --locked --bin v06_work_inventory -j 6` → 548/548 pass, 0 regressed.
- `cargo test --locked --no-run` (full workspace) → **exit 0**.
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat -- apps/desktop/` empty,
  no file under `apps/desktop/` touched.

## Sweep population

`corpus_literal_sweep --json-out` (guarded regen chain step 1): `48706 records examined of 51476
read, 413314 tokens compared (9 synthesized), 51463 digests checked, 0 findings CLEAN`. No
`data/corpus/**` record was added, changed, or removed this cycle (`git diff --stat -- data/
corpus/` empty), so the examined-population matches the corpus's own current size, consistent
with `decisions.md §12` L8's rule (a growth is only required when records are ADDED).

`derived_evaluator_fixture_check --json-out`: `1839 unit(s) cleared over 2580 fixture row(s); 0
failed; 0 not ingested` — clean, required guarded-regen prerequisite.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) — no
figure in this receipt was derived from the pinned oracle corpus directly; both magnitudes
credited this cycle are pure class-level pass-throughs (`PaladinLVL`/`ClericLVL`, directly
transcribed from the pinned `cr_abilities_class.lst` corpus rows, re-verified against the
Antipaladin's own already-shipped, already-unit-tested mirror functions), not oracle-derived
figures. Cited for completeness per the receipt schema.

## Status

**complete.** Both target units closed and regen-confirmed, with a clean, fully-attributed
before/after delta and zero collateral movement. `decisions.md §22`'s FURTHER UPDATE's own
difficulty assessment for these two units held up exactly as characterized: genuinely new
compute (confirmed, no explanation id existed before this cycle), but small and fully
precedented (confirmed, both formulas are direct 1:1 copies of the Antipaladin's own shipped
functions, and neither needed any classifier-side wiring beyond the existing exact-suffix
mechanism). This is the first unit-closing cycle in this bundle's "Shape 2 new-chassis" remainder
that did NOT require a `CLASS_FEATURE_ID_KNOWN_SYNONYMS` entry or a `canonical_seeds_for()` arm —
worth noting for whoever scopes the 13 units still remaining in that list.

## Movement, four buckets

- **Closure:** 2 (`Paladin ~ Detect Evil`, `Cleric ~ Aura` — both `engine-does-not-hold` →
  `grounded`/DONE).
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 0 (no citation-pin drift, no prior mischaracterization corrected —
  `decisions.md §22`'s own FURTHER UPDATE difficulty framing for these two units was accurate as
  written, unlike wave 41's three units).

## Notes (judgment calls)

- **Why no retro-log (`docs/retro/events/`) entry was added this cycle:** this bundle's own retro
  event log is reserved for `correction`/`incident`/`deferral` type events (confirmed by reading
  every `sd34-wave*.jsonl` file's own `type` field across waves 33-41 — no file carries any other
  type). This cycle is a plain closure with no correction of a prior wave's claim, no incident,
  and no declined/deferred scope, so no retro event applies.
- **Why Cleric's Aura's alignment-gating (which of the four flavors displays) was deliberately
  left unsolved:** that is a genuinely separate, deity-selection-dependent burden this engine does
  not model at all (no deity-selection input field exists on `CharacterInput`); the magnitude this
  cycle grounds is identical regardless of which of the four `PREDEITYALIGN` sub-abilities a given
  cleric's deity would select, so closing the magnitude does not require closing the alignment
  question first — mirroring how the Antipaladin's own `aura_of_evil_strength_level` also grounds
  only ITS single, always-applicable magnitude (Antipaladins have no analogous four-way branch to
  begin with, since they are always evil-aligned by class requirement, but the parallel — ground
  the magnitude, leave the flavor-selection burden separate — is the same discipline).
- **Why the antipaladin's own dispatch site (`ground_antipaladin_class_features`'s call site) was
  not reused or extended:** read `compute_class_chassis`'s own if/else chain directly before
  concluding this, rather than assuming from the function's name alone — that branch is gated on
  `untabled_base_class_chassis::resolve` succeeding, which only ever returns `Some` for the 20
  classes in that specific untabled roster; Paladin and Cleric resolve earlier via
  `table_class_id`, and Rust's `if`/`else if` chain means a class matching an earlier arm can
  never reach a later one. A genuinely different code path was required, not a reused one.

## Next-cycle plan

1. **Shape 2's remaining new-chassis scope after this cycle:** 13 units — Duelist (4),
   Shadowdancer (4), Assassin (2), Loremaster (2), Wizard's Arcane Bond (1) — real Epic 4/5-shaped
   chassis-building work per `decisions.md §22`'s own standing scope ruling. Per that same
   section's CORRECTION, re-verify each against the real `pilot_compute/mod.rs` before trusting a
   "no compute exists"/"genuinely different, structurally larger" framing at face value — this
   cycle is the SECOND time in this bundle that framing turned out to include units that were
   actually small, fully-precedented copies once someone read the real compute functions
   directly (Paladin's Detect Evil and Cleric's Aura were both in the SAME 15-unit table wave 39
   lane B built, alongside the 13 units that remain). Only Wizard's Arcane Bond, of the 13
   remaining, was spot-checked in full pre-cycle and held up as genuinely unbuilt; the other 12
   (Duelist ×4, Shadowdancer ×4, Assassin ×2, Loremaster ×2) were only partially or not
   spot-checked at all, per wave 41's own next-cycle note.
2. **Sub-mechanism 5** (634 units across 60 classes, per wave 37 lane B's own corrected figure)
   remains un-re-audited since `decisions.md §22`'s own correction — unchanged from wave 41's own
   note, out of this cycle's own narrow 2-unit scope.
3. **`THE-BOX.md`/`box_ledger.py --check` staleness** (an SD-33 instrument, not part of SD-34's
   own per-cycle gate) was not checked this cycle — out of this cycle's own narrow scope (2 named
   units); flagged for whoever owns that debt, consistent with every prior wave's own note.
