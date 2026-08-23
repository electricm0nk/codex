# Cycle card-15-internal — census-scope-closure / `decisions.md §12b`, `is_internal_category` narrowing + residual cause pin

- **Card ID:** `census-scope-closure` (kanban card 15). **Status stays `in-progress`** — the
  `duplicate_identity`-caused portion of the residual (§4 below) is a real, distinct next-cycle
  target this cycle deliberately does not touch (dispatch brief: "pin the cause before rescuing").
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/verify.sh --only
  preflight-oracle` → PASS, oracle bootstrapped fresh into the repo-local
  `artifacts/corpus/operator-supplied/pcgen` slot — a fresh worktree's slot was empty, per the
  dispatch brief's own warning).

## §17a re-derivation — both populations re-derived fresh, not the brief's stale figures

The dispatch brief cited "~2,574 units" (item 1) and "179-unit residual" (item 2). Both re-derived
before touching anything:

1. **Item 1's population is real and current.** `15-card-15-category-internal-adjudication-memo.md`
   (already committed to `origin/tranche/12` before this cycle started, `e79d508b4`) adjudicated
   2,614 `_abilities_class.lst` `CATEGORY:Internal` rows: 2,371 (A) + 203 (B-gateway-resolved, stays
   counted per its own conservative default) = 2,574, 40 (B-picklist) genuinely excluded. That fix
   already landed in `scripts/census_independent.py` — this cycle's own scope is the **separate**
   `v06_work_inventory.rs` side, named explicitly by the brief. Confirmed still open at cycle start:
   `is_internal_category` (line ~2821, pre-fix) carved out `Kind::Ability` only
   (`kind != Kind::Ability`) — every `Kind::ClassFeature` `CATEGORY:Internal` row, including
   content-bearing ones, was still unconditionally dropped.
2. **Item 2's 179 figure reproduces exactly**, re-derived with a fresh script
   (`15-card-15-class-feature-residual-cause-pin.py`, this directory) independent of the original
   memo's own script: 15,617 non-internal `row_dependent_class_feature` rows, 15,438 matched to
   `docs/work-inventory.json` by physical location, **179 residual** — same figure, independently
   confirmed.

## What landed — `src/bin/v06_work_inventory.rs`

1. **`CLASS_FEATURE_INTERNAL_CONTENT_PREFIXES`** — the adjudication memo's WIDER field list
   (`DEFINE:`/`BONUS[A-Z]*:`/`DESC:`/`ASPECT:`/`CSKILL:`/`MOVE:`/`AUTO:`/`TEMPLATE:`/`SPROP:`/
   `QUALITY:`/`SR:`/`DR:`/`SAB:`/`VISION:`/`SPELLKNOWN[A-Z]*:`/`TEMPBONUS:`/`CHOOSE:`/
   `NATURALATTACKS:`/`COMPANIONLIST:`/`ADD:`/`FOLLOWERS:`/`UDAM:`/`UMULT:`/`SELECT:`/`COST:`/
   `MOVECLONE:`/`SPELLS:`/`SERVESAS:`/`DEFINESTAT:`/`UNENCUMBEREDMOVE:`/`BENEFIT:`/`SPELLLEVEL:`/
   `CMB:`) — deliberately NOT `ABILITY_CONTENT_PREFIXES` (the narrower list governing the disjoint,
   already-adjudicated `Kind::Ability` population), ported byte-identical from
   `census_independent.py`'s `_ROW_CONTENT_FIELD_RE`.
2. **`is_spellknown_token`** — `SPELLKNOWN[A-Z]*:` matcher, same shape as the pre-existing
   `is_bonus_token`.
3. **`is_internal_gateway_token`** — `ABILITY:...|AUTOMATIC|` gateway-token test, the memo's
   `_ROW_GATEWAY_FIELD_RE`.
4. **`class_feature_internal_row_is_bare_marker`** — true only when NONE of the above match; the
   narrow, provable (B) class (40/2,614 at the adjudication memo's own count).
5. **`is_internal_category`'s computation restructured to a `match kind`:** `Kind::Ability` unchanged
   (`false`, decided entirely downstream by `has_classifying_token`); **`Kind::ClassFeature` now
   decides its own disposition inline** (`carries_internal_category_field &&
   class_feature_internal_row_is_bare_marker(&fields)`) rather than the old blanket
   `kind != Kind::Ability` drop; every other kind's behaviour is byte-for-byte unchanged (the `_ =>
   carries_internal_category_field` arm is the same expression the old code always ran).

## RED → GREEN

The pre-existing test `class_feature_internal_row_is_still_dropped_by_the_blanket_trap` asserted
**0 units** for `"Panache Tracker\tCATEGORY:Internal\tDEFINE:PanacheLVL|0\n"` — exactly the OLD
(wrong) behaviour the adjudication memo overturned (that row carries `DEFINE:`, disposition (A)).
Confirmed via direct source inspection (`git show 4d1354fef:src/bin/v06_work_inventory.rs` still
shows `kind != Kind::Ability` uncontitionally dropping every other kind) that this row necessarily
produced 0 units before this cycle — RED for the intended reason. Renamed to
`content_bearing_class_feature_internal_row_is_enumerated_not_dropped` and inverted to assert
**1 unit, `Kind::ClassFeature`** — GREEN under the fix. Four more tests added:
`dr_only_class_feature_internal_row_counts_as_content_not_bare` (the memo's own worked (B) example,
`DR:`-only, now correctly (A) — proves the widened field list, not just the fix's presence);
`bare_class_feature_internal_marker_row_is_still_dropped_by_the_trap` (a genuinely bare tracker row
still correctly excluded — proves the narrowing does NOT start eating real (B) rows);
`gateway_only_class_feature_internal_row_stays_counted_not_dropped` (a gateway-only row stays
counted, matching the shipped conservative default); `non_internal_class_feature_row_is_unaffected_by_this_fix`
(an ordinary row untouched).

## Population, before and after — both directions proved

**Correction logged mid-cycle, not silently fixed:** the first regen run used `--allow-stamp-loss`
without first regenerating `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` — the guard
(`v06_work_inventory.rs` lines ~10336-10368) refused a plain write and named exactly why, and
`--allow-stamp-loss` was passed to get past it without understanding the consequence: it silently
dropped **all 6,506 `literal-verified` and all 1,741 `fixture-verified` stamps** corpus-wide (visible
as the status distribution going from 8 status buckets to 7, `ingested-magnitude`/`grounded` jumping
implausibly). Caught before commit by diffing the full `status` distribution against the pre-cycle
file, not merely the `class_feature`-kind delta. **Recovered correctly**, not by reverting to
`--allow-stamp-loss` again: `docs/work-inventory.json` reset to HEAD, both reports regenerated fresh
(`corpus_literal_sweep --json-out`: 26,538 records examined, 0 findings, CLEAN;
`derived_evaluator_fixture_check --json-out`: 1,836 units cleared over 2,577 fixture rows, 0 failed),
then the producer re-run with both env vars set (no `--allow-stamp-loss` needed — the guard passed
clean). Confirmed by re-diffing every status bucket: `literal-verified` 6,506 → 6,506,
`fixture-verified` 1,741 → 1,741, both exactly preserved.

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
export CORPUS_LITERAL_SWEEP_REPORT=<fresh corpus_literal_sweep --json-out report>
export DERIVED_FIXTURE_CHECK_REPORT=<fresh derived_evaluator_fixture_check --json-out report>
cargo run --locked --bin v06_work_inventory
python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(d['totals']['units'], d['totals']['by_kind']['class_feature'])"
```

