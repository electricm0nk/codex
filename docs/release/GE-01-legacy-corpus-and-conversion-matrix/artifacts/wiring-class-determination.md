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
  static       2601   64.2%
  derived       959   23.7%
  computed      445   11.0%
  ambiguous      26    0.6%
  display        19    0.5%
  dual-signal (derived AND computed) 97
```

64.2% of the stalled bucket is **static datum**: a longsword's `COST:15`/`WT:4`, `Rod (Enemy Detection)`'s `COST:23500 WT:5` (`core_rulebook/cr_equip_magic_items.lst:452`). These have a real number that must be *right*, but the number is a constant on the row. It is not a function of character state (so not `computed`) and there is nothing to evaluate (so not `derived`). Folding it into either would either overstate the work or understate the check. It gets its own class.

The `derived` class the operator described is real and is 23.7% of the bucket — 959 units. It is not the majority.

## The taxonomy

`wiring_class` is orthogonal to `status`. `status` says *how far along this unit is*; `wiring_class` says *what finishing it requires*. Every unit carries exactly one class and a `wiring_class_signals` list (below).

| class | the record's magnitude is… | DONE means | today's nearest status |
|---|---|---|---|
| `display` | absent — the row carries no magnitude-bearing token at all | the record is present and its description renders on screen | `text-complete` |
| `static` | one or more literal constants on the row itself | the stored value is byte-equal to the corpus literal, and is rendered or consumed | (none — currently `ingested-magnitude`) |
| `derived` | a deterministic function of a character or item scalar (caster level, class level, BAB, an ability modifier, the item's own `PLUSTOTAL`) | the evaluator returns the correct value at sampled inputs against a fixture | (none — currently `ingested-magnitude`) |
| `computed` | dependent on character state through a conditional guard, a temporary effect, or a player choice | a real consumer observes a delta — **today's `grounded` bar, unchanged** | `grounded` |
| `ambiguous` | **not a class.** Determination failed or found scaling stated only in English prose | nothing. Never provable while ambiguous | — |

`ambiguous` exists so that a determination failure is visible rather than silently defaulted into the cheapest class. It is a work item (resolve the record), not a state of completion.

### Ordering
The classes form a strict evidence lattice: `display` < `static` < `derived` < `computed`. **Highest bar wins.** A unit carrying both a `derived` and a `computed` signal is `computed`. This ordering is the structural anti-gaming property: a determinator change can only move a unit *down* the lattice by removing a signal, and removing a signal is exactly what the GE-09 audit looks for.

### Units that are two classes at once
368 held units carry both a `derived` and a `computed` signal — 3.7% of the 9,828 held units (`dual-signal` line in the determinator output above). `advanced_class_guide/acg_equip.lst:332` *Amulet of the Spirits (Heavens)* is the shape:

```
BONUS:SKILL|TYPE.Charisma|max(0,WIS)|TYPE=WisdomBonus          <- derived
SPROP:increase effective level of mystery or spirit powers by 2|PREABILITY:1,...  <- computed
```

A formulaic main effect with a guarded rider. **Representation:** `wiring_class` holds the single collapsed class (`computed`, by highest-bar-wins) and `wiring_class_signals` retains the full signal set, e.g. `["derived:bonus", "computed:pre_guard"]`. Reporting keys off `wiring_class`; the audit and any future per-effect breakdown key off `wiring_class_signals`. **The signal list is not optional** — without it, a dual-class unit is indistinguishable from a purely-`computed` one and the audit in GE-09 cannot run.

## Determination

The determinator reads the raw tab-delimited `.lst` row named by the unit's `source_file`/`source_line`. It is a signal scan followed by a lattice collapse; there is no per-unit judgement anywhere in it. A dependency-free reference implementation ships as `./wiring-class-determination.py` — that file is a documentary artifact, not production code; the real determinator belongs in the work-inventory generator.

### Magnitude-bearing fields
The set of magnitude-bearing tab prefixes is `MAGNITUDE_TOKENS` in `src/bin/v06_work_inventory.rs` (currently 20 prefixes: `BONUS:`, `TEMPBONUS:`, `DEFINE:`, `COST:`, `WT:`, `CR:`, `AC:`, `ACCHECK:`, `DAMAGE:`, `CRITMULT:`, `CRITRANGE:`, `RANGE:`, `REACH:`, `MOVE:`, `HITDIE:`, `LEVELADJUSTMENT:`, `SR:`, `DR:`, `SPELLFAILURE:`, `STAT:`). **The determinator MUST NOT fork this list.** It classifies what the generator already selects; a second copy would drift, and the two would disagree about which units even have a magnitude.

Let `M` = the row's magnitude-bearing fields.

### Rules, in order

**D0 — no resolvable corpus line → `ambiguous:no_corpus_line`.**
47 held units hit this: 46 `core_essentials` race/race-trait units and one ACG class feature (`pfs_acg_abilities_class.lst:115` *Extra Performance*). These are synthetic targets the generator injects without corpus provenance. They are honestly unclassifiable and must not be assumed `display`.

**D1 — `M` is empty → `display`.**
A row with no magnitude token has no number to compute. A `PRE*` guard or a `CHOOSE:` on such a row gates *text*, not a magnitude, and does not promote it. This is the corpus half of the operator's standing text-complete ruling, unchanged.

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
| `derived:prose_expr` | a parenthesised expression containing a scalar inside `DESC:`, `DURATION:`, `TARGETAREA:`, `SPROP:`, `RANGE:`, or `SPECIALS:` | `core_rulebook/cr_spells.lst` *Fireball*: `DESC:…deals (min(10,CASTERLEVEL))d6 points of fire damage…`; *Burning Hands*: `(min(5,CASTERLEVEL))d4`; `DURATION:(CASTERLEVEL) minutes` |
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

**D4 — otherwise, a prose-scaling phrase in a prose field → `ambiguous:prose_scaling_phrase`.**
29 held units. `advanced_class_guide/acg_spells.lst:14` *Air Geyser* is the type case: `DESC:…deals 2d6 points of bludgeoning damage and hurls the target upward a number of feet equal to 5 x your caster level.` The `2d6` is a literal; the displacement is level-scaling stated only in English. The row is neither honestly `static` nor mechanically `derived`. Detection phrases: `per (caster) level`, `per N levels`, `x/times your caster level`, `every N levels`, `caster level (max…`.

**D5 — otherwise → `static`.** Every magnitude on the row is a literal constant.

### Fallback and visibility
There is no silent default. Determination either lands on one of the four classes with a named signal, or it lands on `ambiguous` with a named reason. Both `wiring_class` and the reason MUST be emitted per unit. GE-09 requires `ambiguous` to be rendered as a first-class value and forbids counting it toward coverage; the ambiguity is a work item, not a rounding error.

## Validation against the existing classifier
The determinator agrees with the generator's own `text-complete` rule on **2,368 of 2,390 units (99.1%)**:

```
$ python3 - <<'PY'
import json,collections,importlib.util
s=importlib.util.spec_from_file_location('w','docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py')
w=importlib.util.module_from_spec(s); s.loader.exec_module(w)
U=json.load(open('docs/work-inventory.json'))['units']
tc=[u for u in U if u['status']=='text-complete']
cl=lambda u: w.wiring_class(w.signals(w.corpus_line(u['book'],u['source_file'],u['source_line'])))
print(len(tc), collections.Counter(cl(u)[0] for u in tc))
PY
2390 Counter({'display': 2368, 'derived': 18, 'static': 3, 'ambiguous': 1})
```

That 99.1% agreement is the strongest available evidence that the determinator is measuring the same thing the generator already measures, on a rule the operator has already ruled correct.

**The 22 disagreements are a real, small over-claim in today's `proven` number.** They are counted `text-complete` — and therefore `proven` — because `magnitude_token_count == 0`, yet they carry a magnitude the token count cannot see. All 22 are in `advanced_players_guide/apg_spells.lst` except one:

| n | units | signal |
|---|---|---|
| 12 | `Summon Monster I`–`IX`, `Summon Eidolon`, `Unfetter`, `Unravel Destiny` | `derived:range_keyword` **and** `derived:prose_expr` |
| 6 | `Evolution Surge` ×3, `Rejuvenate Eidolon` ×3 | `derived:prose_expr` |
| 3 | `Malediction`, `Purified Calling`, `Transmogrify` | `static:literal_magnitudes_only` |
| 1 | `pfs_acg_abilities_class.lst:115` `Extra Performance` | `ambiguous:no_corpus_line` |

Under this taxonomy the 18 `derived` units become provable — but only once the evaluator exists, and the 1 `ambiguous` unit is not provable at all. They must not be quietly left in `proven` in the interim; GE-09's transition rule covers this.

## Distribution across held units

```
$ python3 docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py HELD
inventory docs/work-inventory.json generated_at 2026-08-02T04:02:12Z
scope HELD  n=9828
  display      3882   39.5%
  static       3024   30.8%
  computed     1570   16.0%
  derived      1276   13.0%
  ambiguous      76    0.8%
  dual-signal (derived AND computed) 368
per book:
   book                         held  display  static  derived  computed ambiguous
   core_rulebook                4743     1003    2304      718       696        22
   advanced_class_guide         2527     1544     278      210       488         7
   advanced_players_guide       2466     1334     439      308       384         1
   bestiary                       46        1       3       40         2         0
   core_essentials                46        0       0        0         0        46
```

The finding that matters for planning: of `core_rulebook`'s 4,743 held units, **696 (14.7%) genuinely require bespoke engine wiring**. The other 4,025 are reachable by three mechanical checks — a literal comparison, a formula evaluation, and a render assertion. Corpus-wide the same split is 1,570 `computed` against 8,182 addressable.

## What this artifact does not decide
- It does not change any `status` value, and it does not by itself move one unit into `proven`. It reclassifies the *remaining work*; the evaluators still have to be built and run.
- It does not choose the evaluator. That is GE-04's `Candidate-selection boundary`.
- It does not define the aggregate coverage formula or the audit. Those are GE-09's.
- It does not extend `MAGNITUDE_TOKENS`. If a token family is missing from that list, that is a GE-01 conversion-matrix / unsupported-token-ledger question, resolved there and inherited here.
