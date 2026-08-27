# Cycle AT-33-E4-001 — Epic 4 Unknown classification / AT-33-E4-001

- **Commit SHA:** `5bce7235d6`
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-4-unknown-classification/unknown-rootcause.md` (new)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** one hit —
  `+| 4: Feat, served description is a placeholder marker | 48 | content-integrity gap (upstream pipeline) | stays unclassifiable, renamed \`unmeasurable\` |`.
  Reviewed: this documents a real, pre-existing production evidence string
  (`feat_served_description_is_a_placeholder_marker_not_prose`, already shipped in
  `classify()` before this cycle) — a factual finding about corrupted SHIPPED
  CORPUS TEXT, not a stub/placeholder in code this cycle writes. No token in the
  actual diff's code (`AT-33-E4-002`'s commit, audited separately) matches.
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**

  > ### AT-33-E4-001 — the cause of `unknown` is established before reclassification
  >
  > Whether these units are genuinely unmeasured, or measured by an instrument that could not
  > express the result, is **answered before any count moves** (`instrument-correction-is-not-closure`:
  > a count that drops because measurement changed is not closure).
  >
  > **Evidence:** `artifacts/epic-4-unknown-classification/unknown-rootcause.md`.

## What landed

`unknown-rootcause.md` traces every one of the 4,224 `status: "unknown"` units, pre-cycle, back to
its exact `classify()` call site (5 evidence strings, verified by execution — see the figure below),
and answers the cause question **per shape**, not as a single population-wide verdict:

- **519 (Feat, `in_catalog_with_corpus_magnitude_but_no_observed_consumer`):** instrument asymmetry —
  every sibling `Kind` already reads `ingested-magnitude` for this identical evidence shape;
  `Kind::Feat` alone had no code path to say so.
- **309 of 579 (Feat/Equipment/EquipmentModifier, `text_only_but_corpus_record_carries_no_description_to_show_a_player`,
  `wiring_class != "display"`):** instrument asymmetry, proven structurally: `wiring_class::signals`
  cannot emit a `computed:`/`derived:`/`static:` signal on an empty magnitude-token set, so
  `wiring_class != "display"` is proof the closure carries a real magnitude the record's own line
  does not show.
- **26 (Spell, `spell_list_entry_with_no_corpus_level_and_no_description`):** same instrument
  asymmetry, same proof; all 26 have `wiring_class != "display"`.
- **3,052 (ClassFeature, `class_feature_group_names_no_class_at_all`):** instrument asymmetry for the
  disposition (the same effect-wired probe already justifies `not-ingested` for this shape's
  `text_only` sibling); a **separate, unattempted** research opportunity exists to resolve some of
  these to a real owner by widening `REGISTERED_POOL_GROUPS` (2 entries registered against 1,128
  distinct unmatched group prefixes in this population) — named, not pursued this cycle.
- **270 of 579 (Feat/Equipment/EquipmentModifier, `wiring_class == "display"`):** genuinely empty
  corpus records — no magnitude anywhere in the closure (proven), no description. No existing status
  honestly fits. Stays unclassifiable.
- **48 (Feat, `feat_served_description_is_a_placeholder_marker_not_prose`):** content-integrity gap
  (PI redaction / upstream "not implemented" marker) in the served-description pipeline, outside this
  file's write scope. No existing status honestly fits. Stays unclassifiable.

**No count moved in this cycle before this document existed.** `AT-33-E4-002`'s receipt implements
the reclassifications this document identifies, and cites this document as the cause statement for
each one.

## Figures + their re-derive commands

- **4,224** units at `status: "unknown"`, pre-cycle —
  `jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json` (run against the
  commit immediately prior to this cycle's regen; denominator: the full committed inventory,
  49,438 units — `jq '.units|length' docs/work-inventory.json`).
- **5** distinct evidence strings partition the 4,224 exactly —
  `jq -r '.units[]|select(.status=="unknown")|.evidence' docs/work-inventory.json | sort | uniq -c`
  (3052 + 579 + 519 + 48 + 26 = 4224).
- **1,128** distinct unmatched `class_feature` group prefixes inside the 3,052-unit shape —
  `jq -r '.units[]|select(.evidence=="class_feature_group_names_no_class_at_all")|.corpus_key' docs/work-inventory.json | sed 's/ ~ .*//' | sort -u | wc -l`
  (denominator: the 3,052-unit shape itself).
- **2** registered pools against that 1,128 —
  `grep -n "REGISTERED_POOL_GROUPS" src/rules_core/class_feature_pool_catalog.rs`.

## Status: complete

## Movement, four buckets

- **closure:** 0 — this criterion is the analysis; it moves no unit.
- **reclassification:** 0 — reclassification is `AT-33-E4-002`'s deliverable, gated on this
  document existing first.
- **reachability:** 0
- **instrument-correction:** 0

## Notes

- **Method, not memory.** Every shape's cause is traced to its exact source line in `classify()`
  (`src/bin/v06_work_inventory.rs`), not inferred from the evidence string's name. The Feat/Equipment
  asymmetry claim is a structural proof from `wiring_class::signals`'s own guard conditions
  (`src/rules_core/wiring_class.rs`), not an assertion.
- **Judgment call:** the 270+48=318-unit remainder (Shapes 2-display and 4) is determined
  genuinely unclassifiable **within this file's write scope** this cycle — not because no further
  work is conceivable (Shape 4's cause names a concrete, different owning file), but because no
  existing status in `STATUS_VOCABULARY` honestly describes either shape, and fabricating a
  description or a magnitude is the exact conduct the no-stub doctrine forbids.

## Next-cycle plan

`AT-33-E4-002` implements the reclassifications named above and reports the four-bucket movement
by execution. `AT-33-E4-003` verifies the reclassified population against `box_ledger.py --check`.
Widening `REGISTERED_POOL_GROUPS` against the 1,128 distinct unmatched group prefixes remains
future scope, not committed to any cycle.
