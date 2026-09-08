# `prereq` family — PRE\* gates and the ownership structure → `Applies`

Lane: prereq (SD-35 Epic 2 token-mapping). Branch `tranche/15`. Date 2026-09-07.
Companion: `prereq.json` (93 rows, this directory). Read order and rules: the shared brief.

**Denominators used everywhere below.** Instances = every `raw_tokens` entry across
`data/corpus/**` (48,706 records with `raw_tokens`, 37 books, every status), counted by this
lane's own walk (`index_prereq.py`, scratchpad). Units = non-DONE units under
`completion_atlas._bucket_of` (**23,315 of 49,438**) joined to a corpus record carrying the
token. 22,503 of the 23,315 join to a record with `raw_tokens`; 351 more are feat records that
carry a `prerequisites` list instead of `raw_tokens`. TOKEN-MODEL's figures (35-book remainder as
of 2026-08-31, 22,366 units) are quoted only where marked.

**Family coverage.** 22,081 non-DONE units carry at least one token of this family in
`raw_tokens`, plus 351 token-less feat units with a `prerequisites` list: **22,432 of 23,315
non-DONE units (96.2%)** touch this family. That is not surprising: `CATEGORY`/`TYPE`/`KEY` are on
nearly every record. The PRE\* gates alone are on **6,930 records corpus-wide** for
`PREABILITY`+`PREMULT`+`PREVARGTEQ` and 14,734 instances on the TOKEN-MODEL remainder.

## 1. The shape of the family in one page

A `SheetRule` has two ownership questions and one gate question. PCGen answers them with
different tokens; our schema answers them with one structure:

```rust
pub struct SheetRule {
    // ... id, label, value, prose, provenance (technical-design §1) ...
    pub pool: PoolId,                 // from CATEGORY:  (Special Ability, FEAT, Archetype, Internal, ...)
    pub tags: Vec<Tag>,               // from TYPE:      (dot-segments)
    pub visible: bool,                // from VISIBLE:   (NO -> held silently, never printed)
    pub subject: Subject,             // Character (default) | Item (equipment/eqmod rules gated by PRETYPE)
    pub repeatable: bool,             // MULT:YES
    pub applies: Applies,             // the gate: may THIS character hold it?   (PRE*, converted)
    pub granted_by: Vec<Grant>,       // the edges: WHO hands it out             (ABILITY, class level lines, TEMPLATE, DOMAINS, CLASSES, choices)
    pub offers: Option<Choice>,       // choice-bearing records                  (CHOOSE, ADD, SELECT, LANGBONUS)
    pub grants: Vec<Effect>,          // what holding it does to the fact set    (AUTO, CSKILL, FACT declarations, SERVESAS, QUALIFY, REMOVE)
}

/// The gate. Closed vocabulary; every leaf names a fact the live character has,
/// or is Words (never a PCGen variable, never a formula string).
pub enum Applies {
    Always, Never,
    All(Vec<Applies>),
    AtLeast { n: u8, of: Vec<Applies> },              // PREMULT:n  and every PRE<x>:n,... count form
    Not(Box<Applies>),                                    // !PRE<x>
    Compare { lhs: Expr, op: Cmp, rhs: Expr },          // PRESTAT, PRESKILL, PRETOTALAB, PRELEVEL, PREHD, PRESIZE*, PREBASESIZE*, PRECLASS, resolved PREVAR*
    Holds { what: Holdable, count: u8 },               // PREABILITY, PRERACE, PREFACT, PRETEMPLATE, PREALIGN, PREDEITY*, PREDOMAIN, PRECSKILL, PREWEAPONPROF, PRESPELL*, PREMOVE, PREVISION, PRELANG, flag-shaped PREVAR*
    Chosen { choice: ChoiceId, option: Option<OptionId> }, // %CHOICE / %LIST bound grants
    ItemHas { tags: Vec<Tag>, n: u8 },                  // PRETYPE (subject = Item)
    Words { text: String },                             // PRETEXT and anything unresolvable: prints, never filters, never blocks
}
pub enum Cmp { Eq, Ne, Lt, Lte, Gt, Gte }
pub enum Holdable {
    Rule(RuleId), RuleTag { pool: PoolId, tag: Tag }, Template(RuleId), Spell(RuleId),
    Race(RaceId), RaceType(Tag), RaceSubtype(Tag),
    Alignment(AlignSet), Deity(DeityRef), DeityGrantsDomain(RuleId), DeityAlignment(AlignSet),
    ClassSkill(SkillId), Proficiency(ProfRef), Language(LangRef), Movement { mode: Tag, min: u16 }, Vision(Tag),
    ClassTag(Tag), Gender(Tag), AgeCategory(Tag),
}
pub struct Grant  { pub by: Granter, pub when: Applies }          // Granter = Rule(id) | Class { id, at_level } | Race(id) | Deity(id) | Choice(ChoiceId)
pub struct Choice { pub id: ChoiceId, pub count: Expr, pub from: OptionSet }
pub enum OptionSet { Rules { pool, tags, requires: Applies }, Skills(..), Weapons(..), Spells { class, levels }, Languages(..), Templates(..), Classes(..), FreeText, Number }
pub enum Effect { FactGrant(Fact), FactRevoke(Fact), CountsAs(RuleId), Waives(RuleId), Revokes(RuleId), FactDeclare { name, value } }
```

