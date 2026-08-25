# Cycle card-15-integration — census-scope-closure integration (this dispatch)

- **Card ID:** `census-scope-closure` (card 15, `decisions.md §12b`)
- **Note on concurrency:** this dispatch's working-tree changes (the `ce__sizes.lst` exclusion,
  the `class_feature` `CATEGORY:Internal` reroute, `scripts/card15_reconcile.py`, both new tests)
  were found and committed by a concurrent `reclosure-epilogue` cycle running in the same shared
  checkout before this cycle's own commit — `git log` shows HEAD moved from `053d4cb7b` to
  `e47f641b9` mid-session. That commit's own message and receipt
  (`artifacts/epic-5-protective-sweep/closure-epilogue_cycle-2_correction_cycle_receipt.md`)
  correctly attribute and describe the work. This receipt covers only what remained after that:
  the reconciliation re-verification, the fabricated-new-kind gate test's pass confirmation, the
  cross-lane `CATEGORY:Internal` tension finding, and the retro/kanban addenda.
- **Commit SHAs:** `e47f641b9` (concurrent commit carrying this cycle's own code changes),
  this cycle's own commit (kanban.md addendum, this receipt, retro event, sd31-transcribe.jsonl
  append) — see push output below.
- **Files touched by this cycle directly:** `kanban.md` (row 15 addendum), `docs/retro/events/
  card-15-integration.jsonl` (one new correction event), `docs/retro/events/sd31-transcribe.jsonl`
  (harmless `verify.sh --only reach` preflight side-effect append), this receipt.
- **Dual-audit (this cycle's own diff only):**
  ```
  BASE_BRANCH=$(git merge-base HEAD origin/develop)
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- kanban.md docs/retro/events/card-15-integration.jsonl \
    docs/retro/events/sd31-transcribe.jsonl artifacts/gate-0-census-closure/15-integration_cycle_receipt.md \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- kanban.md docs/retro/events/card-15-integration.jsonl \
    docs/retro/events/sd31-transcribe.jsonl artifacts/gate-0-census-closure/15-integration_cycle_receipt.md \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
  ```
  Both `OK_*`.
- **Verification re-run this cycle (live, pasted):**
  - `python3 -m unittest scripts.tests.test_census_independent scripts.tests.test_shape_coverage_standing_gate`
    → 23 tests, OK (13 census + 10 gate, including the new
    `test_fabricated_new_kind_with_uncovered_object_fails_the_gate`)
  - `scripts/verify.sh --only shape-coverage-standing-gate` → PASS
    `population=24914 unclassified=0 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6`
  - `scripts/verify.sh --only shape-coverage-standing-gate-selftest` → PASS (10 cases)
  - `scripts/verify.sh --only reach` → PASS (31 passed)
  - `cargo test --locked --lib` → 2388 passed, 0 failed
  - `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml` → 516 passed, 0 failed
  - `python3 scripts/card15_reconcile.py --output artifacts/gate-0-census-closure/15-reconcile.json`
    → piles reconcile exactly: `still_counted_reroute_plus_already_tracked_plus_pending_a_plus_pending_b == 27838` (`true`), `remaining_undisposed: 0`
  - `grep -rn "18,231\|18231\b\|27,847\|27847\b" tests/ src/ scripts/ apps/` → no pinned assertions
    of the old figures anywhere outside this bundle's own doc prose (already updated); no sweep
    needed.
- **Item-by-item against the dispatch brief:**
  1. **Apply (A) dispositions:** NOT done this cycle for any of the 9,008 identified (A) units
     (class_feature 179, ability 5,108, other-kinds 3,551, skill 170) — see "why not" below.
  2. **Apply (B) dispositions:** DONE for the 2 well-proven, low-risk cases (2,614 `CATEGORY:
     Internal` reroute, 9 `ce__sizes.lst` exclusion), both with tests. NOT done for the 778
     `ability_category` B-gateway/B-picklist/B-duplicate rows (no per-row rule written) — blocked
     partly on budget, partly on the cross-lane tension below making the class_feature lane's own
     blanket-B claim for the *newly-rerouted* population suspect.
  3. **Reconcile with one committed command:** DONE — `scripts/card15_reconcile.py`, re-run above,
     piles reconcile exactly, every population named per `decisions.md §12c`.
  4. **Extend Gate 3's standing gate:** proved (not newly coded — `shape_ledger.classify_unit` was
     already kind-agnostic by construction) via a new test that fabricates an uncovered object
     under a kind ("ability") this gate has never seen, confirms it goes red, and the `mock.patch`
     context manager reverts it automatically — the existing green run after the `with` block IS
     the revert-and-confirm-still-green proof.
  5. **Re-run every gate:** DONE, pasted above, all PASS/green.
  6. **Sweep pinned counts:** DONE — grepped old (18,231/27,847) and new (15,617/27,838) figures
     across `tests/`, `src/`, `scripts/`, `apps/`; no executable pinned assertion found anywhere,
     only this bundle's own prose (already current).
