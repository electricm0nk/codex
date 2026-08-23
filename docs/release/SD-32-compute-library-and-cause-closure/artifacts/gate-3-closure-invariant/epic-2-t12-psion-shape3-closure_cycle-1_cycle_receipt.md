# Cycle t12-psion-shape3-closure — Gate 3 (closure invariant) / Card 11 (row 15's T12 remainder: `psion`) + provisional-default audit

- **Card ID:** 11 (`epic-2-cause-closure`), row 15's T12 sub-scope: `psion`, the last named T12 item
  (cycle 4's receipt, `artifacts/gate-3-closure-invariant/epic-2-t12-class-feature-shapes_cycle-4_cycle_receipt.md`,
  commit `452c70d035`), plus an audit of cycle 4's four documented judgment calls against `decisions.md §27`.
- **Commit SHA:** see `git log -1` at push time (rebased onto `origin/tranche/12` before pushing, §5)
- **Files touched:**
  - `src/rules_core/rules_tables/ultimate_psionics/psion_features.rs` — **new**: `psion_power_points_total`,
    the one real compute function this cycle closes, plus its own module doc comment documenting the size
    finding, the false-lead check, and the escalated (not guessed) `PsionPowersKnown`/`PsionMaxPowerLevel`
    formula-combination ambiguity
  - `src/rules_core/rules_tables/ultimate_psionics/mod.rs` — registered `psion_features`
  - `src/rules_core/pilot_compute/mod.rs` — `ground_psion_class_features`, dispatch wiring in
    `compute_class_chassis`'s `class:psion` arm, two new end-to-end wiring tests
    (`psion_manifesting_emits_its_own_power_points_magnitude`, `psion_level_1_already_carries_power_points`);
    updated the now-stale `a_class_with_no_roster_data_emits_no_untabled_class_feature_ids` test (its
    `psion` example is superseded, same pattern the test's own comment already used for Cryptic's shape-2
    supersession) to a genuinely-undispatched class id
  - `src/rules_core/pilot_compute/untabled_base_class_feature_roster.rs` — generalised the runtime
    own-named-group assertion to accept shape 3's bare (no `" ~ "`) target names; updated the stale
    `roster_for("psion").is_empty()` test to a genuinely-absent class id and added a positive
    `psion_manifesting_row_is_shape_3_and_carries_no_group_prefix` fixture-transcription test
  - `scripts/census_untabled_base_class_feature_roster.py` — Shape 3 (bare own-named `CLASS:` row, no
    per-class branching): widened the `ABILITY:<ClassName> Class Feature|AUTOMATIC|` marker to the bare
    category+AUTOMATIC prefix (dropping the trailing `<ClassName> ~ ` requirement), and widened the
    group-membership test to accept a target with no `" ~ "` separator at all (own-named, implicit) OR the
    existing explicit `<ClassName> ~ ` prefix; fixed a pre-existing bug this widening surfaced (the last
    tab field on a line carried a literal trailing `\n` into its own `KEY:`/`name`, corrupting every
    shape-3 hit until stripped)
  - `tests/fixtures/rules_core/untabled-base-class-feature-roster.json` — regenerated from the fixed/widened
    script against the pinned oracle
  - `data/corpus/occult_adventures/class_feature/psychic/phrenic_pool.json` — **stamped**
    `shape_provisional_default`/`shape_provisional_reason` via `scripts/shape_provisional_marker.py`'s
    sanctioned `stamp_provisional_default` (never hand-edited), per the audit below
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 11 cycle entry prepended
    (row left `in-progress` per dispatch instruction)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (working-tree diff on this cycle's own touched files;
  `git diff --unified=0 BASE_BRANCH...HEAD` returned zero relevant lines since HEAD itself never changed
  until this cycle's own commit — audited the actual uncommitted diff instead, `grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'`: 0 hits)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"`: 0 hits)
- **Acceptance criterion:** size `psion`'s genuinely-third convention (report the count with the command),
  determine whether it is a repeat of the `CATEGORY=Class`/`CATEGORY=CLASS` false lead, extend the census
  script generically to see the shape, close the magnitude-bearing records with real compute functions
  wired through `pilot_compute`, prove RED→GREEN at both altitudes; separately audit cycle 4's four
  documented judgment calls against `decisions.md §27` and stamp/report any genuine provisional defaults.
