# Cycle 005 — Gate 1 (shape closure) / T9-onboarding wave 3: equipment/equipment_modifier `no_record`

- **Card ID:** `gate-1-shape-closure`
- **Commit SHA:** (filled after commit, see below)
- **Files touched:**
  - `src/rules_core/cache_gen/hand_authored_equipment.rs` (fix + tests)
  - `src/bin/gen_cache_hand_authored_equipment.rs` (report both counts)
  - `data/corpus/ultimate_psionics/equipment/equipmods/*.json` (113 new records)
  - `data/corpus/ultimate_combat/equipment/equipmods/*.json` (19 new records)
  - `data/corpus/ultimate_intrigue/equipment/equipmods/*.json` (7 new records)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 -- src/rules_core/cache_gen/hand_authored_equipment.rs src/bin/gen_cache_hand_authored_equipment.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no match)
- **Wired-integration audit result:** `OK_NO_TOKENS`
  (same diff `| grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match)
- **Acceptance criterion:** `decisions.md §20` — "`no_record` must reach ZERO. The budget is a
  ratchet, not a finish line." Scope for this cycle: `equipment_modifier` (175 at start).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
  re-derive via `scripts/verify.sh --only preflight-oracle`)
- **Status:** complete
- **Notes:**

## Task and lead

Dispatch brief's lead: *"`gen_equipment_gap_tables.rs`'s row selection captures a `.COPY=` alias key
instead of the base declaration for at least one traced `ultimate_psionics` unit."* Re-derived per
`§17a` before trusting it (a sibling lane's own precedent finding, itself corrected under `§17a` once
already this bundle).

## Re-derivation of the starting count

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json
```
→ `no_record` 1,114 total; `equipment` 170, `equipment_modifier` 175 — **matches the brief exactly.**

## What the brief's lead actually was (traced, not assumed)

Traced the named `ultimate_psionics` unit end to end
(`ultimate_psionics:equipment_modifier:special_ability_psionic_blade_weapon`,
`up_equipmods.lst:12`, `KEY:Special Ability ~ Psionic Blade ~ Weapon`, `COST:0`) against
`data/corpus/ultimate_psionics/equipment/**` and `equipment_gap_tables.rs`. Findings:

1. `gen_equipment_gap_tables.rs` (the row-selection generator the brief names) is **byte-identical**
   on a clean regen against the pinned oracle — confirmed by running it and diffing (`git status`
   showed zero changes). It is not stale, and its `.COPY=` inheritance logic
   (`collect_base_fields`/`parse_lst`) is correct for this unit: the `.COPY=PSIBLADE` alias row at
   `up_equipmods.lst:161` correctly inherits `cost_gp: Some(0.0)` from the real base declaration at
   line 12, and both rows are correctly assigned *different* keys (`"PSIBLADE"` vs. `"Special Ability
   ~ Psionic Blade ~ Weapon"`), so there is no key collision inside this generator.
2. The real defect is one level up, in a **different generator**:
   `cache_gen::hand_authored_equipment.rs` (`gen_cache_hand_authored_equipment`). Its four per-book
   adapters (`ultimate_psionics_entries`, `ultimate_combat_entries`, `ultimate_intrigue_entries`,
   `ultimate_magic_entries`) called only each book's `equipment_tables()` accessor. But
   `equipment_tables()` **never held any `Equipmods`-category rows to begin with** — those live in a
   wholly separate compiled array behind a distinct `equipmod_tables()` accessor
   (`EQUIPMENT_TABLE` vs. `EQUIPMODS_TABLE`, confirmed by reading
   `rules_tables::ultimate_psionics::equipment_tables::{equipment_tables, equipmod_tables}`
   directly). The adapter's own `.filter(|e| e.category != Equipmods)` was therefore a **no-op**
   whose presence was read (by a prior cycle, and initially by me) as proof the exclusion was live
   and deliberate — it was excluding a population that was never there.
3. **`equipmod_tables()`'s population (113/19/7/0 across UPSI/UC/UI/UM) was never reached by this
   generator at all.** Meanwhile `gen_equipment_gap_tables.rs`'s own `held` skip-set
   (`equipment_resolver::hand_authored_equipment_rows()`) *does* include every `equipmod_tables()`
   key — that function does not filter by category — so it correctly assumed these rows were
   "already hand-authored" and skipped them too. **Two generators, each assuming the other's
   territory covered a population neither actually wrote.**

This is the real, provable shape of the brief's lead — not literally "captures a `.COPY=` alias key
instead of a base declaration" (that specific claim does not hold for `gen_equipment_gap_tables.rs`,
which handles `.COPY=`/base keying correctly for this unit), but the *same symptom* for the same
traced unit: the alias (`PSIBLADE`, key present in neither `held` set) got a corpus record via
`gen_cache_equipment_gap`; the base declaration (key present in the hand-authored `equipmod_tables()`)
got a corpus record from **nobody**.

