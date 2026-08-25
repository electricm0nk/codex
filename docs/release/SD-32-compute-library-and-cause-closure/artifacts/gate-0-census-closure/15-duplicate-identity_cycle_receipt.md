# Cycle card-15-duplicate-identity — census-scope-closure / `decisions.md §12b`, `duplicate_identity` collision rescue (narrow, validated)

- **Card ID:** `census-scope-closure` (kanban card 15). **Status stays `in-progress`** — 134 of
  the 156 non-internal residual rows (the `*Choice`-typed and keyed-collision populations, see
  below) and the 22 genuinely-unpinned rows are real, distinct next-cycle scope this cycle
  deliberately does not touch.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/verify.sh --only
  preflight-oracle` → PASS, oracle bootstrapped fresh into the repo-local
  `artifacts/corpus/operator-supplied/pcgen` slot — a fresh worktree's slot was empty, per the
  dispatch brief's own warning).

## §17a re-derivation — both the 207/158/179 split and this cycle's own figures re-derived fresh

`15-card-15-class-feature-residual-cause-pin.py` re-run against the pinned oracle and the
current `docs/work-inventory.json`: **180 non-internal residual, 158 collide, 22 do not** —
reproduces exactly. Full derivation and the major correction found mid-cycle:
`15-card-15-duplicate-identity-memo.md` (this directory).

## What landed — `src/bin/v06_work_inventory.rs`

**The identity fix, not the symptom.** `disambiguate_class_feature_fallback_collisions` (new
fn) — for a `Kind::ClassFeature` row with no declared `KEY:` field (the exact fallback
signature that produced the weak identity), `CATEGORY:` is the field that reliably
distinguishes two genuinely distinct records sharing a bare display name. A new
`BookEnumeration::class_feature_categories` field (`(source_file, source_line) -> CATEGORY:`
value, populated only for `Kind::ClassFeature`) carries this without widening `CorpusUnit`
itself. Runs immediately before `duplicate_identity`'s existing filter, in the same per-book
loop. Full mechanism and validation: this fn's own doc comment, `src/bin/v06_work_inventory.rs`.

**A major correction, found mid-cycle, before landing anything on trust.** The dispatch brief's
own worked example (`advanced_class_guide`'s four "Aberrant Bloodline" rows) turned out to be
the SAME shape as a **pre-existing, operator-confirmed 33-id allowlist**
(`DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`, SD-31 `decisions.md` Decision 17) of bare
`class_feature` rows already proven to be a duplicate-chooser-picker row beside its own real
feature, not a second object — and Decision 17's own text explicitly forbids building a live
adjacency filter to auto-detect more of them. **Consequence:** every fallback-key collision
group whose members ALL carry a `TYPE:` facet ending in `"Choice"` is EXCLUDED from this
cycle's rescue (39 of 64 validated groups, including the brief's own flagship example) — same
disposition as `CATEGORY:Internal`, left untouched, reported as a hand-review population for
the next cycle. Only the confirmed-safe 25 groups (`TYPE:FavoredClass` tracker rows colliding
with unrelated `TYPE:Class` chassis rows, one pair per class) are rescued. Full trace, including
the specific `ultimate_magic:class_feature:accursed_bloodline` collision that surfaced this:
`15-card-15-duplicate-identity-memo.md` §"The correction".

## RED → GREEN

Six new unit tests in `disambiguate_class_feature_fallback_collisions_tests`
(`src/bin/v06_work_inventory.rs`):

- `four_way_category_collision_rescues_all_four` — the `Barbarian`-shape 4-way `CATEGORY:`
  collision (non-`*Choice`-typed): all four survive with distinct keys. Without this fn, three
  of four are silently dropped by `duplicate_identity`'s later filter — confirmed RED by direct
  inspection of the prior (unmodified) filter logic, GREEN under the fix.
- `choice_typed_collision_is_left_untouched` — the real, corpus-confirmed
  `ultimate_magic:accursed_bloodline` shape (566/2070): neither key changes. Proves the
  correction actually holds, not just documented.
- `same_category_collision_is_left_to_collapse_normally` — a true restatement (same key, same
  category) still collapses to one, unaffected.
- `internal_category_sibling_never_gets_disambiguated` — the `Disable Device Class Skill` shape
  (`15-internal_cycle_receipt.md` §3): unaffected by this fn, byte-for-byte the prior cycle's
  own behaviour.
- `declared_key_row_is_never_touched` — a row with an explicit `KEY:` field is out of scope.
- `non_class_feature_kind_is_never_touched` — the kind guard holds.

## Population, before and after — both directions proved

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
export CORPUS_LITERAL_SWEEP_REPORT=<fresh corpus_literal_sweep --json-out report>
export DERIVED_FIXTURE_CHECK_REPORT=<fresh derived_evaluator_fixture_check --json-out report>
cargo run --locked --bin v06_work_inventory
```