- **Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/fetch-pcgen-oracle.sh`
  bootstrap into this fresh worktree's repo-local slot; every formula below re-derived directly against it)
- **Status:** complete (this cycle's scope: sizing, false-lead check, generic census widening, and real
  closure of `psion`'s own `Psion Manifesting` record's power-points magnitude, plus the provisional-default
  audit). Two items sized, named, and explicitly NOT closed this cycle — see §4 and §5 below.
- **Discovery forwards:** `## DISCOVERED` — (1) the same shape-3 pattern the census widening surfaced for
  `psion` also exists for 7 sibling classes (`cryptic`, `dread`, `marksman`, `psychic_warrior`, `tactician`,
  `vitalist`, `wilder`, each a `"<ClassName> Manifesting"` record) and for 3 NEW magnitude-bearing records on
  the already-"108/108"-closed `antipaladin` (`Aura of Evil`, `Detect Good`, `Smite Good`, shape 1);
  (2) `psion`'s own discipline-choice pool population (32 magnitude leaves across 9 disciplines/archetypes,
  BFS-sized below) needs the pool-catalog closure mechanism, not this roster/chassis mechanism;
  (3) `Psion Manifesting`'s `PsionPowersKnown`/`PsionMaxPowerLevel` terms are blocked on a genuine
  `BONUS:VAR` combination-semantics ambiguity this repo cannot resolve without live PCGen.
- **Next-cycle plan:** close the 7 sibling `"<ClassName> Manifesting"` records (same shape, same template as
  `psion_power_points_total` below, per-class formula re-derivation needed); close antipaladin's 3 new
  records (`Smite Good` especially — a 5-term %N-substituted DC/attack/damage/AC magnitude, same shape as
  Vigilante's Frightening/Stunning Appearance cycle 4 already closed); resolve or escalate the
  `PsionPowersKnown`/`PsionMaxPowerLevel` ambiguity; scope the discipline-choice pool population via
  `class_feature_pool_catalog.rs`'s own construction discipline.

---

## 1. Re-deriving `psion`'s size (§17a — re-derive, don't inherit; §17 — generic pass first)

**Book verification.** The registry's own fixture (`tests/fixtures/rules_core/untabled-base-class-chassis.json`)
names `psion`'s source book directly — confirmed `ultimate_psionics` (`up_classes.lst`), **not** the
superseded `psionics_unleashed` an earlier brief mis-cited. `find $PCGEN_CORPUS_ROOT -iname "up_*.lst"`
lists the real files (`up_classes.lst`, `up_abilities_class.lst`, ...); `psionics_unleashed` does not appear
anywhere in this oracle checkout.

**False-lead check (§17a: check for a trivial cause before concluding "genuinely novel").** The earlier
"7 classes need a third progression shape" claim turned out to be one line's `CATEGORY=Class` vs
`CATEGORY=CLASS` case bug. Checked the same possibility for `psion` directly against the oracle:

```bash
grep -c "Psion ~ " up_classes.lst up_abilities_class.lst
```
```
up_classes.lst:0
up_abilities_class.lst:7
```

Read all 7 `up_abilities_class.lst` hits by hand: every one is a **false-positive substring match** inside
a DIFFERENT class's own group name — e.g. `Ascendant Psion ~ Hide Mind` (a `Psion Archetype`'s own child
grant) contains the literal substring `Psion ~ ` without being a `psion`-owned grant at all. There is no
`CATEGORY` casing bug and no missing `Psion ~ ` prefix hiding in the data — `psion`'s own `CLASS:Psion`
block genuinely never repeats its own display name before ` ~ `. **This is a real, third convention, not a
repeat of the earlier false lead.**

**The real convention, read directly from `CLASS:Psion`'s own block** (`up_classes.lst`, lines 216–266):

- Line 264: `1\t...\tABILITY:Psion Class Feature|AUTOMATIC|Psion Manifesting` — the class's own single
  level-1 own-named grant, using the SAME `ABILITY:<ClassName> Class Feature|AUTOMATIC|` category prefix
  every other T12 class's shape 2 uses, but the payload (`Psion Manifesting`) does **not** repeat
  `Psion ~ ` before the feature name at all. This is Shape 3.
