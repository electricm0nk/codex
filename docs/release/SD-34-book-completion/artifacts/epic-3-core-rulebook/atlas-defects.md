---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-27
---

# Atlas defects — Epic 3 (Core Rulebook)

Per `decisions.md §2`: "Any remaining step discovered that the atlas did not predict is a
DEFECT IN THE ATLAS." An empty file would be an excellent result; this file is not empty.
Each entry names the discovery, its `correction` retro event, and the atlas re-derivation that
followed.

## 1. Vacuous PCGen placeholder rows inside `class_feature_option_pool_record_not_held_by_engine`

**Discovered:** AT-34-E3-001, `class_feature_option_pool_record_not_held_by_engine` mechanism,
Cycle 3 (`AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`).

**What the atlas predicted.** `decisions.md §2`'s ten-bucket taxonomy treats every bucket-B
unit as a real content gap: a record the engine should hold and currently does not. Cycle 2's
own next-cycle plan named 5 sub-causes of this one mechanism's remaining 55 units, including
"vacuous placeholder rows (3)" with a flag that this shape needed a ruling before disposition.

**What the corpus actually holds.** `Empty Selection ~ Standard {Barbarian, Monk, Rogue}`
(`data/corpus/core_rulebook/class_feature/empty_selection/*.json`) are PCGen's own "no
selection" default rows for a `CHOOSE`-menu, not Pathfinder rules content: `data.description`
is JSON `null`, `data.raw_tokens` carries exactly three structural entries (`KEY`, `CATEGORY`,
`TYPE`) and nothing else — no `DESC:`, no mechanical token of any kind. There is genuinely
nothing to compute and nothing to display; "the engine does not hold a record for this" is not
what is wrong with these three rows — the corpus's own record carries no content for any engine
to hold in the first place. This is an unpredicted verdict shape: neither "real content not yet
held" (bucket B's own predicted meaning) nor any of the ten buckets' existing "cleared by"
column describes "nothing to clear, by the corpus's own construction."

