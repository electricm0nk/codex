---
canonical: true
owner: sd31-e2-groundtruth
purpose: Methodology note for the SD31-E2-F1 hand-labelled ground-truth sample (Epic 2's F1 gate).
cycle: SD31-E2-F1-001, extended SD31-E2-F1-002
date: 2026-08-15
data_file: SD31-E2-F1-ground-truth-sample-v1.json
---

# SD31-E2-F1 — Hand-labelled ground-truth sample, methodology note

**Card:** `epic-2-verdict-paths`, feature seed SD31-E2-F1 (original draw, `SD31-E2-F1-001`) and its
repair (`SD31-E2-F1-002`, `OPEN-ISSUES.md` rows 3/4/5). **No classifier code was written in either
cycle** — this is the gate that runs first, per `epic-breakdown.md` "## Epic 2 (SD31-E2)" and
`decisions.md` Decision 1(e).

**`SD31-E2-F1-002` (this revision) did three things** to the artifact `SD31-E2-F1-001` produced, each
recorded in its own section below rather than silently folded into the original text:

1. **Re-labelled all 105 canned units** (`OPEN-ISSUES.md` row 3, `BLOCKER`) — see "Re-labelling the
   105 canned units" below.
2. **Widened the sample by 35 units** targeting thin `(hand_wiring_class, kind)` cells
   (`OPEN-ISSUES.md` row 5) using a newly-committed, seeded sampling script
   (`scripts/sample_ground_truth_units.py`) that also partially resolves `OPEN-ISSUES.md` row 4 — see
   "Widening the sample" below. **185 units total after this cycle**, not 150.
3. **Added a machine-checkable evidence-provenance guard** (`scripts/ground_truth_evidence_guard.py`
   + `scripts/tests/test_ground_truth_evidence_guard.py`) that would have caught the original 105-unit
   defect, and used it to find a further, smaller defect in the untouched 45 — see "The evidence guard"
   below.

## What this is

185 units (150 from `SD31-E2-F1-001`'s original draw + 35 from `SD31-E2-F1-002`'s widening draw),
hand-labelled from the corpus record (the whole record — base `.lst` row plus every `.MOD` row
targeting it, read directly under `$PCGEN_CORPUS_ROOT`, plus the unit's full JSON object in
`docs/work-inventory.json`), each with a `wiring_class` label, the token evidence that decides it, and
a confidence. The labelling method applies
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

150 total after de-duplication (no unit drawn twice). This is the `SD31-E2-F1-001` draw ("v1
draw" below); it is joined by a 35-unit widening draw from `SD31-E2-F1-002` (see "Widening the
sample" below), for **185 total** in the committed data file.

**Reproducibility gap — v1 draw only, NOT fully resolved this cycle (`OPEN-ISSUES.md` row 4,
originally `NOTE`d 2026-08-15, Opus adversarial-review CONFIRMED finding).** Neither the v1 sampling
script (`sample_units.py`, `random.seed(31)`) nor the two evidence-extraction scripts described above
were committed alongside the original artifact — `git diff --stat` between `origin/tranche/11` and
that branch's merge-base showed 6 files, all under `docs/`, zero `.py`/`.rs`. **`SD31-E2-F1-002` does
NOT reconstruct that missing script or the v1 draw** — a reconstruction could not prove the original
draw was unbiased, and presenting one as the original would be worse than admitting the gap plainly:
**the v1 150-unit draw remains permanently non-reproducible.** What this cycle DID do is commit a
real, runnable, seeded sampling script for every draw from here forward
(`scripts/sample_ground_truth_units.py`, `python3 -m unittest scripts/tests/test_sample_ground_truth_units.py`
proves it deterministic/stratifying/exclusion-respecting/verdict-free) and use it for the 35-unit
widening draw below — that draw, and only that draw, is independently re-runnable and auditable. The
claim that the v1 evidence-extraction scripts were "pure evidence extraction, no verdict output" still
cannot be checked by anyone but the original `SD31-E2-F1-001` labeller.

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

## Widening the sample (`SD31-E2-F1-002`, `OPEN-ISSUES.md` row 5)

The v1 draw left 30 of 44 occupied `(hand_wiring_class, kind)` cells at `n<=2` — too thin for
SD31-E2-F2's per-class-AND-per-kind agreement rate plus confusion matrix. `SD31-E2-F1-002` widened it:

```
$ python3 scripts/sample_ground_truth_units.py \
    --inventory docs/work-inventory.json \
    --exclude-ids-from <150-id list from the v1 draw> \
    --current-cell-counts <(hand_wiring_class, kind) -> count, computed from the v1 draw> \
    --target-per-cell 2 --seed 31 --out widening_draw.json
# -> drew 35 units across 28 (engine_wiring_class, kind) cells
```

**Stratification target is necessarily `engine_wiring_class`, not `hand_wiring_class`.** A pre-draw
script cannot stratify on a label that does not exist until a human reads the record — the same
constraint the v1 draw worked under. Given how large this cycle's relabelling correction rate turned
out to be (see "Re-labelling the 105 canned units" below), several widened units landed, once hand-
labelled, in a *different* cell than the one they were drawn to fill — the net effect on thin-cell
count was smaller than the 35-unit draw size might suggest (see "Stratification actually achieved"
below for the honest before/after).

All 35 were hand-labelled to the identical whole-record standard as the relabelled 105 (base row via a
recursive search, full `.MOD` closure, real quoted tokens, `confidence`, `corpus_path_verified`) — no
shortcut for being a smaller batch. **13 of the 35 (37%) disagreed with the engine**, a materially
higher rate than the v1 draw's non-`no_corpus_line` disagreement rate, because the widening draw was
necessarily concentrated in exactly the kinds/classes the v1 draw under-sampled, several of which turn
out to be exactly where `Finding A`'s bug and a previously-unnoticed variant of `Finding B` cluster —
see Findings D–F below.

## Stratification actually achieved

**By `wiring_class`** (engine's current label, v1 draw only, 150 total — unchanged from
`SD31-E2-F1-001`, reproduced here for its own record):

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

**Population tags (185 total, post-`SD31-E2-F1-002`):** `null` (v1 general sample) 70,
`ambiguous_target` (v1 oversample) 40, `display_grounded_target` (v1 oversample) 40,
`widening_batch_v2` (`SD31-E2-F1-002`) 35.

**Stratification depth caveat, UPDATED `SD31-E2-F1-002` — improved but NOT resolved.** The v1-draw
finding (`OPEN-ISSUES.md` row 5, `NOTE`) was: sample meets the letter of SD31-E2-F1's own size/
stratification floor (150 ≥ 100, all 5 classes, 11 kinds ≥ 4), but is thin against SD31-E2-F2's *own*
acceptance criterion, which requires the agreement rate "reported per class AND per kind, plus its
full confusion matrix" — 45 occupied `(hand_wiring_class, kind)` cells, 31 of them `n<=2`. Re-derived
against the current 185-unit file, by **hand-label** (the axis F2 actually needs, not the engine label
the pre-widen table above uses):
```
python3 -c "
import json, collections
d = json.load(open('docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E2-F1-ground-truth-sample-v1.json'))
ct = collections.Counter((r['hand_wiring_class'], r['kind']) for r in d)
print(len(ct), 'occupied cells,', sum(1 for v in ct.values() if v <= 2), 'with n<=2')
"
# -> 48 occupied (class, kind) cells, 29 of them n<=2
```
**The 35-unit widening draw moved this from 31/45 thin to 29/48 thin — a real but modest
improvement, not a fix.** Root cause: `scripts/sample_ground_truth_units.py` (see "Widening the
sample" above) can only stratify a pre-draw by `engine_wiring_class`, since `hand_wiring_class` does
not exist until a human reads the record — and this cycle's own correction rate (13 of 35 widened
units disagreed with the engine, on top of the v1 draw's own high disagreement rate in exactly the
populations that were oversampled) means a meaningful fraction of units drawn to fill an
engine-labelled thin cell landed, once hand-labelled, in a *different* cell instead. **F2 still must
not report a per-class/per-kind rate for a cell this sample does not defensibly cover** (`n<=2`); a
further widening pass stratified by `hand_wiring_class` from a prior partial draw (i.e. drawing again
only after seeing where THIS draw's hand-labels actually landed, repeating until each target cell's
gap closes) would do better, but was out of this cycle's bounded time budget after the 105-unit
relabel. `OPEN-ISSUES.md` row 5 stays open with this narrower, re-derived shape rather than closing.
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

## Finding D (`SD31-E2-F1-002`) — `SPELLS:` fields carrying a scalar-dependent formula are never scanned

`signals()` scans two field categories only: `MAGNITUDE_TOKENS`-prefixed fields, and `prose_fields`
(`DESC:`/`DURATION:`/`TARGETAREA:`/`SPROP:`/`RANGE:`/`SPECIALS:`/`BENEFIT:`). `SPELLS:` — PCGen's field
for granting a spell-like ability, e.g. `SPELLS:Ice Staff|CASTERLEVEL=10|Cone of Cold,15+CHA` — is in
neither list, so a save DC or other formula stated only inside a `SPELLS:` field is invisible to the
scanner regardless of how it is written.

- `bestiary_4:monster_ability:winter_hag_ice_staff` — engine: `display:no_magnitude_token` (the row's
  ONLY field of any kind carrying content is the `SPELLS:` grant; nothing else on the row is scanned).
  True: `derived` — `Cone of Cold,15+CHA` is an unambiguous CHA-scalar save-DC formula (`CHA` is in
  `SCALARS_WORD`). Contrast with `core_rulebook:race_trait:racial_sla_death_knell` elsewhere in this
  sample, whose SLA DC is built through a `BONUS:VAR` chain instead of an inline `SPELLS:` formula and
  is correctly scanned — the difference between the two units is purely which PCGen authoring
  convention was used for otherwise-equivalent content.
- `bestiary_2:monster:spriggan` (widening batch) independently carries the same pattern
  (`SPELLS:Innate|TIMES=ATWILL|CASTERLEVEL=4|Flare,10+CHA+SprigganMagicBonus|...`) but lands on
  `computed` anyway via a genuine, same-row `PREVAREQ` guard on a separate `BONUS:VAR` field — so this
  specific unit's verdict is unaffected, but it corroborates that the `SPELLS:`-field gap is not a
  one-off.

Not quantified corpus-wide this cycle (would require a `SPELLS:`-field grep across all 23 in-scope
books, out of this card's bounded scope) — flagged for Epic 2-F2.

## Finding E (`SD31-E2-F1-002`) — `PLUS:` fields (equipment-modifier equivalent-bonus values) are never scanned

Similarly, `PLUS:` — PCGen's field for a weapon/armor special ability's equivalent enhancement-bonus
value (used in cost/stacking calculations, e.g. `PLUS:3` for a +3-equivalent special ability) — is not
in `MAGNITUDE_TOKENS`. A record whose only magnitude is a `PLUS:` field falls through to
`display:no_magnitude_token` even though it carries a real, literal numeric value.

- `core_rulebook:equipment_modifier:special_ability_ghost_touch_armor` — engine: `display`. True:
  `static` (`PLUS:3`, a flat literal — not scalar-dependent, so `static` rather than `derived`).
- `ultimate_combat:equipment_modifier:special_ability_reliable_firearm` (widening batch) — engine:
  `display`. True: `static` (`PLUS:1`). Corroborates this is a recurring `equipment_modifier`-kind
  pattern, not a one-off.

Two units this cycle, both `equipment_modifier`. Not quantified corpus-wide (out of scope) — flagged
for Epic 2-F2; likely a small, cleanly-fixable addition to `MAGNITUDE_TOKENS`.

## Finding F (`SD31-E2-F1-002`) — `ASPECT:` fields carrying a scalar-dependent formula are never scanned, and `Finding B`'s false positive is not always rescued

Two related widening-batch findings, both concentrated in exactly the `no_corpus_line`-bug population
Finding A already flagged as load-bearing:

**F1 — `ASPECT:` fields.** Like `SPELLS:` (Finding D), `ASPECT:` is in neither `MAGNITUDE_TOKENS` nor
`prose_fields`. `bestiary_5:monster_ability:chuspiki_air_blast` states its real magnitude only there:
`ASPECT:Ability Benefit|(%1d6+%2 damage, 30-ft. range)|HD/2|HD/2+CON/2` — an unambiguous HD/CON-scalar
damage formula. This unit is a *double* gap: (1) its base row lives under
`support/b5_abilities_race_oa.lst`, only reachable by a recursive search, so it currently lands in
`ambiguous:no_corpus_line` (Finding A) and the engine never even reaches the row; (2) even a fixed
path join would still miss it, because `ASPECT:` is unscanned. True class: `derived`.

**F2 — `Finding B`'s false positive is not universally rescued.** `OPEN-ISSUES.md` row 2 documented
the `BONUS:STAT` selector-name / `DR:`-slash false positive and checked three sampled units, all of
which happened to also carry a genuinely separate scalar/arithmetic field that independently justified
`derived` — the false positive was noise, not an error, in every case row 2 checked. **That does not
hold universally.** Three widening-batch units carry the false positive with NO rescuing genuine
signal anywhere in their closure:

- `ultimate_equipment:equipment:belt_of_stoneskin` — sole trigger `DR:10/Adamantine` (bypass-slash false
  positive); `COST:60000`/`WT:10` are flat literals, nothing else on the row. Engine: `derived`. True:
  `static`.
- `bestiary_2:monster:twigjack` — every `BONUS:STAT` field is a selector-name false positive
  (`STR`/`DEX`/`CON`/`WIS`/`CHA`); every `BONUS:VAR` field (`AC_Natural_Armor`, `DarkvisionRange`,
  `SneakAttackDice`) is a flat literal. Engine: `derived`. True: `static`.
- `horror_adventures:race:undead_phantom` — same shape as `twigjack`: `BONUS:STAT` selector-name false
  positives only, `MOVE:Walk,30`/`REACH:5` flat literals. Engine: `derived`. True: `static`.

Also two related, judgement-call false positives discovered this cycle in `derived:prose_expr`, the
same general shape as Finding B but in the *prose* scanner rather than the field scanner: a
parenthesised group citing another spell's or ability's own FIXED stat block (using `CL`/`HD` as plain
abbreviations, e.g. `(Will DC 16, CL 9th)`, `(CL 1st; concentration +0)`) trips `has_scalar` exactly
like a genuine formula would, because the check cannot distinguish "this parenthetical states a
formula" from "this parenthetical cites a fixed number using a scalar's own abbreviation":

- `mythic_adventures:equipment:chaos_hammer` — the false-positive citation coexists with a SEPARATE,
  genuine `ambiguous:prose_scaling_phrase` signal ("increases to 1d10 points of damage per caster
  level") elsewhere in the same closure; `classify()`'s highest-bar-wins ordering lets the
  false-positive `derived` signal outrank the genuine `ambiguous` one. Engine: `derived`. True (medium
  confidence — a genuine judgement call): `ambiguous`.
- `bestiary_3:race_trait:fuath_spell_like_abilities` — no other signal anywhere in the closure once the
  false positive is discounted. Engine: `derived`. True (medium confidence): `display`.

**Six widening-batch units total for this finding, all `derived`→`static`/`ambiguous`/`display`
corrections** — the highest concentration of any single finding this cycle, and confirmation that
Finding B's "the false positive is always rescued" observation was itself only checked against 3
units and does not generalize.

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

## Re-labelling the 105 canned units (`SD31-E2-F1-002`, `OPEN-ISSUES.md` row 3)

**Every one of the 105 boilerplate `token_evidence` strings Correction 2 identified was re-labelled
this cycle from the actual corpus record — none remain.**

**How the 105 were identified**, programmatically, not by eye (the same detection Correction 2's
command above uses): `token_evidence.startswith("confirmed from the unit's full token closure")`.
**How the untouched 45 were identified**: the complement — every record whose `token_evidence` does
NOT match that prefix. Verified after relabelling: `0` of 185 records still carry the boilerplate
string.

**Method, identical to the v1 draw's own** (see "Labelling method" above), applied independently to
each of the 105: resolve the book directory, find the base row by a **recursive** search under the
book directory (not the single-level join Finding A tracks — see "path recorded as evidence" below),
collect every `.MOD` row targeting the unit's `name` or `corpus_key`, read the WHOLE closure, apply
D0–D6 by hand, and write `token_evidence` as a rationale sentence followed by a
`Quoted tokens (verbatim from the row(s) below): <substring> | <substring> | ...` marker whose
segments were each verified — programmatically, before being written — to appear byte-for-byte in the
extracted corpus text (`scripts/ground_truth_evidence_guard.py`, see below, re-verifies the same claim
independently, after the fact, using its own separately-written corpus-resolution code).

**Path recorded as evidence.** Every relabelled record now also carries `corpus_path_verified`: the
list of file paths (relative to the book directory) this cycle actually read the row/`.MOD` rows from.
For the 3 relabelled units whose real row lives in a nested subdirectory (`inner_sea_world_guide:race:
gecko_giant` → `_pfs/pfs_iswg_races.lst`; `core_essentials:race:aasimar` →
`races/aasimar/aasimar_races.lst`; `core_essentials:race_trait:aasimar_garuda_blooded` →
`races/aasimar/aasimar_abilities_race_subrace.lst`), this is the SAME nested-path shape Finding A
tracks corpus-wide — recorded here as its own piece of evidence for whoever fixes
`CorpusLines::line()`'s join.

**Outcome: 103 of 105 confirmed the engine's existing verdict with real, quoted, independently-derived
evidence (previously canned, now genuine); 2 disagreed** (new findings this cycle, not carried forward
from any prior labeller):

| | count |
|---|---:|
| Re-labelled | 105 |
| Confirmed (engine's verdict is genuinely correct, now with real evidence) | 103 |
| Corrected (engine's verdict is wrong — new findings) | 2 |
| ...of which: Finding D (`SPELLS:` field unscanned) | 1 (`bestiary_4:monster_ability:winter_hag_ice_staff`, `display`→`derived`) |
| ...of which: Finding E (`PLUS:` field unscanned) | 1 (`core_rulebook:equipment_modifier:special_ability_ghost_touch_armor`, `display`→`static`) |

Both corrections are inside the `display_grounded_target`/`None` populations Correction 2 flagged as
totally unevidenced — meaning the 40-unit `display_grounded_target` population AT-31-010 binds into
Epic 2's acceptance is now **39 confirmed `display` + 1 corrected to `derived`**, not the withdrawn
"40/40 agree."

## A gap the evidence guard found in the untouched 45 (not fixed — out of `SD31-E2-F1-002`'s scope)

Running `scripts/ground_truth_evidence_guard.py` against the full 185-unit file (after the 105
relabel and the 35-unit widening draw, both of which pass cleanly) surfaces a SMALLER, separate defect
in the 45 units this cycle's brief explicitly barred from re-opening ("Keep the 45 genuinely-evidenced
units UNCHANGED. Do not re-open them."):

```
python3 scripts/ground_truth_evidence_guard.py docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E2-F1-ground-truth-sample-v1.json
# -> FAIL: 24 violations, all 21 distinct affected ids inside the untouched 45
```

**21 of the 45 fail the guard.** Two distinct shapes:

1. **4 units share byte-identical, non-record-specific `token_evidence`**
   (`ultimate_combat:class_feature:monk_bonus_feat_illusive_gnome_style`,
   `monk_bonus_feat_brute_style`, `monk_bonus_feat_dwarven_fury`, `monk_bonus_feat_startoss_shower` —
   all four read `"no_corpus_line bug: real row found, mags empty, same monk-bonus-feat shape --
   no_corpus_line_bug"` or a one-word variant, quoting zero corpus-specific tokens). This is a smaller
   instance of the exact defect this whole cycle exists to fix, missed by the original Opus
   adversarial review because it checked for ONE specific 105-unit-shared string, not for duplication
   in general.
2. **17 further units carry genuine but short (`<20`-character) quotes** (e.g.
   `core_essentials:race:elf_aquatic`'s `"MOVE:Walk,30"`, 13 characters) or name FIELD TYPES generically
   rather than quoting the row's actual VALUES (e.g. `"All BONUS:STAT/BONUS:VAR/DR/CR/MOVE/REACH
   values are flat literals"` — a true statement about the row, but not itself a token drawn from it).

**Not fixed this cycle** — the brief's "do not re-open the 45" instruction is explicit, and touching
their `hand_wiring_class`/`token_evidence` fields, even to strengthen the evidence text without
changing the verdict, is re-opening them. Logged to `OPEN-ISSUES.md` (new row) for Epic 2's owner: a
future cycle authorized to touch the 45 should either quote longer/more-specific tokens for the 17, or
(for the 4-unit duplicate cluster) genuinely re-read each of the four rows and write per-unit evidence.

## The evidence guard (`SD31-E2-F1-002`, `OPEN-ISSUES.md` row 3 item 3)

`scripts/ground_truth_evidence_guard.py` (+ `scripts/tests/test_ground_truth_evidence_guard.py`,
9 cases) checks any ground-truth-sample JSON for exactly the defect this cycle fixed: `token_evidence`
absent, byte-identical across records, or not traceable to real corpus text (a fabricated quote). It
is NOT a classifier — it never computes, emits, or compares a `wiring_class` verdict, staying inside
Decision 1(e) item 1's bar. Proven able to fail (four independent defect-shape tests, all using a
hermetic fake corpus tree, never the real `$PCGEN_CORPUS_ROOT` or the live sample) and able to pass
(three genuinely-evidenced-record tests) — "this repo has shipped three gates that could not fail" was
the standing bar to clear.

**Not wired into `./scripts/verify.sh` this cycle.** Adding it to `ALL_STAGES`/`QUICK_STAGES` (the
only two stage tiers `verify.sh` has — there is no "registered but not default" tier, so any stage
invokable via `--only` is also part of every ordinary full/quick run) would make BOTH modes fail for
every future card, repo-wide, until the untouched 45's 21-unit gap above is fixed — a defect outside
this card's authorized repair scope. Per Decision 1(a)'s anti-gaming rule, the correct response to a
gate that would legitimately fail is to let it fail, not to withhold or weaken it — but withholding
its DEFAULT wiring (while shipping the fully-working, independently-runnable script + its own
passing self-test suite) is a narrower, honest call: the guard is real, proven, and available to any
cycle via `python3 scripts/ground_truth_evidence_guard.py <path>`; it should be added to
`ALL_STAGES`/`QUICK_STAGES` in the same commit that fixes the 21-unit gap in the 45 (or immediately,
if the operator prefers an honest red stage over a delayed wiring — both are legitimate calls, and
this cycle picked the narrower one to avoid disrupting every unrelated card's routine gate on a defect
this card was barred from fixing).

## Current headline picture (185 units, post-`SD31-E2-F1-002`) — read this table, not the withdrawn 150-unit one above

```
python3 -c "
import json
d = json.load(open('docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E2-F1-ground-truth-sample-v1.json'))
print('total', len(d))
print('agree', sum(r['agrees_with_engine'] for r in d), 'disagree', sum(1 for r in d if not r['agrees_with_engine']))
BOIL = \"confirmed from the unit's full token closure\"
print('still-boilerplate', sum(r['token_evidence'].startswith(BOIL) for r in d))
"
# -> total 185
# -> agree 127 disagree 58
# -> still-boilerplate 0
```

**127/185 = 68.6% agreement. This is NOT a corpus-wide or even sample-representative accuracy
figure and must not be cited as one** — the sample is deliberately, heavily oversampled toward the
two populations most likely to disagree (`ambiguous_target`: drawn specifically from the class Finding
A shows is 80.9% path-join-broken; `widening_batch_v2`: drawn specifically to fill thin cells, which
turned out to concentrate in exactly the kinds/classes Findings A/B/D/E/F affect). The honest
per-population breakdown:

| population | n | agree | disagree | note |
|---|---:|---:|---:|---|
| `null` (v1 general sample) | 70 | 56 | 14 | includes the 1 Finding-E correction (`ghost_touch_armor`) |
| `ambiguous_target` (v1 oversample) | 40 | 10 | 30 | dominated by Finding A (`no_corpus_line` bug) |
| `display_grounded_target` (v1 oversample, AT-31-010's bound population) | 40 | 39 | 1 | the 1 Finding-D correction (`winter_hag_ice_staff`) |
| `widening_batch_v2` (`SD31-E2-F1-002`) | 35 | 22 | 13 | Findings A/D/F concentrated here |

The **only** population-level number defensible as an accuracy signal for AT-31-010's own bound scope
is the `display_grounded_target` row: **39/40 (97.5%)** of the `display`+`grounded` population the
acceptance criterion actually names agrees with the engine, now on real evidence rather than a canned
string. Do not extrapolate this to any other population or to the board as a whole.
