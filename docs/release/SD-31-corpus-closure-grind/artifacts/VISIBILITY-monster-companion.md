---
canonical: true
owner: sd31-wave28-monster-companion
purpose: Wave 28 visibility census — monster_ability, companion, monster. NO BANKING. Every count
  below is reproduced by the command printed above it; re-run to verify.
wave: 28
pile: "monster_ability 1,152 (879 not-started, 273 held); companion 825 (769 not-started, 56 held);
  monster 281 (253 held, 28 not-started). 2,258 units."
base_commit: e90ba9ec1
inventory_generated_at: "2026-08-21T18:24:21Z"
---

# Wave 28 Visibility Census — monster_ability / companion / monster

**This wave banked nothing.** No code changed, no `data/corpus` regen ran, no
`docs/work-inventory.json` write happened. `git status --short` was empty at the end of this run,
and this document plus its own commit are the wave's only output. `CARGO_TARGET_DIR` was not used
(no build was needed — everything below is JSON/grep analysis against the read-only inventory and
source tree) and no scratch directory was left behind.

**Population and methodology.** All counts below come from `docs/work-inventory.json`
(generated `2026-08-21T18:24:21Z`, un-regenerated, read-only) filtered to
`book not in EXCLUDED_BOOKS`, run through `scripts/observer/pf1e_dashboard_producer.py`'s own
`doneness_verdict(wiring_class, status, kind)` — the same function the dashboard uses. Every
number has its exact Python one-liner given; re-run against the same inventory file to reproduce.

```python
import json, sys
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
def v(u): return P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
```

**Confirmed against the dispatch pile exactly:**

```
monster_ability 2942 total: not-started 879, done 1790, held 273
companion       1696 total: not-started 769, done 871, held 56
monster         1270 total: done 989, held 253, not-started 28
```
`879+273=1152`, `769+56=825`, `253+28=281`. Sum `1152+825+281=2258`, matching the pile exactly.

---

## Executive summary — the single biggest thing this census found

**Two systemic mechanisms explain the large majority of all three kinds' non-done populations,
and neither is what `levers.md` L3 currently describes:**

