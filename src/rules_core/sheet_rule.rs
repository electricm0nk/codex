//! The sheet rule schema -- OUR rule record, schema v2 (SD-35 `technical-design.md` §1-§2,
//! `decisions.md` §1, §11, §15).
//!
//! Nothing in this module knows what a `.lst` file is, what a PCGen token looks like, what a
//! PCGen variable was called, or what `%1` meant. It is the data contract between the ingest-time
//! converter (`crate::pcgen_import::sheet_rule`, tool side, the only reader of PCGen) and the
//! live evaluator (this module's `evaluate`, AT-35-E2-002) that renders the line a player
//! writes on paper: one final number, dice in final form, or the rule's words.
//!
//! `data/sheet_rules/<book>/<kind>/<key>.json` holds a JSON array of [`SheetRule`] for one
//! corpus record (one rule for most records; a record that yields several sheet lines -- a
//! spell-like ability block, a double weapon, a save bonus to all three saves -- carries
//! siblings whose `id` is the record id plus a `#<suffix>`). `data/sheet_rules/_vars/<VarId>.json`
//! holds one [`VarTable`] per corpus variable. Every id inside is an opaque product id
//! (`<book>:<kind>:<slug>` for rules, `v` + 16 hex for variables); no source-format name
//! survives outside `provenance`.

use serde::{Deserialize, Serialize};

/// A rule id: `"<book>:<kind>:<slug>"`, the same id `docs/work-inventory.json` keys a unit by.
pub type RuleId = String;
/// An opaque converter-minted variable id: `"v"` + 16 hex of SHA-256 over the upper-cased
/// source name. The name -> id map is written tool-side only.
pub type VarId = String;
/// A class id: the slug of the class name (`"fighter"`, `"psychic_detective"`).
pub type ClassId = String;
/// A skill id: the slug of the skill name (`"perception"`, `"knowledge_arcana"`).
pub type SkillId = String;
/// A pool id: the slug of the record's category (`"special_ability"`, `"feat"`).
pub type PoolId = String;
/// A choice id: the id of the rule that offers the choice.
pub type ChoiceId = String;
/// An option id within a choice: the slug of the option's name.
pub type OptionId = String;
/// A game-rule tag: one dot-segment of the record's type facet (`"Racial Spell-Like Ability"`).
pub type Tag = String;
/// A race id: `<book>:race:<slug>` or the race's slug when the record is not in the corpus.
pub type RaceId = String;

/// Our rule record. No source token, formula string, or variable name in it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SheetRule {
    pub id: RuleId,
    /// Display name; codex-neutral when the name is product identity.
    pub label: String,
    /// The line's principal value.
    pub value: SheetValue,
    /// Second/third numbers on one line: uses + caster level + save DC.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub also: Vec<(ValueRole, SheetValue)>,
    /// The rule's words with typed slots, filled at evaluate time.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prose: Vec<ProseSegment>,
    /// May THIS character hold it.
    pub applies: Applies,
    /// What sheet total this value feeds, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<BonusTarget>,
    /// Stacking type for the cross-rule fold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bonus_type: Option<BonusType>,
    /// Printed on the sheet (`false`: held silently -- a selector or helper record).
    pub print: bool,
    pub pool: PoolId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    pub subject: Subject,
    pub repeatable: bool,
    /// Who hands it out.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub granted_by: Vec<Grant>,
    /// Choice-bearing records.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offers: Option<Choice>,
    /// What holding it does to the fact set.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grants: Vec<Effect>,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueRole {
    Uses { period: String },
    CasterLevel,
    SaveDc,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SheetValue {
    /// Exact evaluation; ONE truncation toward zero at this boundary.
    Number(Expr),
    /// `"1d8"` + `Const(2)` -> `"1d8+2"`; `size_steps` = held die-size steps.
    Dice { dice: String, modifier: Option<Expr>, size_steps: Option<Expr> },
    /// The list form indexed by `Size` ordinal, Fine..Colossal.
    DiceBySize([String; 9]),
    /// Nothing to compute; the prose IS the value.
    Text,
}

