# Cycle epic-2-t12-class-feature-shapes — Gate 3 (closure invariant) / Card 11 (row 15's T12 remainder)

- **Card ID:** 11 (`epic-2-cause-closure`), row 15's named T12 sub-scope: the 7 classes with zero
  own-named class-feature coverage, plus the 80/108 magnitude-bearing records across covered classes.
- **Commit SHA:** see `git log -1` at push time (rebases before pushing per §5)
- **Files touched:**
  - `scripts/census_untabled_base_class_feature_roster.py` — case-fold fix to the shape-1 `CATEGORY=`
    match
  - `src/rules_core/pilot_compute/untabled_base_class_feature_roster.rs` — updated doc comment, added
    two tests (Kineticist shape-1 fixture-check, five-class no-longer-empty guard)
  - `tests/fixtures/rules_core/untabled-base-class-feature-roster.json` — regenerated (135 -> 235
    records, 13/20 -> 19/20 classes)
  - `src/rules_core/rules_tables/apg/mod.rs` — registered new module
  - `src/rules_core/rules_tables/apg/antipaladin_features.rs` — **new**: 7 real per-feature compute
    functions for Antipaladin's magnitude-bearing roster records
  - `src/rules_core/pilot_compute/mod.rs` — `ground_antipaladin_class_features` + dispatch wiring +
    3 end-to-end tests
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 11 correction, prepended
  - `docs/retro/events/t9-onboarding.jsonl` — correction event
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff, this cycle's touched files only —
  grepped for bundle/wave/tranche tags, none found in new code, only in doc-comment citations of prior
  cycle receipts which is expected)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — no stub markers, no placeholder
  returns; every new function is a real formula transcribed from the corpus's own `BONUS:VAR` tokens
  and reaches `compute_pilot_base_chassis` through the real dispatch path, proven by the wiring tests)
- **Acceptance criterion:** re-derive the shape inventory for the 7 classes by file:line against the
  oracle (not trust the inherited framing), extend the census script generically if a real instrument
  defect is found, identify `medium`'s grant route, report the magnitude-bearing population honestly,
  and attempt at least one group of real compute functions end-to-end.
- **Corpus SHA:** not re-derived — no `data/corpus/**` write this cycle (fixture regen reads the
  existing oracle checkout only, writes `tests/fixtures/`, not `data/corpus/`)
- **Status:** complete (this cycle's scope only — does not close row 15 or card 11; 101 of 108
  magnitude-bearing records across 18 classes remain named, not attempted)
- **Discovery forwards:** the census script's case-sensitivity defect (item below), the wrong-book
  citation for `psion` in the inherited brief, the 80->108 magnitude-bearing re-derivation
- **Next-cycle plan:** the remaining 101 magnitude-bearing records across 18 classes (Aegis 7, Cryptic
  6, Dread 6, Kineticist 6, Magus 5, Marksman 5, Medium 7, Mesmerist 10, Occultist 6, Psychic 4,
  Psychic Warrior 3, Shifter 5, Soulknife 4, Spiritualist 3, Tactician 6, Vigilante 7, Vitalist 6,
  Wilder 5 — `python3 -c` census over the regenerated fixture, command in §4 below), worked one class
  at a time the same way this cycle worked Antipaladin; `psion`'s genuinely-third convention, once
  scoped (start by checking whether its chained per-discipline `ABILITY:` indirection can be walked
  generically or needs a hand-authored roster the way `push_pu_class_feature_records` is).

---

## 1. Re-deriving the shape inventory (§17a — validate before trusting a lead)

The dispatch brief's own evidence pointer named a receipt
(`epic-2-feat-prereqs-stale-eligible-count-fix_cycle-1_cycle_receipt.md`) that turned out to be about
an unrelated fix (the `701`/`755` feat-count pin); the actual psion/kineticist/medium evidence was
never in that file. Re-derived everything directly against the oracle instead.

```bash
export PCGEN_REPO_DIR=<worktree>/docs/release/.../artifacts/corpus/operator-supplied/pcgen
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
grep -n "^1\tABILITY:" "$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/occult_adventures/oa_classes.lst"
```
```
13:1	ABILITY:CLASS|AUTOMATIC|Kineticist
44:1	ABILITY:CLASS|AUTOMATIC|Medium
74:1	ABILITY:CLASS|AUTOMATIC|Mesmerist
106:1	ABILITY:CLASS|AUTOMATIC|Occultist
139:1	ABILITY:CLASS|AUTOMATIC|Psychic
172:1	ABILITY:CLASS|AUTOMATIC|Spiritualist
```

All 6 `occult_adventures` classes carry the identical wrapper row. Followed the wrapper (`Kineticist`
ability, `CATEGORY:CLASS`, in `oa_abilities_class.lst:19`) to its own `DEFINE:Kineticist_CFP_Level|0`
and `BONUS:VAR|Kineticist_CFP_Level|KineticistLVL` tokens, then searched for the `.MOD` shape keyed to
that same level variable:

