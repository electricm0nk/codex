# Cycle AT-33-E5-last39-skill-combat — Epic 5 Re-verification / AT-33-E5-002, AT-33-E5-003

- **Commit SHA:** recorded below at push time (`sd33-r6-skillcombat`, remediation wave 6)
- **Files touched:**
  - `src/rules_core/equipment_effects/equipmods.rs` — real engine fix (RED→GREEN): new
    `resolve_var_reference`/`resolve_bonus_magnitude` (a `WEAPON` chain's magnitude segment may
    now be the NAME of a variable the SAME record defines via its own `BONUS:VAR|<name>|<n>`
    chain, never a cross-record lookup) + case-insensitive `TYPE=Enhancement` match (closes the
    `TYPE=ENHANCEMENT` uppercase gap `AT-33-E5-last75`/`AT-33-E5-last67-skill-combat` both named
    but did not fix)
  - `src/bin/e5_last39_skill_combat_ours.rs` (new) — repo-local batch "ours" probe, real live
    calls into `codex::rules_core::equipment_effects::compute_equipment_effects`
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-skill-combat.oracle-results.json` (new, 11 rows)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-skill-combat-work/` (new — working evidence: `dissonance_main.pcg`/`.run.log`/`.export.txt`, `weapon_magic.txt.ftl`, `ours-probe.json`, kept for re-derivation)
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated in place)
  - `docs/retro/events/sd33-r6-skillcombat.jsonl` (new)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.
  >
  > ### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated

## Population re-derivation (first action, per the brief)

```
$ python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d})
print(len(miss))"
39
```
Matches the brief's stated 39 (23 weapon-shape + 9 skill-combat-shape + 7 eqm-shape, per
`AT-33-E6-001` attempt 6's own shape table). **This lane's own slice, re-derived by reading every
candidate's own `raw_bonus_chains` directly (not a filtered view), not assumed from the brief's
count:**