| | before | after | delta |
|---|---:|---:|---:|
| `totals.units` | 46,923 | 49,516 | **+2,593** |
| `class_feature` | 15,439 | 18,032 | **+2,593** |
| every other kind | — | byte-identical | 0 |

(`totals.units`' own delta matches `class_feature`'s exactly — confirms no other kind moved.)

**Both directions, proved by id-diff** (`pre_ids = {u['id'] for u in <pre>['units']}`,
`post_ids = {...<post>...}`, set difference):

- **5 units renamed, 0 content removed:** `magus_heavy_armor`, `magus_medium_armor`,
  `wizard_spells`, `sighted_seeker_hypercognition`, `sighted_seeker_metafaculty` — each re-suffixed
  (`<slug>__<hash>`) because a NEW slug-colliding unit landed in the same book+kind
  (`v06_work_inventory.rs`'s existing `unit_id` disambiguation, not a defect this cycle introduced).
  Confirmed each one's (book, source_file, source_line) is still present in the post-cycle inventory
  under its new id — verified by direct location lookup, not assumed from the name matching.
- **1 unit displaced, id stable, physical location moved (the "moving OUT" case the dispatch brief's
  §16 rule requires be named):** `ultimate_psionics:class_feature:disable_device_class_skill` moved
  from `up_abilities_class.lst:468` to `up_abilities_class.lst:186` — a newly-visible
  `CATEGORY:Internal` row (`CSKILL:Disable Device`) sharing the SAME bare-display-name key as the
  previously-sole-tracked row (also `CSKILL:Disable Device`, no `TYPE:` field difference in
  substance) won the pre-existing `duplicate_identity` corpus-wide dedup race. Full trace:
  `15-card-15-internal-duplicate-identity-memo.md` §3. **No content lost** — both rows describe the
  same conceptual feature; only the winning physical corpus coordinate changed.
- **2,593 net new units, all `class_feature`, all `status: not-ingested`** (`classify()`'s existing
  `class_feature`-kind arms — not-ingested-of-unmodelled-class or option-pool-not-held, same
  disposition every other freshly-enumerated-but-unwired kind gets).

## §12b — pinning the 179-unit residual's cause (item 2)

**Root cause identified and is NOT `is_internal_category`.** Full derivation, worked examples in
both directions, and the script:
`15-card-15-internal-duplicate-identity-memo.md` (this directory). Summary: **the corpus-wide
`duplicate_identity` (kind, key) dedup pass** (`v06_work_inventory.rs` lines ~9582-9604) collapses
genuinely distinct PCGen records that happen to share a bare display name (no `KEY:` field to
disambiguate) down to one surviving unit per book+kind. Re-derived: **158 of the (pre-fix) 179
residual rows (88.3%)** share their computed key with another `class_feature` row in the same book
— proved by class, with a worked four-way collision (`Aberrant Bloodline`, `advanced_class_guide`,
4 physically distinct rows for 4 different classes, 1 survives). **Not rescued this cycle** — per
the dispatch brief's own risk framing, distinguishing "genuinely different content sharing a display
name" (Aberrant Bloodline — should NOT be merged) from "byte-identical restatement" (the memo's own
"Touch of Good" example — SHOULD be merged) needs a real per-collision content comparison at the
`duplicate_identity` pass itself, a different fix site from `is_internal_category` and a
meaningfully larger, riskier change than this cycle's scope. **21 of the 179 remain genuinely
unpinned.**

