# Cycle 8 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism)

- **Commit SHA:** `<filled by this cycle's immediate follow-up commit, matching every prior
  cycle's own two-commit pattern on this receipt>`
- **Files touched:** this receipt, `docs/release/SD-34-book-completion/progress.md`,
  `docs/release/SD-34-book-completion/kanban.md`. No `src/` file touched this cycle — see
  "Why no code changed" below. `docs/work-inventory.json` is unchanged (byte-identical; no
  regeneration needed since no engine table changed).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (no code diff to scan)
- **Wired-integration audit result:** `OK_NO_TOKENS` (no code diff to scan)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "**970** Core Rulebook units whose
  table exists but which are not in it. **Evidence:** the atlas reporting bucket B at zero for
  `core_rulebook`, and the mechanism that placed them named — by mechanism, not per record."
  This receipt covers only the `class_feature_option_pool_record_not_held_by_engine` mechanism,
  one of nine; the criterion itself does NOT close this cycle (8 other mechanisms remain owned
  by other cycles).

## Re-derived population, this cycle's start

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold'
     and x['evidence']=='class_feature_option_pool_record_not_held_by_engine']
print(len(u))
"
34
```
Matches cycle 7's own closing figure exactly — of **543** `core_rulebook` bucket-B units (whole
book, all 9 mechanisms, `python3 scripts/completion_atlas.py --check --book core_rulebook`), of
**49,438** corpus-wide units total (`content-unit-inventory.md`'s own re-derive command).

## Cycle 7's own next-cycle plan, read before touching anything

Cycle 7's remainder table named 6 sub-causes summing to 34, unchanged from cycles 5/6 for 5 of
them: (1) weapon-flavored generic indirection, 8; (2) `Weapon and Armor Proficiency ~
{Druid,Monk}`, excluded, 2; (3) armor/shield-flavored generic indirection + extras
(`Add Spoken Language`, `Channel {Negative,Positive} Energy`, `Evasion`), 10; (4) companion/mount
registration, 3 (newly split out); (5) wizard opposition-school spell tracking, 9; (6) Domain
Power `CLASS_FEATURE_POOLS` registration gap, 2. The task brief for this cycle named the same
two groups the receipt already called "genuinely new engine subsystems" — proficiency/grant
possession-tracking and wizard opposition-school tracking — and instructed: build one properly,
or return `partial` stating plainly that no narrow work remains and naming exactly what must be
built.

## Investigation this cycle — verifying, not re-quoting, that no narrow lever remains

Per `AGENTS.md` §9 ("never state a derived figure as settled before the work that derives it has
returned") and the criterion's own instruction not to move a unit into X/U on my own authority, I
independently re-verified each of the 4 distinct sub-causes (the excluded pair folds into
sub-cause 1's group) against the live corpus and the live engine, rather than trusting the prior
cycles' characterization at face value.

### Companion/mount registration (3 units) — investigated first, looked most promising

The 3 keys are `Companion ~ Animal Companion` (`cr_abilities_class.lst:2728`), `Companion ~
Special Mount` (`:2729`), `Special Mount ~ Standard Choices` (`:1390`). Read all three corpus
records directly:

```
$ cat data/corpus/core_rulebook/class_feature/companion/companion_animal_companion.json
$ cat data/corpus/core_rulebook/class_feature/companion/companion_special_mount.json
$ cat data/corpus/core_rulebook/class_feature/special_mount/standard_choices.json
```

All three are `CATEGORY:Internal` `AUTOMATIC` indirection targets (the same PCGen shape as the
already-excluded `Weapon Prof ~ Auto/Martial/Simple` group), referenced by grep across
*multiple, distinct* real class features: `data/corpus/core_rulebook/class_feature/
animal_companion/animal_companion_base.json` (Druid), `.../hunter_s_bond/animal_companion.json`
(Ranger), `.../divine_bond/special_mount.json` (Paladin), plus `nature_s_bond` and `domain_power`
variants also grant `Companion ~ Animal Companion`.

Checked whether the underlying mechanism is at least universally engine-computed (which would
distinguish it from the excluded weapon-prof case, where the granted content genuinely differs
per class): `Companion ~ Animal Companion`'s progression IS real for the Druid path —
`ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL` (`src/rules_core/pilot_compute/mod.rs:7608`) plus the
`class_chassis.druid.animal_companion.*` explanation ids (`pilot_view_model.rs:520-559`) are
wired and tested. But:

```
$ grep -n "class_chassis.paladin.*special_mount\|special_mount" src/rules_core/pilot_compute/mod.rs src/rules_core/pilot_view_model.rs
```
returns **zero** Paladin special-mount computation anywhere — only comment-only references to
the corpus's own `SpecialMountLVL` token (`mod.rs:7045,7608`), never a formula that evaluates it.
`Special Mount ~ Standard Choices`'s own `COMPANIONLIST` choice set has no `choice:special_mount`
registration anywhere in `composed_input.rs`. So 2 of the 3 keys have no engine backing at all,
and even the 1 key that does (`Companion ~ Animal Companion`) is shared by 3 other real class
features (Ranger's Hunter's Bond, Cleric's Domain Power, the Nature's Bond archetype) with **no**
wired progression of their own — crediting the shared record as "engine holds it" would
misrepresent those 3 owners exactly the way the already-excluded weapon-prof group was ruled out
for the identical reason. **Confirmed, not assumed: a genuinely unbuilt companion/mount grant
subsystem, not narrow work.**

### Wizard opposition-school spell tracking (9 units)

```
$ grep -n "opposition_school\|OppositionSchool" src/bin/v06_work_inventory.rs
```
`probe_wizard_arcane_school_wiring`'s own doc comment (`v06_work_inventory.rs:7427-7431`) states
directly: "every OTHER school ... and both schools' own top-level `"<School> School"` /
`"<School> Opposition School"` recognition records have no such formula built yet." The 9 units
here (`Abjuration Wizard Spells` … `Universal Wizard Spells`, `magnitude_token_count: 0`,
`description: null`) are a further-different record from even that recognition pair — a
per-school spell-list-access record. Confirmed no consumer exists:

```
$ grep -rn "Wizard Spells" src/rules_core/ --include='*.rs' | grep -v test
```
returns nothing. **Confirmed: a real, unbuilt spell-list-per-opposition-school subsystem** (which
school's spells a character can/cannot learn given their 2 opposition-school choices) — this
requires a per-school spell list keyed against the character's own opposition-school choice,
which no existing table models.

### Proficiency/mechanical-grant possession-tracking (20 units: 8 + 2 excluded + 10)

Re-confirmed unchanged from cycles 5/6's own finding on each remaining key: `Weapon Prof ~
Auto/Martial/Simple`, `Armor Prof ~ Heavy/Light/Medium`, `Shield Prof`, `Shield Prof ~ Tower`,
`Armor Training ~ Heavy Armor`, `Weapon Proficiencies ~ {Cleric,Monk}`, `Weapon and Armor
Proficiency ~ {Druid,Monk}` are all generic multi-class indirection targets with no single
owning class-table row — no new lever found this cycle. `Add Spoken Language`, `Channel
{Negative,Positive} Energy`, `Evasion` each need a standalone, genuinely new possession-tracked
engine fact (a spoken-language ledger, a channel-energy-pool fact, an evasion flag) that no
existing table models — none of the 3 shares a shape with any table already shipped.

### Domain Power `CLASS_FEATURE_POOLS` registration gap (2 units: Leadership, Sun's Blessing)

Confirmed these reach into the `class_feature_option_pool_record_with_magnitude_not_held_by_
engine` sibling mechanism's own domain-power population — per this cycle's own instruction ("do
not fix theirs"), left untouched.

## Conclusion — `status: partial`, 0 units moved

No narrow, safely-verifiable lever survived this cycle's investigation. All 34 remaining units
fall into one of 3 genuinely new, unbuilt engine subsystems, plus 2 units that belong to a
sibling mechanism's own population. Building any one of the 3 correctly (not just
plausibly-looking) is real, multi-record engineering — not a same-shape extension of an
already-shipped table the way cycles 5-7's own closures were. Per `decisions.md §9` ("a
measurement wave that banks zero units is a legitimate deliverable") and the criterion's own
instruction that X/U dispositions require an operator ruling I cannot make on my own authority,
0 units moved this cycle rather than risk an unsafe or dishonest closure.

## Row-count command output (this cycle's own artifact)

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold'
     and x['evidence']=='class_feature_option_pool_record_not_held_by_engine']
print(len(u))
"
34
```
Before this cycle: 34. After this cycle: 34. Delta: 0.

