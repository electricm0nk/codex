# Token mapping — family `formula`

DEFINE, the PCGen formula grammar wherever it appears, the dice- and number-bearing static
tokens, and the `Expr` vocabulary itself. Author: mapping lane `formula`, 2026-09-07, branch
`tranche/15`. Machine-readable rows: `formula.json` (79 rows, same directory).

**Denominators, stated once.** *Instances* are this lane's own count over the **51,474**
`data/corpus/<book>/<kind>/*.json` records on disk (recursive walk; `_parity/` and
`LICENSE.json` excluded; the misspelled `beastiary` dir counted as `bestiary`). *Units* are
`docs/work-inventory.json` units (**49,438**; 48,608 of them join to a corpus record by
`(book, source_file basename, source_line)` with a `corpus_key` fallback). *Non-DONE* is
`scripts/completion_atlas.py::_bucket_of` (imported, not re-implemented): **23,315** of 49,438.
TOKEN-MODEL.json's figures cover only the 22,369-unit 35-book remainder and are not reused here.
PCGen semantics cite the pinned oracle checkout on this box
(`~/workspace/repos/pcgen`, `scripts/pcgen-oracle-pin.env` SHA `7f818006`), read in source.

## 1. The family's shape in one page

**DEFINE is empty.** All 15,763 readable `DEFINE:` values are the literal `0` (55 more are
`[redacted PI]`). DEFINE never carries a number; it only declares that a name is a variable.
The "seven-DEFINE block" on racial SLAs is seven zero seeds whose values all come from the
same record's `BONUS:VAR` tokens. The converter uses DEFINE once, to build the variable index,
then drops it. **Metadata.**

**The grammar is small and already parsed.** Across every formula position (BONUS formula
fields, `SPELLS` `TIMES=`/`CASTERLEVEL=`/DC fields, `TEMPBONUS`, `SR`, `DR`, `COST`, `DESC` `%N`
arguments, `PREVAR*` operands) the corpus uses: integer literals, `+ - * /` and parentheses,
15 functions (`classlevel` 1,873 calls, `max` 1,250, `if` 623, `floor` 604, `min` 544,
`charbonusto` 251, `var` 205, `skillinfo` 188, `count` 70, `ceil` 50, `mastervar` 19, `cl` 1),
seven `%`-substitutions, **12 builtin terms**, and corpus variables. `formula_interpreter.rs`
(F1..F9) is the right parser; the converter maps its AST by the table in §2.

**Builtins map cleanly, with three holes.** `STR..CHA` → `AbilityMod` (4,676 uses on 3,625
units), `<STAT>SCORE` → `AbilityScore`, `TL` → `Level` (1,385 / 1,041 units), `CL` and bare
`classlevel()` → `ClassLevel(owning class)`, `classlevel("X")` → `ClassLevel(X)`, `HD` →
`HitDice`, `BAB` → `BaseAttack`, `SIZE` → `Size` (index). Missing from technical-design §1:
**`ChallengeRating`** (`SR:11+CR`, 32 units), **`SizeMod`** (1 unit, or a fixed table over
`Size`), and the holder-relative **`CasterLevel`** that spell records need (§3).

**Variables are the whole problem.** 11,144 units reference a corpus variable; **8,518 of them
(6,646 non-DONE — 28.5% of all 23,315 non-DONE units)** reference a variable whose contributors sit on
*another* record. `Expr` has no way to say "add this record's term only if the character holds
it". That is the one gap that blocks a large population; §3 states the rule and the two
mechanical fixes.

**Everything else is static.** DAMAGE → `Dice`; CRITRANGE/CRITMULT/CR → `Text`; ACCHECK,
SPELLFAILURE, MAXDEX, REACH, MOVE, VISION, DR, SR, HD, HITDIE → `Number(Const)` or a short
`Expr`; WT, COST, PLUS, EQMOD, PROFICIENCY, WIELD, CONTAINS, SPELLKNOWN, SPELLLEVEL, KEYSTAT,
ITEM → `Metadata` (typed record fields the kind tables already hold). `SPELLS` is the one
static token that needs **three numbers per line** (uses, CL, DC).

## 2. Row table