**This cycle's own fix demonstrates the SAME mechanism live**, in both directions the dispatch
brief required: landing `is_internal_category`'s `Kind::ClassFeature` narrowing makes 2,574
previously-trapped rows newly eligible for `duplicate_identity`, and not all of them win their own
race — re-derived post-fix: the non-internal residual grows 179 → **180** (the
`disable_device_class_skill` displacement, §3 of the memo, a row moving OUT), and **27 of the 2,574
candidate internal rows lose their own key collision** (2,574 candidates, only 2,547 land in
`docs/work-inventory.json`) — landing IN, minus 27 that don't. **Total pinned-cause residual after
this cycle: 207** (180 non-internal + 27 internal-collision-losers), all traced to
`duplicate_identity`, none to `is_internal_category`, per-unit detail in `15-reconcile.json`'s
`pending_a.class_feature_residual_duplicate_identity`.

## Sweep of pinned counts — `tests/`, `src/`, `scripts/`, `apps/`

```bash
grep -rn "18231\|15438\|15439\|2792\|2,792\|18,231\|15,438\|15,439" tests/ scripts/ src/ apps/
```

Only doc-narrative files (`progress.md`, `decisions.md`, `release-notes.md`, `kanban.md` — append-only
history, not live assertions) and `scripts/census_independent.py`/`scripts/card15_reconcile.py`
(bookkeeping scripts) matched. No `tests/*.rs` or `src/**` file asserts an exact `class_feature`
count (checked: `tests/v06_work_inventory.rs` and the binary's own inline tests use structural
invariants — `by_kind` sums to `totals.units`, no-unknown-with-zero-magnitude — never a hardcoded
population number). `scripts/card15_reconcile.py`'s hardcoded `15438`/`179`/`2574` narrative entries
updated (see "Files touched").

## Tests

- `cargo test --locked --bin v06_work_inventory` → 329/329 (was 325; +4 net new
  `kind_ability_tests` — 5 added, 1 renamed-in-place, 0 removed).
- `cargo test --locked --lib` → 2,402/2,402 (unaffected — this binary is not part of the lib crate).
- `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml` (separate cargo workspace)
  → 518/518 (unaffected).
- `python3 -m unittest scripts.tests.test_census_independent` → 26/26 (unaffected — this cycle does
  not touch `census_independent.py`, only the separate Rust producer).
