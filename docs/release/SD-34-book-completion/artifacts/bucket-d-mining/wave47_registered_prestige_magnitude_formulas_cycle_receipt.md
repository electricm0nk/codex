# Cycle — SD-34 wave 47 — Divine Scion's magnitude-only remainder: 43 of 45 units closed, one real correction found and fixed mid-recovery

- **Commit SHA:** `24666d0667` (`24666d066794e1fc578c4a11cf3913cced8ee2e2`, feat commit; this
  receipt's own SHA fill-in lands in a second, docs-only commit immediately after, following this
  bundle's own established two-commit pattern)
- **Recovery context.** This wave's own build agent (`sd34-wave47.workflow.js`) ran for ~48
  minutes, wrote real, substantial code (a `probe_divine_scion_wiring` function and
  `divine_scion_wired` `EngineFacts` field in `src/bin/v06_work_inventory.rs`, and a
  `ground_divine_scion_class_features` implementation plus a full test module in
  `src/rules_core/pilot_compute/mod.rs`), then stalled without committing, writing a receipt, or
  running final verification. This cycle recovered that work: read the FULL uncommitted diff
  against the real corpus records directly, found one genuine correctness bug (below), fixed it,
  finished the wiring, wrote/adjusted tests, and completed all verification the stalled agent
  never ran.
- **Files touched:** `src/rules_core/pilot_compute/mod.rs` (1 new class-id const
  `DIVINE_SCION_CLASS_ID` plus 2 new choice-set-id consts
  `DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID`/`DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID`
  added this cycle; 6 new pure formula functions (`divine_scion_domain_specialization_pool_size`,
  `divine_scion_divine_wrath_bonus`, `divine_scion_deific_defense_bonus`,
  `divine_scion_opposition_alignment_dr`, `divine_scion_weapon_and_armor_proficiency_qualify_flag`,
  `total_character_level`) plus a 35-entry domain table
  (`DIVINE_SCION_DOMAIN_SPECIALIZATION_USES_PER_DAY`); 1 new dispatch function
  `ground_divine_scion_class_features`, called unconditionally from
  `compute_pilot_base_chassis` (no `ClassId` enum entry exists for this class); 1 test module,
  `wave47_divine_scion_class_features_tests`, rewritten this cycle to test the corrected
  choice-gating, 13 tests), `src/bin/v06_work_inventory.rs` (1 new `EngineFacts` field
  `divine_scion_wired`; `probe_divine_scion_wiring` — the recovered agent's own version rewritten
  this cycle to sweep every candidate domain/alignment selection, not assume all are
  simultaneously active; 1 new local correspondence table
  `DIVINE_SCION_DOMAIN_SPECIALIZATION_SLUGS`; 1 new `canonical_seeds_for` match arm for
  `"divine_scion"`; 1 new `classify()` early-return check (unchanged from the recovered draft);
  2 test modules — `wave47_divine_scion_probe_reachability_tests` (1 test) and
  `wave47_divine_scion_classify_tests` (3 tests) — 4 tests total, unchanged from the recovered
  draft), `scripts/completion_atlas.py` (10 bucket citation-pin re-derivations — this cycle's own
  insertions shifted every downstream construction-site line number in `v06_work_inventory.rs`,
  the same pattern every prior wave in this series hit), `scripts/shape_engine_boundary.py` /
  `scripts/missing_engine_tables.py` / `scripts/tests/test_shape_engine_boundary.py` (citation-pin
  re-derivations for the same reason — the pre-existing, unrelated `not_held_by_engine` population
  drift named by waves 44-46 is left named, not fixed, same as those waves' own choice),
  `docs/work-inventory.json` (regenerated via the guarded path), `docs/release/SD-34-book-
  completion/artifacts/epic-1-atlas/completion-atlas.json` (a `--check` re-run's own artifact),
  `scripts/verify-baselines.env`, this receipt, `progress.md`, `decisions.md`, `kanban.md`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- src/rules_core/
  pilot_compute/mod.rs src/bin/v06_work_inventory.rs`, no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}`
  hits outside this wave's own house-style `wave47_*` module names, matching the file's own
  existing `wave4[1-6]_*` convention).
- **Wired-integration audit result:** `OK_NO_TOKENS` (0 hits for `placeholder`/`STUB`/`MOCK`/
  `not yet implemented`/`fixme`/`hack` in this cycle's own diff).

## The one real correction: Divine Scion's Domain Specialization and Opposition Alignment are real one-of-N choices, not single-owner unconditional grants

**What the recovered draft got wrong.** The stalled agent's `ground_divine_scion_class_features`
ground all 35 per-domain Domain Specialization sub-records AND all 4 Opposition Alignment DR
records unconditionally, for every Divine Scion character simultaneously — regardless of which
domain or opposition alignment the character actually has. This is exactly the class of defect
SD-33/SD-34's own recurring doctrine exists to catch: a computed value that looks right (a real
formula, a real corpus citation, real tests) but is factually wrong for any real character
querying it, since a divine scion only ever has ONE domain specialization and ONE opposition
alignment, never all of them.

**How this was found.** Reading the real, non-ingested PCGen oracle directly
(`~/workspace/repos/pcgen/data/pathfinder/paizo/campaign_setting/inner_sea_magic/
ism_classes.lst:103`/`:104`) shows both are granted via `BONUS:ABILITYPOOL|Opposition
Alignment|1` / `BONUS:ABILITYPOOL|Domain Specialization|1` — pool SIZE 1, a genuine one-of-N
selection — and `ism_abilities_class.lst`'s own section headers at lines 35 and 47 read literally
`# Opposition Alignment choices` and `# Domain Specialization choices`. This is the IDENTICAL
shape this codebase already gates everywhere else via a real `choice_selection(input,
CHOICE_ID)` check (`SORCERER_BLOODLINE_CHOICE_ID`, `CLERIC_DOMAIN_CHOICE_ID`,
`BLOODRAGER_BLOODLINE_CHOICE_ID`, `RANGER_COMBAT_STYLE_CHOICE_ID`, and roughly 30 other call
sites) — the recovered draft's own doc comment even correctly excluded True Scion Charisma/Wisdom
citing this EXACT reasoning ("a genuine pool-selection-state question this engine does not yet
track"), but did not apply the same reasoning to Domain Specialization/Opposition Alignment,
despite both carrying the identical `ABILITYPOOL` shape.

**The fix.** Two new choice-set-id consts
(`DIVINE_SCION_DOMAIN_SPECIALIZATION_CHOICE_ID`/`DIVINE_SCION_OPPOSITION_ALIGNMENT_CHOICE_ID`),
following the `"domain:<slug>"`/`"alignment:<slug>"` selection-id convention already established
elsewhere. `ground_divine_scion_class_features` now gates the per-domain block and the
per-alignment DR block on `choice_selection(input, <CHOICE_ID>)` matching the recorded selection
— a real character now surfaces exactly the ONE domain and ONE alignment it actually recorded,
never all 39 (35 + 4) simultaneously, never a false default. The 4 genuinely unconditional
single-owner facts (Domain Specialization's own pool-SIZE, Divine Wrath, Deific Defense, Weapon
and Armor Proficiency) are untouched — their magnitude does not depend on WHICH domain/alignment
was picked, only that the class feature was granted at all, matching the same reasoning that kept
Divine Wrath/Deific Defense correct in the recovered draft.

**Reachability, corrected to match.** `probe_divine_scion_wiring` (used both by the corpus-wide
census sweep and this wave's own reachability tests) is rewritten to sweep every one of the 35
domain selections and 4 alignment selections in turn (via `class_sweep_input` + an explicit
choice override per iteration), collecting the union of corpus keys the real pipeline resolves —
the same `probe_cleric_domain_generic_member_wiring` idiom this file already uses for Cleric
Domain/Sorcerer Bloodline, adapted for a hand-rolled (not generic-pool-group) grounding function.
`canonical_seeds_for` gained a `"divine_scion"` arm (`domain:fire` / `alignment:evil` as the
canonical default pair), matching the same "give the sweep one canonical default choice"
convention this function already uses for wizard/cleric/sorcerer/fighter/psychic and others.

**Net effect on THIS wave's own closure count: none.** All 43 corpus keys (the same 43 the
recovered draft targeted) still resolve reachable — because reachability at the corpus-wide
census level means "the real pipeline resolves this key for SOME real character configuration,"
not "every character has it," the same existence-based semantics Cleric Domain's own ~9-domain
population already established. What changed is CORRECTNESS: a real Divine Scion character's own
receipt now shows exactly the domain/alignment they recorded, not a fabricated 39-facts-at-once
answer — the actual defect this correction closes, even though the atlas bucket-count outcome is
identical to what the (wrong) recovered draft would have produced.

## The 43 units closed, and the 2 left named

**43 of Divine Scion's 45 sub-mechanism-5 units closed** — Domain Specialization's own pool-size
base record, Divine Wrath, Deific Defense, Weapon and Armor Proficiency, all four Opposition
Alignment DR records (Chaotic/Evil/Good/Lawful), and all 35 per-domain Domain Specialization
sub-records (Air through Weather) — every formula verified directly against the real corpus
JSON AND independently cross-checked against the real, non-ingested PCGen oracle
(`ism_abilities_class.lst` lines 29-83, `ism_classes.lst` lines 103-106).

**Representative sample checked token-by-token against the raw oracle** (per this wave's own
verification brief): Domain Specialization pool size (`BONUS:ABILITYPOOL|Domain
Specialization|1` — literally 1, unconditional), Divine Wrath (`BONUS:VAR|DivineWrathBonus|1`),
Deific Defense (`BONUS:VAR|DeificDefenseBonus|2`), one Opposition Alignment DR (Evil:
`DR:DeificDefenseBonus/evil|PREABILITY:1,...`, restating the identical `DeificDefenseBonus`
magnitude), and one per-domain caster-level/uses-per-day pair (Fire:
`SPELLS:Innate|TIMES=1|CASTERLEVEL=TL|Flame Arrow,...` — caster level = total character level,
1/day) — all matched exactly.

**True Scion Charisma/Wisdom (this class's own remaining 2 units) are NOT attempted this cycle**
(unchanged from the recovered draft's own honest scoping): a real `ABILITYPOOL|True Scion|1`
mutually-exclusive choice between an ability-score bump to Charisma or Wisdom, each ALSO
re-stating the same `DomainSpecBonus`/`DivineWrathBonus`/`DeificDefenseBonus` increments already
excluded above — a genuine pool-selection-state question this engine does not yet track for this
specific choice, left named for a future wave.

## Tests

The recovered draft already had substantial tests; this cycle verified they were real (not just
present) and rewrote the ones the correction invalidated:

- `src/rules_core/pilot_compute/mod.rs`'s `wave47_divine_scion_class_features_tests` (13 tests):
  pure-formula tests for all 6 new functions (including the 35-entry domain table's own oracle
  cross-check), a real-pipeline reachability test for the 4 unconditional facts, a new RED-for-
  the-right-reason test proving the 39 choice-gated facts are ABSENT with no recorded selection
  (the exact bug this cycle fixed), a test proving exactly the recorded alignment/domain surfaces
  and no sibling leaks, the existing level-gate and cross-class negative-control tests (updated
  to record a choice so the level gate is isolated from the choice gate).
- `src/bin/v06_work_inventory.rs`'s `wave47_divine_scion_probe_reachability_tests` (1 test,
  `divine_scion_is_wired_end_to_end`, unchanged assertion — still proves all 43 keys reachable,
  now via the corrected sweep) and `wave47_divine_scion_classify_tests` (3 tests, unchanged from
  the recovered draft — the `classify()`-level dispatch and negative control were already correct).

## Verification (mandatory)

`cargo check --locked --lib -j 6` → exit 0. `cargo check --locked --bin v06_work_inventory` →
exit 0.

`cargo test --locked --lib -j 6` (full lib suite) → **3134 passed, 0 failed, 14 ignored** (up
from the standing 3121 baseline by exactly this cycle's 13 new lib tests).

`cargo test --locked --bin v06_work_inventory -j 6 divine_scion` → 4 passed, 0 failed.

`cargo test --locked --no-fail-fast -j 6` (full workspace), **run 1** → **8562 passed, 0 failed,
67 ignored, across 589 suites** (up from the standing 8545 baseline by exactly +17 = the same 13
new lib tests, counted again since root-full runs the lib suite too, plus 4 new bin tests). The
F1/shape_ledger pin update below (a real `.rs` edit, `formula_interpreter_corpus_wide.rs`)
triggered this wave's own "re-run the full suite a SECOND time if any code change happens after
the first run" rule. **Run 2**, against the fully-settled tree → **8562 passed, 0 failed, 67
ignored, across 589 suites** — byte-identical to run 1, confirming the pin update introduced no
regression.

## Guarded regen

`corpus_literal_sweep --json-out` → `48706 records examined of 51476 read, 413314 tokens compared
(9 synthesized), 51463 digests checked, 0 findings` / `3138 tokens exempted under decisions.md
§24 redaction across 1058 codex_generated_name records` / **CLEAN** — byte-identical to wave 46's
own pre-cycle figures (`git diff --stat -- data/corpus/` empty; no corpus file touched this
cycle).

`derived_evaluator_fixture_check --json-out` → `1839 unit(s) cleared over 2580 fixture row(s); 0
failed; 0 not ingested` — also byte-identical.

`cargo run --locked --bin v06_work_inventory` with both reports set → completed cleanly (exit 0),
no refusal, `docs/work-inventory.json` regenerated (`git status --porcelain` shows `M`, not
stale).

## Before/after bucket movement

`python3 scripts/completion_atlas.py --check` at this cycle's own pre-edit HEAD
(`6afdd4922a759a7adab763c0229d2142b19f9e44`, wave 46's own wave-end-gate commit):
`population=49438 unclassified=0 overlap=0`, `DONE: 25419`, `D: 2441`, `V: 345` — exactly the
wave 46 gate baseline named in this wave's dispatch brief.

**Post-regen:** `python3 scripts/completion_atlas.py --check`: `population=49438 unclassified=0
overlap=0 citation_failures=0`, `DONE: 25419→25458 (+39)`, `D: 2441→2398 (−43)`,
`V: 345→349 (+4)`, every other bucket unchanged.

Independently re-derived via a direct Python `id`→`status` join over both the pre- and
post-regen inventory snapshots (not just `--check`'s own summary): pre/post population both
49438, 0 added, 0 removed, **exactly 43 units changed status, zero collateral movement** — all 43
are Divine Scion's own `inner_sea_magic:class_feature:divine_scion_*` ids, every one
`engine-does-not-hold` before. 39 landed `grounded` (all 35 per-domain Domain Specialization
sub-records plus all 4 Opposition Alignment DR records — every one `wiring_class: computed`); 4
landed `literal-verified` (Deific Defense, Divine Wrath, Domain Specialization's own pool-size
record, Weapon and Armor Proficiency — every one `wiring_class: static`, and each one's own
`(book, file, line)` triple was independently confirmed present in this cycle's own
`corpus_literal_sweep` report). True Scion Charisma and True Scion Wisdom are confirmed absent
from the changed set — still `engine-does-not-hold`, exactly as intended.

## F1/shape_ledger pin

**Moved: 5193 → 5155.** This cycle's 43 closures include exactly 38 F1-shaped units, verified
per-id via `python3 scripts/shape_ledger.py --inventory <pre-cycle snapshot> --corpus-root
data/corpus --output <path>` against the PRE-cycle inventory (since a unit that leaves the
not-done population no longer appears in a post-regen scan): all 4 unconditional records (Domain
Specialization's own pool-size record, Divine Wrath, Deific Defense, Weapon and Armor
Proficiency — each one's own `BONUS:VAR`/`BONUS:ABILITYPOOL` token is a bare literal) plus 34 of
the 35 per-domain records (each one's own secondary `BONUS:SKILL`/`BONUS:SAVE`/`BONUS:COMBAT`
token is a bare literal — the shape `shape_ledger.py` classifies on the record's own raw-token
shape, not this engine's resolution chain, which instead grounds the SPELLS-field caster level).
The other 5 closed units are NOT F1-shaped: 4 are `F0` (the four Opposition
Alignment records' own `DR:` token is not recognized by this classifier's token scan at all —
`no_formula_tokens`) and 1 is `F8` (Void Specialization — the one domain record whose secondary
`BONUS:CONCENTRATION|4|...` token this classifier's own rule list does not resolve to F1, a
residual per its own documented blind spot, not a re-derivation error: confirmed directly against
`shape_ledger.py --output`, not assumed from the other 34 domains' shared shape). 38 + 4 + 1 = 43.
`python3 scripts/shape_ledger.py
--inventory docs/work-inventory.json --corpus-root data/corpus` re-run against the post-regen
inventory to confirm the population-level arithmetic: `F1 = 5155` exactly (5193 − 38).
`formula_interpreter_corpus_wide.rs`'s own pinned census test updated to match
(`f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census`, 5193 →
5155), following its own established dated doc-comment convention.

## Findings for a future wave

1. **Sub-mechanism 5's remaining population after this wave: 591** (634 − 43),
   split across the remaining 54 registered prestige classes (down from 55 — Divine Scion,
   formerly the single largest remaining class at 45 units, is now down to its own 2-unit True
   Scion remainder) and the 88 not-registered units (unchanged, named by slug in prior waves'
   gate entries).
2. **Divine Scion's own True Scion Charisma/Wisdom (2 units)** remain named, not attempted — a
   real `ABILITYPOOL|True Scion|1` mutually-exclusive choice this engine does not yet track a
   selection for (see above).
3. **Every prior wave's own "confirmed AS/MB/Ma-shaped Ultimate Psionics classes" and "large
   heterogeneous classes" findings are unchanged** — this wave touched only Divine Scion.
4. **A generalizable finding for future waves against this same population:** before grounding
   ANY corpus record shaped `# <X> choices` / granted via `BONUS:ABILITYPOOL|<X>|1`, check
   whether the class's OTHER already-excluded units (like True Scion here) share the identical
   `ABILITYPOOL` shape — a wave that correctly excludes one such choice but grounds a sibling
   choice unconditionally is the exact defect this cycle's own correction fixed. Grep the raw
   oracle's own section-header comments (`# ... choices`) as a cheap first signal.

## Build scope verified

- `cargo check --locked --lib -j 6` → exit 0.
- `cargo check --locked --bin v06_work_inventory` → exit 0.
- `cargo test --locked --lib -j 6 wave47_divine_scion` → 13 passed, 0 failed.
- `cargo test --locked --bin v06_work_inventory -j 6 divine_scion` → 4 passed, 0 failed.
- `cargo test --locked --lib -j 6` (full lib suite) → 3134 passed, 0 failed, 14 ignored.
- `cargo test --locked --no-fail-fast -j 6` (full workspace), run to completion twice against the
  fully-settled tree → **8562 passed, 0 failed, 67 ignored, 589 suites, exit 0, identically both
  times**.
- `scripts/verify-baselines.env` updated: `BASELINE_ROOT_LIB_TESTS` 3121→3134,
  `BASELINE_ROOT_FULL_TESTS` 8545→8562 (dated entry appended following the file's own convention).
- `python3 scripts/completion_atlas.py --check` (post-regen, re-run independently after the full
  suite): `population=49438 unclassified=0 overlap=0 citation_failures=0`, bucket counts match
  this receipt's own before/after table exactly.
- `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus`
  (re-run independently after the full suite): `F1 = 5155`, matching this receipt's own F1/pin
  section exactly.
- `python3 scripts/denominator_gate.py --check`: `files_checked=182 violations=0`.
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat -- apps/desktop/` empty, no
  file under `apps/desktop/` touched this cycle.