Full rows (rule, expr shape, applies, prose effect, three quoted records each, oracle check)
are in `formula.json`. Units column: all units / non-DONE, denominators above.

| # | token_type | units (all / non-DONE) | maps_to | conf. |
|---:|---|---:|---|---|
| 1 | DEFINE | 6,353 / 4,599 | Metadata | high |
| 2 | DEFINE (PI-redacted) | 40 / 40 | **REFUSE** | high |
| 3 | DEFINESTAT | 42 / 36 | Text | medium |
| 4 | FORMULA: ability modifier `STR..CHA` | 3,625 / 1,649 | Number(Expr) `AbilityMod` | high |
| 5 | FORMULA: ability score `XSCORE` | 87 / 72 | Number(Expr) `AbilityScore` | high |
| 6 | FORMULA: `TL` | 1,041 / 556 | Number(Expr) `Level` | high |
| 7 | FORMULA: `CL`, bare `classlevel()` | 141 / 98 | Number(Expr) `ClassLevel(owning)` | high |
| 8 | FORMULA: `classlevel("X")`, `cl`, `var("CL=X")` | 666 / 621 | Number(Expr) `ClassLevel(X)` | high |
| 9 | FORMULA: `HD` | 406 / 84 | Number(Expr) `HitDice` | high |
| 10 | FORMULA: `BAB` | 42 / 30 | Number(Expr) `BaseAttack` | high |
| 11 | FORMULA: `SIZE`, `SIZEMOD` | 8 / 6 | Number(Expr) `Size` (+gap SizeMod) | medium |
| 12 | FORMULA: `CR` | 32 / 19 | Number(Expr) **gap ChallengeRating** | medium |
| 13 | FORMULA: bare `CASTERLEVEL`/`SPELLLEVEL` | 78 / 62 | Number(Expr) `CasterLevel`/`SpellLevel` | medium |
| 14 | FORMULA: integer literal, arithmetic | every formula unit | Number(Expr) `Const/Sum/Mul/Div` | high |
| 15 | FORMULA: fractional literal | 192 / 95 | Number(Expr) (7 units refuse) | high |
| 16 | FORMULA: `max`/`min` | 1,054 / 566 (max) | Number(Expr) `Max/Min` nested | high |
| 17 | FORMULA: `floor` | 492 / 433 | Number(Expr) (Div floors) | high |
| 18 | FORMULA: `ceil` | 49 / 41 | Number(Expr) **gap Ceil** (rewrite for X≥0) | medium |
| 19 | FORMULA: `if(cmp,a,b)`, comparisons | 98 / 93 | Number(Expr) (exact Min/Max encoding) | high |
| 20 | FORMULA: `charbonusto("PCLEVEL","X")` | 227 / 209 | Number(Expr) **gap PcLevelBonus** unless CasterLevel includes it | medium |
| 21 | FORMULA: `var("<export token>")` | 134 / 117 | **REFUSE** per prefix (CL=/STAT./SKILL. resolve) | medium |
| 22 | FORMULA: `skillinfo(...)` | 65 / 53 | Number(Expr) **gap SkillRank** | high |
| 23 | FORMULA: `count("ABILITIES",…)` | 57 / 44 | **REFUSE** (gap CountHeld) | medium |
| 24 | FORMULA: `mastervar`, `MasterLevel` | 14 / 7 (+ companion gates) | **REFUSE** (gap MasterVar) | high |
| 25 | FORMULA: `%CHOICE`, `%LIST` | 207 / 182, 101 / 27 | Number(Expr) `Choice` | medium |
| 26 | FORMULA: `%SPELLLEVEL %CASTERLEVEL %CHARGES …` (EQMOD price) | ≤23 each | Metadata | high |
| 27 | FORMULA: corpus variable, same-record only | 1,318 / 793 | Number(Expr) folded | high |
| 28 | **FORMULA: corpus variable, cross-record** | **8,518 / 6,646** | Number(Expr) **gap Held/Var** | high |
| 29 | FORMULA: variable with an unheld contributor row | ≤3,494 / ≤2,657 | **REFUSE** per file | medium |
| 30 | FORMULA: identifier DEFINEd nowhere | 756 / 618 | Number(Expr) `Const(0)` (PCGen prints 0) | medium |
| 31 | FORMULA: malformed (parser refusals) | 240 fields (inherited) | **REFUSE** | medium |
| 32 | DAMAGE / ALTDAMAGE | 535 / 7 | **Dice** | high |
| 33 | CRITRANGE | 510 / 7 | Text (4-entry table) | high |
| 34 | CRITMULT | 524 / 7 | Text | high |
| 35 | UDAM | 113 / 113 | **Dice** (5 units gap DiceBySize) | high |
| 36 | UMULT | 113 / 113 | Text | high |
| 37 | RANGE (spell keyword) | 2,581 / 665 | Number(Expr) 3-row table; **gap CasterLevel(holder)** | medium |
| 38 | WT | 4,948 / 12 | Metadata | high |
| 39 | COST (literal) | 7,716 / 2,191 | Metadata | high |
| 40 | COST (formula) | 84 / 78 | Metadata | high |
| 41 | ACCHECK | 178 / 1 | Number(Expr) `Const` | high |
| 42 | SPELLFAILURE | 182 / 0 | Number(Expr) `Const` | high |
| 43 | MAXDEX | 142 / 0 | Number(Expr) `Const` | high |
| 44 | MOVE | 1,918 / 172 | Number(Expr) `Const` per mode | high |
| 45 | MOVECLONE | 39 / 24 | Number(Expr) **gap Speed(mode)** | high |
| 46 | UNENCUMBEREDMOVE | 21 / 13 | Text | high |
| 47 | REACH | 1,614 / 60 | Number(Expr) `Const` | high |
| 48 | HITDIE (n) | 59 / ≤26 | Number(Expr) `Const` | high |
| 49 | HITDIE (%-step) | 8 / 8 | **REFUSE** (gap HitDieSize) | high |
| 50 | HD (class) | 181 | Number(Expr) `Const` | high |
| 51 | HD (template band) | 27 | Applies `HitDice(lo,hi)` | high |
| 52 | DR | 699 / 108 | Number(Expr) `Const` + qualifier prose | high |
| 53 | SR | 407 / 43 | Number(Expr) (30 need ChallengeRating) | high |
| 54 | CR | 1,611 / 78 | Text | high |
| 55 | CRMOD | 1 / 1 | Metadata | high |
| 56 | SPELLS | 1,423 / 671 | Number(Expr) ×3 — **gap multi-value SheetValue** | high |
| 57 | SPELLS (PI-redacted) | 78 / 66 | **REFUSE** | high |
| 58 | SPELLKNOWN | 1,491 / 1,049 | Metadata | high |
| 59 | SPELLLEVEL | 503 / 374 | Metadata | high |
| 60 | TEMPBONUS | 575 / 418 | Number(Expr), Applies::Situational | medium |
| 61 | TEMPVALUE | 164 / 143 | Number(Expr) `Choice` with range (gap) | high |
| 62 | TEMPDESC | 184 / 102 | Text | high |
| 63 | DESC / BENEFIT `%N` argument | 5,013 / 3,634 | Number(Expr) slot — **gap prose slots** | high |
| 64 | SAB | 2 / 0 | Text | high |
| 65 | SPROP | 3,614 / 259 | Text | high |
| 66 | SIZE | 2,168 / 63 | Metadata (binds Size) | high |
| 67 | SIZE (formula) | 1 / 1 | **REFUSE** | high |
| 68 | LEGS / HANDS | 1,337 / 119 | Metadata | high |
| 69 | VISION | 510 / 102 | Number(Expr) `Const` per entry | high |
| 70 | PLUS | 534 / 35 | Metadata | high |
| 71 | ADDLEVEL | 22 / 22 | Metadata | high |
| 72 | LEVELSPERFEAT | 9 / 9 | Metadata | high |
| 73 | KEYSTAT | 148 / 50 | Metadata | high |
| 74 | ITEM | 685 / 193 | Metadata | high |
| 75 | SITUATION | 182 / 182 | Text | high |
| 76 | EQMOD / ALTEQMOD | 2,210 / 140 | Metadata | high |
| 77 | PROFICIENCY / WIELD | 730 / 7 | Metadata | high |
| 78 | CONTAINS / CHARGES / BASEITEM / BASEQTY / MODS / FUMBLERANGE | 210 / 3 … | Metadata | high |
| 79 | `STAT:` / `AC:` / `LEVELADJUSTMENT:` | 0 | (no corpus instance) | high |

