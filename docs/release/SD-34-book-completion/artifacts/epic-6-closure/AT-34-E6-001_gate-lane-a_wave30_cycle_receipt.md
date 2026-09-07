---
canonical: true
owner: sd34-at-34-e6-001
bundle_id: SD-34
criterion: AT-34-E6-001
date: 2026-09-02
verdict: PASS
---

# Cycle Wave 30, Gate Lane A — Epic 6 Closure / AT-34-E6-001

**`AT-34-E6-001` is this cycle's tracking label, not the final-acceptance scan.** That scan's
own receipt is `AT-34-E6-001_cycle_receipt.md` (verdict FAIL, attempt 1) and this cycle does
**not** write it. This lane's assigned population is the three remaining `root-full` test
failures. The filename follows the established gate-lane convention already in this directory
(`_gate-lane-a_wave24_`, `_wave26_`, …), which the dispatch brief's literal
`AT-34-E6-001_cycle_receipt.md` would have overwritten. **The repo wins over the brief**
(`workflow-instruction.md §12` L19).

- **Commit SHA:** `538aceea3d` (the fix) + this receipt/ledger commit
- **Files touched:** `tests/sd24_wired_integration_audit.rs`,
  `tests/sd27_pathfinder_unchained_cache_shape.rs`, this receipt,
  `docs/release/SD-34-book-completion/progress.md`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** 3 hits, all `placeholder`, all inside
  `tests/sd24_wired_integration_audit.rs` — the new exclusion's own explanatory comment and its
  matcher literal. A test that widens a `placeholder` allowlist necessarily contains the word it
  matches. **No hit in shipping source.** See "Audit detail" below.
- **Acceptance criterion:** verbatim, `epic-breakdown.md ### AT-34-E6-001`: *"Every criterion
  `AT-34-E1-001` … `AT-34-E5-004` is `complete`, and every `kanban.md` card is `complete`. A card
  at `in-progress`, `blocked-escalated`, or `complete`-with-a-deferred-half blocks closure. There
  is no "complete *or* filed under `## Open blockers`"."* — that is the bundle-level bar, which
  this lane does not claim. This lane's own bar is the `root-full` stage: `run_root_full()`
  (`scripts/verify.sh:1294`) exits 0 with every `tests/*.rs` suite executed.

## The correction this cycle exists to make

The dispatch brief carried wave 26's two leads and instructed that both be re-derived before use.
Both were. **One of the two was wrong in the most expensive direction** — following it would have
repinned a live assertion to current output.

### sd27_pathfinder_unchained_cache_shape (2 tests) — NOT a repin

Wave 26 reported `42 -> 38` and `7 -> 3` as "restatements matching the corrected corpus". They are
not. Causal chain, traced to source:

