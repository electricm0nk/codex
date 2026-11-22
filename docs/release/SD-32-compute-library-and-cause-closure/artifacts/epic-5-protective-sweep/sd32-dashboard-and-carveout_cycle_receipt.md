# sd32-dashboard-and-carveout — cycle receipt

Actor: `sd32-dashboard-and-carveout`. Territory: `scripts/observer/pf1e_dashboard_producer.py`,
`scripts/coverage_ledger.py`, `site/dashboard/**`, tests added under `scripts/tests/`.

Branch: `tranche/12`, based at `6e8b4bf9b4` (origin had advanced past the brief's cited
`0ef8dd5cf` — re-fetched and confirmed before starting).

## Item 1 (PRIMARY) — the `beginner_box` carve-out — **RESOLVED THIS CYCLE**

**Re-derived population** (not trusting the brief's "19"):

```
python3 -c "
import json
d=json.load(open('docs/work-inventory.json'))
units=d.get('units',[])
bb=[u for u in units if (u.get('book') or '')=='beginner_box']
print(len(bb))"
```
→ **19**, all `kind == equipment`, all `status == not-started`. Confirmed exact.

**Safety check — was either §27b admissible reason true?**

- *Source data absent?* No. Fetched the pinned oracle (`PCGEN_ORACLE_SHA
  7f818006e371188e5717fd18d74d18a420747fc6`) and found both source files:
  `docs/release/.../artifacts/corpus/operator-supplied/pcgen/data/pathfinder/paizo/
  roleplaying_game/beginner_box/bbox_equip_magic_items.lst` and `bbox_equip_arms_armor.lst`.
  The 19 units are real, declared records (`origin: "declared"`, `source_file:
  "bbox_equip_magic_items.lst"`) already flowing into `docs/work-inventory.json` as
  `not-started` (`evidence: "no_compiled_rule_set_for_book"`) — the carve-out lived ONLY in
  the dashboard-reporting layer (`EXCLUDED_BOOKS`), not at ingestion.
- *Licensing forbids shipping?* No such note anywhere; Beginner Box is an ordinary Paizo
  product under the same PCGen-oracle sourcing every other in-scope book uses.
- Neither admissible reason held → per `decisions.md §27b` this closes, it does not escalate.

**Change**: `EXCLUDED_BOOKS` in `pf1e_dashboard_producer.py` is now `frozenset()` (was
`{"beginner_box"}`). Every consumer (`work_inventory_panel()`'s `excluded` var,
`compute_wiring_class_summary()`'s inline `book not in EXCLUDED_BOOKS` check,
`build_unit_shards()`'s exclusion loop, `coverage_ledger.not_done_population()`'s default arg)
reads this one constant, so clearing it closed the carve-out everywhere at once — verified by
re-running the producer against the real corpus (below), not by inspection alone.

**Mechanism added (item 4 — "stops the next one hiding in code")**: `EXCLUDED_BOOKS_REASONS`
(book → reason) and `ADMISSIBLE_EXCLUSION_REASONS = {"source_data_absent",
"licensing_forbids_shipping"}`, with two module-level `assert`s run at import time: every
`EXCLUDED_BOOKS` entry must have a paired reason in `EXCLUDED_BOOKS_REASONS`, and every reason
must be admissible. A future carve-out added to `EXCLUDED_BOOKS` alone, with no declared reason,
now fails at import rather than shipping silently.

**RED→GREEN regression tests** (`scripts/tests/test_coverage_ledger.py`,
`ExcludedBooksNeedsDeclaredAdmissibleReasonTest` +
`NotDonePopulationTest.test_default_excluded_books_is_empty_so_no_book_is_hidden_by_default`):
- RED reproduced live: with `EXCLUDED_BOOKS = {"beginner_box"}` (pre-fix),
  `test_only_evidence_corroborated_display_grounded_class_features_reclassify` in
  `test_pf1e_dashboard_producer.py` failed with `count 3 != 2` when its "excluded book" fixture
  case was still keyed on `book="beginner_box"` (that fixture now uses a fabricated book id
  patched into `EXCLUDED_BOOKS` for the duration of the test, so it proves the MECHANISM, not
  that any specific book stays excluded forever).
- GREEN: 45/45 tests pass —
  `python3 -m unittest scripts.tests.test_pf1e_dashboard_producer scripts.tests.test_coverage_ledger`.

**Cross-file sweep** (per the brief's "grep old AND new counts across tests/src/apps"):
`grep -rln "EXCLUDED_BOOKS|beginner_box"` over `scripts/`, `tests/`, `src/`, `apps/` found no
hardcoded totals pinned outside `scripts/tests/test_pf1e_dashboard_producer.py` and
`scripts/tests/test_coverage_ledger.py` (both fixed above) and `scripts/tests/
test_census_independent.py` (lane B territory — untouched, references `beginner_box` only as a
`BookDir` fixture id unrelated to `EXCLUDED_BOOKS`, not broken by this change — verified by
inspection, not run, since `census_independent.py` is out of my territory).
`grep -rl "38,521|38521|49,438|49438|6,208|6208"` over `tests/`, `src/`, `apps/` found nothing
but one unrelated `Cargo.lock` hash collision — no Rust-side pinned dashboard totals exist.

**Live re-derivation against the real corpus** (`site/dashboard/PF1e-dashboard.json`, regenerated
this cycle — see item 3): `work_inventory.total_units` = 49,438 (was silently 49,419 under the
carve-out — the full inventory, no subtraction); `beginner_box` now has its own book row,
`units: 19`, `by_status: {"not-started": 19}`; `coverage_ledger.not_done_population()` over the
live inventory now returns 34,416 units (includes the 19) instead of silently excluding them.

## Item 2 — `wiring_class` classifier deferral (T8/D13) — **ALREADY RESOLVED** (proof re-derived)

D13 (`docs/release/SD-31-corpus-closure-grind/todo/defects.md` row) names 12 CRB flag-shaped
`class_feature` units (Evasion, Improved Evasion, Timeless Body, Woodland Stride, Quarry,
Improved Uncanny Dodge and siblings) stamped `display`+`grounded` and never re-examined. T8's
fix is already implemented in `compute_wiring_class_summary()` (`WIRING_SUMMARY_SCHEMA = 13`,
the `T8_RECLASSIFY_EVIDENCE` predicate) and reproduces live:

```
python3 -c "import json; d=json.load(open('site/dashboard/PF1e-dashboard.json'));
r=d['work_inventory']['classifier_reclassified_units']; print(r['count'], sorted(r['units']))"
```
→ `count: 12`, exactly D13's named 12 units, all `core_rulebook`. No further action needed —
tests `ClassifierReclassifiedUnitsTest` in `test_pf1e_dashboard_producer.py` already cover this
(19/19 green in that class, all passing before AND after this cycle's changes).

`monster_ability` shape-alike siblings are DELIBERATELY out of D13's scope (documented inline:
they don't share the corroborating evidence string) — not a gap in this fix, a scoping boundary
D13 itself drew. Not touched.

## Item 3 — stale `site/dashboard/PF1e-dashboard.json` — **RESOLVED THIS CYCLE** (partially —
one downstream defect ESCALATED, out of territory)

`./scripts/publish-site-dashboard.sh --check` confirmed STALE before any other change (this is
independent of the `beginner_box` fix — the committed shard index
(`site/dashboard/units/index.json`) carried only 11 unit kinds
(`class, class_feature, companion, equipment, equipment_modifier, feat, monster,
monster_ability, race, race_trait, spell`); the live corpus has grown 8 more
(`ability, deity, domain, language, power, skill, template, trait`) — `ability` alone is 4,337
units, unrelated to beginner_box entirely).

Ran `./scripts/publish-site-dashboard.sh` (full regenerate, ~49s). `site/dashboard/
PF1e-dashboard.json`, its `.last-good` copy, and every `site/dashboard/units/*.json` shard
(including 8 newly-appearing kind shards) are now current and committed. Verified beginner_box's
19 units land correctly in the regenerated `PF1e-units-equipment.json` shard (19 rows, `book ==
"beginner_box"`).

**Escalation (out of my territory, not fixed):** the same run's second stage,
`scripts/site/build_public_status.py` (NOT under `site/dashboard/**` or my named files — a
separate `scripts/site/` tree), crashed:

```
KeyError: "unit kind 'ability' (PF1e-units-ability.json) has no curated label in
KIND_LABELS — add one before regenerating."
```

This blocks `site/status-data.json` / `site/status-data/*.json` from refreshing to match the
now-current `site/dashboard/units/*.json`. Population: likely all 8 newly-surfaced kinds need a
`KIND_LABELS` entry in `scripts/site/build_public_status.py` (not just `ability` — the script
fails closed on the first missing kind alphabetically, so the others are unconfirmed until that
one is fixed and it's re-run). Cost: adding N curated labels to one dict, small, but requires
someone who owns that file to make the editorial call on each label's wording — not something I
should improvise into a file outside my territory. **Needs**: whichever lane/file-owner covers
`scripts/site/build_public_status.py` to add the missing `KIND_LABELS` entries and re-run
`./scripts/publish-site-dashboard.sh` to complete the public status projection refresh.
`site/status-data.json`/`site/status-data/` were NOT touched this cycle (the script crashed
before writing) — no partial/corrupt state was introduced, just left exactly as stale as it
already was.

Producer write path (`_atomic_write_json`) is atomic (`tempfile` + `os.fsync` + `os.replace`,
same filesystem, `.last-good` snapshot on success) — safe against a crash mid-write. It has no
cross-process advisory lock, so two concurrent invocations (the note's "1-minute renderer writes
a different file" plus a manual publish) can still race last-writer-wins; this is a pre-existing
architectural property, not something this cycle's staleness was caused by (the actual cause was
simply: nobody had re-run the publish script since new kinds entered the corpus). No corruption
observed or reproduced this cycle; noting it here as background truth, not a new open item.

## Movement, in the four buckets

- **Closure**: 19 `beginner_box` equipment units now counted in every SD-32 closure figure
  (`total_units`, `not_done_population`, per-book/kind/status/wiring_class/doneness rollups,
  unit-search shards) instead of being silently hidden. `site/dashboard/PF1e-dashboard.json` and
  `site/dashboard/units/*.json` refreshed to current corpus state (8 new kinds surfaced,
  4,300+ `ability` units alone now visible on the dashboard for the first time).
- **Reclassification**: none this cycle (T8/D13's reclassification was already landed and is
  unchanged; re-confirmed live, not reclassified further).
- **Reachability**: not touched — this cycle's scope was dashboard reporting, not engine
  reachability.
- **Instrument-correction**: `pf1e_dashboard_producer.py`'s `EXCLUDED_BOOKS` mechanism corrected
  from a silent, undeclared carve-out to an empty default with an import-time admissible-reason
  guard. `scripts/coverage_ledger.py`'s `not_done_population()` default corrected the same way
  (inherits the fix via the shared `EXCLUDED_BOOKS` constant). One NEW instrument defect
  discovered and escalated (not fixed, out of territory): `scripts/site/build_public_status.py`'s
  `KIND_LABELS` dict is missing entries for 8 unit kinds that have existed in the corpus long
  enough to reach 4,337 units (`ability`) without anyone noticing the public status projection
  had stopped refreshing.

## Commands used (re-derivable)

```
python3 -m unittest scripts.tests.test_pf1e_dashboard_producer scripts.tests.test_coverage_ledger
./scripts/publish-site-dashboard.sh --check   # STALE, before this cycle's regenerate
./scripts/publish-site-dashboard.sh           # regenerated; second stage escalated (see item 3)
```
