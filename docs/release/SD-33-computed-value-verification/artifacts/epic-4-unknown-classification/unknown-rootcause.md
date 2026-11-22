---
canonical: true
owner: sd33-e4-unknown
bundle_id: SD-33
date: 2026-08-25
---

# AT-33-E4-001 — Root cause of the `unknown` population

**Population, confirmed by execution against the committed pre-cycle inventory:**

```
$ jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json
4224
```

Matches `THE-BOX.md`'s `unknown` group count (4,224) exactly. This document
answers the question `AT-33-E4-001` requires answered **before any count
moves**: for each shape inside this population, are these units genuinely
unmeasured (nobody has looked, or no instrument exists that could look), or
measured by an instrument that could not **express** the result it already
had evidence for?

## Method

`status: "unknown"` is produced by exactly 6 call sites in
`classify()` (`src/bin/v06_work_inventory.rs`), 5 of them reachable by the
live corpus (the 6th is a `#[cfg(test)]` fixture, unrelated). Each is
identified by its `evidence` string, which is itself unique per call site.
Grouping the 4,224 units by `evidence` and cross-tabulating against
`wiring_class` (a second, independently-computed signal already carried on
every unit) gives five distinct shapes:

```
$ jq -r '.units[]|select(.status=="unknown")|.evidence' docs/work-inventory.json | sort | uniq -c
   3052 class_feature_group_names_no_class_at_all
    579 text_only_but_corpus_record_carries_no_description_to_show_a_player
    519 in_catalog_with_corpus_magnitude_but_no_observed_consumer
     48 feat_served_description_is_a_placeholder_marker_not_prose
     26 spell_list_entry_with_no_corpus_level_and_no_description
```//4224 total, denominator: the committed pre-cycle work-inventory.json unit count

## Shape 1 — `in_catalog_with_corpus_magnitude_but_no_observed_consumer` (519, Feat)

