# AT-33-E3-001 — coverage-gap root cause

**Derived by execution, 2026-08-24**, against `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (pinned, not consulted for this criterion — no oracle byte is read here, this is a population/join question, not a value question).

## 1. The symptom, restated as a question

`README.md §4` row G: **6,854** of **11,652** F1..F9 formula-bearing units had
never been run through `formula_interpreter`'s corpus-wide scan — a bare
41.2% (`4,798 / 11,652`). **41% is a symptom, not a cause.** This document
traces *why*, per family, by mechanism — not by re-running anything blind.

## 2. Two independent staleness layers, each execution-verified

### Layer 1 — the committed Gate 2 RUN ARTIFACT predates 9 later Gate 1
### CENSUS fixes inside SD-32 itself, and was never regenerated

`docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/formula_interpreter.corpus-wide.json`
(`total_population: 4,798`) was committed in a single commit,
**`25dbee17aa`** ("Gate 2 corpus-wide run for formula_interpreter.rs
(F1..F9, card 8)"). Re-derive: `git log --oneline --follow -- docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/formula_interpreter.corpus-wide.json`.

`docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/ledger.json`
(the Gate 1 census this run's population is fixture-checked against) received
**9 further commits after `25dbee17aa`**, each changing the F1..F9
population it counts — kind-aware join fixes, `no_record` closures,
duplicate-identity rescues, a Gate 3 repin. Re-derive:
`git log --oneline --graph 25dbee17aa..80329736f4 -- docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/ledger.json`
(9 commits: `f70cc7d941`, `57780b5bc4`, `5ed69f29fb`, `45012f6a9f`,
`391993eee6`, `004bbe8c25`, `438990b5ad`, `2508c05ed4`, `11a84bced5`,
`662ae1b63d`, `80329736f4` — the full chain).

**Concrete proof, not inference**: this module's own Rust scan logic is
**unchanged** since `25dbee17aa` in every way relevant to population walking
(git history: `25dbee17aa`, then only `f70cc7d941` — a family-vocabulary
docs reconciliation, no population-affecting code change). Re-running that
same, unmodified scan logic **today** against the frozen `ledger.json` (still
sitting at its post-SD-32-closure value) produces `total_population: 11,338`
— not `4,798`. The code was always capable of walking the full census it was
given; **the 4,798 committed artifact reflects a census snapshot from before
9 subsequent fixes grew it inside the same bundle, and the run was simply
never repeated after they landed.** This single layer accounts for
`11,338 − 4,798 = 6,540` of the `6,854`-unit gap — **95.4%** of it
(`6,540 / 6,854`, re-derive: the two population figures above, subtracted).

### Layer 2 — the frozen Gate 1 census is itself stale against the CURRENT
### corpus/inventory (314 more units exist today)

The remaining `11,652 − 11,338 = 314` units (**4.6%** of the gap) are new
since the Gate 1 ledger froze on 2026-08-14: `docs/work-inventory.json` and
`data/corpus` have grown (later book-onboarding/kind-fix cycles, outside
SD-32/SD-33's own scope). Re-derive: `python3 scripts/shape_ledger.py
--inventory docs/work-inventory.json` reports `matched 11,671` / family
rollup `F1 6,308 ... F9 62` (sums to 11,652 in F1..F9), against the frozen
file's `11,338`.

**Concrete sample coordinates** (360 gross `(id, family)` rows present in a
fresh regeneration but absent from the frozen `ledger.json`; net 314 after
some rows' family also shifted — re-derive:
`/tmp/.../diff_new_rows.py` diffing both ledgers' `rows` by `(id, family)`):

| id | family | book | kind |
|---|---|---|---|
| `advanced_class_guide:class_feature:arcanist` | F1 | advanced_class_guide | class_feature |
| `advanced_class_guide:class_feature:bloodrager` | F1 | advanced_class_guide | class_feature |
| `advanced_class_guide:class_feature:bloodrager_bloodline_tracker` | F2 | advanced_class_guide | class_feature |
| `advanced_class_guide:class_feature:brawler_brawler_s_flurry` | F5 | advanced_class_guide | class_feature |
| `advanced_class_guide:class_feature:daring_champion_champion_s_finesse` | F3 | advanced_class_guide | class_feature |

## 3. Why the gap is uneven by family — both layers are population-uniform,
## so the unevenness comes from each family's ORIGINAL share of the 4,798

Neither staleness layer targets a family; both are whole-census snapshots
taken at a point in time. The per-family unevenness the criterion names
(`F1 28%, F8 21%, F2 64%`) falls out mechanically from how large each
family's slice of the **original, small, 2026-08-1x-era snapshot** happened
to be relative to its **current true size** — families that grew fastest
between the snapshot and today show the largest gap.

| Family | Old run pop. (SD-32 artifact) | Stale census pop. (frozen ledger.json) | True pop. (fresh, 2026-08-24) | Old-run coverage (old / true) |
|---|---:|---:|---:|---:|
| F1 | 1,790 | 6,032 | 6,308 | 28.4% |
| F2 | 1,490 | 2,337 | 2,337 | 63.8% |
| F3 | 303 | 650 | 671 | 45.2% |
| F4 | 570 | 1,073 | 1,086 | 52.5% |
| F5 | 361 | 586 | 589 | 61.3% |
| F6 | 211 | 391 | 391 | 54.0% |
| F7 | 5 | 12 | 12 | 41.7% |
| F8 | 41 | 200 | 196 | 20.9% |
| F9 | 27 | 57 | 62 | 43.5% |
| **Total** | **4,798** | **11,338** | **11,652** | **41.2%** |

Re-derive the "old run" and "true pop." columns:
```
python3 - <<'PY'
import json
old = json.load(open("docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/formula_interpreter.corpus-wide.json"))
new = json.load(open("docs/release/SD-33-computed-value-verification/artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json"))
for fam in [f"F{i}" for i in range(1,10)]:
    print(fam, old["families"][fam]["population"], new["families"][fam]["population"])