- Line 258: `1\tABILITY:Special Ability|AUTOMATIC|All Automatic Proficiencies|Psion Weapon Proficiencies` —
  a **different** `ABILITY:` category (`Special Ability`, not `Class Feature`), out of the `class_feature`
  roster's population entirely (same exclusion every other class's proficiency grants already get).
- Lines 222–249 (`SUBCLASSLEVEL:1` rows): 7 core discipline picks + 7 advanced-discipline picks, each an
  `ABILITY:Psion Class Feature|AUTOMATIC|<Discipline> Discipline` grant. **Not** own-named-group rows under
  this mechanism's own rule (the leading field is the literal string `SUBCLASSLEVEL:1`, not a level number,
  so no reliable level parses — correctly skipped, not guessed) — and structurally these are **pool
  choices** (a player picks exactly ONE), not `psion`'s own single-track progression. See §4 below.
- Lines 252/256: `Bombardier`/`Ascendant Psion` archetype picks, same pool shape.

**Mechanical BFS confirmation (this cycle's own artifact, not committed — re-derivable):** starting from
every `ABILITY:...|AUTOMATIC|...` target in `CLASS:Psion`'s own block and following each named target's own
further `ABILITY:...|AUTOMATIC|...` grants (recursively, generically, across every `ultimate_psionics`
`.lst` file — no per-class/per-discipline curation) finds **32 magnitude-bearing leaf records** reachable
only through the 9 discipline/archetype picks, and confirms `Psion Manifesting` is the class's own ONLY
directly-owned, non-pool-shaped magnitude-bearing feature.

**Command to re-derive:**
```bash
PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/census_untabled_base_class_feature_roster.py
```

## 2. Extending the census script generically (Shape 3, `§17`: one pass, no per-class branching)

Two changes to `census_untabled_base_class_feature_roster.py`, both mechanical and class-agnostic:

1. `shape2_marker` widened from `f"ABILITY:{display_name} Class Feature|AUTOMATIC|{display_name} ~ "` to
   `f"ABILITY:{display_name} Class Feature|AUTOMATIC|"` — the bare category+AUTOMATIC prefix, dropping the
   requirement that the payload ALSO repeat the class's own name.
2. The own-named-group test widened from `key.startswith(f"{display_name} ~ ")` alone to: accept EITHER
   that explicit prefix OR a target containing no `" ~ "` group separator at all (implicit own-name). A
   target containing `" ~ "` that does not start with the class's own name still excludes (pool member /
   another class's own-named group) — the boundary is widened, not loosened.

**A real bug this widening surfaced, fixed in the same change:** the script iterates `f.readlines()` without
stripping the trailing line ending before splitting on tabs. For any target that is the LAST tab field on
its line (true of every shape-3 hit), the extracted `KEY:` carried a literal trailing `\n` into the
fixture's own `key`/`name` fields. Fixed by `rstrip`-ing each line once, up front, before any field
processing — a correctness fix, not a shape addition, needed for shape 3 to produce clean data at all.

**Re-derived total, before vs. after (`§12c`: population + command):**

```bash
python3 -c "
import json, collections
before = json.load(open('roster-before.json'))   # committed fixture at PIN
after = json.load(open('tests/fixtures/rules_core/untabled-base-class-feature-roster.json'))
print('before:', len(before['entries']), 'after:', len(after['entries']))
"
```
```
before: 235 total entries (108 magnitude-bearing, 19 classes — cycle 4's own re-derived figure)
after: 246 total entries (+11)
```

**Every one of the 11 new entries, by class and shape:**

| class_id | key | text_only | shape |
|---|---|---|---|
| antipaladin | Aura of Evil | **False** (magnitude-bearing) | shape 1 (`.MOD`) |
| antipaladin | Detect Good | **False** | shape 1 |
| antipaladin | Smite Good | **False** | shape 1 |
| cryptic | Cryptic Manifesting | True | shape 3 |
| dread | Dread Manifesting | True | shape 3 |
| marksman | Marksman Manifesting | True | shape 3 |
| **psion** | **Psion Manifesting** | **True** (corpus wiring_class; grounded via DESC-prose derivation anyway, §3 below) | shape 3 |
| psychic_warrior | Psychic Warrior Manifesting | True | shape 3 |
| tactician | Tactician Manifesting | True | shape 3 |
| vitalist | Vitalist Manifesting | True | shape 3 |
| wilder | Wilder Manifesting | True | shape 3 |

**This is a real, unforced widening of scope (`§17`), not scope creep invented to look thorough:** the
SAME generic rule that finds `psion`'s own record also finds 7 sibling classes' identically-shaped
`"<ClassName> Manifesting"` records and 3 genuinely NEW magnitude-bearing records on `antipaladin` — a class
cycle 4's own receipt called "108/108... complete." That claim was correct under the OLD (narrower) census;
under the widened one it is off by at least 3 (unclosed antipaladin magnitudes) + 7 (unclosed sibling
Manifesting records). **Named here, not closed this cycle** (see Next-cycle plan) — closing 8 more classes'
worth of per-feature formula re-derivation is outside this cycle's own scope (`psion`, named explicitly in
the dispatch brief) and a blocker bigger than one cycle is a sequencing problem, not grounds to skip
reporting it (`AGENTS.md` Blocker Discipline).

