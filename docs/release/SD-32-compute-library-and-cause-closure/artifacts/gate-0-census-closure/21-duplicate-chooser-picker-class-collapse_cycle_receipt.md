# Cycle 21-duplicate-chooser-picker-class-collapse — gate-0-census-closure / Criterion: implement decisions.md Decision 21

- **Card ID:** 15 (duplicate-chooser-picker groups, Decision 21)
- **Commit SHA:** (this cycle's commit, see `git log -1`)
- **Files touched:**
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-class-collapse.py` (new)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-class-collapse_test.py` (new)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-collapse-log.json` (new)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-class-collapse-memo.md` (new)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-card15-reconcile-after.json` (new)
  - `scripts/card15_reconcile.py` (bucket reallocation + stale-figure correction)
  - `docs/retro/events/sd31-transcribe.jsonl` (append-only, auto-logged by `scripts/verify.sh --only preflight-oracle`)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** `decisions.md` Decision 21 binding conditions 21b-1 through 21b-5, verbatim
  (predicate exactness, logged collapse, over-reach test, denominator report + reconcile re-run, scope
  discipline).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/verify.sh --only
  preflight-oracle` → PASS; oracle bootstrapped fresh into this worktree's repo-local
  `artifacts/corpus/operator-supplied/pcgen` slot, empty by default).
- **Status:** complete
- **Notes:**
  - **§17a re-derivation:** 39 groups / 113 rows / 74 residual, book split 27/7/2/2/1
    (`advanced_class_guide`/`ultimate_magic`/`advanced_race_guide`/`occult_adventures`/
    `monster_codex`) — reproduces the dispatch brief's figures exactly, zero exceptions.
  - **No `v06_work_inventory.rs` change**: `git diff --stat HEAD -- src/bin/v06_work_inventory.rs`
    is empty. The runtime `(book, key)` collision collapse already drops these 74 residual rows —
    verified by direct id lookup (none of the 74 physical `(book, file, line)` triples corresponds
    to a distinct id anywhere in the committed `docs/work-inventory.json`). This cycle's work is
    proving that disposition correct with a committed, reviewable instrument and reallocating the
    bookkeeping, not changing generation code.
  - **Over-reach proof (binding condition 3), including the literal RED→GREEN mutation:**
    `is_duplicate_chooser_picker_group`'s body was temporarily replaced with `all_type_choice(rows)`
    alone (the adjacency-only rule Decision 17 rejected). Re-running the 5-test suite: 2 failures
    (`test_differing_targets_group_is_NOT_covered_left_alone`,
    `test_loosening_to_adjacency_ignoring_targets_WOULD_over_reach`), captured verbatim in the memo.
    Reverted; re-run: 5/5 pass, `OK`.
  - **`DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` relationship — ruled COMPLEMENTS, not
    supersedes/absorbs.** All 7 `ultimate_magic` groups' surviving row IS on that 33-id list and is
    removed post-construction by `apply_duplicate_chooser_removal`; the OTHER (residual) row of the
    same group never reaches construction at all (this cycle's population). Disjoint populations,
    same underlying game shape — see the memo's own dedicated section for the id-lookup evidence.
    **The constant is left unchanged.**
  - **Denominator change, before/after (binding condition 4):** `docs/work-inventory.json` itself
    is byte-unchanged this cycle (`git diff --stat HEAD -- docs/work-inventory.json` empty — no
    regen was run; not needed, since the 74 rows were already absent). The change is entirely in
    `scripts/card15_reconcile.py`'s bucket bookkeeping: `pending_a` 179→105 (−74),
    `disposed_b.duplicate_chooser_picker_group_class_disposed` 0→74 (+74). `arithmetic_check` total
    (`disposed_b_still_counted + already_tracked + pending_a + pending_b`) is **18,992 both before
    and after** (captured via two full `scripts/card15_reconcile.py --output` runs, diffed) —
    `equals_total_this_run: true`, `remaining_undisposed: 0` both times. Nothing lost beyond the
    named 74.
  - **Also found and corrected while re-deriving (not new scope, a stale-figure fix):**
    `card15_reconcile.py`'s own hardcoded `class_feature_residual_duplicate_identity` (183) and
    `class_feature_already_in_inventory` (18008) were stale — the committed
    `docs/work-inventory.json` already includes the prior review cycle's 4-unit rescue (confirmed by
    direct id lookup: `native_cunning_grapple_overrun`,
    `vigilante_favored_maneuver_bull_rush_favored_maneuver_sunder`,
    `social_grace_craft_armor_craft_baskets`, `green_faith_marshal_panther_domain_vulture` are all
    present), landed by a sibling cycle once `af2f07f68` fixed the `source.path` defect that was
    blocking `corpus_literal_sweep`. Corrected 183→179 and 18008→18012 (−4/+4, matched) before
    applying Decision 21's own −74/+74 on top. **This is the expected 4-unit landing the dispatch
    brief named — reporting it here, not silently absorbing it into this cycle's own 74.**
  - **Scope discipline (binding condition 5):** the predicate applies ONLY to fallback-key (no
    `KEY:`) `class_feature` collision groups. The 16 keyed-collision groups are an untouched,
    disjoint population (12 correctly left uncollapsed, 4 already rescued by the prior cycle) — no
    file in this diff touches keyed collisions.
  - **Regeneration safety:** no corpus regen run this cycle (`docs/work-inventory.json` unchanged) —
    `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT`/stamp-diffing discipline does not
    apply, nothing was regenerated.
  - **Tests:** `python3 .../21-duplicate-chooser-picker-class-collapse_test.py -v` → 5/5 `OK`
    (includes a live re-derivation against the pinned oracle, skipped only if `PCGEN_CORPUS_ROOT`
    unset). `python3 scripts/card15_reconcile.py --output <path>` → `equals_total_this_run: true`,
    `remaining_undisposed: 0`.
  - **§15 Product Identity:** no record disposed this cycle was transcribed, ingested, or scored
    against `ogl-pi-blacklist.md` — identity-collapse bookkeeping and predicate validation only. No
    PI question at this layer.
- **Discovery forwards:** none.
- **Next-cycle plan:** the 12 remaining keyed-collision groups (24 rows) and the 22 fully-traced
  non-colliding rows are the only class_feature residual left in `pending_a` (105 units) — both are
  already fully explained (not an unexplained defect), not this cycle's scope (Decision 21 binding
  condition 5 excludes keyed collisions), and need no further code change; a future cycle could
  formally reallocate the 22 fully-traced rows from `pending_a` to `disposed_b` in
  `card15_reconcile.py` (named but not performed by the prior review-cycle memo, still true here) if
  card 15's own closure bar requires it.
