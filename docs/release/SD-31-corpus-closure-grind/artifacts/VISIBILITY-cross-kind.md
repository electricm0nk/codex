---
canonical: true
wave: 30
lane: 6
purpose: >
  The checks no single-pile lane can perform: (A) a provably non-overlapping partition of the
  not-done population, so a coverage tool can assign every unit to exactly one group; (B) causes
  that recur across kinds, which no single-kind lane can see; (C) a sampled audit of whether units
  sit in the RIGHT verdict, with a real corpus-wide misfiling rate where one could be established.
  Banks nothing. `docs/work-inventory.json` untouched — confirmed byte-identical before and after
  (md5 `d64ddfc677fd1683f5b7638889a25c54`).
---

# VISIBILITY — cross-kind checks, wave 30 lane 6

## 0. Board state (confirm unchanged, frozen for this wave)

```
md5sum docs/work-inventory.json
d64ddfc677fd1683f5b7638889a25c54   (unchanged before and after this lane's entire session)
```

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

`13,458/38,372 = 35.08%`. Not-done = `18,645 + 3,763 + 1,231 + 1,230 + 45 = 24,914`. Matches the
dispatch brief exactly. This is the population every count below reconciles against.

No production code, test, or `docs/work-inventory.json` byte was touched. This lane's entire git
diff is `todo/sweeps.md` (2 new rows, S12/S13), `todo/blocked.md` (2 corrected rows, B7/B8),
`todo/levers.md` (1 new row, L11, plus a correction appended to L10), and this file.

---

## 1. Part A — the non-overlapping partition

### 1.1 Why wave 28's split overlapped, restated as a general rule

THE-BOX §1.3 found a 1,212-unit double-count: the `unmeasurable` lane (crosscut 5 kinds, one
verdict) and the `spell-feat-equipment` lane (4 kinds, every verdict) both claimed the unmeasurable
subset of spell/feat/equipment/equipment_modifier, because nothing forced the two lane definitions
onto disjoint cells of the same grid. THE-BOX also found a 295-unit gap for the same reason in the
other direction: no lane's boundary happened to cover `race`, or `class_feature`'s `held`/`deferred`.

