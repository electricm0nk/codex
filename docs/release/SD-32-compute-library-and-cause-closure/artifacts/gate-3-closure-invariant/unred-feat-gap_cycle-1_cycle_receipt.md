# Cycle unred-feat-gap-1 — un-red `origin/tranche/12` / `tests/feat_gap_tables.rs:181`

- **Card ID:** none (dispatched directly against a red build at `origin/tranche/12` tip; not a
  numbered kanban row — same shape as the sibling `unred-branch`/`unred-powers` cycles this one
  follows).
- **Commit SHA:** (recorded after push, see below)
- **Files touched:** `tests/feat_gap_tables.rs`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** the branch must build and test GREEN at tip; no test may be deleted or
  loosened to pass (`decisions.md §1a`); a stale pin gets re-pinned to the new true state with its
  cause named, per the deferral-revisit shape this bundle has already applied twice
  (`unred-branch`, `unred-powers`).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`) — oracle slot was empty in this fresh worktree, bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`, confirmed on-pin before trusting
  any figure below.
- **Status:** complete
- **Notes:** this is the fourth stale-pin assertion fixed this bundle (`unred-branch`, `unred-powers`,
  `t2b-refine-kind-fix`, this cycle) and the second time a commit's own message claimed a pin was
  updated everywhere when one file was actually missed.
- **Discovery forwards:** none requiring a new card.
- **Next-cycle plan:** none for this specific assertion. Swept for other stale pins touching this
  wave's four sibling-lane deltas (Kind::Ability +4,824, class_feature narrowing +2,593,
  duplicate-identity rescue +24, T9's horror_adventures spell family) — see "Sweep" below.

## Root cause

`origin/tranche/12` tip (`ca82102d8`) was red:

```
cargo test --locked --test feat_gap_tables
FAILED: the_gap_rows_are_exactly_the_joined_catalog_minus_the_hand_authored_one
        (tests/feat_gap_tables.rs:164)
  left: 540
 right: 531
test result: 7 passed; 1 failed
```

`tests/feat_gap_tables.rs` computes `all_feat_tables().len() - hand_authored_feat_tables().len()`
against the **checked-in generated file** `src/rules_core/rules_tables/feat_gap_tables.rs` — no
regeneration happens at test time, so this diff is caused only by that generated file (or the
hand-authored one) changing on disk, not by any environmental/corpus drift.

`git log --oneline -12 -- src/rules_core/rules_tables/feat_gap_tables.rs` names the most recent
touch: `a50b7da04` "feat(sd32): onboard 4 unbuilt books' compiled rule sets (Gate 0 close)"
(AT-32-G0-003). That commit's own message says explicitly:

> Every pinned catalog total/per-book/per-category count that moved with the new records was
> re-derived by running the failing test and reading its own left/right mismatch, then updated —
> **feat gap lane 531->540**, feat catalog 2109->2118, spell catalog 2056->2113, PRE-kind census
> deltas, category-count deltas, all across `feat_catalog.rs`, ...

`git show --stat a50b7da04` confirms the files it actually touched:

```
apps/desktop/src-tauri/src/feat_catalog.rs         |  40 +-
src/bin/gen_feat_gap_tables.rs                     |  13 +
src/rules_core/rules_tables/feat_gap_tables.rs     |  16 +-
tests/v06_apg_acg_feat_catalog.rs                  |   9 +-
```

`tests/feat_gap_tables.rs` is **not** in that list. The commit's own generator addition (a
`BookInput` entry for `RuleSetId::InnerSeaTaverns`, `istav_feats.lst`, "Taverns has no spell file;
its first family is feat via the existing generalised `gen_feat_gap_tables.rs` generator") landed
9 real new gap rows —

```
$ grep -n "inner_sea_taverns —" src/rules_core/rules_tables/feat_gap_tables.rs
648:/// inner_sea_taverns — 9 record(s) the hand-authored `inner_sea_taverns` feat table does not hold.
```

— and 531 + 9 = 540, exactly the observed left/right delta. This is **disposition (a): the catalog
legitimately grew.** The join is correct; the pin in `tests/feat_gap_tables.rs` alone was missed
when `a50b7da04` updated every other pinned total it named.

## Fix

Rewrote the pinned assertion (did **not** delete or loosen it) to pin the new, real total:

```diff
-        added, 531,
-        "the gap lane is 531 rows: 83 from the original 7-book lane \
+        added, 540,
+        "the gap lane is 540 rows: 83 from the original 7-book lane \
          ...
-         feats, which all do and correctly stay dropped), verbatim from \
+         feats, which all do and correctly stay dropped) + 9 from \
+         `inner_sea_taverns` (`a50b7da04`, SD-32 Gate 0 book-onboarding \
+         precondition, AT-32-G0-003 -- Inner Sea Taverns' first compiled \
+         `RuleSetId` of any kind, feat as its first family via this same \
+         generalised generator; that commit's own message claims \
+         "feat gap lane 531->540" was updated everywhere but never touched \
+         this file, leaving this pin stale), verbatim from \
          `gen_feat_gap_tables`'s own stdout at the pinned oracle"
```

The doc-comment breakdown above the assertion now names the ninth addend (`inner_sea_taverns`, 9
rows) and cites the commit that landed it, matching the existing style for every other addend in
that comment.

## RED → GREEN, proven by mutation (not merely re-run)

1. **Baseline (pre-fix) reproduced RED** exactly as the dispatch brief reported:
   ```
   left: 540
   right: 531
   test result: 7 passed; 1 failed
   ```
2. **Fix applied → GREEN:**
   ```
   $ cargo test --locked --test feat_gap_tables
   test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   ```
