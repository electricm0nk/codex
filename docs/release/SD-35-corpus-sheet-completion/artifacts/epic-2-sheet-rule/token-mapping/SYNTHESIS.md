---
canonical: true
owner: synthesis lane (SD-35 Epic 2 token-mapping)
bundle_id: SD-35
date: 2026-09-07
head: tranche/15
---

# Token-mapping synthesis -- schema v2 and the consolidated table

Inputs: the four lane tables (`bonus.json` 41 rows, `formula.json` 79, `prereq.json` 93,
`prose.json` 21) and their four adversarial verdict files. Output: `mapping-table.v1.json`
(249 rows, one per token type / BONUS sub-target; every row is the lane row as corrected by
its judge), `technical-design.md` §1-§2 rewritten to schema v2, and `blockers.md`.

**Tie-breaks.** Wherever a lane and its judge disagreed, this lane re-read the PCGen source in
the pinned checkout (`~/workspace/repos/pcgen`, `7f818006`, read-only) and re-derived the count
from the corpus. Every citation below was opened this session unless marked "judge cites".
Verified this session: `VariableProcessor.java:378-420` (old parser, `total /= valFloat` on a
float) and `:455-471` (JEP path, `parser.addVariable(element, d.doubleValue())`, `return null`
when any symbol is unknown), `:532-560` (`lookupVariable`: DEFINEd variable, then internal term,
then export token); `Description.java:295` (`.intValue()` on the slot value) and `:199`
(`theVariables.get(ind-1)`); `DescLst.java:85-110` (`desc.addVariable(token)` per non-PRE
segment, left to right); `BonusManager.java:99` (`toUpperCase`), `:475-489` (`.REPLACE` =
`max(plain+stack, replace)`), `:640-700` (typed fold: BONUSSTACKS list, `.STACK`, negatives);
`system/gameModes/Pathfinder/miscinfo.lst:17` (`BONUSSTACKS:Defense.Dodge.Circumstance.Racial.NotRanged.NotFlatFooted`);
`PlayerCharacter.java:2090-2140` (`getVariable`: `includeBonus=false` when no held object
DEFINEs the name); `Visibility.java:23-27`; `PCHDTermEvaluator.java:35` and
`CharacterDisplay.java:1084-1087` (`totalHitDice` = `getMonsterLevelCount`);
`TextProperty.java:81-140` (SPROP/SAB positional `%`, `atLeastOneNonZero`);
`VariableKey.java:36` (`CaseInsensitiveMap`); `MovecloneLst.java:66-82` (bare integer is an
addend); `EntityEncoder.java:42-49` (eight entities); `SizeFacet.java:155-170` (SIZEMOD added
to the racial ordinal); `SpellSupportForPCClass.java:160-166, 498-506` (PCLEVEL into caster
level; SPELLCAST per class+level cell); `core_rulebook/_pfs/_.pcc` (`PRECAMPAIGN:1,Pathfinder
Society Core Assumption`); `ce_abilities_race.lst:33-50` (the `Default.MOD` sheet-feeder rows);
`cr_abilities_class.lst:236-259` (`CATEGORY=Class|Fighter.MOD` level-gated grants).

**Denominators.** Units 49,438 (`docs/work-inventory.json`); non-DONE 23,315
(`completion_atlas._bucket_of != DONE`); corpus records 51,474 (recursive, `_parity/` and
`LICENSE.json` excluded, `beastiary` joined as `bestiary`); 48,608 units join a record;
23,312 of the 23,315 non-DONE units join (the 3 that do not are spells at
`botd2_spells_ndl.lst:6`, `uc_spells.lst:121`, `oa_spells.lst:464`). Counts labelled CW are
corpus-wide records; every other count is non-DONE units of 23,315 unless a denominator is
stated inline.

## A. Schema decisions (what changed from `technical-design.md` v1)

The operator's three rules stay the frame: a final number, dice as dice, words for what the
character has not settled (`decisions.md §1`). No PCGen reaches the live side
(`decisions.md §11`). Mapping is per token type, never per record.

| # | Decision | Why (the deciding citation) | Units it unblocks |
|---|---|---|---:|
| A1 | **`Expr::Var(VarId)` + a converter-emitted, TYPED contribution table** (`data/sheet_rules/_vars/<VarId>.json`), folded over HELD rules by the BONUSSTACKS rule; the bonus lane's corpus-wide inlining is dropped. | `PlayerCharacter.java:2090-2140` sums only held objects' active bonuses; `BonusManager.java:640-700` folds same-type contributions to the max (athach `ColdResistanceBonus|10|TYPE=Resistance` + Shadow Creature `15|TYPE=Resistance` = 15, not 25). | 6,699 (BONUS:VAR carriers) + 2,824 (PREVARGTEQ) |
| A2 | **`VarId` is opaque.** `VarId = "v" + 16 hex of SHA-256(upper-cased PCGen name)`; the name -> id map is written by the converter to `scripts/oracle_harness/var_names.json` (tool side, generated, for the oracle only). Nothing PCGen-shaped is in `data/sheet_rules/`. | `decisions.md §11`; `VariableKey.java:36` makes the upper-casing the right canonical form. | -- |
| A3 | **Division is exact; ONE truncation toward zero at the `SheetValue` boundary; `Expr::Floor` and `Expr::Ceil` are explicit.** v1's "Div floors" is removed. | `VariableProcessor.java:461` carries doubles; `Description.java:295` truncates at consumption; per-step flooring diverges on 255 formulas / 452 units (301 non-DONE), judge simulation over all 6,380 distinct formulas. | 452 (301 non-DONE) + 493 `floor()` + 49 `ceil()` + 7 bare-fraction seeds |
| A4 | **`prose` is `Vec<ProseSegment>` with typed slots** (`Slot(Expr)`, `ChoiceName`, `Dice`), per-segment `applies`, a `pick_last` group rule for ASPECT, and `suppress_when_all_zero` for SPROP/SAB. Slots resolve at evaluate time; DESC args are LEFT-TO-RIGHT; a same-row DEFINE is never a Const. | `DescLst.java:105`, `Description.java:199,295`; every DEFINE seed is 0 (15,763 of 15,763); 5,161 arguments would have printed 0 under the prose lane's rule. | 5,149 CW records with `%N` (3,608 non-DONE by the judge's count) |
| A5 | **SPROP/SAB use the positional bare-`%` grammar** and are omitted when every variable is 0. | `TextProperty.java:81-140`. | 99 CW positional SPROP records (259 non-DONE SPROP carriers) |
| A6 | **`Applies` is two-valued (Include/Exclude) plus `Situational{text}`.** The three-valued Words leaf is removed; a gate over an absent character fact evaluates Exclude for a grant (never silently held) and is listed as a fact-gap blocker. | PCGen's testers all return pass/fail; unknown variable = 0 (`VariableProcessor.java:395-403`); 71 AUTOMATIC/VIRTUAL grants on 47 records would be held unearned under Words=include. | 2,824 PREVARGTEQ + 1,860 `!PRE*` gates stop hiding per-record outcomes |
| A7 | **`Expr::HitDice` = racial/monster hit dice** (0 for every PC race); `Level` = total level. | `PCHDTermEvaluator.java:35` -> `CharacterDisplay.java:1084-1087` `getMonsterLevelCount`. | 410 HD-formula units + PREHD 3 |
| A8 | **`CasterLevel(ClassRef)` where `ClassRef = Class(id) \| Holder`; `CasterLevel(X) := ClassLevel(X) + held PCLEVEL contributions to X`.** No `PcLevelBonus` variant. | `SpellSupportForPCClass.java:165-166`. `Holder` binds the class the rule is held through (RANGE keywords, bare CASTERLEVEL). | 227 `charbonusto` + 125 PCLEVEL + 2,581 RANGE |
| A9 | **`SheetRule.also: Vec<(ValueRole, SheetValue)>`** for the second and third numbers on one line (SPELLS uses / CL / DC; ASPECT:CheckCount uses). | `SpellsFacet.java:83-90` builds (times, timeunit, casterlevel, dc). | 1,423 SPELLS + 692 CW CheckCount |
| A10 | **`SheetRule.target` + `bonus_type`** (S1) so sheet totals fold same-type bonuses across held rules with the same BONUSSTACKS rule. | `BonusManager.java:640-700`. | every typed bonus (11,000+ instances) |
| A11 | **`SheetValue::Dice.size_steps: Option<Expr>`** and **`SheetValue::DiceBySize([String; 9])`**; the base die comes through the grant closure (NATURALATTACKS on a granted sibling). | `Equipment.java:5163-5164` (judge cites); 585 DAMAGESIZE instances; 5 list-form UDAM. | 586 + 5 |
| A12 | **New `Expr` variants confirmed necessary:** `Floor`, `Ceil`, `Var`, `BaseSize` (41), `SizeMod` (2), `BaseSave` (2), `SkillRanks`/`SkillTotal` (65 + 143 PRESKILL), `HeldCount` (58 + 21), `ChallengeRating` (37), `Speed(mode)` (39), `HighestSpellLevel` (16), `MasterLevel`/`MasterVar` (254 gates + calls), `CasterLevel(Holder)`. **Not added:** `If`/`Cmp` (exact Min/Max encoding; `&&` -> `Min`), `ChoiceCount` (pool counters are `Var` sums), `PcLevelBonus`, `FeatureNotReplaced` (= `Not(Holds{archetype setter})`), `Toggle` (= `Situational`), `Held(RuleId, Expr)` (form A, superseded by `Var`). | §D | -- |
| A13 | **`print: bool` on `SheetRule` (one owner, from VISIBLE):** print iff YES / EXPORT / QUALIFY-when-held; NO and DISPLAY held silently. Recommendation for the operator, `blockers.md` ruling 1. | `Visibility.java:26` (`DISPLAY_ONLY`: GUI only, not the output sheet). | 2,319 DISPLAY + 3,123 NO instances |
| A14 | **Closure hygiene is part of the converter's input contract:** skip `_pfs/` by path; match `.MOD` on (file kind, CATEGORY, KEY-else-name); resolve bases corpus-wide (recommendation; `blockers.md` ruling 3). | `core_rulebook/_pfs/_.pcc`; `wiring_class.rs:1000-1057` keys by bare name per book (judge reproduced 826 CW leaks and 6,686 unattached cross-book rows). | 826 CW records de-leaked; 4,419 CW targets attached |
| A15 | **PI: a term hit is handled like a declaration** -- omit the field, stamp `provenance.pi`, never refuse. Recommendation for the operator, `blockers.md` ruling 2. | The refused set would be permanent (~900 CW records). | ~900 CW |
| A16 | **Refusals are per token SHAPE, never per record.** Remaining REFUSE rows: `[redacted PI]` values (BONUS 99 units, DEFINE 40, SPELLS 66), `DAMAGESIZE without a base die`, `HITDIE %-step` (8), `SIZE:<formula>` (1), equipment/encumbrance state in a formula (~60, `var("<export token>")` outside CL=/STAT./SKILL.), `count()` outside ABILITIES, malformed formula text, unheld contributor rows (row 29, an ingest remainder), undefined name inside a function or under `* /` (123 uses), `no_source_row` (2 non-DONE), `no_corpus_record` (3), `PRESUBCLASS` until the SUBCLASS reader exists (17). | mapping table `maps_to == REFUSE` | -- |