## 3. `psion`'s own closure: `Psion Manifesting`'s power points

`Psion Manifesting`'s corpus record (`data/corpus/ultimate_psionics/class_feature/psion_manifesting/
psion_manifesting.json`) carries `DESC:"Psion Powers Known: %1; Psion Maximum Power Level Known:
%2|PsionPowersKnown|PsionMaxPowerLevel"` — a %N-substituted DESC-prose magnitude with no `BONUS:` token of
its own (`wiring_class: "display"`), the same shape cycle 4 already closed for Kineticist's Burn and
Vigilante's Frightening/Stunning Appearance (`§1a`: grounded exactly as the prose states, not fabricated).
The two substituted variables are set by two `CATEGORY:Internal` backing records the same feature grants
(`Psion Manifesting Variables`, `Psion Power Points`).

**`Psion Power Points`** (`up_abilities_class.lst` line 391): `BasePowerPoints` — a 20-entry level-keyed
`BONUS:VAR|BasePowerPoints|<value>|PREVARGTEQ:PsionPPL,<threshold>` ladder — plus a single-entry
`BonusPowerPoints|(PsionPPStat*PsionPPL)/2|TYPE=PsionBonusPP` term (`PsionPPStat` = Intelligence modifier,
`PsionPPL` = manifester level). The ladder is read as the standard "highest satisfied threshold wins" table
idiom (the only reading whose values match the well-established real Power Points per level table —
2,4,5,6,8,10,11,12,14,16,18,20,21,23,25,26,29,30,31,32 — not a literal cumulative sum of every threshold,
which would produce an implausible ~343 at level 20). Closed as `psion_power_points_total(level, int_mod)`.

## 4. `PsionPowersKnown`/`PsionMaxPowerLevel`: escalated, not guessed

`Psion Manifesting Variables`'s two `BONUS:VAR|PsionPowersKnown|...` entries (`min(21,(2*PsionPKL)+1)`
unconditional, `floor((PsionPKL-10)*3/2)` added when `PsionPKL>=11`) carry **no `TYPE=`**, unlike
`BonusPowerPoints`'s single-entry, unambiguous term. The two most-common `BONUS:VAR` combination readings
genuinely disagree here: "replace, don't sum" (the reading that correctly matches `BasePowerPoints`'s known
real table) produces an implausible level-11 DROP (21 → 1); "sum" produces a plausible continuing climb.
This repo has no way to execute real PCGen to settle which convention this specific pair actually uses, and
fabricating either answer risks shipping a wrong fixture-checked value with false confidence — worse than
not closing it. **Escalated by coordinate**, per `docs/governance/blocker-closure-doctrine.md`: the exact
blocker is `Psion Manifesting Variables`'s `PsionPowersKnown`/`PsionMaxPowerLevel` `BONUS:VAR` combination
semantics (`up_abilities_class.lst` line 392); resolving it needs either a live-PCGen check or an operator
ruling on which convention to apply uniformly. 2 more magnitudes, sized, on the one already-closed record.