| | before | after | delta |
|---|---:|---:|---:|
| `totals.units` | 49,516 | 49,540 | **+24** |
| `class_feature` | 18,032 | 18,056 | **+24** |
| every other kind | — | byte-identical | 0 |

**Both directions, proved by physical-location diff**: 0 physical locations lost, 24 gained, 0
duplicate ids, 0 duplicate physical locations. 5 ids renamed (`unit_id`'s existing
slug-collision suffix mechanism, not a defect this cycle introduced) — each confirmed still
present under its new id at the SAME physical location. **Full `status` distribution diffed**:
`literal-verified` 6,506 → 6,506, `fixture-verified` 1,741 → 1,741, `grounded` 2,515 → 2,515,
`text-complete` 3,858 → 3,858, `deferred-with-reason` 46 → 46, `ingested-magnitude` 1,404 →
1,404, `not-started` 19 → 19, `unknown` 4,264 → 4,264 — every verification stamp preserved
exactly. Only `not-ingested` grew (29,163 → 29,187, +24), matching the 24 new units.

`apply_duplicate_chooser_removal`'s own drift guard (`std::process::exit(1)` if the removed
count ≠ 33) fired NOT once across this regen — direct, mechanical confirmation that the `*Choice`
exclusion actually kept this fix out of the risky population, not merely a documented intention.

## Re-derived residual after this cycle

156 non-internal residual (was 180, −24), 134 still collide (was 158), 22 unexplained
(unchanged). Total pinned-cause residual: **183** (was 207 — 156 + the pre-existing,
unaffected-by-this-cycle 27 internal-collision-losers). `scripts/card15_reconcile.py` re-run:
`equals_total_this_run: True`, `remaining_undisposed: 0`, 18,992 total.

## Gate 3 (`scripts/shape_coverage_standing_gate.py`) — still FAIL, budget NOT touched

```bash
python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json
```

