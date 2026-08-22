---
canonical: true
wave: 30
last_amended: "wave 31 — see 'Wave 31 addendum' at the top of the body below; the 46-group partition itself is UNCHANGED (no new evidence codes discovered), only dispositions/counts within existing groups were corrected"
supersedes: "wave 28 (+ wave 29 addendum) six-pile framing — see 'What changed since wave 28' below"
purpose: >
  The complete inventory. Wave 28 mapped six piles and left 298 units in no lane at all; naming that
  gap did not close it. Wave 30 finishes the inspection: every one of the 24,914 not-done units is
  now assigned to exactly one of 46 evidence-code-keyed groups, verified by a purpose-built tool
  (not hand arithmetic), with zero uncovered and zero overlap, and every group carries a todo entry.
  This document is the ranked map of what those 46 groups actually are and what to do about each.
board_at_close: "13,458 / 38,372 (35.07%) — UNCHANGED from wave 29 close. This wave banks nothing."
---

# THE BOX — SD-31 wave 30 complete inventory

## Wave 31 addendum — what this wave proved or disproved about the 46 groups

**The partition itself is untouched: still 46 groups, still 24,914 population, still `uncovered: 0,
overlap: 0`.** Wave 31 was a root-cause and compute-shape measurement wave (see
`MEASURE-TWICE.md`, the wave's primary deliverable) and did not add, remove, split, or re-key any
group. What changed is depth within specific groups, and one important correction to how the groups
relate to each other.

- **`class_feature_owner_matched_by_name_but_record_not_held_by_engine` + the `data.class` field
  underneath it (S8's population).** The anchor (11,502) re-confirms exactly. The numerator, flagged
  since wave 30 as "the 71.4% breakdown itself was not re-run," WAS re-run this wave: **8,243 of
  11,502 (71.7%)**, not 8,210/71.4%. New: only **2,360 of the 8,243 (28.6%) are cleanly
  prefix-remappable plumbing**; the remaining **5,883 are a MIX of category-label plumbing and
  genuine unmodelled-class content**. Do not read this group's whole count as "plumbing" going
  forward — see the next point.
- **`class_feature_of_unmodelled_corpus_class:*` (2,453 units, the L1/T12 "genuine missing
  mechanism" group) overlaps the group above by a MEASURED 1,354 units (floor) to ~2,124 (scaled),
  not the ~532 wave 30 assumed.** This is the single most consequential correction this wave made:
  these two groups were treated as disjoint contributions to a plumbing-vs-complexity split, and they
  are not. The overlap units are named, real classes (Psychic, Vigilante, Medium, Magus, Shifter,
  Kineticist, Spiritualist, Occultist) that need to be modelled, not relabeled. Full arithmetic:
  `MEASURE-TWICE.md` §2, filed as `sweeps.md` S20 (CLOSED, measured).
- **`race_trait_race_not_modelled` (S3, 2,472) and `no_compiled_rule_set_for_book` (L10, 422)**
  re-confirm exactly, unchanged.
- **`class_feature_group_names_no_class_at_all` (S12)** re-confirms at 1,321 of 12,114 (not the
  intermediate 1,377 some prior citations carried).
- **The `no_explanation_id_and_no_diagnostic_names_this_feature` group (G3/L6, 3,320)** is now cross-
  referenced against the compute-shape families (`MEASURE-TWICE.md` §3): its ~2,287
  formula-interpretable-but-unconsumed sub-population is not evenly hard — some of it (ability-
  modifier-derived, clamped-scaling) has fully-covered grammar and only needs consumer wiring; some
  of it (`classlevel(...)`-derived, 211 units program-wide) is blocked on a real interpreter bug
  (`defects.md` D2, reach now measured at 1,957/523 names, corrected from a first-filed 2,340/616).
- **The L20 residual (6,966 units, 32 families, NAMED not EXAMINED)** is now partially informed by
  the compute-shape census: a compute library, even fully built, closes at most **3,201 of the
  24,914 not-done population (12.8%)** — meaning most of L20's 6,966 units are NOT formula-shaped
  work at all and will not be touched by any compute-library investment, however good.
- **A 447-unit gap in a prior wave's "absent from X table" narrative was corrected**, not a gap in
  THE-BOX's own partition: `race_trait_absent_from_race_traits` (238), `class_absent_from_ClassId_ALL_
  and_book_class_id_enums` (152), and `race_absent_from_the_character_creation_roster` (57) were
  always inside the 46-group total, just narrated separately from a wave-31 lane's own 6-family
  "onboarding backlog" summary, which silently excluded them. Restated in full in `MEASURE-TWICE.md`
  §1's T9 row.
- **A wave-31 lane attempted to close `sweeps.md` S2 (generalizing the Monk shape beyond classes) on
  a false architectural claim and was reviewed GAMED; reverted to PARTIAL.** A genuine new
  Monk-shaped instance was found live in equipment (APG's `Equipmods` table-variant gap, ~35 records,
  mostly mitigated already) — the sweep's own open question (does this shape recur outside the 3
  enum-mediated kinds already checked?) is now MORE open, not less, than wave 30 left it. Full
  account: `MEASURE-TWICE.md` §4, `sweeps.md` S2.
- **Board confirmed unchanged**: `md5sum docs/work-inventory.json` = `d64ddfc677fd1683f5b7638889a25c54`,
  identical to wave 30's close and wave 31's own start and end. Nothing banked, nothing reclassified,
  no regen run.

**One retraction, named loudly per this wave's own instruction.** The wave-31 dispatch brief's own
cited figure — "33,830 formula tokens reduce to 1,049 shapes, the top 15 covering 80%" — has its
first two components (33,830; 14,752 distinct raw values) reproduced exactly, but the third (1,049
shapes / 80%) did NOT reproduce under any normalisation two independent measurement lanes tried.
**Treat "1,049 shapes / top-15 / 80%" as retracted**, including everywhere it may already have been
cited as settled fact in downstream planning (see `MEASURE-TWICE.md` §6).

Full detail, every corrected number's reproduction command, the twelve root-cause shapes, the ten
compute-shape families, and the ranked automation candidate list: `MEASURE-TWICE.md` (this wave's
primary deliverable — read that first, this addendum is the map-maintenance summary).

---


## Answer to the operator's question, first: is the inspection now complete?

**Structurally, yes — provably, not by assertion.** Every one of the 24,914 not-done units is
covered by exactly one of 46 named groups; the coverage tool reports **uncovered = 0, overlap = 0**,
reproduced by the exact command below against the live, unmodified `docs/work-inventory.json`.
Every group carries a `todo/*.md` entry — `--strict` exits 0.

```
python3 scripts/coverage_ledger.py --groups docs/release/SD-31-corpus-closure-grind/artifacts/w30-coverage-table.json --strict
population (not-done units considered): 24914
covered by >=1 group:                    24914
uncovered (0 groups):                     0
overlap (>1 group):                        0
```

**But "covered" and "deeply examined" are not the same claim, and conflating them is exactly the
gaming shape this wave was warned against (Decision 1a).** Be precise about which of the 46 groups
got which treatment this wave:

- **14 groups (17,948 units, 72.0%)** were individually examined this wave or a prior wave, with a
  named root cause, a general fix, and — where the fix is cheap — an exact sub-costed lever. This
  includes the entire 141-unit "never examined by any wave" population the operator's own directive
  named (§2 below, groups R1–R5/D1–D3/H1–H2), the corrected class_feature G1/G3/G6 populations
  (9,791 units — G1 3,064 + its sibling branch 3,378 + G3 3,320 + G6 29), the race-trait
  key-matcher sweep's first-ever corpus-wide count (2,472), and the
  book-onboarding gate (422).
- **32 groups (6,966 units, 28.0%)** are real, reproducible, evidence-code-exact corpus-wide counts
  — genuine structural coverage, not a placeholder — but were **not individually root-caused this
  wave**. They are named, sized, and filed under one consolidated lever (`levers.md` L20) with an
  explicit recommendation for how a future wave should be split (by evidence-code family, not by
  `kind` — see "What changed" below for why kind-scoped lanes are the failure mode that produced the
  298-unit gap in the first place).

**So: nothing is face-down anymore. A meaningful fraction (28.0%) is face-up but not yet read in
detail.** That is a true, checkable, non-gamed status — worse than "100% examined," better than
wave 28's "295 units nobody looked at," and it is reported as exactly that rather than rounded up.

---

## 0. Board state (confirm unchanged)

```
md5sum docs/work-inventory.json
d64ddfc677fd1683f5b7638889a25c54  (matches wave-29 close, matches wave-30 start, matches wave-30 close)
```

No lane wrote production logic, reclassified a unit, ran the guarded regen, or touched
`docs/work-inventory.json`. Confirmed independently by two adversarial reviewers across all 6
lanes: every lane's git diff is new files under `artifacts/`/`todo/` (documents) plus, for lane 1,
two new untracked files under `scripts/` (a tool, not production logic — `docs/work-inventory.json`
untouched by it). **Board movement this wave = 0.**

```
python3 -c "
import json,sys,collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
v=lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
c=collections.Counter(v(u) for u in U)
print(len(U), c.most_common())
"
# -> 38372 [('not-started', 18645), ('done', 13458), ('unmeasurable', 3763),
#           ('in-progress', 1231), ('held', 1230), ('deferred', 45)]
```

`13,458/38,372 = 35.07%`. Not-done = `18,645 + 3,763 + 1,231 + 1,230 + 45 = 24,914`. Matches the
wave dispatch's frozen denominator exactly.

---

## 1. Methodology — how "every unit examined" is actually proven, not asserted

Wave 28 reconciled six lane piles by hand: sum each lane's reported count, subtract the hand-found
1,212-unit overlap, compare to the population, and hope nothing was missed. It missed 298 units.
Hand arithmetic across free-text lane reports cannot be checked by anyone who did not redo it.

Wave 30 builds a **tool** instead (`scripts/coverage_ledger.py`, lane 1, extended by this
integration cycle after adversarial review found it could not express 4 of lane 6's 6 findings — it
had a `match` schema with no key for `evidence`, `visible`, or `origin`, only `kind`/`book`/
`status`/`wiring_class`/`verdict` plus regex on `id`/`name`/`corpus_key`/`type_facet`/`source_file`;
`evidence_regex` + `visible`/`origin` list-match were added this cycle, with new tests, 22/22
passing). It takes a **classification table** — plain JSON, not code, not prose — naming every
group's exact match predicate and its `todo/*.md` entry, and reports population, per-group counts,
uncovered units (named by id, not just counted), overlap (named by id and by which groups), and
which groups lack a todo entry. Anyone can re-run it; it cannot be talked past.

**Why the partition is keyed on `evidence`, not `kind`+`verdict`.** The adversarial review of wave
30's own lane 6 flagged its (kind, verdict) 33-cell grid as the Decision-1a gaming shape — "a group
defined as everything, split by two fields already on the unit" — because `kind` and `verdict` carry
no diagnostic content; every unit has exactly one of each by construction, so partitioning on them
proves nothing about whether anyone looked. `evidence` is different: it is the specific diagnostic
string the engine itself attaches explaining WHY a unit is not done (`no_compiled_rule_set_for_book`,
`race_trait_race_not_modelled`, `class_feature_owner_matched_by_name_but_record_not_held_by_engine`,
...) — it is literally what every prior wave's G1/G3/G6/F1/F2 groups already were, just never
formalized into one exhaustive table. 219 distinct evidence strings exist across the 24,914 not-done
units; this wave clusters them (by exact match or regex family, e.g. every
`class_feature_of_unmodelled_corpus_class:<X>` for 90-odd unmodelled classes) into 46 groups that
partition the population exactly, verified by the tool, not by hand:

```
python3 -c "
import json, re, collections, sys
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
inv=json.load(open('docs/work-inventory.json'))
units=[u for u in inv['units'] if (u.get('book') or 'unknown') not in frozenset(P.EXCLUDED_BOOKS)]
notdone=[u for u in units if P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))!=P.DONENESS_DONE]
print(len(notdone), 'not-done units,', len(set(u.get('evidence') for u in notdone)), 'distinct evidence codes')
"
# -> 24914 not-done units, 219 distinct evidence codes
```

The full 46-group table, with every match predicate and every todo entry, lives at
`docs/release/SD-31-corpus-closure-grind/artifacts/w30-coverage-table.json` — re-run it yourself:

```
python3 scripts/coverage_ledger.py --groups docs/release/SD-31-corpus-closure-grind/artifacts/w30-coverage-table.json --strict
echo $?   # 0
```

---

## 2. The 46 groups, ranked by size, with disposition

**Column key**: *Depth* = EXAMINED (root-caused this wave or a prior wave, with a real fix path) or
NAMED (real corpus-wide count, not yet root-caused — filed under L20). *Todo* = the exact
`todo/*.md` entry.

| Group (evidence code / family) | Count | Depth | Disposition | Todo |
|---|---:|---|---|---|
| `class_feature_owner_matched_by_name_but_record_not_held_by_engine` | 3,378 | EXAMINED | Sibling branch to G1 below — SAME `REGISTERED_POOL_GROUPS` catalog gate, owner class name DOES match. Combined real L5 population is 6,442, not the previously-filed ~10,000/3,347. | `levers.md` L5 |
| `no_explanation_id_and_no_diagnostic_names_this_feature` (G3) | 3,320 | EXAMINED | Real parser run (not a regex proxy): ~2,287 formula-interpretable/no consumer wired, 3 genuinely refuse, ~864 no formula token at all, 167 no corpus record. Corrected population from wave 28's stale ≥1,764/2,583. | `levers.md` L6 |
| `class_feature_option_pool_record_not_held_by_engine` (G1) | 3,064 | EXAMINED | 1,017 no corpus record, 391 empty description, 1,656 real-prose (the Ruling §18 OPEN/EXCLUSIVE population) — of which 488 units/17 names now have a proposed axis (corrected from a first-filed 546/20; 3 names were wrongly counted, one is `defects.md` D10's false positive). | `levers.md` L5 |
| `class_feature_group_names_no_class_at_all` | 2,551 | EXAMINED (evidence-honesty) | 471 of the 1,971 matched-with-corpus-record units carry an unambiguous TYPE-token class signature contradicting the evidence string's own "no class at all" claim — corrected from a first-filed 1,377/12,114 (5-code table sums to 1,321, not 1,377). Not reclassified; may still lack a real consumer once re-owned. | `sweeps.md` S12 |
| `race_trait_race_not_modelled` | 2,472 | EXAMINED (first corpus-wide count) | The 6-wave-old compound-key matcher gap (S3) finally has a real number. Not fixed — a reclassification project, out of this no-banking wave's scope. | `sweeps.md` S3 |
| `class_feature_of_unmodelled_corpus_class:*` (regex family, ~90 classes) | 2,453 | EXAMINED (family-level) | class_feature belongs to a class outside the 34-dispatch universe (Vigilante 196, Medium 147, Magus 127, Occultist 125, Psychic 125, Mesmerist 114, Shifter 110, Kineticist 99, Spiritualist 86, Aegis 131, + ~80 more, mostly Ultimate Psionics/occult/3pp classes at 1-30 each). Same lever as the 152-unit `class` kind itself. | `levers.md` L0/L1 |
| `equipment_table_entry_with_corpus_magnitude` | 775 | NAMED | Equipment in engine tables with a real corpus magnitude, not-done for a reason not yet individually read. | `levers.md` L20 |
| `spell_key_absent_from_spell_list` | 726 | NAMED | Spell ingestion/onboarding gap, per-book. | `levers.md` L20 |
| `companion_absent_from_*` (regex family, 8 books) | 726 | NAMED | Companion book-by-book onboarding backlog: ultimate_wilderness 248, advanced_players_guide 203, ultimate_magic 138, core_rulebook 86, advanced_race_guide 18, bestiary_4/5 2+2, book_of_the_damned_volume_1 29. | `levers.md` L20 |
| `text_only_but_corpus_record_carries_no_description_to_show_a_player` | 632 | NAMED | Text-only record, corpus row has no player-showable description. | `levers.md` L20 |
| `monster_ability_absent_from_*` (regex family, 8 books) | 517 | NAMED | monster_ability book-by-book onboarding backlog: bestiary_4 191, bestiary_1 83, horror_adventures 65, ultimate_psionics 64, inner_sea_bestiary 38, bestiary_2 49, inner_sea_world_guide 16, inner_sea_gods 7, bestiary_3 4. | `levers.md` L20 |
| `in_catalog_with_corpus_magnitude_but_no_observed_consumer` | 516 | NAMED | Feat in catalog, real magnitude, no observed engine consumer. | `levers.md` L20 |
| `feat_key_absent_from_catalog` | 480 | NAMED | Feat ingestion/onboarding gap, per-book. | `levers.md` L20 |
| `no_compiled_rule_set_for_book` | 422 | EXAMINED | Book-onboarding gate. `adventurers_guide` already registered (wave 29, 971 not-done units now gated on OTHER levers, not this one — corrected from a filed 973/973). Remaining: `inner_sea_magic` 335, `inner_sea_temples` 64 (no `data/corpus` tree at all), `inner_sea_taverns` 20, `inner_sea_faiths` 3 (previously unnamed). | `levers.md` L10 |
| `monster_ability_resolve_returned_a_real_record` (suffix family, 8 books) | 263 | NAMED | Monster_ability resolves via a book resolver but is held/in-progress for another reason. | `levers.md` L20 |
| `monster_*_resolve_returned_a_real_stat_block` (suffix family, 7 books) | 253 | NAMED | Monster resolves via a book resolver but is held/in-progress for another reason. | `levers.md` L20 |
| `equipment_key_absent_from_equipment_tables` | 174 | NAMED | Equipment ingestion/onboarding gap, per-book. | `levers.md` L20 |
| `monster_ability_has_no_engine_table` | 362 | NAMED | monster_ability kind has no engine table for this book at all. | `levers.md` L20 |
| `spell_list_entry_with_resolved_level` | 310 | NAMED | Spell list entry with a resolved level, not-done for another reason. | `levers.md` L20 |
| `in_equipment_tables_and_corpus_record_carries_no_magnitude_token` | 276 | NAMED | Equipment in engine tables, corpus record carries no magnitude token at all. | `levers.md` L20 |
| `class_absent_from_ClassId_ALL_and_book_class_id_enums` | 152 | EXAMINED (prior wave) | The wave-27 157-class census family (prestige/structurally-non-PC/untabled-base/book-gated/CRB-NPC/Ninja-Samurai). Already thoroughly examined; see `sweeps.md`'s wave-27 table. | `sweeps.md` wave-27 table + `levers.md` L0/L1 |
| `race_trait_applied_by_the_race_corpus_but_no_verified_consumer` | 234 | NAMED | race_trait applied by corpus, no verified consumer. | `levers.md` L20 |
| `companion_resolve_returned_a_real_record` (suffix family, 6 books) | 56 | NAMED | Companion resolves via a book resolver but is held/in-progress for another reason. | `levers.md` L20 |
| `race_trait_absent_from_race_traits` | 238 | NAMED | race_trait ingestion/onboarding gap. | `levers.md` L20 |
| `race_absent_from_the_character_creation_roster` | 57 | EXAMINED | The full wave-30 race census: R1 6 (book missing from `RACE_CORPUS_BOOKS`) + R2 7 (book registered, never transcribed) — both cheap onboarding; R4 43 (non-PC monster/companion/eidolon content reusing RACE syntax) + R5 1 (ARG Race Builder meta-tool) — both need an operator ruling. | `levers.md` L11 (13) + `blocked.md` B12 (43) + B13 (1) |
| `in_catalog_and_corpus_record_carries_no_magnitude_token` | 65 | NAMED | Zero-magnitude text-only feat, in catalog. | `levers.md` L20 |
| `companion_content_has_no_engine_table` | 43 | NAMED | Companion, no engine table for its book at all. | `levers.md` L20 |
| `engine_diagnostic:class_feature.*` (regex family) | 43 | EXAMINED | 38 units (D1+D2) need three new engine mechanisms (companion sub-engine, bardic-performance engine, generic sub-choice chooser). 5 units (D3, PU Unchained Barbarian/Rogue) are a join gap, not a capability gap — magnitude already computed under a sibling explanation id. | `levers.md` L13 (38) + L14 (5) |
| `feat_served_description_is_a_placeholder_marker_not_prose` | 38 | NAMED | Feat's served description is a placeholder, not real prose. | `levers.md` L20 |
| `spell_list_entry_with_description_but_no_corpus_level` | 33 | NAMED | Spell has description, no corpus level. | `levers.md` L20 |
| `class_feature_no_dedicated_magnitude_id_matched_the_record_slug` (G6) | 29 | EXAMINED | Population confirmed unchanged at 29 (pathfinder_unchained 24 / core_rulebook 3 / advanced_players_guide 2). Real split: 5 cheap (4 allowlist-word-away, 1 dot-namespace-miss), 10 two-words-away, 14 no-candidate-at-all. | `levers.md` L18 (5) + `sweeps.md` S15 |
| `monster_absent_from_*` (regex family) | 28 | NAMED | Monster book-by-book onboarding backlog, incl. `monster_absent_from_MonsterId_ALL`. | `levers.md` L20 |
| `spell_list_entry_with_no_corpus_level_and_no_description` | 26 | NAMED | Spell, neither corpus level nor description. | `levers.md` L20 |
| `feat_effect_probe_observed_computed_delta` | 23 | NAMED | Feat probe observed a real delta, still held. | `levers.md` L20 |
| `explanation_id_observed_in_a_real_computation` | 20 | NAMED | class_feature explanation id observed in a real computation, still held. | `levers.md` L20 |
| `explanation_id_observed_after_known_magnitude_suffix_strip` | 17 | NAMED | class_feature explanation id observed only after suffix strip, still held. | `levers.md` L20 |
| `spell_effect_probe_observed_computed_delta` | 110 | NAMED | Spell probe observed a real delta, still held. | `levers.md` L20 |
| `race_trait_states_a_universal_sheet_modifier_pending_compute` | 8 | NAMED | Universal sheet modifier, compute pending. | `levers.md` L20 |
| `monster_ability_held_and_corpus_record_carries_real_description` | 10 | NAMED | monster_ability held, real description present. | `levers.md` L20 |
| `race_offered_by_the_roster_but_no_pilot_compute_magnitude_consumer` | 3 | EXAMINED | Aasimar, Tiefling, Changeling — ingested and roster-visible, need `race_magnitude_consumer_races` registration. | `levers.md` L12 |
| `engine_diagnostic:ultimate_campaign::feat_tables::DEFERRED_WITH_REASON` | 2 | EXAMINED | 2 UCA feats, confirmed upstream-splice `.MOD BENEFIT:` row. | `levers.md` L21 |
| `superseded_byte_identical_reprint_first_print_owns_it_decisions_13_19` | 2 | EXAMINED (already ruled) | Disposition already settled, decisions.md §13/§19. Not a gap. | decisions.md 13/19 |
| `equipment_effect_probe_observed_computed_delta` | 2 | NAMED | Equipment probe observed a real delta, still held. | `levers.md` L20 |
| `race_trait_record_loaded_but_never_applies` | 2 | NAMED | race_trait loaded, never applies. | `levers.md` L20 |
| `class_modelled_but_no_observed_delta_on_the_rendered_snapshot` | 2 | NAMED | Class modelled, no observed delta on rendered snapshot. | `levers.md` L20 |
| `class_feature_probe_observed_a_delta_attributable_to_this_record` | 1 | NAMED | class_feature probe observed a delta, still held. | `levers.md` L20 |

**Sum check**: every count above is the tool's own `count` field for that group; the tool's own
`population`/`covered_distinct`/`uncovered_count` fields (24,914 / 24,914 / 0) are the authoritative
total, not a manual re-addition of this table (46 rows is error-prone to hand-sum; the tool doesn't
have that problem).

---

## 3. What changed since wave 28 — and why the six-pile framing is retired

Wave 28's six piles (class_feature not-started 11,971 / unmeasurable 4,270 / race_trait 2,954 /
monster·companion·monster_ability 2,258 / spell·feat·equipment_modifier·equipment 4,380 / sweeps)
were assembled by splitting lane assignments along `kind` boundaries. **That split is the direct
cause of the 298-unit gap the operator caught**: `class_feature`'s not-started slice got a lane;
its held and deferred piles did not, because "not-started" was the big number and held/deferred
were small ones that lost the assignment. `race` got no lane at all, for the same reason — 60 units
is small next to 11,971. Small piles keep losing to big ones under a kind-scoped split; this wave's
own dispatch names that as the root cause, not the individual lanes.

Wave 30 does two things differently. First, it dedicated a whole lane (lane 2) to exactly the
residual the operator named — the 141 units in no lane at all — and treated it as a first-class
census, not a footnote. Second, this integration cycle replaced the kind-scoped organizing
principle with the evidence-code partition (§1 above), which cannot leave a same-shaped gap: every
not-done unit has exactly one `evidence` string, so a table built by iterating the 219 distinct
strings (not the 11 `kind` values) cannot silently drop a small `kind`. **`levers.md` L20's own
recommendation makes this explicit for the next wave**: assign census lanes by evidence-code
family, not by `kind`.

The wave-28 six-pile detail (§2.1–§2.6 of the prior version of this document) is superseded by the
46-group table above, which is a strict refinement: every wave-28 pile's units are inside one or
more of the 46 groups (verified by the tool's population/coverage numbers matching exactly), and
every wave-28 finding that still holds (G1/G3/G6 corrected populations, the book-onboarding gate,
S3's race-trait matcher) is carried forward with its wave-30-corrected number, cited in §2 above.
Nothing from wave 28 was silently dropped; what changed is cited in `todo/`, not buried in prose.

---

## 4. Tools worth building, across the whole box

**The coverage ledger tool itself (`scripts/coverage_ledger.py`), now that it exists, should be the
required handoff format for every future census wave** — not a nice-to-have. Its own adversarial
review proved it can detect a real hole (fed a deliberately-incomplete table, it named the missing
units by id and `--strict` exited 1) and cannot be gamed by an empty predicate (fails closed, not
open). The one gap the review found — no match key for `evidence`/`visible`/`origin` — is fixed
this cycle (`evidence_regex`, `visible`, `origin`, 3 new tests, 22/22 passing).

**A TYPE-facet / group-name corpus-row triage tool** (wave 28 finding, still valid, still not
built) — the same underlying need recurs in the G1 pool-name census (444 real-prose names, 424-426
still unclassified on the OPEN/EXCLUSIVE axis) and the `class_feature_group_names_no_class_at_all`
TYPE-token cross-check (S12, 471 of 1,971 misfiled by evidence-string). Estimated at about half a
day, unchanged by review.

**Rejected — do not build**: `equipment_key_is_wired()` two-clause widen (closes 0, risks
fabricating a computed zero — §2.5 of the wave-28 version); the twice-run-diff fixture regression
test (provably blind to the S6 bug it was proposed for — see `sweeps.md` S6/`defects.md` D7).

---

## 5. Expensive but unavoidable

- **L0 — prestige-class entry-requirement gating.** Gates 77 of the 157 `class` units. No shortcut.
- **The formula interpreter itself (Ruling §20).** The real fix behind G3's 3,320 units (corrected
  this wave) and a large share of the L20 residual's held/in-progress buckets.
- **L13 — three new engine mechanisms** (companion sub-engine, bardic-performance engine, generic
  sub-choice chooser) for the 38-unit class_feature capability-gap family, named for the first time
  this wave with an exact count.
- **L1 — the ~90-class `class_feature_of_unmodelled_corpus_class:*` family** (2,453 units). Each
  unmodelled class needs its own chassis table before any of its features can move.
- **L20's 6,966-unit residual.** Real, structural, individually cheap-to-verify per group (each is
  one evidence code with an exact reproduction command) but there are 29 of them — genuine,
  unavoidable per-family verification work, not a single mechanism.

## 6. Probably not worth doing

- **Bulk-registering class_feature pool names or race_trait groups without per-group oracle
  verification.** Wave 28 found real false positives on small spot checks; wave 30's own G1 axis
  classification found 3 of 20 named groups received no real axis at all when first summarized —
  confirming the same caution applies to axis classification, not just registration.
- **Treating the DESC-tail formula-extraction experiment's 2,507 figure (`levers.md` L17) as a
  planning input.** It has a known, unguarded false-positive shape (a `PRE*` token in a `DESC` tail
  misread as a formula). Re-derive once the guard is built.

---

## 7. Needs operator ruling (consolidated)

New this wave: `blocked.md` B12 (43 units, non-PC race-syntax content), B13 (1 unit, ARG Race
Builder), B14 (10 units, "First Boon" — named wave 28, finally filed), B15 (Supersession Register
origin-ownership, gates the whole mechanism — fully formed in decisions.md §14, finally filed).

Corrected in scope, question unchanged: B7 (Core Essentials shared glossary — 566 not-done across 7
kinds, not 233 across 2), B8 (equipment_modifier `VISIBLE:NO` — 921 not-done across 8 kinds, not 504
in 1), B2 (race branch classification — narrowed to a precise 24-unit question with evidence already
assembled, not "unknown").

Reconfirmed exact: B4 (48), B5 (5). Not reproduced this wave (flagged, not corrected — the original
predicates were not available to re-run): B6 (310 filed / 344 by an unverified proxy), B9 (2 filed /
0 by an unverified proxy), B10 (30 filed / 34 by this wave's proxy), B11 (12 filed / 0 by an
unverified proxy).

Full text of every question: `docs/release/SD-31-corpus-closure-grind/todo/blocked.md`.

---

## 8. Wave-30 receipt (summary — full detail in `progress.md`)

Six lanes (5 census + 1 tool-build) plus two independent adversarial-review lanes. Zero units
banked, zero code touched in production paths, `docs/work-inventory.json` byte-identical
throughout. The coverage tool (built this wave, extended by integration after review) proves
structural completeness for the first time — 46 groups, 24,914 population, 0 uncovered, 0 overlap,
every group has a todo entry. 14 of 46 groups (17,948 units, 72.0%) were individually root-caused;
32 groups (6,966 units, 28.0%) are real, named, corpus-wide counts filed for a future evidence-code-
scoped wave under `levers.md` L20. `todo/` reconciled: 5 new sweeps (S12-S16), 4 new defects
(D10-D13), 4 new blocked items (B12-B15), corrected scope on 2 existing blocked items (B7, B8), 9
new levers (L11, L12, L13, L14, L15, L17, L18, L20, L21), 2 existing levers corrected (L5, L6), 1
existing lever's remaining scope corrected (L10). Full wave-30 receipt with every reproducible
command: `progress.md`.
