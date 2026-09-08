# BONUS family — converter mapping design note

Lane: `bonus`. Date: 2026-09-07. Branch `tranche/15`. Companion table: `bonus.json` (41 rows).
Every count below states its denominator. Counts marked **own** were derived this session by
walking `data/corpus/**/*.json` recursively (51,474 records with `raw_tokens`; `LICENSE.json`
and `_parity/` excluded) and joining each record to `docs/work-inventory.json` by
`(book, source basename, source line)` with a `(book, kind, key)` fallback, non-DONE = Completion
Atlas bucket ≠ DONE via the live `scripts/completion_atlas.py::_bucket_of`. Counts marked
**TOKEN-MODEL** are the 2026-08-31 figures from `TOKEN-MODEL.json` (non-DONE, 35 books).

## 1. The family's shape in one page

**Envelope.** Every token is `BONUS:<SUB>|<target[,target…]>|<formula>|<extras…>` where the
extras are, in any order, one `TYPE=<type>[.STACK|.REPLACE]` and zero or more `PRExxx:` /
`!PRExxx:` gate fields. `WEAPONPROF=<name>` carries the weapon in the SUB slot and the property
(`DAMAGE`, `TOHIT`, `DAMAGESIZE`, …) in the target slot. Two spellings leak through the data
(`Type=`, `TYPE.NaturalArmor.STACK`, 5 instances total) and are accepted as `TYPE=`.

**Size (own).** 39 distinct sub-targets, 49,270 instances corpus-wide, on 13,794 records; on the
non-DONE side, **9,360 of 23,315 non-DONE units** (all 37 inventory books) carry at least one
BONUS. TOKEN-MODEL's figure for its narrower population was 21,810 instances on 8,660 units.
The head is as steep as TOKEN-MODEL said: VAR 27,331 · STAT 8,835 · ABILITYPOOL 2,247 ·
SKILL 2,075 · WEAPONPROF 1,678 · SPELLCAST 620 · COMBAT 586 · SPELLKNOWN 582 · SITUATION 549 ·
SAVE 549 = 92.5% of instances; 29 more sub-targets share the remaining 7.5%.

**Three things every sub-target needs, stated once.**

1. **Formula → Expr.** The converter calls `formula_interpreter.rs`'s parser (F1..F9) for the
   AST and maps it by a fixed table: integer → `Const`; `TL` → `Level`; `CL`, bare `classlevel()`,
   `classlevel("APPLIEDAS=NONEPIC")` → `ClassLevel(owning class)`; `CL=X`, `classlevel("X")` →
   `ClassLevel(X)`; `HD` → `HitDice`; `BAB` → `BaseAttack`; `SIZE` → `Size`; `STR…CHA` →
   `AbilityMod` and `STRSCORE…CHASCORE` → `AbilityScore` (PCGen `TermEvaluatorBuilderPCVar.java`:
   bare abbreviation is the modifier, `+SCORE` the score); `+ − × ÷` → `Sum`/`Mul`/`Div`;
   `min`/`max` → nested `Min`/`Max`; `floor(x)` → `x` (Div floors); `%LIST`/`%CHOICE` → `Choice`;
   a DEFINEd identifier → its own producer chain inlined (rule 2); `var("CL=X")+var("BL=X")` →
   `CasterLevel(X)`. Anything else is a **formula-shape refusal**, reported per shape (§4).
2. **Cross-record variables.** A `BONUS:VAR` is a *binding*, never a sheet line. Its value is the
   corpus-wide producer chain `bonus_stack_reader.rs::resolve_producer_chain_corpus_wide` already
   computes (DEFINE base + every addend for that name, each gated by its own PRE), mapped to one
   Expr and inlined wherever the closure consumes the name. Consumer census, corpus-wide by
   instance (denominator 27,331): DESC `%N` 9,938 · SPELLS 1,230 · SAB/ASPECT 1,127 · other
   BONUS/PRE only 11,766 · no corpus consumer 3,946 (877 names). The last set is consumed by
   PCGen system rows that are not corpus records; a fixed **name table** (per name, not per
   record) maps the sheet-feeding ones — `AC_Natural_Armor` 1,574, `DarkvisionRange` 344,
   `CMB_*`/`CMD_*`, `SLA_CL` — to their sheet totals, and the rest (`ARG_RaceBuilderPoints`,
   `TempEvolution*`, `Pool_*`, `DomainTaken`, every `TYPE=Boolean` flag) to Metadata.
   Identifiers DEFINEd nowhere in the corpus (231 names, 1,417 instances family-wide) refuse.
