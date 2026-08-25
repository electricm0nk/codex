---
canonical: true
owner: card15/21-duplicate-chooser-picker-class-collapse
status: complete -- implements decisions.md Decision 21, re-derived clean, zero exceptions
date: 2026-08-23
---

# Decision 21 implementation — duplicate-chooser-picker groups, ruled as a class

**Decision:** `decisions.md` Decision 21 (operator ruling 2026-08-23), answering the escalation in
`15-card-15-duplicate-identity-review-memo.md`. Rule in force:

> Every fallback-key `class_feature` collision group whose members **all** carry a `TYPE:*Choice`
> facet **and** whose granted targets pairwise coincide is a duplicate-chooser-picker group, not
> distinct objects.

**Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/verify.sh --only
preflight-oracle` → PASS; oracle bootstrapped fresh into this worktree's own repo-local
`artifacts/corpus/operator-supplied/pcgen` slot, empty by default in a fresh worktree).

## §17a re-derivation — the 39/113/74 figures reproduce exactly

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-class-collapse.py \
  --output docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-collapse-log.json
```

```
groups covered (duplicate-chooser-picker, collapsed): 39
residual rows removed from the unit ledger: 74
groups NOT covered (left alone): 0
```

Book split (`advanced_class_guide` 27, `ultimate_magic` 7, `advanced_race_guide` 2,
`occult_adventures` 2, `monster_codex` 1 = 39): matches the dispatch brief's expected figures
exactly. **Zero groups failed the predicate** — the full per-group evidence (every member's
`CATEGORY:`/`TYPE:`/`ABILITY:AUTOMATIC` grant targets, book/file/line, survivor and every residual
row removed) is in the committed
`21-duplicate-chooser-picker-collapse-log.json` alongside this memo — binding condition 2's
"named, never silent" requirement.

## The predicate, exactly as implemented (binding condition 1)

`21-duplicate-chooser-picker-class-collapse.py`'s `is_duplicate_chooser_picker_group`:

```python
def is_duplicate_chooser_picker_group(rows):
    return all_type_choice(rows) and targets_pairwise_coincide(rows)
```

Both halves, every member — `all_type_choice` requires every row's `TYPE:` facet end in `"Choice"`;
`targets_pairwise_coincide` requires every row's real `ABILITY:...|AUTOMATIC|...` grant-target set to
be shared by at least one *other* row in the group (a row with a unique or empty target set fails the
group). **One refinement over the raw review worksheet** (`15-card-15-residual-group-review.py`,
reused for row collection, not superseded): a `TYPE=<pool> ~ ...` self-tag segment some
`ABILITY:AUTOMATIC` fields also carry (naming the row's own `CHOOSE:` pool, not a second granted
feature) is excluded from the target set — without this exclusion the worked example below would NOT
signal a pairwise match, because the self-tag is row-unique and inflates the apparent target count.
Documented in the module's own doc comment ("The `TYPE=` exclusion").

**Worked example** (`advanced_class_guide` "Aberrant Bloodline", 4 rows, 2 real targets in pairs):