/// Words with typed holes. Text pieces are plain English; no marker, no token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProseSegment {
    pub family: ProseFamily,
    pub pieces: Vec<ProsePiece>,
    /// A segment's own gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applies: Option<Applies>,
    /// Print only the LAST segment of this family whose gate passes.
    #[serde(default)]
    pub pick_last: bool,
    /// Omit the line when every slot evaluates to 0.
    #[serde(default)]
    pub suppress_when_all_zero: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProseFamily {
    Desc,
    Benefit,
    Special,
    Aspect(String),
    WhenActive,
    StatBlock(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProsePiece {
    Text(String),
    /// `"DC "` + `Slot(...)` -> `"DC 15"`; truncated toward zero once per slot.
    Slot(Expr),
    /// The chosen option's name(s); unmade -> the choice lane's unmade words.
    ChoiceName(ChoiceId),
    /// `"1d6+"` with a modifier stays dice.
    Dice { dice: String, modifier: Option<Expr> },
}

/// Our expression form. Closed vocabulary; every variant names a fact the live character has.
/// Evaluation is exact (rational); nothing here truncates -- the `SheetValue` boundary does, once.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Const(i32),
    /// Total character level.
    Level,
    ClassLevel(ClassId),
    /// `ClassLevel(X)` + held bonus caster levels to X; `Holder` = the class the rule is held through.
    CasterLevel(ClassRef),
    SpellLevel,
    AbilityMod(Ability),
    AbilityScore(Ability),
    /// Race-table size ordinal + held size-step contributions.
    Size,
    /// Race-table size ordinal before templates/evolutions.
    BaseSize,
    /// The size modifier to AC/attack for the current `Size`.
    SizeMod,
    /// Racial / monster-class hit dice; 0 for every PC race.
    HitDice,
    BaseAttack,
    BaseSave(Save),
    SkillRanks(SkillId),
    SkillTotal(SkillId),
    HeldCount { pool: PoolId, filter: HeldFilter },
    ChallengeRating,
    Speed(MoveMode),
    HighestSpellLevel(SpellKind),
    /// Companion records: the master's level.
    MasterLevel,
    MasterVar(VarId),
    /// A corpus variable: folded over the HELD contributions in `_vars/<VarId>.json`.
    Var(VarId),
    Sum(Vec<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    /// Exact division.
    Div(Box<Expr>, Box<Expr>),
    Min(Box<Expr>, Box<Expr>),
    Max(Box<Expr>, Box<Expr>),
    Floor(Box<Expr>),
    Ceil(Box<Expr>),
    /// A term the player settles at pick time; unresolved -> the rule prints as words.
    Choice(ChoiceId),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClassRef {
    Class(ClassId),
    Holder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ability {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Save {
    Fortitude,
    Reflex,
    Will,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HeldFilter {
    Any,
    Tag(Tag),
    Rule(RuleId),
}

/// A movement mode, as a game-rule word (`"Walk"`, `"Fly"`, `"Swim"`).
pub type MoveMode = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpellKind {
    Any,
    Arcane,
    Divine,
    Psychic,
}

/// `data/sheet_rules/_vars/<VarId>.json` -- every contribution to one variable, corpus-wide.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VarTable {
    pub var: VarId,
    /// Rules whose closure declares the name; no held declarer -> the value is 0.
    pub declared_by: Vec<RuleId>,
    pub contributions: Vec<VarContribution>,
    pub provenance: VarProvenance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VarContribution {
    pub rule_id: RuleId,
    pub expr: Expr,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bonus_type: Option<BonusType>,
    pub when: Applies,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VarProvenance {
    /// Source rows (`path:line`) that declare or contribute to this name but belong to no
    /// corpus record -- an ingest remainder, neither refused nor zero.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outside_corpus_rows: Vec<String>,
}

/// A stacking type. `name` is a game-rule word (`"Racial"`, `"Base"`), never a token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BonusType {
    pub name: String,
    pub mode: StackMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StackMode {
    Plain,
    Stack,
    Replace,
}

/// Game-rule constant (Pathfinder): typed bonuses of these types stack; every other
/// same-type pair takes the max.
pub const STACKING_TYPES: [&str; 6] =
    ["Defense", "Dodge", "Circumstance", "Racial", "NotRanged", "NotFlatFooted"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BonusTarget {
    Ability(Ability),
    Skill(SkillId),
    /// A skill in a named situation; the situation prints on the line.
    SkillSituation { skill: SkillId, situation: String },
    Save(Save),
    /// The class save progression itself.
    BaseSave(Save),
    Ac,
    Attack,
    /// The class attack progression itself.
    BaseAttack,
    Damage(WeaponRef),
    Hp,
    Initiative,
    Cmb,
    Cmd,
    Speed(MoveMode),
    Vision(Tag),
    Dr,
    SpellDc(Scope),
    CasterLevel(Scope),
    /// Levels of spellcasting in a class (a monster that casts as a 9th-level sorcerer).
    SpellcastingLevels(ClassId),
    SpellCell { class: ClassId, level: u8 },
    SpellsKnown { class: ClassId, level: u8 },
    Pool(PoolId),
    /// Die-size steps on a weapon's damage.
    DamageSize(WeaponRef),
    WeaponAttack(WeaponRef),
    /// Every skill carrying the tag.
    SkillGroup(Tag),
    /// The thing the player chose on this rule (an ability score, a skill, a spell).
    Chosen(ChoiceId),
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WeaponRef {
    Any,
    Melee,
    Ranged,
    Named(String),
    Group(Tag),
    Chosen(ChoiceId),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Scope {
    All,
    Class(ClassId),
    School(String),
    Subschool(String),
    Descriptor(String),
    Spell(String),
    Chosen(ChoiceId),
}

/// The gate. Two-valued (Include / Exclude) plus `Situational`, which includes and prints
/// its condition on the line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Applies {
    Always,
    Never,
    All(Vec<Applies>),
    AtLeast { n: u8, of: Vec<Applies> },
    Not(Box<Applies>),
    Compare { lhs: Expr, op: Cmp, rhs: Expr },
    Holds { what: Holdable, count: u8 },
    Chosen { choice: ChoiceId, option: Option<OptionId> },
    /// `subject = Item`: the item carries at least `n` of the tags.
    ItemHas { tags: Vec<Tag>, n: u8 },
    /// Include; the condition prints on the line.
    Situational { text: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cmp {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Holdable {
    Rule(RuleId),
    RuleTag { pool: PoolId, tag: Tag },
    /// A rule named by the source that resolves to no corpus record (listed in `_defects/`);
    /// evaluates Exclude.
    MissingRule { pool: PoolId, name: String },
    Template(RuleId),
    Spell(RuleId),
    Race(RaceId),
    RaceType(Tag),
    RaceSubtype(Tag),
    Alignment(Vec<String>),
    AlignmentMatchesDeity,
    Deity(DeityRef),
    DeityInPantheon(String),
    DeityGrantsDomain(RuleId),
    DeityAlignment(Vec<String>),
    ClassSkill(SkillId),
    Proficiency(ProfRef),
    Language(String),
    Movement { mode: MoveMode, min: u16 },
    Vision(Tag),
    ClassTag(Tag),
    Gender(Tag),
    AgeCategory(Tag),
    /// A named fact declared by a `FactDeclare` effect on some held rule.
    Fact { name: String, value: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeityRef {
    Any,
    Named(RuleId),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProfRef {
    Weapon(String),
    WeaponGroup(Tag),
    WeaponTag(Tag),
    ArmorGroup(Tag),
    ShieldGroup(Tag),
    DeityFavoredWeapon,
    /// The weapon the player chose on this rule.
    Chosen(ChoiceId),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grant {
    pub by: Granter,
    pub when: Applies,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Granter {
    Rule(RuleId),
    Class { id: ClassId, at_level: u8 },
    /// A class spell list holds the rule at this spell level.
    ClassSpellList { id: ClassId, spell_level: u8 },
    Race(RaceId),
    Deity(RuleId),
    Choice(ChoiceId),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub id: ChoiceId,
    pub count: Expr,
    pub from: OptionSet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptionSet {
    Rules { pool: PoolId, tags: Vec<Tag>, requires: Applies },
    Skills(Vec<SkillId>),
    Weapons(Vec<String>),
    Spells { class: ClassId, levels: (u8, u8) },
    Languages(Vec<String>),
    Templates(Vec<RuleId>),
    Classes(Vec<ClassId>),
    Races(Vec<RaceId>),
    Schools,
    Deities,
    Domains,
    Equipment,
    FreeText,
    Number { min: Expr, max: Expr },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    FactGrant(Fact),
    FactRevoke(Fact),
    CountsAs(CountsAs),
    Waives(RuleId),
    Revokes(RuleId),
    FactDeclare { name: String, value: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Fact {
    ClassSkill(SkillId),
    ClassSkillGroup(Tag),
    /// The skill the player chose on this rule becomes a class skill.
    ClassSkillChosen(ChoiceId),
    CrossClassSkill(SkillId),
    Language(String),
    Proficiency(ProfRef),
    Equipment(String),
    /// A companion of this role is available.
    CompanionSlots { role: String, count: Expr },
    /// The fact the player chose on this rule (a language, a proficiency) is granted.
    Chosen(ChoiceId),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CountsAs {
    Rule(RuleId),
    Class(ClassId),
    Race(RaceId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Subject {
    Character,
    Item,
}

/// Where the rule came from. The only place a source path or row citation may appear.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Provenance {
    pub book: String,
    pub kind: String,
    /// `path:line` of every closure row read, in application order.
    pub closure_rows: Vec<String>,
    pub oracle_pin: String,
    pub converter_version: String,
    /// Product-identity handling on this rule: field families omitted and why.
    #[serde(default, skip_serializing_if = "PiStamp::is_empty")]
    pub pi: PiStamp,
    /// `"pfs"` when the record's own base row sits in an organized-play overlay file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overlay: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PiStamp {
    /// Fields omitted because the corpus record declares them product identity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub declared: Vec<String>,
    /// Fields omitted because a blacklist term appeared in the text.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub term_hits: Vec<String>,
}

impl PiStamp {
    pub fn is_empty(&self) -> bool {
        self.declared.is_empty() && self.term_hits.is_empty()
    }
}

/// The refusal report, `data/sheet_rules/_refused.json`: records the converter would not write
/// a rule for, counted per token type (never per unit).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RefusedReport {
    pub records: usize,
    pub converted: usize,
    pub refused: usize,
    /// token type -> number of refused records carrying it.
    pub by_token_type: std::collections::BTreeMap<String, usize>,
    pub entries: Vec<RefusedRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefusedRecord {
    pub id: RuleId,
    pub book: String,
    pub kind: String,
    /// Every token type that refused this record, sorted.
    pub token_types: Vec<String>,
}

// `mul` / `neg` / `div` are folding constructors, not operator overloads: `Expr` is data the
// converter writes and the evaluator reads, never arithmetic performed on `Expr` itself.
#[allow(clippy::should_implement_trait)]
impl Expr {
    /// `Sum` with flattening only: `Sum([a, Sum([b, c]), Const(0)])` -> `Sum([a, b, c])`; term
    /// order is kept and constants are NOT folded, so a converted shape reads exactly as the
    /// mapping table states it (`Sum([Const(10), Const(1), AbilityMod(Cha)])`). A one-term sum is
    /// the term itself; an empty sum is `Const(0)`.
    pub fn sum(terms: Vec<Expr>) -> Expr {
        let mut out: Vec<Expr> = Vec::new();
        for t in terms {
            match t {
                Expr::Sum(inner) => match Expr::sum(inner) {
                    Expr::Sum(v) => out.extend(v),
                    Expr::Const(0) => {}
                    other => out.push(other),
                },
                Expr::Const(0) => {}
                other => out.push(other),
            }
        }
        match out.len() {
            0 => Expr::Const(0),
            1 => out.pop().unwrap(),
            _ => Expr::Sum(out),
        }
    }

    /// `a * b` with constant folding.
    pub fn mul(a: Expr, b: Expr) -> Expr {
        match (&a, &b) {
            (Expr::Const(x), Expr::Const(y)) => Expr::Const((*x as i64 * *y as i64).clamp(i32::MIN as i64, i32::MAX as i64) as i32),
            (Expr::Const(1), _) => b,
            (_, Expr::Const(1)) => a,
            _ => Expr::Mul(Box::new(a), Box::new(b)),
        }
    }

    pub fn neg(a: Expr) -> Expr {
        match a {
            Expr::Const(c) => Expr::Const(c.saturating_neg()),
            other => Expr::Mul(Box::new(Expr::Const(-1)), Box::new(other)),
        }
    }

    pub fn div(a: Expr, b: Expr) -> Expr {
        Expr::Div(Box::new(a), Box::new(b))
    }

    pub fn min(a: Expr, b: Expr) -> Expr {
        Expr::Min(Box::new(a), Box::new(b))
    }

    pub fn max(a: Expr, b: Expr) -> Expr {
        Expr::Max(Box::new(a), Box::new(b))
    }

    /// Every `Var` id referenced anywhere inside this expression.
    pub fn var_ids(&self, out: &mut Vec<VarId>) {
        match self {
            Expr::Var(v) | Expr::MasterVar(v) => out.push(v.clone()),
            Expr::Sum(v) => v.iter().for_each(|e| e.var_ids(out)),
            Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Min(a, b) | Expr::Max(a, b) => {
                a.var_ids(out);
                b.var_ids(out);
            }
            Expr::Floor(a) | Expr::Ceil(a) => a.var_ids(out),
            _ => {}
        }
    }
}

impl Applies {
    /// `All` with flattening: `Always` terms drop, a `Never` term wins, one term is itself.
    pub fn all(terms: Vec<Applies>) -> Applies {
        let mut out = Vec::new();
        for t in terms {
            match t {
                Applies::Always => {}
                Applies::Never => return Applies::Never,
                Applies::All(inner) => out.extend(inner),
                other => out.push(other),
            }
        }
        match out.len() {
            0 => Applies::Always,
            1 => out.pop().unwrap(),
            _ => Applies::All(out),
        }
    }

    pub fn var_ids(&self, out: &mut Vec<VarId>) {
        match self {
            Applies::All(v) => v.iter().for_each(|a| a.var_ids(out)),
            Applies::AtLeast { of, .. } => of.iter().for_each(|a| a.var_ids(out)),
            Applies::Not(a) => a.var_ids(out),
            Applies::Compare { lhs, rhs, .. } => {
                lhs.var_ids(out);
                rhs.var_ids(out);
            }
            _ => {}
        }
    }
}

impl SheetValue {
    pub fn var_ids(&self, out: &mut Vec<VarId>) {
        match self {
            SheetValue::Number(e) => e.var_ids(out),
            SheetValue::Dice { modifier, size_steps, .. } => {
                if let Some(m) = modifier {
                    m.var_ids(out);
                }
                if let Some(s) = size_steps {
                    s.var_ids(out);
                }
            }
            _ => {}
        }
    }
}

impl SheetRule {
    /// Every `Var` id this rule references anywhere (value, also, prose slots, gates, grants).
    pub fn var_ids(&self) -> Vec<VarId> {
        let mut out = Vec::new();
        self.value.var_ids(&mut out);
        for (_, v) in &self.also {
            v.var_ids(&mut out);
        }
        for seg in &self.prose {
            for p in &seg.pieces {
                match p {
                    ProsePiece::Slot(e) => e.var_ids(&mut out),
                    ProsePiece::Dice { modifier: Some(m), .. } => m.var_ids(&mut out),
                    _ => {}
                }
            }
            if let Some(a) = &seg.applies {
                a.var_ids(&mut out);
            }
        }
        self.applies.var_ids(&mut out);
        for g in &self.granted_by {
            g.when.var_ids(&mut out);
        }
        if let Some(c) = &self.offers {
            c.count.var_ids(&mut out);
            if let OptionSet::Rules { requires, .. } = &c.from {
                requires.var_ids(&mut out);
            }
            if let OptionSet::Number { min, max } = &c.from {
                min.var_ids(&mut out);
                max.var_ids(&mut out);
            }
        }
        out.sort();
        out.dedup();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_flattens_and_drops_zero_terms() {
        let e = Expr::sum(vec![Expr::Const(10), Expr::Sum(vec![Expr::Const(1), Expr::AbilityMod(Ability::Cha)]), Expr::Const(0)]);
        assert_eq!(e, Expr::Sum(vec![Expr::Const(10), Expr::Const(1), Expr::AbilityMod(Ability::Cha)]));
        assert_eq!(Expr::sum(vec![]), Expr::Const(0));
        assert_eq!(Expr::sum(vec![Expr::Level]), Expr::Level);
    }

    #[test]
    fn applies_all_drops_always_and_short_circuits_never() {
        assert_eq!(Applies::all(vec![Applies::Always, Applies::Always]), Applies::Always);
        assert_eq!(Applies::all(vec![Applies::Always, Applies::Never]), Applies::Never);
        let h = Applies::Holds { what: Holdable::Rule("a:b:c".into()), count: 1 };
        assert_eq!(Applies::all(vec![Applies::Always, h.clone()]), h);
    }

    #[test]
    fn schema_round_trips_through_json() {
        let rule = SheetRule {
            id: "core_rulebook:race_trait:racial_sla_x".into(),
            label: "X".into(),
            value: SheetValue::Number(Expr::Sum(vec![Expr::Const(10), Expr::SpellLevel, Expr::AbilityMod(Ability::Cha)])),
            also: vec![(ValueRole::Uses { period: "day".into() }, SheetValue::Number(Expr::Const(1)))],
            prose: vec![ProseSegment {
                family: ProseFamily::Desc,
                pieces: vec![ProsePiece::Text("DC ".into()), ProsePiece::Slot(Expr::Var("v0123456789abcdef".into()))],
                applies: None,
                pick_last: false,
                suppress_when_all_zero: false,
            }],
            applies: Applies::Always,
            target: None,
            bonus_type: None,
            print: true,
            pool: "spell_like_ability".into(),
            tags: vec![],
            subject: Subject::Character,
            repeatable: false,
            granted_by: vec![],
            offers: None,
            grants: vec![],
            provenance: Provenance::default(),
        };
        let json = serde_json::to_string(&rule).unwrap();
        let back: SheetRule = serde_json::from_str(&json).unwrap();
        assert_eq!(back, rule);
        assert_eq!(rule.var_ids(), vec!["v0123456789abcdef".to_string()]);
    }
}
