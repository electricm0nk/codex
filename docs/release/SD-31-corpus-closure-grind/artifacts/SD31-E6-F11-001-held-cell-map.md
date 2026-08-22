---
cycle-id: SD31-E6-F11-001
actor: sd31-e6-heldcells
date: 2026-08-15
oracle-pin: 7f818006e371188e5717fd18d74d18a420747fc6
---

# Held-cell map — every `(wiring_class, status, kind)` cell that evaluates `held`, corpus-wide

This is the map Epic 6-F11 asked for: every cell the dashboard's own `doneness_verdict()`
classifies `held`, the exact instrument (rung, binary, file) that could move it, and — for the
majority of the mass — **why the instrument that exists today cannot yet reach it**, re-derived
against source, one record deep, not inferred from a doc comment.

**Headline correction up front.** The card's own brief assumed "growing fixture coverage" is
generally available work. It is not, right now. Every avenue this cycle checked — new BONUS:STAT
equipment fixtures, a new BONUS:WEAPON equipmod token family, static-sweep coverage on any of the
2,481 static-held units — is provably exhausted or structurally blocked in the *currently ingested*
corpus. See "Deliverable 2" below for the full negative-result trail. This is not a shortfall of
effort; it is a real, re-derived ceiling. Widening it needs either a new book ingest or new engine
capability, not more searching.

## Command

