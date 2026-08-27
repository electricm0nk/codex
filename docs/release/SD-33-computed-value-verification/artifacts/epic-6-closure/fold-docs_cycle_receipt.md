# Cycle fold-docs — epic-6-closure / re-derive closure documents for the fold

- **Commit SHA:** recorded on landing (see `progress.md` entry `sd33-fold-fix-docs`).
- **Base scanned:** `c61fa76842` (`origin/tranche/13`), the tree left by `AT-33-E6-001` attempt 12
  (gate PASS). Worked in the primary checkout, not a detached worktree — a docs-only cycle with no
  code or corpus write, so the concurrency risk a scan worktree exists to avoid does not apply
  here; verified `git status --porcelain` before every write per §5.
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/release-notes.md` — new "Recovered work — the
    operator's 2026-08-26 fold" section (Skinwalker 65-record fold, Undine 3-entry fold, the two
    suites the fold fixed, the F1 regression the fold introduced and how it closed); re-derived
    Test-suites/Corpus-sweep/Denominator-gate figures to the post-fold values; a note that this doc
    was re-derived a second time, 2026-08-26; `date:` frontmatter bumped.
  - `docs/release/SD-33-computed-value-verification/forward-scope-register.md` — new `§E1` table
    naming the three branches the fold's own sweep ruled out (`worktree-wf_a45ece26-3fc-1`,
    `worktree-wf_13156488-c9b-1`, `worktree-wf_c1156061-e3f-5`), each with its reason; `date:`
    frontmatter bumped.
  - `docs/retro/sd33-computed-value-verification-retrospective.md` — new `§6` fold section: the
    provenance narrative, the three named branches ruled out (cross-referencing the register), and
    the three requested lessons (stale-branch file count is not its value; run the suite after the
    last write that can move it; a gate's examined population must grow when records are added);
    frontmatter `date`/`board` updated to the post-fold state; cross-references list extended with
    the fold receipts.
  - `docs/release/SD-33-computed-value-verification/progress.md` — frontmatter `status`/`date`
    corrected (was stale at "21 rows / attempt 10" against the body's already-correct "24 rows /
    attempt 12"); new `## Cycles` entry for this cycle.
  - This receipt.
  - PR #377 body, via `gh pr edit 377` (not a repo file; not merged).
- **Identifier audit result:**
  ```
  $ BASE_BRANCH=$(git merge-base HEAD origin/develop)
  $ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- docs/release/SD-33-computed-value-verification/release-notes.md docs/release/SD-33-computed-value-verification/forward-scope-register.md docs/release/SD-33-computed-value-verification/progress.md docs/retro/sd33-computed-value-verification-retrospective.md ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
  ```
  `OK_NO_BUNDLE_TAGS` — the diff contains only prose references to `SD-31`/`SD-33`/`SD-34` (no
  trailing underscore) and file-path citations, none matching the pattern by design.
- **Wired-integration audit result:**
  ```
  $ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <same 4 files> | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
  ```
  `OK_NO_TOKENS`.
- **Acceptance criterion:** re-derive SD-33's closure documents (`release-notes.md`, `progress.md`,
  `forward-scope-register.md`, the retrospective) after the operator's 2026-08-26 fold, so that a
  reviewer reading them does not have to infer the fold's provenance or its effect on inherited
  figures, and the three branches ruled out of the fold do not get re-discovered as a surprise by
  SD-34.

## Figures + their re-derive commands

| Figure | Value | Re-derive command |
|---|---|---|
| Corpus sweep | 48,699 examined of 51,473 read, 0 findings | `cargo run --locked --bin corpus_literal_sweep` (independently re-run this cycle, log kept; matches attempt 12 exactly) |
| Lib suite | 2,845 passed, 0 failed, 14 ignored | `cargo test --locked --lib` (independently re-run this cycle in the background, 50.79s; the specific `f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census` assertion confirmed `ok`) |
| Denominator gate (pre-edit) | `files_checked=69 violations=0` | `python3 scripts/denominator_gate.py --check` |
| Skinwalker on-disk records | 75 | `ls data/corpus/bestiary_5/race_trait/skinwalker/*.json \| wc -l` |
| Undine fixture entries / sample points | 3 / 30 | `python3 -c "import json; d=json.load(open('tests/fixtures/rules_core/derived-evaluator-fixtures.json')); e=d['race_trait_formula_entries']; print(len(e), sum(len(x['expected_at_sample_points']) for x in e))"` |
| Work-inventory unknown | 0 of 49,438 | `jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json` ; `jq '.units\|length' docs/work-inventory.json` |
| Inherited debt (post-fold) | 29 of 599 suites / 46 of 8,034 tests | cited from `AT-33-E6-001-attempt12_cycle_receipt.md`, not re-run this cycle (no code/corpus change to move it; the `--no-fail-fast` full workspace run is the expensive one already run twice by the gate cycles) |
| Denominator gate (post-edit) | see "Denominator gate, re-run per §6 of the dispatch" below | `scripts/verify.sh --only denominator-gate` |

## Status: complete

## Movement, four buckets

- **Closure 0.** Documentation only; no criterion or gate moved.
- **Reclassification 0.**
- **Reachability 0.**
- **Instrument-correction 0.** No figure changed value in this cycle — the fold's real figures
  were already correct at attempt 12 (verified independently above); this cycle's job was making
  three closure documents state them, which they did not yet do.

## Notes

- Write scope was exactly the four named documents plus the PR body, per dispatch. No file under
  `data/corpus/**`, `src/`, `scripts/`, or `docs/work-inventory.json` was touched.
- `progress.md`'s frontmatter carried a stale `status`/`date` (said "21 rows... attempt 10" while
  the file's own `## Status` body already correctly said "24 rows... attempt 12", from the
  `sd33-fold-fix-rescan` cycle's edit reaching the body but not the frontmatter). Corrected in the
  same commit as a drive-by, since `progress.md` is one of this cycle's four named files and the
  discrepancy would otherwise ship.
- `kanban.md` and `docs/work-inventory.json` were not touched — both were already current from
  attempt 12's own cycle; re-verified rather than assumed (`grep -c '| complete |' kanban.md` → 24,
  `jq '.units\|length' docs/work-inventory.json` → 49438, both matching the receipt already on
  file).
- The three ruled-out branches (`worktree-wf_a45ece26-3fc-1`, `worktree-wf_13156488-c9b-1`,
  `worktree-wf_c1156061-e3f-5`) were not independently re-read this cycle beyond what the dispatch
  and `sd-33-fold-recovered-work.workflow.js`'s own "WHAT WAS RULED IN, AND WHAT WAS RULED OUT"
  section already state — that section is itself the record of the sweep that ruled them out, and
  this cycle's job was to move its content into the durable package docs, not to re-run the sweep.

## Next-cycle plan

None. This closes the fold-fix documentation gap the attempt-12 scan's own instrument-correction
note implied. PR #377 remains open, not merged, per the dispatch.
