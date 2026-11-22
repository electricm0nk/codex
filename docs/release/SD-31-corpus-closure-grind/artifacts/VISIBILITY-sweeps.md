---
canonical: true
owner: sd31-wave28-sweeps-lane
purpose: Wave 28 "look, don't bank" deliverable — the sweeps pile. Runs S9, S2, S6, S7 (partial),
  S5 (partial), S8/D5 in priority order. Banks nothing; every number ships with the command that
  produced it.
started: 2026-08-21
base_commit: e90ba9ec1
board_at_start: "13,456 / 38,372 (35.07%) — re-derived below, matches the dispatch exactly"
---

# VISIBILITY — sweeps lane, wave 28

## Board re-derivation (so a verifier has the baseline this report reasons from)

```
python3 -c "
import json, sys
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
def v(u): return P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
from collections import Counter
print(len(U), Counter(v(u) for u in U))
"
```
→ `38372 Counter({'not-started': 18188, 'done': 13456, 'unmeasurable': 4270, 'in-progress': 1231,
'held': 1185, 'deferred': 42})`. Matches the dispatch's stated figures (in-progress/held off by
±10 from the dispatch prompt's numbers, a few hours' drift from other lanes' concurrent work in
their own worktrees — not this lane's doing; `docs/work-inventory.json` was read-only here).

**Nothing in this report changed `docs/work-inventory.json`.** It was read, never written. No
`git add`/`git commit` touches it. `git status` at the end of this session (below) confirms.

---

## Headline: the single biggest finding this lane made is that S9's own premise is false

The dispatch states, and `sweeps.md`'s own wave-27 addendum states: *"Ninja and Samurai have
complete, tested, correctly-dispatched class chassis and are blocked ONLY by one missing row in a
weapon-proficiency table."* **That is not what the code says, and has not been what the code says
since wave 20** — a comment already committed to `weapon_tables.rs` at the exact line the sweep
would have to touch documents, in detail, that neither class is a one-row fix. Filed as a
`retro.py correction` (`1787341221999-sd31-w28-sweeps-9f0d54`). See S9 below.

---

## S9 — "one table row away": closed at 0, with the real shape of the 2 units named

**Command that establishes the universe (34 dispatched classes — the 5 enums that back
`table_class_id` and its APG/ACG/UC/PU siblings):**

```
sed -n '/enum ClassId/,/^}/p'   src/rules_core/rules_tables/crb/class_tables.rs           # 11
sed -n '/enum UcClassId/,/^}/p' src/rules_core/rules_tables/ultimate_combat/mod.rs        # 3
sed -n '/enum PuClassId/,/^}/p' src/rules_core/rules_tables/pathfinder_unchained/class_chassis.rs  # 4
sed -n '/enum AcgClassId/,/^}/p' src/rules_core/rules_tables/acg/mod.rs                   # 10
sed -n '/enum ApgClassId/,/^}/p' src/rules_core/rules_tables/apg/mod.rs                   # 6
```
11 + 3 + 4 + 10 + 6 = **34**, matching S1's closed count exactly.

**Command that lists `CLASS_WEAPON_PROFICIENCIES`'s coverage:**
```
sed -n '458,562p' src/rules_core/rules_tables/crb/weapon_tables.rs | grep -oP 'class_id: "class:\K[a-z_]+'
```
→ 32 class_ids. **34 − 32 = 2 missing: Ninja and Samurai.** So far this matches the dispatch's
premise exactly.

**But the table's own next 20 lines (already committed, wave 20, `weapon_tables.rs:536–556`)
explain WHY those two are absent, and neither reason is "a row was never added":**

- **Ninja** — the class's own DESC prose says "proficient with all simple weapons," but the
  *ingested corpus token* on that record carries no matching `AUTO:WEAPONPROF|TYPE=Simple` (or
  equivalent indirection). The table's stated discipline is to transcribe the **token**, not the
  prose. Adding a Simple-tier row here would be inventing a fact the ingest does not carry — an
  ingestion/DESC-vs-token contradiction to resolve, not a row to append.
