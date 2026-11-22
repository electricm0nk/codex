# Cycle 001 — Gate 0 census closure / Card 15 measurement lane — `ability_category:*` buckets

- **Card ID:** `census-scope-closure` (card 15), `ability_category:*` measurement lane
  (`decisions.md §12b`)
- **Commit SHA:** (recorded at push, see progress.md entry)
- **Files touched:**
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-ability-category-classify.py`
    (new) — committed, self-checking per-row disposition classifier, reuses
    `census_independent.py`'s own scope/walk/parse functions.
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-ability-category-rows.jsonl`
    (new, generated) — per-row evidence, 5,886 rows.
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-ability-category-summary.md`
    (new, generated) — per-bucket disposition table.
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-ability-category-memo.md`
    (new) — the deliverable decision memo, one section per bucket.
  - `docs/retro/events/card-15-ability-category.jsonl` (new) — one `correction` event.
  - No writes to `docs/work-inventory.json`, `scripts/census_independent.py`,
    `scripts/shape_ledger.py`, or any pinned-count file, per this card's scope boundary.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** `decisions.md §12b` / kanban card 15 row: "Close the 27,847
  kind-unenumerable objects: enumerate + classify, or prove not-an-object by class ... Acceptance =
  census, inventory, and ledger populations reconcile with ONE committed command ('sum the piles'),
  and every unit in the reconciled total carries a family." This cycle covers this card's
  `ability_category:*` third of that population (5,886 of 27,847 units); the two sibling lanes
  (`class_feature` 18,231; "everything else" 3,551 + `unclassified:<file>` 179) and the integration
  cycle that applies all three memos close the remainder.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** complete (this lane's measurement-and-memo deliverable). Card 15's row stays
  `in-progress` — three lanes share it, per the dispatch brief, and only the integration cycle may
  set it `complete`.
- **Notes:**
  - Method: every `ability_category:*` row gets exactly one of four dispositions
    (`A` real/distinct, `B-duplicate` exact-`KEY:` match on a tracked kind, `B-gateway` facet
    wrapper, `B-picklist` bare chooser value), derived by a committed script that self-checks its
    own bucket totals against `diff.json`'s (`self-check: MATCH`, this cycle).
  - Result: 5,108 of 5,886 units (86.8%) are real, currently-uncounted content — recommend a new
    tracked kind `ability`. 778 units (13.2%) are not objects: 8 exact-`KEY:` duplicates already
    counted under `race_trait`/`monster_ability`, 210 gateway/wrapper rows, 560 bare pick-list
    entries.
  - The operator's own flagged risk ("`Special Ability` at 3,436 is the one most likely to be a
    double-count") did not hold: only 7 of 3,436 are genuine duplicates (0.2%); 3,363 are real.
  - A genuine early-pass error was caught and corrected in-cycle: identity-*string* collision rate
    (up to 88.2% for some buckets) was initially read as a double-count signal, then replaced with
    an exact-`KEY:`-field-only join (which PCGen itself uses for cross-references) after per-record
    inspection showed the string collisions were coincidental name reuse across disjoint object
    populations. Logged as `scripts/retro.py correction`
    (`docs/retro/events/card-15-ability-category.jsonl`, id
    `1787448814998-card-15-ability-category-4a1508`).
  - `UNKNOWN` (15 units) is a `census_independent.py` labeling gap, not a genuinely unknown
    category — true category is `Special Ability`, expressed via a nonstandard identity-embedded
    `CATEGORY=` syntax `_row_category_tag` doesn't parse. Named as a discovery-forward, not fixed
    here (out of this card's write scope — `scripts/census_independent.py` is not writable by this
    lane).
- **Discovery forwards:** four items in the memo's "Discoveries / forwards for the integration
  cycle" section — new kind `ability` (5,108 units), `shape_ledger.py`'s extraction-key gap for
  several non-`DEFINE`/`BONUS` content tokens, the 8-unit duplicate-exclusion list, and the
  `UNKNOWN`/`CATEGORY=` labeling-gap fix.
- **Next-cycle plan:** the integration cycle (per the dispatch brief) reads this memo plus the two
  sibling lanes' memos, adds the `ability` kind (or whatever kind name the reconciliation across all
  three lanes settles on) to `docs/work-inventory.json`'s vocabulary and pinned-count files, and
  reconciles the census/inventory/ledger populations with the one committed "sum the piles" command
  `decisions.md §12b` names as this card's acceptance bar.