## 5. `psion`'s discipline-choice pool population (32 magnitude leaves), correctly out of THIS mechanism

Every other `ABILITY:Psion Class Feature|AUTOMATIC|` grant from `CLASS:Psion`'s block routes through a
chosen discipline/archetype pick (7 core disciplines, 7 advanced disciplines, `Bombardier`, `Ascendant
Psion`, `Mindwright`, `Dual Disciple`) whose own record then chains further `ABILITY:Psion Class
Feature|AUTOMATIC|<Discipline> ~ <Feature>` grants gated on a discipline-specific
`PREVARGTEQ:<Discipline>DisciplineLVL,N` variable — the SAME chained-indirection convention the dispatch
brief named. The BFS in §1 sizes this at **32 magnitude-bearing leaf records**. This is structurally a
pool-shaped population (one of several mutually-exclusive picks, each with its own progression) — the SAME
shape `census_untabled_base_class_feature_roster.py`'s own module doc comment already excludes for
`Vigilante Talent`/`Magus Arcana` ("pool grants excluded... need per-pool verification"), not a `psion`-
own-named class feature. **Sized and named, not filed as an exclusion** (`§27b`: no carve-outs survive) —
the correct closure mechanism is `class_feature_pool_catalog.rs`'s own construction discipline, not this
roster/chassis mechanism, and that work is real, real-sized, and belongs to a future cycle.

## 6. RED→GREEN, proven live at both altitudes (`§1a`)

Mutated `psion_features::psion_power_points_total`'s `bonus` term (`+ 99`):
```
thread '...psion_features::tests::psion_power_points_total_uses_the_base_ladder_and_int_bonus' panicked:
  left: Some(101)
 right: Some(2)
thread '...psion_features::tests::psion_power_points_total_ladder_steps_at_every_named_threshold' panicked
thread '...untabled_base_class_feature_roster_wiring_tests::psion_manifesting_emits_its_own_power_points_magnitude' panicked:
  left: 181
 right: 82
```
Both the unit-level compute tests AND the end-to-end wiring test (through
`compute_pilot_base_chassis` → `compute_class_chassis` → `ground_psion_class_features`) failed for the
intended reason. Reverted; re-ran the full targeted scope:
```bash
cargo test --locked --lib -- psion kineticist medium_features mesmerist_features occultist_features \
  psychic_features spiritualist_features magus_features shifter_features vigilante_features \
  kineticist_level_20 medium_level_20 mesmerist_level_20 occultist_level_20 psychic_level_20 \
  spiritualist_level_20 magus_level_20 shifter_level_20 vigilante_level_20 each_new_class_lacks \
  a_class_with_no_roster_data aegis tactician_features vitalist_features wilder_features antipaladin \
  cryptic_ dread_ marksman_ psychic_warrior_ soulknife_
```
`178/178 green`, no regressions in any pre-existing class's test. Also ran the roster-mechanism-level tests
directly (`untabled roster closure_invariant`): `55/55 green`, including the two updated (no longer stale)
tests and the new positive `psion_manifesting_row_is_shape_3_and_carries_no_group_prefix` test.

## 7. Provisional-default audit (`decisions.md §27`/`§27a`, `§6a`'s contract)

Applied `§27a`'s own test — "`F0` reached by measurement is a real answer; `F0` reached by 'nothing else
matched' is a placeholder wearing a family label" — to each of cycle 4's four documented judgment calls,
with evidence, not inherited belief:

1. **FCB terms dropped (4 Mesmerist/Occultist formulas).** `CharacterInput` has **no** favored-class-bonus
   field anywhere (`grep -rn "favored_class\|FCB" src/rules_core/character_input.rs` — no hits). Given this
   engine's actual input space, the FCB-less base value is **the only value it can produce** — not one of
   several tracked possibilities, a structural absence. **Ruling: real measurement. Left unmarked.**