```
acg_abilities_class.lst:156   TYPE:SorcererBloodlineChoice    grants: {Sorcerer Bloodline ~ Aberrant}
acg_abilities_class.lst:2412  TYPE:SorcererBloodlineChoice    grants: {Sorcerer Bloodline ~ Aberrant}    <- SAME as :156
acg_abilities_class.lst:566   TYPE:BloodragerBloodlineChoice  grants: {Bloodrager Bloodline ~ Aberrant, Aberrant Bloodrager Bloodline ~ Feat Tracker}
acg_abilities_class.lst:2754  TYPE:BloodragerBloodlineChoice  grants: {Bloodrager Bloodline ~ Aberrant, Aberrant Bloodrager Bloodline ~ Feat Tracker}  <- SAME as :566
```
Survivor kept: `:156` (first in file-iteration order, matching
`disambiguate_class_feature_fallback_collisions`'s own unmodified tie-break). Residual removed:
`:566`, `:2412`, `:2754`.

## No `v06_work_inventory.rs` change required — the runtime behaviour is already correct

`disambiguate_class_feature_fallback_collisions` already leaves every `TYPE:*Choice`-typed fallback
group's keys untouched (its own "The `*Choice` exclusion" doc comment). Untouched keys keep
competing for the same bare-key identity, so the corpus-wide `(book, key)` collision collapse this
repo already performs (independent of this predicate) already keeps exactly one survivor per group
and silently drops the rest — precisely the disposition Decision 21 calls for. This cycle's own diff
to `src/bin/v06_work_inventory.rs`: **zero lines** — verified: `git diff --stat HEAD --
src/bin/v06_work_inventory.rs` is empty. The 74 residual rows named in the collapse log were **already**
absent from `docs/work-inventory.json` before this cycle (confirmed: none of their 74 `(book, file,
line)` triples corresponds to a distinct id in the committed inventory — every survivor's id is
present, no residual sibling id exists anywhere).

This cycle's actual work is: (1) proving the predicate holds for all 39 groups with a committed,
reviewable, re-runnable instrument (this memo + the collapse script + the collapse log), satisfying
binding condition 2; (2) proving the predicate cannot over-reach (binding condition 3, next section);
(3) reallocating the 74 residual rows from `pending_a` ("identified, awaiting integration") to
`disposed_b` ("proven not an object") in `scripts/card15_reconcile.py`'s own bookkeeping, since they
are now disposed by class rather than merely named for a ruling.

## Binding condition 3 — the over-reach proof

`21-duplicate-chooser-picker-class-collapse_test.py`, 5 tests:

- `test_real_worked_example_shape_is_covered` / `test_five_row_single_target_shape_is_covered` —
  the two real group shapes (4-row-in-pairs, 5-row-single-target) are correctly covered.
- **`test_differing_targets_group_is_NOT_covered_left_alone`** — the over-reach proof itself: two
  synthetic rows, both genuinely `TYPE:*Choice`-typed, colliding on the same `(book, key)`, but each
  grants a **different** target (no partner). The real predicate returns `False` — left alone, not
  collapsed.
- **`test_loosening_to_adjacency_ignoring_targets_WOULD_over_reach`** — runs the SAME
  differing-targets fixture through `all_type_choice` alone (the "loosen to adjacency" mutation
  Decision 21 §21c and Decision 17 both name) and asserts it wrongly returns `True`, then asserts the
  real predicate disagrees. This is a standing, permanent proof — it stays red if the production
  predicate ever regresses to the adjacency-only shortcut.
- `test_all_39_real_groups_covered_zero_exceptions` — re-derives the real corpus and asserts the
  39/74/0 figures above.

**The literal RED → GREEN mutation, performed this cycle:** `is_duplicate_chooser_picker_group`'s
body was temporarily replaced with `all_type_choice(rows)` alone (the rejected adjacency rule).
Re-running the test suite:

```
FAIL: test_differing_targets_group_is_NOT_covered_left_alone
    AssertionError: True is not false : predicate over-reached: a group with two DIFFERENT
    grant targets was flagged as a duplicate-chooser-picker group
FAIL: test_loosening_to_adjacency_ignoring_targets_WOULD_over_reach
    AssertionError: True is not false
Ran 5 tests in 0.001s
FAILED (failures=2, skipped=1)
```

The mutation was then reverted (`git diff` against this commit's own
`21-duplicate-chooser-picker-class-collapse.py` shows the real `and targets_pairwise_coincide(rows)`
predicate, never the mutated one); re-running: `Ran 5 tests in 1.9s — OK` (5/5, including the live
39-group re-derivation). **This demonstrates the new rule is strictly narrower than the rule
Decision 17 rejected** — exactly what binding condition 3 and `decisions.md §21c` require.

## Relationship to `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` — COMPLEMENTS, not supersedes or absorbs

The dispatch brief asks this to be decided and stated explicitly, using `ultimate_magic`'s 7 groups
as the test case. Traced by direct id lookup against the current committed `docs/work-inventory.json`:

- None of the 7 `ultimate_magic:class_feature:*_bloodline` ids (`accursed_bloodline`,
  `djinni_bloodline`, `efreeti_bloodline`, `maestro_bloodline`, `marid_bloodline`,
  `rakshasa_bloodline`, `shaitan_bloodline`) is present in the committed inventory — all 7 are on
  `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` and are removed post-construction by
  `apply_duplicate_chooser_removal`.
- That means, for these 7 groups specifically, **both mechanisms fire on the SAME pair, on opposite
  halves**: the collision-collapse mechanism this cycle documents silently drops the group's OTHER
  (residual) row before it is ever constructed as a unit (e.g. `um_abilities_class.lst:2070`,
  `Accursed Bloodline`'s Crossblooded-archetype picker — never reaches `docs/work-inventory.json` at
  all), while the pre-existing 33-id list removes the SURVIVOR row that DOES reach construction
  (`um_abilities_class.lst:566`, the id `ultimate_magic:class_feature:accursed_bloodline`) after the
  fact. Neither mechanism is redundant with the other: they operate on two **disjoint** populations —
  rows that never become inventory units (collision losers, this cycle's 74) vs. rows that DO become
  inventory units but are still confirmed duplicate pickers (the pre-existing 33, `apply_duplicate_
  chooser_removal`'s own population).
- **Ruling: COMPLEMENTS.** The class predicate does not make `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`
  redundant (it still does real, necessary work removing constructed survivor units the collision
  collapse cannot reach) and is not absorbed into it (the id list only ever names units that reach
  construction; this cycle's 74 residual rows never do, so they have no id to add to that list in the
  first place — the brief's own memo already noted these residual rows are "not yet addressable by
  id"). **`DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` is left unchanged, deliberately** — no id is added,
  none removed. This memo is the record of why that is correct rather than an oversight.

## The denominator change — before/after, sum the piles (binding condition 4)

No `docs/work-inventory.json` change (already established above — this cycle changes no
`(book, file, line)` triple's presence in the final ledger, since the residual rows were already
absent). The change is entirely in `scripts/card15_reconcile.py`'s own **bookkeeping bucket**:

| | before | after | delta |
|---|---:|---:|---:|
| `pending_a.class_feature_residual_duplicate_identity.units` | 179 | 105 | **−74** |
| `disposed_b_applied.duplicate_chooser_picker_group_class_disposed.units` | 0 (did not exist) | 74 | **+74** |
| `arithmetic_check` total (`accounted_total` vs. `census_unenumerable_total`) | reconciles | reconciles | **0** (moved bucket, not lost) |

Full command and output: `scripts/card15_reconcile.py`'s own receipt below (this cycle's own
`21-card15-reconcile-after.json`, committed alongside this memo). `equals_total_this_run: true`,
`remaining_undisposed: 0`, both before and after — the 74-unit move nets to zero, proving nothing was
lost beyond the 39 named collapses.

**Also corrected this cycle, found while re-deriving:** `scripts/card15_reconcile.py`'s own hardcoded
`class_feature_residual_duplicate_identity.units` (183) and `class_feature_already_in_inventory.units`
(18008) were stale relative to the CURRENT committed `docs/work-inventory.json` — a fresh
`15-card-15-class-feature-residual-cause-pin.py` run (§17a) reproduces **153 non-internal + 26
internal-collision-losers = 179** residual (not 183) and confirms the review cycle's 4-unit rescue
(`native_cunning_grapple_overrun`, `vigilante_favored_maneuver_bull_rush_favored_maneuver_sunder`,
`social_grace_craft_armor_craft_baskets`, `green_faith_marshal_panther_domain_vulture`) is **already
present** in the committed inventory (id lookup, all 4 found) — landed by a sibling cycle after the
`source.path` defect blocking `corpus_literal_sweep` was fixed (`af2f07f68`), exactly as the dispatch
brief said to expect and report rather than silently absorb. `class_feature_already_in_inventory` is
corrected 18008 → 18012 (+4) to match; `class_feature_residual_duplicate_identity` corrected
183 → 179 (−4, the same 4 units moving buckets) as part of the same edit that then applies this
cycle's own further −74/+74 move.

## Scope discipline (binding condition 5)

This predicate is applied ONLY to fallback-key (no `KEY:`) `class_feature` collision groups. It is
NOT applied to the 16 keyed-collision groups (a disjoint population, already reviewed and disposed
per-case by the prior cycle — 4 rescued, 12 left alone, per
`15-card-15-duplicate-identity-review-memo.md`), and it is not generalised to any other `Kind`. No
other file in this cycle's diff touches keyed collisions or any other kind.

## Regeneration safety

No corpus regeneration was run this cycle — `docs/work-inventory.json` is unchanged (verified: `git
diff --stat HEAD -- docs/work-inventory.json` empty), so the `CORPUS_LITERAL_SWEEP_REPORT`/
`DERIVED_FIXTURE_CHECK_REPORT`/stamp-diffing discipline does not apply here (nothing was regenerated).
This is a deliberate choice, not an oversight: the residual rows this ruling disposes were already
absent from the ledger before this cycle, so no regen is needed to "apply" the collapse — only the
bookkeeping (`scripts/card15_reconcile.py`) needed correcting.

## Product Identity (§15)

No record disposed this cycle was transcribed, ingested, or scored against `ogl-pi-blacklist.md` —
identity-collapse bookkeeping and predicate validation only, same as every prior card-15 cycle. No
PI-screening question arises at this layer.

## Files touched

- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-class-collapse.py`
  (new) — the Decision 21 predicate + collapse-log generator, reusing
  `15-card-15-residual-group-review.py`'s row collection.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-class-collapse_test.py`
  (new) — 5 tests, including the over-reach proof and its RED→GREEN mutation.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-collapse-log.json`
  (new) — the committed, per-group, per-row evidence log (binding condition 2).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-class-collapse-memo.md`
  (new, this file).
- `scripts/card15_reconcile.py` — bucket reallocation (79-line note update: −4/+4 stale-figure
  correction, then −74/+74 Decision 21 move) and a new `disposed_b_applied` entry.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-card15-reconcile-after.json`
  (new) — this cycle's own re-run of `scripts/card15_reconcile.py --output`, the after-state receipt.