- **Samurai** — the class's proficiency record is `AUTO:WEAPONPROF|TYPE=Samurai`, a weapon **TYPE**
  selector. This table models tiers (Simple/Martial), named weapons, and weapon *groups* — a weapon
  **type** is none of those, and the table has zero representation for it, unlike the tier/named/
  group shapes a "row" can hold. This is the *same structural boundary* already named twice in this
  file for Unchained Monk's `TYPE=Monk` and Gunslinger's `TYPE=Firearm` — except for those two the
  gap costs nothing (every weapon the corpus actually needs is already covered by the named list),
  while for Samurai it costs **everything** (zero named/tier coverage at all).

**Verification that no other one-row gap exists elsewhere in the 34-class universe:** cross-checked
every one of the 34 dispatched classes against `docs/work-inventory.json`'s `class`-kind units
(script below). **30 of 32 matched units are `grounded`/`computed` (done). Only Ninja and Samurai
are not-done, and both are `status=not-ingested`** — i.e. the corpus-ingestion layer, not the
weapon-proficiency table, is the recorded blocker for the `class` unit itself; the weapon-prof gap
is a *downstream* consumer-side gap that would still block them even if ingestion were fixed.

```
python3 -c "
import json
d=json.load(open('docs/work-inventory.json'))
names34 = {'core_rulebook':['Barbarian','Bard','Cleric','Druid','Fighter','Monk','Paladin','Ranger','Rogue','Sorcerer','Wizard'],
'ultimate_combat':['Gunslinger','Ninja','Samurai'],
'advanced_class_guide':['Arcanist','Bloodrager','Brawler','Hunter','Investigator','Shaman','Skald','Slayer','Swashbuckler','Warpriest'],
'advanced_players_guide':['Alchemist','Cavalier','Inquisitor','Oracle','Summoner','Witch']}
for u in d['units']:
    if u.get('kind')=='class' and u.get('book') in names34 and u.get('name') in names34[u['book']]:
        print(u['book'], u['name'], u['status'], u['wiring_class'])
"
```
(Pathfinder Unchained's 4 classes carry no `class`-kind unit at all — they are represented as
alternate-feature archetypes on their parent class, not standalone `class` records. Confirmed by
`docs/work-inventory.json` having no `book == "pathfinder_unchained"` among `kind == "class"`.)

### S9 verdict

**CLOSED. Corpus-wide count: 0 of 34 dispatched classes are genuinely one missing table row from
working.** The 2 units the pattern was named after (Ninja, Samurai) are each a *different*,
non-trivial gap — one an ingestion/DESC-token contradiction, one a weapon-TYPE-selector modelling
capability this table does not have at all. **General fix, named specifically:**

1. **Ninja** — re-derive against the pinned oracle whether the "all simple weapons" DESC prose has
   a corresponding token ANYWHERE else on the class's records (a second `AUTO:WEAPONPROF` line, a
   `PCC`-level default, or an errata). If none exists, this is a genuine PCGen data omission and the
   correct disposition is "not ingestible as stated," not "add a row."