3. **Mutation** — temporarily reverted the assertion's `540` back to `531` (simulating the
   pre-`a50b7da04` pin) → **RED**, the exact same left/right mismatch reproduced:
   ```
   assertion `left == right` failed: ...
     left: 540
    right: 531
   ```
   Confirmed the assertion still catches the real defect shape, not a vacuously-true rewrite.
4. Re-applied the fix; re-ran `cargo test --locked --test feat_gap_tables` → **GREEN** again
   (8 passed, 0 failed), confirmed via `git diff --stat tests/feat_gap_tables.rs` that only the
   intended 9-line delta stands.

## Sweep for sibling staleness (dispatch brief's explicit ask)

The dispatch brief named four deltas this wave landed: `Kind::Ability` (+4,824), a `class_feature`
narrowing (+2,593), a duplicate-identity rescue (+24), and T9's first spell book
(`horror_adventures`, `d0c36e27b`).

1. **`Kind::Ability` + `class_feature` narrowing + duplicate-identity rescue** — all three are
   Gate 3's `shape_coverage_standing_gate` inputs. `git log --oneline -3` shows they were already
   repinned in this same lineage, one and two commits before this cycle's dispatch tip:
   - `391993eee` "CATEGORY:-based identity for class_feature duplicate_identity fallback
     collisions (card 15)" — the +24 duplicate-identity rescue.
   - `64badfecf` "Gate 3 repin 4 -- Kind::Ability + class_feature narrowing growth (004bbe8c2)" —
     repins `NO_RECORD_BUDGET_COUNT`/`NO_RECORD_BUDGET_POPULATION` 13968/28490 -> 21521/36028 in
     `scripts/shape_coverage_standing_gate.py`, confirmed current:
     ```
     $ grep -n "NO_RECORD_BUDGET_COUNT = \|NO_RECORD_BUDGET_POPULATION = " scripts/shape_coverage_standing_gate.py
     NO_RECORD_BUDGET_COUNT = 21521
     NO_RECORD_BUDGET_POPULATION = 36028
     ```
   - `ca82102d8` (this cycle's own dispatch tip) "Gate 3 repin 4 progress.md entry" — the
     matching progress-doc receipt for the above.
   `grep -rln "28490\|13968\|21521\|36028"` across `tests/`, `src/`, `scripts/`, `apps/`, `docs/`
   turns up only this already-current script plus historical receipts/provenance-log entries
   (`docs/retro/events/`, `artifacts/gate-3-closure-invariant/00[45]_*_cycle_receipt.md`,
   `progress.md`) that are dated snapshots, not live pins — nothing else stale.
2. **`horror_adventures` spell family (T9, `d0c36e27b`)** — searched for other pinned spell-catalog
   totals that could have staled from this landing (`grep -rn "2109\|2118\|2056\|2113"
   tests/ apps/desktop/src-tauri/src/`): the hits are `tests/v06_apg_acg_feat_catalog.rs`,
   `tests/sd27_known_spells_must_be_on_the_class_spell_list.rs`,
   `tests/sd27_feat_prerequisite_enforcement.rs`, and the desktop-crate catalog source files
   themselves — all already repinned by `a50b7da04`'s own diff, ran and confirmed GREEN below.
3. **`feat_gap_tables.rs` itself** — re-checked every other pinned total in the file
   (`REPRESENTATIVES`, `CE_FEATS_LST_POPULATION`) against the current joined catalog; both pass
   unchanged (they pin membership/name-filing facts, not the total count, so `a50b7da04`'s growth
   does not touch them).

**No other stale pin found from this wave's four deltas beyond the one fixed here.**

## Verification

```
$ cargo test --locked --test feat_gap_tables
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

$ cargo test --locked --lib
test result: ok. 2409 passed; 0 failed; 13 ignored; 0 measured; 0 filtered out; finished in 13.77s

$ cargo test --locked --bin v06_work_inventory
test result: ok. 335 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.57s

$ cargo test --locked --test v06_apg_acg_feat_catalog --test sd27_known_spells_must_be_on_the_class_spell_list --test sd27_feat_prerequisite_enforcement
test result: ok. 9 passed  (v06_apg_acg_feat_catalog)
test result: ok. 6 passed  (sd27_known_spells_must_be_on_the_class_spell_list)
test result: ok. 9 passed  (sd27_feat_prerequisite_enforcement)

$ (cd apps/desktop/src-tauri && CARGO_TARGET_DIR=<scratch> cargo test --locked)
test result: ok. 518 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 49.73s

$ scripts/verify.sh --only reach
PASS  reach  (31 passed)
RESULT: PASS
(foreground invocation timed out twice at the tool's 2-minute cap; re-run backgrounded and polled
to completion inside this turn per §2.5, not left waiting across turns)
```

## Other red found (not fixed, out of scope, named per dispatch brief)

`site-dashboard-check` known-failing from unrelated dashboard-JSON staleness (declared in dispatch
brief; not re-derived here to avoid duplicating an already-recorded finding).

## Retro logging

`scripts/retro.py correction` logged (`docs/retro/events/t9-onboarding.jsonl`, id
`1787477443224-t9-onboarding-07b453`): subject the pinned gap-row total in `tests/feat_gap_tables.rs`,
claimed 531, actual 540 — `a50b7da04` (AT-32-G0-003, Inner Sea Taverns' first compiled
`RuleSetId`, feat as its first family) added 9 real `inner_sea_taverns` gap rows and its own commit
message claimed this pin was updated along with every other one it named, but the diff shows it
never touched `tests/feat_gap_tables.rs`. Verified by the cargo test RED→GREEN cycle, mutation-proved,
and `git show --stat a50b7da04`.