→ `FAIL` (unchanged verdict, `decisions.md §14`'s already-reopened tension). `population`
36,015 → 36,028 (this cycle's own regen measured 36,039 before the post-rebase re-derive below
moved 11 more units off not-done via a concurrent sibling lane's unrelated work — re-confirmed
against the final pushed tree). `no_record` share 21,521/36,028 (59.7%) vs. the committed budget
baseline 13,968/28,490 — same already-reopened condition, one more instance of real enumeration
growth outrunning ingestion, not a new blocker. **Budget constants NOT modified.**
`docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/ledger.json`
regenerated for consistency (population 36,015 → 36,028, `unclassified_count: 0`, piles
reconcile).

**Post-rebase regeneration note:** `origin/tranche/12` advanced during this cycle (T9/T12 sibling
lanes, `895cc4e55`/`d0c36e27b`/`9838c344d`, touching `src/bin/v06_work_inventory.rs` and real
engine wiring). After rebasing, `docs/work-inventory.json` was regenerated FRESH from the real
producer rather than trusted from git's text-level merge of the JSON — confirmed necessary: the
merged JSON's `ingested-magnitude`/`not-ingested`/`text-complete` buckets were stale relative to
the sibling lanes' own source changes (`totals.units` identical at 49,540, but those three
buckets shifted 1,474/29,106/3,869 vs. the merged snapshot's 1,404/29,187/3,858 once regenerated)
— exactly the hazard the dispatch brief's regeneration warning names. `class_feature` (18,056),
every `literal-verified`/`fixture-verified` stamp, and this cycle's own reconciliation all
re-confirmed unchanged against the fresh regen.

## §15 — Product Identity

No record disposed this cycle was transcribed, ingested, or scored against
`ogl-pi-blacklist.md` — enumeration only. No PI-screening question arises at this layer.

## §16 — a unit moved out of a shape is not a unit closed

All 24 newly-landed units are `status: not-ingested` — enumerated, not engineered. No unit was
removed from any shape; the 5 renamed units are traced above with their physical location
confirmed unchanged.

## Sweep of pinned counts — `tests/`, `src/`, `scripts/`, `apps/`

```bash
grep -rn "18032\|17,984\|17984\|207\b|180\b.*residual|residual.*180\b|158\b.*collide|49516\|49,516" tests/ scripts/ src/ apps/
```

Only `scripts/card15_reconcile.py` (already updated this cycle) and `docs/release/.../progress.md`
(append-only history, not a live assertion) matched. No `tests/*.rs` or `src/**` file asserts an
exact `class_feature` population number — `tests/v06_work_inventory.rs` and this binary's own
inline tests use structural invariants only.

## Tests

- `cargo test --locked --bin v06_work_inventory` → 335/335 (was 329; +6 net new).
- Full sweep NOT run (out of scope per dispatch brief's own scoping instruction — the touched
  file is isolated to `src/bin/v06_work_inventory.rs` and `scripts/card15_reconcile.py`, neither
  consumed by the lib crate or the desktop crate).
- `python3 scripts/shape_ledger.py` → `unclassified_count: 0`, piles reconcile, population
  36,015 → 36,039.

## Identifier / wired-integration audit (this cycle's own diff, scoped to touched files only)

```bash
git diff --unified=0 45012f6a9 -- src/bin/v06_work_inventory.rs \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
git diff --unified=0 45012f6a9 -- src/bin/v06_work_inventory.rs \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```

Both `OK_*`.

## Files touched

- `src/bin/v06_work_inventory.rs` — `BookEnumeration::class_feature_categories` (new field);
  `disambiguate_class_feature_fallback_collisions` (new fn, 6 new tests); wired into the
  `duplicate_identity` per-book loop.
- `docs/work-inventory.json` — regenerated through the real producer (see Population above).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-reconcile.json`
  — regenerated.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/ledger.json`
  — regenerated for consistency.
- `scripts/card15_reconcile.py` — `class_feature_residual_duplicate_identity` (207 → 183,
  updated narrative) and `class_feature_already_in_inventory` (17,984 → 18,008) updated;
  arithmetic-check narrative updated to match.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-duplicate-identity-memo.md`
  (new) — the fix + correction memo.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-duplicate-identity-key-validation.py`
  (new) — the committed, re-runnable validation script.

## Next-cycle plan

1. **134 still-colliding, still-not-rescued residual rows** (39 `*Choice`-typed fallback groups
   + 16 keyed-collision groups) need the SAME per-case hand review SD-31 `decisions.md`
   Decision 17 already did for its own 33 confirmed ids — determine, row pair by row pair,
   whether the colliding sibling is a picker beside its own real feature (add to
   `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`) or a genuinely distinct feature (rescue via this
   cycle's own mechanism, once reviewed). Not a smarter automatic heuristic — Decision 17's own
   text forbids that.
2. **22 genuinely-unpinned residual rows** — pin the cause or report precisely what remains
   unknown; not attempted this cycle, per the dispatch brief's own instruction not to fold them
   into the collision fix on assumption.
3. Card 15 reaches `complete` only when `total_kind_unenumerable_units` reaches 0 (unaffected by
   this cycle) and the `duplicate_identity` residual above is closed by class.

## Disk

`df -h /`: reported at end of turn.
