---
title: GE-01 Wiring-Class Determination
stc_id: STC-CODEX-GE-01
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts
source_stc: ../README.md
related:
  - ./conversion-matrix.csv
  - ./pilot-token-taxonomy.csv
  - ./wiring-class-determination.py
  - ../../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
  - ../../GE-04-rules-engine-and-explainability-core/artifacts/expression-language-runtime-requirements.md
  - ../../GE-09-expansion-packaging-and-release-governance/artifacts/coverage-dashboard-requirements.md
---

# GE-01 Wiring-Class Determination

## Purpose
Define `wiring_class` — a second, orthogonal axis on every corpus unit that states **what kind of evidence would prove that unit done** — and specify how that class is determined mechanically from the PCGen record, with no per-unit human judgement.

This artifact is the single normative definition. GE-02 carries the field on the canonical object, GE-04 carries the evaluator requirement for one class, and GE-09 carries the aggregation and audit rules. Those three packages **cite** this file; none of them restates the class definitions.

## Why a second axis exists
Today's `status` axis (`src/bin/v06_work_inventory.rs`, `status_vocabulary`) does two jobs at once. `grounded` means *a real computed magnitude was OBSERVED reaching a consumer*. That test can only ever pass for a unit whose magnitude comes from bespoke engine wiring, so every unit whose magnitude is a plain function of a scalar — a spell dealing `(min(10,CASTERLEVEL))d6` — sits in `ingested-magnitude` permanently and reads as unfinished even when the engine is entirely correct about it.

Measured, on `docs/work-inventory.json` generated `2026-08-02T04:02:12Z`:

```
$ python3 - <<'PY'
import json,collections
U=json.load(open('docs/work-inventory.json'))['units']
cr=[u for u in U if u['book']=='core_rulebook']
held={'grounded','ingested-magnitude','text-complete','deferred-with-reason'}
print(len(cr), collections.Counter(u['status'] for u in cr))
print('held', sum(1 for u in cr if u['status'] in held))
print('proven', sum(1 for u in cr if u['status'] in ('grounded','text-complete')))
print('corpus-wide', collections.Counter(u['status'] for u in U))
PY
5716 Counter({'ingested-magnitude': 3062, 'not-ingested': 973, 'text-complete': 791,
              'unknown': 762, 'grounded': 121, 'deferred-with-reason': 7})
held 3981
proven 912
corpus-wide Counter({'not-started': 27696, 'not-ingested': 6667, 'ingested-magnitude': 4050,
                     'unknown': 3107, 'text-complete': 2390, 'grounded': 252,
                     'deferred-with-reason': 29})
```

`core_rulebook`: 5,716 units, 3,981 held by the engine (69.6%), 912 `proven` (16.0%), 3,062 stalled in `ingested-magnitude`. Corpus-wide `ingested-magnitude` is 4,050.

## Correction to the three-way framing
The operator's proposal names three kinds: *needs full wiring*, *just needs basic math*, *display only*. **The data does not partition into three; it partitions into four.** The single largest bucket is neither of the first two.

Reproduce with the reference determinator shipped alongside this file:

```
$ python3 docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py ingested-magnitude
inventory docs/work-inventory.json generated_at 2026-08-02T04:02:12Z
scope ingested-magnitude  n=4050
  static       2562   63.3%
  derived       898   22.2%
  computed      511   12.6%
  ambiguous      63    1.6%
  display        16    0.4%
  dual-signal (derived AND computed) 161
```

63.3% of the stalled bucket is **static datum**: a longsword's `COST:15`/`WT:4`, `Rod (Enemy Detection)`'s `COST:23500 WT:5` (`core_rulebook/cr_equip_magic_items.lst:452`). These have a real number that must be *right*, but the number is a constant on the row. It is not a function of character state (so not `computed`) and there is nothing to evaluate (so not `derived`). Folding it into either would either overstate the work or understate the check. It gets its own class.

The `derived` class the operator described is real and is 22.2% of the bucket — 898 units. It is not the majority.

## The taxonomy

`wiring_class` is orthogonal to `status`. `status` says *how far along this unit is*; `wiring_class` says *what finishing it requires*. Every unit carries exactly one class and a `wiring_class_signals` list (below).