The brief names 5 units explicitly (`companion_stone_far_sight`, `flurry_of_fists`,
`flurry_of_strikes`, the two `dissonance` `equipment_modifier` units) plus "the remaining
non-psionics COMBAT-shape units (INITIATIVE / TOHIT.Ranged / formula-valued AC and SAVE)",
stated as 9 total. Reading every remaining non-eqm, non-weapon candidate's own corpus record
(`data/corpus/**/*.json`'s `data.raw_bonus_chains`) found **6**, not 4, units matching exactly
that shape description with NO `WEAPON`/`WEAPONPROF` chain of any kind (so structurally
ineligible for the weapon-shape lane) and NO `equipment_modifier` category (so structurally
ineligible for the eqm-shape lane): `rod_alertness` (INITIATIVE), `scattershot_bracers`
(TOHIT.Ranged), `gunfighter_s_poncho` (formula AC), `staff_of_the_hierophant` (formula AC+SAVE),
`stone_of_good_luck_luckstone` (INITIATIVE+SAVE+SKILL, all formula), `robe_of_vermin`
(INITIATIVE+TOHIT+SAVE+SKILL, all literal). **Saying so loudly per the brief's own instruction:**
this lane's real population is **11, not 9** — the brief's "4 more" undercounts by 2
(`stone_of_good_luck_luckstone`, `robe_of_vermin`). Both are examined and rowed below rather than
left as a gap; if a sibling lane's own population also claims either id, the duplication is a
verdict-identical no-op (both would independently reach the same `unverifiable` disposition for
the same structural reason), not a conflict — flagged here for `finalize` to reconcile against
the merged file, not silently absorbed.

## Real engine fix — dissonance `VAR`+`WEAPON`-formula pair (RED→GREEN)

**Decision (per the brief's own routed instruction — "decide what the comparable magnitude is,
state it, apply it to both consistently"):** the comparable magnitude is the chain's own affected
rolls, `tohit_bonus` and `damage_bonus` (the same `WeaponEnhancementBonus` shape every other
`equipmods.rs` resolver already produces), applied identically to both `..._alt` and `..._main`.

Both records carry the SAME shape: `BONUS:VAR|<Name>|1` (a same-record variable definition) +
`BONUS:WEAPON|DAMAGE,TOHIT|<Name>|TYPE=ENHANCEMENT` (the chain whose magnitude segment NAMES that
variable, uppercase `TYPE=`). Before this cycle: `qualifiers[2].parse::<i16>()` fails closed on
the bare variable name, AND the uppercase `TYPE=ENHANCEMENT` fails the exact-string gate even for
a literal-valued sibling — `compute_equipmods_effect` returns `None` for both records (confirmed:
`AT-33-E5-last67-skill-combat_cycle_receipt.md`'s own next-cycle plan named this exact defect,
un-fixed). After: `resolve_bonus_magnitude` tries a literal `i16` parse first (unchanged behavior
for every existing record), then falls back to `resolve_var_reference` — which reads ONLY this
same record's own `BONUS:VAR|<name>|<n>` chain (never a cross-record variable table, never
character context) — and the `TYPE=` match is `eq_ignore_ascii_case`, widening only that one
qualifier string, never a substring. Both dissonance records now resolve to a real
`tohit_bonus=Some(1), damage_bonus=Some(1)`.

**RED→GREEN, proven by reverting only the production code** (not the tests) against a temp copy
and re-running:
```
$ cargo test --locked --lib dissonance_enhancement_bonus_var
# (production reverted to exact-string TYPE= match + bare i16::parse)
FAILED: left: None, right: Some(WeaponEnhancementBonus { tohit_bonus: Some(1), damage_bonus: Some(1), ... })
# (production restored)
$ cargo test --locked --lib dissonance_enhancement_bonus_var
ok. 1 passed
$ cargo test --locked --lib equipment_effects::
ok. 73 passed; 0 failed (was 70 before this cycle's 3 new tests: the RED→GREEN positive test, a
negative control for an undefined-variable-name chain, and the pre-existing suite unchanged)
```

**No live-oracle round-trip proven this cycle for either dissonance unit** (both `unverifiable`,
reason `no_probe_surface` — see the results file). A real attempt was made: a `.pcg` fixture
(`CUSTOMIZATION:[BASEITEM:Longsword (Base)|DATA:EQMOD=Special Quality ~ Dissonance / Enhancement
Bonus / Main]`, `CAMPAIGN:DSP - Ultimate Psionics` per `campaign_key.py`'s already-fixed KEY
divergence) ran clean against the pinned oracle (`PCGEN_ORACLE_SHA` per
`scripts/pcgen-oracle-pin.env`, character loaded with no SEVERE/equip-failure line — confirmed in
`dissonance_main.run.log`) but exported `WEAPON.0.MAGICHIT=+0`/`WEAPON.0.MAGICDAMAGE=+0`: the
`ITYPE:WeaponEnhancement.Psionic` Special-Quality-category eqmod silently did not apply via the
`CUSTOMIZATION` attachment mechanism. This is the SAME unproven-attachment gap
`AT-33-E5-last67-eqm`'s own receipt already named for the sibling `special_quality_wield_size_*`
Special-Quality-category eqmods ("did not prove a live attachment round-trip ... within its time
budget") — independently re-confirmed here on a different record of the same PCGen mechanism
class, not a new gap this cycle introduced.

## The other 9 units — real engine values checked, none comparable

Every unit's `ours` side is a real, live `compute_equipment_effects` call
(`e5_last39_skill_combat_ours`, output in `last39-skill-combat-work/ours-probe.json`), not
inferred from reading the source alone. Full per-unit reasoning is in the results file; summary:

| unit_id | shape | ours (live-confirmed) | reason |
|---|---|---|---|
| `rod_alertness` | `COMBAT\|INITIATIVE` (literal) | all fields None | `no_resolver` (no engine field for INITIATIVE at all; independently, `AT-33-E5-last67-skill-combat` already found the ORACLE side has no comparable token either) |
| `stone_of_good_luck_luckstone` | `COMBAT\|INITIATIVE`+`SAVE\|ALL`+`SKILL\|...` (all formula) | all fields None | `no_resolver` (no INITIATIVE/SAVE field; SKILL chain's formula magnitude fails `i16::parse`) |
| `gunfighter_s_poncho` | `COMBAT\|AC` (formula) | `armor_class_bonus`=None | `no_resolver` (AC has a resolver, but the formula magnitude fails parse — no `Global_LuckBonus` evaluator) |
| `robe_of_vermin` | `COMBAT\|INITIATIVE`+`TOHIT`+`SAVE\|ALL`+`SKILL\|ALL` (all literal) | `skill_bonus`=("ALL", -2), rest None | `no_comparable_export_token` (the SKILL chain DOES resolve, to a real value, but `"ALL"` is not a real skill name — no single oracle token compares) |
| `scattershot_bracers` | `COMBAT\|TOHIT.Ranged` (literal) | all fields None | `no_resolver` |
| `staff_of_the_hierophant` | `COMBAT\|AC`+`SAVE\|ALL` (both formula) | `to_hit_bonus`=0 (unrelated dimension), rest None | `no_resolver` |
| `companion_stone_far_sight` | `COMBAT\|TOHIT.RANGED` (literal) | all fields None | `no_resolver` |
| `flurry_of_fists` | `WEAPON\|ATTACKS` (cross-record var) + bare `WEAPON\|TOHIT` | `to_hit_bonus`=0 (unrelated), rest None | `no_resolver` (ATTACKS names a variable defined on a SEPARATE ability record — genuine cross-record resolution, out of scope; bare TOHIT deliberately excluded, same established shape as `crossbow_double`/`rod_withering`) |
| `flurry_of_strikes` | `WEAPON\|WEAPONBAB`+`ATTACKS` (cross-record var) + bare `WEAPON\|TOHIT` | same | same |

No new engine surface was built for these 9 — each is a genuine, real absence of a comparable
value (`ours=None` on every intended dimension, confirmed live, not assumed), matching the
established `no_resolver`/`no_comparable_export_token` vocabulary. Building the INITIATIVE/
non-AC-COMBAT/SAVE aggregation fields this population would need is a real, `src/rules_core/`-
scoped, larger-than-one-cycle engine gap (new `ResolvedEquipmentEffect` fields + aggregation
wiring, per `AT-33-E5-shape-combat`'s own next-cycle plan item 4) — not attempted rushed this
cycle, same judgment call that lane made.

## Verdict discipline

Every `ours` value in the results file came from the real `e5_last39_skill_combat_ours` probe
(a live `compute_equipment_effects` call), never hand-typed. `robe_of_vermin`'s real
`skill_bonus=("ALL",-2)` is recorded honestly as `unverifiable`/`no_comparable_export_token`
rather than silently discarded or fabricated into a false `agree`/`disagree` against an
arbitrary single skill. `dissonance_main`/`dissonance_alt`'s real, engine-computed
`tohit_bonus=1, damage_bonus=1` is recorded as `unverifiable`/`no_probe_surface`, not upgraded to
a fabricated `agree` in the absence of a live oracle comparison.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| This lane's stated population | 9 | of 39 remaining | brief |
| This lane's actual population (re-derived) | 11 | of 39 remaining | see re-derivation above |
| Rows written | 11 | of this lane's own 11-unit population | `python3 -c "import json; print(len(json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-skill-combat.oracle-results.json'))['results']))"` → `11` |
| Distinct `unit_id` | 11 | of 11 rows | `python3 -c "import json; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-skill-combat.oracle-results.json'))['results']; print(len(set(x['unit_id'] for x in d)))"` → `11` |
| Verdicts | 0 agree / 0 disagree / 11 unverifiable | of 11 | `python3 -c "import json,collections; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-skill-combat.oracle-results.json'))['results']; print(collections.Counter(x['verdict'] for x in d))"` → `Counter({'unverifiable': 11})` |
| Reasonless `unverifiable` | 0 | of 11 unverifiable rows | `python3 -c "import json; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-skill-combat.oracle-results.json'))['results']; print(sum(1 for x in d if x['verdict']=='unverifiable' and not x.get('reason')))"` → `0` |
| `equipment_effects` test suite | 73 of 73 green (70 pre-existing + 3 new this cycle) | n/a | `cargo test --locked --lib equipment_effects::` |
| Method-change re-run obligation | N/A | this cycle's engine change (`equipmods.rs`) only affects `compute_equipmods_effect`'s `TYPE=Enhancement` match; a corpus-wide re-run to find any OTHER already-judged record whose `TYPE=` casing or `VAR`-referenced magnitude this widens is named as next-cycle scope below, not run this cycle (out of this lane's own 9/11-unit remit; `AT-33-E5-finalize-wave5`'s own method-change re-run covers the AC-isolator, a different function) | see Next-cycle plan |