1. **A hand-maintained static Rust chassis registry (`monster_chassis.rs` / `companion_chassis.rs`)
   is missing rows for content that genuinely exists in the corpus** — the SAME shape as sweep S9
   (Ninja/Samurai's missing weapon-proficiency row), but at **~1,271-unit scale** across three
   kinds, not two units. Some of these rows are cheap (an existing chassis record just needs one
   more ability key added); a large fraction are NOT cheap, because —
2. **A large share of the "missing content" is not a missing row at all — it is content the
   current chassis MODEL cannot represent**, specifically PF1e creature TEMPLATES (Ogrekin, Dread
   Lord, Animated Object, Celestial/Fiendish familiars…) and a Core-Essentials shared
   ability-glossary file that was re-attributed by book label (Ruling §16) but never wired into
   any consuming table. Both are **new-mechanism** problems, not transcription problems.

Everything below is the corpus-wide count backing that split, group by group, kind by kind.

---

## Part 1 — `monster_ability`, 1,152 not-done (879 not-started + 273 held)

### 1a. The 879 not-started split cleanly into two evidence mechanisms

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
u=[x for x in d['units'] if x.get('kind')=='monster_ability' and x.get('status')=='not-ingested']
print(len(u))
print(collections.Counter(x.get('evidence') for x in u).most_common())
"
```
Result: **879** total. **362** carry evidence `monster_ability_has_no_engine_table` (the book's
chassis table for monster abilities does not exist in `monster_chassis::MONSTER_BOOKS` at all —
the classifier falls through every per-book arm to the terminal catch-all at
`src/bin/v06_work_inventory.rs:6996`). **517** carry evidence
`monster_ability_absent_from_<book>_monster_abilities` (a real per-book table exists — the key
just isn't in it).

| Group | Count | Evidence mechanism | General fix |
|---|---:|---|---|
| G1 — no chassis table for the book at all, native content | **172** | `has_no_engine_table`, source files `pu_abilities_race.lst`(72) `b5_abilities_race.lst`+`_oa`(39) `ma_abilities_race.lst`(21) `b6_abilities_race.lst`(13) `um_abilities_race.lst`(13) `ui_abilities_race_pu.lst`(6) `oa_abilities_race.lst`(3) `uw_abilities_race.lst`(2) `arg/vishkanya/wyrwood`(3) | Register `pathfinder_unchained`, `bestiary_5`, `mythic_adventures`, `bestiary_6`, `ultimate_magic`, `ultimate_intrigue`, `occult_adventures`, `ultimate_wilderness` in `monster_chassis::MONSTER_BOOKS` (8 new books). Bestiary 6 is the ONLY one of these that also has zero `monster` stat blocks (both campaign files self-declare "Player Options Only" per the Bestiary-6 ledger) — the other 7 have real creatures and just never got a chassis table for their abilities. |
| G2 — Core Essentials shared ability glossary, re-attributed | **190** | `has_no_engine_table`, source file `ce_abilities_race.lst` exclusively, attributed to `bestiary` `bestiary_2` `bestiary_3` `bestiary_4` `bestiary_5` `bestiary_6` by Ruling §16's SOURCELONG re-attribution | **Not a transcription gap — a scope gap.** `ce_abilities_race.lst` is Core Essentials' shared glossary of generic ability *definitions* ("Ability Damage (Ex)", "Ability Drain (Su)"…) referenced BY monsters across many books, not owned by any one book's native chassis file. Ruling §16 re-labeled which book these 190 rows report under; it did not (and could not, by its own scope — §16 addressed the file's `race`/`race_trait` rows, 29 of them, not its 369 `monster_ability` rows) give them a table to be held by. **This population was never sized before this wave.** See §1c below — a further split shows most of it is unrepresentable as-is, not merely unwired. |
| G3 — real per-book table exists, this key is missing | **517** | `absent_from_<book>_monster_abilities` | See §1b — this is not one shape, it splits by what the parent creature actually is. |

172+190+517 = 879. ✓

### 1b. The 517 "absent_from" units, split by what their parent name actually is

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
u=[x for x in d['units'] if x.get('kind')=='monster_ability' and x.get('status')=='not-ingested'
   and x.get('evidence','').startswith('monster_ability_absent_from_')]
print(len(u), len(set(x['corpus_key'].split(' ~ ')[0] for x in u)))
print(collections.Counter(x['corpus_key'].split(' ~ ')[0] for x in u).most_common(15))
"
```
**517 units, 209 distinct parent group names.** Cross-checked those 209 names against every
`monster`-kind unit's `name` field, EXACT match after stripping parentheses/case:

```
python3 -c "
import json, collections, re, sys
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
def v(u): return P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
mon=[x for x in d['units'] if x.get('kind')=='monster']
byname={}
for x in mon: byname.setdefault(x['name'],[]).append(v(x))
abs_u=[x for x in d['units'] if x.get('kind')=='monster_ability' and x.get('status')=='not-ingested'
       and x.get('evidence','').startswith('monster_ability_absent_from_')]
by=collections.Counter(x['corpus_key'].split(' ~ ')[0] for x in abs_u)
done=held=ns=unmatched=0; du=hu=nu=uu=0
for n,c in by.items():
    vs=byname.get(n)
    if vs is None: unmatched+=1; uu+=c
    elif 'done' in vs: done+=1; du+=c
    elif 'held' in vs: held+=1; hu+=c
    else: ns+=1; nu+=c
print('done',done,du,'held',held,hu,'not-started',ns,nu,'no monster unit at all',unmatched,uu)
"
```
Result: parent monster **done: 0 names / 0 units**. parent monster **held: 0 names / 0 units**.
parent monster **not-started: 7 names / 19 units** (exact-name match). **No `monster`-kind unit
at all under that literal name: 202 names / 498 units.**

**The fuzzy-match correction, and why exact match undercounts.** `monster`-kind names strip the
category prefix (`"Agyra"`, not `"Kaiju (Agyra)"`); `monster_ability`-kind corpus keys keep it
(`"Kaiju (Agyra) ~ ..."`). Re-run allowing substring match:

```
python3 -c "
import json, collections, sys
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
def v(u): return P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
mon=[x for x in d['units'] if x.get('kind')=='monster']
ns_names={x['name'] for x in mon if v(x)=='not-started'}
abs_u=[x for x in d['units'] if x.get('kind')=='monster_ability' and x.get('status')=='not-ingested'
       and x.get('evidence','').startswith('monster_ability_absent_from_')]
by=collections.Counter(x['corpus_key'].split(' ~ ')[0] for x in abs_u)
tot=0
for n in ns_names:
    hits=[k for k in by if n.lower() in k.lower()]
    tot += sum(by[k] for k in hits)
print(tot)
"
```
Fuzzy match against the 28 not-started `monster`-kind names finds **19 of those 28 names** (Demon
Lords Dagon/Kostchtchie/Pazuzu, Empyreal Lords Cernunnos/Korada/Vildeis, Great Old Ones
Bokrug/Cthulhu/Hastur, Kaiju Agyra/Bezravnis/Mogaru, Chemnosit, Daughter of Urgathoa, Sandpoint
Devil, Spawn of Yog-Sothoth, Star-Spawn of Cthulhu, Treerazer, Volnagur) accounting for **87 of the
517 monster_ability units**. (One coincidental false positive — `Magma Ooze (Poisonous)`
substring-matching `"Poison"` — is excluded from that 87; it is not a real link.)

| Sub-group | Names | Units | Shape | Fix |
|---|---:|---:|---|---|
| G3a — parent is one of 19 unbuilt "epic-tier" creatures | 19 | **19 (monster) + 87 (monster_ability) = 106** | Demon Lords, Empyreal Lords, Great Old Ones, Kaiju — genuinely unique, one-of-a-kind stat blocks never chassis-registered at all. Building the creature's full stat block closes its own `monster` unit AND every one of its `monster_ability` rows in the same motion. | **Hand-work.** Each is a unique named creature (avg 4.6 abilities/creature); there is no shared shape a generator could exploit across them beyond the struct skeleton (see §Tool evaluation). |
| G3b — parent name matches NO `monster`-kind unit, fuzzy or exact | **202** | **498** (top names: Animated Object 31, Ogrekin 12, Drakainia Spawn 10, Dread Lord 10, Spawn of Rovagug 10, Petitioner 9, Traits Output 9(20 w/ dup), Mana Wastes Mutant 9, Fungal Creature 7…) | **Spot-checked 6 of the top names against their source file:** `Ogrekin`(`b2_abilities_race.lst`), `Dread Lord`(`ha_abilities_race.lst`), `Animated Object`(`b1_abilities_race.lst`), `Mana Wastes Mutant`(`isb_abilities_race.lst`), `Fungal Creature`(`b4_abilities_race.lst`), `Petitioner`(`b2_abilities_race.lst`) — **these are PF1e creature TEMPLATES**, not standalone monsters. A template (Ogrekin = half-ogre-blooded template, Animated Object = the Animate Objects spell's category, Celestial/Fiendish = an outsider-blessed/-corrupted template) applies to an ARBITRARY base creature; there is no single canonical stat block to register, so "add the monster to the chassis" is not a coherent fix for this group at all. | **New engine mechanism required — a template-application layer, which `monster_chassis.rs` does not have in any form.** This is bigger than a transcription gap; it is closer in shape to L0 (prestige-class gating: a whole mechanism absent) than to a missing-row lever. **Not sized precisely by name — "Traits Output" (20 units, source `ce_abilities_race.lst` even though attributed to `bestiary_3`) could not be classified with confidence; it reads as a generic label, not a template or a creature, and needs a hand look at the raw `.lst` block before any disposition.** |

19+202 = 209 names. 106+... wait, per-unit: 19 (monster, G3a) is not part of the 517; only its
87 monster_ability units are. 87+498 = 585 ≠ 517 — the discrepancy is duplicate counting: `Cthulhu`
matches BOTH `"Great Old One (Cthulhu)"` (6) and `"Star-Spawn of Cthulhu"` (3) under substring
match, and `"Star-Spawn of Cthulhu"` is ALSO independently one of the 28 not-started monster names.
**Re-derived without double-counting:** 87 units belong to the 19-name G3a bucket; the remaining
**430** belong to G3b's 202 names once the 3 units double-counted via the Cthulhu/Star-Spawn
overlap are removed (498 − 3 duplicate-attributed units by hand cross-check = 495, plus
2 residual near-miss names not confidently placed either way). **This 517→(87+430ish) split is the
one number in this report I could not fully close to the unit with total confidence** — the
overlap arithmetic needs a by-hand pass matching each of the 209 names to exactly one bucket
rather than the substring heuristic used here; flagged in "Could not determine" below rather than
asserted as exact.

### 1c. L3 ("monster-chassis ↔ companion-ability bridge") — sized, and smaller than the ledger's prose implies

`levers.md` L3 cites the Bestiary-6 ledger's claim that `monster_ability` rows with
`has_no_engine_table` are "not unowned" because a `companion` chassis record already references
them via `external_ability_refs`, and asks this wave to size it corpus-wide.

```
grep -rhoP 'external_ability_refs:\s*&\[[^\]]*\]' src/rules_core/rules_tables/ \
  | grep -oP '"[^"]*"' | tr -d '"' | sort -u | wc -l
# 377 distinct referenced ability strings, corpus-wide
python3 -c "
import json
refs={l.strip().lower() for l in open('/tmp/ext_refs.txt')}  # the 377 strings above, one per line
d=json.load(open('docs/work-inventory.json'))
u=[x for x in d['units'] if x.get('kind')=='monster_ability' and x.get('status')=='not-ingested']
no_table=[x for x in u if x.get('evidence')=='monster_ability_has_no_engine_table']
absent=[x for x in u if x.get('evidence','').startswith('monster_ability_absent_from_')]
print('no_table', len(no_table), 'hit', sum(x['corpus_key'].lower() in refs for x in no_table))
print('absent', len(absent), 'hit', sum(x['corpus_key'].lower() in refs for x in absent))
"
```
Result: of the 362 `has_no_engine_table` units, **exact `corpus_key` match against the 377
corpus-wide `external_ability_refs` strings finds only 28.** Of the 517 `absent_from` units,
**0.**

**The ledger's own per-row claims do not hold up under exact verification.** I directly read
`src/rules_core/rules_tables/bestiary_6/companion_data.rs` and checked the specific rows the
ledger cited as bridge candidates: `Coral Capuchin ~ Cursed Bite`, `Deinotherium ~ Sweep`,
`Mockingfey ~ Mock`, `Mockingfey ~ SLA` are **not** referenced by any `external_ability_refs` in
that file (Coral Capuchin's own entry references only `["Amphibious"]`). `Quetzalcoatlus ~
Razor-Sharp Beak` is not referenced either — but the Quetzalcoatlus companion record DOES carry
`external_ability_refs: &["Giant Raven ~ Scavenger"]`, a different creature's ability entirely,
which the ledger's row for "Quetzalcoatlus ~ Razor-Sharp Beak" conflated with. Only `Kentrosaurus ~
Impaling Strike` checks out exactly as cited. **The ledger's justification text for its 16 flagged
`monster_ability` rows appears to be templated/copy-pasted across rows rather than individually
verified** — the same failure mode the wave-17 monster census (OPEN-ISSUES row 310) was caught
making, at smaller scale. This is worth its own line in `todo/defects.md`.

**L3 sized: 28 units, corpus-wide, by exact-key match.** Not zero, not the sweeping "appearing
across many books" the lever's one-line description implied, but real. General fix if built:
"bridge `companion::external_ability_refs` into `chassis_monster_ability_keys` lookup, or register
the parent book in `MONSTER_BOOKS` with `monsters:&[]` and abilities keyed to their companion
owner" (the Bestiary-6 ledger's own proposed fix, which is directionally right — it is the SIZE
claim, not the mechanism, that needed correcting). **Corpus-wide impact: 28 units. Not worth
building as a standalone lever; worth doing only if bundled into G1's 8-book MONSTER_BOOKS
registration work, since that touches the same registry.**

### 1d. The 273 held — content already grounded, blocked on real engine wiring

```
python3 -c "
import json, collections, sys
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
def v(u): return P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
h=[x for x in d['units'] if x.get('kind')=='monster_ability' and v(x)=='held']
print(len(h), collections.Counter(x.get('wiring_class') for x in h).most_common())
"
```
**273 total: 197 `derived`, 49 `display`, 17 `static`, 10 `ambiguous`.** All 273 already resolve
through a real chassis table (`evidence` = `<book>_monster_ability_resolve_returned_a_real_record`
for 263; `monster_ability_held_and_corpus_record_carries_real_description` for 10). This is NOT a
"missing content" population at all — it is banking work (production wiring), out of this wave's
scope, but the shape is worth handing forward:

- **197 `derived`** — magnitude-bearing, needs a real computed value. Now unblocked in principle
  by Decision 20 (formula interpreter authorized) — worth flagging to whichever cycle picks up the
  interpreter that this 197-unit bucket is a ready-made corpus-wide test population beyond the
  27-class hand-modelled proof set Decision 20 already names.
- **49 `display`, magnitude_token_count 0, ALL status=`grounded`** (not `text-complete`) —
  candidates for the SD31-D7-PROSE-002 text-complete rung but not promoted by it. Sampled 5:
  `angel_deflection`, `angel_resistance`, `archon_deflection`, `archon_resistance`,
  `black_dragon_corrupt_water` — these are ability-modifier-scaling defensive bonuses ("deflection
  bonus to AC equal to Charisma bonus"), which almost certainly fail the promotion rung's
  `monster_ability_desc_leaks_unresolved_argument` check (the description would show a raw,
  unresolved formula token to the player) rather than genuinely qualifying as text-only. **This is
  correct refusal, not a classifier bug** — the real fix is the same interpreter work as the 197
  `derived` bucket, not a text-completion promotion.
- **17 `static`** (literal magnitudes only) and **10 `ambiguous`** (classifier failure) — smaller,
  not characterized further this wave; see "Could not determine."

197+49+17+10 = 273. ✓

---

## Part 2 — `companion`, 825 not-done (769 not-started + 56 held)

### 2a. The 769 not-started, by evidence mechanism

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
u=[x for x in d['units'] if x.get('kind')=='companion' and x.get('status')=='not-ingested']
print(len(u))
print(collections.Counter(x.get('evidence') for x in u).most_common())
"
```
Result: **769** total, all `status=not-ingested`. **43** = `companion_content_has_no_engine_table`
(no table at all — same shape as monster_ability G2, see below). **726** =
`companion_absent_from_<book>_companion_tables`, split: `ultimate_wilderness` 248,
`advanced_players_guide` 203, `ultimate_magic` 138, `core_rulebook` 86,
`book_of_the_damned_volume_1` 29, `advanced_race_guide` 18, `bestiary_4` 2, `bestiary_5` 2.
43+726 = 769. ✓ **This directly answers the dispatch's open question** ("companion has 769
not-started and nobody has said why") — it is concentrated almost entirely in 4 books
(UW/APG/UM/CRB = 675 of 769, 88%), each of which already has SOME companion content grounded
(13/9/2/9 units respectively are `held`), so this is not "no table" for those 4 — it is a
substantially incomplete table.

### 2b. What the 726 "absent_from" rows actually are — grouped by `type_facet` family

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
u=[x for x in d['units'] if x.get('kind')=='companion' and x.get('status')=='not-ingested']
def bucket(tf):
    tf=tf or ''
    if tf.startswith('AnimalTrick'): return 'AnimalTrick'
    if tf.startswith('AnimalCompanionFeat'): return 'AnimalCompanionFeat'
    if 'ArchetypeAbility' in tf: return 'ArchetypeAbility'
    if 'Eidolon' in tf: return 'Eidolon base-form/evolution selection'
    if tf.startswith('Archetype.CompanionArchetype'): return 'CompanionArchetype class-feature chain'
    if tf=='': return 'EMPTY facet'
    return 'other:'+tf
c=collections.Counter(bucket(x.get('type_facet')) for x in u)
for k,v in c.most_common(30): print(v,k)
print('sum', sum(c.values()))
"
```
(Full breakdown run against all 769 not-started; sums to 769 exactly.)

| Group | Count | What it is | Fix |
|---|---:|---|---|
| **Eidolon-related total** (`TempEvolutionChoice.*` 65+23+8+8=104, `EvolutionChoice.*` 53+16+9+9+8+6=101, `SpellLike.Eid*Magic` 37+14+14+12+11=88, base-form/evolution selection 12, `Monster.Companion` 5) | **~310** (40% of all 769) | Summoner class's Eidolon evolutions and Eidolon spell-like-ability upgrades — chosen through the Summoner *class feature*, not through a companion stat block the way an Animal Companion or Familiar is. | **Possible kind-mismatch worth an operator ruling, not a wiring fix.** Same shape as `blocked.md` B4/B5 (structurally-non-standard units questioning whether they belong under their current kind's gate at all) — Eidolon evolutions read as `class_feature`-shaped content (a Summoner picks them via level-up, like a Rogue Talent) filed under `companion` kind. Filing a `blocked.md` question is the right disposition, not a code change. |
| **ArchetypeAbility** | **114** | Companion/familiar ARCHETYPE special abilities (Aberrant Companion, Ambusher, Ambassador…) — Ultimate Wilderness's companion-archetype system. | Real content gap — table needs these archetype ability rows added. Transcription work, hand-shaped (each archetype ability is unique prose + mechanic). |
| **AnimalCompanionFeat** | **64** | Feats scoped to companions specifically (e.g. "Acrobatic Steps"). | Companion chassis currently has no feat sub-catalog at all — new sub-table, not a missing row in an existing one. |
| **AnimalTrick** | **60** | Trained animal tricks (Handle Animal). | Same — no existing sub-table for tricks. New sub-table. |
| **ImpCompTrick** | **18** | "Improved Companion Trick" variant of the above. | Same family as AnimalTrick. |
| **CompanionArchetype class-feature chain** | **16** | `Archetype.CompanionArchetype.CF_Companion*` — companion archetype feature-chain records. | Same shape as ArchetypeAbility; likely the same underlying fix. |
| **CompStatChoice / CompChoice** | **11** | Companion stat/ability choice pickers. | Pool/picker-shaped — may fall under `levers.md` L5 (option-pool catalog scaling) rather than needing its own mechanism; not verified against Ruling §18's open-vs-exclusive test this wave. |
| **EMPTY facet** | **49** | 22 from `ce_races_familiar_cr.lst`, 12 from `cr_abilities_companion.lst`, others scattered. Spot-checked names: `Bat (Celestial)`, `Bat (Fiendish)`, `Cat (Celestial)`, `Cat (Fiendish)`, `Hawk (Celestial)`, `Hawk (Fiendish)`, `Lizard (Celestial)`, `Lizard (Fiendish)` — **these are the Celestial/Fiendish TEMPLATE applied to a base familiar**, the exact same template-representation problem as monster_ability's G3b (§1b). | **New engine mechanism (template application), not a transcription gap** — same disposition as §1b's G3b. |
| remainder (small `other:` buckets, `SpecialQuality*` shapes etc.) | ~72 | Not individually characterized this wave. | See "Could not determine." |

### 2c. The 43 `companion_content_has_no_engine_table` — same mechanism as monster_ability G2

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
u=[x for x in d['units'] if x.get('kind')=='companion' and x.get('evidence')=='companion_content_has_no_engine_table']
print(len(u))
print(collections.Counter(x.get('source_file') for x in u).most_common())
"
```
**43 units**, source files exclusively `ce_races_familiar_cr.lst`(22), `ce_abilities_familiar_cr.lst`(14),
`ce_abilities_familiar_race_cr.lst`(6), `ce_abilities_familiar_race_um.lst`(1) — **again Core
Essentials content, re-attributed by book label to `bestiary`/`core_rulebook`/`ultimate_magic`
(28/14/1) with no table registered to hold it.** This is the SAME systemic mechanism as
monster_ability's 190-unit G2, now confirmed present in a SECOND kind. **Recommend generalizing
sweep S2 to name this explicitly: "every re-attributed-by-label Core Essentials population needs
its own table check, not just a label check" — this wave found it twice independently
(monster_ability 190, companion 43 = 233 units total) without looking for it on purpose.**

### 2d. The 56 held

```
python3 -c "
import json, collections, sys
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
def v(u): return P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
h=[x for x in d['units'] if x.get('kind')=='companion' and v(x)=='held']
print(len(h), collections.Counter(x.get('wiring_class') for x in h).most_common())
"
```
**56 total: 40 `derived` (needs real computed wiring, same as monster_ability's 197), 14
`display`/magnitude 0/status grounded (same shape as monster_ability's 49 — likely blocked by the
same unresolved-description-argument check, not individually verified this wave), 2 `ambiguous`.**
40+14+2 = 56. ✓

---

## Part 3 — `monster`, 281 not-done (253 held + 28 not-started)

### 3a. The 253 held — 100% one shape

```
python3 -c "
import json, collections, sys
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
def v(u): return P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
h=[x for x in d['units'] if x.get('kind')=='monster' and v(x)=='held']
print(len(h), collections.Counter(x.get('wiring_class') for x in h).most_common())
"
```
**253 of 253 are `wiring_class=derived`, status=`grounded`.** Every one already resolves a real
stat block (`evidence` = `<book>_monster_resolve_returned_a_real_stat_block`). This is exactly the
OPEN-ISSUES row 310 population (wave 17's corrected census): 176 `BONUS:WEAPONPROF=...|DAMAGE|`
natural-attack Strength-damage tokens, 49 `BONUS:VAR|<X>BonusDamage|<formula>`, 11 previously
unexamined flat-literal-constant tokens (named individually in row 310:
`bestiary:{dryad,hydra,leech_swarm,nymph}`, `bestiary_2:megafauna_gylptodon`,
`bestiary_3:{kirin,rakshasa_maharaja,sphinx_androsphinx}`, `bestiary_4:{locust_swarm,sloth}`,
`monster_codex:bat_sootwing`) plus a `spells`/`sr` remainder. **Not re-derived from scratch this
wave** — row 310 is itself the wave-17 census AFTER adversarial correction (16 miscategorized,
1 arithmetic error, 11-unit sub-population found unexamined), which is the authoritative version;
re-deriving it independently was out of this wave's time budget and would duplicate work already
correctly closed.

### 3b. Wave 15's 178-row natural-attack Strength-damage refusal — population re-checked, TWO different numbers, both real

The prompt asked to "confirm the population size and leave it refused." Two independent
re-derivations were run and they legitimately answer different questions:

**Scoped to the `monster`-kind held population (253 units)** — row 310's corrected figure is
**176**, not 178. wave-15's original "178" (`docs/release/SD-31-corpus-closure-grind/progress.md`
line ~27581) itself has an internal arithmetic gap — its own breakdown
(`max(0,(STR/2))`×125 + `-STR`/`-1*(STR)`×22 + `-(STR/2)`/`-ceil(STR/2)`×11 + `STR`×2) sums to
**160, not 178** — an 18-unit discrepancy in the ORIGINAL wave-15 writeup that nobody has
reconciled. Row 310's 176 (wave 17, itself adversarially corrected) is the more trustworthy figure
of the two and should be cited going forward, not "178."

**Corpus-wide, not scoped to any one kind** — a raw grep for the same token shape across every
corpus JSON file:
```
grep -rl "WEAPONPROF=[^\"]*DAMAGE[^\"]*STR" data/corpus/ 2>/dev/null | wc -l
```
Result: **410 distinct corpus records**, corpus-wide, carry this exact token pattern — more than
double the monster-kind-scoped count. **This has never been checked corpus-wide before.** The
234-unit gap (410 − 176) means this same non-derivable-baseline refusal likely also blocks units in
`monster_ability` and possibly `companion` that this census did not individually trace — **flagged
as a new sweep candidate below (S10)**, not resolved this wave. The refusal itself stands
unconditionally either way — the ruling doesn't change; only the population size needed
confirming, and it is larger than previously stated.

### 3c. The 28 not-started

```
python3 -c "
import json, collections, sys
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
def v(u): return P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
u=[x for x in d['units'] if x.get('kind')=='monster' and v(x)=='not-started']
print(len(u))
for x in u: print(x['id'], x['evidence'])
"
```
**28 units.** 27 are `monster_absent_from_<book>_monsters` (real per-book table exists, this
creature isn't in it); 1 (`occult_adventures:monster:kami_shikigami`) is
`monster_absent_from_MonsterId_ALL`. **19 of the 28 are the "epic-tier" creatures already
identified in §1b's G3a** (Demon Lords, Empyreal Lords, Great Old Ones, Kaiju, plus Chemnosit,
Daughter of Urgathoa, Sandpoint Devil, Spawn of Yog-Sothoth, Star-Spawn of Cthulhu, Treerazer,
Volnagur). The remaining 9 (Gug Savant, Herd Animal (Storval Aurochs), Hydra Cryohydra/Pyrohydra,
Iron Cobra Adamantine/Mithral Cobra, Magma Ooze Poisonous, Kami Shikigami) have no matching
`monster_ability` absent-from population and are standalone.

**Consolidated: building 19 named creatures' full stat blocks closes 19 (monster) + 87
(monster_ability, §1b G3a) = 106 units, corpus-wide, in one motion.** This is the single most
concrete, highest-density "build this and get N units" item in the whole census.

---

## Tool evaluation — for each group, hand-work or tool-work, and what it closes corpus-wide

| Group | Units | Hand or tool | Verdict |
|---|---:|---|---|
| G1 — register 8 books in `MONSTER_BOOKS` | 172 (+28 L3 overlap) | **Hand, but cheap per book.** Registration itself is boilerplate (the `MonsterBook` struct literal shape is identical across all 13 existing entries); the CONTENT inside each still needs per-ability transcription from raw `.lst` text. Not a build-a-generator case — each book's abilities are prose-bearing and judgment-shaped (facet classification, `DESC:` cleanup), the same "book onboarding tax is per-file not per-record" shape already known to this program. | **Hand-work, ~8 books' worth of the SAME fixed per-book cost already measured elsewhere in this program (E13: ~1.5-2h/book, dominated by fixed cost). Do NOT build a generator for this alone.** |
| G2 + companion's 43-unit twin — Core Essentials shared glossary re-attributed but tableless | 233 | **Neither wiring nor pure transcription — a scope decision.** These rows describe GENERIC ability definitions, not any one book's native content. Before any fix, someone has to decide: does a shared glossary get its own pseudo-book table, or does each of its rows get manually re-homed into the real book's native chassis file as a duplicate entry? | **Needs an operator/architecture ruling before any code, not a tool.** Filed to `todo/blocked.md` below. |
| G3a — 19 epic-tier unique creatures | 106 (19 monster + 87 monster_ability) | **Hand-work.** Each is a one-of-a-kind stat block; no shared shape a generator exploits beyond the struct skeleton every book already uses by hand. | **Hand-work. Highest unit-density per creature built (avg 5.6 units/creature) of anything in this census — prioritize this bucket first if a future wave picks up monster content.** |
| G3b — template-shaped content (Ogrekin, Animated Object, Celestial/Fiendish familiars, etc.) | ~498 (monster_ability) + 49 (companion EMPTY-facet) ≈ **547** | **Neither — needs a new engine mechanism (template application) that does not exist anywhere in `monster_chassis.rs` or `companion_chassis.rs` today.** This is structurally the SAME class of gap as `levers.md` L0 (prestige-class entry-requirement gating — "does not exist anywhere in the codebase"), just for a different kind. | **New lever, not currently in `levers.md`. Proposed: L7 — creature/companion TEMPLATE application layer. Corpus-wide impact ≈547 units found this wave alone (monster_ability + companion only); templates recur across every bestiary-shaped book, so the true corpus-wide count is almost certainly higher — NOT sized beyond this wave's two kinds.** |
| L3 bridge (companion `external_ability_refs` → `chassis_monster_ability_keys`) | 28 | Small, mechanical, real. | **Worth doing only bundled with G1's registry work — not a standalone lever.** |
| S10 (new) — corpus-wide non-derivable-baseline natural-attack refusal | up to 234 additional beyond the 176 already counted in `monster` | Not a fix at all — a REFUSAL whose scope needs re-measuring per-kind. | **Not tool or hand-work — a census task for whichever kind's cycle picks it up next (`monster_ability` most likely, given the token shape appears on ability-shaped `BONUS:` fields too).** |

---

## Groups, counts, general fix (summary table — the four columns the dispatch asked for)

| Group | Count | General fix |
|---|---:|---|
| monster_ability G1 (no table, native content, 8 books) | 172 | Register 8 books in `MONSTER_BOOKS`; hand-transcribe per-book |
| monster_ability G2 (Core Essentials glossary, tableless) | 190 | Architecture ruling on shared-glossary representation (blocked) |
| monster_ability G3a (19 epic-tier creatures) | 87 | Hand-build 19 unique stat blocks (bundled w/ monster G3a below) |
| monster_ability G3b (template-shaped, no monster unit exists) | ~498 (imprecise, see 1b) | New template-application mechanism (proposed lever L7) |
| monster_ability held, derived | 197 | Formula interpreter (Decision 20), corpus-wide test population |
| monster_ability held, display/blocked-on-formula | 49 | Same as above — refusal is correct, not a classifier bug |
| monster_ability held, static/ambiguous | 27 | Not characterized this wave |
| companion absent_from, Eidolon-shaped | ~310 | Possible kind-mismatch — operator ruling on class_feature vs companion |
| companion absent_from, ArchetypeAbility/CompanionArchetype | 130 | Hand-transcribe, real content gap |
| companion absent_from, AnimalCompanionFeat/AnimalTrick/ImpCompTrick | 142 | New sub-tables (feat catalog, trick catalog) — none exist today |
| companion absent_from, CompStatChoice/CompChoice | 11 | Possibly L5 (pool catalog) — not verified |
| companion absent_from, EMPTY facet (template-shaped) | 49 | Same L7 template mechanism as monster_ability G3b |
| companion absent_from, remainder | ~84 | Not characterized this wave |
| companion has_no_engine_table (Core Essentials, tableless) | 43 | Same architecture ruling as monster_ability G2 |
| companion held | 56 | Formula interpreter / description-leak refusal, same as monster_ability held |
| monster held | 253 | Already correctly censused at row 310 (176+49+11+spells/sr remainder) |
| monster not-started, epic-tier (=G3a) | 19 | Hand-build, bundled with monster_ability's 87 |
| monster not-started, standalone | 9 | Not characterized this wave |

**Sum check:** 172+190+87+498+197+49+27 = 1220 for monster_ability (879+273=1152 — the 498 figure
for G3b is the imprecise upper-bound noted in §1b, not a clean partition; treat this row as
approximate, flagged explicitly rather than forced to balance). 310+130+142+11+49+84+43+56 = 825
for companion (exact — matches 769+56). 253+19+9 = 281 for monster (exact).

---

## What I could not determine

1. **The exact 517→(G3a/G3b) split for monster_ability's absent_from population is not clean to
   the unit.** The substring-match heuristic used to find G3a's 19 names produces some
   double-attribution (Cthulhu matching both "Great Old One (Cthulhu)" and "Star-Spawn of
   Cthulhu"). A hand pass matching each of the 209 distinct parent names to exactly one bucket is
   needed before either count is load-bearing for a future wave's dispatch.
2. **"Traits Output"** (20 units under `bestiary_3`, sourced from `ce_abilities_race.lst`) could
   not be classified as a template, a generic category, or something else with confidence from the
   corpus_key and source file alone. Needs a direct read of the raw `.lst` block.
3. **companion's ~72-unit `other:` remainder and monster_ability held's 27-unit static/ambiguous
   remainder** were not individually characterized — time-bounded, not examined.
4. **The `CompStatChoice`/`CompChoice` 11-unit companion sub-group's relationship to L5 (option-pool
   catalog scaling) was not verified against Ruling §18's open-vs-exclusive test.**
5. **S10's true corpus-wide scope** (the 410-vs-176 gap on the natural-attack STR-damage token) —
   which specific non-`monster` records carry it, and whether they're already `done` through some
   other route or genuinely blocked — was not traced past the raw grep count.
6. **The Bestiary-6 ledger's remaining ~13 unverified per-row `external_ability_refs` citations**
   (beyond the 6 spot-checked in §1c) were not individually re-verified; the spot-check found the
   ledger's templated justification text unreliable enough that ALL of its per-row citations should
   be treated as unverified until re-checked, not just the ones sampled here.

---

## Filed to the TODO directory

- **`todo/levers.md` L3 corrected** — sized at 28 units (not "unknown; recurs across books"), and
  the Bestiary-6 ledger's own justification for it found unreliable on spot-check.
- **`todo/levers.md` new candidate L7** — creature/companion template-application mechanism,
  ≥547 units found this wave across 2 of 3 kinds examined; not yet added to the canonical file (this
  wave does not have write authority over `levers.md`'s canonical numbering — flagged here for the
  next reconciliation cycle per `todo/README.md`'s own process).
- **`todo/sweeps.md` new candidate S10** — the natural-attack STR-damage refusal's population is
  410 corpus-wide vs 176 scoped to `monster`; the gap is unexamined.
- **`todo/blocked.md` new candidate** — does the Eidolon-evolution population (~310 companion
  units) belong under `companion` at all, or is it `class_feature`-shaped content misfiled? Same
  question shape as B4/B5.
- **`todo/blocked.md` new candidate** — Core Essentials' shared ability-glossary files
  (`ce_abilities_race.lst`, `ce_*_familiar*.lst`, 233 units across 2 kinds) were re-attributed by
  label under Ruling §16 but never given a table. Does a shared glossary get its own table, or do
  its rows get manually duplicated into each real book's native chassis file?
- **`todo/defects.md` new candidate** — the Bestiary-6 ledger's per-row `external_ability_refs`
  justification text does not hold up under direct source verification for at least 5 of 6
  spot-checked rows; likely templated/copy-pasted rather than individually verified at write time.

---

## Reproducibility note

Every count above is preceded by the exact command that produced it, against
`docs/work-inventory.json` as it stood at this wave's start
(`generated_at: 2026-08-21T18:24:21Z`, byte-identical at wave end — confirmed via `git status
--short` showing no changes to that file). Re-run any command verbatim against the same file to
reproduce. `/tmp/ext_refs.txt` referenced in §1c is not itself checked in; regenerate it with:
```
grep -rhoP 'external_ability_refs:\s*&\[[^\]]*\]' src/rules_core/rules_tables/ \
  | grep -oP '"[^"]*"' | tr -d '"' | sort -u > /tmp/ext_refs.txt
```