```bash
grep -rhoP "CATEGORY=\w+\|Kineticist\.MOD" "$PCGEN_CORPUS_ROOT"
```
```
CATEGORY=CLASS|Kineticist.MOD
```

**Found it: `CATEGORY=CLASS` (uppercase), not `CATEGORY=Class` (mixed case) —
`census_untabled_base_class_feature_roster.py`'s own `is_shape1` check
(`f"CATEGORY=Class|{display_name}.MOD" in line`) is case-sensitive and only matches the mixed-case
form.** Confirmed the same for all 6 classes:

```bash
grep -rhoP "CATEGORY=CLASS\|\w[\w ]*\.MOD" "$PCGEN_CORPUS_ROOT" | sed -E 's/\.MOD//' | sed -E 's/.*\|//' | sort -u
```
```
Kineticist
Medium
Mesmerist
Occultist
Psychic
Shifter
Spiritualist
Vigilante
```
(Shifter/Vigilante are already covered elsewhere — via shape 2 and shape 1's mixed-case form
respectively — so the case bug was silently absorbed for them and only visible for the 6 with no other
route in.)

**This is one instrument defect closing 6 classes generically, not 6 new shapes.** `medium`'s grant
route was never unidentified — it is the identical shape-1 row at `oa_abilities_class.lst`, missed by
the same case bug.

### `psion`: the inherited brief cited the wrong book

The brief's own file:line (`psionics_unleashed_classes_base.lst:33`) is real corpus text, but
`psionics_unleashed` is not psion's registered source book:

```bash
grep -n 'class_id.*psion' -A2 -B1 tests/fixtures/rules_core/untabled-base-class-chassis.json
```
```
"class_id": "class:psion",
"display_name": "Psion",
"source_book": "ultimate_psionics",
"source_file": "pathfinder/dreamscarred_press/ultimate_psionics/up_classes.lst",
```

Checked the real source (`up_classes.lst:264`, chained through `up_abilities_class.lst:390`):
```
264	1									ABILITY:Psion Class Feature|AUTOMATIC|Psion Manifesting
390 Psion Manifesting  CATEGORY:Special Ability  TYPE:PsionClassFeatures.SpecialQuality ...
```
`Psion Manifesting` has no `"Psion ~ "` group separator at all — a genuinely different, third
convention: a singly-named `ABILITY:<Class> Class Feature|AUTOMATIC|<Feature>` grant (not a `.MOD`
list, not a `CLASS:` level-table row) that then chains through further per-discipline abilities (e.g.
`Clairsentience Discipline` itself grants `Psion Class Feature|AUTOMATIC|Clairsentience ~ ...` rows —
`up_abilities_class.lst:396`). Confirmed `psion` has zero `"Psion ~ "`-prefixed own-named rows anywhere
in the oracle (`grep -rn "Psion ~ " $PCGEN_CORPUS_ROOT` — the 7 hits found are all `"Ascendant Psion ~
"`, an archetype's own group, not the base class's). Sized, not closed, this cycle — walking the
discipline-indirection chain generically is real scoping work for a next cycle.

## 2. Generic fix, one line, no per-class branching (§17)

```python
is_shape1 = f"category=class|{display_name.lower()}.mod" in line.lower()
```

```bash
python3 scripts/census_untabled_base_class_feature_roster.py
```
```
total records: 235
classes with data: [... 19 classes ...]
classes with NO `.MOD`-shaped own-named grant found (not covered by this mechanism): ['psion']
```

Before: 135 records, 13/20 classes. After: 235 records, 19/20 classes. Verified the 13 pre-existing
classes' per-class counts are byte-identical to the pre-fix fixture (no regression):
```
aegis 9, antipaladin 12, cryptic 12, dread 9, magus 16, marksman 7, psychic_warrior 7, shifter 17,
soulknife 7, tactician 11, vigilante 12, vitalist 10, wilder 6
```
— all unchanged. The 100 new records are exactly the 6 newly-covered classes (kineticist 18, medium
17, mesmerist 15, occultist 16, psychic 12, spiritualist 22 = 100), confirmed by summing the script's
own per-class breakdown.

## 3. RED→GREEN, proven live, twice (§1a)

**Fixture-level (Python):** reverted the case-fold to the original mixed-case-only match, re-ran the
census script — reproduced the original 135/13-classes/7-zero-matches state exactly (including
`kineticist` back in the zero-matches list). Restored the fix, re-ran — 235/19-classes/psion-only
zero-matches, confirmed reproducible.

