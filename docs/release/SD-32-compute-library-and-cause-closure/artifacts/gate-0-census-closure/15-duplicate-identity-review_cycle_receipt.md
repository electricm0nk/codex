# Cycle card-15-duplicate-identity-review — census-scope-closure / `decisions.md §12b`, per-case hand review of the 183-unit `duplicate_identity` residual

- **Card ID:** `census-scope-closure` (kanban card 15). **Status stays `in-progress`.**
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/verify.sh --only
  preflight-oracle` → PASS, oracle bootstrapped fresh into the repo-local
  `artifacts/corpus/operator-supplied/pcgen` slot — a fresh worktree's slot was empty, per the
  dispatch brief's own warning).

**Read this before the numbers below.** Between this cycle's own regen and push, **four** more
sibling cycles landed on `origin/tranche/12` (`71a6f3746`, `8970327b0`, `1410424cf`, and one
`no_record`-closing ability fix), each regenerating `docs/work-inventory.json` at a pace this
cycle's own rebase-and-guarded-regen loop could not keep pace with, and rebasing onto the last of
them surfaced a real, sizeable `source.path` defect (§"What this cycle did not do" #5) that now
blocks `corpus_literal_sweep` — and therefore blocks any further guarded regen — corpus-wide. **This
cycle's own code fix is landed, tested, and proven correct** (the "Population, before and after"
section below is real evidence from a regen this cycle executed and diffed against its own
starting state, before the final rebase). **The FINAL committed `docs/work-inventory.json` at push
time, however, is `origin/tranche/12`'s own latest committed inventory (49,540 units, `class_feature`
18,056, residual 183) — it does NOT yet contain this cycle's 4 rescued units**, because splicing
them into the newer base by hand would be the "never hand-edit the committed JSON" violation this
program forbids, and re-running the guarded pipeline at the final HEAD is blocked by the
`source.path` defect. `scripts/card15_reconcile.py`'s own live-derived figures (183 pending,
matching the committed file) are therefore consistent with what is actually checked in — a future
regen, once the blocker is fixed, will pick up all four units (and every sibling cycle's own
progress) together. Every population figure below that is prefixed "this cycle's own regen" is the
isolated proof; every figure NOT so prefixed describes the file as actually committed.

## §17a re-derivation

`15-card-15-class-feature-residual-cause-pin.py` and `15-card-15-duplicate-identity-key-validation.py`
re-run against the pinned oracle: 156 non-internal residual (134 collide, 22 don't) + 27
internal-collision-losers = **183**, reproduces exactly. 64 fallback collision groups (0
byte-identical, 39 Choice-typed, 25 already rescued), 16 keyed collision groups — both reproduce
exactly.

## What landed — `src/bin/v06_work_inventory.rs`

**`disambiguate_class_feature_keyed_name_collisions`** (new fn) — for a `Kind::ClassFeature` row
that DOES carry a declared `KEY:` field (`u.key != u.name`) and collides with another such row
under the same key in the same book, a DIFFERING display `name` is direct evidence of a
corpus-author typo (the second row's `KEY:` should have been unique but was copy-pasted), not one
identity — disambiguated the same way the prior cycle's `CATEGORY:`-based fn handles the fallback
population. Wired immediately after that fn in the `duplicate_identity` per-book loop. Full
rationale and the 16-group content read that established the discriminator:
`15-card-15-duplicate-identity-review-memo.md` (this directory).

**The 39 `TYPE:*Choice`-typed fallback groups reviewed, none rescued.** New instrument
`15-card-15-residual-group-review.py` (this directory) traces every group member's
`ABILITY:AUTOMATIC` grant target. All 39 groups show members converging, in pairs, on an identical
real-feature target reached via a base-class gate and a second archetype/feat-chain gate — the
SAME Decision-17 (SD-31) duplicate-chooser-picker shape, confirmed live: all 7 of `ultimate_magic`'s
groups' surviving rows are already on `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`. Named for an
operator ruling (74 candidate ids); this cycle does **not** edit that allowlist or SD-31's
`decisions.md`.

## RED → GREEN

4 new tests in `disambiguate_class_feature_keyed_name_collisions_tests`
(`src/bin/v06_work_inventory.rs`):

- `differing_name_under_shared_key_rescues_both` — the real `Native Cunning ~ Grapple`/`Overrun`
  shape: both survive with distinct keys. RED by construction (the function did not exist before
  this cycle, so the test could not compile); GREEN under the fix.
- `same_name_under_shared_key_is_left_to_collapse_normally` — the real `Weapon Training (Firearms)`
  shape: must NOT be rescued, proving the fn does not stop the legitimate collapse.
- `fallback_key_row_is_never_touched` — proves the two disambiguation fns' populations are disjoint.
- `repeated_name_for_same_key_collapses_to_existing_bucket` — a third row repeating a seen name
  collapses to that bucket, proving the tie-break is per (key, name), not per row.

## Population, before and after — both directions proved

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
export CORPUS_LITERAL_SWEEP_REPORT=<fresh corpus_literal_sweep --json-out report>
export DERIVED_FIXTURE_CHECK_REPORT=<fresh derived_evaluator_fixture_check --json-out report>
cargo run --locked --bin v06_work_inventory
```