```
python3 -c "
import json, sys, collections
sys.path.insert(0, 'scripts/observer'); import pf1e_dashboard_producer as P
d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
cells = collections.Counter()
for u in U:
    v = P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))
    if v == 'held':
        cells[(u.get('wiring_class'), u.get('status'), u.get('kind'))] += 1
print('TOTAL_HELD', sum(cells.values()), 'of', len(U))
for k, n in sorted(cells.items(), key=lambda kv: -kv[1]): print(n, k)
"
```
→ **TOTAL_HELD 6,916 of 38,521** (matches the card's own "~6,916" framing exactly). Snapshot:
`docs/work-inventory.json` generated_at `2026-08-15T01:34:18Z`; oracle pin
`7f818006e371188e5717fd18d74d18a420747fc6`.

## Every held cell, sorted by size

| n | wiring_class | status | kind |
|---:|---|---|---|
| 2175 | static | ingested-magnitude | equipment |
| 1229 | derived | grounded | monster |
| 981 | display | grounded | monster_ability |
| 472 | derived | grounded | spell |
| 454 | derived | ingested-magnitude | spell |
| 303 | derived | grounded | companion |
| 223 | ambiguous | grounded | race_trait |
| 219 | derived | grounded | monster_ability |
| 182 | display | grounded | companion |
| 85 | static | grounded | monster_ability |
| 73 | derived | ingested-magnitude | equipment |
| 73 | static | grounded | spell |
| 69 | ambiguous | text-complete | feat |
| 54 | display | grounded | class_feature |
| 46 | static | ingested-magnitude | spell |
| 36 | static | text-complete | equipment |
| 32 | ambiguous | grounded | spell |
| 23 | ambiguous | text-complete | equipment |
| 23 | display | grounded | race_trait |
| 20 | derived | grounded | class_feature |
| 20 | ambiguous | ingested-magnitude | equipment |
| 19 | static | grounded | companion |
| 15 | derived | text-complete | spell |
| 15 | static | grounded | feat |
| 14 | static | ingested-magnitude | equipment_modifier |
| 13 | static | grounded | class_feature |
| 10 | ambiguous | grounded | monster_ability |
| 8 | ambiguous | ingested-magnitude | spell |
| 7 | ambiguous | grounded | race |
| 5 | derived | ingested-magnitude | equipment_modifier |
| 3 | static | grounded | monster |
| 3 | display | grounded | feat |
| 3 | ambiguous | grounded | monster |
| 2 | ambiguous | text-complete | spell |
| 2 | ambiguous | grounded | companion |
| 1 | ambiguous | grounded | class_feature |
| 1 | static | text-complete | feat |
| 1 | static | text-complete | spell |
| 1 | derived | grounded | feat |
| 1 | derived | grounded | race_trait |

(`computed` never appears — its table has no `held` branch; `done` or `in-progress` only.)

## The two rungs and what actually moves each cell

Traced in `src/bin/v06_work_inventory.rs::apply_done_rung_stamps` (~3763-3800): the stamping `match`
gates strictly on `item.wiring_class`. **Only `Static` and `Derived` are ever stamped** —
`Display`, `Computed`, `Ambiguous` are left untouched "on purpose," proven by the adjacent test
`ambiguous_display_computed_items_in_both_verified_sets_stay_unstamped`.

### `static` (2,481 held units) → `corpus_literal_sweep` → `literal-verified`

`src/bin/corpus_literal_sweep.rs` walks `data/corpus/**/*.json` generically (any kind), and its
`src/rules_core/corpus_literal_sweep.rs::parse_transcription` (line 324) requires **both**:
`source.kind == "lst_token"` **and** a non-empty `data.raw_tokens` array. Only then does a record
enter `records_examined`, and only a record with `!record.tokens.is_empty()` gets written into the
`--json-out` `verified` list `v06_work_inventory` joins against.

**Re-derived, corpus-wide, for every one of the 2,481 static-held units** (build once, join by
`(book, source_file, source_line)`):

```
cargo run --locked --bin corpus_literal_sweep -- --json-out /tmp/sweep-$RETRO_ACTOR.json
# → 3516 records examined of 9328 read, 0 findings, CLEAN
```
→ **overlap between the 2,481 static-held units and the sweep's 3,515-entry `verified` set: 0.**
Every single static-held unit is unreachable today, for one of three separate, re-derived reasons:

| n | reason | example |
|---:|---|---|
| 2,367 | **no corpus JSON record at that `(book, kind)` location at all** — a genuine ingest gap, not a sweep-coverage gap (e.g. `class_feature` has a `data/corpus/<book>/class_feature/` directory in exactly **one** book, `pathfinder_unchained`; the other 12 static-held `class_feature` units' books carry no such directory) | `core_rulebook:class_feature:barbarian_uncanny_dodge` |
| 95 | **has an `lst_token`-sourced JSON record, but `data.raw_tokens` is absent** — the record cites the right `.lst` line but the ingestion that produced it never captured a token breakdown, so `parse_transcription` returns `None` and the record never enters the sweep's population | `advanced_class_guide:spell:blade_lash` (`acg_spells.lst:27`, `source.kind=lst_token`, no `raw_tokens`) |
| 19 | **`source.kind` is `lst_corrected_ingest`/`lst_inherited_copy`**, deliberately excluded by `parse_transcription`'s own gate — these records were intentionally hand-corrected away from the raw `.lst` literal, so a byte-match against that literal would be the wrong bar by design | 14 `lst_corrected_ingest` + 5 `lst_inherited_copy` |

The single largest cell, `static|ingested-magnitude|equipment` (2,175), is dominated by a *fourth*,
even more structural case not yet in the table above: many of its records are shipped from a
`source.kind == "web_second_source"` citation (a website page, e.g.
`legacy.aonprd.com/advancedPlayersGuide/advancedGear.html`), which carries no `.lst` `path`/`line`
at all — `corpus_literal_sweep`'s whole model (compare shipped bytes to a PCGen `.lst` literal) is
inapplicable to these by construction, not by omission. Confirmed on the corpus:
```
python3 -c "import json; d=json.load(open('data/corpus/advanced_players_guide/equipment/abacus.json')); print(d['source'])"
# -> {'kind': 'web_second_source', 'url': 'https://legacy.aonprd.com/...', ...}  -- no path/line at all
```

**Instrument verdict for `static`:** the byte-sweep itself is real, already wired into
`verify.sh`'s `corpus-sweep` stage, and genuinely runs clean — but it has **zero currently-reachable
target** anywhere in the ingested corpus. Nothing in this cycle's scope (fixture-file growth) can
move it; the fix is either richer ingestion (`raw_tokens` backfill for the 95), real per-book ingest
for the 2,367 no-directory units (Epic 5/Epic 6-F1..F10's job), or an operator ruling on whether
`web_second_source`/`lst_corrected_ingest`/`lst_inherited_copy` need a *different* done-bar than
byte-match-the-literal (logged to `OPEN-ISSUES.md` below).

### `derived` (2,792 held units) → `derived_evaluator_fixture_check` → `fixture-verified`

**Corrected 2026-08-15 (`SD31-W2-INTEGRATE-001`, Finding 6):** this section originally published
`2,777`, a subtotal that dropped the 15-unit `derived|text-complete|spell` cell (listed in this
page's own per-kind table below and included in its `941 spell` row, then omitted from the prose
subtotal). Re-derived: `python3 -c "import sys,json,collections; sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P; d=json.load(open('docs/work-inventory.json')); U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]; c=collections.Counter(u.get('wiring_class') for u in U if P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))=='held'); print(dict(c))"`
→ `{'display': 1243, 'derived': 2792, 'ambiguous': 400, 'static': 2481}`, sum 6916 (matches this
page's own `TOTAL_HELD`). The per-kind table below already summed to 2,792 correctly — only this
prose subtotal and the one two paragraphs down were wrong. This package had already established
2,792 once before (`progress.md:188`/`:240`, retro correction `1786803206746-sd31-ready-s2-919f72`);
this fix restores it.

`src/rules_core/derived_evaluator_fixture_check.rs::run_bar_check` is **hard-locked to
`kind == "equipment"`**: `ingested_equipment_dir` only ever opens `data/corpus/<book>/equipment/`,
`load_equipment_corpus` only parses that directory's `EquipmentCacheData`-shaped records, and the
comparison (`bar_check`, lines 118-159) reads exactly one field off the result —
`item.ability_bonus`, i.e. a `BONUS:STAT|...` chain — via `compute_equipment_effects` /
`magic_items::compute_magic_items_effect`.

| n | kind | reachable by this instrument? |
|---:|---|---|
| 1229 | monster | **no** — the checker has no evaluator seam for `monster` at all |
| 941 | spell | **no** — same |
| 303 | companion | **no** — same |
| 219 | monster_ability | **no** — same |
| 73 | equipment | conditionally — only the `BONUS:STAT` shape; see below |
| 20 | class_feature | **no** |
| 5 | equipment_modifier | conditionally — only if it carries `BONUS:WEAPON|.../TYPE=Enhancement` (a *different*, already-built field, `weapon_enhancement_bonus`, that the checker's comparison does not read at all today) |
| 1 | feat | **no** |
| 1 | race_trait | **no** |

**2,719 of the 2,792 held `derived` units (97.4 %) sit under a `kind` the instrument cannot
evaluate no matter what the fixture file contains** — adding a `monster`/`spell`/`companion`/
`monster_ability` entry to `derived-evaluator-fixtures.json` today would not move a single unit; the
checker would either not find the record (no `equipment/` dir for that kind) or dereference a field
(`ability_bonus`) that a non-equipment record structurally cannot populate. This is Epic 6-F1's own
finding for `monster` generalized and re-proven for the other three kinds by the same code read.

The remaining 73+5 = 78 units are `kind=equipment`/`equipment_modifier` and *could* be reached in
principle. Deliverable 2 traces exactly how far that principle goes in the real corpus — see below.

### `display` (1,243 held) and `ambiguous` (400 held) — no rung exists yet, by design

**Corrected 2026-08-15 (`SD31-W2-INTEGRATE-001`, Finding 7):** originally published as `309`. Same tally command as the correction above, `c['ambiguous']` → **400**. No filter (text-complete exclusion, stale board) reproduces 309; the error also broke this page's own three-population reconciliation (see the fix below).

Both are explicitly excluded from `apply_done_rung_stamps`. Their bar is not "run a check" but
"resolve the classifier" — `display|grounded` means the wiring-class determinator found no
magnitude token even though a real consumer computed a delta (a misclassification the current
determinator cannot see past); `ambiguous|*` means the determinator could not assign a class at
all. Both need Epic 2's verdict-path capability build (not yet landed — see `kanban.md`
`epic-2-verdict-paths`, still `READY`), confirmed directly in the dashboard producer's own code
comment (`pf1e_dashboard_producer.py` ~3630-3660: "the instrument that would actually resolve this
is a wiring-class classifier that checks the full token closure GE-01 defines, which does not exist
yet"). This matches the epic-breakdown's own framing for `monster_ability`'s 981-unit
`display|grounded` cell exactly — **not** an Epic 6-F11 target.

## Cells with no instrument at all — logged to OPEN-ISSUES.md

Three distinct populations have no path to `done` today under any binary that exists in this repo,
and none of them are fixed by "grow the fixture JSON":

1. **2,719 `derived`-held units outside `kind=equipment`** (monster/spell/companion/monster_ability/
   class_feature/feat/race_trait) — the checker's evaluator seam does not exist for these kinds.
2. **2,481 `static`-held units, 100 % of them** — see the three-way breakdown above (no corpus dir /
   `raw_tokens`-absent / provenance-excluded).
3. **1,643 `display`+`ambiguous`-held units** (1,243 + 400, corrected from the originally-published 1,552 — `SD31-W2-INTEGRATE-001`, Finding 7) — capability-blocked on Epic 2, already tracked as such
   in `kanban.md`; not a new finding, cited here for completeness of the 6,916 total.

Logged as `RULING-NEEDED` rows in `OPEN-ISSUES.md` (below) with the proving commands.

---

## Deliverable 2 — the exhaustive search for a genuine fixture batch, and its result

The only `derived`-held population the existing `derived_evaluator_fixture_check` can reach at all
is `kind=equipment` with a `BONUS:STAT` (or, if the checker were extended, `BONUS:WEAPON`) chain,
present as an ingested `lst_token` corpus record. This section proves, exhaustively, that this pool
is already fully consumed.

**Step 1 — every `BONUS:STAT`-shaped equipment record in the whole ingested corpus** (not just the
held ones — every one, so "already fixture-verified" and "never fixture-covered" can both be seen):

```
python3 -c "
import json, glob
rows=[]
for p in glob.glob('data/corpus/*/equipment/**/*.json', recursive=True):
    rec=json.load(open(p)); chains=rec['data'].get('raw_bonus_chains',[])
    if any(c.get('qualifiers') and c['qualifiers'][0]=='STAT' for c in chains): rows.append(p)
print(len(rows))
"
```
→ **51** total. Cross-referenced against `tests/fixtures/rules_core/derived-evaluator-fixtures.json`
(`existing_keys = {(book, record_key)}`) and against `docs/work-inventory.json`'s per-unit status:

- **49 already `fixture-verified`** (`done`) — the existing 94-entry fixture file's own coverage.
- **1 is `Special Ability ~ Bonus Ability / Enhancement`**, a `%CHOICE`-templated equipmod (not a
  concrete unit — `wiring_class=computed`, not `derived`; irrelevant to this checker).
- **1 is `Belt of Dwarvenkind`** (`!PRERACE:1,Dwarf%`-conditional `STAT` bonus) —
  `wiring_class=computed`, `status=grounded` → already `done` via the `computed` bar, not `held`,
  and not reachable by this checker either way.

**Zero of the 51 are both `held` and `derived`.** The `BONUS:STAT` equipment pool is **100 %
consumed** — every unit it could ever cover already has been.

**Step 2 — the 73-unit `derived|ingested-magnitude|equipment` held cell itself**, checked
individually:

```
python3 -c "
import json, sys
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
rows=[u for u in U if u['wiring_class']=='derived' and u['kind']=='equipment' and u['status']=='ingested-magnitude']
print(len(rows))
from collections import Counter
print(Counter(r['book'] for r in rows))
"
```
→ **73**, of which **60 are `ultimate_equipment`** — a book with **no `data/corpus/ultimate_equipment/`
directory at all** (confirmed: `ls data/corpus | grep ultimate_equipment` → nothing). These are real
ingest work (Epic 6-F2/F5), categorically outside a fixture-file-only card.

The other 13 (`advanced_class_guide` 1, `advanced_players_guide` 1, `core_rulebook` 8,
`ultimate_intrigue`/`ultimate_psionics`/`ultimate_wilderness` 1 each) were checked individually
against their real corpus rows:
- `core_rulebook`'s 8 (`composite_longbow_base`, `longbow_base`, `sling_base`,
  `halfling_sling_staff_base`, `flurry_of_blows`, `mantle_of_faith`, ...) carry `raw_bonus_chains`
  qualifiers `WEAPON|DAMAGE|min(STR,0)` (bow STR-damage scaling), `WEAPON|WEAPONBAB|...` (Flurry of
  Blows' iterative-attack formula), or `DR:5/Evil` (`mantle_of_faith`) — **none is a `BONUS:STAT`
  chain**, and `compute_equipment_effects` has no field for weapon-damage-formula or
  damage-reduction magnitudes at all (`item.ability_bonus` would always be `None` for these — not a
  bug, an honest absence). A fixture entry here would fail by construction, every time, forever.
- `advanced_class_guide:equipment:helm_of_the_valkyrie` has `raw_bonus_chains: []` — a pure-prose
  1/day summon effect, misclassified `derived` by a prose-pattern false positive (same shape as
  `OPEN-ISSUES.md` row 2's Finding B), genuinely has no magnitude to fixture.
- `advanced_players_guide:equipment:spindle_of_perfect_knowledge` **is already in the fixture file**
  and is the check's one real `FAIL`:
  ```
  cargo run --locked --bin derived_evaluator_fixture_check
  # FAIL advanced_players_guide:equipment:spindle_of_perfect_knowledge: corpus row states
  #   BONUS:STAT|INT,WIS,CHA|4|TYPE=Enhancement but the evaluator produced no ability bonus at all
  ```
  Traced to source: the PCGen `.lst` line (`apg_equip_magic_items.lst:13`) really does carry
  `BONUS:STAT|INT,WIS,CHA|4|TYPE=Enhancement` — but the **shipped** `data/corpus` JSON for this
  record has `source.kind: "web_second_source"` and **no `raw_bonus_chains`/`raw_tokens` at all**
  (`data/corpus/advanced_players_guide/equipment/spindle_of_perfect_knowledge.json`). The fixture's
  own `independence` contract (its JSON header) derives `expected` from the upstream `.lst` bytes
  *by design*, deliberately never from `data/corpus/` — so this FAIL is the check correctly catching
  a **real ingestion gap** (this one item's mechanical data was never captured), not a fixture bug or
  an evaluator bug. No code change fixes it; only re-ingesting this one record from its `.lst` line
  would.
- The `ultimate_intrigue`/`ultimate_psionics`/`ultimate_wilderness` singles: none of those three
  books has a `data/corpus/<book>/equipment/` directory either (`equipment` subdir list: only
  `advanced_class_guide`, `advanced_players_guide`, `advanced_race_guide`, `beastiary`,
  `core_rulebook`, `pathfinder_unchained` have one) — `not_ingested`, same as `ultimate_equipment`.

**Step 3 — the second field the checker could compare but doesn't**
(`equipmods::compute_equipmods_effect` → `weapon_enhancement_bonus`, a real, already-built,
already-tested field for `BONUS:WEAPON|TOHIT|DAMAGE|DAMAGE,TOHIT|n|TYPE=Enhancement`): searched the
whole ingested corpus —

```
python3 -c "
import json, glob
rows=[]
for p in glob.glob('data/corpus/*/equipment/equipmods/**/*.json', recursive=True):
    rec=json.load(open(p)); chains=rec['data'].get('raw_bonus_chains',[])
    for c in chains:
        q=c.get('qualifiers',[])
        if len(q)>=4 and q[0]=='WEAPON' and q[1] in ('TOHIT','DAMAGE','DAMAGE,TOHIT') and q[3]=='TYPE=Enhancement':
            rows.append(p)
print(len(rows))
"
```
→ **12** (the canonical +1..+5 Weapon/Ammunition enhancement and Masterwork records). All 12 are
`wiring_class=computed` (not `derived`) and `status=grounded` — **already `done`** via the
`computed` bar. Extending the checker to this field today would move **zero** units off `held`.

### Result: **0 new fixtures landed this cycle.**

Every reachable avenue — the checker's one supported token family (`BONUS:STAT`), its one supported
kind (`equipment`), and the one other already-built comparable field (`weapon_enhancement_bonus`) —
was checked against the real, ingested corpus and found to have **zero currently-held, currently-
reachable units**. Adding a fixture entry for any of the 73 candidates would either (a) show
`not_ingested` (60 units, no book dir), (b) fail by construction because the record has no magnitude
field the checker reads (11 units), or (c) duplicate the one already-committed, already-correctly-
failing entry (`spindle_of_perfect_knowledge`). None of those is "genuine fixture coverage moving a
unit to `done`" — fabricating any of them to make a count move is exactly the anti-gaming violation
`decisions.md` Decision 1(a) and this card's own brief warn against. Per the brief's own instruction
("if you find yourself tempted, that is a STOP: log it to `OPEN-ISSUES.md` and press on elsewhere"),
this cycle stopped and logged instead of padding the fixture file.

**Guarded-regen delta, measured as instructed** (before/after, both runs use the freshly-generated
`corpus_literal_sweep`/`derived_evaluator_fixture_check` reports):

```
cp docs/work-inventory.json /tmp/work-inventory-BEFORE-sd31-e6-heldcells.json
cargo run --locked --bin corpus_literal_sweep -- --json-out /tmp/sweep-sd31-e6-heldcells.json
cargo run --locked --bin derived_evaluator_fixture_check -- --json-out /tmp/fixture-sd31-e6-heldcells.json
CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep-sd31-e6-heldcells.json \
DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture-sd31-e6-heldcells.json \
  cargo run --locked --bin v06_work_inventory
# doneness_verdict() tally before vs after: identical in every bucket
#   BEFORE {'done': 5837, 'not-started': 20895, 'unmeasurable': 3989, 'deferred': 36, 'held': 6916, 'in-progress': 848}
#   AFTER  {'done': 5837, 'not-started': 20895, 'unmeasurable': 3989, 'deferred': 36, 'held': 6916, 'in-progress': 848}
```
→ **board delta: 0 across every bucket.** Confirms the committed `docs/work-inventory.json` was
already generated with both reports piped in (already at the ceiling this cycle re-derived), and
confirms no fixture file edit this cycle changed anything (none was made — the file is unmodified,
`git diff --stat tests/fixtures/rules_core/derived-evaluator-fixtures.json` is empty). Restored per
the wave rule: `git checkout -- docs/work-inventory.json` (never committed).

---

## Deliverable 3 — the scale plan

**Per-unit cost, measured from this cycle's own search, not estimated.** This cycle spent the
better part of a cycle-budget doing full-corpus, per-candidate verification (glob every equipment
JSON in 6 books, cross-reference 51 `BONUS:STAT` chains against 94 existing fixtures, hand-check 13
individual `derived|equipment` held records against their raw `.lst` bonus chains, trace one FAIL to
its root ingestion cause) and the answer was **zero addressable units**. That negative result is
itself the cost measurement: **the "fixture growth" lever for `kind=equipment` is not a per-unit
cost problem, it is a zero-supply problem** — there is no backlog of coverable units sitting
unaddressed; the backlog is entirely in kinds and provenance-shapes the current instrument cannot
reach at all.

**Named cells, named counts, named per-unit method — what actually unblocks each:**

| cell | n | what it needs | method, if built |
|---|---:|---|---|
| `derived\|grounded\|monster` | 1,229 | a **new** evaluator seam for `kind=monster` (the checker's `load_equipment_corpus`/`compute_equipment_effects` cannot be pointed at monster records — different corpus shape, different rules-table). Real engine work: a `compute_monster_effects`-shaped module plus a monster-kind branch in the fixture check, mirroring how `equipment_effects`'s four categories were built one at a time (Epic 5's own history). | hand-derive each fixture from the monster's PCGen `.lst` stat block (e.g. a `SR` or ability-damage token) against the *rules*, same discipline as the equipment fixtures' `derivation` field — **not** from running the not-yet-built evaluator |
| `derived\|grounded/ingested-magnitude\|spell` | 926 | same shape: no spell-evaluator seam exists in the checker at all | per-spell hand-derivation from the spell's saving-throw/duration/damage-dice text against the PCGen `.lst` token, one spell school's shape at a time (mirrors Epic 5's per-category rollout) |
| `derived\|grounded\|companion` | 303 | same — no companion seam | as above, per companion-ability-type |
| `derived\|grounded\|monster_ability` | 219 | same — no monster_ability seam | as above |
| `static` (all kinds), 2,481 | 2,481 | **not fixture work at all** — either (a) `raw_tokens` backfill for 95 units whose `lst_token`-sourced JSON lacks the token breakdown (an ingestion-completeness fix, re-run per book), or (b) real per-book/per-kind ingest for the 2,367 units with no corpus directory (Epic 5/Epic 6-F1..F10), or (c) an operator ruling on whether `web_second_source`/`lst_corrected_ingest`/`lst_inherited_copy` provenance needs its *own* done-bar (the single largest cell, 2,175-unit `equipment\|ingested-magnitude`, is mostly this case) | n/a — this is an ingestion/ruling track, not a fixture-growth track; conflating the two is the gaming risk this cycle avoided |
| `derived\|ingested-magnitude\|equipment` (the 73 checked here) | 73 | 60 need `ultimate_equipment` ingested (Epic 6-F2/F5); the other 13 need either a weapon-damage-formula field or a DR field added to `equipment_effects` (real, scoped engine work — one new field, following the exact `arms_armor`/`general`/`magic_items`/`equipmods` pattern this module already uses four times) | for the DR/weapon-formula field: hand-derive from the item's own `DR:n/type` or `min(STR,0)`-style `.lst` token against the PF1 rule text, same as the existing 94 |
| `display\|grounded` (all kinds) | 1,243 | Epic 2's verdict-path classifier — capability-blocked, not this card's lever | n/a |
| `ambiguous\|*` (all kinds) | 309 | Epic 2, same | n/a |

**What can be safely batched vs. what must stay hand-derived**, once a new evaluator seam exists:
records sharing an *identical* token shape (e.g. every `BONUS:STAT|<abilities>|<n>|TYPE=Enhancement`
equipment row, as the existing 94 already show) can be derived by a small script reading the `.lst`
oracle directly and emitting the fixture's `expected` block mechanically — this is what
`derivation`/`independence`/`generated_by: scripts/derive_derived_evaluator_fixtures.py` already
document as the accepted batching shape for equipment. Records with a genuinely different formula
per row (monster special-ability damage dice, spell scaling text, DR types) cannot be batched
honestly; each needs its own by-hand rule-book derivation, exactly as this cycle's `magic_items`
fixtures already do one ability/value pair at a time.

**Recommended next-wave dispatch, in priority order:**
1. **Operator ruling** on the `static` provenance question (`OPEN-ISSUES.md` row below) — this
   single ruling potentially unblocks up to 2,481 units with *no new engine code*, just an ingestion
   enrichment pass, making it the highest-leverage next step by a wide margin.
2. **`ultimate_equipment` book ingest** (Epic 6-F2/F5) — unblocks 60 of the 73 equipment-derived held
   units immediately once ingested, many already `BONUS:STAT`-shaped per this cycle's book-level scan
   of the PCGen oracle (not yet corpus-JSON-verified since the book isn't ingested).
3. **A new `monster` evaluator seam** — highest single-cell payoff (1,229) but the largest build; a
   dedicated card, not a `-001` cycle extension.