- **Why the 9,008 (A) units were not integrated this cycle:** `src/bin/v06_work_inventory.rs` is
  15,821 lines with kind-specific special-casing woven through `enumerate_file`, `refine_kind`,
  `has_classifying_token`, mod-target resolution, and duplicate-identity handling for every
  existing `Kind`. Adding 8 new `Kind::*` variants (`Ability`, `Skill`, `Template`, `Deity`,
  `Power`, `Domain`, `Language`, `Kit`) safely requires understanding and correctly extending all
  of that machinery for each — verified feasible in principle (`shape_ledger.classify_unit` is
  kind-agnostic, so once units exist in the inventory with correct `book`/`source_file`/
  `source_line`, family classification and Gate 3 coverage follow automatically, no
  `shape_ledger.py` changes needed) but not safely landable, correctly, for 9,008 units across 8
  new kinds within this cycle's remaining budget. Landing it wrong (e.g. double-counting the 5,108
  `ability` units against the 685/839 `ability_category:Internal` rows the sibling lane already
  disposed differently — see the tension below) would be worse than an honest partial.
- **New finding this cycle, NOT applied on trust (`decisions.md §1a`):** the class_feature lane's
  memo §2 disposed all 2,614 rerouted `CATEGORY:Internal` rows as (B) citing the walker's existing
  content-blind exclusion precedent. The `ability_category` lane's own per-row classifier, run
  against the *original* 839-unit `ability_category:Internal` population (not the newly-rerouted
  2,614), found 685/839 (81.6%) carry independent `DEFINE:`/`BONUS:`/etc content and disposed them
  (A). The two lanes' dispositions of the same PCGen marker were never cross-checked against each
  other. This cycle's own bucket-relabel fix (moving `_abilities_class.lst` Internal rows into the
  `ability_category:Internal` bucket) remains correct regardless — it only makes
  `census_independent.py` treat this file kind the same way it already treats every other
  `_abilities_*.lst` file, a consistency fix, not a fresh content judgement — but the
  `ability_category` lane's 5,108-A/778-B split must not be mechanically extended to the newly
  arrived 2,614 rows without resolving this tension first. Logged: `scripts/retro.py correction`
  (`docs/retro/events/card-15-integration.jsonl`, id
  `1787450765953-card-15-integration-951829`).
- **Kanban:** row 15 stays `in-progress` — addendum appended (see kanban.md row 15's final
  sentences) naming this cycle's exact contribution and the exact remaining scope: 9,008 (A) units
  across 4 buckets not yet in `docs/work-inventory.json`, 778 (B) units not yet code-excluded, and
  the CATEGORY:Internal cross-lane tension unresolved.
- **Discoveries forwarded:** the cross-lane tension above (no new card — it is card 15's own
  remaining scope, not a separate finding).
- **Next-cycle plan:** (1) resolve the CATEGORY:Internal tension by re-running the
  ability_category-lane's own classifier against the 2,614 rerouted rows specifically (same
  script, wider input) and deciding A/B per-row rather than blanket; (2) add `Kind::Ability` first
  (largest population, 5,108, already has a committed row-level JSONL seed list from the
  ability_category lane) as a single, fully-tested new-kind cycle, proving the pattern before
  repeating it for `Skill`/`Template`/`Deity`/`Power`/`Domain`/`Language`/`Kit`; (3) apply the 778
  `ability_category` (B) exclusions in `census_independent.py` once (1) is resolved; (4) add the
  179 `class_feature` residual only after independently pinning its root cause (the memo's own
  "pool-membership dedup" hypothesis is unconfirmed) — do not add via a blind rescue list; (5) once
  every bucket is disposed and integrated, re-run `scripts/card15_reconcile.py` and confirm
  `total_kind_unenumerable_units` reaches 0, then set row 15 `complete`.
- **Disk:** `df -h /` → see below.