2. **Samurai** (and the two nil-cost precedents, Unchained Monk/Gunslinger) — model a weapon **TYPE**
   dimension in `ClassWeaponProficiency` (a `weapon_types: &[&str]` field plus a `WEAPON_TABLE` column
   recording each entry's PCGen `TYPE:` tags) so `TYPE=<X>` selectors resolve the same way named
   weapons and weapon groups already do. **Tool evaluation:** this is a real modelling extension, not
   a generator — hand-work, one PR. It closes exactly 1 unit today (Samurai) plus removes the
   "deliberately not modelled" caveat already carried for 2 other classes. Not worth building as a
   standalone tool at this yield; worth doing opportunistically alongside whatever next touches this
   table.

**The reusable lesson, stated plainly for future waves:** a "cheapest units in the program" claim
that has sat unquestioned since wave 27 was one `sed`/`grep` pair away from being falsified by a
comment ALREADY IN THE TREE. Read the file the claim is about before repeating it.

---

## S2 — generalising the Monk case beyond classes

**The Monk shape, precisely:** a hand-authored data table exists (e.g. `class_tables()`'s
CRB row) AND a separate `IdEnum` variant exists for the same thing AND the string→enum dispatch
function has no arm connecting them — so real, complete data sits behind a resolver that returns
`None` for a key that does have a row.

**This shape requires TWO separate artifacts (an `IdEnum` + dispatch fn, and a keyed data table)
that can drift apart.** So the first question is: which "kinds" even HAVE that two-artifact
architecture at all?

```
grep -rn "^pub enum .*Id\b" src/rules_core/rules_tables/ --include=*.rs
```
Full result (18 hits): 5 class-id enums (`ClassId`/`ApgClassId`/`AcgClassId`/`UcClassId`/`PuClassId`
— S1, already swept, 0), `RaceId` (crb/race_tables.rs), `MonsterId` (beastiary1/mod.rs), `RuleSetId`
(book-level, not content-level — not in scope), and 8 copies of `Pf1SchoolId` (one per spell-list
module — a small fixed taxonomy of 8 schools of magic reused per-book, not a per-record dispatch,
and every school is used identically everywhere it is declared — not the Monk shape).

**That is the entire universe: class (swept, S1), race, and monster. No `CompanionId`, `EquipmentId`,
`FeatId`, `SpellId`, or `RaceTraitId` enum exists anywhere in the codebase** — `grep` for each
confirms zero hits. Companion, equipment, equipment_modifier, feat, spell, and race_trait kinds are
driven by direct corpus-key/HashMap lookups or the wiring-class classifier, not by an enum-in-front-
of-a-table architecture. **The Monk gap cannot occur in a kind that does not have this
architecture, by construction** — there is no second artifact for the string mapping to drift away
from.

### Race — checked, 0 gap

```
sed -n '/enum RaceId/,/^}/p' src/rules_core/rules_tables/crb/race_tables.rs
```
→ 7 variants (Human, Dwarf, Elf, Gnome, HalfElf, HalfOrc, Halfling) — exactly the 7 CRB core races,
and `RACE_TRAITS` (the data table) is hand-authored 1:1 against those 7, nothing more.

```
grep -n '"race:.*" => RaceId::' src/rules_core/pilot_compute/mod.rs
```
→ all 7 strings dispatch to all 7 variants. **0 orphaned rows, 0 missing arms — the enum and the
table were built together and stayed together.**

### Monster — checked, gap exists but is a DIFFERENT shape (already known, not new)

`MonsterId` (Bestiary 1 only) covers 280 of Bestiary 1's 326 ingested `monster` records (46
missing — re-derivable via `grep -c 'key: "' src/rules_core/rules_tables/bestiary/monster_data.rs`
against the corpus's 326). **This is NOT the Monk shape.** Monk's bug was "the table has the row,
the dispatch doesn't reach it." Here, the 46 records were **never added to `MonsterId`/
`monster_data.rs` at all** — there is no orphaned table row to reach, because no row exists. This
was already found and written up during the `monster_sla` seam's build (see `progress.md`, "a
pre-existing ingest gap this seam surfaced") — re-discovering it here is itself the kind of
re-discovery `todo/README.md` asks to be named, not silently re-filed as new.

### S2 verdict

**CLOSED, corpus-wide.** The two-artifact architecture that created the Monk gap exists in exactly
3 places: class (S1: 0), race (this sweep: 0), monster (this sweep: a real 46-record gap, but of a
different shape — table incompleteness, not dispatch omission — already on record). **No kind
outside these three can have a Monk-shaped gap, because no kind outside these three has the
architecture that makes one possible.** Operator's instinct to check was correct to ask; the answer
this cycle finds is that the check terminates at 3 kinds, not "all objects."

---

## S6 — generator idempotency: 8 of 10 Python fixture generators are safe, 1 is NOT, 1 unassessed;
## Rust `cache_gen` generators NOT reached (time-boxed, see "could not determine")

**This is the highest-value finding in this report and it is LIVE, not historical.**

### The 10 `scripts/derive_*_fixtures.py` generators, assessed by reading each one's selection logic

| # | Script | Selection shape | Idempotent? |
|---|---|---|---|
| 1 | `derive_monster_sla_spell_level_fixtures.py` | `TARGET_STATUSES = {"grounded","fixture-verified"}` | **Yes** (wave-15 fix present) |
| 2 | `derive_companion_strength_damage_fixtures.py` | same pattern | **Yes** (wave-15 fix present) |
| 3 | `derive_monster_ability_save_dc_fixtures.py` | same pattern | **Yes** (wave-15 fix present) |
| 4 | `derive_companion_save_dc_fixtures.py` | same pattern | **Yes** |
| 5 | `derive_companion_skill_bonus_fixtures.py` | same pattern | **Yes** |
| 6 | `derive_spell_range_fixtures.py` | `STAMPABLE_STATUSES = ("ingested-magnitude","grounded","fixture-verified")` | **Yes** |
| 7 | `derive_spell_caster_level_duration_fixtures.py` | same pattern | **Yes** |
| 8 | `derive_class_feature_description_fixtures.py` | filters by `kind=="class_feature"` only — **no status filter at all** | **Yes, structurally** — nothing to erase itself with |
| 9 | `derive_class_feature_level_scaling_fixtures.py` | `TARGETS` is a **hand-picked fixed list of 9 unit ids**, not a live status query (its own docstring: *"`docs/work-inventory.json` is read for IDENTITY ONLY"*) | **Yes, structurally** |
| 10 | `derive_derived_evaluator_fixtures.py` | `HELD_STATUSES = ("ingested-magnitude","grounded","text-complete")` — **omits `"fixture-verified"`** | **NO — confirmed live, reproduced below** |

### The live bug, reproduced (read-only — `--report` mode writes nothing)

```
python3 scripts/derive_derived_evaluator_fixtures.py --report
```
Output (verbatim):
```
held `derived` units in the inventory: 758
fixture entries derived from the corpus: 0
...
```
**Zero `kind=equipment`/`equipment_modifier` units appear anywhere in the 758 "held" pool or in the
"uncovered, by reason" breakdown — not even as a rejected candidate.** That is total exclusion, not
partial loss.

Cross-check against the currently COMMITTED fixture this same script produced on some earlier run:
```
python3 -c "
import json
d=json.load(open('tests/fixtures/rules_core/derived-evaluator-fixtures.json'))
print(len(d.get('entries',[])))
"
```
→ **94.** So the committed fixture holds 94 `entries` (the equipment `BONUS:STAT` family this exact
script owns) and a fresh, no-op-except-print `--report` run against the SAME committed
`docs/work-inventory.json` finds **zero** candidates to rebuild them from.

**Root cause, confirmed by direct read of the generator's own write step
(`scripts/derive_derived_evaluator_fixtures.py:206-345`):** it reads `docs/work-inventory.json`
directly (the COMMITTED file, already carrying prior stamps — `INVENTORY = ".../docs/work-
inventory.json"`), selects `held_derived = [u for u in inventory["units"] if
u.get("wiring_class")=="derived" and u["status"] in HELD_STATUSES]`, and **rebuilds `entries` from
scratch every run** (`entries = []` at top, appended to, never merged with the prior file's
`entries`). `apply_done_rung_stamps()` (`src/bin/v06_work_inventory.rs:7502-7504`) is the function
that promotes a covered unit's `status` to `"fixture-verified"` once this exact fixture verifies it
— the same status this generator's `HELD_STATUSES` does not recognize. **This is the identical
wave-15 shape** (`OPEN-ISSUES.md` row 286: *"the generator selects `status == grounded`, but
`apply_done_rung_stamps()` rewrites a covered unit to `fixture-verified`, so run 2 selects the EMPTY
SET"*) reproduced in a generator (`SD31-E6-F11-002`) **built AFTER** that lesson was written down and
fixed in 5 sibling scripts — the lesson did not propagate to this file's authorship.

**11 units are at risk right now** (`derived`+`fixture-verified`, `kind=equipment`, re-derivable via
`python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(sum(1 for u in
d['units'] if u.get('wiring_class')=='derived' and u.get('status')=='fixture-verified' and
u.get('kind')=='equipment'))"` → 11) — plus, structurally, EVERY future equipment unit this seam
ever verifies, because the erasure is total (0, not "94 minus a few"), so this is not a slow leak,
it is a full-family reset waiting for the next non-`--report` invocation of this specific script.

**Why this is more urgent than any other finding in this report:** `OPERATOR-RULINGS-2026-08-21.md
§20` just overturned SD-27's "no formula interpreter" ban and authorised building the interpreter
this exact fixture-and-check pair (`derive_derived_evaluator_fixtures.py` /
`derived_evaluator_fixture_check.rs`) is the gating mechanism for. The population this bug can erase
is about to grow by orders of magnitude as that build-out proceeds, on the SAME generator, with the
SAME bug, unless it is fixed before the next wave writes new `entries` through it.

**General fix, one line, same shape as the 5 already-fixed siblings:**
```python
HELD_STATUSES = ("ingested-magnitude", "grounded", "text-complete", "fixture-verified")
```
**Tool evaluation:** not a tool — a one-line change plus the "no idempotency test" gap `OPEN-ISSUES`
row 286 already named and marked "sized as a small card, not done here" back at wave 15. That test
(run every `derive_*_fixtures.py` generator twice against a stamped inventory, assert byte-identical
`entries`/family-specific-key output) closes this ENTIRE sweep class going forward — write it once,
it protects all 10 current scripts and any new one the interpreter build-out adds. **This is exactly
the "large list of todo → build a tool" case the dispatch asked to watch for**: a 3-line pytest
parametrized over the 10 scripts, run in CI, is cheaper than one more wave rediscovering this by
hand.

### What was NOT reached: the 11 Rust `cache_gen` binaries + `enrich_*` binaries

`src/rules_core/cache_gen/{acg,apg,beastiary1,class_feature,class_feature_grants,equipment_gap,
hand_authored_equipment,lst_provenance_repair,spell_lane_dump,ultimate_equipment}.rs` and the 9
`src/bin/enrich_*_raw_tokens.rs`/`gen_*`/`ingest_*` binaries were **not** run twice and diffed.
D1 already documents 2 of these (`cache_gen::apg::generate_equipment`,
`gen_core_rulebook_cache`) as confirmed non-idempotent with a standing mitigation
(`repair_lst_provenance` re-run) but no upstream fix. The other ~30 binaries' idempotency is
**genuinely unassessed** — this would require building the crate (`cargo build --bin <name>` ×30,
each writing to `data/corpus/*.json`, each needing a pre/post `git diff` and revert) inside an
isolated `CARGO_TARGET_DIR`, which this lane did not have time-budget to do safely alongside the
Python sweep above. **Honest gap, not a silent skip:** see "Could not determine."

---

## S7 — bar checks that assert count without identity: PARTIAL

Searched the primary grounding/gating code (`pilot_compute/mod.rs`, ~69k lines — the file every
`ground_or_block_*`/`compute_*` function lives in) for `.len()`/`.count()` comparisons used as a
GATING decision rather than a plain unit-test assertion:

```
grep -n "\.len() ==\|\.count() ==\|len() >=\|len() !=" src/rules_core/pilot_compute/mod.rs
```
→ 7 hits, all structural (multiclass-level-count ≥2, opposed-school-count ==2, domain-selection-
count ==1/==2) — none is a "how many X, not which X" content gate of the wave-16 shape.

```
grep -rn "\.len() ==\|\.count() ==" src/rules_core/pilot_compute/*.rs | grep -v mod.rs
grep -n "\.len()\|\.count()" src/rules_core/rules_tables/companion_chassis.rs \
  src/rules_core/class_feature_pool_catalog.rs src/rules_core/rules_tables/monster_chassis.rs
```
→ 1 more hit (`class_feature_grant_consumer.rs:404`, `if levels.len() == 1` — a real structural
branch, not a content-identity elision), and zero count-only gates in the three files named by the
wave-16 finding and the Bestiary-6 ledger's `ENGINE_EFFECT_TOKEN_KEYS` discussion.

**This is a PARTIAL sweep, stated plainly.** `src/rules_core/rules_tables/` holds ~150 files this
search did not individually open; the population-wide `assert_eq!(_.len(), N)` grep across all of
`src/` returns **311 hits**, and this lane only triaged the subset most likely to be a live
doneness-decider (compute/grounding modules), not the full 311 — most of the remainder are almost
certainly internal-table-size unit tests (harmless), but "almost certainly" is not the bar this
program holds itself to. **Corpus-wide count of true content-blind doneness gates: not fully
derived. What is derived: the specific wave-16 companion-abilities shape has 0 live recurrences in
the 3 files it was found in or fixed near.**

**General fix if more are found:** the same shape wave 16 used — pair the count assertion with an
identity assertion (a sorted-name/key comparison), not a replacement of the count check.

---

## S5 — generators emitting unscreened fields: PARTIAL, but the news is good

`cache_gen::class_feature.rs` is the fixed instance (wave 19) and now explicitly redacts the `DESC`
token inside `raw_tokens` whenever `description` is redacted (`class_feature.rs:425-427`, mirroring
`enrich_equipment_raw_tokens.rs::screen_field_value`'s precedent per its own comment).

Checked whether the lesson generalised to every OTHER `raw_tokens`-producing binary:

```
grep -c "screen\|redact\|pi_screening\|declared_pi" src/bin/enrich_companion_raw_tokens.rs \
  src/bin/enrich_equipment_raw_tokens.rs src/bin/enrich_monster_ability_raw_tokens.rs \
  src/bin/enrich_monster_raw_tokens.rs src/bin/enrich_spell_raw_tokens.rs
```
→ 12, 36, 40, 15, 26 hits respectively — **all 5 reference PI-screening machinery**, not zero. Spot-
checked the smallest (`enrich_companion_raw_tokens.rs`, 12 hits) directly: `screen_field_value()`
is called generically **per token key/value pair in a loop** (`enrich_companion_raw_tokens.rs:263`),
not scoped to `DESC` alone — this is actually a STRONGER design than `class_feature.rs`'s original
DESC-only fix, and there is a mutation-proof test for it
(`enrich_one_redacts_a_blacklist_term_hit_anywhere_in_the_closure`).

`cache_gen::spell_lane_dump.rs` writes **no `raw_tokens` at all** — deferred by design to
`enrich_spell_raw_tokens.rs` as a separate pass (stated in its own module doc) — so it cannot leak
an unscreened field it never emits.

**S5 verdict: PARTIAL, encouraging.** 5 of 5 `raw_tokens`-producing binaries built AFTER the wave-19
fix reference and apply PI screening (1 field-generically, matching or exceeding the fix's own
design). **NOT verified:** field-by-field completeness for every emitted key in the ~10 `cache_gen`
modules that DON'T touch `raw_tokens` (equipment_gap, hand_authored_equipment, ultimate_equipment,
acg, apg, beastiary1) — those emit their own field sets and were not individually audited against
`pi_screening::classify_field`'s full key list. That audit is a bounded, mechanical task (grep each
module's `Serialize`d struct fields, confirm each string-typed field routes through
`classify_field`) that a future lane or a small script could close in under an hour.

---

## S8 / D5 — `ClassFeatureData.class` read from key text: MEASURED for the first time

`sweeps.md` carried this as *"~12,247 records, never measured."* Measured this cycle:

```
python3 <<'PY'
import json, glob, collections
files = glob.glob('data/corpus/**/class_feature/**/*.json', recursive=True)
real_classes = {...34 dispatched names + 5 CRB NPC classes + Antipaladin...}  # see script
real_lower = {c.lower() for c in real_classes}
class_counts = collections.Counter()
matches_real = not_real = 0
for f in files:
    c = json.load(open(f)).get('data', {}).get('class')
    if c is None: continue
    class_counts[c] += 1
    (matches_real := matches_real + 1) if c.strip().lower() in real_lower else (not_real := not_real + 1)
print(matches_real, not_real)
PY
```
(exact script archived at the scratchpad path in this session's tool log; re-runnable verbatim)

- **12,481 total `class_feature` corpus files** (vs `docs/work-inventory.json`'s 15,439
  `class_feature` units — the gap is `.MOD` rows, superseded duplicates, and non-corpus-file units;
  not reconciled here, a separate question).
- **11,502 of those carry `data.class` set** (979 are `None` — genuinely no key to split).
- **3,292 (28.6%) match one of the 34 dispatched real class names exactly.**
- **8,210 (71.4%) do NOT.** Broken down (heuristic prefix-match, stated as a heuristic, not exact):
  - **2,682** are `"<RealClassName> <Suffix>"` shaped (`Warpriest Bonus Feat` 432, `Ranger Combat
    Style Feat` 179, `Monk Bonus Feat` 133, `Rogue Talent` 130, `Witch Hex` 64, `Magus Arcana` 57,
    `Arcanist Exploit` 46, ...) — the class name is a literal prefix of the group name. **General
    fix:** same shape `class_feature_owner`'s existing resolver already uses — a
    `GROUP_PREFIX → real class` table. Cheap, mechanical, reproducible.
  - **532** are themselves the name of a REAL class that is simply outside the 34-dispatch universe
    (`Medium` 92, `Psychic` 85, `Magus` 70, `Vigilante` 67, `Shifter` 50, ...) — these need the class
    ITSELF modelled (per `levers.md` L0/L1's "18 untabled base classes" and B4/B5's Vigilante
    ruling-pending question) before the `class` field question is even answerable; fixing the field
    alone would be cosmetic.
  - **4,996** did not prefix-match my heuristic (`Domain Power` 172, `Rage Power` 170, `Discovery`
    129, `Wild Talent` 128, `Refined Education` 94, `Ki Power` 80, ...) — **this is my classifier's
    limitation stated honestly, not a claim that 4,996 need individual investigation.** Nearly all
    of these ARE resolvable group names too (Rage Power→Barbarian/Bloodrager, Discovery→Alchemist,
    Wild Talent→Kineticist, Ki Power→Monk) — they just aren't literally prefixed by the class name
    string, so a real fix needs a proper `GROUP → CLASS` lookup table (the kind
    `class_feature_owner` already half-builds), not a prefix heuristic. **I did not build that
    table** — doing so accurately for all ~1,069 distinct group names is real work, correctly out of
    scope for a look-don't-bank wave.

**S8/D5 verdict: MEASURED, not closed.** The corpus-wide scale is **8,210 records (71.4% of all
11,502 `class`-bearing `class_feature` records)** carrying a `data.class` value that is not a real
dispatched class name — bigger than the `~12,247` placeholder implied, because that number was the
POPULATION, not the DEFECT COUNT. **Tool evaluation: this is a tool-shaped fix.** A `GROUP_PREFIX →
CLASS` table (~40-60 entries would likely cover the top 90% by frequency, per the Zipf-shaped
frequency list above) fixes potentially thousands of records in one PR — this is exactly the "large
list of todo → build a tool" case. **What it does NOT close:** the 532 real-but-undispatched-class
records (Magus/Medium/Psychic/Vigilante/Shifter) need the class chassis work itself (L0/L1), not a
field remap.

---

## Tool evaluation summary (per the dispatch's required format)

| Group | Count | Hand-work or tool-work | What it closes corpus-wide |
|---|---:|---|---|
| S6 idempotency test (parametrized over all `derive_*_fixtures.py`) | protects 10 scripts, 1 live bug | **Tool** — one ~30-line pytest, run twice-and-diff per script | Prevents the wave-15 shape from recurring in ANY of the 10 present scripts or future ones the interpreter build-out adds. High leverage, low cost — build this first. |
| S6 `HELD_STATUSES` fix in `derive_derived_evaluator_fixtures.py` | 11 equipment units now, unbounded going forward | **Hand-work** — 1 line | Stops an active silent-erasure risk on the exact seam `OPERATOR-RULINGS §20` just authorised scaling up |
| S8/D5 `GROUP_PREFIX → CLASS` remap table | up to ~2,682 confirmed prefix-shaped + a large share of the 4,996 unclassified | **Tool** — one lookup table + a script pass over `data/corpus/*/class_feature/*.json` | Corrects `data.class` on thousands of records across every book with class features — this is the single largest-population fix named in this report |
| S9 weapon-TYPE-selector modelling | 1 unit today (Samurai), removes 2 documented caveats | **Hand-work** — extend one struct + table | Small, not worth a standalone tool at this yield |
| S7 full 311-hit triage | unknown remaining risk | **Tool-assisted hand-work** — a script to flag `.len()==`/`.count()==` sites inside functions named `ground_*`/`compute_*`/`classify_*` (filters the 311 down to the doneness-relevant subset automatically) | Would make S7 closeable in a future wave without re-reading 150 files by hand |
| S5 remaining field audit (6 `cache_gen` modules, non-`raw_tokens` fields) | unknown, bounded | **Tool** — a script asserting every `Serialize`d string field routes through `pi_screening::classify_field` | Closes S5 completely; currently PARTIAL only because of time, not difficulty |

**Biggest single lever in this report: the S6 idempotency test.** It is the cheapest of everything
named here, it protects a mechanism the operator just authorised scaling by orders of magnitude, and
it converts this entire sweep class from "found by hand, wave after wave" into "cannot ship again."

---

## What could not be determined

- **The ~30 Rust `cache_gen`/`enrich_*`/`gen_*`/`ingest_*` binaries' idempotency.** Only the 10
  Python `derive_*_fixtures.py` scripts were assessed by direct read + a `--report`-mode
  reproduction. The Rust binaries would need an actual build (`cargo build`, isolated
  `CARGO_TARGET_DIR`) and a run-twice-diff-revert cycle per binary; this lane's time budget went to
  the Python sweep (which surfaced a live, unambiguous, high-urgency bug) instead. D1's 2 known-bad
  Rust generators are the only ones with a documented verdict.
- **S7's full 311-site population.** Only the subset inside `pilot_compute/*.rs` +
  `companion_chassis.rs` + `class_feature_pool_catalog.rs` + `monster_chassis.rs` was triaged.
  ~150 other `rules_tables/` files were not individually opened.
- **S8/D5's 4,996 "neither" bucket.** Confirmed most are legitimately resolvable group names, but
  building the full `GROUP → CLASS` table needed to prove each one would take real, dedicated work —
  correctly out of scope for this wave.
- **Whether the 979 `class_feature` records with `data.class == None` are a separate defect
  (key text with no `~` to split) or intentional** (a class feature that genuinely has no owning
  class, e.g. a racial-only or template feature that happens to be classified `class_feature`) — not
  investigated this cycle.

---

## Git status at close (proof nothing was banked)

```
git status --porcelain
```
Only this file, `todo/sweeps.md` (updated in place per instructions), and `docs/retro/events/
sd31-w28-sweeps.jsonl` (the correction event) are touched. `docs/work-inventory.json`,
`data/corpus/**`, and every `src/`/`scripts/` file this report reasoned about are unmodified —
confirmed by `git status` showing no changes under those paths.
