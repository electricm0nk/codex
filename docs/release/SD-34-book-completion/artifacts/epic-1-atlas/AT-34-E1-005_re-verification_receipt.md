# Cycle AT-34-E1-005-R — Epic 1 Completion Atlas / AT-34-E1-005 (re-verification at HEAD)

This cycle was dispatched against `AT-34-E1-005` after the criterion had already landed
(original cycle receipt: `AT-34-E1-005_cycle_receipt.md`, kanban row already `complete`).
Between the original cycle and this dispatch, Epic 2 (eight tables), Epic 3's `AT-34-E3-001`
per-unit work, and the `AT-34-E1-002`/`003`/`004` re-verification cycles all regenerated
`docs/work-inventory.json` and touched `src/bin/v06_work_inventory.rs` extensively. Per
`decisions.md §12` L2, the rename is re-derived fresh at HEAD rather than trusting the
original receipt's numbers.

- **Commit SHA:** see push output below (this receipt's own landing commit).
- **Files touched (this cycle):**
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-005_re-verification_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md` (prepended)
  - `docs/release/SD-34-book-completion/kanban.md` (row re-confirmed with re-verification receipt link, status unchanged — already `complete`)
  - No production code or `docs/work-inventory.json` edits were required — see re-derivation below.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (this cycle's own diff is docs-only)
- **Wired-integration audit result:** `OK_NO_TOKENS` (this cycle's own diff is docs-only)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-34-E1-005 — the `not-ingested` status field is renamed
  >
  > The field asserts the opposite of its meaning: 26,002 of 26,002 of its units (100%) carry a
  > real `source_file` and `source_line`, and every evidence string is engine-side. It has
  > already misled once, during this package's own authoring.
  >
  > **Evidence:** the field renamed to state what it means (e.g. `engine-does-not-hold`) across
  > `src/bin/v06_work_inventory.rs`, `docs/work-inventory.json`, and every consumer, with
  > RED→GREEN. A count sweep across `tests/`, `src/`, `apps/`, `scripts/` for the old string,
  > reported. The atlas's A/B/C/D arms key on this string and are updated in the same cycle.

## Re-derivation at HEAD

Old-string live-use sweep (`tests/`, `src/`, `apps/`, `scripts/`, this repo's whole current
tree, not the population as it stood at the original cycle):

```
$ grep -rl 'not-ingested\|not_ingested' tests/ src/ apps/ scripts/ 2>/dev/null
scripts/tests/test_legacy_not_ingested_string_swept.py
```

The one hit is the sweep test's own source (its docstring/regex necessarily name the retired
spelling to detect it) — the same exclusion the original cycle documented. Confirmed via the
test's own `sweep()` function, which already excludes its own file:

```
$ python3 -c "
import sys; sys.path.insert(0, '.')
from scripts.tests.test_legacy_not_ingested_string_swept import sweep
hits = sweep()
print('legacy_not_ingested_live_uses =', len(hits))
print(hits)"
legacy_not_ingested_live_uses = 0
{}
```

```
$ python3 -m unittest scripts.tests.test_legacy_not_ingested_string_swept -v
test_no_live_uses_remain_under_the_four_scanned_directories ... ok
test_sweep_goes_red_on_a_planted_use_and_green_on_its_revert ... ok
Ran 2 tests in 4.341s
OK
```

`docs/work-inventory.json` still carries zero occurrences of the old spelling; the new spelling's
occurrence count has moved because Epic 2/3 closure work promoted real units past
`engine-does-not-hold` — a reclassification, not a rename defect:

```
$ grep -c '"not-ingested"' docs/work-inventory.json
0
$ grep -c '"engine-does-not-hold"' docs/work-inventory.json
20066
```

(the original cycle measured `26239`; the delta of `6,173` matches Epic 2/3's own closure
receipts moving units out of bucket A/D, not any regression in the rename itself.)

`v06_work_inventory.rs`'s closures still use the renamed identifiers only — zero legacy hits:

```
$ grep -c "not_ingested\b" src/bin/v06_work_inventory.rs
0
$ grep -c "engine_does_not_hold" src/bin/v06_work_inventory.rs
39
```

The atlas's bucket-D citation, which the original cycle updated in the same commit as the
rename, still resolves by content at HEAD (line number unchanged — this citation sits earlier
in the file than the `AT-34-E1-004` promotion-ladder citation that did drift):

```
$ sed -n '9165,9170p' src/bin/v06_work_inventory.rs
    let text_only = unit.magnitude_token_count == 0 && !carries_prose_magnitude;
    let engine_does_not_hold = |evidence: &str| Verdict {
        status: "engine-does-not-hold",
        ...
$ python3 -c "
import sys; sys.path.insert(0,'.')
import scripts.completion_atlas as ca
print(ca.BUCKET_DEFINITIONS['D']['citation'])"
{'file': 'src/bin/v06_work_inventory.rs', 'line': 9167, 'must_contain': 'engine-does-not-hold'}
```

The atlas's own fail-closed citation check confirms this by execution, not just by reading the
constant:

```
$ python3 scripts/completion_atlas.py --check
population=49438 buckets=10 unclassified=0 overlap=0
  DONE: 14584
  A: 449
  B: 11964
  C: 4395
  D: 3053
  M: 5076
  V: 9516
  U: 211
  X: 171
  Z: 19
done_evidence_violations=0
missing_clearing_mechanisms=0
stale_derived_at=False
citation_failures=0
```

`citation_failures=0` — the D-bucket citation (keyed on `engine-does-not-hold`) resolved cleanly
against live HEAD content. `population=49438 unclassified=0 overlap=0` confirms the rename did
not disturb the atlas's total population or leave any unit unclassified.

`missing_engine_tables.py` and `shape_engine_boundary.py` (the two other consumers the original
cycle's handoff note flagged) both key on `engine-does-not-hold` only — confirmed above with
`grep -n "not-ingested\|engine-does-not-hold" scripts/*.py`; no legacy spelling present in
either.

- **Figures + their re-derive commands:**
  - `legacy_not_ingested_live_uses=0` — `python3 -m unittest
    scripts.tests.test_legacy_not_ingested_string_swept -v` (denominator: every `.rs`/`.py`/
    `.ts`/`.tsx`/`.js`/`.json`/`.md`/`.html`/`.env` file under `tests/`, `src/`, `apps/`,
    `scripts/`, excluding the sweep test's own file)
  - `"not-ingested"` occurrences in `docs/work-inventory.json`: `0` —
    `grep -c '"not-ingested"' docs/work-inventory.json` (denominator: the file's full text)
  - `"engine-does-not-hold"` occurrences: `20066` (of `49438` total units) —
    `grep -c '"engine-does-not-hold"' docs/work-inventory.json` — **moved from `26239` at the
    original cycle**; delta attributed to Epic 2/3 closure work, verified by
    `git log --oneline <original-sha>..HEAD -- docs/work-inventory.json` showing Epic 2/3
    regeneration commits between the two measurements, not by this cycle
  - `population=49438 buckets=10 unclassified=0 overlap=0` —
    `python3 scripts/completion_atlas.py --check` (denominator: the full corpus population)
  - `citation_failures=0` — same command (denominator: 10 bucket citations, of which the
    D-bucket one depends on this criterion's rename)
  - Denominator gate on this package: `python3 scripts/denominator_gate.py --check
    'docs/release/SD-34-book-completion/*.md'` → `files_checked=15 violations=0`

- **Row-count command output:**
  ```
  $ python3 -c "
  import sys; sys.path.insert(0, '.')
  from scripts.tests.test_legacy_not_ingested_string_swept import sweep
  hits = sweep()
  print('legacy_not_ingested_live_uses =', len(hits))
  print(hits)"
  legacy_not_ingested_live_uses = 0
  {}
  ```
  Same population as the criterion's own bar and the original cycle's row-count row: every
  scanned file under the four directories, count `0` of that population.

- **Build scope verified:** no production code changed this cycle (re-derivation only), so no
  new build was required to satisfy the rename itself. For the record at this HEAD:
  `cargo test --locked --no-run` — **exit 0** (full workspace, `CARGO_TARGET_DIR=/tmp/cargo-
  sd34-at-34-e1-005`, `CARGO_INCREMENTAL=0`, run at commit `11a15ec7fc` plus this cycle's
  docs-only diff). `apps/desktop/src-tauri` not touched this cycle, not run.
- **Sweep population:** N/A — this cycle adds no corpus records and regenerates none.
- **Oracle pin:** N/A — no figure in this cycle came from the pinned PCGen corpus.
- **Status:** complete
- **Movement, four buckets:**
  - **Closure:** none new — the rename itself closed at the original cycle; this cycle confirms
    it has not regressed.
  - **Reclassification:** the `engine-does-not-hold` occurrence count moved (`26239` →
    `20066`) because Epic 2/3's per-unit work promoted 6,173 units past that status since the
    original cycle — attributed to Epic 2/3's own receipts, not this cycle.
  - **Reachability:** none from this cycle.
  - **Instrument-correction:** none required — unlike `AT-34-E1-004`'s promotion-ladder
    citation, the D-bucket citation this criterion's rename updated sits at a part of
    `src/bin/v06_work_inventory.rs` that did not shift line position across the intervening
    commits, so `citation_failures=0` at HEAD with no edit needed.
- **Notes:**
  - This re-verification found the rename fully intact: zero legacy-spelling live uses, zero
    stale citations, atlas population and unclassified/overlap counts unchanged. No RED step
    was needed because nothing had drifted (contrast `AT-34-E1-004`'s re-verification, which
    found and fixed a genuinely stale line citation).
  - The `epic-breakdown.md` criterion text's own `26,002 of 26,002` figure is the pre-rename,
    launch-time measurement and is not re-stated as a live number here — same posture the
    `AT-34-E1-004` re-verification took toward its own launch-time figure.
- **Next-cycle plan:** none required from this cycle — `AT-34-E1-005` remains closed and
  self-verifying. A later Epic 3/4 cycle that further regenerates `docs/work-inventory.json`
  should re-run `python3 scripts/completion_atlas.py --check` before trusting bucket-D
  citation health; the atlas fails closed and names the exact stale line if one ever appears.