| Step | Commit | What it did |
|---|---|---|
| 1 | `b34bf2b4f0` | `git mv`'d PU's four `+0 ABP (Enhancement to …)` equipmods from the flat `equipment/<slug>.json` layout to the category-nested `equipment/equipmods/<slug>.json` layout |
| 2 | (same cycle's regen) | `gen_book_cache`'s write guard checked only the flat path, so it could not see the relocated files and re-created four flat **duplicates** beside them |
| 3 | `e5fd8dddb1` | deleted those four **stale flat duplicates** — and fixed the guard |

**The four real records were never deleted.** They are on disk at
`data/corpus/pathfinder_unchained/equipment/equipmods/`. What broke is that this test file's
`load_all` did a flat, non-recursive `read_dir` and therefore stopped seeing them.

Re-derive:

```
$ ls data/corpus/pathfinder_unchained/equipment/equipmods/
0_abp_enhancement_to_ammunition.json
0_abp_enhancement_to_armor.json
0_abp_enhancement_to_shield.json
0_abp_enhancement_to_weapon.json
```

So the fix belongs in the **loader**, not the ceilings. `load_all` now recurses. Arithmetic:

- **42** = 38 flat `.json` + 4 nested. Denominator: real `KEY:`-bearing records in
  `pu_equipmods.lst`, the test's own independently re-verified corpus ceiling.
- **7** = 3 flat `+0 Attuned …` + 4 nested `+0 ABP …`. Denominator: `+0` records among those 42,
  one per ladder (4 ABP Enhancement + 3 Attunement).

**Both ceilings are unchanged, at 42 and 7. Nothing was repinned.**

Corroborating detail that made the wrong diagnosis detectable before compiling anything: the third
equipment test, `equipment_cache_covers_all_4_abp_slot_types_and_3_attunement_slot_types`, was
**passing** throughout. Its `attune_count == 18` can only hold if no `+0 Attuned` record was lost,
so the four missing `+0` records had to be the four ABP ones — exactly one per slot type, exactly
the set `b34bf2b4f0` relocated. A genuine corpus deletion would have moved that test too.

Scope of the loader change: `feat/` has no subdirectory and its `17` is unchanged;
`equipment/equipmods` is the only subdirectory under either kind that this file loads
(`find data/corpus/pathfinder_unchained -type d`).

### sd24_wired_integration_audit (1 test) — allowlist widened by name, gate not weakened

One hit survives every existing filter:

```
apps/desktop/src-tauri/src/reach_gate.rs:3192:         outside AT-34-E3-001's scope (ingesting the two placeholder rows for shape-coverage, \
```

Introduced by `170c9219c4` (re-derived with `git log -L 3192,3192:apps/desktop/src-tauri/src/reach_gate.rs`).

Read in context before widening anything. It is a string literal inside the `OPEN_FINDINGS` table,
in the entry recording that Core Rulebook's `Human Ethnicity ~ None` / `~ Unknown` rows
(`cr_abilities_race.lst:157`/`:158`) reach no player. The word describes **PCGen's own corpus
data** — upstream flavor placeholder rows — not anything unfinished in this codebase. That is
bucket E/F's established shape: prose *about* an upstream placeholder.

**Why widening here does not disable the gate.** `OPEN_FINDINGS`' own doc comment states it is
*"pinned in both directions and is not a suppression list"*:
`unsurfaced_families_are_exactly_the_recorded_findings` computes the unsurfaced set from live
behaviour and requires it to **equal** the table. An unreached family that is not listed fails the
gate; a listed family that someone surfaces also fails until its entry is deleted. Each entry
states its remedy. An entry is therefore a *declaration* that something does **not** reach a
player — it cannot make a gap pass, and the reachability gate still counts this family as
unsurfaced. The exclusion added here is to the **stub-marker audit only**, and moves no
reachability number.

**Named exclusion (bucket G), scoped by path AND by the hit's own distinctive phrase:**

```rust
line.starts_with("apps/desktop/src-tauri/src/reach_gate.rs:")
    && line.contains("ingesting the two placeholder rows for shape-coverage")
```

Any *different* `placeholder` hit in that same file still fails, which is the discipline every
prior bucket in this test uses.

**Correction to wave 26, for the record.** Wave 26 characterized this hit as legitimate
`placeholder` **UI** prose. It is not UI text: `reach_gate.rs` renders no UI, and bucket A
(`is_ui_placeholder_text`, which matches `placeholder=` / `placeholder:` / `::placeholder`) neither
does nor should match it. Had the allowlist been widened on that premise — by relaxing bucket A —
the audit would have lost its ability to catch real JSX-adjacent stub markers. It was widened as
reviewed corpus-shape prose instead.

## RED → GREEN evidence

RED first, at `aed7420408`, before any edit — and independently re-derived by re-implementing the
test's six filters in Python against the raw `git grep` output before compiling anything, which
predicted the single `reach_gate.rs:3192` hit exactly.

```
---- equipment_cache_has_all_42_real_pu_equipmods_records ----
assertion `left == right` failed: real, independently re-verified KEY:-bearing pu_equipmods.lst record count
  left: 38
 right: 42
---- equipment_cache_plus_zero_records_have_no_fabricated_plus_value ----
assertion `left == right` failed: 7 real +0 records: 4 ABP Enhancement ladders + 3 Attunement ladders
  left: 3
 right: 7
---- placeholder_findings_are_ui_text_prose_or_the_one_documented_deferral ----
apps/desktop/src-tauri/src/reach_gate.rs:3192:         outside AT-34-E3-001's scope (ingesting the two placeholder rows for shape-coverage, \

test result: FAILED. 4 passed; 1 failed   (sd24)
test result: FAILED. 5 passed; 2 failed   (sd27)
```

Each failed for the intended reason: the two counts short by exactly the four relocated records,
and the audit naming the one line by path and content.

GREEN, at `538aceea3d`:

```
$ cargo test --locked --test sd24_wired_integration_audit --test sd27_pathfinder_unchained_cache_shape --no-fail-fast
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (sd24)
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (sd27)
```

## Audit detail

Base: `BASE_BRANCH=$(git merge-base HEAD origin/develop)` = `ea2b3396f2`.
Scoped paths (this lane's territory): `src/ tests/ docs/governance/`.

- **Over this cycle's own diff (`aed7420408..HEAD`), added lines only:** `OK_NO_BUNDLE_TAGS`. The
  six raw matches over the unfiltered diff are all `diff --git` / `---` / `+++` header lines naming
  the two pre-existing test files; no added content line matches.
- **Token audit over this cycle's own diff:** 3 hits, listed above, all in `tests/`, all the new
  exclusion's own comment and matcher. Self-referential by construction; no shipping-source hit.
- **Over the full branch range (`ea2b3396f2...HEAD`):** 1,171 identifier and 48 token matches.
  Denominator: **all of `tranche/14`'s work by every lane across `src/` + `tests/` +
  `docs/governance/` — 56,509 diff lines** — which is not this cycle's population. Reported for
  honesty, not claimed clean by this lane.

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---|---|---|
| PU equipmods records | 42 | `cargo test --locked --test sd27_pathfinder_unchained_cache_shape` | real `KEY:`-bearing records in `pu_equipmods.lst` |
| ↳ flat | 38 | `ls data/corpus/pathfinder_unchained/equipment/*.json \| wc -l` | of those 42 |
| ↳ nested | 4 | `ls data/corpus/pathfinder_unchained/equipment/equipmods/*.json \| wc -l` | of those 42 |
| PU `+0` records | 7 | same suite | of those 42 |
| PU feat records | 17 | same suite | real distinct records in `pu_feats.lst`; unchanged this cycle |
| `placeholder` grep hits, shipping trees | 220 | `git grep -nE '\bplaceholder\b' -- apps/desktop/ apps/desktop/src-tauri/ src/`, minus the test-path filter | the 3 shipping source trees |
| ↳ unexplained before fix | 1 | the RED run above | of those 220 |
| ↳ unexplained after fix | 0 | the GREEN run above | of those 220 |
| Corpus subdirs under PU `feat`/`equipment` | 1 | `find data/corpus/pathfinder_unchained -type d` | both kinds this file loads |

## Row-count command output

The count is over this lane's assigned population: the three named tests.

Derived mechanically from the `root-full` log, not self-assessed
(`decisions.md §4`). Script: `/tmp/cargo-sd34-at-34-e6-001/rowcount.sh` — for each test, `PASS`
iff the log carries the exact line `test <name> ... ok`.

```
$ bash rowcount.sh
PASS placeholder_findings_are_ui_text_prose_or_the_one_documented_deferral
PASS equipment_cache_has_all_42_real_pu_equipmods_records
PASS equipment_cache_plus_zero_records_have_no_fabricated_plus_value
rows=3 PASS=3 FAIL=0
```

**3 of 3.** Denominator: the three tests named in this lane's dispatch brief, which is this lane's
whole assigned population. Status `complete` follows from this count.

## Build scope verified

Run at `1fd5244c79` — the last commit in this cycle that can move any figure an
assertion depends on (`decisions.md §12` L7).

**Re-checked after a rebase.** Two other lanes' commits (`a893bfcb39`, `ff5f19e05e`) landed on
`tranche/14` while this verification was in flight and now sit between `1fd5244c79` and this
receipt. They do **not** invalidate the figures, and that is verified rather than assumed:

```
$ git diff --name-only 1fd5244c79 HEAD -- src/ tests/ apps/ data/ Cargo.lock Cargo.toml
(no output — 0 files)
```

Their whole touch set is `scripts/` and `docs/`. No Rust source, no test, no corpus record and no
lockfile changed, so nothing the root or desktop suites compile or read moved. This cycle's own
later commits are ledger-only.

| Scope | Command | Result |
|---|---|---|
| `--no-run` | `cargo test --locked --no-run` | **EXIT=0** |
| root workspace | `cargo test --locked --no-fail-fast` | **EXIT=0** — 590 result lines, **8,372 passed / 0 failed**, `test result: FAILED` count = 0 |
| ↳ suite coverage | `ls tests/*.rs \| wc -l` vs `grep -c '^     Running tests/'` | **543 of 543** `tests/*.rs` suites executed, 0 never ran |
| desktop crate | `cd apps/desktop/src-tauri && cargo test --locked --no-fail-fast` | **EXIT=0** — **572 passed / 0 failed** |

Totals were derived twice by independent implementations (`awk` and a Python `re` pass) and agreed
exactly, per `AGENTS.md` "Derive counts with `awk`, not `grep -o`".

**Attribution, re-derived rather than inherited (`decisions.md §12` L14).** SD-34's registered
inherited baseline was **29 of 599** suites carrying **46 of 8,034** failures. The root workspace
now carries **0 failures across 590 result lines**. That is not a measurement anomaly and it is not
this lane's claim to have fixed 46 things: waves 24–29 closed the bulk, wave 29's independent sweep
recorded `root-full`'s remaining failing set as exactly **2 suites / 3 tests**
(`sd24_wired_integration_audit`, `sd27_pathfinder_unchained_cache_shape`), and those 3 are
precisely the 3 this cycle closed. The two figures corroborate: 3 remaining − 3 closed = 0.

**The `root-full` stage is therefore green.** The desktop crate was tested explicitly, in its own
separate cargo workspace, with its own `CARGO_TARGET_DIR`.

**Sweeps NOT run this cycle, and named as such:** the full `scripts/verify.sh` (40 stages), the
frontend stages (`frontend-install` / `frontend-test` / `frontend-typecheck`), `clippy`,
`corpus-sweep`, `site-dashboard-check` and `denominator-gate`. The last two are wave 29's other
two live FAILs and sit in other lanes' territory.

- **Sweep population:** N/A — this cycle changed **no** `data/corpus/**` record. Both files touched
  are under `tests/`. `corpus_literal_sweep`'s examined-population is unmoved by construction and
  the record delta is 0, so `decisions.md §12` L8 is satisfied vacuously — stated as such rather
  than reported as a pass.
- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus. Every count
  above is derived from this repo's generated cache or from `git grep` over this repo.
- **Status:** complete
- **Movement, four buckets:**
  - **closure:** 3 tests RED → GREEN (`sd24_wired_integration_audit` 1,
    `sd27_pathfinder_unchained_cache_shape` 2). This lane's entire assigned population.
  - **reclassification:** 0.
  - **reachability:** 0. Explicitly: the bucket-G exclusion touches the stub-marker audit only and
    moves no reachability number; `OPEN_FINDINGS` still counts `crb`/`race_traits` as unsurfaced.
  - **instrument-correction:** 2. (a) `load_all` in `sd27_…_cache_shape.rs` was blind to the
    category-nested layout `gen_book_cache` legitimately writes — a defect in the *instrument*,
    which had been misread as a corpus change. (b) Wave 26's characterization of the
    `reach_gate.rs` hit as UI text is corrected to reviewed corpus-shape prose.
- **Notes:**
  - The single judgment call is bucket G. It is a widening, and a widening is how gates die. It is
    justified above on the mechanism (`OPEN_FINDINGS` is pinned in both directions), scoped by path
    **and** content, and the alternative — relaxing bucket A to cover a non-UI file — is named and
    rejected.
  - `docs/work-inventory.json` and `completion-atlas.json` were not read, written, or regenerated;
    `completion_atlas.py --check` was not run, so there is no timestamp side effect to restore.
  - `kanban.md` not touched: no board row tracks an individual gate-remediation wave, matching
    waves 23/25/26/27/28/29's own precedent. Row 26 remains as it was.
- **Next-cycle plan:** `root-full` should now be green. The two remaining `verify.sh` FAILs from
  wave 29's sweep (`site-dashboard-check`, `denominator-gate`) are named to other lanes' territory
  and untouched here. Once all three are green, the final-acceptance scan
  (`AT-34-E6-001_cycle_receipt.md`, currently FAIL at attempt 1 on five non-`complete` kanban
  cards) can be re-attempted — noting that its shortfall was never the test suite.