## Build scope verified

`cargo test --locked --no-run` exits 0 (workspace), run at SHA `2c56ac5a71` (HEAD before this
cycle's own docs-only commit — no code changed this cycle, so no re-run was needed after; the
figure this receipt depends on, `docs/work-inventory.json`, is unchanged from that SHA). Desktop
crate (`apps/desktop/src-tauri`) not re-tested this cycle: no file under that tree, or under
`src/`, was touched.

## Sweep population

N/A — no corpus records added or regenerated this cycle. Baseline unchanged: 48,699 of 51,473.

## Figures + their re-derive commands

| Figure | Command | Denominator |
|---|---|---|
| This mechanism, before | `python3 -c "...evidence=='class_feature_option_pool_record_not_held_by_engine'..."` above | of 543 `core_rulebook` bucket-B units |
| This mechanism, after | same command | of 543 `core_rulebook` bucket-B units |
| `core_rulebook` bucket B, whole book | `python3 scripts/completion_atlas.py --check --book core_rulebook` | of 6,701 `core_rulebook` units |
| Denominator gate | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` | 15 files checked, 0 violations |

## Movement, four buckets

- **Closure:** 0
- **Reclassification:** 0
- **Reachability:** 0
- **Instrument-correction:** 0

## Status

- **Status:** partial. My whole assigned population (34 units) was investigated this cycle; none
  closed narrowly and none was moved into X/U without an operator ruling. The remainder below
  names every unit by sub-cause with a population that sums exactly to 34, so the dispatch can
  pick up any one subsystem next.

## Remainder — 34 units, named by sub-cause and what must be built for each

| Sub-cause | Units | What must be built |
|---|---:|---|
| Proficiency/mechanical-grant possession-tracking, weapon-flavored generic indirection (`Weapon Prof ~ Auto/Martial/Simple`, `Weapon Proficiencies ~ {Cleric,Monk}`) | 8 | A generic multi-class `GrantedFact` possession ledger the engine can point >1 class's indirection target at, distinguishing which class's grant applies per character. |
| `Weapon and Armor Proficiency ~ {Druid,Monk}` | 2 | Same possession ledger; excluded from cycle 6's per-class table because Druid's own weapon list mismatches (`Scythe`) and Monk repeats the established 16/17 mismatch — needs the mismatch resolved first, not just the ledger. |
| Proficiency/mechanical-grant possession-tracking, armor/shield-flavored generic indirection + standalone extras (`Armor Prof ~ {Heavy,Light,Medium}`, `Armor Training ~ Heavy Armor`, `Shield Prof`, `Shield Prof ~ Tower`, `Add Spoken Language`, `Channel {Negative,Positive} Energy`, `Evasion`) | 10 | Same possession ledger for the armor/shield indirection targets; `Add Spoken Language`/`Channel {Negative,Positive} Energy`/`Evasion` each need their own standalone new possession-tracked fact (a spoken-language ledger, a channel-energy-pool fact, an evasion flag) — no existing table shape covers any of the three. |
| Companion/mount registration (`Companion ~ Animal Companion`, `Companion ~ Special Mount`, `Special Mount ~ Standard Choices`) | 3 | A shared-indirection-target catalog keyed by the internal `FOLLOWERS:`/`COMPANIONLIST:` ability name (not by owning class), PLUS — before it can honestly close all 3 — a real Paladin Special Mount computation (`class_chassis.paladin.special_mount.*`, currently absent) and a `choice:special_mount` choice-set registration, neither of which exists today. Only the Druid-owned key has any wired progression today, and it is shared by 3 other unwired owners. |
| Wizard opposition-school spell tracking (`Abjuration`…`Universal Wizard Spells`) | 9 | A per-school spell-list-access table (`WIZARD_SCHOOL_SPELL_ACCESS` or equivalent) recording which spells each school excludes given a character's own 2 opposition-school choices, wired to a spell-list consumer that does not exist yet. |
| Domain Power `CLASS_FEATURE_POOLS` registration gap (`Leadership`, `Sun's Blessing`) | 2 | Owned by the `class_feature_option_pool_record_with_magnitude_not_held_by_engine` sibling mechanism (a different AT-34-E3-001 cycle) — not this cycle's to fix. |

**8 + 2 + 10 + 3 + 9 + 2 = 34.** Every remaining unit is named by sub-cause with a population and
what must be built; none is folded into "the rest".

`decisions.md §16` ("only the count grounds") was checked against this remainder: none of these
34 units carry the "pick N from an eligible set" choice shape — they are attribute/possession-
tracking grants, generic PCGen indirection targets, a per-school spell-list-access gap, and a
sibling-owned domain-power gap. §16 does not apply to any of them.

## Next-cycle plan

Pick exactly ONE of the 3 genuinely new subsystems above (proficiency possession-tracking [20
units, the largest single lever], wizard opposition-school spell-list access [9 units], or
companion/mount registration [3 units, but gated on building Paladin's own Special Mount
computation first]) and build it as real, tested engine code — not a lookup that merely defers
the shape check. The Domain Power 2 units stay with the `with_magnitude` sibling.
