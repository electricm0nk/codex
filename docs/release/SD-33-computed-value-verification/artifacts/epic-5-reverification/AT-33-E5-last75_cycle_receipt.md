# Cycle AT-33-E5-last75 — Epic 5 Re-verification / AT-33-E5-002 (75-unit residual)

- **Commit SHA:** recorded on landing (see `progress.md` entry `AT-33-E5-last75`)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-last75.oracle-results.json` (new — this lane's committed deliverable, 8 rows)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-last75_cycle_receipt.md` (this file)
  - `docs/release/SD-33-computed-value-verification/progress.md` / `kanban.md` (updated in place)
  - `docs/retro/events/sd33-r4-last75.jsonl` (new — 1 incident, 1 deferral)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.

## Population re-derivation (first action, per the brief)

```
$ python3 -c "import json,collections
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d})
print(len(miss)); [print(m) for m in miss]"
75
{'equipment': 61, 'equipment_modifier': 14}
```

Matches the brief's stated 75 (61 `equipment` + 14 `equipment_modifier`) exactly. This lane's
population is confirmed to be those literal 75 ids, all named in the brief's own shape lists.
Rebased onto `origin/tranche/13` before this derivation (this worktree started from the `develop`
merge commit, not the tranche branch — corrected via `git fetch origin tranche/13 && git rebase
origin/tranche/13` before any read).

## Shape table (all 75, by mechanism — every id accounted for)

Re-derive: `python3 census.py` (script inline below) over the 75 ids, reading each unit's whole
corpus record (`raw_bonus_chains`), not a filtered view.

| Shape | Population | Examined this cycle | Verdicts this cycle |
|---|---:|---:|---|
| SKILL-shape, single skill, non-psionics, no slot hazard | 1 | 1 | agree: 1 |
| SKILL-shape, single skill, `Magic.Wondrous.Implant` slot (no matching `EQSLOT`) | 1 | 1 | unverifiable: 1 |
| SKILL-shape, multi-skill comma-joined chain (no single PCGen token) | 3 | 3 | unverifiable: 3 |
| SKILL-shape, single skill, `ultimate_psionics` book | 14 | 0 | — blocked, see Finding 2 |
| COMBAT-shape, `ultimate_psionics` book | 1 | 0 | — blocked, see Finding 2 |
| `ultimate_psionics` dissonance `VAR`+`WEAPON`-formula pair | 2 | 0 | — blocked, see Finding 2 + case-mismatch below |
| COMBAT-shape, non-psionics (`INITIATIVE`/`TOHIT.Ranged`/formula-valued `AC`/`SAVE`) | 6 | 0 | no resolver / formula-valued |
| Wield-size, `WIELDCATEGORY` chain only (non-scalar) | 3 | 3 | unverifiable: 3 |
| Wield-size, `WIELDCATEGORY` + bare `WEAPON\|TOHIT` (no-penalty variant) | 3 | 0 | real magnitude, deliberately unmatched by design (see Finding 3) |
| `EQMARMOR` material family (`draco`/`dragonhide`/`material_dragonhide`) | 3 | 0 | resolver exists, needs new fixture pattern |
| `EQMWEAPON\|DAMAGESIZE` | 2 | 0 | no resolver |
| `EQMWEAPON\|RANGEADD` | 1 | 0 | no resolver |
| `EQM\|WEIGHTDIV` | 1 | 0 | no resolver |
| `WEAPON\|DAMAGEMULT` (fractional crit-multiplier) | 4 | 0 | no resolver, non-integer value |
| `WEAPONPROF=<x>` / `WEAPON` enhancement family (`compute_equipmods_effect`) | 24 | 0 | resolver exists — see Finding 4 |
| Bare `WEAPON\|TOHIT`/`DAMAGE`/`ATTACKS`, no `TYPE=` qualifier | 6 | 0 | 3 flurry-extra-attacks (`ATTACKS` formula) + 3 plain offset, no resolver |
| **Total** | **75** | **8** | **agree 1, unverifiable 7** |

## The 8 examined rows

