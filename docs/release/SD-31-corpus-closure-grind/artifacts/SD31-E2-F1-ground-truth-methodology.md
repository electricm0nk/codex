---
canonical: true
owner: sd31-e2-groundtruth
purpose: Methodology note for the SD31-E2-F1 hand-labelled ground-truth sample (Epic 2's F1 gate).
cycle: SD31-E2-F1-001
date: 2026-08-15
data_file: SD31-E2-F1-ground-truth-sample-v1.json
---

# SD31-E2-F1 — Hand-labelled ground-truth sample, methodology note

**Card:** `epic-2-verdict-paths`, feature seed SD31-E2-F1 only. **No classifier code was written this
cycle** — this is the gate that runs first, per `epic-breakdown.md` "## Epic 2 (SD31-E2)" and
`decisions.md` Decision 1(e).

## What this is

150 units, hand-labelled from the corpus record (the whole record — base `.lst` row plus every
`.MOD` row targeting it, read directly under `$PCGEN_CORPUS_ROOT`, plus the unit's full JSON object
in `docs/work-inventory.json`), each with a `wiring_class` label, the token evidence that decides it,
and a confidence. The labelling method applies
`docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.md`'s
D0–D6 rules directly to the corpus tokens I read — the same taxonomy the production determinator
(`src/rules_core/wiring_class.rs`) implements, but arrived at independently per unit rather than by
running that code, so the sample can serve as an accuracy check on it (Decision 1(e) item 1: *"a
sample ... is hand-labelled ... before the classifier is written"*).

## How units were sampled

Fixed seed (`random.seed(31)`, chosen for SD-31) over `docs/work-inventory.json`'s 38,521 non-
`beginner_box` units (`beginner_box` excluded — matches the live dashboard producer's own exclusion).

1. **General stratified sample — 70 units.** For each of the five `wiring_class` values
   (`display`, `computed`, `static`, `derived`, `ambiguous`), draw 14 units stratified proportionally
   by `kind` within that class (every kind present gets at least 1 if the budget allows, remainder
   allocated proportional to that kind's population within the class).
2. **Oversample `ambiguous` — 40 more units**, population `wiring_class == 'ambiguous'` (re-derived
   2,109 — see below), same proportional-by-kind stratification, drawn from the pool minus whatever
   the general sample already picked. Tagged `"population": "ambiguous_target"`.
3. **Oversample `display`+`grounded` — 40 more units**, population `wiring_class == 'display' AND
   status == 'grounded'` (re-derived 1,243 — see below), same method, tagged
   `"population": "display_grounded_target"`. This is the population AT-31-010 binds into Epic 2's
   acceptance.

150 total after de-duplication (no unit drawn twice).

**Reproducibility gap (`NOTED 2026-08-15`, Opus adversarial-review CONFIRMED finding).** Neither the
sampling script (`sample_units.py`, `random.seed(31)`) nor the two evidence-extraction scripts
described above were committed alongside this artifact — `git diff --stat` between `origin/tranche/11`
and this branch's merge-base shows 6 files, all under `docs/`, zero `.py`/`.rs`. The stratified draw
therefore cannot be re-run or independently audited by a later cycle, and the claim that the
evidence-extraction scripts were "pure evidence extraction, no verdict output" — the assertion that
keeps this cycle inside Decision 1(e) item 1's "no classifier before the sample" gate — cannot be
checked by anyone but the labeller. Tracked as `OPEN-ISSUES.md` row 4 (owner: Epic 2, `NOTE`):
commit the sampling script (or inline it verbatim) alongside any future re-draw so the draw is
independently reproducible.

**Re-derived population counts** (commands, run against this cycle's checkout of
`docs/work-inventory.json`):

```
$ python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json'))
units = [u for u in d['units'] if u.get('book') != 'beginner_box']
c = collections.Counter(u.get('wiring_class') for u in units)
print(c)
amb = [u for u in units if u.get('wiring_class')=='ambiguous']
dg = [u for u in units if u.get('wiring_class')=='display' and u.get('status')=='grounded']
print('ambiguous', len(amb))
print('display+grounded', len(dg))
"
Counter({'display': 14366, 'computed': 8477, 'static': 7394, 'derived': 6175, 'ambiguous': 2109})
ambiguous 2109
display+grounded 1243
```

Both match `decisions.md` Decision 1(e)'s corrected figures (2,109 / 1,243) exactly — re-derived
independently, not transcribed.

## Stratification actually achieved

**By `wiring_class`** (engine's current label, 150 total):

| class | units | kinds represented |
|---|---:|---|
| `display` | 54 | class_feature(6), feat(3), monster_ability(30), race_trait(2), equipment_modifier(1), companion(8), spell(1), equipment(1), race(1), class(1) |
| `computed` | 14 | class_feature(3), race_trait(2), companion(1), monster_ability(1), equipment_modifier(1), feat(1), equipment(1), spell(1), class(1), monster(1), race(1) |
| `static` | 14 | equipment(4), class_feature(2), monster_ability(1), spell(1), race_trait(1), companion(1), feat(1), equipment_modifier(1), race(1), monster(1) |
| `derived` | 14 | spell(2), class_feature(2), monster(2), monster_ability(1), companion(1), race_trait(1), equipment(1), feat(1), race(1), equipment_modifier(1), class(1) |
| `ambiguous` | 54 | class_feature(21), race_trait(14), feat(4), spell(3), equipment(3), monster_ability(2), race(2), companion(2), monster(2), class(1) |

All five wiring classes represented; **11 of 11 corpus kinds** represented overall (well above the
"at least four" floor). `display` and `ambiguous` run larger than the other three because the two
oversampled populations (`ambiguous_target`, `display_grounded_target`) both draw exclusively from
those two classes — by design, per the card's "oversample the two populations Epic 2 must actually
decide" instruction.

**Population tags:** `null` (general sample) 70, `ambiguous_target` 40, `display_grounded_target` 40.

**Stratification depth caveat (`NOTED 2026-08-15`, Opus adversarial-review CONFIRMED finding).** The
sample meets the letter of SD31-E2-F1's own size/stratification floor (150 ≥ 100, all 5 classes, 11
kinds ≥ 4), but is thin against SD31-E2-F2's *own* acceptance criterion, which requires the agreement
rate "reported per class AND per kind, plus its full confusion matrix":
```
python3 -c "
import json, collections
d = json.load(open('docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E2-F1-ground-truth-sample-v1.json'))
print(collections.Counter(r['kind'] for r in d))
ct = collections.Counter((r['hand_wiring_class'], r['kind']) for r in d)
print(len(ct), 'occupied cells,', sum(1 for v in ct.values() if v <= 2), 'with n<=2')
"
# -> 45 occupied (class, kind) cells, 31 of them n<=2
```
Every unit of *independently-evidenced* signal in the sample sits almost entirely in the 40
`no_corpus_line`-tagged units plus 5 strays (engine-class evidenced-unit counts: `display` 0,
`computed` 0, `static` 2, `derived` 3, `ambiguous` 40 — see the Result section's Correction 2 for the
full evidence-quality picture). **F2 must not report a per-class/per-kind rate for a cell this sample
does not defensibly cover**; either a future re-draw widens thin cells, or F2 states explicitly which
cells it is not permitted to report on. Tracked as `OPEN-ISSUES.md` row 5 (owner: Epic 2, `NOTE`).

## Labelling method

For each sampled unit: resolve its book directory under `$PCGEN_CORPUS_ROOT`
(`/home/ubuntu/workspace/repos/pcgen/data/pathfinder/...`, pinned to oracle SHA
`7f818006e371188e5717fd18d74d18a420747fc6`), read the base row at `source_file`/`source_line`
(searched recursively under the book directory, not assumed to sit at the book root — see Finding A),
then search every `.lst` file in that book tree for a `.MOD` row whose resolved base name
(`CATEGORY=<x>|<Base>.MOD` → `<Base>`; `CLASS:<Base>.MOD` → `<Base>`; bare `<Base>.MOD` → `<Base>`)
matches the unit's `name` or `corpus_key` — the token closure, per the GE-01 doc. Apply D0–D6 in
order: no resolvable line → `ambiguous:no_corpus_line`; a `TEMPBONUS:`/`CHOOSE:`/`%CHOICE`/`PRE<X>:`
guard (excluding `PRERULE`) on a magnitude-bearing closure → `computed`; a magnitude field whose value
carries a recognized scalar (`CASTERLEVEL`, `CLASSLEVEL`, `BAB`, `HD`, `STR`/`DEX`/`CON`/`INT`/`WIS`/
`CHA`, `TL`, `CL`, `RACESIZE`, ...) or arithmetic, a `RANGE:Close/Medium/Long` keyword, or a `%N`
prose placeholder whose pipe-segment carries one of those → `derived`; a prose field stating a
scaling phrase (`per level`, `per caster level`, `your character level`, ...) or an ability-score
phrase with a nearest-preceding grant construction (`add`, `gain`, `equal to`, `plus`, ... vs.
`lose`, `instead of`, ...) → `ambiguous`; otherwise empty magnitude set → `display`; otherwise
→ `static`.

Two small Python scripts (not a classifier — pure evidence extraction, no verdict output) did the
mechanical parts: (1) pulling each sampled unit's base row + `.MOD` closure verbatim from the corpus
tree and any ingested `data/corpus/<book>/<kind>/*.json`, and (2) flagging which fields carry a
`MAGNITUDE_TOKENS` prefix, a `PRE*` guard, `CHOOSE`/`%CHOICE`/`TEMPBONUS`, or a candidate D4 phrase —
so I could go read every flagged occurrence in context rather than re-deriving the phrase list from
memory. Every final class assignment below was made by reading the actual row text, not by trusting
either script's flag.

## Judgement calls on ambiguous records

1. **Bare cross-referenced, non-literal, non-scalar variable values** (e.g.
   `BONUS:VAR|FavoredHumanoidChangeling|FavoredBaseBonus`, `BONUS:VAR|FighterWeaponQualifyLVL|MonkLVL`
   — the referenced variable is set by a *different* record entirely, most likely a class's own
   level-scaling chassis, but this unit's own closure carries no literal number and no recognized
   scalar for it). I labelled these `ambiguous` (a determination failure, per the taxonomy's own
   philosophy — see the GE-01 doc's "Accursed" worked example) rather than `static`, at
   **medium** confidence. The production code's own fallback (`static:literal_magnitudes_only`
   inserted whenever no computed/derived signal fires and the magnitude set is non-empty) would call
   these `static`, which I judge to be a narrower, related accuracy gap — distinct from Finding B
   below because it is a genuine definitional gray zone, not a clear miss. 2 of 150 sampled units hit
   this (`core_essentials:race_trait:favored_enemy_humanoid_changeling`,
   `ultimate_combat:class_feature:martial_artist_martial_arts_master`).
2. **A `PRE*` guard scoped to record eligibility rather than to the magnitude itself**
   (`core_essentials:race:changeling`'s `PREGENDER:F` — Changeling is only playable as female; its
   `MOVE:Walk,30` isn't itself conditional on anything once the race is taken). The mechanical D2 rule
   as coded doesn't distinguish "guard on the record's own eligibility" from "guard on this specific
   magnitude" — any `PRE*` field anywhere on a magnitude-bearing row counts. I labelled `computed` for
   consistency with the rest of the sample's guard-bearing race traits (same shape as
   `android_ability_scores`, `sylph_ability_scores`, etc., all confirmed `computed`), but at
   **medium** confidence, flagging the scope distinction as a live question for Epic 2-F2.
3. **Prose ability-score grants the mechanical grant/refer word-list discriminator doesn't cleanly
   match** (`horror_adventures:class_feature:exciter_rapture`'s "a +4 morale bonus to his Strength and
   Constitution scores ... normally gained from bloodrage" — the discriminator's word list has
   `gain`/`gains`/`gaining` but not the past tense `gained`, and `bonus to` rather than the listed
   `bonus of`). I read this as a genuine D4b ambiguous case (a real magnitude stated only in English,
   no structured token for it) at **medium** confidence, coincidentally landing on the same class the
   engine assigns via the unrelated `no_corpus_line` bug (see Finding A) — the class matches, the
   reason does not.

## Finding A (load-bearing) — the `no_corpus_line` bucket is 97% path-resolution bug, not genuine ambiguity

`wiring_class::CorpusLines::line()` (`src/rules_core/wiring_class.rs:758`) resolves a unit's row via
`dir.join(file)` — a **single join, one level deep**. Several books' `.lst` files live in nested
subdirectories the join cannot reach: `core_essentials/races/<race>/*.lst`,
`ultimate_combat/support/*.lst`, `horror_adventures/support/*.lst`,
`inner_sea_world_guide/_pfs/*.lst`, `advanced_race_guide/_pfs/*.lst`,
`adventurers_guide/support/*.lst`, `bestiary_2/_pfs/*.lst`, and more. The join fails,
`std::fs::read_to_string` returns `Err` (swallowed by `.unwrap_or_default()`), and the unit falls
through D0 to `ambiguous:no_corpus_line` — indistinguishable, in the JSON, from a genuinely
synthetic/provenance-free target.

Re-derived corpus-wide:

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
units = [u for u in d['units'] if u.get('book') != 'beginner_box']
ncl = [u for u in units if u.get('wiring_class_reason') == 'no_corpus_line']
print(len(ncl))
"
1707
```

The GE-01 doc (`wiring-class-determination.md` "D0 — no resolvable corpus line") documents this
bucket at **47** units, "46 core_essentials race/race-trait units and one ACG class feature ...
honestly unclassifiable". That figure is now stale by 36x — corpus growth and/or determinator changes
since that doc was written have inflated the bucket from 47 to 1,707, but the doc's own claim ("these
are synthetic targets... honestly unclassifiable") no longer holds.

**Every single one of the 1,707 is resolvable.** Recursive search confirms:

```
$ python3 - <<'PY'
import json, os, glob, subprocess
d = json.load(open('docs/work-inventory.json'))
units = [u for u in d['units'] if u.get('book') != 'beginner_box']
ncl = [u for u in units if u.get('wiring_class_reason') == 'no_corpus_line']
all_books = sorted(set(u['book'] for u in ncl))
root = '/home/ubuntu/workspace/repos/pcgen/data/pathfinder'
dirs = subprocess.run(['find', root, '-maxdepth', '4', '-type', 'd'], capture_output=True, text=True).stdout.split('\n')
bookmap = {b: next((p for p in dirs if p.rsplit('/',1)[-1]==b), None) for b in all_books}
found_nested = missing = 0
for u in ncl:
    bdir = bookmap.get(u['book'])
    sf = u.get('source_file')
    if not bdir or not sf: missing += 1; continue
    if os.path.isfile(os.path.join(bdir, sf)): continue  # would already resolve today
    if glob.glob(os.path.join(bdir, '**', sf), recursive=True): found_nested += 1
    else: missing += 1
print('findable-if-recursive:', found_nested, 'genuinely missing:', missing)
PY
findable-if-recursive: 1707 genuinely missing: 0
```

**1,707 of 1,707 (100%).** Not one of the corpus-wide `no_corpus_line` units is genuinely
provenance-free under the current corpus; all are misfiled by the path join. This is **80.9% of the
whole 2,109-unit `ambiguous` population.**

Within this 150-unit sample, **40 units** carry `wiring_class_reason == 'no_corpus_line'**. I
hand-resolved each from its real (recursively-found) row and applied D0–D6 directly — see the
`OVERRIDES` entries in the data file, each carrying `"no_corpus_line_bug"` in `token_evidence`. The
sample-level redistribution (40 units; **not** a corpus-wide extrapolation — Epic 2-F2 must re-derive
the true redistribution once the join is fixed and the classifier re-run):

| true class (my hand label) | sampled units |
|---|---:|
| `display` | 19 |
| `computed` | 12 |
| `static` | 3 |
| `derived` | 4 |
| `ambiguous` (genuine — real prose/variable-reference reasons, not the bug) | 2 |

Worked example — `inner_sea_gods:monster:the_first_blade`: engine says `ambiguous:no_corpus_line`
(zero rows resolved for this monster at all). Its real row (`isg_races_b4.lst:6`, found under
`campaign_setting/inner_sea_gods/`, one level deeper than the join reaches) carries
`BONUS:STAT|STR|22`, `BONUS:VAR|AC_Natural_Armor|20|TYPE=Base`, `DR:15/Adamantine`, `CR:15`, and
~10 more literal magnitude fields with zero `PRE*` guards anywhere in the row — an unambiguous
`static` record that the engine currently cannot see at all.

**Not a blocker to this card** — the sample itself is unaffected, since every affected unit was
hand-labelled from its real row rather than trusted at the engine's `ambiguous` verdict. It is
load-bearing for Epic 2-F2 (classifier build/accept) and F3 (`ambiguous` dead-end closure): F3's
"genuinely unreachable, propose to the Structural Exclusion Register" review must not run against the
current, 97%-artificially-inflated `no_corpus_line` bucket, and F2's classifier-vs-ground-truth
accuracy numbers will be dominated by this single root cause unless the path join is fixed first (or
F2 explicitly carves the `no_corpus_line` population out and reports it separately). Logged to
`OPEN-ISSUES.md` row 1 (`NOTE`, not `BLOCKER` — informational, feeds Epic 2-F2).

## Finding B — `BONUS:STAT` selector name and `DR:`/`CR:` "/" notation both trip the scalar/arithmetic scanner as false positives for `derived`

`wiring_class::signals()`'s scalar/arithmetic check (`has_scalar_or_arith`, `src/rules_core/
wiring_class.rs:398`) scans a magnitude field's **whole value string**, not just the numeric portion.
Two consequences:

- `BONUS:STAT|<ABILITY>|<value>` always carries the ability abbreviation (`STR`/`DEX`/`CON`/`INT`/
  `WIS`/`CHA`) as the STAT *selector*, and those same six abbreviations are in `SCALARS_WORD` (used
  elsewhere to detect genuinely scalar-dependent formulas like `10+HD/2+CON`). The selector collides
  with the scalar list regardless of whether `<value>` is itself scalar-dependent — a completely flat
  `BONUS:STAT|DEX|2|TYPE=Racial` reads as `derived:bonus`.
- `has_arith`'s `value.contains('/')` check is unconditional. PCGen's own `DR:10/Cold Iron`,
  `DR:10/-`, `CR:1/3` notation uses `/` as a literal type/fraction separator, not division; every such
  field trips `derived:dr`/`derived:cr`.

Three sampled units hit this as a **clean, single-cause** misclassification (no other signal present
to independently justify `derived`):

- `core_rulebook:race_trait:2_dexterity` — sole field `BONUS:STAT|DEX|2|TYPE=Racial` (plus
  `STACK:NO MULT:NO`, confirming a plain fixed modifier). Engine: `derived`. True: `static`.
- `ultimate_equipment:equipment:staff_of_mithral_might` — sole scalar-tripping field
  `BONUS:STAT|INT|2|TYPE=Enhancement`; every other field (`COST`/`WT`/`CRITMULT`/`CRITRANGE`/`DAMAGE`)
  is a flat literal. Engine: `derived`. True: `static`.
- `bestiary:monster:neothelid` — both mechanisms compound (`BONUS:STAT|STR|20` etc. **and**
  `DR:10/Cold Iron`); every magnitude field in the row is a literal constant, zero `PRE*` guards.
  Engine: `derived`. True: `static`.

Several other sampled `derived` units carry the **same** false-positive signal alongside a genuinely
separate scalar/arithmetic field (e.g. `core_essentials:companion:pig`'s
`BONUS:WEAPONPROF=Bite|DAMAGE|max(0,(STR/2))`, `bestiary:monster:dog_riding`'s identical pattern,
`bestiary:race:iron_cobra_adamantine_cobra`'s `BONUS:HP|CURRENTMAX|HD*5`) — those remain correctly
`derived` because the real signal independently justifies it; the false positive there is noise, not
an error in the final class. Logged to `OPEN-ISSUES.md` row 2 (`NOTE`).

## Finding C (minor) — case-sensitive scalar matching misses lowercase PCGen function calls

`has_scalar`'s `SCALARS_SUBSTRING` check (`value.contains(s)`, `s` uppercase) is case-sensitive.
`ultimate_magic:class_feature:dragon_shaman_totem_transformation`'s
`BONUS:VAR|TotemTransformationDuration|classlevel("Druid")` is an unambiguous class-level-scaling
formula, but lowercase `classlevel(...)` doesn't match `"CLASSLEVEL"`, and the field contains no
`*`/`/`/`+`/`min(`/`max(` for `has_arith` to catch either. Engine: `static`. True: `derived`
(high confidence — the formula is unambiguous once read). One unit in this sample; not quantified
corpus-wide this cycle (out of scope — flagged for Epic 2-F2 to re-derive if it chooses to fix the
case sensitivity).

## Result

**CORRECTED 2026-08-15 (`SD31-W1-INTEGRATE-001`, fixing two Opus adversarial-review CONFIRMED
findings against this section).** The table and the two headline percentages below were wrong or
unusable as originally published. Both defects and their fixes are recorded here rather than
silently edited in place, per this program's correction convention.

| | count |
|---|---:|
| Sampled units | 150 |
| Agree with engine's current `wiring_class` | 107 |
| Disagree | 43 |
| ...of which: `no_corpus_line` bug (Finding A) | **38** (corrected from 40 — see below) |
| ...of which: `BONUS:STAT`/`DR`/`CR` "/" false positive (Finding B) | 3 |
| ...of which: case-sensitive scalar miss (Finding C) | included above (1, also counted toward the 43 — see note below) |

**Correction 1 — the `no_corpus_line` attribution was 40, the true figure is 38.** Two of the 40
`no_corpus_line`-tagged units actually **agree** with the engine (both land on `ambiguous` for a
different reason than the engine's, so they are not disagreements at all): re-derived by
```
python3 -c "
import json
d = json.load(open('docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E2-F1-ground-truth-sample-v1.json'))
ncl = [r for r in d if 'no_corpus_line_bug' in r['token_evidence']]
print(len(ncl), sum(1 for r in ncl if not r['agrees_with_engine']), sum(1 for r in ncl if r['agrees_with_engine']))
print([r['id'] for r in ncl if r['agrees_with_engine']])
"
# -> 40 38 2
# -> ['core_essentials:race_trait:favored_enemy_humanoid_changeling', 'horror_adventures:class_feature:exciter_rapture']
```
38 (no_corpus_line disagreements) + 3 (Finding B) + 1 (Finding C, subset of Finding B's 3) + 1
(judgement-call unit `martial_artist_martial_arts_master`) = 43, the disagreement total, with no
double count: the 2 agreeing `no_corpus_line` units are excluded from every disagreement bucket.
Exact per-unit accounting is in `SD31-E2-F1-ground-truth-sample-v1.json`'s `agrees_with_engine`
field — trust that field, not this prose summary, for the authoritative count. Retro `correction`
event emitted this cycle: subject this file's original Result table, claimed 40, actual 38,
`--verified-by` the command above.

**Correction 2 — the 95.5% and 71.3% headline agreement figures are WITHDRAWN.** Both numbers
were composed almost entirely of unevidenced rows and must not be cited, quoted, or acted on
(including as grounds for a Decision 1(e) item 4 "close Epic 2 at F1" call) until the artifact is
re-labelled. Re-derived:
```
python3 -c "
import json
d = json.load(open('docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E2-F1-ground-truth-sample-v1.json'))
BOIL = \"confirmed from the unit's full token closure\"
isb = lambda r: r['token_evidence'].startswith(BOIL)
non = [r for r in d if 'no_corpus_line_bug' not in r['token_evidence']]
ag = [r for r in non if r['agrees_with_engine']]
print('excl-no_corpus_line:', len(non), 'agree:', len(ag), 'of-those-boilerplate:', sum(isb(r) for r in ag))
for pop in (None, 'ambiguous_target', 'display_grounded_target'):
    g = [r for r in d if r['population'] == pop]
    print(pop, 'n=', len(g), 'agree=', sum(r['agrees_with_engine'] for r in g), 'boilerplate=', sum(isb(r) for r in g))
"
# -> excl-no_corpus_line: 110 agree: 105 of-those-boilerplate: 105   (the withdrawn "95.5%")
# -> None n=70 agree=57 boilerplate=56 | ambiguous_target n=40 agree=10 boilerplate=9
# -> display_grounded_target n=40 agree=40 boilerplate=40             (the withdrawn "71.3%" traces to the
#    same defect: 105 of 150's agreements carry a single canned string, not a quoted token)
```
105 of the 150 labels — including **all 40** of the `display_grounded_target` population (the exact
population AT-31-010 binds into Epic 2's acceptance) and all of engine classes `display` (54/54) and
`computed` (14/14) — carry the identical boilerplate string `"confirmed from the unit's full token
closure ... -- matches engine's own wiring_class and reason"` as their `token_evidence`, which quotes
no token from the record and is a restatement of the engine's own output. Every one of those 105
agrees with the engine (105/105), so the field is a perfect function of `agrees_with_engine`, not of
the record — it cannot detect a systematic engine error in the one population Epic 2 exists to test.
**No agreement/accuracy rate may be quoted from this artifact** until those 105 units (and
particularly the 40 `display_grounded_target` units) are re-labelled with an actual quoted token or
an explicit absence-of-token statement drawn from that unit's own row. Tracked as `OPEN-ISSUES.md`
row 3 (owner: Epic 2, `BLOCKER`-severity for any F1-close decision, not for this integration cycle).

**What still stands, unwithdrawn.** The 45 non-boilerplate, record-evidenced labels (43 disagree, 2
agree) are unaffected by Correction 2 — they carry real quoted evidence and their per-unit findings
(A, B, C above) are load-bearing as written. Per Decision 1(e) item 4: this sample does **not** show
"the current classifier substantially correct and any contradiction rare" across the *whole* board —
the `no_corpus_line` bug alone means 80.9% of the `ambiguous` population is misclassified by a fixable
defect, which is neither "substantially correct" for that population nor "rare". SD31-E2-F2
(classifier build) is therefore **in scope**, but its first task per this sample's evidence should be
fixing `CorpusLines::line()`'s path resolution (Finding A) before any new classifier logic is
evaluated against ground truth — otherwise F2's accuracy numbers measure the path bug, not
classification quality. This conclusion does not depend on either withdrawn headline figure.