**Rust-level, unit and end-to-end:** added `kineticist_class_skills_matches_the_oracle_s_uppercase_
category_shape_1_grant` (fixture-checked against `oa_abilities_class.lst:37`) and `the_other_five_
occult_adventures_classes_are_no_longer_empty`. Mutated the fix back to the original literal, re-ran:
```
test ...kineticist_class_skills_matches_the_oracle_s_uppercase_category_shape_1_grant ... FAILED
thread '...' panicked at src/rules_core/pilot_compute/untabled_base_class_feature_roster.rs:209:14:
Class Skills must be in the fixture
```
Reverted, re-ran: 12/12 green (`cargo test --locked --lib untabled_base_class_feature_roster`).

## 4. Re-deriving the magnitude-bearing population (§17a)

The brief's own "80 magnitude-bearing records across the 13 already-covered classes" figure predates
this cycle's fix, which added 100 more records (28 of them magnitude-bearing) under 6 newly-covered
classes:
```bash
python3 -c "
import json, collections
d = json.load(open('tests/fixtures/rules_core/untabled-base-class-feature-roster.json'))
mag = [e for e in d['entries'] if not e['text_only']]
print('total:', len(mag))
print(collections.Counter(e['class_id'] for e in mag))
"
```
```
total: 108
Counter({'mesmerist': 10, 'aegis': 7, 'antipaladin': 7, 'medium': 7, 'vigilante': 7, 'cryptic': 6,
'dread': 6, 'kineticist': 6, 'occultist': 6, 'shifter': 6, 'tactician': 6, 'vitalist': 6, 'magus': 5,
'marksman': 5, 'wilder': 5, 'psychic': 4, 'soulknife': 4, 'psychic_warrior': 3, 'spiritualist': 3})
```
**108 across 19 classes, not 80 across 13.** Reported, not silently carried forward.

## 5. One group attempted end-to-end, per the dispatch brief's own requirement

Built real per-feature compute functions for all 7 of Antipaladin's magnitude-bearing records
(`src/rules_core/rules_tables/apg/antipaladin_features.rs`), every formula transcribed from the
corpus's own already-ingested `BONUS:VAR` tokens (`data/corpus/advanced_players_guide/class_feature/
antipaladin/*.json`), not from memory of the printed rulebook:

| Record | Formula (source: corpus `raw_tokens`) |
|---|---|
| Touch of Corruption uses/day | `level/2 + CHA` (identical to Paladin's Lay on Hands) |
| Touch of Corruption dice | `level/2` d6 |
| Unholy Resilience save bonus | `max(CHA, 0)` |
| Cruelty DC | `10 + CHA + level/2` |
| Cruelties known | `min(level/3, 6)` |
| Channel Negative Energy dice | `(level+1)/2` d6 |
| Channel Negative Energy DC | `10 + level/2 + CHA` |
| Fiendish Boon selections | `min((level-1)/4, 4)` |
| Aura of Depravity | flat DR 5/good (no `BONUS:VAR` token — genuinely not level-scaled) |
| Unholy Champion caster level | antipaladin level |

**Upstream data bug found and NOT perpetuated (§22):** `unholy_champion.json`'s own
`BONUS:VAR|UnholyChampionCasterLevel|HolyChampionLVL` reads a Paladin-only tracker variable that is
never set on an Antipaladin — literal PCGen behavior would silently zero the capstone's Banishment
caster level. Grounded the RAW-correct value (antipaladin level, the formula this token was clearly
copy-pasted from) instead, documented in the module's own doc comment.

Wired into `compute_class_chassis` via `ground_antipaladin_class_features`, called immediately after
`push_untabled_base_class_feature_records` for `class:antipaladin` only. 8 pure-function tests, 3
end-to-end wiring tests (level 20 with a real +3 Charisma modifier reaching every one of the 10 explanation
values; level 1 confirming none fire early). RED→GREEN proven live (mutated
`touch_of_corruption_uses_per_day` to add `+99`; both the unit test and the end-to-end wiring test
failed for the intended reason — `left: 112, right: 13` at the wiring level; reverted, re-ran: 15/15
green).

## 6. Sweep (§3)

```bash
grep -rn '\b135\b' tests/ src/ scripts/ apps/
```
No hit refers to this fixture's population (all pre-existing, unrelated: `PRECLASS` counts, feat-book
line numbers, page citations, etc.). No other file pins the old `135`/`13 classes`/`7 zero-coverage`
figures — checked via `grep -rln` for the class-name list and "zero-coverage" phrasing across
`src/`, `tests/`, `scripts/`, `docs/release/SD-32-.../` — only this cycle's own touched files and two
prior receipts (historical, not live assertions) hit.

## 7. Scope discipline

Did not attempt: the remaining 101 magnitude-bearing records across 18 classes (named above, by
class and count); `psion`'s genuinely-third convention (sized, not closed — see §1); the pool-shaped
groups (`Vigilante Talent`, `Magus Arcana`, ...) `census_untabled_base_class_feature_roster.py`'s own
doc comment already excludes by design; row 15's own 27,847 kind-unenumerable-object scope (untouched,
different sub-lane). Rows 11 and 15 both left `in-progress`.

`df -h /`: reported in the dispatch's final report.