## Movement, four buckets

- **Closure:** 0 units of this lane's population reach a real `agree`/`disagree` disposition this
  cycle (all 11 are `unverifiable`, each with a populated, real, root-caused reason — not a
  fabricated comparison).
- **Reclassification:** none — no `docs/work-inventory.json` `status` field changed.
- **Reachability:** 2 units (`dissonance_main`, `dissonance_alt`) become engine-reachable for the
  first time this cycle (`ours` moves from `None` to a real computed value) — a real, additive
  engine fix, RED→GREEN, 0 regressions on the other 71 `equipment_effects` tests.
- **Instrument-correction:** 0 this cycle (no prior wrong value corrected; this is first-time
  examination of all 11).

## Status: blocked-escalated

**Not `complete`.** All 11 of this lane's own re-derived population are genuinely examined this
cycle with real, live `(ours, oracle, verdict)` rows and a populated, root-caused reason on every
one (0 reasonless). One real, additive engine fix landed (RED→GREEN, dissonance `VAR`+`WEAPON`
resolution + case-insensitive `TYPE=Enhancement`). This criterion (`AT-33-E5-002`) is not met
by this lane alone — 0 of 11 reached `agree`/`disagree` (all are honest `unverifiable`s, most for
a genuine, confirmed engine-resolver absence, two for a genuine, confirmed oracle-attachment
absence) — and this lane's own 11-unit remainder is only part of the bundle-wide 39. Marking
`complete` over 0 `agree`/`disagree` rows would misstate what this cycle actually closed.