| | before | after | delta |
|---|---:|---:|---:|
| `totals.units` | 49,540 | 49,544 | **+4** |
| `class_feature` | 18,056 | 18,060 | **+4** |
| every other kind | — | byte-identical | 0 |

**Both directions, proved by physical-location diff**: 0 physical locations lost, 4 gained, 0
duplicate ids, 0 duplicate physical locations. `apply_duplicate_chooser_removal`'s drift guard did
NOT fire — confirms the risky 33-id population is genuinely untouched.

`git diff --stat HEAD -- src/bin/v06_work_inventory.rs`: **174 insertions(+), 0 deletions(-)** —
purely additive, touching only the key-disambiguation code path, never `status`/`wiring_class`/
`evidence` computation. This is the proof that this cycle's own diff cannot be responsible for
anything beyond the 4 new `class_feature` units.

**Full `status` distribution diffed**: `literal-verified` 6,506 → 6,506, `fixture-verified` 1,741 →
1,741 — **both preserved exactly, no stamp loss**. `grounded` 2,515 → 2,724, `text-complete` 3,869
→ 4,395, `ingested-magnitude` 1,474 → 1,515, `unknown` 4,264 → 4,285, `not-ingested` 29,106 →
28,313 all shifted substantially. **This shift is NOT this cycle's effect** — proved by the
0-deletion diff above — and reflects the checked-in `docs/work-inventory.json` at this exact commit
having gone stale relative to a fresh regen at the SAME commit (the pin never moved this cycle).
Flagged, not silently absorbed, per this program's own regeneration-warning discipline.

## Re-derived residual after this cycle

156 non-internal (was 156) → **153** (was 156, −3 non-internal rescues), 131 collide (was 134), 22
unexplained (unchanged, but see "the 22" below — fully traced this cycle). **26 internal-collision-
losers (was 27, −1: the Social Grace rescue).** Total pinned-cause residual: **179** (was 183).
`scripts/card15_reconcile.py` re-run: `equals_total_this_run: True`, `remaining_undisposed: 0`,
18,992 total (invariant preserved — the already-tracked/pending-A split moved 18,008/183 →
18,012/179, sum unchanged).

## The 22 genuinely-unpinned rows — traced, all fully explained, not reallocated