| unit_id | ours | oracle | verdict | reason |
|---|---:|---:|---|---|
| `advanced_class_guide:equipment:hunter_s_sight` | -2 | -2 | agree | — |
| `book_of_the_damned_volume_2:equipment:demon_senses` | — | — | unverifiable | Implant slot, no `EQSLOT` mapping |
| `advanced_class_guide:equipment:ring_of_eloquence` | — | — | unverifiable | multi-skill comma-joined |
| `inner_sea_races:equipment:scarf_of_glorious_histories` | — | — | unverifiable | multi-skill comma-joined |
| `ultimate_psionics:equipment:meld_stone_nimble_trickster` | — | — | unverifiable | multi-skill comma-joined |
| `core_rulebook:equipment_modifier:special_quality_wield_size_1_step_greater` | — | — | unverifiable | `WIELDCATEGORY`, non-scalar |
| `core_rulebook:equipment_modifier:special_quality_wield_size_2_steps_greater` | — | — | unverifiable | `WIELDCATEGORY`, non-scalar |
| `core_rulebook:equipment_modifier:special_quality_wield_size_3_steps_greater` | — | — | unverifiable | `WIELDCATEGORY`, non-scalar |

Re-derive:
```
$ python3 -c "import json,collections
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-last75.oracle-results.json'))['results']
print('rows', len(d)); print(collections.Counter(x['verdict'] for x in d))
print('reasonless', len([x for x in d if x['verdict']=='unverifiable' and not (x.get('reason') or '').strip()]))
ids=[x['unit_id'] for x in d]; print('dupes', len(ids)-len(set(ids)))"
rows 8
Counter({'unverifiable': 7, 'agree': 1})
reasonless 0
dupes 0
```

## Finding 1 — real instrument correction: `gradlew` vs the direct-`java` runner

