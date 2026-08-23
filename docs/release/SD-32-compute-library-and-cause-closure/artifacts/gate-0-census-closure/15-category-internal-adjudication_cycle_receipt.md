# Cycle category-internal-adjudication — Gate 0 (census-scope-closure) / Card 15, `decisions.md §14c` item 4

- **Card ID:** 15 (`census-scope-closure`)
- **Commit SHA:** `e79d508b4`
- **Files touched:**
  - `scripts/census_independent.py` — narrowed the `row_dependent_class_feature` blanket-exclusion
    to a proven per-row test (`_row_is_bare_internal_marker`)
  - `scripts/tests/test_census_independent.py` — replaced the stale test asserting the old blanket
    behaviour with 4 tests covering bare/content/gateway/non-Internal rows
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-category-internal-classify.py` (new) — committed re-derive script
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-category-internal-rows.jsonl` (new) — per-row output, 2,614 rows
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-category-internal-summary.md` (new) — disposition/token/gateway tables
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-category-internal-adjudication-memo.md` (new) — the full adjudication memo
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json` — regenerated via its own designated command (§5 of the memo); `total_kind_unenumerable_units` unchanged (27,838), `class_feature` 15,617→18,191, `ability_category:Internal` 3,453→879
  - `docs/retro/events/category-internal-adjudication.jsonl` (new) — 1 correction event
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** `decisions.md §14c` item 4 — "Two lanes reached opposite conclusions on the same rows and neither cross-checked... Settle it by evidence before any of it is enumerated or excluded."
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** complete
- **Notes:**
  - Re-derived the 2,614 independently; unchanged from the figure named in `decisions.md §14c`.
  - Final disposition: **2,371 (A) / 243 (B)** — 203 proven-facet (gateway resolves) + 40 proven-inert
    (zero content, zero gateway). Both class_feature and ability_category lanes' original reasoning
    was directionally right method, wrong test width: the class_feature memo's own two worked
    examples ("Damage Reduction ~ All/Silver") flip from its claimed (B) to (A) once the content
    test includes `DR:` (a real mechanical field the memo's DEFINE/BONUS-only test missed).
  - Verifier's 92.6%/2,420 figure directionally confirmed; 4 of its 6 per-token counts
    (`SPELLKNOWN`/`DEFINE`/`TEMPBONUS`/`AUTO`) reproduce exactly, the other 2 (`BONUS`/`ABILITY`)
    reproduce exactly once traced to a substring-match artifact (conflates `TEMPBONUS:` into
    `BONUS:`, `PREABILITY:` into `ABILITY:`) — filed as a correction, not silently adjusted.
    Verifier's 910-unresolved figure could not be reproduced by any join method tried (own KEY:
    field, `corpus_key`, or bare identity) — this cycle's own resolution test found 55 unresolved,
    all traceable to `%LIST` runtime placeholders or an out-of-scope target kind (`domain`), not
    orphaned references. Reported as unreconciled rather than adjusted to match.
  - Applied a deliberately conservative code fix: only the 40 provably-bare-with-no-cheap-test-cost
    rows are excluded in `census_independent.py`; the 203 gateway-resolved facets are NOT
    additionally excluded (cross-file target resolution is out of this single-pass walker's current
    architecture) — they stay counted as `class_feature`, flagged for a future card. This is the
    safer direction under the anti-gaming/burden-of-proof rules (`decisions.md §1a`/`§12b`):
    under-exclude, never over-exclude, when a cheaper robust test is available and a more thorough
    one is not yet built.
  - RED→GREEN: pre-fix module (loaded from `git show HEAD:scripts/census_independent.py`) returns
    `ability_category:Internal` for the DR:-bearing row the new test asserts stays `class_feature`
    — confirmed failing for the intended reason before the fix. Post-fix: 16/16 tests pass
    (`python3 -m unittest scripts.tests.test_census_independent -v`).
  - Flagged, not fixed (out of granted scope): `scripts/card15_reconcile.py` (line 96, hardcoded
    `"units": 2614` disposed-B) and `15-reconcile.json` are now stale given this cycle's finding —
    named exactly in the memo §6 for the integration/enumeration lane's next cycle.
  - Did not modify `docs/work-inventory.json` or its producer, per this cycle's granted scope.
- **Discovery forwards:**
  - `## DISCOVERED`: `scripts/card15_reconcile.py`'s hardcoded 2,614-disposed-B assumption and its
    `class_feature_lane_claim`/`ability_category_lane_claim` narrative fields are stale post-fix —
    needs a follow-up cycle in card 15's own scope (not this cycle's).
- **Next-cycle plan:** the enumeration lane adds the 2,574 real `class_feature` rows this memo names
  (2,371 A + 203 B-gateway-resolved-but-still-counted) to `docs/work-inventory.json`'s `class_feature`
  kind, alongside the previously-identified 179-row residual, the `ability` (5,108), `skill` (170),
  and six other-kinds candidates (3,551) — then updates `card15_reconcile.py`'s stale assumptions and
  re-runs the "sum the piles" reconciliation for card 15's real acceptance bar.