**Evaluation is three-valued, exactly as `pre_tokens.rs` does it today.** A leaf is `true`,
`false`, or `words`. `AtLeast` is met when n leaves are true; it is unmet only when even counting
every `words` leaf as true still falls short; otherwise it is `words`. `Not` flips true/false and
leaves `words` alone. **Only `false` excludes.** A `words` gate means "include, and show the
requirement as not checked". This is the property that makes the per-character filter honest
without a PCGen engine: a gate we cannot evaluate never denies a feat the character is entitled to.

**The held set is a fixpoint.** Start from the character's race, class levels, templates and
choices. Add every rule some held rule grants (`granted_by`) whose `when` and `applies` are not
`false`. Apply `Effect`s (facts, counts-as, waivers, revokes). Repeat until nothing changes.
Everything on the sheet is read off that set: `visible` rules print through their `SheetValue`;
`Internal` and `VISIBLE:NO` rules stay silent.

**AT-35-E5-004, the per-character choice filter, is one query over this structure.** At level-up
the character is offered every `Choice` on a held rule whose `count` (an `Expr`) is above the
picks already made. For a `Choice` whose `from` is `OptionSet::Rules`, the option list is: every
rule in that pool with those tags whose `applies` (and the pool's own `requires`) evaluates to
`true` or `words` against the character's facts, minus rules already held unless `repeatable`,
plus anything a held `Waives` unlocks. Excluded options are the `false` ones. No PCGen token,
variable or parser is touched on the live side; the converter did all the reading.

**What the live character must expose** (facts, not tokens). Present today on
`CharacterPrereqFacts` + the computed chassis: race id, class levels, total level, HD, BAB,
six ability scores, selected feats / held rule ids, skill ranks, size. **Missing today and
needed by rows in this table:** alignment (141 `PREALIGN` + 37 `!PREALIGN` + 808 `DOMAINS`
gates), deity (50 `PREDEITY`, 37 `PREDEITYDOMAIN`, 8 `PREDEITYALIGN`), gender (3), age
category (2), base size before modifiers (41 `PREBASESIZE*`), base saves (2), highest spell
level per casting kind (16), class skills / proficiencies / languages / movement / vision as
held-fact sets (derived from the `AUTO`/`CSKILL`/race rows once those convert). Until a fact
lands its gates evaluate to `words`; the unit is still `sheet-complete` because the rule
converted and the evaluator renders it.

## 2. The mapping table

Instances and units per the denominators at the top. `resolvability` is the E5-004 verdict:
*filter* = evaluates true/false per character; *situational* = `Words`; *metadata* = consumed by
the converter, never a gate; *Choice* = resolves once the player picks.

| token_type | instances | units | maps_to | resolvability | confidence |
|---|---:|---:|---|---|---|
| `PREABILITY` | 3,411 | 2,414 | Applies | filter (per-character-resolvable) | high |
| `PREMULT` | 4,708 | 2,729 | Applies | filter (per-character-resolvable) | high |
| `PREVARGTEQ` | 4,213 | 2,848 | Applies | filter for classes a-c (about 90% of i | medium |
| `PREVARGT` | 69 | 67 | Applies | filter (per-character-resolvable) | medium |
| `PREVARLT` | 506 | 402 | Applies | Words for counters (about 45%), filter | medium |
| `PREVARLTEQ` | 22 | 22 | Applies | situational (prints as words; never bl | medium |
| `PREVAREQ` | 304 | 256 | Applies | filter (per-character-resolvable) | medium |
| `PREVARNEQ` | 4 | 4 | Applies | situational (prints as words; never bl | high |
| `PRECLASS` | 1,282 | 1,184 | Applies | filter (per-character-resolvable) | high |
| `PRECLASSLEVELMAX` | 1 | 1 | Applies | filter (per-character-resolvable) | high |
| `PRESUBCLASS` | 17 | 17 | Applies | filter (per-character-resolvable) | medium |
| `PRESTAT` | 272 | 242 | Applies | filter (per-character-resolvable) | high |
| `PRESKILL` | 198 | 143 | Applies | filter (per-character-resolvable) | high |
| `PRECSKILL` | 9 | 9 | Applies | filter (per-character-resolvable) | high |
| `PRETOTALAB` | 90 | 67 | Applies | filter (per-character-resolvable) | high |
| `PRECHECKBASE` | 2 | 2 | Applies | filter (per-character-resolvable) | high |
| `PRELEVEL` | 26 | 16 | Applies | filter (per-character-resolvable) | high |
| `PREPCLEVEL` | 61 | 2 | Applies | filter (per-character-resolvable) | high |
| `PRELEVELMAX` | 1 | 1 | Applies | filter (per-character-resolvable) | high |
| `PREHD` | 3 | 3 | Applies | filter (per-character-resolvable) | high |
| `PRERACE` | 305 | 106 | Applies | filter (per-character-resolvable) | high |
| `PREFACT` | 727 | 516 | Applies | filter (per-character-resolvable) | medium |
| `PRETEMPLATE` | 67 | 65 | Applies | filter (per-character-resolvable) | high |
| `PRESIZEEQ` | 885 | 885 | Applies | filter (per-character-resolvable) | high |
| `PRESIZEGTEQ` | 8 | 8 | Applies | filter (per-character-resolvable) | high |
| `PRESIZELTEQ` | 10 | 9 | Applies | filter (per-character-resolvable) | high |
| `PRESIZELT` | 1 | 1 | Applies | filter (per-character-resolvable) | high |
| `PREBASESIZEGTEQ` | 19 | 19 | Applies | filter (per-character-resolvable) | high |
| `PREBASESIZEEQ` | 16 | 16 | Applies | filter (per-character-resolvable) | high |
| `PREBASESIZELT` | 6 | 6 | Applies | filter (per-character-resolvable) | high |
| `PREALIGN` | 141 | 117 | Applies | filter once alignment is a character f | high |
| `PREDEITY` | 50 | 50 | Applies | filter once deity is a character fact; | high |
| `PREDEITYDOMAIN` | 37 | 2 | Applies | filter once deity is a character fact | medium |
| `PREDEITYALIGN` | 8 | 8 | Applies | filter once deity is a character fact | medium |
| `PREDOMAIN` | 8 | 5 | Applies | filter (per-character-resolvable) | high |
| `PREWEAPONPROF` | 12 | 11 | Applies | filter (per-character-resolvable) | high |
| `PREPROFWITHSHIELD` | 1 | 1 | Applies | filter (per-character-resolvable) | high |
| `PREARMORTYPE` | 1 | 1 | Applies | situational (prints as words; never bl | high |
| `PRESPELL` | 16 | 12 | Applies | filter (per-character-resolvable) | medium |
| `PRESPELLTYPE` | 10 | 10 | Applies | filter once Expr::HighestSpellLevel ex | medium |
| `PRESPELLCAST` | 5 | 5 | Applies | filter (per-character-resolvable) | medium |
| `PRESPELLBOOK` | 1 | 1 | Applies | filter (per-character-resolvable) | medium |
| `PRESPELLDESCRIPTOR` | 1 | 1 | Applies | filter once Holdable::Spell carries de | low |
| `PREMOVE` | 11 | 7 | Applies | filter (per-character-resolvable) | medium |
| `PREVISION` | 6 | 6 | Applies | filter (per-character-resolvable) | medium |
| `PRELANG` | 1 | 1 | Applies | filter (per-character-resolvable) | high |
| `PREGENDER` | 3 | 2 | Applies | Words today; filter once the fact exis | high |
| `PREAGESET` | 1 | 0 | Applies | Words today; filter once the fact exis | high |
| `PREEQUIP` | 2 | 0 | Applies | situational (prints as words; never bl | high |
| `PREDR` | 1 | 1 | Applies | situational (prints as words; never bl | high |
| `PREHANDSGTEQ` | 1 | 1 | Applies | situational (prints as words; never bl | high |
| `PREREACHGTEQ` | 1 | 1 | Applies | situational (prints as words; never bl | high |
| `PRETEXT` | 168 | 139 | Applies | situational (prints as words; never bl | high |
| `PRETYPE` | 1,252 | 190 | Applies | filter over item facts (equipment chas | high |
| `PRERULE` | 7 | 7 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `PRECAMPAIGN` | 4 | 0 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | medium |
| `PRECHARACTERTYPE` | 2 | 2 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `PRE` | 1 | 1 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `!PRE* (all negated forms)` | 2,559 | 1,516 | Applies | filter (per-character-resolvable) | high |
| `feat.prerequisites (token-less feat records)` | 2,270 | 351 | Applies | filter (per-character-resolvable) | high |
| `KEY` | 31,498 | 15,245 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `CATEGORY` | 30,997 | 17,385 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `TYPE` | 41,700 | 18,216 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `VISIBLE` | 7,505 | 4,775 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `ABILITY` | 18,559 | 5,976 | Applies | filter (grant edge evaluated per chara | high |
| `AUTO` | 1,298 | 246 | Applies | filter (fact grant) | high |
| `TEMPLATE` | 838 | 465 | Applies | filter (grant edge) | high |
| `KIT` | 1,632 | 107 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `CSKILL` | 842 | 495 | Applies | filter (fact grant) | high |
| `CCSKILL` | 9 | 8 | Applies | filter (fact revoke) | high |
| `MONCSKILL` | 403 | 21 | Applies | filter (fact grant) | high |
| `CLASSES` | 5,199 | 1,228 | Applies | filter (grant edge) | medium |
| `DOMAINS` | 808 | 435 | Applies | filter once alignment and deity are ch | high |
| `RACETYPE` | 1,853 | 111 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `RACESUBTYPE` | 1,066 | 207 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `SUBRACE` | 146 | 146 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `FACT / FACTSET` | 9,690 | 0 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | medium |
| `SERVESAS` | 318 | 136 | Applies | filter (per-character-resolvable) | high |
| `QUALIFY` | 7 | 4 | Applies | filter (per-character-resolvable) | high |
| `REMOVE` | 10 | 7 | Applies | filter (per-character-resolvable) | high |
| `ADD` | 113 | 42 | Applies | Choice (filter once the option set is  | medium |
| `CHOOSE` | 2,098 | 1,777 | Applies | Choice: filter over the option set onc | medium |
| `SELECT` | 93 | 74 | Applies | Choice count (per-character number) | high |
| `MULT` | 2,107 | 1,779 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `STACK` | 1,651 | 1,508 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `LANGBONUS` | 196 | 97 | Applies | Choice | high |
| `STARTFEATS` | 1,410 | 45 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `LEVELSPERFEAT` | 9 | 9 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `%CHOICE / %LIST (value markers)` | 798 | 0 | Applies | Choice-bound: filter/number once chose | high |
| `class LEVEL grants (CLASS .lst level lines)` | 65 | 12,109 | Applies | filter (grant edge) | medium |
| `.MOD / .COPY closure rows` | 0 | 874 | Metadata(ignored-by-sheet) | metadata (consumed by the converter, n | high |
| `trailing |PRE... on host tokens (per-addend gates)` | 21,889 | 0 | Applies | filter (per-character-resolvable) | high |
| `ABILITYCATEGORY (pool declarations)` | 0 | 1,091 | REFUSE | blocked on ingest | high |

Synthetic rows (`!PRE*`, `feat.prerequisites`, `FACT / FACTSET`, `%CHOICE / %LIST`, class
`LEVEL` grants, `.MOD` closure, trailing `|PRE`, `ABILITYCATEGORY`) state their own denominators
in `prereq.json`; their `units` must not be summed with the token rows.

### 2.1 The 21 PRE types `pre_tokens.rs` does not name

TOKEN-MODEL counts 69 prereq types on the remainder (negations counted separately).
`pre_tokens.rs` names 16 in `MODELLED_KINDS` (+`PRETEXT`) and 25 in `UNMODELLED_KINDS`; stripping
the `!`, 48 of the 69 fall under one of those. The 21 that do not, with TOKEN-MODEL
remainder counts (denominator 22,366 units) and this table's disposition:

| type | instances | units | this table |
|---|---:|---:|---|
| `PRETYPE` | 150 | 136 | Applies / filter over item facts (equipment chassis has TYPE:) |
| `!PRETYPE` | 68 | 58 | Applies / filter over item facts (equipment chassis has TYPE:) |
| `!PRECHARACTERTYPE` | 45 | 45 | Metadata(ignored-by-sheet) / metadata (consumed by the converter, not printed) |
| `PREDEITYDOMAIN` | 37 | 37 | Applies / filter once deity is a character fact |
| `!PRECAMPAIGN` | 20 | 20 | Metadata(ignored-by-sheet) / metadata (consumed by the converter, not printed) |
| `PREBASESIZEGTEQ` | 19 | 19 | Applies / filter (per-character-resolvable) |
| `PRESUBCLASS` | 17 | 17 | Applies / filter (per-character-resolvable) |
| `PREBASESIZEEQ` | 16 | 16 | Applies / filter (per-character-resolvable) |
| `PRECSKILL` | 9 | 9 | Applies / filter (per-character-resolvable) |
| `PREBASESIZELT` | 6 | 6 | Applies / filter (per-character-resolvable) |
| `PREVARNEQ` | 4 | 4 | Applies / situational (prints as words; never blocks) |
| `PRECHARACTERTYPE` | 2 | 2 | Metadata(ignored-by-sheet) / metadata (consumed by the converter, not printed) |
| `PREGENDER` | 2 | 2 | Applies / Words today; filter once the fact exists |
| `PRE` | 1 | 1 | Metadata(ignored-by-sheet) / metadata (consumed by the converter, not printed) |
| `PRECLASSLEVELMAX` | 1 | 1 | Applies / filter (per-character-resolvable) |
| `PREARMORTYPE` | 1 | 1 | Applies / situational (prints as words; never blocks) |
| `PRESPELLBOOK` | 1 | 1 | Applies / filter (per-character-resolvable) |
| `PRESIZELT` | 1 | 1 | Applies / filter (per-character-resolvable) |
| `PREREACHGTEQ` | 1 | 1 | Applies / situational (prints as words; never blocks) |
| `!PRECSKILL` | 1 | 1 | Applies / filter (per-character-resolvable) |
| `PRELANG` | 1 | 1 | Applies / filter (per-character-resolvable) |

Every one of them has a row above. None needs per-record handling; the two biggest
(`PRESIZEEQ` 885, `!PRECHARACTERTYPE` 45) are the most mechanical of all.

### 2.2 Which PRE types are which

- **Filter (per-character, mechanical):** `PREABILITY`, `PREMULT`, `PRECLASS`,
  `PRECLASSLEVELMAX`, `PRESUBCLASS`, `PRESTAT`, `PRESKILL`, `PRECSKILL`, `PRETOTALAB`,
  `PRECHECKBASE`, `PRELEVEL`, `PREPCLEVEL`, `PRELEVELMAX`, `PREHD`, `PRERACE`, `PREFACT`,
  `PRETEMPLATE`, all `PRESIZE*` and `PREBASESIZE*`, `PREDOMAIN`, `PREWEAPONPROF`,
  `PREPROFWITHSHIELD`, `PRESPELL`, `PRESPELLCAST`, `PRESPELLBOOK`, `PREMOVE`, `PREVISION`,
  `PRELANG`, every `!PRE` of these, and the level-alias / stat / flag classes of `PREVAR*`
  (about 90% of its 5,118 instances).
- **Filter once a character fact exists (Words today):** `PREALIGN`, `PREDEITY`,
  `PREDEITYDOMAIN`, `PREDEITYALIGN`, `PREGENDER`, `PREAGESET`, `PRESPELLTYPE`,
  `PRESPELLDESCRIPTOR`.
- **Situational (Words by design):** `PRETEXT`, `PREARMORTYPE`, `PREEQUIP`, `PREDR`,
  `PREHANDSGTEQ`, `PREREACHGTEQ`, `PREVARNEQ`, and the pool-counter / computed-total classes of
  `PREVAR*` (`*Count`, `*Taken`, `var("AC.Natural")`; about 7% of `PREVAR*` instances).
- **Metadata (never a gate):** `PRERULE`, `PRECAMPAIGN`/`!PRECAMPAIGN`,
  `PRECHARACTERTYPE`/`!PRECHARACTERTYPE` (→ `Always`/`Never`), `PRE:.CLEAR`, `!PREKIT`.
- **Refuse:** none of the PRE types. The single REFUSE row is `ABILITYCATEGORY` — a converter
  *input* that is not in the corpus, not a per-record judgment.

## 3. Expr and schema gaps (consolidated)

`technical-design.md §1`'s `Expr` lacks, and rows in this family need:

| gap | needed by | instances |
|---|---|---:|
| `Expr::SkillRanks(SkillId)` | `PRESKILL` | 198 (+113 in feat lists) |
| `Expr::BaseSize` (race-table size before templates/evolutions; `FACT:BaseSize` has 1,782 declarations) | `PREBASESIZEGTEQ/EQ/LT` | 41 |
| `Expr::BaseSave(Save)` | `PRECHECKBASE` | 2 |
| `Expr::HighestSpellLevel(SpellKind)` | `PRESPELLTYPE`, `PRECLASS:…SPELLCASTER` | 16 |
| `Expr::ChoiceCount(ChoiceId)` (picks made in a pool) | pool-counter `PREVARLT/LTEQ` | ~300 |
| `Expr::HeldCount { pool, tag }` (`count("ABILITIES",…)`) | 21 `PREVARLT` | 21 |
| comparison-as-value (`1+(X>2)`) | 5 `SELECT:` forms | 5 |

Schema-level gaps, outside `Expr`: `Applies` needs a **subject** (`Character` | `Item`) for the
1,252 `PRETYPE` item gates; `Holdable` needs `Proficiency`, `Language`, `Movement`, `Vision`,
`ClassSkill`, `Alignment`, `Deity*`, `Gender`, `AgeCategory` targets; the character record needs
the facts listed in §1. None of these is per-record work; each is one variant and one fact.

Converter-side components this family requires (tool side, allowed to read PCGen):
1. **Variable-setter index** over every `DEFINE:` / `BONUS:VAR` in the corpus, so `PREVAR*`
   resolves a name to `ClassLevel`, `AbilityScore`, `Holds{setter}` or `Words`. Shared with the
   map-formula lane; needs one owner.
2. **Fact-declaration index** over `FACT:`/`FACTSET:` so `PREFACT` resolves to the declaring rule.
3. **Key join** `(pool, key) → RuleId`, case-insensitive; measured hit rates in the `PREABILITY` row.
4. **Class level lines** from `lst_parser/class.rs::ClassLevelLine` — see hard case 2.
5. **`*_abilitycategories.lst` ingest** — see hard case 3.

## 4. The three hardest cases

### 4.1 `PREVAR*` — a variable name must become a fact, never survive as a name

`occult_adventures:class_feature:KB Selector`
(`pathfinder/paizo/roleplaying_game/occult_adventures/oa_abilities_class.lst:607`) carries ten
grants of the shape `ABILITY:Internal|AUTOMATIC|KB ~ 03d6|PREVAREQ:KB_Tier,03` and, on the same
record, `BONUS:VAR|KB_Tier|max(1,floor((KineticistLVL+1)/2))`. The gate names a variable; the
variable's only setter is a level formula on the same closure. The converter resolves the
setter through the F1..F9 parser and emits
`when: Compare { Max(Const(1), Div(Sum([ClassLevel(kineticist), Const(1)]), Const(2))) == Const(3) }`.
No `KB_Tier` in the output. The Kineticist wild talents
(`oa_abilities_class.lst:689`, `PREMULT:1,[PREVARGTEQ:KineticistLVL_Earth,4],[PREVARGTEQ:KineticistLVL_Water,4]`)
are the same shape one level up: `AtLeast{1, [Compare{ClassLevel(kineticist, element=earth) >= 4}, …]}`,
where `KineticistLVL_Earth` resolves to "kineticist level if the earth element is held, else 0" —
a `Mul(ClassLevel, Holds)` shape the setter index has to recognise. Measured split of 5,118
`PREVAR*` instances: 2,666 level-alias (52%), 56 `PreStatScore_`, ~2,100 flag/other set by a
rule, 300 pool counters, 40 computed-total reads. The first three classes resolve; the last two
are `Words` until `Expr::ChoiceCount` exists. **Confidence medium** because the 'other-var' bucket
(1,902 instances) was classified by name pattern, not by reading each setter; the index will
settle it mechanically.

### 4.2 Class `LEVEL` grants are not in `data/corpus`

`data/corpus/core_rulebook/class/fighter.json` holds eight tokens (`BONUS`×8 for BAB/saves,
`FACT`, `HD`, `TYPE`, `MAXLEVEL`, `SOURCEPAGE`, `DEFINE`, `ROLE`) and no level lines. The rows
`1<TAB>ABILITY:Fighter Class Feature|AUTOMATIC|Fighter ~ Bonus Feat` in `cr_classes.lst` are
parsed by `src/pcgen_import/lst_parser/class.rs` (`ClassLevelLine { level, raw_line }`) and
then dropped. Only 4 of 168 class records (the Unchained classes, e.g.
`data/corpus/pathfinder_unchained/class/monk_unchained_class.json`) carry a structured
`feature_grants` list — 65 entries with `min_level` and `suppressed_by_var`. 17,132 of 18,074
`class_feature` records name their class in `data.class` (12,109 non-DONE), but the level at
which the class hands them out lives only on the `.lst`. Without this edge **no class feature
has a holder** and the held-set fixpoint starts empty for every class. The mapping is trivial
(`Grant { by: Class { id, at_level }, when }`; `suppressed_by_var` → `Not(Holds{archetype rule})`);
the *input* is the problem. `technical-design.md §9` freezes `data/corpus`, so the converter
must either read the parser's level lines at ingest or the operator must rule that the class
records are regenerated with `feature_grants` for all 168. This needs a ruling before
AT-35-E2-001 starts.

### 4.3 The option pool for the choice filter is declared in a file we never ingested

`occult_adventures:class_feature:Medium Cleric Spell 6`
(`oa_abilities_class.lst:901`, `CHOOSE:SPELLS|CLASSLIST=Cleric[LEVELMIN=6;LEVELMAX=6]`) is the
easy shape: the option set is typed inline. The hard shape is every `CHOOSE:ABILITYSELECTION`
and every `BONUS:ABILITYPOOL|Rogue Talents|1` (map-bonus lane; 1,091 remainder units per
TOKEN-MODEL §4): the pool named `Rogue Talents` is defined in PCGen's `*_abilitycategories.lst`
as `ABILITYCATEGORY:Rogue Talent … CATEGORY:Special Ability TYPE:RogueTalent PRE…`. That file
has **zero** records in `data/corpus` and appears in `src/` only as a trap
(`src/pcgen_import/corpus_traps.rs`) and two hand tables. `OptionSet::Rules { pool, tags,
requires }` cannot be filled without it, so AT-35-E5-004 cannot list "which rogue talents may
this rogue take" for any pool. This is the one REFUSE row: not per record, a missing input.
The unblock is a ~30-line tool-side parser for four fields.

## 5. Oracle check

No PCGen exported *total* covers a gate. The nearest oracle is **held-set parity**:
`scripts/oracle_harness/charbuild-remainder.txt.ftl` exports `SA.COUNT` and
`SA.<n>.NAME` — the exact list of Special Ability rules PCGen judged held for a built character.
For the fixture roster, the set of rules whose `applies` evaluates `true` (restricted to
`pool = Special Ability`, `visible`) must equal that list. That single comparison exercises
every filter-grade row in this table at once. `PRESTAT` additionally has `STAT.<i>.SCORE` for
its inputs, and the trailing-`|PRE` addend gates are already oracle-traced in
`bonus_stack_reader.rs`.

## 6. Fixture used in the `sheet_line` examples

**F1**: human Fighter 3, lawful neutral, Medium, no deity. Str 15, Dex 13, Con 14, Int 10,
Wis 12, Cha 8. BAB +3, base Fort +3. Ranks: Ride 3, Intimidate 2. Feats: Power Attack,
Cleave, Weapon Focus (longsword). Languages: Common. Proficiencies: simple, martial, all
armor, shields. Every `sheet_line` in `prereq.json` is F1's verdict for that record's gate
(`held` / `excluded` / `included, not checked`), because an `Applies` row prints nothing by
itself — it decides whether the rule's own line reaches the sheet.

## 7. Open questions carried to the build cycle

1. Class level lines: parser-side read or corpus regeneration (§4.2). **Needs a ruling.**
2. `*_abilitycategories.lst` ingest for the pool definitions (§4.3).
3. One owner for the variable-setter index (this lane + map-formula).
4. Parameterised feat ids in `PREABILITY` (`Weapon Focus (Longsword)`: 543 of 1,790 FEAT items miss the key join) — base id + option, or per-option ids.
5. 20 `!PRECAMPAIGN` BotD2 deity duplicates: supersession register, not the sheet.
6. `PRETYPE` needs `Applies` to carry a subject; confirm with the equipment lane that the picker reads item tags.