**REFUSE rows: 9**, covering at most 40 + 134 + 57 + 14 + 3,494 + 8 + 78 + 1 units (row 31's
240 fields overlap rows 19–24). The only large refusal, row 29, is an **ingest** remainder (rows
in files we already cite), not per-record work.

## 3. The Expr question, answered

**Does technical-design §1's `Expr` cover every value the corpus needs? No.** Twelve of its
sixteen variants are exercised and correct. The missing variants, each with the token and the
units that need it:

| gap | variant | needed by | units (all / non-DONE) | blocking? |
|---|---|---|---:|---|
| **G1** | `Held(RuleId, Box<Expr>)` — or `Var(VarId)` + a converter-emitted contribution table | every cross-record variable (row 28) | **8,518 / 6,646** | **yes** |
| G2 | prose value slots: `prose: Vec<ProseSegment{Text \| Value(Expr)}>` (a SheetRule gap, not Expr) | DESC/BENEFIT `%N` (row 63) | 5,013 / 3,634 | **yes** |
| G3 | multi-value `SheetValue` (uses, CL, DC) or three sibling rules per spell entry | SPELLS (row 56) | 1,423 / 671 | yes |
| G4 | `CasterLevel(ClassRef::Holder)` — the class the rule is held through, bound at pick time | RANGE keywords (row 37), bare CASTERLEVEL (13) | 2,581 / 665 | yes for scaling spells |
| G5 | `PcLevelBonus(ClassId)` unless `CasterLevel(X)` is defined to include PCLEVEL grants | `charbonusto` (row 20) | 227 / 209 | decision, not code |
| G6 | `SkillRank(SkillId)` (+ `SkillTotal`) | `skillinfo` (row 22), `var("SKILL.x.RANK")` | 65 / 53 (+28 calls) | yes |
| G7 | `CountHeld(category, filter)` | `count("ABILITIES",…)` (row 23) | 57 / 44 | refuse otherwise |
| G8 | `Ceil` | `ceil` (row 18) | 49 / 41 | exact rewrite for X ≥ 0 |
| G9 | `ChallengeRating` | `SR:11+CR` (rows 12, 53) | 32 / 19 | yes (monsters) |
| G10 | `Speed(mode)` | MOVECLONE (row 45) | 39 / 24 | yes |
| G11 | `MasterVar(id)` / `MasterLevel` | `mastervar` (row 24), companion gates | 14 / 7 (+240 gates, bonus lane E7) | refuse otherwise |
| G12 | `Choice` with a domain (integer range `min..max` as Exprs) | TEMPVALUE (row 61), `%CHOICE` (25) | 164 / 143 | yes |
| G13 | `SizeMod` (or a fixed table over `Size`) | `SIZEMOD` (row 11) | 1 / 1 | small |
| G14 | `DiceBySize([String;9])` | UDAM list form (row 35) | 5 / 5 | small |
| G15 | `HitDieSize` + die-step | HITDIE `%+1`, `%/4` (row 49) | 8 / 8 | small |
| G16 | rational constant (or non-flooring divide) | bare `1/3`, `0.5` seeds (row 15) | 7 / 6 | small |
| — | `If(Cond,a,b)` / `Cmp` | `if()` (row 19) | 98 / 93 | **no** — `(X>=k) ≡ Min(1, Max(0, X−k+1))` is exact |