3. **Typed stacking.** Stacking is a *character-level* fold (`BonusManager.java:640-700`,
   pinned `7f81800`): untyped bonuses sum; same-TYPE bonuses take the max unless the type is in
   `system/gameModes/Pathfinder/miscinfo.lst:17` `BONUSSTACKS:Defense.Dodge.Circumstance.Racial.NotRanged.NotFlatFooted`;
   a negative value always adds; `.STACK` always sums; `.REPLACE` replaces the plain-typed value
   (`total = max(plain + stack, replace)`, `BonusManager.java:475-489`). **Inside one record** the
   converter folds tokens sharing `(target, type)` by that rule into one Expr — `Sum([...])` for a
   stacking type, `Max(a, b)` otherwise (1,508 records carry ≥2 tokens on one `(sub, target)`).
   **Across records** the live sheet-total evaluator applies the same rule, so the `SheetRule`
   must carry `target` and `bonus_type` (gap S1). The three-line answer to "one Sum or separate
   lines": one Expr per `(target, type)` per record; separate lines per record on the sheet; one
   folded number per sheet total.

## 2. Row table

| token_type | non-DONE units (of 23,315) | records (of 13,794) | maps_to | confidence |
|---|---:|---:|---|---|
| BONUS:VAR | 6,708 | 10,211 | Number(Expr) — a binding, inlined at its consumer | high |
| BONUS:VAR (TYPE=Boolean flag) | (inside VAR; 1,709 inst.) | — | Text | high |
| BONUS:VAR ([redacted PI] value) | (see PI row) | — | REFUSE | high |
| BONUS:STAT | 426 | 2,306 | Number(Expr) | high |
| BONUS:ABILITYPOOL | 1,105 | 1,647 | Number(Expr) | medium |
| BONUS:SKILL | 763 | 1,571 | Number(Expr) | high |
| BONUS:WEAPONPROF=<name> | 246 | 842 | Number(Expr) / Dice / Text by property | medium |
| BONUS:SPELLCAST | 93 | 93 | Number(Expr) | medium |
| BONUS:COMBAT | 352 | 527 | Number(Expr) | high |
| BONUS:SPELLKNOWN | 362 | 462 | Number(Expr) | high |
| BONUS:SITUATION | 238 | 493 | Number(Expr) + situation words in the label | high |
| BONUS:SAVE | 228 | 317 | Number(Expr) | high |
| BONUS:CASTERLEVEL | 117 | 123 | Number(Expr) | high |
| BONUS:SIZEMOD | 25 | 174 | Number(Expr) | medium |
| BONUS:MOVEADD | 87 | 125 | Number(Expr) | high |
| BONUS:POSTMOVEADD | 3 | 4 | Number(Expr) | high |
| BONUS:MOVEMULT | 0 | 1 | Number(Expr) | medium |
| BONUS:[redacted PI] | 99 | 99 | REFUSE | high |
| BONUS:PCLEVEL | 83 | 122 | Number(Expr) | medium |
| BONUS:SKILLPOOL | 4 | 4 | Number(Expr) | high |
| BONUS:SPECIALTYSPELLKNOWN | 8 | 8 | Number(Expr) | high |
| BONUS:SKILLRANK | 38 | 47 | Number(Expr) | medium |
| BONUS:SLOTS | 1 | 10 | Metadata | high |
| BONUS:MISC | 34 | 42 | Number(Expr) | medium |
| BONUS:HP | 32 | 40 | Number(Expr) | high |
| BONUS:DOMAIN | 43 | 46 | Number(Expr) | medium |
| BONUS:VISION | 19 | 45 | Number(Expr) | high |
| BONUS:DC | 27 | 31 | Number(Expr) | high |
| BONUS:DR | 24 | 26 | Number(Expr) | high |
| BONUS:CONCENTRATION | 8 | 15 | Number(Expr) | high |
| BONUS:SKILLPOINTS | 11 | 14 | Number(Expr) | high |
| BONUS:WIELDCATEGORY | 0 | 9 | Text | high |
| BONUS:RANGEADD | 4 | 4 | Number(Expr) | high |
| BONUS:RANGEMULT | 1 | 1 | Text | high |
| BONUS:WEAPON | 0 | 3 | Number(Expr) | high |
| BONUS:LANGUAGES | 3 | 3 | Number(Expr) | high |
| BONUS:FOLLOWERS | 3 | 3 | Metadata | high |
| BONUS:MONSKILLPTS | 3 | 3 | Metadata | medium |
| BONUS:FEAT | 3 | 3 | Number(Expr) | high |
| BONUS:EQMARMOR | 1 | 1 | Number(Expr) | high |
| BONUS:UDAM | 1 | 1 | Number(Expr) | medium |