## Population explained

```python
# cross-check: hand-authored Equipmods-category keys vs. no_record ledger rows,
# per (book, kind); see receipt appendix script below
```
139 hand-authored `Equipmods` rows across UPSI(113)/UC(19)/UI(7) (UM's `equipmod_tables()` is
genuinely empty, confirmed by its own doc comment and by a test). Of those, **132 were `no_record`**
before the fix (7 already had records via some other path — not investigated further, out of scope).
All 132 are `equipment_modifier` kind; **zero are `equipment` kind** — this defect does not touch
`equipment`'s 170.

## Fix

`src/rules_core/cache_gen/hand_authored_equipment.rs`:
1. All four adapters now `.chain(t::equipmod_tables())` after `t::equipment_tables()`. UM's chain is
   a genuine no-op (proved by test), not a special case.
2. `generate()` now routes an `Equipmods`-category row to `equipment/equipmods/` (matching
   `cache_gen::equipment_gap::generate`'s own directory convention), not the `equipment/` root — a
   `Equipmods` row landing at the root would have been the wrong *kind* directory even after being
   written. Counted separately: new `GenerationReport.equipment_modifier_written` field.
3. Module and struct doc comments corrected to state what is actually true (the old "explicitly
   EXCLUDES every Equipmods-category row" framing was itself part of the confusion this cycle
   untangled).

`src/bin/gen_cache_hand_authored_equipment.rs`: prints both counts; the zero-writes fatal check now
requires both to be zero.

## RED → GREEN

1. **Unit-level RED**: added
   `ultimate_psionics_adapter_includes_equipmods_rows_too` (replacing the old, no-op-proving
   `..._excludes_every_equipmods_row`) before touching the adapter functions. Ran
   `cargo test --locked --lib rules_core::cache_gen::hand_authored_equipment` → FAILED on the new
   test's second assertion (`no entry has category == "Equipmods"`), for the intended reason.
   Implemented the `.chain()` fix → GREEN, 5/5 passed.
2. **Integration-level RED (mutation proof, not merely a pre-fix state)**: added
   `an_equipmods_row_lands_under_equipment_equipmods_not_the_equipment_root`, a real (non-mocked)
   fixture test that runs the actual `generate()` against a temp `corpus_root`/`out_root`, then
   **temporarily reverted the routing branch to always write to `equipment_out`** (mutation) → test
   FAILED (`equipmods/ must exist -- the row must have been written there: NotFound`), confirming the
   test catches the real regression. Reverted the mutation → GREEN. Test also proves no-clobber: a
   second `generate()` run over a slug pre-seeded with `"PRE-EXISTING CONTENT"` leaves that content
   untouched and reports it via `skipped_pre_existing`, not `equipment_modifier_written`.
3. Full `rules_core::cache_gen` module: 137 passed, 0 failed, 10 ignored (pre-existing
   `#[ignore]`s, not introduced by this cycle).
4. `tests/equipment_gap_tables.rs` (the sibling generator's own integration suite, to confirm no
   regression from this cycle touching adjacent territory): 7/7 passed.

## Corpus regeneration — additive-only, verified

Ran `gen_cache_hand_authored_equipment` against the pinned oracle
(`PCGEN_CORPUS_ROOT=.../artifacts/corpus/operator-supplied/pcgen/data`). Output: `0 equipment, 139
equipment_modifier records`; 620 already-shipped `equipment`-kind rows correctly reported
`skipped_pre_existing` (a prior run of this same binary, pre-dating this cycle, already wrote those —
confirmed via `git status --porcelain` showing **zero modifications to any existing tracked file**,
only 139 new untracked JSON files under `equipment/equipmods/`, plus the 2 source files this cycle
edited). No `--allow-stamp-loss` flag exists on this binary's path (it is a pure additive one-off
writer using `write_json`'s no-clobber semantics, never a deleting regen), so the corpus-regeneration
report-env-var requirement does not apply to this generator; verified safety directly via
`git status --porcelain` instead (no deletions, no modifications to `data/corpus/**`).

## Gate 3 standing check (not touched, verified still green)

```
python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json
```
`no_record budget: 982/35328 vs. baseline 21521/36028 -- exceeded: False`. `NO_RECORD_BUDGET_COUNT`/
`POPULATION` constants in `scripts/shape_coverage_standing_gate.py` **not modified**, per the
dispatch brief's explicit instruction.

## Closure figures — three separate numbers (`§16`)

- **Closure** (real ingest, new corpus record): 132 `equipment_modifier` units, `no_record` →
  `matched`/`no_formula_tokens`.
- **Reclassification**: none. No unit changed `kind` this cycle.
- **Reachability**: all 139 written records are `wiring_class: computed` per
  `docs/work-inventory.json`'s existing entries (pre-existing engine wiring via the hand-authored
  Rust tables; this cycle only gave them a corpus JSON record, it did not newly wire them).