Not gaps: `floor` (Div floors, agrees with `Math.floor`), N-ary `max`/`min` (nest), `X*3/4` and
`X*1.5` (reduced fractions under `Div(Mul)`, exact), `classlevel("APPLIEDAS=NONEPIC")` (same
as bare), cross-class `classlevel("X")` (the live evaluator knows every class level).

### The cross-record resolution rule (row 28)

**What PCGen does** (`PlayerCharacter.java:2090–2140 getVariable`, `VariableProcessor.java:532
lookupVariable`, read on this box): the value of variable `V` for character `C` is the largest
`DEFINE` seed among objects `C` **holds** (always 0 in this corpus) **plus the sum of every
`BONUS:VAR|V|f` on objects `C` holds whose own PRE passes**. If no held object DEFINEs `V`, the
name is not a variable: PCGen tries the builtin terms, then the export tokens, then the
"broken parser" → 0, and **drops** the `BONUS:VAR` contributions (`includeBonus = false`).

**What the converter must do.** `V` is the sum of contributions from *held* rules. `Expr` has no
"held" term, so there are exactly two mechanical forms:

- **(A) Inline.** Replace `V` by `Sum([Held(r₁, f₁), Held(r₂, f₂), …])` over every contributor
  record. Correct; no PCGen name survives; but `AC_Natural_Armor` has **1,545** contributors,
  `PrimaryAttackDamageDice` 538, `Maneuverability` 511, `DarkvisionRange` 342 — 506 of the
  20,563 (unit, variable) reference pairs name a variable with 50+ contributors, and every
  consumer would carry all of them.