Unit columns overlap (a unit may carry several sub-targets); the family total is 9,360 non-DONE
units. The two REFUSE rows are one population: 99 non-DONE units whose BONUS value is the string
`[redacted PI]`.

## 3. Gates on a BONUS token — rule G

9,603 PRE fields ride on BONUS tokens corpus-wide; `pre_tokens.rs` names 7,265 of them (75.7%)
and 2,338 fall in families it does not model. The converter does not evaluate gates at run time;
it folds them at convert time by what settles them:

- **G-build.** Settled by the character's build → fold. `PREVARGTEQ/GT/LT/LTEQ` on a level-shaped
  variable (1,503 of 3,878 PREVAR* gates) → a gated addend inside the Expr (gap E1);
  `PREABILITY`, `PRECLASS`, `PRERACE`, `PRELEVEL`, `PREHD`, `PRETEMPLATE`, `PRESTAT`, `PRESKILL`,
  `PRECSKILL`, `PRETOTALAB`, `PREMULT` → `Applies` (gaps A2, A5, A7 name the missing variants).
- **G-hr.** House-rule flags — `UseFractionalSave` 534, `UseAlternateSaveProgression` 252,
  `UseFractionalBAB` 184, `UseAlternateBABProgression` 183 = 1,153 of 1,834 `PREVAREQ` — select
  one of two twin tokens. The product's rule setting (standard progression) picks the survivor at
  convert time; the twin is dropped. `PRERULE`, `PRECHARACTERTYPE:1,PC` likewise fold to a constant.
- **G-replace.** `PREFACT:1,ABILITIES,<Class>_CF_<Feature>=True` (1,621) and
  `PREVAREQ:<Class>_CF_<Feature>,1` (~170) mean "unless an archetype replaced this feature" →
  gap A1 `Applies::FeatureNotReplaced`.