### Row accounting (of `mapping-table.v1.json`)

| | rows |
|---|---:|
| kept as the lane wrote them | 128 |
| kept, with a judge annotation appended to `rule` | 49 |
| rewritten per the judge's fix | 49 |
| dropped (merged into another family's row, or moved to `blockers.md`) | 8 |
| added (judges' MISSING rows + synthesis merges) | 23 |
| **total** | **249** |

Dropped: `prereq:%CHOICE / %LIST (value markers)` and `prose:%CHOICE / %LIST inside DESC...`
(merged into the synthesis row `%CHOICE / %LIST (all positions)`), `prose:TEMPVALUE` (into
formula row 61), `prose:KEY`, `prose:VISIBLE`, `prose:TYPE`, `prose:QUALIFY` (into the prereq
rows, corrected), `prereq:ABILITYCATEGORY` (not a token; `blockers.md` B3).

**Sum check.** Of 23,315 non-DONE units, 23,312 are reachable by at least one row (every token
head on every joined unit has a row; the residual head list is empty); 3 are listed under
`uncovered` (the unjoined spells). Stated in `mapping-table.v1.json.sum_check`.

## B. Per family: kept / rewritten / dropped / added

### B.1 `bonus` -- BONUS

Lane rows 41; kept 34 (of which 12 carry a judge annotation), rewritten 7, dropped 0, added 0.

**Rewritten, with the judge's proof:**

- `BONUS:VAR` -- axis E,F,C,G. Proof: Three parts of the rule contradict PCGen and one contradicts the formula lane. (1) Typed VAR stacking is ignored: getTotalBonusTo("VAR", name) runs every VAR addend through the same fold as any bonus (BonusManager.java:640-700, pinned PCGen 7f818006 (~/workspace/repos/pcgen, read-only)): same-TYPE addends whose type is not in miscinfo.lst:17 BONUSSTACKS take the MAX, .STACK sums, .REPLACE is max(plain+stack, ... **Fix applied:** Map BONUS:VAR to Expr::Var(VarId) plus a converter-emitted contribution table data/sheet_rules/_vars/<id>.json of {rule_id, expr, bonus_type} (formula row 28 form B); the live evaluator folds the HELD contributions with the S1 BONUSSTACKS rule (untyped/Dodge/Racial/... sum; other same-type max; ...
- `BONUS:ABILITYPOOL` -- axis D,E. Proof: Rule survives (PCGen sums the pool bonus per category and floors it: PlayerCharacter.java:7507-7524, so Div matches). The refusal evidence does not: '97 undefined identifiers (all ultimate_psionics *EnhancementCount names)' -- own census finds 46 undefined names / 322 instances on ABILITYPOOL formulas, and they include adventurers_guide prestige-class levels (BellflowerTillerLVL, WestcrownDevilLVL, ... **Fix applied:** keep the mapping; redo the refusal census case-insensitively against corpus DEFINEs and pinned PCGen DEFINEs; map the pool size to the prereq lane's Choice.count (prereq.md section 1) so the printed sheet shows the picks, which settles this row's open question.
- `BONUS:SKILL` -- axis D. Proof: Rule and PCGen semantics hold (TYPE=ClassSkill: all 105 instances are on `skill` records as `SKILL|X|3|TYPE=ClassSkill|PRECSKILL:1,X|PRESKILL:1,X=1`, the +3 class-skill mechanism, verified). The unit count is off by 8%: own join gives 706 non-DONE units of 23,315 non-DONE units (completion_atlas._bucket_of over docs/work-inventory.json at HEAD); records joined by (book, source basename, source line) with (book, ... **Fix applied:** keep rule; correct units to 706 (or show the join that yields 763).
- `BONUS:WEAPONPROF=<name>` -- axis A,C. Proof: The DAMAGESIZE rule says the base die is read 'from the same record's NATURALATTACKS or the weapon record', but two of the row's three examples have no die on the record: Blood Beak (acg_abilities_arg.lst:56) and Bloodrager claws (acg_abilities_class.lst:607) get the die from a NATURALATTACKS token on a granted sibling. As written the rule leaves those records with no die to step, so the sheet line 'Beak 1d6' cannot ... **Fix applied:** state the rule: base die = the NATURALATTACKS/weapon record reached through the record's grant closure (ABILITY/AUTO edges, the prereq lane's granted_by); Const step -> step the die at convert time; level-gated step -> gap D1; no die reachable -> refuse per shape 'DAMAGESIZE without a base die'.
- `BONUS:SPELLKNOWN` -- axis D. Proof: Rule holds. Units off by 6.6%: own join gives 338 non-DONE units of 23,315 non-DONE units (completion_atlas._bucket_of over docs/work-inventory.json at HEAD); records joined by (book, source basename, source line) with (book, kind, key) fallback over 48,706 data/corpus/**/*.json records carrying data.raw_tokens (_parity/ and LICENSE.json excluded) (row: 362); all 462 SPELLKNOWN records join by source line. Instances ... **Fix applied:** keep rule; correct units to 338.
- `BONUS:HP` -- axis A. Proof: Example 2 prints the wrong number. Under the ordinal the row's own open question states (Fine=0 ... Colossal=8; PlayerCharacter.java:5554 sizeInt() returns SIZEORDER) Large is 5, so ce_abilities_race.lst:1684 `BONUS:HP|CURRENTMAX|max(SIZE-2,0)*10` gives +30 for Large; +20 is Medium. The Bestiary construct bonus-HP table (Small 10, Medium 20, Large 30) agrees, so the formula pins the ordinal. The rule itself is ... **Fix applied:** keep rule; fix example 2 to 'HP +30 (Large construct: ordinal 5-2 = 3, x10)'.
- `BONUS:CONCENTRATION` -- axis D. Proof: Own join gives 6 non-DONE units of 23,315 non-DONE units (completion_atlas._bucket_of over docs/work-inventory.json at HEAD); records joined by (book, source basename, source line) with (book, kind, key) fallback over 48,706 data/corpus/**/*.json records carrying data.raw_tokens (_parity/ and LICENSE.json excluded), the row says 8 (two units, 25%); 16 instances and 15 records reproduce. Rule holds; the two ... **Fix applied:** keep rule; correct units to 6.

### B.2 `formula` -- formula (DEFINE, formula grammar, static tokens)

Lane rows 79; kept 79 (of which 0 carry a judge annotation), rewritten 0, dropped 0, added 3.

**Added (synthesis rows):** `NATURALATTACKS`; `REACHMULT`; `FOLLOWERS`.

**Rewritten, with the judge's proof:**

- `row 3 DEFINESTAT` -- axis A,E. Proof: DEFINESTAT:MINVALUE|STR|15 is not words in PCGen: StatMinValueFacet + StatCalcFacet floor the ability score TOTAL (code/src/java/pcgen/cdom/facet/analysis/StatMinValueFacet.java, StatCalcFacet.java). A resolvable term (the floored score) is left as words; the player writes Str 15, not 'at least 15'. NONSTAT|CON removes the score (constructs), STAT|INT adds one. 42 units / 36 non-DONE reproduced. **Fix applied:** Metadata feeding the ability-score total: MINVALUE|X|n -> the score chassis evaluates Max(AbilityScore(X), Const(n)); MINVALUE with a variable floor -> the fold of rows 27-28 (words only if unresolved); NONSTAT|X -> metadata 'no X score' shown as '--' on the stat line; STAT|X -> metadata. Not Text.
- `row 14 FORMULA:integer literal, + - * / and parentheses, unary minus` -- axis E,A. Proof: The rule 'a/b -> Div (floors); agrees with PCGen for every non-negative operand; negative operands differ in 0 corpus cases' is wrong on both counts. PCGen carries doubles to the consumer and truncates toward zero there (VariableProcessor.java:461 addVariable double; Description.java:295 .intValue(); formula_interpreter.rs module doc point 1 says the same). Judge simulation of all 6,380 distinct corpus formulas (60 ... **Fix applied:** Make Div exact (rational or f64) in the live evaluator and truncate toward zero ONCE at the SheetValue boundary (matching Number.intValue() / (int) casts in Description.java:295 and BonusManager); add explicit Expr::Floor and Expr::Ceil for the corpus floor()/ceil() calls. technical-design.md ...
- `row 15 FORMULA:fractional literal (0.5, 1.5, 1/3, X*3/4)` -- axis E. Proof: `X*1.5 -> Div(Mul(X,3),2) exact` holds only when the product is the whole formula and X >= 0: `STR*1.5` / `1.5*STR` (Str -5: PCGen -7, Div -8), `STR*3/2` (Str -3: -4 vs -5), `max(MesmeristLVL/2,1)+PainfulStareDamFCB/4` (9, 19: 9 vs 8). Same defect as row 14. The 7 bare-fraction seeds (4 distinct formulas found: `0.5`, `1.5`, `if(DEX>0,0.5,1)-if(STR>0,0.5,1)`, `if(DEX>0,1.5,1)-if(STR>0,1.5,1)`) are correctly refused ... **Fix applied:** Make Div exact (rational or f64) in the live evaluator and truncate toward zero ONCE at the SheetValue boundary (matching Number.intValue() / (int) casts in Description.java:295 and BonusManager); add explicit Expr::Floor and Expr::Ceil for the corpus floor()/ceil() calls. technical-design.md ...
- `row 17 FORMULA:floor(x)` -- axis E. Proof: 'floor(x) converts to x because Div already floors' is false for two reasons shown by the simulation: (1) per-step Div floors BEFORE later arithmetic, so `min(floor((Sorcerer_Psychic_BloodlinePower3LVL+3)/6*2),4)` (the row's own example 2) gives 2 at level 7 where PCGen (FloorCommand = Math.floor of the double 3.33) gives 3; same for `max(min(floor((Sorcerer_Serpentine_BloodlinePower1LVL+1)/6*2),4),1)`; (2) once Div ... **Fix applied:** Add Expr::Floor (Math.floor semantics); convert floor(x) -> Floor(x); Div exact per row 14.
- `row 18 FORMULA:ceil(x)` -- axis E. Proof: The caveat 'the rewrite agrees only when X >= 0' is false under a flooring Div: ceil(X/n) = floor((X+n-1)/n) is an identity for every integer X and n > 0 (Str -3: Math.ceil(-1.5) = -1 = floor(-2/2); Str -1: ceil(-0.5) = 0 = floor(0/2)). All 49 corpus calls are ceil(V/2) (judge: 49 `V/2` + 1 `ceil(classlevel(...)/2)`), so under the current design G8 is not needed at all. Under the corrected exact-Div design (row 14) ... **Fix applied:** Drop the negative-operand caveat. If Div stays flooring: no Ceil variant, rewrite is exact. If Div becomes exact (recommended, row 14): add Expr::Ceil alongside Expr::Floor.
- `row 19 FORMULA:if(cond, a, b) and comparison arithmetic ((X>=k)*2)` -- axis D,E. Proof: The conclusion (no If variant needed) survives; the evidence does not. (a) '52 with a bare numeric condition the parser refuses': the judge's parser over every if( call finds ZERO bare-variable conditions; what the current parser refuses is `&&` conjunctions -- 189 instances on 20 units (19 non-DONE), e.g. `if((Sorcerer_CF_BloodlinePower3==0&&Sorcerer_Psychic_BloodlineProgressionLVL>=3),1,0)` ... **Fix applied:** Replace '52 bare-numeric conditions' with '189 && conjunctions on 20 units -> Min(c1,c2)'; state 'operands are integer-valued (census-verified)' as the precondition of the Min/Max encoding; conclusion 'no If variant' keeps.
- `row 28 FORMULA:corpus variable, CROSS-record contributors (the resolution rule)` -- axis D,E. Proof: The RULE stands and the PCGen reading is verified line by line (PlayerCharacter.java:2090-2140: seed via VariableFacet.getVariableValue(isMax) + getTotalBonusTo('VAR') only when found; VariableProcessor.java:532-560 falls to term/export lookups otherwise; BonusManager sums only active bonuses of held objects). Two refutations of the evidence: (1) the 8,518 / 6,646 figure could not be reproduced within 5%: the judge ... **Fix applied:** keep option B; state the extraction scope of the count; add the held-DEFINEr gate to the variable-table evaluator (or an explicit divergence note) so the oracle comparison is exact
- `row 30 FORMULA:identifier DEFINEd nowhere (corpus or pinned PCGen data)` -- axis E,C,D,F. Proof: Three independent breaks. (1) PCGen variable names are CASE-INSENSITIVE (VariableKey.java:36,108 CaseInsensitiveMap; VariableKey.valueOf), so 38 of the row's names / 311 uses ARE defined: HellknightLvl == HellknightLVL DEFINEd on the Hellknight class (iswg_classes.lst, ag_classes.lst), AlchemistLvl 75 uses, MHMercyLVL 33 (uc_abilities_class.lst), ClericLvl 20, FighterLvl 18, ArcanistLvl 17, MesmeristLvl 16, MagusLvl ... **Fix applied:** (1) Build the variable index case-insensitively over corpus + pinned-data DEFINEs (PCGen's own rule). (2) For the residue: Const(0) + defect list ONLY where the use is a bare identifier or a +/- chain; REFUSE per name where the use sits inside a function or under * or / (the oracle is not 0 there). ...
- `row 45 MOVECLONE` -- axis E,A. Proof: MovecloneLst.java:66-140: a third argument starting with `/` divides, `*` multiplies, `+N` adds, and a BARE INTEGER is an addend (`conversion = moveRate -> moveRate + diff`). So `MOVECLONE:Walk,Climb,0` on inner_sea_bestiary:template:Vetala (isb_templates.lst:11) means climb = walk speed (Bestiary 3 vetala: speed 30 ft., climb 30 ft.), not 'climb 0 ft' as the row's sheet line says; advanced_race_guide `Augmented ... **Fix applied:** `/k` -> Div(Speed(m), k); `*k` -> Mul (decimal via exact Div); `+k` or bare `k` -> Sum([Speed(m), Const(k)]). Gap G10 Speed(mode) stands.
- `row 69 VISION` -- axis C. Proof: 'Const per entry' gives the wrong answer on 6 records whose range is a VARIABLE, not a number: `Darkvision (UMR_DarkvisionEx_Range)` / `(UMR_DarkvisionSu_Range)` (bestiary_4 universal monster rules), `See in Darkness (UMR_SeeInDarkness_Range)|PREVARGTEQ:...` (bestiary_2), `See in Darkness (Sorcerer_InfernalPowerOfThePit_VisionDistance)` (Infernal Bloodline ~ Power of the Pit, two records), `Blindsense [When Immersed ... **Fix applied:** range = Number(Expr): Const(n) for a numeric range, the rows 27-28 fold for a variable range; label from the name before the parenthesis.

### B.3 `prereq` -- prereq (PRE*, ownership, choices)

Lane rows 93; kept 67 (of which 17 carry a judge annotation), rewritten 24, dropped 2, added 9.

**Dropped:** `%CHOICE / %LIST (value markers)`; `ABILITYCATEGORY (pool declarations)`.

**Added (synthesis rows):** `MONSTERCLASS`; `MAXLEVEL`; `DEITYWEAP`; `ALIGN`; `COMPANIONLIST`; `USEUNTRAINED`; `ROLE`; `NAMEOPT / ITYPE / REPLACES / FORMATCAT / ASSIGNTOALL`; `REGION / REMOVABLE / VARIANTS / INFO / EXCLUSIVE / ALLOWBASECLASS / EXCLASS / WEAPONBONUS / ACHECK / CHANGEPROF / ADDSPELLLEVEL`.

**Rewritten, with the judge's proof:**

- `PREVARGTEQ` -- axis E,C,D,F. Proof: Four independent defects. (1) Semantics: PCGen's value of a variable is DEFINE seed + the SUM of every BONUS:VAR contribution on objects the character holds (PlayerCharacter.java:2090-2140 getVariable -> getTotalBonusTo("VAR")). 543 of the 1,244 top-level level-alias instances name a variable with more than one distinct setter formula -- BloodlineCasterLVL (258) is set by SorcererLVL, MagusLVL, DragonDiscipleLVL and ... **Fix applied:** resolve(var) := Expr::Var(id) over the converter-emitted contribution table (formula lane G1 form B), built from the pinned-tree closure (build_mod_index's walk), never from corpus raw_tokens alone; collapse to ClassLevel(X) only when the name has exactly one setter formula and it is ...
- `PREVARGT` -- axis E. Proof: Same resolve() as PREVARGTEQ, so it inherits defects (1)-(3) above. AstralSuitPrereqLVL (56) has a single setter (AstralSuitLVL) and would collapse correctly; DragonAgeCategory has 13 constant setters (1..12) across templates -- a Var sum, which the row's Holds{setter} shape cannot express (it is a number, not a flag). Example sheet_lines are consistent. **Fix applied:** same as PREVARGTEQ; DragonAgeCategory-style enumerations -> Compare{Var(id) > n}
- `PREVARLT` -- axis G,E. Proof: 506 / 402 non-DONE hold. The 'pool counters stay Words until Expr::ChoiceCount exists' rule is unnecessary: TempEvolutionTaken (120) and AnimalCompanionSkill (90) are ordinary cross-record variables whose contributors are `BONUS:VAR|TempEvolutionTaken|1` addends on the held evolution rules (PlayerCharacter.getVariable sums them); with the formula lane's Var table they are Compare{Var(id) < rhs}, no new variant. Only ... **Fix applied:** pool counters -> Compare{Var(id), Lt, rhs} via the contribution table; drop the ChoiceCount gap; keep HeldCount for count(); PREVARLT:X,1+(EidolonLVL/4) rhs parses as stated
- `PREVARLTEQ` -- axis B,G. Proof: All three sheet_line examples print the PCGen token body verbatim ('RetrainHPLimit,999', 'EvoImpNatArmCount,MasterLevel/5', 'EvoAbiIncDex,MasterLevel/6') -- a PCGen variable name on the sheet, which decisions.md §11 forbids in any field but provenance. The 'Words until Expr::ChoiceCount' reasoning is the PREVARLT defect again; RetrainHPLimit/EvoImpNatArmCount are Var sums, and MasterLevel is the companion-master gap. **Fix applied:** Words text is generated per token type from a template ("limited by the number already taken"), never the token body; lhs -> Var(id); MasterLevel -> companion-master gap
- `PRESUBCLASS` -- axis C,E. Proof: PreSubClassTester.java:53-55 tests display.getSubClassName(class) -- the Wizard's SUBCLASS object (Abjurer..Transmuter, declared by SUBCLASS: lines in cr_classes.lst). The corpus has no subclass records (0 SUBCLASS tokens; no record keyed Illusionist/Diviner/Evoker) and the row names no join from the subclass name to 'the class_feature record the school choice grants' -- that mapping (Illusionist -> the Illusion ... **Fix applied:** tool side: read the SUBCLASS lines (SUBCLASS:<name> ... CHOICE:SCHOOL|<School>) from the class .lst and emit Holds{Rule(arcane-school record for <School>)}; Words until that reader exists
- `PREHD` -- axis E,A. Proof: PreHDTester.java:41 compares display.totalHitDice(), and CharacterDisplay.java:1084-1087 defines totalHitDice() as levelFacet.getMonsterLevelCount -- racial (monster-class) hit dice only. A human Fighter 3 has 0 such HD, so PREHD:MAX=0 is TRUE for fixture F1 and the row's sheet_line ('F1 has 3 HD -> excluded') is wrong; the Skeleton HitDice template exists precisely for class-levelled creatures with no racial HD. ... **Fix applied:** Compare{RacialHitDice op n} where RacialHitDice = monster-class levels (0 for every PC race); define Expr::HitDice that way in §1 or add the variant; correct all three sheet_lines
- `PREALIGN` -- axis D,E. Proof: 141 / 117 hold, but the row says the `Deity` code has 0 instances; one corpus PREALIGN uses it. PreAlignTester.java:113-116 resolves `Deity` to the character's deity's alignment. Small, but the rule as written would turn that clause into an unknown alignment name. **Fix applied:** add: PREALIGN:Deity -> Holds{AlignmentMatchesDeity}; keep the rest
- `PREDEITY` -- axis C,D. Proof: 41 of the 50 instances are `PREDEITY:[redacted PI]` -- the deity name is withheld, so the named-deity rule can apply to at most 9 instances (Y 2, Achaekek 2, Mazmezz, Shivaska, one PANTHEON.<name> form); the row states no disposition for the 41 and its 'Deific/Demonic Obedience feats' population is the redacted set. PreDeityTester.java:38-62 also has a PANTHEON. branch the rule lacks. Y/N is PreHasDeityTester ... **Fix applied:** PREDEITY:[redacted PI] -> Words("requirement withheld") counted under PI residue (Decision 28), not under this row; add PANTHEON.<name> -> Holds{DeityInPantheon(name)}
- `PREWEAPONPROF` -- axis C. Proof: 12 / 11 hold, but the corpus forms include `PREWEAPONPROF:1,DEITYWEAPON` (the deity's favored weapon -- a deity fact, not a weapon id or group) and five `TYPE.Natural` items (a weapon TYPE facet on natural attacks, not a proficiency group); the rule's Weapon(id)|WeaponGroup(tag) vocabulary covers neither. **Fix applied:** add DEITYWEAPON -> Holds{Proficiency(DeityFavoredWeapon)} (Words until deity is a fact); TYPE.<x> -> Holds{Proficiency(WeaponTag(x))} over proficiencies' weapon tags
- `PREARMORTYPE` -- axis B,F. Proof: The rule says Words("while wearing heavy armor") but the example sheet_line prints the token body '1,TYPE.Heavy' -- PCGen token text in a sheet field. The bonus lane (G-situational) places the same condition as words on the rule's line; two placements for one token. **Fix applied:** Words text from a per-type template ("while wearing heavy armor"), never the token body; align with bonus G-situational (condition on the line, Applies two-valued)
- `PREEQUIP` -- axis B. Proof: Both sheet_lines print the token body ('1,Brooch (Folding Plate)', '1,Belt of Giant Strength') rather than the rule's own Words("while <item> is equipped"). **Fix applied:** generate the words from the template; never emit the token body
- `PREDR` -- axis B. Proof: sheet_line prints '1,ANY=1' -- token body on the sheet. **Fix applied:** Words("requires damage reduction") from the template
- `PREHANDSGTEQ` -- axis B. Proof: sheet_line prints '3' from the token; the rule's words are right, the example is not. **Fix applied:** Words("requires three or more hands")
- `PREREACHGTEQ` -- axis B. Proof: sheet_line prints '10' from the token body. **Fix applied:** Words("requires reach 10 ft. or more")
- `PRERULE` -- axis D. Proof: 7 instances, but only 4 are DISPLAYSKILLUSE; 3 are `PRERULE:1,DAMAGE_VW` (a variable-weapon-damage house rule on inner_sea_bestiary). The Metadata/Always disposition still holds for both (Globals.checkRule reads a game-mode toggle), the evidence does not. **Fix applied:** state both rule keys; keep Always
- `PRE` -- axis E,F. Proof: The single PRE:.CLEAR sits on a record whose source is advanced_class_guide/_pfs/pfs_acg_abilities_class.lst:115 -- a Pathfinder Society overlay file that PCGen loads only through `_pfs/_.pcc` (`ABILITY:pfs_acg_abilities_class.lst|PRECAMPAIGN:1,INCLUDES=Advanced Class Guide`), never as part of the book; no book .pcc references any _pfs/ file. The product does not load PFS, so the row's one instance is not corpus ... **Fix applied:** exclude _pfs/ rows from the closure; keep the PRE:.CLEAR fold rule (correct in itself) with 0 instances
- `feat.prerequisites (token-less feat records)` -- axis D. Proof: Counts hold exactly (1,411 / 1,079 / 2,270 / 351), but the open question and the second example claim token-less feat records carry no source line: they do -- `source.path` and `source.line` are on every such record (Extra Touch Treatment: oa_feats.lst:29). The provenance problem the row raises does not exist. **Fix applied:** keep the rule; take provenance from `source`; drop the open question
- `VISIBLE` -- axis E,A. Proof: 7,505 / 4,748 hold, but the rule folds only VISIBLE:NO to not-printed. PCGen's Visibility.java:23-27 defines DISPLAY as 'shows up in the GUI, but not on the output sheet' and EXPORT as the reverse; the output-sheet view is DEFAULT or OUTPUT_ONLY (line 60). Our sheet is the output sheet, so the 2,319 VISIBLE:DISPLAY rules (selector and helper records) would print here when PCGen's own sheet omits them; QUALIFY ... **Fix applied:** printed = YES | EXPORT | QUALIFY(when held); NO and DISPLAY are held silently
- `CSKILL` -- axis D. Proof: 842 instances / 836 records hold, but non-DONE units re-derive to 425 against the row's 495 (14% under, of 495); the rule is fine. **Fix applied:** re-state the unit count with its join
- `CLASSES` -- axis C,E. Proof: 5,199 / 1,244 hold, and the comma-before-level open question is answered yes (spell ClassesToken.java:170 tokenizes the comma list inside a pipe group: every listed class at that level; 1,624 instances). But 20 instances sit on `skill` records, where the Skill CLASSES token (plugin/lsttokens/skill/ClassesToken.java:66-94, with ALL and !Class forms) declares which classes have the skill as a class skill -- a ... **Fix applied:** kind=skill: CLASSES -> FactGrant{ClassSkill(skill)} on each named class (ALL/!X forms); spell/power/class_feature keep the grant rule
- `SERVESAS` -- axis C. Proof: 318 / 136 hold, but 7 instances are not ABILITY= forms: SERVESAS:CLASS|Cavalier (2), CLASS|Rogue (2), RACE|Eidolon, RACE|Elf|Human, RACE|Human|Orc -- the holder counts as that CLASS for PRECLASS tests or that RACE for PRERACE tests; CountsAs{Rule(id)} cannot express either. **Fix applied:** add CountsAs{Class(id)} consumed by PRECLASS Compare and CountsAs{Race(id)} consumed by PRERACE Holds
- `REMOVE` -- axis C. Proof: 10 hold, but two are `REMOVE:FEAT|2|<list>` -- PCGen's count form, where the player chooses which 2 of the listed feats to give up (a choice), not an unconditional revoke of the whole list. **Fix applied:** REMOVE:FEAT|<n>|<list> -> Choice{count: Const(n), from: Rules(list)} whose picks are Revokes; the count-less form stays Revokes
- `%CHOICE / %LIST (value markers)` -- axis D. Proof: Host counts re-derived: ABILITY 202, TEMPBONUS 184 (row: 153, 20% off of 153), BONUS 173, SPROP 100, DESC 86, COST 37, AUTO 33, ASPECT 19, TEMPLATE 5, SR 2; the binding rule itself is right and Expr::Choice exists. **Fix applied:** correct the TEMPBONUS figure; keep the rule
- `class LEVEL grants (CLASS .lst level lines)` -- axis E,D,C. Proof: The mapping (Grant{by: Class, at_level, when}) is right; the evidence and the blocker are not. (1) The quoted example does not exist: cr_classes.lst's only Fighter level line is `1\tABILITY:Class|AUTOMATIC|Fighter`; Fighter's feature grants are `CATEGORY=Class|Fighter.MOD` rows in cr_abilities_class.lst:257-262, gated `PREVAREQ:Fighter_CF_<X>,0|PREVARGTEQ:Fighter_CFP_Level,<lvl>` -- i.e. the level is a PREVARGTEQ ... **Fix applied:** keep the Grant mapping; source = the class record's closure (CATEGORY=Class|X.MOD rows, already read) + ClassFeatureBlock.level_lines read tool-side at convert time for every level-line token head (ABILITY, BONUS, ADD, DOMAIN, TEMPLATE); reuse data/class_feature_grants/ as the cross-check; delete ...
- `.MOD / .COPY closure rows` -- axis E,F. Proof: 874 mod_only units hold (867 non-DONE). But build_mod_index (wiring_class.rs:1000-1057) walks every .lst under a book including _pfs/ and keys by bare name with CATEGORY= stripped, so Pathfinder Society overlay rows (loaded by PCGen only via _pfs/_.pcc under PRECAMPAIGN, verified) and cross-category same-name rows leak into the closure -- the prose lane's case 1 documents 215 corpus records carrying PFS-overlay ... **Fix applied:** skip _pfs/ by path; match .MOD on (category, name); resolve the base record across books when the (book, name) key has no base
- `ABILITYCATEGORY (pool declarations)` -- axis G,C,D. Proof: The input gap is real: 0 corpus tokens; the pinned checkout holds 208 *_abilitycategories.lst files with 2,831 ABILITYCATEGORY rows, and nothing in src/pcgen_import reads them (corpus_traps.rs:196 lists the key as a trap). But (a) this is not a token-mapping REFUSE in the prereq family -- no prereq token maps to it; it is a converter-input dependency, and REFUSE per the lane brief is reserved for per-record ... **Fix applied:** drop the row; file under blockers_confirmed as 'tool-side ABILITYCATEGORY reader (CATEGORY, TYPE, PRE, PLURAL, POOL) needed for BONUS:ABILITYPOOL pools -- 1,013 names, 1,093 non-DONE units (bonus family)'; CHOOSE:ABILITYSELECTION|<parent>|TYPE=... converts today

### B.4 `prose` -- prose (DESC and display tokens)

Lane rows 21; kept 7 (of which 4 carry a judge annotation), rewritten 8, dropped 6, added 10.

**Dropped:** `TEMPVALUE`; `%CHOICE / %LIST inside DESC, BENEFIT, ASPECT, SPROP`; `KEY`; `VISIBLE`; `TYPE (display facet)`; `QUALIFY`.

**Added (synthesis rows):** `SCHOOL`; `SUBSCHOOL`; `DESCRIPTOR`; `DURATION`; `CASTTIME`; `TARGETAREA`; `COMPS`; `SAVEINFO`; `SPELLRES`; `QUALITY`.

**Rewritten, with the judge's proof:**

- `DESC` -- axis A,B,E. Proof: Counts hold (33,841 instances / 28,065 records CW; 4,019 multi-DESC; 636 DESC+BENEFIT). The RULE fails on four points. (1) Rule 4 'a bare name with a same-row DEFINE:X|<int> -> Const (3,936 of 5,220)': every DEFINE in the corpus is |0 (formula.md section 1: all 15,763 readable DEFINE values are 0); re-derived: 3,939 DESC bare-name args have a same-row DEFINE, of which only 312 also have a single ungated literal ... **Fix applied:** Arguments: every non-PRE segment after the first is an argument, left to right, %N = argument N; an integer literal -> Const; ANY other argument (bare name, Name+-int, formula) -> the formula lane's Expr through its Var table (formula G1) -- never Const from a DEFINE. Escapes: %% -> %; a lone % ...
- `DESC:.CLEAR` -- axis C,D,E. Proof: The leak is REAL and reproduced on three records: (1) core_rulebook/class_feature/sorcerer_bloodline_feat/scribe_scroll.json (cr_abilities_class.lst:2071, CATEGORY:Internal, KEY 'Sorcerer Bloodline Feat ~ Scribe Scroll', name 'Scribe Scroll') carries the DESC:.CLEAR/BENEFIT:.CLEAR/TYPE:PFSNotLegal/!PRECHARACTERTYPE tokens of core_rulebook/_pfs/pfs_cr_feats.lst:20 'CATEGORY=FEAT|Scribe Scroll.MOD' in its raw_tokens, ... **Fix applied:** Mod index: skip every .lst under a _pfs/ directory; key rows by (file kind from the .pcc line type, CATEGORY= for abilities, KEY-else-name); look up by the record's KEY (name only when it has no KEY). Escalate: cross-book .MOD rows (6,686) need a design ruling (per-book SheetRule vs corpus-wide ...
- `BENEFIT` -- axis E,C. Proof: Counts hold (672 / 638 CW; 111 %N tokens on 107 records; 9 BENEFIT:.CLEAR records; 636 DESC+BENEFIT). The rule inherits every DESC argument defect (same-row DEFINE -> Const; from-the-right split), e.g. adventurers_guide/feat_generic Falcon's Cry (ag_feats.lst:33) '...%1 times per day.|1+TL/3' is fine, but any BENEFIT whose argument is a DEFINEd name prints 0. Second: 24 CW records carry BENEFIT:[redacted PI]; under ... **Fix applied:** Same argument rule as the corrected DESC row. A redacted BENEFIT (marker or term hit) is omitted and stamped in provenance.pi.redacted exactly like a declared DESC redaction; never refused. Keep DESC-then-BENEFIT concatenation as a product choice (PCGen exports FEAT.x.DESC and FEAT.x.BENEFIT ...
- `ASPECT:<display sub-key>` -- axis E,B,A. Proof: Counts hold (6,420 instances / 4,419 records CW; Ability Benefit 1,316; SaveBonus 376; CombatBonus 330; SkillBonus 231; 320 ASPECT-only-prose records; 118 redacted sub-keys). Semantics do not: (1) pcgen/cdom/helper/Aspect.java printAspect prints ONE aspect per sub-key -- lastPassingAspect returns the LAST aspect whose PRE passes -- never one line per aspect; 6 CW records carry >=2 same-sub-key display aspects with ... **Fix applied:** Per sub-key, emit the LAST segment whose gate passes (convert-time when decidable, else the per-segment Applies picks the last passing at evaluate time). Replace the allowlist with an explicit 71-entry sub-key table; route *CheckCount/*CheckType to the uses row; refuse malformed sub-keys carrying ...
- `ASPECT:CheckCount / ASPECT:CheckType` -- axis D,E. Proof: Counts hold (CheckCount 694 instances on 692 records; CheckType 696 on 693). Evidence fails the brief: two of the three examples are placeholders ('second and third real records to be quoted by the formula lane'), and no formula-lane row covers CheckCount (bonus.json 0 mentions, formula.json 1 mention). Semantics: the %1 variable is a cross-record variable in most cases (e.g. Sorcerer_Psychic_BloodlinePower1Times is ... **Fix applied:** Keep Number(Expr) uses/period. Quote three real records. Value = formula lane's Expr for the named variable through the Var table; label the gap dependency (formula G1) explicitly. Add a converter check for glued tokens (a token value containing '\t<TOKEN>:' or ' <TOKEN>:' where TOKEN is a known ...
- `ASPECT:NAME` -- axis B,D. Proof: Count wrong: 103 is the INSTANCE count; only 52 CW records carry ASPECT:NAME (many carry two). The fallback rule 'until the gap is closed the converter uses the record name as label and puts the template into prose as a labelled line' ships 'Str +%1' -- a %N marker -- into prose: a PCGen substitution marker in a SheetRule field other than provenance (axis B). The second example has no source line; the shape is real ... **Fix applied:** label = record name (or OUTPUTNAME); the NAME template becomes a prose segment with the argument as a slot under G1 (ProseSegment) -- no separate label-slot variant is needed; never emit an unsubstituted template. Correct the count to 103 instances / 52 records.
- `ASPECT:<structural sub-key>` -- axis D. Proof: Rule holds (bookkeeping, not player text; SourceBook -> provenance). Count is loose: re-derived CW ChildAbility 833 instances/773 records, Archetype Base Class 483/470, SourceBook 365/364, MasterAbility 116/116, Bloodline 57/43, StatBlockName 27/27 = 1,881 instances on 1,793 record-mentions (overlapping), not '~1,500 records' (16% off). **Fix applied:** keep; state 1,881 instances on up to 1,793 records CW.
- `SAB` -- axis E. Proof: Both records verified (b3_races_companion.lst:15, mc_races_companion.lst:15; 'SAB:+4 on saves vs. disease.'; 0 non-DONE). The forward rule is wrong: 'if a future book brings SAB:...|<formula> it follows the DESC argument rule' -- PCGen's SpecialAbility extends TextProperty, whose getParsedText (pcgen/core/TextProperty.java) fills each bare % positionally from the |-separated variables and suppresses the whole line ... **Fix applied:** keep Text for the two records; state that SAB shares SPROP's positional-% grammar, not DESC's.
- `SPROP` -- axis A,E. Proof: Counts hold (3,816 / 3,662 CW; 2,491 SPROP-only records; 52 %CHOICE; 48 %LIST). The grammar is wrong. pcgen/core/TextProperty.java getParsedText (SpecialProperty extends TextProperty): 'SPROP:<text>|<v1>|<v2>...' -- each BARE % in the text is replaced, left to right, by the next variable's intValue(); if every variable evaluates to 0 the entire line is suppressed; %CHOICE is replaced by the equipment modifier's ... **Fix applied:** SPROP/SAB grammar: split on |, pop PRE gates, the remaining tail segments are positional variables; each bare % in the text is a slot for the next variable (formula lane Expr, Const for literals); the line is omitted when every slot resolves to 0 (PCGen) -- state that suppression as a rule; %CHOICE ...
- `%CHOICE / %LIST inside DESC, BENEFIT, ASPECT, SPROP` -- axis E. Proof: Counts hold (DESC: `%CHOICE` 29 + `%LIST` 57; ASPECT 4 + 15; SPROP 52 + 48; BENEFIT 0; 2 CW records use %CHOICE with no CHOOSE token: advanced_players_guide/ability/gifted_adept.json and trait_generic/trait_gifted_adept.json). Semantics are per token family, not one rule: Description.java -- %CHOICE = the first association's export string, %LIST = the SORTED associations joined with ', ' or with ' and ' when exactly two, ... **Fix applied:** State the three substitution rules per family; %LIST join = sorted, ', ' / ' and '; unmade -> words (deviation, choice-lane table); record the 2 CHOOSE-less records as an upstream defect list, not a refusal.
- `OUTPUTNAME` -- axis E,B. Proof: Counts hold (2,124 / 2,123 CW; 684 [NAME]; 13 redacted). pcgen/core/analysis/OutputNameFormatting.java: [NAME] expands to getPreFormatedOutputName(displayName) = the text inside the parentheses, split on '/' and REVERSED, space-joined; '[BASE]' = the name before the parenthesis; and an OUTPUTNAME with '|' substitutes each '%' positionally from the variables. 5 CW records hit the '/' rule ... **Fix applied:** Add the '/'-reversal and [BASE] rules; state NAMEISPI precedence over OUTPUTNAME (neutral label wins); note [NAME] on a name without parentheses = the whole name (PCGen returns displayName).
- `NAMEISPI / DESCISPI (and the corpus license/pi_field/pi_marker stamps)` -- axis C,F. Proof: Flags pi_screening.rs and its consumers key on, enumerated: NAMEISPI and DESCISPI declarations (value YES, case-insensitive, declared_product_identity); license (shape_b_v1::License has THREE states OGL | PI | PI-REDACTED); pi_field (comma-separated list of redacted field names); pi_marker ('redacted'); the field value REDACTED_PI_MARKER '[redacted PI]'; codex_generated_name (bool, the audit's marker that a PI name ... **Fix applied:** A term hit is handled exactly like a declaration: omit that field from prose/label (label -> codex-neutral name), stamp provenance.pi.redacted, never refuse. Re-key the shipping audit to provenance.pi and prose (rule: description_declared or 'description' in redacted => the DESC family contributed ...
- `VISIBLE` -- axis E,F. Proof: Counts hold (7,505 / 7,005 CW; NO 3,123; DISPLAY 2,319; QUALIFY 1,288; YES 471; EXPORT 303; DISPLAY|READONLY 1). Semantics: pcgen/util/enumeration/Visibility.java -- DISPLAY_ONLY 'shows up in the GUI, but not on the output sheet'; OUTPUT_ONLY 'on the output sheet, but not in the GUI'; HIDDEN neither. The sheet is PCGen's output sheet, so 'print = VISIBLE != NO' prints 2,319 DISPLAY rows PCGen's sheet omits; the ... **Fix applied:** print iff VISIBLE in {YES(default), EXPORT, QUALIFY-when-held}; DISPLAY and NO -> held, not printed (print: false); one owner for the flag (prereq's visible).
- `QUALIFY` -- axis F,G. Proof: 7 instances / 5 records CW -- exact; examples verified. Two lanes model it in opposite directions: prose puts a waiver on the TARGET rule's Applies (gap G9 'Applies needs an or-holds-rule form'); prereq.md section 1 puts QUALIFY in the HOLDER's grants: Vec<Effect> (with SERVESAS, REMOVE), which is PCGen's own shape (pcgen/core/prereq/PrereqHandler.java lines 77 and 114 check QUALIFY on the character before ... **Fix applied:** drop the prose row's Applies mapping and G9; prereq owns QUALIFY as a holder-side Effect.

## C. Cross-family conflicts, each resolved to ONE rule

Each entry names the rows that disagreed, the rule that wins, and the citation that decides it.
Rows in the table carry the entry id in `resolved_conflict`.

**C1 -- Names DEFINEd nowhere (bonus V1 "refuse per name, 231 names" vs formula row 30
"Const(0), 618 units" vs prereq "falls to Words").** Rule: one CASE-INSENSITIVE variable index
over corpus DEFINEs plus every DEFINE in the pinned data (`VariableKey.java:36`
`CaseInsensitiveMap`; `BonusManager.java:99` upper-cases), then a three-way split per NAME:
(a) DEFINEd on a never-ingested pinned row (130 names / 778 instances / 434 non-DONE units in
the bonus family; `up_classes.lst`, `ag_classes.lst`, `ce_abilities.lst`, `sos_abilities.lst`,
companion mods) -> ingest remainder per FILE, neither refused nor 0; (b) a case variant of a
DEFINEd name (21 names / 67 units; `HellknightLvl`, `FighterLvl`, `MONKLVL`, `MASTERLEVEL`) ->
resolves through the index; (c) DEFINEd nowhere (68 names / 134 units; the
`BloodlinePower{1,3,9,15,20}LVLBonus` / `EldrtichHeritage*` family, a PCGen data defect) ->
`Const(0)` plus `data/sheet_rules/_defects/undefined-variables.json` ONLY where the use is a
bare identifier or a `+/-` chain (PCGen's broken parser yields 0 there,
`VariableProcessor.java:395-403`), and REFUSE per name where the use sits inside a function or
under `*` or `/` (123 uses) because `processJepFormula` returns null for the whole formula
(`:463-471`) and the fallback parser has no precedence. Prereq gates on set (c) names are
DECIDED (`PREVARGTEQ:X,1` is false), never Words. **Deciding citation:** `VariableProcessor.java:395-403, 463-471`; `VariableKey.java:36`.

**C2 -- Cross-record variables (bonus rule 2 "inline the corpus-wide producer chain" vs formula
row 28 form B vs prereq "level alias -> ClassLevel(C)").** Rule: `Expr::Var(VarId)` at every
consumer; the converter emits `_vars/<VarId>.json` = `{declared_by, contributions:[{rule_id,
expr, bonus_type, when}]}`; the evaluator folds the HELD contributions exactly as
`BonusManager.java:640-700`: untyped, `.STACK`, negative, and BONUSSTACKS-listed types
(`miscinfo.lst:17`) sum; other same-type contributions take the max; `.REPLACE` =
`max(plain+stack, replace)` (`:475-489`); a name no held rule declares yields 0 with its
contributions dropped (`PlayerCharacter.java:2124-2136`). Inlining sums records the character
does not hold (`BloodlineLVL`: 5 corpus setters + Robe of Arcane Heritage; a sorcerer holds
one). Collapsing to `ClassLevel(C)` is allowed only when the name has exactly one setter and it
is `CL`/`classlevel("X")`/`TL` (543 of 1,244 level-alias gates have several setters:
`BloodlineCasterLVL` x4, `RogueTalentLVL` x6). Contributions are TYPED: athach + Shadow Creature
`ColdResistanceBonus` = 15, not 25; `AC_Natural_Armor` has 1,385 `TYPE=Base` addends.
**Deciding citation:** `PlayerCharacter.java:2090-2140`; `BonusManager.java:640-700`.

**C3 -- Division (bonus rule 1 and formula rows 14/17 "Div floors, floor(x) -> x" vs the
formula judge).** Rule: `Div` is exact (rational); ONE truncation toward zero at the
`SheetValue` boundary and at each prose slot; `Floor`/`Ceil` explicit. 255 distinct formulas /
452 units (301 non-DONE) diverge under per-step floor, including the multiclass BAB formula and
`HD/2+CON/2`; the 68 Div-under-Mul BONUS formulas (Hawkeye `5+(5*((FighterLVL-2)/4))` = 27 in
PCGen, 25 in the book) are recorded as a named PCGen-divergence class in
`artifacts/epic-6-pcgen-exit/` so parity does not chase them one by one. **Deciding
citation:** `VariableProcessor.java:378-420, 455-471`; `Description.java:295`.

**C4 -- The `_pfs/` overlay leak and `.MOD` matching (prose case 1 "215 records, match on
(category, name)" vs prereq rows PRE / .MOD "closure as is").** Rule: the mod index skips every
`.lst` under a `_pfs/` directory (PCGen loads them only through `_pfs/_.pcc` under
`PRECAMPAIGN:1,Pathfinder Society Core Assumption`; the Fetish overlay row carries no
`!PRECHARACTERTYPE`, so gate-honouring is insufficient) AND matches `.MOD` rows on (file kind,
CATEGORY, KEY-else-name), looking up by the record's KEY -- PCGen matches on KEY only, so this
drops no legitimate row and stops 4,508 display-name attachments and 79 same-name collisions.
The leak is 826 CW records (192 show the marker in `raw_tokens`; the lane's 215 counted
markers). The converter's closure must be CORPUS-WIDE: 4,419 `.MOD` targets / 6,686 rows name a
base in another book and PCGen applies them when both campaigns load. Because this lets a later
book alter an earlier book's rule, and because 56 CW records have their BASE row inside a
`_pfs/` file, both go to the operator as `blockers.md` ruling 3 with corpus-wide as the
recommendation. **Deciding citation:** `core_rulebook/_pfs/_.pcc`; `wiring_class.rs:1000-1057`
(judge reproduced the index).

**C5 -- Prose slots (prose rule 4 "same-row DEFINE -> Const; args from the right" vs formula
rows 1/27/63).** Rule: `prose: Vec<ProseSegment>`; every non-PRE segment after the first is an
argument LEFT TO RIGHT (`DescLst.java:105`; `Description.java:199`); an integer literal is a
`Const`, anything else is `Slot(Expr)` through the FORMULA rows and the Var table; a same-row
DEFINE is NEVER a value (every seed is 0; 5,161 arguments would print 0). Escapes: `%%` ->
literal `%`; a lone `%` stays text; `%{N}` = `%N`; missing argument -> empty. Entities: PCGen's
eight (`EntityEncoder.java:42-49`); `&comma;` is not one. Per-segment gates -> `ProseSegment.applies` through the prereq table (prose G2; ~1,300 CW records). **Deciding citation:** `DescLst.java:85-110`; `Description.java:199, 295`.

**C6 -- SPROP (prose "DESC rules; bare % refused, 31 rows" vs the prose judge).** Rule: SPROP and
SAB use `TextProperty.getParsedText`: tail segments are positional variables, each bare `%` is
the next slot, `'%1'` is a slot followed by a literal `1`, and the whole line is suppressed when
every variable is 0 (`suppress_when_all_zero`). `%CHOICE` -> `ChoiceName` of the eqmod
association; `%LIST` as a variable name in eqmod context is UNVERIFIED (48 CW) and needs a
citation before transcription. The bare-`%` refusal is dropped (99 CW positional records).
**Deciding citation:** `TextProperty.java:81-140`.

**C7 -- VISIBLE:DISPLAY (prose and prereq "print iff not NO" vs both judges).** Rule (recommended,
not ruled): `print` iff YES (default) / EXPORT / QUALIFY-when-held; DISPLAY and NO are held and
not printed. `Visibility.java:26` defines DISPLAY as GUI-only; the 2,319 DISPLAY rows are
selector/helper records whose feature prints elsewhere (Wizard ~ Scribe Scroll,
`cr_abilities_class.lst:2572`, would print Scribe Scroll twice). The unit is `sheet-complete`
either way (converted, held, renderable). Operator decides "PCGen's sheet" vs "print the rule":
`blockers.md` ruling 1. **Deciding citation:** `Visibility.java:23-27`.

**C8 -- `Expr::HitDice` (formula row 9 and prereq PREHD assumed total HD).** Rule: `HitDice` =
racial/monster hit dice = monster-class levels; 0 for every PC race; `Level` is total level.
PREHD:MAX=0 (Skeleton HitDice) is TRUE for a human Fighter 3. Templates' `HD:lo-hi` bands are
`Compare{HitDice}`. **Deciding citation:** `PCHDTermEvaluator.java:35`;
`CharacterDisplay.java:1084-1087`.

**C9 -- Applies arity (prereq three-valued Words vs the prereq judge and the bonus lane's
G-situational).** Rule: two-valued Include/Exclude plus `Situational{text}`, which is Include
with the condition printed on the rule's own line from a per-type template (never the token
body: the lane's PREVARLTEQ/PREEQUIP/PREDR/PREHANDSGTEQ/PREREACHGTEQ examples printed PCGen
text). PRETEXT is `Situational` (PreTextTester passes unconditionally). A gate over an absent
fact (alignment, deity, gender, age, base size, spells known, companion master) is Exclude for
a grant and is a named blocker with counts (`blockers.md` B4), not a Words verdict. TEMPBONUS is
`Situational{"when active"}` (C16). **Deciding citation:** PCGen testers return pass/fail
(`PreMultTester.java:39-61`, judge cites); the 71 unearned AUTOMATIC grants (judge census).

**C10 -- Class LEVEL grants (prereq 4.2 "no class feature has a holder; needs a ruling" vs the
prereq judge).** REFUTED for Paizo base classes: the grants are `CATEGORY=Class|<Class>.MOD`
rows in the class ability files gated `PREVARGTEQ:<Class>_CFP_Level,<lvl>` (Fighter:
`cr_abilities_class.lst:257-259`, verified), already in the class closure via
`build_mod_index`, and already extracted to `data/class_feature_grants/` (3,483 facts; 3,452
class_feature records / 1,918 non-DONE units have a holder). What is unpersisted is the numbered
level-line set: 2,016 `<lvl>\tABILITY:` lines / 72 files (630 Paizo lines at level != 1) plus
682 BONUS, 381 ADD, 168 DOMAIN, 41 TEMPLATE numbered lines -- a tool-side read of
`ClassFeatureBlock.level_lines` at convert time, allowed by `technical-design §0`, touching
`data/corpus` not at all. No ruling. **Deciding citation:** `cr_abilities_class.lst:236-259`;
`data/class_feature_grants/` (240 files on disk).

**C11 -- ABILITYCATEGORY (prereq REFUSE row, 1,091 units).** Rule: not a token-mapping refusal;
an INPUT gap owned by `BONUS:ABILITYPOOL` (1,013 pool names, 1,093 non-DONE units): the pool ->
(CATEGORY, TYPE, PRE, PLURAL, POOL) map lives only in the 208 `*_abilitycategories.lst` files
(2,831 rows; 0 in `data/corpus`; `corpus_traps.rs:196` lists the key as a trap).
`CHOOSE:ABILITYSELECTION|<parent category>|TYPE=<t>` is self-contained for 122 of 144 records
(`AbilitySelectionToken.java:239`). A ~30-line tool-side reader closes it (`blockers.md` B3).

**C12 -- Unit-count corrections.** Adopted from the judges and reproduced by this lane's census:
BONUS:SKILL 706 (lane 763), BONUS:SPELLKNOWN 338 (362), BONUS:CONCENTRATION 6 (8), BONUS:VAR
6,699 (6,708), BONUS:COMBAT 344 (352), CSKILL 425 (495), PREMULT 2,605 (2,729), PREABILITY 2,391
(2,414), `!PRE*` union 1,860 (1,516 = !PREABILITY alone), CHOOSE 1,721 (1,777), MULT 1,723
(1,779), STACK 1,453 (1,508), CLASSES 1,244 (1,228), DOMAINS 446 (435), TEMPLATE 464 (465),
ABILITY 5,957 (5,976), PREFACT 504 (516), PRERACE 105 (106), FACT 1,638 / FACTSET 463 (lane
quoted TOKEN-MODEL 8,635), ASPECT:NAME 103 instances / 52 records (not 103 records), ASPECT
structural 1,881 instances, formula row 30 ~312 units / 264 non-DONE (lane 756 / 618),
`(no raw_tokens)` 1,008 by the 37-book join (lane 921 / judge 914 on 35 books), `%CHOICE` on
TEMPBONUS 184 (lane 153). Every token-head row in the table now carries this lane's census
figure with its denominator.

**C13 -- CasterLevel and PCLEVEL (bonus C1 / E5 vs formula row 20 / G5).** Rule:
`CasterLevel(X) := ClassLevel(X) + held PCLEVEL contributions to X`; no `PcLevelBonus`
variant; `charbonusto("PCLEVEL","X")` alone maps to `Sum([CasterLevel(X), Mul(Const(-1),
ClassLevel(X))])`. **Deciding citation:** `SpellSupportForPCClass.java:165-166`.

**C14 -- Archetype replacement (bonus A1 `Applies::FeatureNotReplaced`, "PREFACT 1,621" vs
prereq).** Rule: the corpus has 0 `PREFACT` tokens with a `_CF_` fact; the flags are
`PREVAREQ:<Class>_CF_<X>,0` on the class `.MOD` grants and `BONUS:VAR|<Class>_CF_<X>|1` on
archetypes -> `Not(Holds{archetype rule that sets the var})`. A1 is dropped.

**C15 -- QUALIFY (prose "waiver on the TARGET's Applies, gap G9" vs prereq "holder-side
Effect").** Rule: holder-side `Effect::Waives{Rule(id)}`; `PrereqHandler.java:77,114` checks
QUALIFY on the character before the target's prerequisites (judge cites); "or holds rule X" is
`AtLeast{1,[normal, Holds{Rule(id)}]}` anyway. G9 dropped.

**C16 -- TEMPBONUS (formula row 60 "Applies::Situational" vs prereq "no such form"; formula
judge proposed `Toggle`).** Rule: `Situational{"when active"}` -- the value computes, the line
carries the condition, TEMPDESC supplies the words; no `Toggle` variant (a player-activated
condition is exactly the operator's rule 3 applied to the condition, not the magnitude).

**C17 -- `If`/`Cmp` (prereq §3 "comparison-as-value is a gap" vs formula row 19 and bonus E1).**
Rule: no variant. `(X>=k) = Min(1, Max(0, X-k+1))` is exact for integer operands (census:
every comparison operand in the corpus is integer-valued); `if(c,a,b) = b + c*(a-b)`; `&&` ->
`Min(c1,c2)` (189 instances / 20 units); `||` absent.

**C18 -- One name per shared variant.** `SkillRanks(SkillId)` + `SkillTotal(SkillId)` (bonus E3,
formula G6/row 22, prereq PRESKILL); `HeldCount{pool, filter}` (bonus E4, formula G7/row 23,
prereq `count("ABILITIES")`). Parameterised feat ids in PREABILITY (`Weapon Focus (Longsword)`,
543 misses) resolve as base-feat id + `Chosen{option}`.

**C19 -- Pool counters (prereq `Expr::ChoiceCount`).** Rule: no variant. `TempEvolutionTaken`
(120), `AnimalCompanionSkill` (90), `RetrainHPLimit`, `EvoImpNatArmCount` are sums of
`BONUS:VAR|<counter>|1` addends on held rules = `Var` over the contribution table.

**C20 -- PI residue (prose rule 3 "term hit REFUSES the record" vs prose rule 2 "declared
redaction -> sheet-complete with empty prose"; the 41 `PREDEITY:[redacted PI]`, 327 PREMULT
bodies, 295 ABILITY, 5 PRECAMPAIGN, 24 BENEFIT, 431 DESC term hits, 456 term-hit renames).**
Rule: one outcome for one fact -- omit the field, stamp `provenance.pi.redacted`, use the
codex-neutral label, print `Situational{"requirement withheld"}` for a redacted gate; never
refuse; re-key `declared_pi_shipping_audit.rs` to `provenance.pi` + `prose`; PI residue is
counted once (`blockers.md` B6), not per row. Operator hears the "refuse forever vs
label-only" question as `blockers.md` ruling 2.

**C21 -- ASPECT segments (prose "one line per aspect" vs `Aspect.java`).** Rule: per sub-key the
LAST segment whose gate passes prints (`lastPassingAspect`); `ProseSegment{pick_last: true}`
groups; a 71-entry sub-key -> label table (no default-to-display; no PCGen sub-key name as a
label); `*CheckCount`/`*CheckType` -> the uses row; `%CHOICE` in ASPECT is a recorded deviation
(PCGen prints it literally).

**C22 -- SERVESAS and CLASSES by kind.** `SERVESAS:CLASS|X` / `RACE|X` (7 instances) ->
`CountsAs{Class(id)}` / `CountsAs{Race(id)}` read by PRECLASS / PRERACE; `CLASSES` on the 20
`skill` records is the Skill token (class-skill declaration -> `FactGrant{ClassSkill}`), on
spell/power/class_feature the spell-level grant (`ClassesToken.java:170`, judge cites).

**C23 -- MOVECLONE third argument.** `/k` divides, `*k` multiplies, `+k` adds, and a BARE
INTEGER adds too (`MovecloneLst.java:72-73`): `MOVECLONE:Walk,Climb,0` on Vetala = climb equal
to walk (Bestiary 3: speed 30, climb 30), not "climb 0 ft". Needs `Speed(mode)`.

**C24 -- Rows no family wrote.** NATURALATTACKS (980 non-DONE) -> one `Dice` line per entry;
REACHMULT, MONSTERCLASS (feeds `HitDice`), MAXLEVEL; the nine spell stat-block tokens (SCHOOL
695, CASTTIME 679, DURATION 664, TARGETAREA 662, COMPS 651, SAVEINFO 634, SPELLRES 634,
DESCRIPTOR 361, SUBSCHOOL 174) -> `ProseFamily::StatBlock(label)` text; DEITYWEAP/ALIGN (414),
COMPANIONLIST (156), FOLLOWERS (142), USEUNTRAINED (42) and the small equipment/class facts ->
metadata or one typed field. After these, no non-DONE unit carries a token head without a row.

## D. Consolidated gap list -- every gap any lane or judge raised

Verdict: **real** (the need exists in the corpus), **necessary** (no exact encoding in v1),
**units** (non-DONE of 23,315 unless marked), **became** (the v2 variant or field, or why not).

| id (lane) | gap | real | necessary | units | became |
|---|---|---|---|---:|---|
| bonus E1 / formula -- / prereq §3 | `If(Cond)`, `Cmp` | yes | **no** | ~2,500 (98 `if()`) | exact Min/Max encoding, C17 |
| bonus E2 / formula G8 | `Ceil`, `Abs` | yes | yes (Ceil) | 41 | `Expr::Ceil`; `Abs` has 0 uses, not added |
| formula judge MISSING | `Floor` + single boundary truncation | yes | yes | 433 explicit + 301 divergent | `Expr::Floor`; exact `Div`; §2 truncation rule |
| bonus E3 / formula G6 / prereq | `SkillRanks(SkillId)` (+`SkillTotal`) | yes | yes | 53 + 143 PRESKILL | `Expr::SkillRanks`, `Expr::SkillTotal` |
| bonus E4 / formula G7 / prereq HeldCount | `count("ABILITIES",...)` | yes | yes | 45 + 21 | `Expr::HeldCount{pool, filter}` |
| bonus E5 / formula G5 / bonus C1 | `BonusCasterLevels` / `PcLevelBonus` | yes | **no** | 209 + 83 | definition of `CasterLevel(X)`, C13 |
| bonus E6 | equipment/encumbrance/armor state in formulas (`ENCUMBERANCE`, `COUNT[EQTYPE]`, `ACCHECK`, `MOVEBASE`) | yes | -- | ~60 | REFUSE by shape (formula row 21 sub-table); `Situational` words where the value is a gate |
| bonus E7 / formula G11 / prereq MasterLevel | companion master facts | yes | yes | 209 gates + 13 calls | `Expr::MasterLevel`, `Expr::MasterVar(VarId)` |
| bonus V1 | undefined identifier -> refuse per name | **no** as stated | -- | 629 -> 434 ingest / 67 case / 134 zero | C1 split |
| bonus S1 | `SheetRule.target` + `bonus_type` | yes | yes | 11,000+ instances | fields on `SheetRule` and on `VarContribution` |
| bonus T1 | table-cell target (class, spell level) | yes | yes | 1,354 instances | `BonusTarget::SpellCell{class, level}` |
| bonus D1 / formula G14 | dice with level-dependent size; `DiceBySize` | yes | yes | 586 + 5 | `Dice.size_steps`, `SheetValue::DiceBySize` |
| bonus Z1 / formula G13 / prereq BaseSize | `Size` mutated by SIZEMOD; `BaseSize`; `SizeMod` | yes | yes | 175 + 41 + 2 | `Size` := race size + held SIZEMOD; `Expr::BaseSize`; `Expr::SizeMod` |
| bonus C1 | `CasterLevel(X)` includes PCLEVEL | yes | definition | 125 | C13 |
| bonus A1 | `Applies::FeatureNotReplaced` | duplicate | **no** | 1,791 claimed | `Not(Holds{setter})`, C14 |
| bonus A2-A7 / prereq Holdable | `ClassSkill`, `NotWearing`, `Alignment`, `Size`, `Vision`, `HitDice(min,max)` | yes | yes | ~330 | `Holdable::{ClassSkill, Alignment, ...}`; `NotWearing` -> `Situational`; HD band -> `Compare{HitDice}` |
| formula G1 / bonus judge MISSING / prereq judge MISSING | `Var(VarId)` + contribution table | yes | yes | 6,646-7,518 | `Expr::Var`, `_vars/` tables, A1 |
| formula G2 / prose G1 | prose slots | yes | yes | 3,608 (5,149 CW) | `Vec<ProseSegment>` with `Slot(Expr)` |
| formula G3 / prose G5 | multi-value line (SPELLS uses/CL/DC; CheckCount) | yes | yes | 671 + 692 CW | `SheetRule.also` |
| formula G4 | `CasterLevel(Holder)` | yes | yes | 665 (RANGE) + 22 | `ClassRef::Holder` |
| formula G9 | `ChallengeRating` | yes | yes | 19 | `Expr::ChallengeRating` |
| formula G10 / bonus MOVEMULT | `Speed(mode)` | yes | yes | 24 + 1 | `Expr::Speed(MoveMode)` |
| formula G12 / prose G8 | bounded integer choice | yes | yes | 143 | `OptionSet::Number{min, max}` on the `Choice`, not on `Expr` |
| formula G15 | `HitDieSize` + die step (HITDIE `%+1`, `%/4`) | yes | -- | 8 | REFUSE by shape (`HitDieStep` semantics not read this session); `blockers.md` B8 |
| formula G16 | rational constant | yes | **no** | 6 | disappears under exact `Div` |
| formula judge MISSING | natural-attack value | yes | yes | 980 | row `NATURALATTACKS` -> `Dice` per entry |
| prose G2 | per-segment `Applies` | yes | yes | ~1,300 CW | `ProseSegment.applies` |
| prose G3 | choice-as-name | yes | yes | ~200 CW | `ProsePiece::ChoiceName` |
| prose G4 | spell one-line summary (`!PRERULE:1,DisplayFullSpell`) | yes | **no** | 1,872 CW | product rule: full text; PRERULE folds Always, the summary segment is dropped |
| prose G6 / prereq visible | print flag | yes | yes (duplicate) | 3,123 + 2,319 | `SheetRule.print`, one owner, C7 |
| prose G7 | label slot (ASPECT:NAME) | yes | **no** | 52 CW | a prose segment under G1 |
| prose G9 | waiver in Applies (QUALIFY) | **no** | no | 5 CW | holder-side `Waives`, C15 |
| prose entity table | `&colon;`, `&amp;` | yes | yes | 14 CW | PCGen's eight entities |
| prereq `Expr::ChoiceCount` | pool counters | **no** | no | ~300 instances | `Var`, C19 |
| prereq `BaseSave(Save)` | PRECHECKBASE | yes | yes | 2 | `Expr::BaseSave` |
| prereq `HighestSpellLevel(kind)` | PRESPELLTYPE, PRECLASS SPELLCASTER | yes | yes | 16 | `Expr::HighestSpellLevel` |
| prereq subject | `Applies` subject Character / Item (PRETYPE) | yes | yes | 190 | `SheetRule.subject`, `Applies::ItemHas` |
| prereq judge MISSING | `RacialHitDice` | yes | definition | 3 + 86 | `HitDice` redefined, C8 |
| prereq judge MISSING | `CountsAs{Class}` / `CountsAs{Race}` | yes | yes | 7 instances | `Effect::CountsAs{Rule\|Class\|Race}` |
| prereq judge MISSING | `DeityInPantheon`, `Proficiency(DeityFavoredWeapon)`, `AlignmentMatchesDeity` | yes | yes | 3 instances | `Holdable` leaves |
| formula judge (row 60) | `Applies::Toggle` for TEMPBONUS | yes | **no** | 418 | `Situational`, C16 |
| prose judge | SPROP positional grammar + all-zero suppression | yes | yes | 99 CW | `ProseSegment.suppress_when_all_zero`, C6 |
| prose judge | ASPECT last-passing | yes | yes | 6 CW + gated pairs | `ProseSegment.pick_last`, C21 |
| prose judge | cross-book `.MOD` closure | yes | input contract | 6,686 CW rows | A14, ruling 3 |

Variants added to `Expr` vs v1: `Var`, `Floor`, `Ceil`, `BaseSize`, `SizeMod`, `BaseSave`,
`SkillRanks`, `SkillTotal`, `HeldCount`, `ChallengeRating`, `Speed`, `HighestSpellLevel`,
`MasterLevel`, `MasterVar`; `CasterLevel(ClassId)` widened to `CasterLevel(ClassRef)`. Removed
from v1: nothing (the `// Div floors` comment is replaced by the exact-division rule).
`SheetValue` gains `DiceBySize` and `Dice.size_steps`. `SheetRule` gains `also`, `target`,
`bonus_type`, `print`, and the prereq lane's ownership fields; `prose` becomes segments.