| class | the record's magnitude is… | DONE means | today's nearest status |
|---|---|---|---|
| `display` | absent — **no row in the unit's token closure** carries a magnitude-bearing token, and no prose row states a magnitude | the record is present and its description renders on screen | `text-complete` |
| `static` | one or more literal constants on the row itself | the stored value is byte-equal to the corpus literal, and is rendered or consumed | (none — currently `ingested-magnitude`) |
| `derived` | a deterministic function of a character or item scalar (caster level, class level, BAB, an ability modifier, the item's own `PLUSTOTAL`) | the evaluator returns the correct value at sampled inputs against a fixture | (none — currently `ingested-magnitude`) |
| `computed` | dependent on character state through a conditional guard, a temporary effect, or a player choice | a real consumer observes a delta — **today's `grounded` bar, unchanged** | `grounded` |
| `ambiguous` | **not a class.** Determination failed or found scaling stated only in English prose | nothing. Never provable while ambiguous | — |

`ambiguous` exists so that a determination failure is visible rather than silently defaulted into the cheapest class. It is a work item (resolve the record), not a state of completion.

### Ordering
The classes form a strict evidence lattice: `display` < `static` < `derived` < `computed`. **Highest bar wins.** A unit carrying both a `derived` and a `computed` signal is `computed`. This ordering is the structural anti-gaming property: a determinator change can only move a unit *down* the lattice by removing a signal, and removing a signal is exactly what the GE-09 audit looks for.

**`ambiguous` outranks `display`, and this is not a detail.** `display` is the *last* resort, reached only when no other signal fired — never a short circuit. A record can carry no magnitude token at all and still state a magnitude in prose; `ultimate_campaign`'s story feats do exactly that (worked below). If `display` short-circuited, such a unit would be marked done the moment its text rendered, which is precisely the over-claim this axis exists to prevent. Resolution order is: no-corpus-line → `computed` → `derived` → `ambiguous` → `display` → `static`.

### Units that are two classes at once
470 held units carry both a `derived` and a `computed` signal — 4.8% of the 9,828 held units (`dual-signal` line in the determinator output above). `advanced_class_guide/acg_equip.lst:332` *Amulet of the Spirits (Heavens)* is the shape:

```
BONUS:SKILL|TYPE.Charisma|max(0,WIS)|TYPE=WisdomBonus          <- derived
SPROP:increase effective level of mystery or spirit powers by 2|PREABILITY:1,...  <- computed
```

A formulaic main effect with a guarded rider. **Representation:** `wiring_class` holds the single collapsed class (`computed`, by highest-bar-wins) and `wiring_class_signals` retains the full signal set, e.g. `["derived:bonus", "computed:pre_guard"]`. Reporting keys off `wiring_class`; the audit and any future per-effect breakdown key off `wiring_class_signals`. **The signal list is not optional** — without it, a dual-class unit is indistinguishable from a purely-`computed` one and the audit in GE-09 cannot run.

## Determination

The determinator reads the unit's **token closure** (below), scans it for signals, and collapses the signal set through the lattice. There is no per-unit judgement anywhere in it. A dependency-free reference implementation ships as `./wiring-class-determination.py` — that file is a documentary artifact, not production code; the real determinator belongs in the work-inventory generator.

### Magnitude-bearing fields
The set of magnitude-bearing tab prefixes is `MAGNITUDE_TOKENS` in `src/bin/v06_work_inventory.rs` (currently 20 prefixes: `BONUS:`, `TEMPBONUS:`, `DEFINE:`, `COST:`, `WT:`, `CR:`, `AC:`, `ACCHECK:`, `DAMAGE:`, `CRITMULT:`, `CRITRANGE:`, `RANGE:`, `REACH:`, `MOVE:`, `HITDIE:`, `LEVELADJUSTMENT:`, `SR:`, `DR:`, `SPELLFAILURE:`, `STAT:`). **The determinator MUST NOT fork this list.** It classifies what the generator already selects; a second copy would drift, and the two would disagree about which units even have a magnitude.

`BENEFIT:` is deliberately **not** proposed for that list. It is prose, not a magnitude token; it enters determination through the prose-field scan in D3/D4 instead. 2,087 corpus rows carry it.

### The token closure — which rows govern a unit

**A unit's classification surface is not one row.** It is:

> the unit's base declaration row, **plus every `.MOD` row in the same book whose resolved base name matches the unit's `name` or `corpus_key`.**

**Why this rule is mandatory rather than a refinement.** A `<Name>.MOD` row modifies an existing base record rather than declaring one, so the work-inventory generator emits no unit for it (`src/bin/v06_work_inventory.rs` ~line 546 counts it into `trap_hits["mod_record"]`, stashes it into `mod_targets`, and returns). Its magnitude count is consumed **only** by the `mod_only_rescue` path — that is, only when the base name appears nowhere else in the corpus. When a base declaration does exist, the `.MOD` row's magnitudes are discarded and never reach the base unit's `magnitude_token_count`. The exclusion is a real, documented precedent (`src/rules_core/rules_tables/pathfinder_unchained/feat_tables.rs`, module doc: *"`CATEGORY=FEAT|Extra Rogue Talent.MOD` … modifies an existing Advanced Player's Guide feat's prerequisite rather than defining a new feat of its own … Deliberately excluded, mirroring `rules_tables::crb::feats`'s own documented precedent"*), and it is correct **for enumeration** — a `.MOD` row is not a new record. It is wrong for **classification**: the magnitude is still there, it just lives on a different row.

Measured scale of the exposure:

```
$ python3 - <<'PY'
import os,collections
C='/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game'
MAG=("BONUS:","TEMPBONUS:","DEFINE:","COST:","WT:","CR:","AC:","ACCHECK:","DAMAGE:","CRITMULT:",
     "CRITRANGE:","RANGE:","REACH:","MOVE:","HITDIE:","LEVELADJUSTMENT:","SR:","DR:","SPELLFAILURE:","STAT:")
tot=mod=mod_mag=ben=0
for root,_,files in os.walk(C):
    for fn in files:
        if not fn.endswith('.lst'): continue
        for ln in open(os.path.join(root,fn),encoding='utf-8',errors='replace'):
            if not ln.strip() or ln.lstrip().startswith('#'): continue
            f=[x.strip() for x in ln.split('\t') if x.strip()]
            if not f: continue
            tot+=1
            if any(x.startswith('BENEFIT:') for x in f): ben+=1
            if '.MOD' in f[0]:
                mod+=1
                if any(x.startswith(MAG) for x in f): mod_mag+=1
print(tot, mod, mod_mag, ben)
PY
96540 20831 8234 2087
```

**8,234 `.MOD` rows carry a magnitude token** and are structurally invisible to per-unit classification without the closure. 1,895 of the 9,828 held units (19.3%) have at least one `.MOD` row targeting them, and **294 change class** once the closure is applied:

```
   derived    -> computed     86        display    -> computed     27
   display    -> static       79        static     -> computed     12
   display    -> ambiguous    50        display    -> derived       4
   static     -> ambiguous    33        static     -> derived       2
                                        ambiguous  -> static        1
```

293 of the 294 move **up** the lattice, 160 of them out of `display`. That direction is the whole point: without the closure, magnitudes hide on `.MOD` rows and their units read as text-only, which marks them done the moment text renders.

**Base-name resolution MUST mirror the generator's own** (`CATEGORY=<x>|<Base>.MOD` → `<Base>`; `CLASS:<Base>.MOD` → `<Base>`), or the two components will disagree about which record a `.MOD` row belongs to and the closure will silently miss rows.

**Closure signal union.** Signals are unioned across every row in the closure. The `display` signal survives only if **no** row in the closure carries a magnitude-bearing field.

Let `M` = the magnitude-bearing fields across the unit's **whole token closure**.

### Rules, in order

**D0 — no resolvable corpus line → `ambiguous:no_corpus_line`.**
47 held units hit this: 46 `core_essentials` race/race-trait units and one ACG class feature (`pfs_acg_abilities_class.lst:115` *Extra Performance*). These are synthetic targets the generator injects without corpus provenance. They are honestly unclassifiable and must not be assumed `display`.

**D1 — `M` is empty → `display` is a CANDIDATE, not a verdict.**
A closure with no magnitude token has no tokenised number to compute. A `PRE*` guard or a `CHOOSE:` on such a closure gates *text*, not a magnitude, and does not promote it. This is the corpus half of the operator's standing text-complete ruling, unchanged. But D1 does **not** terminate determination: D4's prose scan still runs, because a record with no magnitude token can still state a magnitude in English. `display` is emitted only if D2, D3 and D4 all decline.

**D2 — otherwise, any of these → `computed`:**

| signal | token evidence | why |
|---|---|---|
| `computed:tempbonus` | a `TEMPBONUS:` field | a temporary, activation-scoped effect; nothing about it is a pure function of level. `advanced_players_guide/apg_spells.lst:160` *Lead Blades*: `TEMPBONUS:EQ\|Weapon,Melee\|COMBAT\|DAMAGESIZE\|1\|TYPE=Temporary` |
| `computed:choice` | `%CHOICE` anywhere, or a `CHOOSE:` field | the magnitude is a function of a player selection the engine must model. `core_rulebook/cr_equipmods.lst:34` *Composite Bow Strength Rating*: `BONUS:WEAPON\|DAMAGE\|MIN(%CHOICE,STR)` |
| `computed:pre_guard` | a `PRE<X>:` or `!PRE<X>:` guard, at field start or after a `\|`, **excluding `PRERULE`** | the magnitude applies conditionally on character state |

**`PRERULE` is excluded and this exclusion is load-bearing.** `!PRERULE:1,DisplayFullSpell` is a renderer directive appended to essentially every spell row. Counting it as a guard classified 1,019 of 1,067 spells as `computed` in an earlier pass of this analysis — a 23× overstatement of the wiring bar for the spell kind. Any reimplementation that drops the exclusion will reproduce that error, and it will look like a *conservative* error, which is how it survives review.

**D3 — otherwise, any of these → `derived`:**

| signal | token evidence | scalar |
|---|---|---|
| `derived:<token>` | a field in `M` whose value matches a character/item scalar (`CASTERLEVEL`, `CLASSLEVEL`, `TOTALLEVELS`, `BAB`, `HD`, `PLUSTOTAL`, `SPELLLEVEL`, `STR`/`DEX`/`CON`/`INT`/`WIS`/`CHA`, `TL`, `CL`, `RACESIZE`) or carries arithmetic (`*`, `/`, `+<VAR>`, `min(`, `max(`) | e.g. `core_rulebook/cr_equipmods.lst:263` *Amulet of Mighty Fists*: `COST:4000*PLUSTOTAL*PLUSTOTAL` |
| `derived:prose_expr` | a parenthesised expression containing a scalar inside `DESC:`, `DURATION:`, `TARGETAREA:`, `SPROP:`, `RANGE:`, `SPECIALS:`, or `BENEFIT:` | `core_rulebook/cr_spells.lst` *Fireball*: `DESC:…deals (min(10,CASTERLEVEL))d6 points of fire damage…`; *Burning Hands*: `(min(5,CASTERLEVEL))d4`; `DURATION:(CASTERLEVEL) minutes` |
| `derived:range_keyword` | `RANGE:` whose value is exactly `Close`, `Medium`, or `Long` | PCGen keyword ranges are caster-level functions: Close = 25 ft + 5 ft/2 CL, Medium = 100 ft + 10 ft/CL, Long = 400 ft + 40 ft/CL. 474 of the 1,067 `ingested-magnitude` spells carry one (Close 290, Medium 132, Long 52) |

The `RANGE:` keyword tally is reproduced by:

```
$ python3 - <<'PY'
import json,os,collections
C='/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game'
U=json.load(open('docs/work-inventory.json'))['units']
sp=[u for u in U if u['status']=='ingested-magnitude' and u['kind']=='spell']
c=collections.Counter()
for u in sp:
    L=open(os.path.join(C,u['book'],u['source_file']),errors='replace').read().split('\n')
    for f in L[u['source_line']-1].split('\t'):
        if f.strip().startswith('RANGE:'): c[f.strip()[6:]]+=1
print(len(sp), c.most_common(5), sum(v for k,v in c.items() if k.strip() in ('Close','Medium','Long')))
PY
1067 [('Close', 290), ('Touch', 237), ('Personal', 158), ('Medium', 132), ('Long', 52)] 474
```

The operator's headline case is therefore **machine-determinable after all** — but not from a `BONUS:` token. It lives as a parenthesised PCGen expression inside `DESC:`, and `DESC:` is not in `MAGNITUDE_TOKENS`. Any determinator that scans only the magnitude tokens will miss every scaling spell in the corpus.

**D4 — a prose-scaling phrase in any prose field of the closure → `ambiguous:prose_scaling_phrase`.**
240 held units. Two type cases:

- `advanced_class_guide/acg_spells.lst:14` *Air Geyser*: `DESC:…deals 2d6 points of bludgeoning damage and hurls the target upward a number of feet equal to 5 x your caster level.` The `2d6` is a literal; the displacement is level-scaling stated only in English.
- `ultimate_campaign/uca_feats.lst:59` *Accursed* (a `.MOD BENEFIT:` row): `You gain spell resistance equal to 5 + your character level…` No magnitude token anywhere in the closure, and the formula exists only as prose.

Neither is honestly `static`, and neither is mechanically `derived` — the determinator cannot produce the formula without guessing. **Semantically these are derived magnitudes; mechanically they are undetermined, and `ambiguous` is the honest verdict.** The work item is to give the record a machine-readable magnitude, not to have a human eyeball the formula into a class.

Detection phrases: `per (caster) level`, `per N levels`, `x`/`times your caster level`, `every N levels`, `caster level (max…`, `your character/class/total level`, `per hit die`/`per HD`, `your <Ability> score/modifier/bonus`.

**D5 — otherwise, `M` is empty → `display`.** Nothing in the closure states a magnitude, in tokens or in prose.

**D6 — otherwise → `static`.** Every magnitude in the closure is a literal constant.

### Fallback and visibility
There is no silent default. Determination either lands on one of the four classes with a named signal, or it lands on `ambiguous` with a named reason. Both `wiring_class` and the reason MUST be emitted per unit. GE-09 requires `ambiguous` to be rendered as a first-class value and forbids counting it toward coverage; the ambiguity is a work item, not a rounding error.

## Worked hard case — a magnitude that lives on a different row

Found by the `epic-13-calibration` actor working `ultimate_campaign`, verified here independently. It is the case that forced the token closure and the `ambiguous`-over-`display` precedence, and it is worth walking end to end because it defeats three separate rules at once.

All 23 `ultimate_campaign` units report `magnitude_token_count: 0`:

```
$ python3 -c "
import json,collections
U=json.load(open('docs/work-inventory.json'))['units']
uc=[u for u in U if u['book']=='ultimate_campaign']
print(len(uc), collections.Counter(u['magnitude_token_count'] for u in uc), collections.Counter(u['status'] for u in uc))"
23 Counter({0: 23}) Counter({'not-started': 23})
```

And the whole file contains no magnitude token of any kind — its only tab prefixes are `TYPE:`, `SOURCEPAGE:`, `PRETEXT:`, `DESC:`, `CATEGORY:`, `BENEFIT:`, 23 of each. Yet *Accursed* has a real, formulaic magnitude. The unit's three rows:

```
uca_feats.lst:7   Accursed  CATEGORY:FEAT  TYPE:Story  PRETEXT:Prerequisite:…  SOURCEPAGE:p.67
uca_feats.lst:33  CATEGORY=FEAT|Accursed.MOD  DESC:[Not Implemented] Your curse weighs down your soul…
uca_feats.lst:59  CATEGORY=FEAT|Accursed.MOD  BENEFIT:You gain spell resistance equal to 5 + your
                  character level, as the curse interferes with all magic. …
```

**Three independent reasons a naive determinator files this as `display`, all of which had to be fixed:**

1. The magnitude is on a `.MOD` row, and `.MOD` rows are structurally excluded from the unit — fixed by the **token closure**.
2. The magnitude is in `BENEFIT:`, which is in no magnitude-token list and was in no prose-field list — fixed by adding `BENEFIT:` to the **prose-field scan**. Note that fixing only (1) changes nothing here: the `.MOD` rows carry no magnitude token either, so a closure alone still yields `M = ∅`. Both fixes are required, and an implementer who lands only the closure will believe the case is handled when it is not.
3. `display` was short-circuiting ahead of `ambiguous` — fixed by the **precedence order** above.

**Verdict: `ambiguous:prose_scaling_phrase`, not `derived`, and emphatically not `display`.** *"spell resistance equal to 5 + your character level"* is semantically a textbook derived magnitude — character level is a scalar the engine already holds, and SD-28 `decisions.md §27` rules that computing it is display-value work rather than engine work. But the corpus states it in English with no machine-readable expression, so **determination cannot produce the formula without guessing, and guessing is what `ambiguous` exists to prevent.** The work item is to give the record a machine-readable magnitude; once it has one, D3 will classify it `derived` on its own.

The rule discriminates rather than blanket-demoting: 6 of the 23 carry a prose-scaling phrase and become `ambiguous`; the other 17 carry no magnitude at all in any row of their closure and remain honestly `display`.

```
$ python3 -c "
import json,collections,importlib.util
s=importlib.util.spec_from_file_location('w','docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py')
w=importlib.util.module_from_spec(s); s.loader.exec_module(w)
U=json.load(open('docs/work-inventory.json'))['units']
uc=[u for u in U if u['book']=='ultimate_campaign']
print(collections.Counter(w.wiring_class(w.closure_signals(w.token_closure(u)))[0] for u in uc))
a=[u for u in uc if u['name']=='Accursed'][0]
print(sorted(w.closure_signals(w.token_closure(a))), w.wiring_class(w.closure_signals(w.token_closure(a))))"
Counter({'display': 17, 'ambiguous': 6})
['ambiguous:prose_scaling_phrase', 'display:no_magnitude_token'] ('ambiguous', 'prose_scaling_phrase')
```

### Does the four-way split survive this? Yes — and the case is what makes `ambiguous` load-bearing

This is the sharpest evidence that a taxonomy of *classes alone* is insufficient. The record has a magnitude, that magnitude is a plain function of a held scalar, and the corpus will not say so mechanically. Any three- or four-class scheme with no failure value must file it somewhere, and every available destination is a lie: `display` says there is no number, `static` says the number is constant, `derived` says the engine can evaluate it. `ambiguous` is the only truthful answer, and it is why the value is specified as mandatory rather than as a convenience.

## Upstream `[Not Implemented]` markers — a different claim, kept separate

Every one of the 23 `ultimate_campaign` `.MOD DESC:` rows is prefixed literally `[Not Implemented]` — PCGen's own stock data admitting the record is not mechanically implemented upstream.

**This is an upstream-completeness signal and it is not a `wiring_class` input.** Determination MUST NOT read it, in either direction:

- A `[Not Implemented]` marker MUST NOT make a unit `ambiguous`, `display`, or anything else. What upstream did or did not implement says nothing about what *this* record's tokens contain, and *Accursed* proves the point — it is marked `[Not Implemented]` and still carries a fully specified benefit formula.
- Conversely, a unit MUST NOT be treated as done on the strength of a `[Not Implemented]` `DESC:` alone. Rendering upstream's admission of incompleteness is not rendering the record's benefit.

Instead the marker is carried as its own reported field, `upstream_implementation_marker` (boolean, plus the marker text), and reported beside `wiring_class` without ever feeding it. The two answer different questions — *did upstream implement this?* versus *what evidence would prove we have?* — and a report that conflates them will read an upstream gap as our own, or our completeness as upstream's.

The reference determinator counts the marker and prints it on a separate line, explicitly labelled `reported, never classifying`.

**Bounded by the `epic-13-calibration` owner's standing ruling, which this specification does not disturb:** for those 23 units, displaying the full accurate benefit text (`DESC` + `BENEFIT`) is what keeps them from being stubs, and `text-complete` there is real work rather than papering over a gap. That ruling is about `status`; `wiring_class` is a different axis. A unit may legitimately be `status: text-complete` — we render accurate text, it is not a stub — while carrying `wiring_class: ambiguous`, meaning a scaling magnitude exists that we cannot yet determine mechanically. Nothing here reclassifies that work as incomplete; it records that one further thing is known to be outstanding.

## Validation against the existing classifier

The determinator agrees with the generator's own `text-complete` rule on **2,130 of 2,390 units (89.1%)**:

```
$ python3 - <<'EOF'
import json,collections,importlib.util
s=importlib.util.spec_from_file_location('w','docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py')
w=importlib.util.module_from_spec(s); s.loader.exec_module(w)
U=json.load(open('docs/work-inventory.json'))['units']
tc=[u for u in U if u['status']=='text-complete']
cl=lambda u: w.wiring_class(w.closure_signals(w.token_closure(u)))
print(len(tc), collections.Counter(cl(u)[0] for u in tc))
EOF
2390 Counter({'display': 2130, 'ambiguous': 128, 'static': 73, 'derived': 32, 'computed': 27})
```

**Correction to an earlier figure in this artifact.** Before the token closure and the `BENEFIT:` prose scan were added, this section reported 99.1% agreement and 22 disagreements. Both were measured correctly, against a determinator that could not see `.MOD` rows or `BENEFIT:` prose. The real figures are **89.1% and 260 disagreements**. The earlier number was not merely imprecise; it was reassuring in the wrong direction. Near-perfect agreement with the existing rule read as evidence that the determinator was sound, when in fact both components shared the same blind spot. **Agreement with a rule that is wrong in a known way is not validation** — and a cross-check between two components built on the same assumption will always look like confirmation.

**The 260 disagreements are a real over-claim in today's `proven` number** — 10.9% of `text-complete`. They are counted `text-complete`, and therefore `proven`, because `magnitude_token_count == 0`, yet they carry a magnitude the token count cannot see:

| n | class under closure | what the token count missed |
|---|---|---|
| 128 | `ambiguous` | a magnitude stated only in prose, largely on a `.MOD BENEFIT:` row |
| 73 | `static` | a literal magnitude on a `.MOD` row |
| 32 | `derived` | the `RANGE:Close` level-scaling keyword, or a parenthesised `CASTERLEVEL` expression |
| 27 | `computed` | a guarded or temporary magnitude on a `.MOD` row |

Of these, 132 become provable once their class's evidence exists; the 128 `ambiguous` units are not provable at all until the underlying records carry a machine-readable magnitude. None may be quietly left in `proven` in the interim; GE-09's transition rule covers this.

**The invariant that does hold, and the one worth auditing.** Agreement between the two components is not symmetric and must not be audited as if it were. The correct one-directional invariant is:

> **No unit with `magnitude_token_count > 0` may ever be classified `display`.**

Verified at 0 violations across all 9,828 held units:

```
$ python3 -c "
import json,importlib.util
s=importlib.util.spec_from_file_location('w','docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py')
w=importlib.util.module_from_spec(s); s.loader.exec_module(w)
U=json.load(open('docs/work-inventory.json'))['units']
HELD=('ingested-magnitude','text-complete','grounded','deferred-with-reason','unknown')
held=[u for u in U if u['status'] in HELD]
cl=lambda u: w.wiring_class(w.closure_signals(w.token_closure(u)))[0]
print(len(held), sum(1 for u in held if u['magnitude_token_count']>0 and cl(u)=='display'))"
9828 0
```

Divergence in the other direction — the token count says zero, the determinator finds a magnitude — is expected, is the entire value of the token closure, and must not be treated as a regression.

## Distribution across held units

```
$ python3 docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py HELD
inventory docs/work-inventory.json generated_at 2026-08-02T04:02:12Z
scope HELD  n=9828
  display      3577   36.4%
  static       3046   31.0%
  computed     1695   17.2%
  derived      1224   12.5%
  ambiguous     286    2.9%
  dual-signal (derived AND computed) 470
  carrying upstream '[Not Implemented]' marker 0 (reported, never classifying)
per book:
   book                         held  display  static  derived  computed ambiguous
   core_rulebook                4743      912    2260      672       777       122
   advanced_class_guide         2527     1448     283      211       518        67
   advanced_players_guide       2466     1216     500      301       398        51
   bestiary                       46        1       3       40         2         0
   core_essentials                46        0       0        0         0        46
```

The finding that matters for planning: of `core_rulebook`'s 4,743 held units, **777 (16.4%) genuinely require bespoke engine wiring**. Another 3,844 are reachable by three mechanical checks — a literal comparison, a formula evaluation, and a render assertion — and 122 must first be disambiguated. Corpus-wide the same split is 1,695 `computed` against 7,847 addressable and 286 ambiguous.

## What this artifact does not decide
- It does not change any `status` value, and it does not by itself move one unit into `proven`. It reclassifies the *remaining work*; the evaluators still have to be built and run.
- It does not choose the evaluator. That is GE-04's `Candidate-selection boundary`.
- It does not define the aggregate coverage formula or the audit. Those are GE-09's.
- It does not extend `MAGNITUDE_TOKENS`. If a token family is missing from that list, that is a GE-01 conversion-matrix / unsupported-token-ledger question, resolved there and inherited here.
