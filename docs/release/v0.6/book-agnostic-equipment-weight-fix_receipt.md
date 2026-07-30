# Fix receipt — book-agnostic equipment weight AND cost resolution

- **Finding:** `docs/release/v0.6/book-agnostic-backend-gaps-scoping.md`, Finding 1 (currently
  pending in PR #342, not yet on `develop` — see Note on doc references below)
- **Branch:** `fix/book-agnostic-equipment-weight-and-cost`, forked from `origin/develop` @
  `7cb2f0cf` (after v0.6/PR #343 merged) — **not** `tranche/7`/PR #342, so this lands independently
  of that PR's own review timeline.
- **Status:** built, tested, verified. Superseded an earlier attempt of this same fix
  (`fix/book-agnostic-equipment-weight-local`, forked from `tranche/7` before v0.6 merged) — that
  branch targeted a now-stale snapshot of `encumbrance.rs` and covered weight only, not cost. This
  version is built against `encumbrance.rs`'s real, current `develop` state and covers both.

## Change

`src/rules_core/encumbrance.rs`: `compute_encumbrance` resolved each equipment selection via the
already-book-agnostic `equipment_id_resolve`, then re-looked-up that item's weight *and* cost in
the CRB-only compiled `rules_tables::crb::equipment_tables()` static table — silently dropping
both for any non-Core-Rulebook item. (This is a slightly larger bug than originally scoped: at
scoping time, only weight went through this path; v0.6's own later work on this file, landed as
part of the merged PR #343, added `cost_gp` tracking reusing the same CRB-only lookup, so the bug's
blast radius grew to match.)

Fix: read both `WT:` and `COST:` directly off the already-resolved record's own raw tokens
(`weight_and_cost_from_record`, new helper) instead of the second, CRB-only lookup.
`EquipmentRecord.tokens` carries both for every book's records already, since the LST parser
itself is book-agnostic — only the compiled static table was CRB-only.

Removed now-unused `equipment_key_token`/`equipment_tables` imports. Left `RuleSetId::Crb` as the
`rule_set` argument to `equipment_id_resolve` unchanged — confirmed (same as the original scoping)
it only affects the discarded `_table_cell` citation value, not resolution; fixing that mislabeling
belongs with Finding 2 (`RuleSetId` needs new per-book variants), out of scope here.

## Verification

- **New regression test** (`compute_encumbrance_resolves_weight_and_cost_for_a_non_crb_book_item`):
  real ARG Dogslicer fixture (`COST:8 WT:1`, verbatim from `arg_equip_arms_armor.lst`), corpus
  tagged `advanced_race_guide` not `core_rulebook`. Asserts both weight and cost are counted, not
  just weight.
- `cargo test --lib rules_core::encumbrance` — 6/6 passed (5 pre-existing + 1 new).
- `cargo test --test v06_encumbrance` — 15/15 passed, zero regressions (covers size scaling, load
  penalties, real CRB cost/weight totals — none of which previously exercised a non-CRB item).
- Full workspace suite: `cargo test --workspace --locked --no-fail-fast` (with `PCGEN_REPO_DIR`
  set) — **5,352 passed / 2 failed**. Both failures are pre-existing and environment-path-dependent
  (`/home/ubuntu/workspace/programs/codex/requirements/...` — a different machine's absolute path),
  identical to the baseline every prior receipt this session has documented. Zero regressions.
- **Real end-to-end confirmation carried over from the original (weight-only, tranche/7-based)
  version of this fix**: `cargo test --test sd27_advanced_race_guide_parity -- --nocapture` showed
  `encumbrance.total_carried_weight_lbs` matching PCGen exactly (30=30, was 29≠30) and ARG's overall
  parity result improving from 13/15 to 14/15 dimensions. Not re-run against this exact branch
  (the ARG corpus/parity fixtures only exist on `tranche/7`, pending PR #342, not on `develop`) —
  the underlying resolution logic is unchanged between the two versions, only extended to also
  cover `cost_gp`, so this result is expected to hold once both PRs are on the same branch.
- Manual review of the full diff in lieu of the dual-audit gate scripts (`identifier-discipline-
  audit.sh`, `wired-integration-audit.sh`), which don't exist on `develop` yet — they're vendored
  by SD-27, pending in PR #342. No bundle tags, stub markers, mock leaks, or empty handlers in this
  diff.

## Note on doc references

This fix's doc comments and test doc comment cite
`docs/release/v0.6/book-agnostic-backend-gaps-scoping.md`, which does not exist on `develop` yet —
it's pending in PR #342 alongside the ARG/PU content this fix was discovered fixing. If this PR
merges before #342, that reference will be a dangling path until #342 also lands; not a build
break (it's prose in a doc comment, not a compiled reference), just worth knowing. No action
needed either way — both PRs are independently mergeable in any order.