Every named `oracle_harness_ultimate_psionics_campaign_load_failure` row in prior waves used
`scripts/pcgen-run-character.sh` (drives `./gradlew run`). This cycle re-ran the exact same failing
`.pcg` files (unmodified, from `statsave-fixtures/skill-pcg/`) through
`scripts/oracle_harness/charbuild_remainder_run_one.sh` (the repo's own proven direct-`java` runner,
`AT-33-E5-002`'s own lever, against `PCGEN_REPO_DIR/build/install/pcgen`) — **the campaign loads with
no error**, for `ultimate_psionics` AND for two other books that hit the identical failure class
under `gradlew` (`advanced_class_guide`, `book_of_the_damned_volume_2`):

```
$ bash scripts/oracle_harness/charbuild_remainder_run_one.sh <hunter_s_sight.pcg> <ftl> <out> <settings>
...
SKILL.NAME=Perception
SKILL.TOTAL=-2
SKILL.MISC=-2
```
(`hunter_s_sight`, `advanced_class_guide` — real live agreement, `-2` matches the corpus
`BONUS:SKILL|Perception|-2` chain exactly.)

This is a real, re-derivable instrument correction (`scripts/retro.py incident`,
`sd33-r4-last75.jsonl`) — the previously-recorded root cause ("Could not find campaign") was real
under `gradlew` but is not a fundamental oracle-data gap.

## Finding 2 — a SECOND, deeper, still-open `ultimate_psionics` defect

Fixing Finding 1 does **not** unblock the 17 `ultimate_psionics` SKILL/COMBAT/dissonance units. Two
of two sampled `ultimate_psionics` SKILL-shape items — both with a bare, PRE-free
`BONUS:SKILL|<name>|<n>|TYPE=Competence` chain, confirmed directly against the pinned corpus LST
line — export `SKILL.<name>.MISC=0` under the direct-`java` runner, where a non-psionics item with
the identical shape (`Circlet of Persuasion`, same runner, same template) correctly exports its real
`+3`:

```
$ bash .../charbuild_remainder_run_one.sh <circlet_of_persuasion.pcg> ...   # non-psionics control
SKILL.MISC=3
$ bash .../charbuild_remainder_run_one.sh <crystal_mask_psionic_craft.pcg> ...
SKILL.MISC=0        # expected 10 (BONUS:SKILL|Spellcraft|10|TYPE=Competence, no PRE)
$ bash .../charbuild_remainder_run_one.sh <meld_stone_alchemist.pcg> ...
SKILL.MISC=0        # expected 8 (BONUS:SKILL|Craft (Alchemy)|8|TYPE=Competence, no PRE)
```

Ruled out this cycle, each empirically, none of which explains it:
- **Not a slot/`LOCATION` issue** — `crystal_mask_psionic_craft`'s `TYPE:Psionic.Universal.Eyegear`
  maps to a real `EQSLOT:Eyes` in the pinned Pathfinder gamemode's own `equipmentslots.lst`; adding
  an explicit `LOCATION:Eyes` to the `.pcg` made no difference (still `MISC=0`).
- **Not an `OUTPUTNAME` mismatch** — the `.pcg`'s `EQUIPNAME`/`EQUIPSET VALUE` use the record's real
  `KEY`-less positional name, confirmed byte-identical against the raw LST line.
- **Not the two-campaign load itself** — `Circlet of Persuasion` (single campaign) and every prior
  wave's non-`ultimate_psionics` two-campaign SKILL item worked; only `ultimate_psionics` items show
  this pattern (2 of 2 sampled).
- **Not a game-mode mismatch** — `GAMEMODE:Pathfinder_RPG` (used identically by every working
  fixture) has no dedicated `system/gameModes/` directory of that exact name; it resolves the same
  way for working and non-working fixtures alike.

**Not root-caused further this cycle** — named honestly rather than guessed at, per doctrine. This
blocks 17 of the 75 units (14 SKILL + 1 COMBAT + 2 dissonance) until a next cycle isolates it. Given
this is empirically confirmed on 2 of 2 sampled units (not 1), it is treated as blocking the full
`ultimate_psionics` SKILL/COMBAT population, not just the two sampled ids.

## Finding 3 — `compute_equipmods_effect` already resolves 24 of 75 units; zero new `src/rules_core/` code needed

Reading `src/rules_core/equipment_effects/equipmods.rs` directly (not assumed from its module doc
comment) shows `compute_equipmods_effect` already matches:
- a bare `WEAPON|<roll>|<n>|TYPE=Enhancement` chain,
- a `WEAPONPROF=TYPE.Natural|<roll>|<n>|TYPE=Enhancement` chain (Amulet of Mighty Fists family), and
- a bare `WEAPONPROF=<specific name>|<roll>|<n>` chain **with no `TYPE=Enhancement` requirement at
  all** (the Cursed-Weapon/Horseshoes-of-a-Zealous-Warhorse family) —

which together cover all 24 `weaponprof_family` units in the shape table (amulets, rods, claw
blades, mattock, talons, belt/berserking/backbiter/cursed-sword/horseshoes/pistol/hammer). This was
established by reading the resolver's real match logic against every one of the 24 records' real
`raw_bonus_chains`, not assumed.

**Live oracle round-trip started, real progress, not finished:** PCGen's own
`WeaponToken.getMagicHitToken`/`getMagicDamageToken` back `WEAPON.<n>.MAGICHIT`/`MAGICDAMAGE` export
tokens (confirmed against the real pinned PCGen source). **Real, execution-confirmed indexing hazard
found and fixed this cycle:** `WeaponToken.getToken` parses the index segment as `weaponList.get(weapon)`
— **zero-indexed**, not one-indexed. `WEAPON.1.MAGICHIT` on a character with exactly one weapon
equipped silently returns empty (no error); `WEAPON.0.MAGICHIT` is the equipped weapon:

```
$ bash .../charbuild_remainder_run_one.sh <mattock_of_the_titans.pcg> <ftl querying WEAPON.0.*> ...
WEAPON.NAME=*Mattock of the Titans
WEAPON.MAGICHIT=-3
WEAPON.MAGICDAMAGE=+3
```

Corpus chain: `BONUS:WEAPON|TOHIT,DAMAGE|3|TYPE=Enhancement` (our engine: `bonus=3` for both rolls).
`MAGICDAMAGE=+3` **agrees**. `MAGICHIT=-3` does **not** match the expected `+3` and the sign flip is
unexplained this cycle — not committed as a `disagree` (doctrine forbids closing a disagreement by
assumption) and not committed as `agree` either. A second attempt, `rod_flailing` (an Exotic/Double
weapon, `PROFICIENCY:WEAPON|Flail (Dire)`), returned a **blank** `WEAPON.0.*` — the fixture's
Level-1 Fighter likely lacks Exotic Weapon Proficiency for this specific weapon, a real, different,
also-unresolved blocker for the 4 Rod units in this same 24-unit family.

**Next-cycle plan for these 24:** (1) resolve the `MAGICHIT` sign question against a corpus record
whose real value is independently known (e.g. from `AT-33-E2-002`'s own worked fixture) before
trusting any comparison; (2) grant the right `PROFICIENCY`/`FEAT` in the fixture for the 4 Exotic
Rod units. No `src/rules_core/` change is anticipated — this is fixture engineering only.

## Finding 4 — dissonance pair has a second, independent gap even once psionics is unblocked

`ultimate_psionics:equipment_modifier:special_quality_dissonance_enhancement_bonus_{alt,main}` carry
`BONUS:WEAPON|DAMAGE,TOHIT|DissonanceEnhancementBonusAlt|TYPE=ENHANCEMENT` — **uppercase**
`TYPE=ENHANCEMENT`, not `TYPE=Enhancement`. `compute_equipmods_effect`'s literal string comparison
(`qualifiers[3] == "TYPE=Enhancement"`) is case-sensitive and would not match even after the
`ultimate_psionics` harness gap (Finding 2) is fixed; qualifiers[2] is also a variable name, not a
literal integer, so `compute_var_effect` (not `compute_equipmods_effect`) is the correct resolver for
the `VAR` half of this chain. Named for the next cycle, not fixed this cycle (out of the 75-unit
population's reachable-this-turn set).

## Verdict discipline

No `disagree` was recorded this cycle. `mattock_of_the_titans`'s `MAGICHIT` mismatch is a real,
observed discrepancy but is withheld from both `agree` and `disagree` because the harness's own
correctness for `MAGICHIT` specifically has not yet been independently confirmed (unlike
`MAGICDAMAGE`, which the same live run corroborates) — recording a verdict on an unverified
comparison would risk exactly the false-defect shape `AT-33-E5-003`'s doctrine forbids. This is
consistent with leaving the unit **unexamined** rather than writing a row with a wrong or premature
verdict.

## Status: blocked-escalated

**Not `complete`.** 8 of this lane's 75-unit population are genuinely examined with real,
per-unit `(ours, oracle, verdict)` rows and populated reasons on every `unverifiable` row. The
remaining 67 are named per-shape above with concrete structural reasons (two distinct, genuinely
unresolved harness defects; two fixture-engineering gaps; five genuinely unhandled engine shapes)
and a concrete next-cycle plan — not "ran out of time" vaguely.

## Movement, four buckets

- **Closure:** 8 units of this lane's 75-unit population get a real, committed oracle disposition for
  the first time (1 agree, 7 unverifiable, each reasoned).
- **Reclassification:** none — no unit's `docs/work-inventory.json` `status` field changed (oracle
  results live in this directory's JSON, matching every prior `AT-33-E5-00x` lane's convention).
- **Reachability:** 24 of 75 units confirmed reachable via an *already-existing* resolver
  (`compute_equipmods_effect`) plus a real, previously-undocumented indexing fix
  (`WEAPON.0`, not `WEAPON.1`) for the export side — real forward movement on reachability even
  though no row was written yet, because the `MAGICHIT` sign question is not yet resolved.
- **Instrument-correction:** 2 found — (1) `gradlew` vs the direct-`java` runner for the
  "Could not find campaign" failure class (real, confirmed, `scripts/retro.py incident`
  `sd33-r4-last75.jsonl`); (2) `WEAPON.<n>` export tokens are zero-indexed, not one-indexed
  (silently returns empty rather than erroring on an off-by-one, so this could otherwise reach a
  future cycle as a false "no data" verdict).

## Notes

This cycle spent real budget on root-causing two genuinely distinct harness defects rather than
forcing 75 rows through a partially-broken pipeline. The 8 rows committed are each independently
re-derivable and none is a guess. The 67 remaining are not a single "ran out of time" bucket — they
split into 2 harness defects (17 units, Findings 1+2), 2 fixture-engineering gaps with an existing
resolver (24 + 3 units, Finding 3 + wield-size-no-penalty), and 5 genuinely new engine shapes needing
their own RED→GREEN cycles (20 units: 6 COMBAT-non-AC, 4 DAMAGEMULT, 2 DAMAGESIZE, 1 RANGEADD, 1
WEIGHTDIV, 6 bare-WEAPON/ATTACKS — see the shape table for the exact per-shape count).

## RED→GREEN

No `src/rules_core/` change landed this cycle — every examined unit used an existing resolver
(`compute_general_effect`) or needed none (the `unverifiable` rows are structural facts about the
corpus record, not a computation). **Before this cycle:** `equipment-last75.oracle-results.json` did
not exist; 0 of the 75-unit population had any per-unit disposition. **After:** 8 real per-unit rows,
each backed by either a live PCGen export (`hunter_s_sight`) or a direct read of the corpus record's
full `raw_bonus_chains` (`demon_senses`'s `EQSLOT` grep, the three multi-skill chains, the three
`WIELDCATEGORY`-only chains).

## Test scoping

Ran `bash scripts/oracle_harness/charbuild_remainder_run_one.sh` (unmodified, reused) against 7 fresh
`.pcg`/`.ftl` fixtures built this cycle (`crystal_mask_psionic_craft` re-run, `circlet_of_persuasion`
control, `hunter_s_sight`, `demon_senses`, `demon_senses` `LOCATION:Eyes` variant, `rod_flailing`,
`mattock_of_the_titans`) plus one existing fixture re-run unmodified
(`meld_stone_alchemist`). All real, live PCGen invocations against the pinned oracle
(`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`). Ran
`python3 scripts/box_ledger.py --check --oracle-results .../equipment-last75.oracle-results.json`
(below). **Did not** run the root `cargo test` sweep or `apps/desktop/src-tauri` — no `src/` or
`apps/` file changed this cycle (results/receipt only).

```
$ python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-last75.oracle-results.json
```
(output recorded in `progress.md`; population/coverage figures are computed over the FULL merged
Epic-5 set at finalize time, not this lane's own 8-row file in isolation — this lane's file is a
disjoint addition, never a rewrite of the merged files, per this lane's own write-scope restriction.)

## Next-cycle plan

1. Root-cause the `ultimate_psionics` equipment-BONUS-not-registering defect (Finding 2) — unblocks
   14 SKILL + 1 COMBAT + 2 dissonance = 17 units.
2. Resolve `WEAPON.0.MAGICHIT`'s sign against an independently-known value, then grant Exotic Weapon
   Proficiency in the 4 Rod fixtures — unblocks up to 24 units (Finding 3) with **zero** new
   `src/rules_core/` code.
3. Build the base-armor+attached-`EQMARMOR`-modifier fixture pattern (named by
   `AT-33-E5-remainder-equipment_cycle_receipt.md`'s own next-cycle plan) — unblocks 3 units
   (`draco`/`dragonhide`/`material_dragonhide`); the resolver (`arms_armor::resolve_check_penalty`)
   already exists.
4. New `src/rules_core/` resolvers, each its own RED→GREEN cycle: `COMBAT|INITIATIVE`/`TOHIT.Ranged`
   (6 units, 2 formula-valued), `WEAPON|DAMAGEMULT` (4, fractional), `EQMWEAPON|DAMAGESIZE` (2),
   `EQMWEAPON|RANGEADD` (1), `EQM|WEIGHTDIV` (1), `WEAPON|ATTACKS` extra-attack formulas (3).
5. Fix the dissonance pair's case-sensitive `TYPE=ENHANCEMENT` vs `TYPE=Enhancement` mismatch in
   `compute_equipmods_effect` once Finding 2 unblocks live verification for those 2 units.
6. Re-run `AT-33-E6-001` as the next attempt once this and the sibling disagreement-resolution lane
   both land — population will still be short of 8,330 by up to 67 units unless a further cycle
   closes this lane's remainder.
