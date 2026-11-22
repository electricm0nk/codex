# Cycle — SD-34 wave 35, Lane A — Skinwalker `Change Shape` TYPE-pool option resolver

**Status: partial.** Closes wave 33 lane B's own named 20-unit remainder (next-cycle plan item
2, `wave33_laneB_race_trait_never_applies_cycle_receipt.md`). Builds the TYPE-pool
option-picker resolver mechanism the receipt named as missing (`trait_pool.rs`'s
`resolve_adopted_race_options` idiom, extended for a shape that idiom cannot cover as-is — see
below), wires it end to end into the desktop TypeScript boundary and a real picker UI section
(verified: a selected kin's options render their real names on a real screen), and gives all
19 resolvable option records a precise `engine-does-not-hold` evidence string in place of the
blanket "never applies." **Zero units reach `DONE` this cycle** — every one of these 20 records
carries real, non-zero magnitude (a `TEMPBONUS` applied only when a player activates that
benefit during play), and `AGENTS.md`'s "a magnitude is not wired until it moves on the twin the
player reads" bar is not met by naming the option; no mechanism in this engine computes an
activated-during-play temporary bonus onto any character sheet today. This is the same
disposition wave 33 lane B gave its own 27-unit "Adopted Race"/"Adoptive Parentage" subset
before wave 34 lane B's UI wiring — except those 27 were zero-magnitude and closed to
`text-complete` once wired; these 20 are not, so wiring alone cannot close them.

## The real shape: a two-level TYPE-pool the ingest pipeline only half-captures

Each of Skinwalker's 9 kins (Werebat/Werebear/Wereboar/Werecrocodile/Wererat/Wereshark/
Weretiger/Werewolf/Wereraptor) carries a master `<Kin>-Kin ~ Change Shape` record whose own
`ABILITY:` token names a per-kin PCGen `TYPE=` pool (`ABILITY:Skinwalker Racial Trait|AUTOMATIC|
TYPE=Skinwalker Change Shape Werebear-Kin`). `RaceTraitRecord::automatic_trait_grants` already
reads this generically — reused, not re-parsed.

**`trait_pool.rs`'s idiom could not be reused unmodified.** That resolver indexes its pool
members by reading a `TYPE:` token directly off each member's own ingested record. Verified this
cycle: PCGen encodes a Change Shape option's pool membership as a `.MOD` row appended to an
*already-declared* ability elsewhere in the same `.lst` file (`CATEGORY=Special Ability|
Skinwalker ~ Change Shape (Bite).MOD  TYPE:Skinwalker Change Shape Werebear-Kin`), and this
project's `.lst` ingest pipeline does not fold a `.MOD` row's added `TYPE:` tokens back onto the
target record. Confirmed directly: every one of the 20 option records under
`data/corpus/bestiary_5/race_trait/skinwalker/skinwalker_change_shape_*.json` carries only its
own declaring row's `TYPE:Skinwalker Racial Trait`, never a per-kin tag. Fixing the ingest
pipeline generically (`.MOD` is a general PCGen mechanism, not Skinwalker-specific) is a
cross-cutting change well outside this cycle's scope — named as the real remaining gap, not
routed around silently.

**Resolution this cycle:** a new module, `src/rules_core/skinwalker_change_shape.rs`, carries a
static, cited pool-membership table (`KIN_OPTION_KEYS`, transcribed directly from the pinned
oracle's own `.MOD` rows, every entry citing its exact `path:line`), cross-checked by this
module's own tests against the live corpus (all 9 kins resolve, union of resolved keys pinned at
exactly 19). Six option names PCGen tags into more than one pool (`Claw`/`Constitution`/
`Darkvision`/`Dexterity`/`Natural Armor`/`Strength`) are declared only in the CRB "Default" pool's
own block and were never ingested into this project's curated `race_trait/` directory at all
(only into the separate `race_trait_generic/` population, a different content kind this module
does not read) — every kin row omits them rather than naming a key with no possible match. Caught
before landing: an initial draft included `Dexterity`/`Claw`/`Darkvision` in three kins' option
lists (assuming they were curated); the module's own test suite failed loudly
(`Werebat-Kin should resolve exactly 5 real grants, found 4`), and the omission was verified
against the live oracle `.lst` text before correcting the table — never guessed.

## `Endurance` is a genuine, verified orphan