**Cause: instrument asymmetry, not genuine unmeasurement.** This is
`Kind::Feat`'s terminal fallback: the feat IS in the engine's catalog, its
own corpus row carries a real magnitude token (or a prose formula), and the
feat-effect probe observed no delta for it. Every sibling `Kind` with the
identical shape (`Kind::Equipment`'s bottom fallback, `Kind::Spell`'s
`Some(true)` arm, `Kind::RaceTrait`'s consumer-check fallback) already reads
`ingested-magnitude` ("held") for this exact evidence pattern — a real
record with a real magnitude, no verified consumer yet. `Kind::Feat` alone
read `unknown` instead, for a reason unrelated to the evidence (no engine
diagnostic is scoped to a feat, ruling out `deferred-with-reason` — never an
argument against `held`). The pre-existing test
`a_prose_formula_feat_does_not_read_text_complete`'s own comment already
named the honest answer before this fix ("Honestly `unknown`/`held`").

**Verdict: measured, not expressible.** The instrument already had the
evidence (catalog membership + magnitude + no-consumer); it just had no
`Kind::Feat` code path to say `held`. **Fixed this cycle** (see
`AT-33-E4-002`'s receipt): status → `ingested-magnitude`, unconditionally,
matching every sibling `Kind`.

## Shape 2 — `text_only_but_corpus_record_carries_no_description_to_show_a_player` (579, Feat/Equipment/EquipmentModifier) and Shape 5 — `spell_list_entry_with_no_corpus_level_and_no_description` (26, Spell)

**Cause: partly instrument asymmetry, partly genuinely empty records — split
by `wiring_class`.**

```
$ jq -r '.units[]|select(.evidence=="text_only_but_corpus_record_carries_no_description_to_show_a_player")|.wiring_class' docs/work-inventory.json | sort | uniq -c
    140 equipment_modifier / display
     49 equipment_modifier / static
      3 equipment_modifier / derived
    181 equipment_modifier / computed
    119 equipment / display
     49 equipment / static
     27 equipment / computed
     11 feat / display
```//579 total, same command family as above, filtered per-kind

`classify()`'s `text_only` gate for these branches is
`unit.magnitude_token_count == 0 && !carries_prose_magnitude` — **this
record's own corpus line only.** `wiring_class` (`src/rules_core/
wiring_class.rs::signals`) is computed from the **full token closure**,
including any inherited `.COPY=`/`.MOD` rows, and — read directly from the
source — a `computed:`/`derived:`/`static:` signal is **structurally
impossible** unless the closure's `mags` set (`MAGNITUDE_TOKENS`-prefixed
fields) is non-empty (`signals_with_rules`, the `if !mags.is_empty()` and
`else if !out.iter().any(computed/derived)` guards). So `wiring_class !=
"display"` is proof — not inference — that the closure carries a real
magnitude this record's own line does not show, most often a `.COPY=`
alias row inheriting its base row's `BONUS:`/`COST:`/`WT:` chain (the exact
mechanism `token_closure_rows_resolves_a_copy_row_s_inherited_base_row`
proves the closure genuinely follows).

**309 of the 579** (233 equipment_modifier + 76 equipment; wc_class in
{computed, derived, static}) plus **all 26** of the spell shape are this
case: **measured, not expressible** — same disposition as Shape 1. **Fixed
this cycle:** status → `ingested-magnitude` when `wc_class != "display"`,
narrowly scoped to this one branch (does not touch the `has_real_description`
-gated `text-complete` promotion above it, which never fires for this
population since `has_real_description` is false by construction here).

**270 of the 579** (259 equipment/equipment_modifier + 11 feat; `wc_class ==
"display"`) are the genuine remainder: `wiring_class::signals` proves
`mags.is_empty()` for these — the closure carries **no** magnitude token
anywhere, own line or inherited — and no real `DESC:` text either. **Cause:
genuinely empty records** (`.COPY=`/chassis-only rows with no cost, no
weight, no prose — the same 634-unit shape `Kind::Equipment`'s own comment
already names from an earlier wave). No existing status honestly fits: not
`not-ingested` (the key IS in the engine's catalog — `not-ingested` means
the catalog lookup itself failed, which already returned earlier in the same
match arm), not `text-complete` (no real description, `has_real_description`
is false by construction), not `ingested-magnitude` (no magnitude exists to
hold, proven by `wiring_class`). **This remains genuinely unmeasurable** —
not because nobody looked (the per-unit `reason` field states the exact
finding), but because the corpus record itself carries nothing to verify.

## Shape 4 — `feat_served_description_is_a_placeholder_marker_not_prose` (48, Feat)

**Cause: content-integrity gap, not measurement.** The feat's raw corpus
closure DOES carry real `DESC:` text (`has_real_description` is true by
construction to reach this branch), but the *served* description — the
compiled value a player actually reads — is a PI-redaction marker or one of
PCGen's own "not implemented" editorial admissions (20 known spellings).
`wiring_class` is `display` (38) or `ambiguous` (10) for every one of
these — `ambiguous` here means a prose-scaling phrase was found but could
not be resolved to a clean formula, not proof of a real magnitude the way
`computed`/`derived`/`static` are for Shapes 1/2/5. There is no honest
existing status: `text-complete` is explicitly, definitionally wrong (its
own vocabulary entry requires a "non-PI-redacted DESC: value"); a fabricated
description is never the alternative (`AGENTS.md` §6, no-stub doctrine).
**This remains genuinely unmeasurable** for this cycle — the fix belongs to
the upstream PI-screening/description-serving pipeline
(`src/rules_core/pi_screening.rs` and the served-description compilation
this file only reads, both outside `AT-33-E4`'s write scope), not to this
classifier.

## Shape 3 — `class_feature_group_names_no_class_at_all` (3,052, ClassFeature)

**Cause: instrument asymmetry for the whole shape, real research debt for a
known subset.** This is the terminal fallback of `Kind::ClassFeature`'s
owner-resolution chain (`class_feature_owner` →
`class_feature_owner_via_type_facet` → `class_feature_owner_via_pool_catalog`
all fail) — reached **only** when `text_only` is already false (a real
magnitude token or prose formula exists on this record's own line; a
`text_only` record with the identical owner-resolution failure was already
routed to `not-ingested` one branch up,
`class_feature_option_pool_record_not_held_by_engine`).

Tracing the control flow: `facts.class_feature_effect_wired.get(&unit.key)
== Some(&unit.book.as_str())` — the SAME probe that would ground this record
if it observed a delta — is checked at the very top of this arm, **before**
either the `text_only` or the magnitude-bearing branch is reached. By the
time either branch runs, the probe has already confirmed **no delta was
observed for this specific record's key**, regardless of whether the record
carries its own magnitude token. So the underlying fact this classifier
already holds — "the engine's own effect-wired table has no entry for this
unit's key" — is **identical** for the `text_only` sibling (already
`not-ingested`) and the magnitude-bearing case (previously `unknown`). The
only difference between the two branches is `magnitude_token_count`/
`carries_prose_magnitude`, a fact about this record's own corpus line, which
has no bearing on whether the ENGINE holds a record for it.

**Verdict: measured, not expressible**, for the whole 3,052 — same
disposition as Shapes 1/2. **Fixed this cycle:** status →
`not-ingested`, matching the disposition its own `text_only` sibling already
carries, with a distinct evidence string
(`class_feature_option_pool_record_with_magnitude_not_held_by_engine`) so the
two shapes stay separately re-derivable.

**Separately, a real research opportunity exists inside this population** —
distinct from the classification-correctness question above, and not
attempted this cycle:

```
$ jq -r '.units[]|select(.evidence=="class_feature_group_names_no_class_at_all")|.corpus_key' docs/work-inventory.json | sed 's/ ~ .*//' | sort -u | wc -l
1128
```//distinct group prefixes inside the 3,052-unit population, pre-fix inventory

Only **2** pools are registered in
`src/rules_core/class_feature_pool_catalog.rs::REGISTERED_POOL_GROUPS`
(`"Rogue Talent"`, `"Rage Power"`) against these 1,128 distinct unmatched
group prefixes. Widening that catalog would let some of this population
resolve a real *owner* (and, for units the pool-catalog-holds check reaches,
a real explanation id) rather than landing on the generic not-ingested
finding this cycle applies uniformly — genuine future reclassification work,
named here rather than attempted blind. `not-ingested` is still the
factually correct disposition for every one of the 3,052 today: it is not a
carve-out, it is what the engine's own tables currently prove.

## Summary — before any count moved

| Shape | Count | Cause | Disposition this cycle |
|---|---:|---|---|
| 1: Feat, magnitude + no consumer | 519 | instrument asymmetry | reclassify → `ingested-magnitude` |
| 2 (computed/derived/static subset): Feat/Equipment/EquipmentModifier text_only, closure has magnitude | 309 | instrument asymmetry | reclassify → `ingested-magnitude` |
| 5: Spell, no level, closure has magnitude | 26 | instrument asymmetry | reclassify → `ingested-magnitude` |
| 3: ClassFeature, unresolved owner, magnitude-bearing | 3,052 | instrument asymmetry (disposition); real research debt (owner resolution) | reclassify → `not-ingested` |
| 2 (display subset): Feat/Equipment/EquipmentModifier, genuinely empty record | 270 | genuinely empty corpus record | stays unclassifiable, renamed `unmeasurable` |
| 4: Feat, served description is a placeholder marker | 48 | content-integrity gap (upstream pipeline) | stays unclassifiable, renamed `unmeasurable` |
| **Total** | **4,224** | | |

**No count moved before this analysis.** Every reclassification in
`AT-33-E4-002`'s receipt traces back to one of the rows above, and every row
states the concrete evidence (a probe result already computed, a
structural proof about `wiring_class::signals`, or an explicit absence of
any existing honest status) rather than a redefinition of what "measured"
means.