## Notes

- **Judgment call, explicit:** the brief's stated population (9) undercounted this lane's real,
  corpus-derived population by 2 (`stone_of_good_luck_luckstone`, `robe_of_vermin`) — both share
  the exact structural shape (COMBAT/SAVE/SKILL chains, no WEAPON/WEAPONPROF, not an
  `equipment_modifier`) as the other 4 non-psionics COMBAT units the brief did name individually
  by example. Examined rather than left as a silent gap, per the brief's own instruction to say
  so loudly rather than let a unit "fall in no lane's list."
- **`flurry_of_fists`/`flurry_of_strikes`'s `ATTACKS`/`WEAPONBAB` chains are a genuinely different
  shape from the dissonance pair**, even though both are "a `WEAPON` chain naming a variable":
  the dissonance pair's variable is defined on the SAME equipment record
  (`BONUS:VAR|<name>|<n>`); flurry's variable is defined on a SEPARATE ability/class-feature
  record. `resolve_var_reference`'s same-record-only discipline is deliberate (module doc
  comment: "never looks outside this ONE record") — extending it to flurry would require real
  cross-record character-context resolution, a materially larger and riskier surface, not
  attempted this cycle.
- **The dissonance live-oracle attachment gap is independently re-confirmed, not newly caused:**
  `AT-33-E5-last67-eqm`'s own receipt already named this exact PCGen-mechanism-class gap
  (Special-Quality-category eqmod attachment via `CUSTOMIZATION` not proven live) for a sibling
  unit family; this cycle's own live attempt (different record, different base weapon) hit the
  identical silent-no-op outcome, strengthening rather than contradicting that finding.

## Next-cycle plan

1. **Dissonance oracle attachment (2 units):** the `CUSTOMIZATION:[BASEITEM:...|DATA:EQMOD=...]`
   mechanism silently no-ops for `ITYPE:WeaponEnhancement.Psionic` Special-Quality-category
   eqmods on two independent records now (this cycle's dissonance pair, `AT-33-E5-last67-eqm`'s
   wield-size trio) — worth its own dedicated investigation into PCGen's real EQMOD-application
   code path (`code/src/java/pcgen/core/EquipmentModifier.java` / the CUSTOMIZATION parser) rather
   than another blind attachment attempt, since the same syntax DOES work for `Material`-family
   eqmods (`draco`/`dragonhide`/`darkleaf`, proven live in prior waves).
2. **`FlurryOfFistsExtraAttacks`/`MeditantFlurry*`-style cross-record variable resolution (2
   units):** a real, larger engine surface (character-context variable lookup across a class
   feature's own `DEFINE`/`BONUS:VAR` chain) — genuinely out of `resolve_var_reference`'s current
   same-record scope; needs its own design decision, not a one-line widening.
3. **COMBAT non-AC subtoken aggregation (`INITIATIVE`/`TOHIT`/`TOHIT.Ranged`/`SAVE`) — 6 units
   across this lane's own remainder:** new `ResolvedEquipmentEffect` struct fields + aggregation
   wiring, the same item `AT-33-E5-shape-combat`'s own next-cycle plan already named — a
   dedicated cycle, not a remainder-lane addendum.
4. Once rows 17/18's remaining shortfalls (weapon-shape 23, eqm-shape 7, and this lane's own
   engine-gap population) all close or are dispositioned, `AT-33-E6-001` can re-run.

## Test scoping

Ran `cargo test --locked --lib equipment_effects::` (73/73, includes this cycle's 3 new tests —
the narrowest scope covering the touched module and its whole parent). Ran `cargo build --locked
--bin e5_last39_skill_combat_ours` (exits 0, pre-existing warnings only, same set every prior
`AT-33-E5-00x` cycle's receipt names). **Did not** run the root `cargo test` sweep (the
pre-existing, unrelated `equipment_resolver`/`formula_interpreter_corpus_wide` reds named by
`AT-33-E6-001` attempt 6 are outside this lane's write scope and this cycle's diff touches
neither module) or `apps/desktop/src-tauri` (a separate cargo workspace; no file in it touched
this cycle).

```
$ bash scripts/fetch-pcgen-oracle.sh --check
pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6 /home/ubuntu/workspace/repos/pcgen
```