- **(B) Variable table.** `Expr::Var(VarId)` with `VarId` an opaque converter-minted id, plus a
  derived `data/sheet_rules/_vars/<id>.json` listing `(rule_id → Expr)` contributions. The live
  evaluator sums the contributions of held rules — a forty-line loop, no parser. This is PCGen's
  own mechanism restated in our schema, and it is the only form that stays small.
  **Recommendation: B.**

**Why unconditional inlining is wrong.** The bonus lane's rule 2 ("a DEFINEd identifier → its
own producer chain inlined") sums contributions from records the character does not hold.
`BloodlineLVL` has 6 contributors (Blood Arcanist, Eldritch Scion, Bloodline Development,
Crossblooded, …); a Sorcerer holds one. Unconditional inlining sextuples it. Every contributor
must be gated by its own record's `Applies` — which is exactly G1.

**DEFINE-existence gating is dropped.** Every seed is 0, so gating on "some held object DEFINEs
`V`" changes no value; its only effect would be to drop bonuses PCGen drops — the 184 names of
row 30, which the converter maps to `Const(0)` and lists in a defect file (PCGen prints 0 for
them too; the bonus lane's "refuse per name" would refuse 618 non-DONE units the oracle
prints).

**Counts (units with a corpus record, of 48,608):** same-record only **1,318** (793 non-DONE);
cross-record **8,518** (6,646 non-DONE, 28.5% of all 23,315 non-DONE); any variable **11,144**
(8,309). Contributors in *unheld rows of ingested files* (row 29): 7,768 rows across the 677
cited `.lst` files — 5,969 are `.MOD` rows the token closure already reads when the base record
is ingested, 1,799 are base rows never ingested (`acg_abilities_globalvar.lst` 703,
`isg_deities.lst` 594, `cr_abilities_class.lst` 593, `apg_abilities_globalvar.lst` 492) — an
upper bound of **3,494** units (2,657 non-DONE) depends on one. That number must be re-derived
from the closure, not the record; it is an ingest remainder, per file, never per record.

## 4. Oracle coverage