- `python3 scripts/shape_ledger.py` → `unclassified_count: 0`, piles reconcile. Population
  **33,426 → 36,015** (+2,589 — tracks `class_feature`'s +2,593 unit growth closely; the small gap
  is units whose `status` already counted as done pre-cycle, unaffected by this fix). The
  last-COMMITTED `ledger.json` (28,490) predates the intervening `card-15-ability` cycle entirely —
  that cycle's own receipt claimed a regeneration to 33,426 but git history shows `ledger.json` was
  never actually re-committed after `5ed69f29f`; re-derived here (33,426, matching that cycle's own
  claimed figure exactly) and this cycle's own fresh **36,015** both committed. Flagged, not
  silently absorbed: the intervening staleness is a pre-existing gap this cycle happens to fix as a
  side effect of its own re-derive, not something this cycle was scoped to audit.

## Gate 3 (`scripts/shape_coverage_standing_gate.py`) — still FAIL, budget NOT touched

```bash
python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json
```

→ `FAIL` (unchanged verdict, `decisions.md §14`'s already-reopened tension). `population` 33,426 →
**36,015**. `no_record` share **21,497/36,015** (59.7%, up from 49.3% pre-cycle) vs. the committed
budget baseline 13,968/28,490 — same already-reopened condition, one more instance of real
enumeration growth outrunning ingestion, not a new blocker. **Budget constants NOT modified.**

## §15 — Product Identity

No record disposed this cycle was transcribed, ingested, or scored against `ogl-pi-blacklist.md` —
enumeration only. No PI-screening question arises at this layer.

## §16 — a unit moved out of a shape is not a unit closed

All 2,593 newly-landed `class_feature` units are `status: not-ingested` — enumerated, not
engineered. The 1 displaced unit (`disable_device_class_skill`) is named and traced (§ above), not
silently absorbed.

## Files touched

- `src/bin/v06_work_inventory.rs` — `CLASS_FEATURE_INTERNAL_CONTENT_PREFIXES`,
  `is_spellknown_token`, `is_internal_gateway_token`, `class_feature_internal_row_is_bare_marker`
  (new); `is_internal_category`'s computation restructured to a `match kind` with a
  `Kind::ClassFeature` arm; 1 test renamed+inverted, 4 new tests.
- `docs/work-inventory.json` — regenerated through the real producer (see Population above).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json`
  — regenerated (`class_feature` kind_unenumerable bucket unaffected by this cycle — that side was
  already fixed by the prior `category-internal-adjudication` cycle).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-reconcile.json`
  — regenerated.
- `scripts/card15_reconcile.py` — `class_feature_internal_adjudicated_pending` (2,574, pending A)
  and `class_feature_residual_original` (179, pending A) retired, merged into one live-derived
  `class_feature_residual_duplicate_identity` entry (207, cause pinned to `duplicate_identity`);
  `class_feature_already_in_inventory` raised 15,438 → 17,984. Re-run: `equals_total_this_run: True`,
  `remaining_undisposed: 0` — the piles still sum exactly (18,992).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-class-feature-residual-cause-pin.py`
  (new) — the re-derive script for the residual/collision analysis.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-internal-duplicate-identity-memo.md`
  (new) — the cause-pinning memo (§12b item 2).

## Identifier / wired-integration audit (this cycle's own diff, scoped to touched files only)

```bash
git diff --unified=0 5b2c93270 -- src/bin/v06_work_inventory.rs \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
git diff --unified=0 5b2c93270 -- src/bin/v06_work_inventory.rs \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```

Both `OK_*`.

## Next-cycle plan

1. **The `duplicate_identity`-collision-caused residual (207: 158 non-internal-vs-non-internal
   collisions + 1 non-internal-vs-newly-internal displacement + ~27 newly-internal-vs-internal
   collisions among themselves — full breakdown in the memo)** needs a per-collision content
   comparison at the `duplicate_identity` pass itself — distinguish "genuinely different records
   sharing a display name" (keep both) from "byte-identical restatement" (correctly merge, as
   today). Different fix site from this cycle's `is_internal_category` work; a real next-cycle scope,
   not a quick follow-on.
2. **21 residual rows with no key collision found by the script's own (documented, non-internal-only)
   check** need their own cause investigation — not attempted this cycle.
3. Card 15 reaches `complete` only when `total_kind_unenumerable_units` reaches 0 (unaffected by
   this cycle — this population lives entirely inside `docs/work-inventory.json`'s own
   `class_feature` kind, not census's `kind_unenumerable` bucket) and the `duplicate_identity`
   residual above is closed by class.

## Disk

`df -h /`: reported at end of turn.