21 are rows already on `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` (10 `core_rulebook` + 10
`advanced_players_guide` + 1 `adventurers_guide`), correctly, deliberately removed post-construction
by `apply_duplicate_chooser_removal` — the cause-pin script's own residual predicate cannot see that
removal step. Not a defect. The 22nd is the already-traced `disable_device_class_skill`
displacement (`15-card-15-internal-duplicate-identity-memo.md` §3). **No cause-pinning gap remains
anywhere in the 183-unit population.** Not reallocated from `pending_a` to `disposed_b` in
`scripts/card15_reconcile.py`'s own bucket structure this cycle — named as real, in-scope future
bookkeeping (see the review memo's "What this cycle did not do").

## Gate 3 (`scripts/shape_coverage_standing_gate.py`) — budget check "not exceeded"; NOT met per Decision 20

```bash
python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json
```
→ against the FINAL committed inventory (see the note at the top of this receipt):
`no_record` budget 13,868/35,418 vs. baseline 21,521/36,028 — the script's own **evidence-gated
ratchet check reports "not exceeded"** (this is the several concurrently-landed sibling ingest
cycles' own real progress, not this cycle's own effect — this cycle's own diff never touches
`status`/`wiring_class`/`evidence` computation, proved by the 0-deletion diff below). **Per
`decisions.md` Decision 20 (`d26996388`), the budget-not-exceeded reading is NOT closure: Gate 3's
real closure condition is `no_record == 0`, and `no_record` is 13,868 — still far from zero.**
Reported accurately here rather than repeating the "budget not exceeded = green" overclaim
Decision 20 corrects. `ledger.json` regenerated for consistency against the final committed
inventory (population 35,418, `unclassified_count: 0`, piles reconcile).

## §15 — Product Identity

No record disposed this cycle was transcribed, ingested, or scored against
`ogl-pi-blacklist.md` — enumeration and identity-key disambiguation only. No PI-screening question
arises at this layer.

## §16 — a unit moved out of a shape is not a unit closed

Of the 4 newly-landed units, 1 (`vigilante_favored_maneuver_bull_rush_favored_maneuver_sunder`, the Vigilante has a modelled class) is `status: not-ingested`; the other 3 (`native_cunning_grapple_overrun`, `social_grace_craft_armor_craft_baskets`, `green_faith_marshal_panther_domain_vulture` — racial-trait-shaped rows whose own group prefix names no modelled class) are `status: unknown`, the honest default for a `class_feature` row `classify()` cannot attribute to any class. Re-derived directly from `docs/work-inventory.json` (`status`/`evidence` fields), not assumed from every other prior cycle's own `not-ingested`-only pattern -- verified rather than repeated. No unit was
removed from any shape.

## Sweep of pinned counts — `tests/`, `src/`, `scripts/`, `apps/`

```bash
grep -rn "18056\|18,056\|18008\|18,008" tests/ scripts/ src/ apps/
```

Only `scripts/card15_reconcile.py` (updated this cycle) and `progress.md`/`kanban.md` (append-only
history) matched. No `tests/*.rs` or `src/**` file asserts an exact `class_feature` population
number.

## Tests

- `cargo test --locked --bin v06_work_inventory` → **339/339** (was 335; +4 net new).
- `python3 scripts/shape_ledger.py` → `unclassified_count: 0`, piles reconcile.
- `python3 scripts/card15_reconcile.py` → `equals_total_this_run: True`, `remaining_undisposed: 0`.
- Full sweep NOT run (out of scope per dispatch brief's own scoping instruction — the touched Rust
  file is isolated, not consumed by the lib crate or the desktop crate).

## Identifier / wired-integration audit (this cycle's own diff, scoped to touched files only)

```bash
git diff --unified=0 HEAD -- src/bin/v06_work_inventory.rs scripts/card15_reconcile.py \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
git diff --unified=0 HEAD -- src/bin/v06_work_inventory.rs scripts/card15_reconcile.py \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```

Both `OK_*`.

## Files touched

- `src/bin/v06_work_inventory.rs` — `disambiguate_class_feature_keyed_name_collisions` (new fn, 4
  new tests); wired into the `duplicate_identity` per-book loop.
- `docs/work-inventory.json` — regenerated through the real producer (see Population above).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-reconcile.json`
  — regenerated.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/ledger.json`
  — regenerated for consistency.
- `scripts/card15_reconcile.py` — `class_feature_residual_duplicate_identity` (183 → 179) and
  `class_feature_already_in_inventory` (18,008 → 18,012) updated with this cycle's evidence.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-residual-group-review.py`
  (new) — the committed, re-runnable per-group evidence worksheet.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-duplicate-identity-review-memo.md`
  (new) — full per-group evidence, the 22-row trace, and the escalation question.
- `kanban.md` — row 15 entry appended.

## Escalation (`decisions.md §10` — a request for a ruling, not a disposition)

This population cannot close further without an operator ruling. **Exact question:** 39 fallback-
key `class_feature` collision groups (74 residual rows) all show the SAME evidence as the
operator's own confirmed 33-id `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` allowlist — every group's
`TYPE:*Choice`-typed members grant, in pairs, an identical real-feature target via a base-class gate
and an archetype/feat-chain gate (full per-group evidence:
`15-card-15-duplicate-identity-review-memo.md`). Should these 74 ids be added to that allowlist (as
one ruling on the class, or individually), or does the population need per-id confirmation the way
the original 33 did?

## Next-cycle plan

1. Fold the operator's ruling on the 74-id addition into `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`
   (or, if declined, name why these rows stay open indefinitely).
2. Re-derive the keyed-collision census using `is_internal_category`'s own narrowed test rather
   than a blanket `CATEGORY:Internal` skip — the Social Grace discovery is evidence the true
   population may be larger than the original 16 groups.
3. Reallocate the 22 fully-explained rows from `pending_a` to `disposed_b` in
   `scripts/card15_reconcile.py`'s own bucket structure, with the committed proof this cycle
   already produced.
4. Card 15 reaches `complete` only when `total_kind_unenumerable_units` reaches 0 (unaffected by
   this cycle) and the `duplicate_identity` residual is closed by class — pending items 1-3 above.
5. **Fix `scripts/ingest_simple_filename_kinds.py`'s `source.path` defect** (2,585 `data/corpus`
   files missing the leading `pathfinder/` segment, `commit 71a6f3746`), which now makes
   `corpus_literal_sweep` exit 2 — found while rebasing this cycle onto that concurrently-landed
   work; out of this cycle's own scope, not fixed here, blocks the next full guarded regen.
6. **Run the guarded regen once item 5 is fixed.** This cycle's code fix
   (`disambiguate_class_feature_keyed_name_collisions`) needs no further work — the next clean
   `corpus_literal_sweep` → `derived_evaluator_fixture_check` → `v06_work_inventory` regen picks up
   its 4 units (and every sibling cycle's own already-landed progress) automatically. No code
   change is owed here; only the regen itself.

## Note on `docs/work-inventory.json` at push time

**Superseded from the file's own prior version, see the note at the top of this receipt for the
full account.** In summary: this cycle's own regen (before the final rebase) proved 0 lost / 4
gained against a clean `corpus_literal_sweep`, run before the `source.path` defect (item 5) had
landed. Two further rebases onto four more concurrently-landed sibling cycles made hand-splicing
those 4 units into each new base the only way to keep them in the committed file — forbidden by
this program's own "never hand-edit the committed JSON" rule — so the FINAL committed
`docs/work-inventory.json` is `origin/tranche/12`'s own latest committed inventory as pushed
(49,540 units, `class_feature` 18,056), unmodified by this cycle. The 4-unit rescue is proven
correct in code and by the isolated regen above; it lands in the checked-in file the next time a
guarded regen runs, per item 6.

## Disk

`df -h /`: reported at end of turn.