**Disposition.** `status: "deferred-with-reason"` (bucket `X`, `decisions.md §2`'s own "deferred
with a stated reason ... cleared by revisiting the stated condition") — never `text-complete`
(no text exists to render) and never left in bucket B (there is no gap to place a record into).
A new, closed, named-key lookup —
`class_feature_pool_catalog::VACUOUS_PLACEHOLDER_CLASS_FEATURES` — matches only these 3 exact
keys, proven against the live corpus by
`vacuous_placeholder_rows_are_genuinely_empty_in_the_committed_corpus`
(`src/rules_core/class_feature_pool_catalog.rs`). Deliberately a named list, never a shape
predicate: this mechanism's own Cycle 2 receipt already recorded a near-miss where gating a
sibling rung on record SHAPE ALONE promoted 188 unrelated corpus-wide records before being
caught and reverted pre-commit; an independent corpus-wide scan for this same
"`description` null, `raw_tokens` ⊆ {KEY, CATEGORY, TYPE}" shape found 41 matches across 6
other books, none of them vacuous in the same way (witch hex sub-features, uncanny-dodge
trackers, BWBI wondrous-item slots, ...) — those 41 are untouched by this table.

**Retro event:** `docs/retro/events/sd34-at-34-e3-001.jsonl`, `correction`
(`--verified-by "python3 scripts/completion_atlas.py --book core_rulebook --check"`).

**Atlas re-derivation:** `python3 scripts/completion_atlas.py --book core_rulebook --check`
re-run at this cycle's own HEAD; `class_feature_option_pool_record_not_held_by_engine`'s share
of bucket B drops by exactly 3 (55 → 52), and the 3 units now report under bucket `X`.

## 2. The "no description, structural tokens only" shape is 517 of 51,482 records, and it is not one thing

**Discovered:** orchestrator verification of defect 1, 2026-08-27. Defect 1's disposition is
correct and is not changed by this entry. This is a **different** defect at a larger scale.

**What defect 1 established.** Three `Empty Selection ~ Standard {Barbarian, Monk, Rogue}` rows
carry `description: null` and only structural tokens, and were correctly moved to bucket `X`
behind a **named-key** list rather than a shape predicate — because gating on shape alone had
already, in this same mechanism, promoted 188 unrelated records before being caught pre-commit.

**What a corpus-wide re-derivation shows.** That shape is not rare and is not confined to one
book or kind:

```bash
# description is null AND raw_tokens carry only KEY / CATEGORY / TYPE
python3 - <<'PY'
import json, glob, collections
vac = collections.Counter()
for f in glob.glob('data/corpus/*/*/**/*.json', recursive=True):
    if f.endswith('LICENSE.json'): continue
    d = json.load(open(f)); data = d.get('data') or {}
    if data.get('description') is not None: continue
    names = {t.get('key') for t in (data.get('raw_tokens') or []) if isinstance(t, dict)}
    if names and names <= {'KEY', 'CATEGORY', 'TYPE'}: vac[f.split('/')[2]] += 1
print(sum(vac.values()), 'of 51,482'); print(vac.most_common())
PY
```

**517 of 51,482** corpus records, across at least 12 books and 8 kinds — `feat_generic` 248,
`language` 111, `race_trait_generic` 74, `class_feature` 41, `race_generic` 14, `ability` 10,
`monster_ability` 8, `template` 5. The Core Rulebook's own share is **38 of 6,701**, of which
defect 1 dispositioned **3**.

**The shape has at least three distinct meanings, and they need different verdicts.** Sampled
directly from the corpus:

1. **Menu placeholders** — `Empty Selection ~ Standard Barbarian`, `Push Selection ~ Sting`,
   `Standard Demilich`. Nothing to render, by the source's own construction. Defect 1's
   disposition (`X`, named-key) is right for these.
2. **Pointer rows whose content lives elsewhere** — `Duergar ~ Stability`,
   `Triaxian ~ Keen Senses`. Re-derived: `Stability` has **14 of 19** records corpus-wide
   carrying a real description, `Keen Senses` **11 of 21**. The rules text exists; this row
   grants it. These are a cross-record ownership question, the same shape as the `companion`
   mechanism's own 14 familiar-pool units — not vacuity.
3. **Genuine content gaps** — `Witch Hex ~ Hag's Eye` (`advanced_players_guide`,
   `class_feature`) is **1 of 1** records corpus-wide bearing that name and carries **no
   description at all**. It is a real published hex with real rules text. Nothing in the corpus
   holds it.

**Why this is an atlas defect and not just work.** The atlas's `X` bucket means *deferred with a
stated reason*, cleared by revisiting that reason. Meaning 3 is not deferred — it is **absent**,
and no bucket's "cleared by" column describes "the corpus itself never captured this record's
content." Meaning 2 is a cross-record ownership shape the atlas also does not name.

**It also sharpens the bundle's headline claim.** `decisions.md §2b` and `README.md §8` state that
ingestion is complete, evidenced by 100% of units carrying a real `source_file` and `source_line`.
That evidence is about **sourcing**, and it holds. It does not establish that every sourced record
carries its **content**, and meaning 3 is a case where it does not. The two claims are different
and should not be read as one.

**Disposition — not settled here.** No unit is reclassified by this entry. What it requires:

- a per-record split of the 517 into the three meanings, by evidence rather than by shape
  (shape alone is exactly what the 188-record near-miss proved unsafe);
- meaning 3's population named with a count — that is the number that matters, and it is
  currently unknown;
- AT-34-E5-002's capability register to carry whatever meaning 3 turns out to need, since
  recovering un-captured content is a corpus-extraction capability this bundle has not built.

**Retro event:** `docs/retro/events/sd34-orchestrator-verify.jsonl`.

## 3. "Grant-token-only" rows are a THIRD no-content shape, distinct from defects 1 and 2, and are 461 of 51,482

**Discovered:** AT-34-E3-001, `companion_absent_from_core_rulebook_companion_tables` mechanism,
cycle 3 (this cycle's own build of Shape 8 cross-book ownership for the 14 familiar-pool rows).
The mechanism-specific direction for this cycle required recording this shape here before ever
calling it excluded-by-design, rather than silently leaving it as a bare bucket-B count.

**What the corpus actually holds.** The 12 `Base Companion ~ …` / `Companion ~ …` rows this
cycle did NOT close (`Base Companion ~ Animal Companion`, `Base Companion ~ Special Mount`,
`Companion ~ Ability Score Increase`/`Bonus Tricks`/`Devotion`/`Evasion`/`Improved Evasion`/
`Link`/`Multiattack`/`Share Spells`/`Spell Resistance (AC)`/`Spell Resistance (SM)`) carry
`description: null` and a `raw_tokens` set of exactly `KEY`, `CATEGORY`, and one-or-more
`ABILITY:` grant tokens — **no `TYPE:`, no `DESC:`, no `BONUS:`**. `Base Companion ~ Animal
Companion` (`cr_abilities_companion.lst:46`) is the clearest example: it carries **eleven**
separate `ABILITY:Companion Class Feature|AUTOMATIC|…` tokens, each routing to a real,
separately-shipped ability (`Animal Companion ~ AC Bonus`, `~ Stat Bonus`, `~ Bonus Tricks`,
...) — this row is PCGen's internal dispatch plumbing for the Animal Companion class feature,
not a rules statement with content of its own.

**Why this is a THIRD shape, not defect 1 or defect 2 again.** Defect 2's own query
(`description is null AND raw_tokens ⊆ {KEY, CATEGORY, TYPE}`) does **not** match these 12
rows — they carry no `TYPE:` token at all, and they carry `ABILITY:` tokens defect 2's query
never looks for. Re-derived corpus-wide with the actual predicate this shape needs
(`description` null AND at least one `ABILITY:` token AND no `TYPE:`/`DESC:`/`BONUS:` token):

```bash
python3 - <<'PY'
import json, glob, collections
vac = collections.Counter()
total = 0
for f in glob.glob('data/corpus/*/*/**/*.json', recursive=True):
    if f.endswith('LICENSE.json'): continue
    d = json.load(open(f)); data = d.get('data') or {}
    raw = data.get('raw_tokens') or []
    keys = [t.get('key') for t in raw if isinstance(t, dict)]
    if 'ABILITY' in keys and not any(k in ('TYPE', 'DESC', 'BONUS') for k in keys):
        total += 1; vac[f.split('/')[2]] += 1
print(total, 'of 51,482'); print(vac.most_common())
PY
```

**461 of 51,482** corpus records, across at least 26 books — `advanced_players_guide` 136,
`bestiary_4` 63, `beastiary` 48, `ultimate_psionics` 31, `core_rulebook` 31 (of which this
mechanism's 12 are one sub-set; the other 19 are `core_rulebook` rows of other `kind`s, e.g.
`race_trait`/`class_feature`, out of this mechanism's scope). Not sampled exhaustively this
cycle — the population is stated, the per-record disposition is not.

**Disposition — not settled here, same discipline as defect 2.** These rows are real PCGen
plumbing (every one this cycle checked routes to real, separately-shipped content, unlike
defect 2's meaning-1 menu placeholders which route to nothing) — they are closer to a
DIFFERENT verdict shape than either "deferred with a stated reason" (bucket `X`) or "real
content not yet held" (bucket B's own predicted meaning), because there is genuinely no rules
text for this specific row to carry; its only job is to fan out to rows that DO carry text and
that already ship. None of the ten buckets names "this record's job is dispatch, not content."
Reclassifying by shape alone here risks the exact near-miss defect 1's own investigation
recorded (188 unrelated records promoted before being caught pre-commit) without a per-record,
corpus-wide check this cycle does not have time to run safely. Left `engine-does-not-hold`
(bucket B) for this mechanism's 12 core_rulebook rows rather than force a shape-based
reclassification through on trust.

**Retro event:** `docs/retro/events/sd34-at-34-e3-001.jsonl`, `deferral`.