Of the 20 real option records, 19 are named by at least one kin's `.MOD` row (cross-checked
independently against the pinned oracle: `grep -c 'Endurance).MOD'` over both `skinwalker_
abilities_race.lst` and `skinwalker_abilities_race_subrace.lst` returns `0`). `Skinwalker ~
Change Shape (Endurance)` is real content (a genuine `TEMPBONUS` granting virtual Endurance) with
no real consumer anywhere in the upstream data — the identical disposition wave 33 lane B gave
Bestiary 6's Rougarou selector: correctly inert, no project-side remedy possible short of an
upstream PCGen data change.

## Files touched this cycle

- `src/rules_core/skinwalker_change_shape.rs` (new) — the resolver, `KIN_OPTION_KEYS` static
  table, 3 tests (all against the real live corpus, no fixtures).
- `src/rules_core/mod.rs` — registers the new module.
- `src/bin/v06_work_inventory.rs` — one new `RaceTraitProbe` field
  (`skinwalker_change_shape_option_resolved`), populated in `probe_race_trait_corpus` by calling
  the resolver; one new `EngineFacts` accessor; one new `classify()` check in the `Kind::
  RaceTrait` arm giving 19 of the 20 units a precise, non-`done` `engine-does-not-hold` evidence
  string; 2 tests updated/added (1 RED→GREEN proof against the real corpus pinning the new
  evidence, 1 regression pinning `Endurance`'s unchanged blanket evidence).
- `apps/desktop/src-tauri/src/race_trait_picker.rs` — two new DTOs
  (`SkinwalkerChangeShapeOptionDto`/`SkinwalkerChangeShapeGrantDto`), a new
  `AlternateRacialTraitsResponse.skinwalker_change_shape_options` field populated in `build_menu`
  over the SAME already-loaded `RaceCorpus` (no second corpus root or pool load needed, unlike
  the Adopted Race shape), 1 new end-to-end test proving the real Tauri command surface carries
  all 9 kin pools with real grants.
- `apps/desktop/src/boundary/loadAlternateRacialTraits.ts` — declares
  `SkinwalkerChangeShapeOptionDto`/`SkinwalkerChangeShapeGrantDto`/
  `AlternateRacialTraitsResponse.skinwalkerChangeShapeOptions`.
- `apps/desktop/src/raceCatalog/alternateTraitPickerModel.ts` — two new pure view-model
  helpers (`describeSkinwalkerChangeShapeOptions`/`describeSkinwalkerChangeShapeGrants`).
- `apps/desktop/src/raceCatalog/AlternateTraitPicker.tsx` — a new "Skinwalker Change Shape"
  picker section: one pill per kin, selecting one renders its real option names.
- `apps/desktop/src/raceCatalog/alternateTraitPickerModel.test.ts`,
  `apps/desktop/src/characterHub/alternateTraitSelection.test.ts` — updated fixture literals
  (new required DTO field) + new assertions for the two new helpers.
- `scripts/completion_atlas.py` — all 10 `BUCKET_DEFINITIONS` citation lines re-derived fresh
  against this cycle's own insertions (see "Citation re-derivation" below); one of the ten (V)
  was ALSO corrected to its real construction site, a pre-existing latent defect this cycle
  found, not introduced.
- `docs/work-inventory.json`,
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` —
  regenerated via the guarded path (below), never hand-merged.
- This receipt, `progress.md`.

## Citation re-derivation — all 10, one a genuine latent defect

This cycle's own additions (a new `RaceTraitProbe` field with its doc comment, the probe's
population block, one new `EngineFacts` accessor, one new `classify()` branch) sit above every
`BUCKET_DEFINITIONS` citation line, shifting all ten. Re-derived fresh (`--check` confirmed
`citation_failures=9` before the fix — DONE/A/B/C/D/M/U/X/Z; V's line coincidentally still
CONTAINED its target substring after the shift):

| Bucket | Old line | New line | Marker |
|---|---:|---:|---|
| DONE | 10290 | 10343 | `grounded` |
| A | 12780 | 12857 | `has_no_engine_table` |
| B | 12460 | 12537 | `not_held_by_engine` |
| C | 12685 | 12762 | `explanation_id` |
| D | 10464 | 10517 | `engine-does-not-hold` |
| M | 10299 | 10352 | `ingested-magnitude` |
| U | 10551 | 10604 | `unmeasurable` |
| X | 10511 | 10564 | `deferred-with-reason` |
| Z | 10372 | 10425 | `not-started` |
| V | 13562 | 13639 | `literal-verified` |

**V is a real, pre-existing latent defect, not this cycle's own shift.** `--check` did not flag
it (line 13562 still happened to contain the substring "literal-verified" after this cycle's own
insertions moved it), but reading the line's real content showed it was a DOC COMMENT mentioning
the literal, not the real `item.verdict.status = "literal-verified";` assignment site — the exact
"a citation that passes the string check but is not the real call site" hazard. Re-derived to the
real construction site (13639) and retro-logged as a `correction`
(`docs/retro/events/sd34-wave35-lanea.jsonl`, `1788405610803-sd34-wave35-lanea-e70e6d`).
`denominator_gate.py`/`--check-provenance`: `violations=0` both.

## RED → GREEN

- RED (confirmed before the fix): the module's own `all_nine_kins_resolve_real_nonempty_grants`
  test failed against the FIRST draft of `KIN_OPTION_KEYS` (Werebat-Kin: expected 5, found 4) —
  the intended reason, a real omitted key (`Dexterity`, CRB-only, not curated), not a typo.
  `v06_work_inventory.rs`'s own `a_skinwalker_change_shape_component_with_a_real_kin_pool_gets_
  the_precise_evidence` test failed against the PRE-fix classify() (asserted the old blanket
  evidence, which the fix must move off of) before the new `classify()` branch was added.
- GREEN: `cargo test --locked --lib skinwalker_change_shape:: -j 6` → 3 passed, 0 failed.
  `cargo test --locked --bin v06_work_inventory race_trait_grounding_tests:: -j 6` → 39 passed
  (37 pre-existing + 2 new/updated), 0 failed. Full binary suite:
  `cargo test --locked --bin v06_work_inventory -j 6` → 516 passed, 0 failed.
  `cargo clippy --locked --bin v06_work_inventory -j 6` / `--lib -j 6` → clean, 0 warnings.
- Desktop crate: `cargo test --locked -j 6 race_trait_picker::` (workspace) → 20 passed (19
  pre-existing + 1 new end-to-end proof), 0 failed. `cd apps/desktop/src-tauri && cargo check
  --locked -j 6` → exit 0, only pre-existing unrelated `spell_catalog.rs` dead-code warnings.
  `cargo clippy --locked -j 6` (desktop crate) → 0 warnings on any file this cycle touched.
- Frontend: `npm run typecheck` → clean. `npm test` (full suite, 100 files) → 100/100 pass,
  including the 2 files this cycle edited.
- `cargo test --locked --no-run` (full workspace) — deferred to wave-end per this fold's
  convention; not re-run here (single-lane cycle, no other lane landed concurrently in this
  worktree to verify against).

## Guarded regeneration

```
cargo run --locked --bin corpus_literal_sweep -- --json-out <scratch>/corpus_literal_sweep_report.json --quiet
-> corpus-literal-sweep: CLEAN
cargo run --locked --bin derived_evaluator_fixture_check -- --json-out <scratch>/derived_evaluator_fixture_check_report.json --quiet
-> fixtures_total=2580 failed=0
CORPUS_LITERAL_SWEEP_REPORT=<scratch>/corpus_literal_sweep_report.json \
DERIVED_FIXTURE_CHECK_REPORT=<scratch>/derived_evaluator_fixture_check_report.json \
cargo run --locked --bin v06_work_inventory -j 6
-> docs/work-inventory.json regenerated, 49438 units
```

`python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"` →
**49438** — unchanged (no corpus records added or removed this cycle, only one new probe field
and one new `classify()` check).

## Movement (four buckets, this cycle)

- **Closure (bucket → DONE):** 0.
- **Reclassification (bucket → different non-DONE bucket):** 0 — all 20 stay in bucket D
  (`engine-does-not-hold`).
- **Reachability:** 0 — the resolved magnitude reaches no character sheet; a player CAN now see
  the real list of Change Shape benefit names for their kin on a real screen (a genuine new UI
  surface), which is real but is not the "a number moved" bar this project's own reachability
  bucket requires.
- **Instrument-correction:** 19 evidence strings replaced (a false blanket "never applies" with
  a precise, verified "this kin pool resolves for real, no activation mechanism computes its
  magnitude"), plus 10 `completion_atlas.py` citation lines re-derived (9 shifted by this
  cycle's own insertions, 1 a genuine pre-existing latent defect this cycle found and fixed).

## Figures (every number, its command, its denominator)

- Population, this shape: **20** —
  `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(len([u for u in d['units'] if u.get('source_file')=='skinwalker_abilities_race_subrace.lst' and 230<=u.get('source_line',0)<=249]))"`,
  denominator: the exact 20-line KEY-declaration block wave 33 lane B named.
- New `..._resolves_real_kin_pool_but_no_activation_mechanism_computes_its_magnitude` evidence:
  **19** —
  `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(len([u for u in d['units'] if u.get('evidence')=='race_trait_skinwalker_change_shape_option_resolves_real_kin_pool_but_no_activation_mechanism_computes_its_magnitude']))"`,
  denominator: the 20-unit population; the 20th (`Endurance`) stays on the blanket evidence.
- Still-blanket `race_trait_record_loaded_but_never_applies`, whole corpus: **7** (was 26 after
  wave 33 lane B: 20 skinwalker + 2 Human Ethnicity + 1 Oversized Goblin + 2 `inner_sea_races` +
  1 Rougarou selector; this cycle moves 19 of the 20 skinwalker units off it, leaving
  `Endurance` + the other 6 unchanged) —
  `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(len([u for u in d['units'] if u.get('evidence')=='race_trait_record_loaded_but_never_applies']))"`,
  denominator: whole corpus.
- `docs/work-inventory.json` total population: **49438** (unchanged) —
  `python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"`.
- `completion_atlas.py --check`: `population=49438 buckets=10 unclassified=0 overlap=0
  citation_failures=0` — `python3 scripts/completion_atlas.py --check`, denominator: whole
  corpus. Bucket D unchanged in count (evidence-string precision only, no unit moves bucket).

## Remainder, named by mechanism (all 20, none unnamed)

1. **19** — `race_trait_skinwalker_change_shape_option_resolves_real_kin_pool_but_no_activation_
   mechanism_computes_its_magnitude`. Real next-cycle work, and the genuinely hard remaining
   step: this project has no mechanism anywhere for an activated-during-play temporary bonus
   (PCGen's own `TEMPBONUS` concept). Building one is a materially different, much larger
   undertaking than a picker — it needs a "declare which benefit is currently active" character
   state this engine does not model at all, then a compute path that reads it. Cheaper partial
   step available: the ingest pipeline gap this cycle's module doc names (`.MOD` rows not
   folded back onto their target record) could be fixed generically, which would let a FUTURE
   cycle build the resolver the ordinary `trait_pool.rs`-idiom way instead of via a static table
   — real, but does not by itself close any unit either.
2. **1** — `Endurance`. Correctly inert, no real consumer anywhere in the upstream data. No
   project-side remedy exists; would need an upstream PCGen data addition. Name it as
   permanently blocked pending upstream, not a to-do — the same disposition as `Rougarou`'s
   selector and `Suli ~ Trusted Mediator` in wave 33 lane B's own remainder table.

## Verification

- `python3 scripts/completion_atlas.py --check` →
  `population=49438 buckets=10 unclassified=0 overlap=0`;
  `DONE=25027 A=449 B=11769 C=4173 D=2891 M=4449 V=289 U=202 X=170 Z=19`;
  `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
  citation_failures=0`. `D` unchanged from immediately before this cycle's regen (2891) —
  confirming the evidence-string change moves no unit between buckets, exactly as this
  cycle's own "Movement" section states.
- `python3 scripts/denominator_gate.py --check` → `files_checked=160 violations=0`.
- `python3 scripts/denominator_gate.py --check-provenance` → `files_checked=90
  figures_examined=128 violations=0`.
- `cargo run --locked --bin corpus_literal_sweep -- --json-out <scratch> --quiet` →
  `corpus-literal-sweep: CLEAN`, examined-count **48706 before → 48706 after** (unchanged,
  matching zero corpus records added/removed).
- `git status --porcelain` clean before every write this cycle; no `git add -A`; each
  `git diff --cached --numstat` read before committing.

## Next-cycle plan

1. **19** (Skinwalker Change Shape options) — the real remaining work is an activation-state
   mechanism, not a picker: this engine needs a concept of "which one Change Shape benefit is
   currently active" before any magnitude can compute onto a character sheet. Out of scope for
   a single cycle; likely needs an operator ruling on whether this project models play-time
   temporary bonuses at all, or whether these 19 are named as a permanently-deferred category
   (the mechanic PCGen calls "Temporary Bonuses" may be genuinely out of scope for a
   character-BUILDER tool, as distinct from a live combat tracker).
2. **Ingest pipeline gap** (a real, separately-named defect, not part of the 20-unit population):
   `.MOD` rows are not folded back onto their target record's own `TYPE:` tokens anywhere in
   this project's ingest pipeline. Cross-cutting, affects any future book using the same PCGen
   pattern. Filed here, not fixed — outside this cycle's granted scope.
3. **Human Ethnicity + Oversized Goblin** (2+1, wave 33 lane B's own remainder items, unchanged
   by this cycle) — still need an operator ruling on whether a dedicated picker UI is in scope.
4. **`inner_sea_races` + Rougarou** (2+1, wave 33 lane B's own remainder items, unchanged by
   this cycle) — upstream data gaps, no project-side remedy, permanently blocked pending
   upstream.