PY
```
`F1 28.4%` and `F8 20.9%` and `F2 63.8%` match the epic-breakdown's stated
`F1 28%, F8 21%, F2 64%` exactly — the criterion's own figures are confirmed
by this trace, not merely repeated.

## 4. The fix (AT-33-E3-002/003/004)

`src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` no longer
reads SD-32's frozen `ledger.json` at all. It now regenerates the Gate 1
census **fresh, at scan time**, by invoking `scripts/shape_ledger.py`
against the CURRENT `docs/work-inventory.json` / `data/corpus` state (never
re-implementing the classifier in Rust — `decisions.md §4`'s single-source-
of-truth rule for the PF1e family vocabulary). This closes **both** layers
at once: population is always the CURRENT true population, and the run
itself always walks 100% of what that population names, because
`scan_ledger_rows` counts every row it is handed.

Fresh corpus-wide run, 2026-08-24 (this cycle's own regeneration):
`total_population: 11,652` — **0 gap** against the true F1..F9 population
(`README.md §4` row G comparison: `11,652 (E) − 11,652 (new F) = 0`).

## 5. Two SEPARATE, honestly-named findings this fix does NOT close (and is
## not asked to)

Per the epic-breakdown NOTE, **recognition rate is a separate number from
coverage**. Closing the population gap does not, and should not, silently
convert refusal or join failure into success:

- **Refused: 240 of 11,652 (2.1%)**. A unit whose formula text was found and
  fed to `recognises_shape`, which declined it for a stated reason —
  overwhelmingly `unrecognised function "var"/"cl"/"count"` (grammar gaps
  named, not silently guessed at). Concrete examples, still refused in the
  fresh run: `advanced_class_guide:class_feature:arcanist`
  (`var("CL=Arcanist")`), `advanced_class_guide:class_feature:bloodrager`
  (`var("CL=Bloodrager")`) — both new F1 rows from Layer 2 above,
  demonstrating that closing the population gap surfaces new, real, named
  refusals rather than hiding them.
- **Unjoined: 786 of 11,652 (6.7%)**. This module's own re-derivation of
  formula text (primary `(book, kind, source_file, source_line)` join
  against `data/corpus` only) is narrower than `scripts/shape_ledger.py`'s
  three-way join (`build_corpus_index` primary + `build_corpus_key_index`
  same-book-key fallback + `build_cross_book_key_index` cross-book-key
  fallback, `scripts/shape_ledger.py` lines 391-660). A unit the census
  correctly places in F1..F9 (because shape_ledger's fuller join found its
  formula text) can still land "unjoined" in this module's report if only
  its own narrower primary join misses. This is walked (counted in
  population, per family) but not actually fed to the grammar — a real,
  separate, named gap, **not silently folded into "recognised."** Closing it
  needs porting `shape_ledger.py`'s fallback-join fully into this module (or
  invoking it per-unit), which is out of AT-33-E3-002/003/004's evidence bar
  (population parity, stated and met above) and is named here as forward
  scope rather than absorbed silently.

Neither bucket is a coverage failure and neither is a silent exclusion —
both are visible, counted fields on every `CorpusWideReport`
(`total_refused_units`, `total_unjoined_units`), printed by the binary, and
present in the committed artifact at
`docs/release/SD-33-computed-value-verification/artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json`.