- **G-situational.** Not settled by the build — `PREEQUIP`/`PREEQUIPPRIMARY`/`PREARMORTYPE` (39),
  `ENCUMBERANCE`/`COUNT[EQTYPE…]` gates (25+), `PREVAREQ:Raging,1` (26), `PREAGESET` (8) → the
  number is still computed; the condition is appended to the label as words ("+8 Acrobatics when
  jumping", "+4 AC dodge, when unarmored"). This is the operator's "a term the character does not
  settle prints as words" applied to the *condition*, not the magnitude.
- **G-fact-gap.** `PREALIGN` (70), `PRESIZE*`/`PREBASESIZE*` (113), `PRESPELLTYPE` (9),
  `PREMOVE` (9), `PREDOMAIN` (1) are build facts the `Applies` vocabulary lacks (A4, A5, A6);
  until added they fold as G-situational words.

## 4. Expr / Applies / SheetRule gaps (consolidated)

| id | gap | family instances | blocking? |
|---|---|---:|---|
| **E1** | `Expr::If(Cond, then, else)` with `Cond = Cmp(Expr, op, Expr) \| And(Cond, Cond)` — `if()` 615 calls, comparison arithmetic (`1+(LVL>=3)+…`), and every `PREVARGTEQ`-gated addend (1,299 on VAR, 439 on ABILITYPOOL, …) | ~2,500 | **No** — exact fallback exists: `(X>=k) ≡ Min(1, Max(0, X−k+1))` for integer `X`,`k`; `if(c,a,b) ≡ b + c·(a−b)`. The variant is a legibility fix, not a blocker. |
| **E2** | `Expr::Ceil`, `Expr::Abs` | 49 + 0 | Yes for those 49 (no fallback for ceil). |
| **E3** | `Expr::SkillRanks(SkillId)` for `skillinfo("TOTALRANK"\|"RANK", s)` | 182 | Yes. |
| **E4** | `Expr::CountHeld(category, key)` for `count("ABILITIES",…)` | 39 | Refuse otherwise. |
| **E5** | `Expr::BonusCasterLevels(ClassId)` for a lone `var("BL=X")`; `var("Name")` ≡ `Name` | 193 `var(` calls | Mostly no (the CL+BL pair and `var(name)` resolve). |
| **E6** | equipment/encumbrance/armor state: `ENCUMBERANCE`, `COUNT[EQTYPE…]`, `ARMORACCHECK`, `SHIELDACCHECK`, `ACCHECK`, `MOVEBASE`, `CR` | ~60 | Refuse or words. |
| **E7** | `Expr::MasterLevel` / `mastervar()` for companion records | 240 gates + 14 calls | Yes for companions. |
| **V1** | identifiers DEFINEd nowhere in the corpus (231 names) | 1,417 | Refuse per name. |
| **S1** | `SheetRule` needs `target` and `bonus_type` so the live evaluator can fold same-type bonuses across records | all typed rows (11,000+) | **Yes** — without it the sheet double-counts two racial +2s. |
| **T1** | a table-cell target `(class, spell_level)` for SPELLCAST/SPELLKNOWN/SPECIALTYSPELLKNOWN/SKILLPOOL | 1,354 | Yes for those rows. |
| **D1** | `SheetValue::Dice` needs `size_steps: Expr` (DAMAGESIZE gated by level; UDAM) | 585 + 1 | Yes when the step is not constant. |
| **Z1** | `Size` is bound from the race table only (§2); SIZEMOD mutates it | 175 | Yes for 174 records. |
| **C1** | `CasterLevel(X)` must accept a PCLEVEL contribution for a class the character has no levels in | 125 | Yes for monsters. |
| **A1–A7** | `Applies::FeatureNotReplaced`, `SkillRanks`/`ClassSkill`, `NotWearing`, `Alignment`, `Size`, `HasVision`, `HitDice(min,max)` | see §3 | A1 (1,791) and A5 (113) matter; the rest are small. |

## 5. Oracle coverage

The export set (`computed-values.txt.ftl`: STAT score/mod, HP, AC total/touch/flat, BAB, CMB,
CMD, CHECK total/base; `charbuild-remainder.txt.ftl`: six STAT scores + every Special Ability's
magnitude-substituted DESC) covers STAT, HP, COMBAT|AC and BASEAB, SAVE, CMB_/CMD_ VARs, and
every VAR a DESC `%N` consumes. It does **not** emit skills, initiative, speed, vision, DR,
spell DCs, caster levels, spells per day/known, or per-weapon attack lines — SKILL alone is 763
non-DONE units with no PCGen total. Add `SKILL.<n>.TOTAL`, `INITIATIVEMOD`, `MOVEMENT.<n>`,
`VISION`, `DR`, and `SPELLLISTCAST`/`SPELLLISTKNOWN` tokens to the harness before AT-35-E2-005,
or those rows verify by sheet line only.

## 6. The three hardest cases, with the records

**1. Witch Hex ~ Ward** — `data/corpus/advanced_players_guide/class_feature/witch_hex/ward.json`
(`apg_abilities_class.lst:898`). Five BONUS:VAR tokens on two targets:
`VAR|WitchHexDC_Ward|WitchHexDC`, `VAR|WitchHexDC_Ward|2|PREABILITY:1,CATEGORY=FEAT,Ability Focus(Witch Hex ~ Ward)`,
`VAR|WitchWardBonus|2`, `VAR|WitchWardBonus|1|PREVARGTEQ:WitchHexAbilityLVL,8`,
`VAR|WitchWardBonus|1|PREVARGTEQ:WitchHexAbilityLVL,16`, consumed by `DESC:DC %1|WitchHexDC_Ward`
and `DESC:…+%1 deflection bonus to AC and a +%2 resistance bonus…`. Everything this family has to
do is in one record: a cross-record chain (`WitchHexDC` is DEFINEd on the class), a feat-gated
addend (→ `Applies` feat, or `If` under E1), two level-stepped addends (E1), and prose
substitution. Sheet line for a level-20 witch, Int 14, without Ability Focus: "Ward: DC 22; +4
deflection to AC, +4 resistance on saves". The naive ungated sum prints +4 at level 1 — the exact
over-count `bonus_stack_reader.rs` was built to stop.

**2. Fighter (class record)** — `data/corpus/core_rulebook/class/fighter.json`
(`cr_classes.lst:139`): `COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE|PREVAREQ:UseAlternateBABProgression,0`
and `SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/2+2|PREVAREQ:UseAlternateSaveProgression,0`.
Three traps at once: `.REPLACE` semantics (this *is* the BAB, not a bonus on it), a house-rule
twin that must be dropped by setting rather than evaluated, and `classlevel("APPLIEDAS=NONEPIC")`
which is `ClassLevel(fighter)` for every character this product builds. 402 SAVE and 192 COMBAT
instances are this shape, on every `class` record. Oracle: `BAB` and `CHECK.n.BASE` cover it
directly.

**3. Abyssal Bloodrager Bloodline ~ Claws** — `acg_abilities_class.lst:607`:
`WEAPONPROF=Claw|DAMAGESIZE|Bloodrager_GenericClaws_SizeBase` and
`WEAPONPROF=Claw|DAMAGESIZE|Bloodrager_GenericClaws_SizeBonus|PREVARGTEQ:Bloodrager_Abyssal_BloodlineLVL,8`.
The sheet line is a *die* ("Claws 1d6" → "1d8" at bloodline level 8), but the token is a
*step count* on a die that lives on a granted NATURALATTACKS token in a sibling record, and the
step is level-gated. Dice-with-a-level-dependent-size is not in `SheetValue::Dice` (gap D1) and
the base die is outside the record (the converter's closure must include grants). 585 DAMAGESIZE
instances share the shape; most are constant steps on monster records, which the converter can
step at convert time.

## 7. Refusals, by mechanism (own, corpus-wide)

| mechanism | instances | records | disposition |
|---|---:|---:|---|
| `[redacted PI]` value | 138 | 99 (all non-DONE) | REFUSE row; PI lane |
| identifier DEFINEd nowhere | 1,417 | ~900 | V1: refuse per name; 231 names, top 20 are ultimate_psionics class levels and the `BloodlinePower*Bonus` family (a PCGen data defect: DEFINEd in no ingested book) |
| `skillinfo(...)` | 182 | ~120 | E3 |
| `var(...)` | 193 | ~130 | E5 resolves the CL/BL pair and `var(name)`; ~30 `COUNT[EQTYPE…]` stay refused (E6) |
| `count(...)` / `charbonusto` / `mastervar` / `cl(` | 39 / 26 / 14 / 1 | ~70 | E4, refuse, E7, refuse |
| decimal literal (`0.5`) | 2 | 2 | refuse |
| malformed envelope (missing target, `TYPE.` spelling) | 5 | 4 | refuse |

Every refusal above is a token *shape*, reported once per shape; none needs a per-record rule.