## Bundle-wide `no_record`, before/after this cycle

| Kind | Before | After |
|---|---:|---:|
| `spell` | 285 | 285 (unstarted this cycle) |
| `monster_ability` | 267 | 267 (not this cycle's scope) |
| `companion` | 217 | 217 (not this cycle's scope) |
| `equipment` | 170 | 170 (this defect does not explain any of it) |
| `equipment_modifier` | 175 | **43** |
| **Bundle total** | **1,114** | **982** |

Re-derive: `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`.

## What is NOT done, named explicitly (no silent narrowing)

- `equipment` (170) — untouched. The cross-generator gap this cycle fixed does not explain any of
  it (cross-checked: 0 of the 132 units this fix closed are `equipment` kind). Needs its own
  root-cause trace; not attempted this cycle (budget spent tracing + fixing + proving the
  `equipment_modifier` gap).
- `equipment_modifier`'s residual 43 — not investigated this cycle. Likely a distinct cause (not the
  hand-authored/gap cross-exclusion this cycle closed, since that population is now fully covered).
- `spell` (285) — **not started this cycle.** The dispatch brief's explicit warning against running
  `gen_cache_spell_lane_dump` (armed self-erasure defect against `spell_mod_access`, a sibling lane
  fixing it concurrently) was honored; no alternative `spell` path was attempted in the time this
  cycle had.
- `monster_ability` (267) and `companion` (217) — explicitly out of scope per the dispatch brief.

## Discoveries

None requiring a `## DISCOVERED` entry — the traced defect was fully explained and fully fixed
within this cycle's own file scope.

- **Discovery forwards:** none.
- **Next-cycle plan:** `equipment` (170, root cause untraced) and `equipment_modifier`'s residual
  (43) are the natural next targets in this scope; `spell` (285) needs either the
  `gen_cache_spell_lane_dump` self-erasure fix to land first, or a different ingestion path per
  `§17`'s "search for the existing path" discipline.