`scripts/oracle_harness/` exports (read from the three `.ftl` templates): `computed-values` —
NAME, RACE, CLASS.0.NAME/LEVEL, STAT.{STR,DEX,CON}.{SCORE,MOD}, HP, AC.TOTAL/TOUCH/FLATFOOTED,
BAB, VAR.CMB, VAR.CMD, CHECK.n.NAME/TOTAL/BASE; `charbuild-remainder` — all six STAT.i.SCORE,
SA.COUNT, SA.n.NAME, **SA.n.DESC** (PCGen's own substituted words); `weapon-family` —
WEAPON.COUNT only. So: rows 4–10, 14–19, 27–28 are checkable through SA.n.DESC (the number lands
in the words) and the totals; row 63 (prose slots) has a *direct* oracle in SA.n.DESC; RANGE,
SR, DR, MOVE, REACH, VISION, DAMAGE, CRITRANGE/CRITMULT, SPELLS uses/CL/DC are **sheet-line
only** — no export line carries them. Row 30's `Const(0)` needs one empirical check (a
`BloodlinePower1TimesBonus` consumer's SA.n.DESC) before it is adopted over refusal.

## 5. The three hardest cases

**1. A variable with six contributors and a level gate — `BloodlineLVL`.**
`occult_adventures:class_feature:psychic_bloodline` (`oa_abilities_class.lst:1872`) consumes
`BloodlineLVL`, `BloodlineProgressionLVL`, `BloodlinePowerTimes` and `Sorcerer_CF_*` flags.
Contributors: `advanced_class_guide:class_feature:blood_arcanist_bloodline`,
`…:eldritch_scion_bloodline`, `…:arcanist_exploit_bloodline_development`,
`ultimate_magic:…:draconic_bloodline_crossblooded`, `core_rulebook:…:bloodline_tracker`,
`ultimate_wilderness:race_trait_generic:ghoran_creators_legacy` (6 records, 3 books). Only
one is ever held. This is G1 in its plainest form, and it is the shape of 6,646 non-DONE units.

**2. The phantom block — `if()` over a master's variable.**
`occult_adventures:race_trait:phantom_companion_progression` (`oa_abilities_race.lst:15`):
`BONUS:VAR|PhantomArmorBonus|if(PhantomMasterLevel>=2,2,0)+if(PhantomMasterLevel>=5,2,0)+…`
and `phantom_companion_deliver_touch_spells` (`:10`):
`BONUS:VAR|DeliverTouchSpellsRange|30+if(PhantomMasterLevel>=12,20,0)` with
`DESC:…|DeliverTouchSpellsRange`. The `if()` chain converts exactly by the Min/Max encoding
(row 19); the DESC argument is a prose slot (G2); `PhantomMasterLevel` is a companion's
master-level fact (G11). One record, three gaps, and it is `literal-verified` on the board
today — meaning the words shipped and the number did not.

**3. A spell-like ability that needs three numbers and a variant pick.**
`monster_codex:race_trait:racial_sla_ironskin` (`mc_abilities_race.lst:95`): seven `DEFINE|0`,
five `BONUS:VAR` (`_LVL|TL`, `_SpellLVL|1`, `_Times|1`, `_DCMod|CHA`, `_DC|10+_SpellLVL+_DCMod`),
and three `SPELLS:Racial|TIMES=…|CASTERLEVEL=…|Ironskin,…_DC|PREVAREQ:…_AtWill,0|…` variants.
Everything is same-record (row 27) and every term has an `Expr` — uses `Const(1)`, CL `Level`,
DC `Sum([Const(10), Const(1), AbilityMod(Cha)])` = **DC 15** for the fixture — but the line
"Ironskin 1/day (CL 5, DC 15)" needs three values in one rule (G3) and the `PREVAREQ` tails
select which of the three `SPELLS` tokens prints (Applies). 1,423 units share this shape; the
racial-SLA subset (≈450) is the cheapest first population once G3 is settled.

## 6. Refusals by mechanism

| mechanism | units (all / non-DONE) | what closes it |
|---|---:|---|
| unheld contributor rows in ingested files (row 29) | ≤3,494 / ≤2,657 | ingest the 1,799 base rows (four `_globalvar`/class files carry 2,382 of 7,768); re-derive with the closure |
| `var("<export token>")` outside CL=/STAT./SKILL. (row 21) | ≤134 / ≤117 | equipment/encumbrance facts (mostly Applies::NotWearing, prereq lane) |
| PI-redacted DEFINE / SPELLS (rows 2, 57) | 118 / 106 | Decision 28 PI ruling; nothing mechanical |
| `count("ABILITIES",…)` (row 23) | 57 / 44 | G7 |
| `mastervar` (row 24) | 14 / 7 | G11 |
| HITDIE %-step, SIZE formula, bare fraction seeds (rows 49, 67, 15) | 16 / 15 | G15, G16 |
| parser-malformed text (row 31) | ≤240 fields | none; literal upstream defects |

Lowest-confidence rows: 30 (`Const(0)` for names DEFINEd nowhere — needs one oracle check),
20 (`charbonusto` folding depends on how §1 defines `CasterLevel`), 29 (upper bound, closure
not yet applied), 37 (RANGE table needs G4), 60 (TEMPBONUS needs an `Applies::Situational`
word from the prereq lane).