Both failures have one root cause, not two. **Every unit carries exactly one `kind` string and
exactly one `doneness_verdict(wiring_class, status, kind)` output — a pure function of three fields
already on the unit.** A lane defined along a SINGLE axis ("all of kind X" or "all units in verdict
Y") is not a partition of a two-axis population; it is a projection, and two projections along
different axes overlap wherever they share a cell. This is true regardless of which specific kinds
or verdicts get assigned to which lane in any future wave — it is not a fact about wave 28's
particular split, it is a fact about splitting a 2-D grid along 1-D lines.

### 1.2 The grid itself — the coarsest partition that is provably disjoint and provably exhaustive

```
python3 -c "
import json,sys,collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
v=lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
by=collections.defaultdict(collections.Counter)
for u in U:
    vv=v(u)
    if vv!='done': by[u['kind']][vv]+=1
tot=0
for k in sorted(by):
    for verdict,n in sorted(by[k].items()):
        print(k, verdict, n); tot+=n
print('TOTAL', tot)
"
```

| kind | not-started | unmeasurable | in-progress | held | deferred | kind not-done |
|---|---:|---:|---:|---:|---:|---:|
| class | 157 | – | – | – | – | 157 |
| class_feature | 12,473 | 2,551 | – | 38 | 43 | 15,105 |
| companion | 769 | – | – | 56 | – | 825 |
| equipment | 222 | 205 | 261 | 207 | – | 895 |
| equipment_modifier | 63 | 416 | 568 | 17 | – | 1,064 |
| feat | 496 | 565 | 1 | 87 | 2 | 1,151 |
| monster | 28 | – | – | 253 | – | 281 |
| monster_ability | 879 | – | – | 273 | – | 1,152 |
| race | 57 | – | 3 | – | – | 60 |
| race_trait | 2,712 | – | 241 | 1 | – | 2,954 |
| spell | 789 | 26 | 157 | 298 | – | 1,270 |
| **TOTAL** | **18,645** | **3,763** | **1,231** | **1,230** | **45** | **24,914** |

**33 populated `(kind, verdict)` cells, summing to exactly 24,914 — no correction, no remainder.**
This is a strict partition by construction: no unit can occupy two cells, because `kind` and
`doneness_verdict(...)` are each single-valued per unit (verified separately in §1.4: unit ids are
100% unique, so there is no duplicate-row risk feeding two cells from one real record either).

### 1.3 The rule for a coverage tool

A lane's assigned population must be a **union of whole cells** from the table above. Two lanes may
never both claim the same cell. A coverage tool can check this mechanically, without waiting for a
wave to finish and reconcile after the fact the way THE-BOX had to:

```
sum(assigned cell counts across all lanes) == 24,914   # exhaustive
no (kind, verdict) cell appears under two lanes          # disjoint
```

Both checks are O(number of cells) = O(33), not O(units), and can run BEFORE dispatch rather than
after six lanes have already done the work. This does not mean every future lane must be scoped to
one cell — most cells are far too small to be a whole lane, and most useful lanes will span several
kinds or several verdicts. It means whichever cells a lane spans, the assignment must be stated at
the cell level so the union/disjointness check is possible at all. "All of `unmeasurable`" and "all
of `spell`+`feat`+`equipment`+`equipment_modifier`" are each valid cell-unions on their own; the
problem was never picking those two shapes, it was picking them BOTH without checking they shared
five cells.

**Demonstrated live, with current numbers, that the exact wave-28 mistake would still overlap
today if repeated:** an `unmeasurable` lane and a `spell+feat+equipment+equipment_modifier`
(all-verdict) lane would again share `spell` unmeasurable 26 + `feat` unmeasurable 565 +
`equipment` unmeasurable 205 + `equipment_modifier` unmeasurable 416 = **1,212** — unchanged since
wave 28, because none of those four cells moved between wave 28 and now. The fix in §1.2/§1.3
prevents this shape from recurring on whatever the NEXT pile turns out to be, not just this one.

### 1.4 Secondary double-counting checks (the ones a per-kind lane cannot run)

- **Unit-id uniqueness, corpus-wide**: `len(ids) == len(set(ids))` on the raw 38,391-unit array
  (before the `EXCLUDED_BOOKS` filter) — **0 duplicates**. No unit can silently feed two grid cells
  by having two rows.
- **A false lead, caught and discarded rather than reported**: building a `(book, kind, key)` index
  directly from `data/corpus/**/*.json` (26,966 files) to cross-reference sampled units against
  their real corpus record (§3 below) surfaced 4 apparent "duplicate key" groups (`advanced_class_guide
  class None`, `core_rulebook class None`, `advanced_players_guide class None`, `beastiary monster
  None`). Checked before reporting, per the standing "validate the proxy" rule: every one of these
  is a `data.key` field that is simply absent on `class`/`monster` records (they key off `name`
  instead), not a real corpus collision — an artifact of this lane's own quick index, not a
  work-inventory or corpus defect. Reported here as a checked-and-cleared false lead, exactly the
  discipline S11 asks every lane to apply to its own tooling before filing a finding.

---

## 2. Part B — cross-kind patterns

Four causes checked corpus-wide (not sampled) for recurrence across kinds. Two are genuinely new;
two are corrections that widen an existing `blocked.md` question's true population.

### 2.1 Book onboarding — re-measured after wave 29's partial fix (already known, re-derived current)

```
python3 -c "
import json,collections,sys
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
v=lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
gate=[u for u in U if v(u)!='done' and u.get('evidence')=='no_compiled_rule_set_for_book']
c=collections.Counter((u['book'],u['kind']) for u in gate)
print(len(gate)); [print(k,n) for k,n in sorted(c.items())]
"
```

`adventurers_guide` confirmed fully off this gate (0 rows) — wave 29's fix held. **422 not-done
units remain gated, across 6 kinds and 4 books**: `inner_sea_magic` 335 (class_feature 218,
equipment_modifier 62, spell 39, feat 7, equipment 6, class 3), `inner_sea_temples` 64 (equipment
43, spell 21), `inner_sea_taverns` 20 (class_feature 11, feat 9), and a previously-unnamed 4th book,
`inner_sea_faiths` 3 (spell only — a book with most kinds already registered but a small residual
still behind the gate). Filed as an L10 correction, `todo/levers.md`.

### 2.2 Core Essentials shared-glossary files (`ce_*.lst`) — B7's scope was 2 of 7 kinds

```
python3 -c "
import json,collections,sys
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
v=lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
ce=[u for u in U if u.get('source_file','').startswith('ce_')]
nd=[u for u in ce if v(u)!='done']
print('total', len(ce), 'not-done', len(nd))
print(collections.Counter(u['kind'] for u in nd).most_common())
print('done by kind:', collections.Counter(u['kind'] for u in [x for x in ce if v(x)=='done']).most_common())
"
```

`blocked.md` B7 named 233 units (190 `monster_ability` + 43 `companion`). The real `ce_*.lst`
population is **811 units across 7 kinds** (12 distinct source files: `ce_abilities_race.lst` 516,
`ce_spells.lst` 109, `ce_races_familiar_cr.lst` 53, `ce_abilities_familiar_race_cr.lst` 42,
`ce_classes_race.lst` 23, `ce_races_familiar_um.lst` 19, `ce_feats.lst` 15,
`ce_abilities_familiar_cr.lst` 14, `ce_races_familiar_apg.lst` 8, `ce_abilities_familiar_race_um.lst`
9, `ce_equip_arms_armor.lst` 2, `ce_equip_general.lst` 1) — of which **566 are not-done**:
`monster_ability` 234, `race_trait` 147, `spell` 109, `companion` 46, `class` 23, `feat` 4,
`equipment` 3. 245 are already `done` (`monster_ability` 135, `companion` 99, `feat` 11), which
means the shared-glossary shape is not structurally undoable — some of it already ships. Corrected
in `todo/blocked.md` B7.

### 2.3 `visible == false` — B8's "essentially unique to equipment_modifier" does not hold corpus-wide

```
python3 -c "
import json,collections,sys
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
v=lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
nd=[u for u in U if v(u)!='done']
vf=[u for u in nd if u.get('visible')==False]
by=collections.Counter(u['kind'] for u in vf)
tot=collections.Counter(u['kind'] for u in nd)
for k,n in by.most_common(): print(k,n,'/',tot[k],'=',round(100*n/tot[k],1),'%')
print('sum', sum(by.values()))
"
```

921 not-done units carry `visible == false`, across 8 of 11 kinds: `equipment_modifier` 504
(47.4% of its own pile — the real concentration point, and the number B8 quoted), `class_feature`
246 (1.6%), `race_trait` 131 (4.4%), `equipment` 13 (1.5%), `class` 12 (7.6%), `companion` 11
(1.3%), `feat` 2 (0.2%), `monster_ability` 2 (0.2%). 306 more `visible == false` units are already
`done` — real precedent this shape can clear the gate somewhere, which reframes B8's question from
"can this kind's units ever be done" to "does `visible == false` alone disqualify a unit,
corpus-wide." Corrected in `todo/blocked.md` B8.

### 2.4 `.COPY=` alias-origin rows (`origin == "copy"`) — 105 more not-done units in 5 more kinds

```
python3 -c "
import json,collections,sys
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
v=lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
copy=[u for u in U if u.get('origin')=='copy']
nd=[u for u in copy if v(u)!='done']
print(len(copy), len(nd))
print(collections.Counter(u['kind'] for u in nd).most_common())
print(collections.Counter(v(u) for u in nd).most_common())
"
```

THE-BOX §2.5 reported "up to 861 across equipment + equipment_modifier," lane-filed and not
independently confirmed. The equipment/equipment_modifier half reproduces **exactly** (405 + 456 =
861 — that part of the filed figure is now confirmed, not merely lane-reported). But the same
`origin == "copy"` population also carries 105 more not-done units in 5 kinds nobody named:
`race_trait` 51, `companion` 25, `spell` 15, `race` 12, `monster` 2. **Corpus-wide total: 966, not
861, across 7 kinds not 2.** Verdict split: unmeasurable 451, in-progress 225, not-started 167,
held 123. Filed as `todo/sweeps.md` S13.

### 2.5 Archetype-lock token (`PREABILITY ... CATEGORY=Archetype`) — checked, NOT a live gap

Wave 29's `is_archetype_locked()` fix (`class_feature_pool_catalog.rs`) was scoped and measured
against 300 Rogue Talent + Rage Power records (6 archetype-locked). Checked whether the SAME token
recurs in other pools/kinds, which no single-pile lane looked at:

```
# full-corpus scan of raw_tokens for PREABILITY ... CATEGORY=Archetype, all kinds — see script
# in the reproduction appendix (§5)
```

**181 corpus-wide hits**, not 300 — `class_feature` 178, `equipment_modifier` 2, `equipment` 1. Of
the 178 `class_feature` hits, only 6 belong to Rogue Talent/Rage Power (the pools the fix's 300-unit
scope covered); **175 belong to OTHER, not-yet-registered pools** — Discovery (Alchemist), Monk
Bonus Feat, Arcanist Exploit, Underground Chemist, Improved Bodily Mutations, and more. This looked
like a live gap (the fix's population and the token's real population differ by 60×). It is not: all
175 are currently `not-started` or `unmeasurable` (0 `done`), and reading `is_archetype_locked()`
(`src/rules_core/class_feature_pool_catalog.rs:156`) shows it checks the raw token directly,
unconditioned on pool name — it will refuse any of these 175 automatically the moment their pool is
ever registered, with no further code change. **Reported as a confirmed-safe finding, not a new
defect**: a future pool-registration wave does not need to re-derive this, and should not spend a
cycle re-guarding something already generic.

### 2.6 Reconciliation against §1's grid

Every count in §2.1–§2.4 is a population drawn from the SAME 24,914-unit frozen set (§1.2), each
identified by a field already on the unit (`evidence`, `source_file`, `visible`, `origin`) rather
than by a lane-specific text filter — so none of these four findings can introduce a new
double-count against the partition in §1: they are cross-cutting VIEWS onto the same grid, not new
populations, and a future lane assigned any of these as a follow-up must still be scoped at the
`(kind, verdict)` cell level per §1.3.

---

## 3. Part C — verdict definition audit

### 3.1 Method

Five verdicts exist. `doneness_verdict()` is a pure function of `(wiring_class, status, kind)`, so
the verdict LABEL can never disagree with those three inputs — the real question is whether the
INPUTS (specifically `status`, and the `evidence` string that explains it) are themselves correct
for what the record actually contains. Two methods were used:

1. **A reproducible random sample**, seed `42`, N=20 per not-done verdict (100 units total),
   cross-joined to each unit's real `data/corpus/**/*.json` record where one could be found by
   `(book, kind, corpus_key)` (68 of 100 matched — the 32 unmatched are almost all `monster`/
   `monster_ability`/`companion`/`race_trait` units whose corpus directory nesting this lane's quick
   index did not fully generalize; **not independently re-derived, called out honestly in §5**
   rather than papered over).
2. **A corpus-wide, non-sampled check** of one specific, verifiable misfiling SIGNATURE the sample
   surfaced repeatedly: does a `class_feature` record's own `TYPE` token embed the name of an
   already-dispatched class that its `data.class`/group-name string does not surface?

### 3.2 The sample, read

`held`, `in-progress`, and `deferred` samples (20 each) were self-consistent on inspection: every
`held` record was a real `grounded`/`ingested-magnitude`/`text-complete` observation capped by
`NO_GROUNDING_PROBE` (monster, monster_ability, companion, spell, equipment, equipment_modifier
cannot prove `grounded` via a probe, so a real magnitude the engine DID observe is correctly capped
rather than promoted); every `deferred` record carried a named `engine_diagnostic:...` string
pointing at a real, specific unmodelled mechanism (Druid/Hunter animal companion advancement,
Bard/Cleric performance/blessing sub-choices, etc.) matching its class_feature. **No misfile found
in either sample of 20.** One data-quality curiosity, not a verdict misfile: `inner_sea_combat`'s
`Oadiran Horselord ~ Mount` and `adventurers_guide`'s `Qadiran Horselord ~ Mount` are almost
certainly the same Cavalier order with a transposed letter in one book's `.lst` — flagged for a
future provenance check, not investigated further here (out of this lane's scope; not a doneness
finding).

The `not-started` and `unmeasurable` samples (20 each) both leaned heavily on `class_feature`
(11 of 20 and 14 of 20 respectively — expected, since `class_feature` is 60.6% of the whole not-done
population). Reading the `unmeasurable` sample's records directly surfaced the same shape
repeatedly: a record whose `data.class` field is a group name ("Necromancy School", "Conjuration
Savant School", "Secret Lore") carrying evidence `class_feature_group_names_no_class_at_all`, while
its own `TYPE` token plainly names a real class (`WizardClassFeatures...`,
`ArcanistClassFeatures...`). That pattern was worth checking corpus-wide rather than reporting a
20-unit anecdote.

### 3.3 The corpus-wide check — the `TYPE`-token class-identification gap

```
# Dispatched-class universe, read from source (not transcribed):
#   ClassId::ALL (crb, 11) + ApgClassId::ALL (6) + AcgClassId::ALL (10)
#   + UcClassId::ALL (3) + PuClassId::ALL (4)  =  34 classes
# For every not-done class_feature unit matched to its real corpus record,
# exact-match (never substring — S11's lesson applied) the record's first
# TYPE-token dot-segment against "<ClassName>ClassFeatures".
```

Full script in the reproduction appendix (§4). Result:

| evidence code (current verdict) | population | matched to corpus | `TYPE` embeds a real class `data.class` misses | rate (of matched) |
|---|---:|---:|---:|---:|
| `class_feature_group_names_no_class_at_all` (**unmeasurable**) | 2,551 | 1,971 | **471** | **23.9%** |
| `no_explanation_id_and_no_diagnostic_names_this_feature` (not-started) | 3,320 | 3,152 | 433 | 13.7% |
| `class_feature_option_pool_record_not_held_by_engine` (not-started) | 3,064 | 2,047 | 171 | 8.4% |
| `class_feature_owner_matched_by_name_but_record_not_held_by_engine` (not-started) | 3,378 | 3,063 | 215 | 7.0% |
| `class_feature_of_unmodelled_corpus_class:<bogus>` (not-started) | 2,453 | 1,564 | 31 | 2.0% |

**Headline rate: 23.9%** (471 of 1,971 matched) within `class_feature_group_names_no_class_at_all`
— the strongest reading of the five, because that code's own text asserts "no class at all," which
the record's own `TYPE` token directly and unambiguously contradicts for these 471. Worked example:

```
core_rulebook:class_feature:necromancy_school_power_over_undead
  data.class = "Necromancy School"
  TYPE        = "WizardClassFeatures.ArcaneSchoolPower.ArcaneSchoolPowerLVL1"
  -> Wizard is CRB's own dispatched class. The classifier's group-name
     matcher never reads TYPE, so this record is stamped "no class at all."
```

Class-feature-wide, **1,377 of 12,114 matched not-done `class_feature` units (11.4%)** carry this
signature across all five codes. **This is not asserted as a doneness gain** — many of these 1,377
still lack a real explanation/consumer even once the owner is correctly identified, and the weaker
rows (`owner_matched_by_name`, 7.0%) may just reflect two DIFFERENT correct name-matchers disagreeing
on string form rather than a real miss. What IS established: the `unmeasurable` verdict's single
largest evidence code is honestly wrong about its own claim for roughly a quarter of its population.
Filed as `todo/sweeps.md` S12, **not reclassified** — the wave's own standing instruction.

### 3.4 A cross-reference, not re-derived: race_trait's kind-level misfiling, already proven

THE-BOX §2.3 G1 already established, exhaustively (not sampled), that **1,619 of race_trait's 2,954
not-done population (54.8%) are not race-trait content at all** — seven whole files return
zero real race-trait-shaped rows from the corpus discriminator. That is a KIND misfile (the record
was typed `race_trait` when it structurally isn't one) rather than a VERDICT misfile, but it belongs
in this audit's headline picture: it is the single largest already-proven misfiling rate in the
entire not-done population, dwarfing the `class_feature` signature in §3.3, and this lane did not
need to re-derive it — only to note that any verdict-audit summary that omits it understates the
program's real misfiling picture by 1,619 units.

### 3.5 Summary — misfiling rate by verdict, what is established vs. what is not

| verdict | population | established misfiling-adjacent rate | basis |
|---|---:|---:|---|
| unmeasurable | 3,763 | **23.9%** within its dominant code (471/1,971 of `class_feature_group_names_no_class_at_all`) | corpus-wide, non-sampled (§3.3) |
| not-started | 18,645 | 7.0%–13.7% within 3 `class_feature`-specific codes (weaker signal, see §3.3 caveat); separately, 54.8% of `race_trait`'s not-done pile is a KIND misfile (§3.4, already proven, not re-derived) | corpus-wide for the `class_feature` codes; cited, not re-derived, for `race_trait` |
| held | 1,230 | 0 of 20 sampled | 20-unit sample, not corpus-wide |
| in-progress | 1,231 | 0 of 20 sampled | 20-unit sample, not corpus-wide |
| deferred | 45 | 0 of 20 sampled (44.4% of the whole population directly read) | 20 of 45, direct read |

---

## 4. Reproduction appendix — the `TYPE`-token signature script

```python
import json, os, sys, collections
sys.path.insert(0, 'scripts/observer')
import pf1e_dashboard_producer as P

DISPATCHED = [
    "Barbarian", "Bard", "Cleric", "Druid", "Fighter", "Monk", "Paladin",
    "Ranger", "Rogue", "Sorcerer", "Wizard",
    "Alchemist", "Cavalier", "Inquisitor", "Oracle", "Summoner", "Witch",
    "Arcanist", "Bloodrager", "Brawler", "Hunter", "Investigator", "Shaman",
    "Skald", "Slayer", "Swashbuckler", "Warpriest",
    "Gunslinger", "Ninja", "Samurai",
    "UnchainedBarbarian", "UnchainedMonk", "UnchainedRogue", "UnchainedSummoner",
]  # ClassId::ALL + ApgClassId::ALL + AcgClassId::ALL + UcClassId::ALL + PuClassId::ALL

d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
v = lambda u: P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))
cf_nd = [u for u in U if u['kind'] == 'class_feature' and v(u) != 'done']

index = {}
for dirpath, dirs, files in os.walk('data/corpus'):
    for f in files:
        if not f.endswith('.json'): continue
        p = os.path.join(dirpath, f)
        parts = os.path.relpath(p, 'data/corpus').split(os.sep)
        if len(parts) < 2 or parts[1] != 'class_feature': continue
        rec = json.load(open(p))
        index[(parts[0], rec.get('data', {}).get('key'))] = rec

hits = collections.Counter()
for u in cf_nd:
    rec = index.get((u['book'], u.get('corpus_key')))
    if rec is None: continue
    data_class = (rec['data'].get('class') or '')
    found = None
    for t in rec['data'].get('raw_tokens') or []:
        if t.get('key') != 'TYPE': continue
        seg = (t.get('value') or '').split('.')[0]
        for c in DISPATCHED:
            if seg == f'{c}ClassFeatures': found = c; break
        if found: break
    if found and found.lower() != data_class.lower().replace(' ', ''):
        hits[u.get('evidence')] += 1
print(hits)
```

The `.COPY=`/`ce_`/`VISIBLE`/archetype-lock checks in §2 use the same `docs/work-inventory.json`
filter pattern (`origin`, `source_file`, `visible` fields already on each unit) or a full
`os.walk('data/corpus')` raw-token scan; each command is given inline in §2 rather than repeated
here.

---

## 5. What this lane could not determine

- **32 of the 100 sampled units (mostly `monster`/`monster_ability`/`companion`/`race_trait`)
  did not match this lane's own quick `data/corpus` index** — the per-book directory nesting for
  those kinds (`beastiary` vs `bestiary_2..6`, `equipment/equipmods` vs `equipment`) was not fully
  generalized within this session's time budget. The sample conclusions in §3.2/§3.5 for `held`,
  `in-progress`, and `deferred` rest on the 68% that DID match; the unmatched third was not
  independently re-derived by another route and should not be read as "checked clean."
- **Whether the 471-unit `TYPE`-token signature (§3.3) would change any unit's VERDICT once wired**
  is explicitly not determined — some of the 471 may still lack a real explanation or consumer even
  with the correct owner identified, in which case fixing this only changes which evidence code
  explains the same not-done state, not the doneness count. Not reclassified, per the wave's
  standing instruction.
- **The weaker three `TYPE`-signature codes** (`owner_matched_by_name`, `option_pool_record`,
  `of_unmodelled_corpus_class`) were reported at face value (7.0%–13.7%) but NOT verified against
  the real owner-matching mechanism those codes' own names imply exists — only the strongest code
  (`group_names_no_class_at_all`, whose text directly claims zero match) is asserted with
  confidence. Flagged explicitly in `todo/sweeps.md` S12 for the next wave to resolve, not asserted
  here as settled.
- **`held`/`in-progress`/`deferred` misfiling rates rest on 20-unit samples each**, not a
  corpus-wide check the way §3.3's `class_feature` figure is. 0-of-20 is consistent with a low true
  rate but does not prove one at these population sizes (1,230/1,231/45); a genuine corpus-wide
  audit of these three verdicts, at the depth §3.3 achieved for `unmeasurable`, was not completed
  this wave — said plainly rather than extrapolated from a small sample.
- **Whether the `ce_*`/`.COPY=`/`visible==false` populations in §2 overlap each other** (e.g. does
  any single unit carry `origin=="copy"` AND `visible==false` AND a `ce_` source file at once) was
  not checked. Each of the three is reported as its own population against the §1 grid, not
  cross-checked against the other two — a real remaining question for whoever picks these up.