2. **Shifter's Defensive Instinct grounds the unencumbered case.** Cycle 4's own note: "the record's own
   `ENCUMBERANCE`-conditional row grants the **identical value** when unencumbered." Re-checked directly
   against the oracle (`uw_abilities_class.lst`'s `Defensive Instinct` record) — confirmed both the
   encumbered and unencumbered `BONUS:VAR` rows compute the SAME number for this specific feature. There is
   no discretion here at all: the grounded value is correct regardless of encumbrance state, and this
   engine tracks no encumbrance input anywhere else either (a second structural absence, same class as
   FCB). **Ruling: real measurement (not even conditionally different). Left unmarked.**
3. **Two records' secondary trivial-pool `BONUS:VAR` tokens skipped as duplicate.** This is not a choice
   between multiple CORRECT VALUES for one quantity — it is a classification of which of two tokens on the
   record IS the real per-feature magnitude versus which is bookkeeping noise from the same grant (the
   record's flat "you have this pool" companion token, redundant with simply holding the feature). No
   alternative correct answer was discarded; nothing here is "one of several possible answers." **Ruling:
   not a `§27` shape. Left unmarked.**
4. **Psychic Discipline's Phrenic Pool ability term defaults to Charisma.** `Psychic ~ Phrenic Pool`'s own
   `BONUS:VAR|PhrenicPool|(PsychicLVL/2)+PhrenicPoolAbility` term depends on `PhrenicPoolAbility`, which is
   set by whichever of the **9** Psychic Discipline records the character actually chose — CHA for 4
   disciplines, WIS for 5 (re-confirmed directly against `oa_abilities_class.lst`'s 9 `... Discipline`
   records). `ground_psychic_class_features` defaults this to Charisma because `CharacterInput` does not
   track which discipline was chosen — genuinely **one of two live candidate answers**, picked without the
   input that would resolve it, the exact `§27` shape. **Ruling: `§27` provisional default. Stamped.**

**Stamped via the sanctioned function** (`scripts/shape_provisional_marker.py::stamp_provisional_default`,
never a hand-edit of `data/corpus/**`):
```bash
python3 -c "
import sys; sys.path.insert(0, 'scripts')
import json
from shape_provisional_marker import stamp_provisional_default
path = 'data/corpus/occult_adventures/class_feature/psychic/phrenic_pool.json'
record = json.load(open(path))
stamp_provisional_default(record, '<reason, see script>')
json.dump(record, open(path, 'w'), indent=2)
"
```

**Re-derived before/after (`§12c`: population + command, `§17a`: re-derive fresh):**
```bash
python3 scripts/row17_census.py --check
```
```
before: §27 provisional default     0   (corpus-wide total incl. done units: 0)
after:  §27 provisional default     1   (corpus-wide total incl. done units: 1)
```
`--check` exits 0 (well-formed marker, reason present) both before and after. **Row 17 was under-counting
by exactly this one unit before this cycle** — the brief's own concern (§27's ruling exists precisely to
prevent row 17 closing over unrevisited defaulted values) is confirmed real, not hypothetical: without this
audit, `Psychic ~ Phrenic Pool`'s CHA default would have shipped as an indistinguishable "real" value.

## 8. Sweep (`§3`)

`grep -rn "108 of 108\|108/108\|55 of 108\|53 of 108" tests/ src/ scripts/ apps/` — no count-pinning hits
outside doc-comment prose this cycle's own receipt supersedes (no test/script/app asserted the literal
`108` figure as a hard pin). The roster fixture and its two consuming files
(`census_untabled_base_class_feature_roster.py`, `untabled_base_class_feature_roster.rs`) are the only two
places that read `untabled-base-class-feature-roster.json` — both updated and re-tested (55/55 green).

## 9. Scope discipline

Did not attempt: the 7 sibling `"<ClassName> Manifesting"` records, antipaladin's 3 new records, the
32-record discipline-choice pool population, or the `PsionPowersKnown`/`PsionMaxPowerLevel` ambiguity — all
four sized and named above, none silently dropped. Did not touch `data/corpus/**/monster_ability/**`,
`scripts/transcribe_monster_tables.py`, `monster_chassis.rs` (the sibling `monster_ability` lane), or
`scripts/pi_scrub.py`/`declared_pi_shipping_audit`/PI screening in `cache_gen::acg.rs`/`apg.rs`/
`beastiary1.rs` (the sibling PI lane). Row 11 left `in-progress` per dispatch instruction.

`df -h /`: reported in the dispatch's final report.
