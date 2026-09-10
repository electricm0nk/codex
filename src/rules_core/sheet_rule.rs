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
/// Mint a [`VarId`] for a source variable name: `"v"` + 16 hex of SHA-256 over the upper-cased
/// name (`technical-design.md` §1, `SYNTHESIS.md` A2). A pure hash -- it knows nothing about
/// what the name meant, which is why it can live on the schema side and be minted by either the
/// converter or a live reader that already holds a name (`record_vars`). The name -> id map is
/// written tool-side only.
pub fn var_id(name: &str) -> VarId {
    use sha2::{Digest, Sha256};
    let upper = name.trim().to_ascii_uppercase();
    let digest = Sha256::digest(upper.as_bytes());
    let hex: String = digest.iter().take(8).map(|b| format!("{b:02x}")).collect();
    format!("v{hex}")
}

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
    /// `All` with flattening: `Always` terms drop, a `Never` term wins, one term is itself,
    /// and a term already in the conjunction is not added twice.
    ///
    /// **Why the dedupe** (SD-35 `AT-35-E6-001`): the converter conjoins a record's own gate
    /// onto each line it emits (`sheet_rule/convert.rs`, the `Applies::all(vec![
    /// record_applies, line.applies])` call), and for a record whose only line-level gate IS
    /// the record gate that produced a conjunction stating every requirement exactly twice --
    /// **963 of the corpus's 1,830 gated feat records** were doubled that way
    /// (`python3 - <<'EOF'` over `data/sheet_rules/*/feat/*.json`, counting `All` gates whose
    /// term list equals itself repeated). It never changed a verdict -- `A and A` is `A` --
    /// but every consumer that REPORTS the gate printed each requirement twice, and
    /// `feat_prereqs` counts terms, so a record with three prerequisites reported six. A
    /// requirement stated twice is one requirement.
    pub fn all(terms: Vec<Applies>) -> Applies {
        let mut out: Vec<Applies> = Vec::new();
        let push = |out: &mut Vec<Applies>, t: Applies| {
            if !out.contains(&t) {
                out.push(t);
            }
        };
        for t in terms {
            match t {
                Applies::Always => {}
                Applies::Never => return Applies::Never,
                Applies::All(inner) => inner.into_iter().for_each(|i| push(&mut out, i)),
                other => push(&mut out, other),
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

// ===========================================================================
// The live evaluator (AT-35-E2-002, `technical-design.md` §2)
//
// Given a rule, the character's held set, the `_vars/` tables and the computed
// character, produce the line the player writes. A match over the enum; no
// parser, no string evaluation. Arithmetic is exact (rational) and truncates
// toward zero exactly once, at the `SheetValue` boundary.
// ===========================================================================

use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Add, Div, Mul};
use std::sync::OnceLock;

/// An exact rational: `num / den`, `den > 0`, reduced. Division by zero yields 0
/// (`technical-design.md` §2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rat {
    pub num: i64,
    pub den: i64,
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.max(1)
}

impl Rat {
    pub const ZERO: Rat = Rat { num: 0, den: 1 };

    pub fn int(n: i64) -> Rat {
        Rat { num: n, den: 1 }
    }

    fn normalized(num: i64, den: i64) -> Rat {
        if den == 0 {
            return Rat::ZERO;
        }
        let g = gcd(num, den);
        let (num, den) = (num / g, den / g);
        if den < 0 { Rat { num: -num, den: -den } } else { Rat { num, den } }
    }

    pub fn floor(self) -> Rat {
        Rat::int(self.num.div_euclid(self.den))
    }

    pub fn ceil(self) -> Rat {
        Rat::int(-((-self.num).div_euclid(self.den)))
    }

    /// Truncation toward zero -- the ONE truncation, applied at the sheet boundary.
    pub fn trunc(self) -> i64 {
        self.num / self.den
    }

    pub fn is_zero(self) -> bool {
        self.num == 0
    }
}

impl Add for Rat {
    type Output = Rat;
    fn add(self, o: Rat) -> Rat {
        Rat::normalized(
            self.num.saturating_mul(o.den).saturating_add(o.num.saturating_mul(self.den)),
            self.den.saturating_mul(o.den),
        )
    }
}

impl Mul for Rat {
    type Output = Rat;
    fn mul(self, o: Rat) -> Rat {
        Rat::normalized(self.num.saturating_mul(o.num), self.den.saturating_mul(o.den))
    }
}

/// Exact division; a zero divisor yields 0 (`technical-design.md` §2).
impl Div for Rat {
    type Output = Rat;
    fn div(self, o: Rat) -> Rat {
        if o.num == 0 {
            return Rat::ZERO;
        }
        Rat::normalized(self.num.saturating_mul(o.den), self.den.saturating_mul(o.num))
    }
}

impl PartialOrd for Rat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.num as i128 * other.den as i128).cmp(&(other.num as i128 * self.den as i128))
    }
}

/// The three forms a sheet line takes (`decisions.md` §1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SheetLineValue {
    /// One final number.
    Resolved(i32),
    /// Dice in final form: `"1d8+2"`.
    Dice(String),
    /// The rule's words; nothing to compute, or a term the character does not settle yet.
    Words,
}

/// The line the player writes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SheetLine {
    pub id: RuleId,
    /// The record kind (`"feat"`, `"class_feature"`, ...) -- the middle segment of `id`; the
    /// sheet section groups by it.
    pub kind: String,
    pub label: String,
    pub value: SheetLineValue,
    /// `value` as the player writes it: `"+2"` for a bonus, `"15"` for a plain number,
    /// `"1d8+2"` for dice, `""` for words.
    pub printed: String,
    /// Second/third numbers on the line, each already printed: `"6/day"`, `"CL 5"`, `"DC 15"`.
    pub also: Vec<(String, SheetLineValue)>,
    /// Slots filled, segments joined; nothing unsubstituted.
    pub prose: String,
    /// `Situational` text: "when jumping", "while raging".
    pub condition: Option<String>,
}

/// The whole `data/sheet_rules/` package as the live side reads it: every rule by id, every
/// variable table by id, and the indexes the held-set fixpoint needs. Built by
/// `corpus_loader::load_sheet_rules`.
#[derive(Debug, Default)]
pub struct SheetRulePackage {
    pub rules: BTreeMap<RuleId, SheetRule>,
    pub vars: BTreeMap<VarId, VarTable>,
    /// `kind -> slug -> rule ids` (the same slug may exist in several books).
    by_kind_slug: BTreeMap<String, BTreeMap<String, Vec<RuleId>>>,
    /// `granter rule id -> rules it grants` (`Granter::Rule`).
    grants_from_rule: BTreeMap<RuleId, Vec<RuleId>>,
    /// Rules with a fact-based granter (`Class`, `Race`, `Deity`, `Choice`).
    fact_granted: Vec<RuleId>,
}

/// `"Trait ~ Magical Knack"` -> `"trait_magical_knack"`; the slug the converter names a
/// record file by. Lower-case, every non-alphanumeric run is one `_`, ends trimmed.
pub fn slug(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut pending = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending && !out.is_empty() {
                out.push('_');
            }
            pending = false;
            out.push(ch.to_ascii_lowercase());
        } else {
            pending = true;
        }
    }
    out
}

/// `"<book>:<kind>:<slug>"` -> `(book, kind, slug)`; a `#suffix` sibling keeps its parent's slug.
pub fn split_rule_id(id: &str) -> (&str, &str, &str) {
    let mut parts = id.splitn(3, ':');
    let book = parts.next().unwrap_or("");
    let kind = parts.next().unwrap_or("");
    let rest = parts.next().unwrap_or("");
    let slug = rest.split('#').next().unwrap_or(rest);
    (book, kind, slug)
}

impl SheetRulePackage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_rule(&mut self, rule: SheetRule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    pub fn insert_var(&mut self, table: VarTable) {
        self.vars.insert(table.var.clone(), table);
    }

    /// Build the indexes; call once after the last insert.
    pub fn finish(&mut self) {
        self.by_kind_slug.clear();
        self.grants_from_rule.clear();
        self.fact_granted.clear();
        for (id, rule) in &self.rules {
            let (_, kind, slug) = split_rule_id(id);
            if !id.contains('#') {
                self.by_kind_slug
                    .entry(kind.to_string())
                    .or_default()
                    .entry(slug.to_string())
                    .or_default()
                    .push(id.clone());
            }
            let mut fact_based = false;
            for grant in &rule.granted_by {
                match &grant.by {
                    Granter::Rule(granter) => {
                        self.grants_from_rule.entry(granter.clone()).or_default().push(id.clone())
                    }
                    Granter::ClassSpellList { .. } => {}
                    Granter::Class { .. } | Granter::Race(_) | Granter::Deity(_) | Granter::Choice(_) => {
                        fact_based = true
                    }
                }
            }
            if fact_based {
                self.fact_granted.push(id.clone());
            }
        }
    }

    pub fn rule(&self, id: &str) -> Option<&SheetRule> {
        self.rules.get(id)
    }

    /// The rule of `kind` with this slug. When several books carry the slug, `core_rulebook`
    /// wins, then the lexicographically first book id -- one line per record, never one per
    /// printing.
    pub fn find(&self, kind: &str, slug: &str) -> Option<&RuleId> {
        let ids = self.by_kind_slug.get(kind)?.get(slug)?;
        ids.iter()
            .find(|id| id.starts_with("core_rulebook:"))
            .or_else(|| ids.iter().min())
    }

    /// Every principal rule (no `#` sibling suffix) of `kind`, plus each one's siblings.
    pub fn rules_of_kind<'a>(&'a self, kind: &'a str) -> impl Iterator<Item = &'a SheetRule> + 'a {
        self.rules.values().filter(move |r| split_rule_id(&r.id).1 == kind)
    }

    /// The siblings a record file carries alongside its principal rule (`id#suffix`).
    fn siblings_of<'a>(&'a self, id: &str) -> Vec<&'a RuleId> {
        let prefix = format!("{id}#");
        self.rules.range(prefix.clone()..).take_while(|(k, _)| k.starts_with(&prefix)).map(|(k, _)| k).collect()
    }
}

/// Everything the character record and the chassis computation say about this character, as
/// the leaves of `Expr` and `Applies` read it. Built by `CharacterFacts::from_character`;
/// a caller that knows more (the desktop's resolved race) fills the rest in.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CharacterFacts {
    /// Total character level.
    pub level: i64,
    /// `(class slug, level)`.
    pub class_levels: Vec<(ClassId, i64)>,
    /// Str, Dex, Con, Int, Wis, Cha.
    pub ability_scores: [i64; 6],
    pub ability_mods: [i64; 6],
    /// Size ordinal, Fine = 0 .. Colossal = 8 (Medium = 4).
    pub size: i64,
    pub base_size: i64,
    pub hit_dice: i64,
    pub base_attack: i64,
    /// Fortitude, Reflex, Will.
    pub base_saves: [i64; 3],
    pub skill_ranks: BTreeMap<SkillId, i64>,
    pub skill_totals: BTreeMap<SkillId, i64>,
    /// `"Walk"` -> feet.
    pub speeds: BTreeMap<MoveMode, i64>,
    pub challenge_rating: i64,
    pub highest_spell_level: i64,
    pub master_level: i64,
    /// Choice id -> chosen `(option id, option name)`s.
    pub choices: BTreeMap<ChoiceId, Vec<(OptionId, String)>>,
    /// Race slug (`"half_orc"`).
    pub race: Option<String>,
    pub race_types: BTreeSet<Tag>,
    pub race_subtypes: BTreeSet<Tag>,
    pub alignment: Option<String>,
    pub deity: Option<RuleId>,
    pub languages: BTreeSet<String>,
    pub class_skills: BTreeSet<SkillId>,
    /// Weapon names / group tags the character is proficient with.
    pub proficiencies: BTreeSet<String>,
    pub visions: BTreeSet<Tag>,
    pub class_tags: BTreeSet<Tag>,
    pub gender: Option<Tag>,
    pub age_category: Option<Tag>,
}

fn ability_index(a: Ability) -> usize {
    match a {
        Ability::Str => 0,
        Ability::Dex => 1,
        Ability::Con => 2,
        Ability::Int => 3,
        Ability::Wis => 4,
        Ability::Cha => 5,
    }
}

fn save_index(s: Save) -> usize {
    match s {
        Save::Fortitude => 0,
        Save::Reflex => 1,
        Save::Will => 2,
    }
}

/// `"class:fighter"` -> `"fighter"`; `"race:half-orc"` -> `"half_orc"`; `"skill:climb"` -> `"climb"`.
pub fn id_slug(id: &str) -> String {
    slug(id.rsplit(':').next().unwrap_or(id))
}

/// Size ordinal (Fine = 0 .. Colossal = 8) of the size modifier table row.
pub fn size_ordinal(size: crate::rules_core::size::SizeCategory) -> i64 {
    use crate::rules_core::size::SizeCategory as S;
    match size {
        S::Fine => 0,
        S::Diminutive => 1,
        S::Tiny => 2,
        S::Small => 3,
        S::Medium => 4,
        S::Large => 5,
        S::Huge => 6,
        S::Gargantuan => 7,
        S::Colossal => 8,
    }
}

fn size_ac_modifier(ordinal: i64) -> i64 {
    match ordinal {
        0 => 8,
        1 => 4,
        2 => 2,
        3 => 1,
        4 => 0,
        5 => -1,
        6 => -2,
        7 => -4,
        _ => -8,
    }
}

impl CharacterFacts {
    /// The facts `compute_pilot_base_chassis`'s input and output already carry. Size comes from
    /// the CRB race table for the seven CRB races (`race_tables::race_size`) and is Medium
    /// otherwise; walk speed is read from the chassis' own `race.<slug>.trait_bundle.speed`
    /// record when it emitted one. Alignment, deity, gender, age category, languages, class
    /// skills and proficiencies are facts the character record does not yet carry: a gate over
    /// them evaluates Exclude (`technical-design.md` §2), never a guessed value.
    pub fn from_character(
        input: &crate::rules_core::character_input::CharacterInput,
        computation: &crate::rules_core::pilot_compute::PilotBaseChassisComputation,
    ) -> CharacterFacts {
        use crate::rules_core::rules_tables::crb::race_tables::{race_id_from_token, race_size};
        let chosen = &input.chosen;
        let class_levels: Vec<(ClassId, i64)> =
            chosen.class_levels.iter().map(|c| (id_slug(&c.class_id), i64::from(c.level))).collect();
        let level = class_levels.iter().map(|(_, l)| *l).sum();
        let s = &chosen.ability_scores;
        let m = &computation.ability_modifiers;
        let race = Some(id_slug(&chosen.race_id));
        let size = race_id_from_token(&chosen.race_id).map(|r| size_ordinal(race_size(r))).unwrap_or(4);
        let mut skill_ranks = BTreeMap::new();
        for a in &chosen.skill_allocations {
            *skill_ranks.entry(id_slug(&a.skill_id)).or_insert(0) += i64::from(a.ranks);
        }
        let mut skill_totals = skill_ranks.clone();
        let sel = &computation.selected_skill_modifiers;
        for (skill, modifier) in [("climb", sel.climb), ("intimidate", sel.intimidate), ("swim", sel.swim)] {
            if modifier != 0 {
                *skill_totals.entry(skill.to_string()).or_insert(0) += i64::from(modifier);
            }
        }
        let mut speeds = BTreeMap::new();
        if let Some(speed) = computation
            .explanations
            .iter()
            .find(|e| e.id.starts_with("race.") && e.id.ends_with(".trait_bundle.speed"))
        {
            speeds.insert("Walk".to_string(), i64::from(speed.value));
        }
        let mut choices: BTreeMap<ChoiceId, Vec<(OptionId, String)>> = BTreeMap::new();
        for c in &chosen.selected_choices {
            choices
                .entry(c.choice_set_id.clone())
                .or_default()
                .push((id_slug(&c.selection_id), c.selection_id.rsplit(':').next().unwrap_or("").to_string()));
        }
        CharacterFacts {
            level,
            class_levels,
            ability_scores: [s.strength, s.dexterity, s.constitution, s.intelligence, s.wisdom, s.charisma]
                .map(i64::from),
            ability_mods: [m.strength, m.dexterity, m.constitution, m.intelligence, m.wisdom, m.charisma]
                .map(i64::from),
            size,
            base_size: size,
            hit_dice: 0,
            base_attack: i64::from(computation.base_attack_bonus),
            base_saves: [computation.base_saves.fortitude, computation.base_saves.reflex, computation.base_saves.will]
                .map(i64::from),
            skill_ranks,
            skill_totals,
            speeds,
            challenge_rating: 0,
            highest_spell_level: 0,
            master_level: 0,
            choices,
            race,
            ..CharacterFacts::default()
        }
    }
}

/// What the character explicitly holds, by the ids the record carries. Resolved to rule ids
/// by `(kind, slug)`; the fixpoint grows it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeldSeed {
    pub race: Option<String>,
    /// `(class slug, level)`.
    pub classes: Vec<(String, i64)>,
    pub feats: Vec<String>,
    pub traits: Vec<String>,
    pub equipment: Vec<String>,
    pub spells: Vec<String>,
    pub skills: Vec<String>,
    /// Applied racial-trait keys (`"Dwarf ~ Minesight"`), as the race resolver names them.
    pub race_traits: Vec<String>,
    /// `(class slug, chassis explanation id)` for every class-feature record the chassis
    /// already grounds for a held class -- `class_feature.fighter.bravery`.
    pub class_features: Vec<(String, String)>,
    /// Rule ids held outright.
    pub rule_ids: Vec<RuleId>,
}

impl HeldSeed {
    /// The seed the character record and the chassis computation give directly. Racial traits
    /// need the race resolver; a caller that has it appends `race_traits`.
    pub fn from_character(
        input: &crate::rules_core::character_input::CharacterInput,
        computation: &crate::rules_core::pilot_compute::PilotBaseChassisComputation,
    ) -> HeldSeed {
        let chosen = &input.chosen;
        let classes: Vec<(String, i64)> =
            chosen.class_levels.iter().map(|c| (id_slug(&c.class_id), i64::from(c.level))).collect();
        let mut class_features = Vec::new();
        for e in &computation.explanations {
            let Some(rest) = e.id.strip_prefix("class_feature.") else { continue };
            if rest.ends_with(".unsupported") {
                continue;
            }
            let owner = classes
                .iter()
                .map(|(c, _)| c.clone())
                .find(|c| rest.split('.').any(|seg| seg == c))
                .unwrap_or_else(|| classes.first().map(|(c, _)| c.clone()).unwrap_or_default());
            class_features.push((owner, e.id.clone()));
        }
        HeldSeed {
            race: Some(id_slug(&chosen.race_id)),
            classes,
            feats: chosen.selected_feats.iter().map(|f| id_slug(f)).collect(),
            traits: chosen.selected_traits.iter().map(|t| id_slug(t)).collect(),
            equipment: chosen.equipment_selections.iter().map(|e| id_slug(&e.item_id)).collect(),
            spells: chosen.spells_selected.iter().map(|s| id_slug(&s.spell_id)).collect(),
            skills: chosen.skill_allocations.iter().filter(|a| a.ranks > 0).map(|a| id_slug(&a.skill_id)).collect(),
            race_traits: Vec::new(),
            class_features,
            rule_ids: Vec::new(),
        }
    }
}

/// One held rule and how it got there.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeldRule {
    /// The class the rule is held through, for `ClassRef::Holder`.
    pub holder_class: Option<ClassId>,
    /// The spell level it is held at, for `Expr::SpellLevel`.
    pub spell_level: Option<i64>,
    /// The granting rule, when the fixpoint added it.
    pub via: Option<RuleId>,
}

/// The held set: the fixpoint over the seed (`technical-design.md` §2).
#[derive(Debug, Clone, Default)]
pub struct HeldSet {
    pub rules: BTreeMap<RuleId, HeldRule>,
    /// `FactDeclare` facts from held rules.
    pub declared: BTreeSet<(String, String)>,
    /// `CountsAs(Rule)` from held rules.
    pub counts_as: BTreeSet<RuleId>,
    /// `Waives` / `Revokes` targets from held rules: dropped from the sheet.
    pub removed: BTreeSet<RuleId>,
    /// The classes the character holds by level (`HeldSeed::classes`, level >= 1). A `class`
    /// kind rule id (`<book>:class:<slug>`) counts as held for one of these whether or not the
    /// package carries the class record -- the character holds the class by having levels in
    /// it, and the class-level `Var`s the converter declares on the class record fold from
    /// `ClassLevel`, which the facts carry.
    pub classes: BTreeSet<ClassId>,
}

impl HeldSet {
    pub fn holds(&self, id: &str) -> bool {
        if self.removed.contains(id) {
            return false;
        }
        if self.rules.contains_key(id) || self.counts_as.contains(id) {
            return true;
        }
        let (_, kind, slug) = split_rule_id(id);
        kind == "class" && !id.contains('#') && self.classes.contains(slug)
    }
}

/// How a gate came out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gate {
    Include,
    Exclude,
    /// Include; the condition prints on the line.
    Situational(String),
}

impl Gate {
    pub fn includes(&self) -> bool {
        !matches!(self, Gate::Exclude)
    }
}

/// Per-evaluation context: the rule being evaluated, and what its holder settles.
#[derive(Debug, Clone, Default)]
pub struct EvalContext {
    pub holder_class: Option<ClassId>,
    pub spell_level: i64,
    /// Tags of the item an `Item`-subject rule sits on; empty for a character rule.
    pub item_tags: Vec<Tag>,
}

struct Evaluator<'a> {
    package: &'a SheetRulePackage,
    held: &'a HeldSet,
    facts: &'a CharacterFacts,
    ctx: EvalContext,
    /// Set when a `Choice` term had no selection: the value prints as words.
    unresolved: std::cell::Cell<bool>,
    /// Variables under evaluation, to stop a self-referential contribution recursing.
    visiting: std::cell::RefCell<Vec<VarId>>,
}

impl<'a> Evaluator<'a> {
    fn new(package: &'a SheetRulePackage, held: &'a HeldSet, facts: &'a CharacterFacts, ctx: EvalContext) -> Self {
        Evaluator { package, held, facts, ctx, unresolved: std::cell::Cell::new(false), visiting: std::cell::RefCell::new(Vec::new()) }
    }

    fn class_level(&self, class: &str) -> i64 {
        self.facts.class_levels.iter().filter(|(c, _)| c == class).map(|(_, l)| *l).sum()
    }

    fn expr(&self, e: &Expr) -> Rat {
        match e {
            Expr::Const(c) => Rat::int(i64::from(*c)),
            Expr::Level => Rat::int(self.facts.level),
            Expr::ClassLevel(c) => Rat::int(self.class_level(c)),
            Expr::CasterLevel(ClassRef::Class(c)) => Rat::int(self.class_level(c)),
            Expr::CasterLevel(ClassRef::Holder) => {
                Rat::int(self.ctx.holder_class.as_deref().map(|c| self.class_level(c)).unwrap_or(0))
            }
            Expr::SpellLevel => Rat::int(self.ctx.spell_level),
            Expr::AbilityMod(a) => Rat::int(self.facts.ability_mods[ability_index(*a)]),
            Expr::AbilityScore(a) => Rat::int(self.facts.ability_scores[ability_index(*a)]),
            Expr::Size => Rat::int(self.facts.size),
            Expr::BaseSize => Rat::int(self.facts.base_size),
            Expr::SizeMod => Rat::int(size_ac_modifier(self.facts.size)),
            Expr::HitDice => Rat::int(self.facts.hit_dice),
            Expr::BaseAttack => Rat::int(self.facts.base_attack),
            Expr::BaseSave(s) => Rat::int(self.facts.base_saves[save_index(*s)]),
            Expr::SkillRanks(s) => Rat::int(self.facts.skill_ranks.get(s).copied().unwrap_or(0)),
            Expr::SkillTotal(s) => Rat::int(self.facts.skill_totals.get(s).copied().unwrap_or(0)),
            Expr::HeldCount { pool, filter } => Rat::int(self.held_count(pool, filter)),
            Expr::ChallengeRating => Rat::int(self.facts.challenge_rating),
            Expr::Speed(mode) => Rat::int(self.facts.speeds.get(mode).copied().unwrap_or(0)),
            Expr::HighestSpellLevel(_) => Rat::int(self.facts.highest_spell_level),
            Expr::MasterLevel => Rat::int(self.facts.master_level),
            Expr::MasterVar(_) => Rat::ZERO,
            Expr::Var(v) => self.var(v),
            Expr::Sum(terms) => terms.iter().fold(Rat::ZERO, |acc, t| acc.add(self.expr(t))),
            Expr::Mul(a, b) => self.expr(a).mul(self.expr(b)),
            Expr::Div(a, b) => self.expr(a).div(self.expr(b)),
            Expr::Min(a, b) => self.expr(a).min(self.expr(b)),
            Expr::Max(a, b) => self.expr(a).max(self.expr(b)),
            Expr::Floor(a) => self.expr(a).floor(),
            Expr::Ceil(a) => self.expr(a).ceil(),
            Expr::Choice(id) => match self.facts.choices.get(id).and_then(|v| v.first()) {
                Some((option, _)) => match option.parse::<i64>() {
                    Ok(n) => Rat::int(n),
                    Err(_) => {
                        self.unresolved.set(true);
                        Rat::ZERO
                    }
                },
                None => {
                    self.unresolved.set(true);
                    Rat::ZERO
                }
            },
        }
    }

    fn held_count(&self, pool: &str, filter: &HeldFilter) -> i64 {
        self.held
            .rules
            .keys()
            .filter(|id| !self.held.removed.contains(*id))
            .filter_map(|id| self.package.rule(id))
            .filter(|r| pool.is_empty() || r.pool == pool)
            .filter(|r| match filter {
                HeldFilter::Any => true,
                HeldFilter::Tag(t) => r.tags.iter().any(|x| x == t),
                HeldFilter::Rule(id) => &r.id == id,
            })
            .count() as i64
    }

    /// The `Var` contribution fold (`technical-design.md` §2): no held declarer -> 0; else every
    /// held contribution whose `when` includes, folded by bonus type -- untyped, `Stack`,
    /// negative and `STACKING_TYPES` contributions sum; the rest of one type take the max; a
    /// type that also has `Replace` contributions is `max(plain + stack, replace)`.
    fn var(&self, id: &str) -> Rat {
        let Some(table) = self.package.vars.get(id) else { return Rat::ZERO };
        if !table.declared_by.iter().any(|d| self.held.holds(d)) {
            return Rat::ZERO;
        }
        if self.visiting.borrow().iter().any(|v| v == id) {
            return Rat::ZERO;
        }
        self.visiting.borrow_mut().push(id.to_string());
        let mut summed = Rat::ZERO;
        let mut plain_max: BTreeMap<&str, Rat> = BTreeMap::new();
        let mut replace_max: BTreeMap<&str, Rat> = BTreeMap::new();
        for c in &table.contributions {
            if !self.held.holds(&c.rule_id) {
                continue;
            }
            let holder = self.held.rules.get(&c.rule_id).cloned().unwrap_or_default();
            let inner = Evaluator::new(
                self.package,
                self.held,
                self.facts,
                EvalContext { holder_class: holder.holder_class, spell_level: holder.spell_level.unwrap_or(0), item_tags: Vec::new() },
            );
            *inner.visiting.borrow_mut() = self.visiting.borrow().clone();
            if !inner.applies(&c.when).includes() {
                continue;
            }
            let value = inner.expr(&c.expr);
            match &c.bonus_type {
                None => summed = summed.add(value),
                Some(t) if t.mode == StackMode::Stack || value < Rat::ZERO || STACKING_TYPES.contains(&t.name.as_str()) => {
                    summed = summed.add(value)
                }
                Some(t) if t.mode == StackMode::Replace => {
                    let e = replace_max.entry(t.name.as_str()).or_insert(value);
                    *e = (*e).max(value);
                }
                Some(t) => {
                    let e = plain_max.entry(t.name.as_str()).or_insert(value);
                    *e = (*e).max(value);
                }
            }
        }
        self.visiting.borrow_mut().pop();
        let mut total = summed;
        for (name, plain) in &plain_max {
            match replace_max.get(name) {
                Some(replace) => total = total.add((*plain).max(*replace)),
                None => total = total.add(*plain),
            }
        }
        for (name, replace) in &replace_max {
            if !plain_max.contains_key(name) {
                total = total.add(*replace);
            }
        }
        total
    }

    fn compare(&self, lhs: &Expr, op: Cmp, rhs: &Expr) -> bool {
        let (l, r) = (self.expr(lhs), self.expr(rhs));
        match op {
            Cmp::Eq => l == r,
            Cmp::Ne => l != r,
            Cmp::Lt => l < r,
            Cmp::Lte => l <= r,
            Cmp::Gt => l > r,
            Cmp::Gte => l >= r,
        }
    }

    fn holds(&self, what: &Holdable, count: u8) -> bool {
        let count = i64::from(count.max(1));
        match what {
            Holdable::Rule(id) | Holdable::Template(id) | Holdable::Spell(id) => self.held.holds(id),
            Holdable::RuleTag { pool, tag } => self.held_count(pool, &HeldFilter::Tag(tag.clone())) >= count,
            Holdable::MissingRule { .. } => false,
            Holdable::Race(id) => self.facts.race.as_deref() == Some(slug(id).as_str()) || self.facts.race.as_deref() == Some(id.as_str()),
            Holdable::RaceType(t) => self.facts.race_types.contains(t),
            Holdable::RaceSubtype(t) => self.facts.race_subtypes.contains(t),
            Holdable::Alignment(list) => self.facts.alignment.as_ref().is_some_and(|a| list.contains(a)),
            Holdable::AlignmentMatchesDeity => false,
            Holdable::Deity(DeityRef::Any) => self.facts.deity.is_some(),
            Holdable::Deity(DeityRef::Named(id)) => self.facts.deity.as_ref() == Some(id),
            Holdable::DeityInPantheon(_) | Holdable::DeityGrantsDomain(_) | Holdable::DeityAlignment(_) => false,
            Holdable::ClassSkill(s) => self.facts.class_skills.contains(s),
            Holdable::Proficiency(p) => match p {
                ProfRef::Weapon(w) => self.facts.proficiencies.contains(w),
                ProfRef::WeaponGroup(t) | ProfRef::WeaponTag(t) | ProfRef::ArmorGroup(t) | ProfRef::ShieldGroup(t) => {
                    self.facts.proficiencies.contains(t)
                }
                ProfRef::DeityFavoredWeapon => false,
                ProfRef::Chosen(c) => self.facts.choices.get(c).is_some_and(|v| !v.is_empty()),
            },
            Holdable::Language(l) => self.facts.languages.contains(l),
            Holdable::Movement { mode, min } => self.facts.speeds.get(mode).copied().unwrap_or(0) >= i64::from(*min),
            Holdable::Vision(t) => self.facts.visions.contains(t),
            Holdable::ClassTag(t) => self.facts.class_tags.contains(t),
            Holdable::Gender(t) => self.facts.gender.as_ref() == Some(t),
            Holdable::AgeCategory(t) => self.facts.age_category.as_ref() == Some(t),
            Holdable::Fact { name, value } => self.held.declared.contains(&(name.clone(), value.clone())),
        }
    }

    /// Two-valued plus `Situational` (`technical-design.md` §2). A leaf over a fact the
    /// character record does not carry evaluates Exclude.
    fn applies(&self, a: &Applies) -> Gate {
        match a {
            Applies::Always => Gate::Include,
            Applies::Never => Gate::Exclude,
            Applies::All(terms) => {
                let mut conditions = Vec::new();
                for t in terms {
                    match self.applies(t) {
                        Gate::Exclude => return Gate::Exclude,
                        Gate::Situational(text) => conditions.push(text),
                        Gate::Include => {}
                    }
                }
                if conditions.is_empty() { Gate::Include } else { Gate::Situational(conditions.join("; ")) }
            }
            Applies::AtLeast { n, of } => {
                let mut included = 0usize;
                let mut conditions = Vec::new();
                for t in of {
                    match self.applies(t) {
                        Gate::Exclude => {}
                        Gate::Situational(text) => {
                            included += 1;
                            conditions.push(text);
                        }
                        Gate::Include => included += 1,
                    }
                }
                if included < usize::from(*n) {
                    Gate::Exclude
                } else if conditions.is_empty() {
                    Gate::Include
                } else {
                    Gate::Situational(conditions.join("; "))
                }
            }
            Applies::Not(inner) => {
                if self.applies(inner).includes() { Gate::Exclude } else { Gate::Include }
            }
            Applies::Compare { lhs, op, rhs } => {
                if self.compare(lhs, *op, rhs) { Gate::Include } else { Gate::Exclude }
            }
            Applies::Holds { what, count } => {
                if self.holds(what, *count) { Gate::Include } else { Gate::Exclude }
            }
            Applies::Chosen { choice, option } => {
                let chosen = self.facts.choices.get(choice);
                let ok = match option {
                    None => chosen.is_some_and(|v| !v.is_empty()),
                    Some(o) => chosen.is_some_and(|v| v.iter().any(|(id, _)| id == o)),
                };
                if ok { Gate::Include } else { Gate::Exclude }
            }
            Applies::ItemHas { tags, n } => {
                let have = tags.iter().filter(|t| self.ctx.item_tags.contains(t)).count();
                if have >= usize::from(*n) && !self.ctx.item_tags.is_empty() { Gate::Include } else { Gate::Exclude }
            }
            Applies::Situational { text } => Gate::Situational(text.clone()),
        }
    }

    fn dice(&self, dice: &str, modifier: Option<&Expr>, size_steps: Option<&Expr>) -> String {
        let mut die = dice.trim().to_string();
        if let Some(steps) = size_steps {
            let n = self.expr(steps).trunc();
            die = step_die(&die, n);
        }
        let modifier = modifier.map(|m| self.expr(m).trunc()).unwrap_or(0);
        fold_dice_modifier(&die, modifier)
    }

    fn value(&self, v: &SheetValue) -> SheetLineValue {
        match v {
            SheetValue::Number(e) => {
                let r = self.expr(e);
                if self.unresolved.replace(false) {
                    SheetLineValue::Words
                } else {
                    SheetLineValue::Resolved(r.trunc().clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32)
                }
            }
            SheetValue::Dice { dice, modifier, size_steps } => {
                let s = self.dice(dice, modifier.as_ref(), size_steps.as_ref());
                if self.unresolved.replace(false) { SheetLineValue::Words } else { SheetLineValue::Dice(s) }
            }
            SheetValue::DiceBySize(table) => {
                let i = self.facts.size.clamp(0, 8) as usize;
                SheetLineValue::Dice(table[i].clone())
            }
            SheetValue::Text => SheetLineValue::Words,
        }
    }

    fn choice_names(&self, id: &str) -> String {
        let Some(chosen) = self.facts.choices.get(id).filter(|v| !v.is_empty()) else {
            return "(choice not yet made)".to_string();
        };
        let mut names: Vec<String> = chosen.iter().map(|(_, n)| n.clone()).collect();
        names.sort();
        names.dedup();
        match names.len() {
            1 => names.pop().unwrap_or_default(),
            2 => names.join(" and "),
            _ => names.join(", "),
        }
    }

    fn prose(&self, segments: &[ProseSegment]) -> String {
        // Family order: Desc, Benefit, Special, Aspect lines, StatBlock lines, WhenActive.
        let rank = |f: &ProseFamily| match f {
            ProseFamily::Desc => 0,
            ProseFamily::Benefit => 1,
            ProseFamily::Special => 2,
            ProseFamily::Aspect(_) => 3,
            ProseFamily::StatBlock(_) => 4,
            ProseFamily::WhenActive => 5,
        };
        // Within a `pick_last` family only the last included such segment prints.
        let mut last_pick: BTreeMap<String, usize> = BTreeMap::new();
        let included: Vec<bool> = segments
            .iter()
            .map(|s| s.applies.as_ref().map(|a| self.applies(a).includes()).unwrap_or(true))
            .collect();
        for (i, s) in segments.iter().enumerate() {
            if s.pick_last && included[i] {
                last_pick.insert(format!("{:?}", s.family), i);
            }
        }
        let mut lines: Vec<(i32, usize, String)> = Vec::new();
        for (i, s) in segments.iter().enumerate() {
            if !included[i] {
                continue;
            }
            if s.pick_last && last_pick.get(&format!("{:?}", s.family)) != Some(&i) {
                continue;
            }
            let mut text = String::new();
            let mut slots = 0usize;
            let mut nonzero = false;
            for p in &s.pieces {
                match p {
                    ProsePiece::Text(t) => text.push_str(t),
                    ProsePiece::Slot(e) => {
                        let n = self.expr(e).trunc();
                        self.unresolved.set(false);
                        slots += 1;
                        nonzero |= n != 0;
                        text.push_str(&n.to_string());
                    }
                    ProsePiece::ChoiceName(id) => text.push_str(&self.choice_names(id)),
                    ProsePiece::Dice { dice, modifier } => {
                        text.push_str(&self.dice(dice, modifier.as_ref(), None));
                        self.unresolved.set(false);
                    }
                }
            }
            if s.suppress_when_all_zero && slots > 0 && !nonzero {
                continue;
            }
            let text = text.trim().to_string();
            if text.is_empty() {
                continue;
            }
            let line = match &s.family {
                ProseFamily::Aspect(label) | ProseFamily::StatBlock(label) => format!("{label}: {text}"),
                ProseFamily::WhenActive => format!("When active: {text}"),
                _ => text,
            };
            if !lines.iter().any(|(_, _, l)| *l == line) {
                lines.push((rank(&s.family), i, line));
            }
        }
        lines.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        lines.into_iter().map(|(_, _, l)| l).collect::<Vec<_>>().join("\n")
    }

    fn line(&self, rule: &SheetRule) -> SheetLine {
        let value = self.value(&rule.value);
        let printed = match &value {
            SheetLineValue::Resolved(n) if rule.target.is_some() => format!("{n:+}"),
            SheetLineValue::Resolved(n) => n.to_string(),
            SheetLineValue::Dice(s) => s.clone(),
            SheetLineValue::Words => String::new(),
        };
        let also = rule
            .also
            .iter()
            .map(|(role, v)| {
                let val = self.value(v);
                let n = match &val {
                    SheetLineValue::Resolved(n) => n.to_string(),
                    SheetLineValue::Dice(s) => s.clone(),
                    SheetLineValue::Words => String::new(),
                };
                let printed = match role {
                    ValueRole::Uses { period } if period == "day" => format!("{n}/day"),
                    ValueRole::Uses { period } => format!("{n} {period}"),
                    ValueRole::CasterLevel => format!("CL {n}"),
                    ValueRole::SaveDc => format!("DC {n}"),
                };
                (printed, val)
            })
            .collect();
        let condition = match self.applies(&rule.applies) {
            Gate::Situational(text) => Some(text),
            _ => None,
        };
        let (_, kind, _) = split_rule_id(&rule.id);
        SheetLine {
            id: rule.id.clone(),
            kind: kind.to_string(),
            label: rule.label.clone(),
            value,
            printed,
            also,
            prose: self.prose(&rule.prose),
            condition,
        }
    }
}

/// The damage-die progression (PF1 Core Rulebook, Table: Size and damage; the same ladder
/// Improved Natural Attack steps). A die not on the ladder is returned unchanged.
const DIE_LADDER: [&str; 21] = [
    "1d1", "1d2", "1d3", "1d4", "1d6", "1d8", "1d10", "2d6", "2d8", "3d6", "3d8", "4d6", "4d8", "6d6", "6d8", "8d6",
    "8d8", "12d6", "12d8", "16d6", "16d8",
];

/// Step a die `n` rungs along the ladder (`"1d8"`, 1 -> `"1d10"`; `"1d8"`, -1 -> `"1d6"`).
pub fn step_die(die: &str, n: i64) -> String {
    let d = die.to_ascii_lowercase();
    let d = d.trim();
    let Some(pos) = DIE_LADDER.iter().position(|x| *x == d) else { return die.to_string() };
    let target = (pos as i64 + n).clamp(0, DIE_LADDER.len() as i64 - 1) as usize;
    DIE_LADDER[target].to_string()
}

/// `"1d8"` + 2 -> `"1d8+2"`; + 0 -> `"1d8"`; - 1 -> `"1d8-1"`.
pub fn fold_dice_modifier(die: &str, modifier: i64) -> String {
    if modifier == 0 { die.to_string() } else { format!("{die}{modifier:+}") }
}

/// Evaluate one rule for a character.
pub fn evaluate(rule: &SheetRule, held: &HeldSet, package: &SheetRulePackage, facts: &CharacterFacts, ctx: EvalContext) -> SheetLine {
    Evaluator::new(package, held, facts, ctx).line(rule)
}

/// Evaluate a gate for a character.
pub fn evaluate_applies(a: &Applies, held: &HeldSet, package: &SheetRulePackage, facts: &CharacterFacts, ctx: EvalContext) -> Gate {
    Evaluator::new(package, held, facts, ctx).applies(a)
}

/// Evaluate one converted [`Expr`] for a character -- the same arithmetic
/// [`evaluate`] applies to a rule's own value, exposed for the live consumers that hold an
/// `Expr` directly rather than a whole [`SheetRule`].
///
/// SD-35 `AT-35-E6-001`: this is the live side's ONE arithmetic path. Before this cycle several
/// `rules_core` modules carried a PCGen formula STRING and ran it through the PCGen formula
/// interpreter at run time (`decisions.md` §11: "there should be nothing left of pcgen" on the
/// live side). Conversion from a PCGen token to an `Expr` now happens at ingest, in
/// `src/pcgen_import/`; the live side evaluates the converted `Expr` and nothing else.
pub fn evaluate_expr(e: &Expr, held: &HeldSet, package: &SheetRulePackage, facts: &CharacterFacts, ctx: EvalContext) -> Rat {
    Evaluator::new(package, held, facts, ctx).expr(e)
}

/// [`evaluate_expr`] for an expression whose every term is settled by [`CharacterFacts`] alone
/// -- no `Var`, no `Choice`, no held-set lookup. The overwhelming majority of the converted
/// arithmetic the live chassis and trait tables carry is of exactly this shape (ability
/// modifiers, class levels, total level), and a caller with no package in hand should not have
/// to fabricate one.
pub fn evaluate_expr_from_facts(e: &Expr, facts: &CharacterFacts) -> Rat {
    static EMPTY_PACKAGE: OnceLock<SheetRulePackage> = OnceLock::new();
    static EMPTY_HELD: OnceLock<HeldSet> = OnceLock::new();
    let package = EMPTY_PACKAGE.get_or_init(SheetRulePackage::new);
    let held = EMPTY_HELD.get_or_init(HeldSet::default);
    Evaluator::new(package, held, facts, EvalContext::default()).expr(e)
}

/// The held set: the fixpoint over the seed (`technical-design.md` §2). Seed rules are held
/// because the player holds them; a granted rule joins when its grant's `when` and its own
/// `applies` both include; effects (facts, counts-as, waivers, revokes) apply; repeat until
/// nothing changes. `print == false` rules stay in the set and off the sheet. A rule nothing
/// grants and nothing seeds -- a template no race or choice applied, a monster ability no
/// monster holds -- is never held: `applies` says whether THIS character may hold it, not that
/// it does.
pub fn held_set(package: &SheetRulePackage, seed: &HeldSeed, facts: &CharacterFacts) -> HeldSet {
    let mut held = HeldSet::default();
    let add = |held: &mut HeldSet, id: &RuleId, entry: HeldRule| {
        if !held.rules.contains_key(id) {
            held.rules.insert(id.clone(), entry.clone());
            for sib in package.siblings_of(id) {
                held.rules.entry(sib.clone()).or_insert_with(|| entry.clone());
            }
        }
    };

    // 1. The seed.
    if let Some(race) = &seed.race
        && let Some(id) = package.find("race", race)
    {
        add(&mut held, id, HeldRule::default());
    }
    held.classes = seed.classes.iter().filter(|(_, level)| *level >= 1).map(|(class, _)| class.clone()).collect();
    for (class, _) in &seed.classes {
        if let Some(id) = package.find("class", class) {
            add(&mut held, id, HeldRule { holder_class: Some(class.clone()), ..Default::default() });
        }
    }
    for (kind, slugs) in [("feat", &seed.feats), ("trait", &seed.traits), ("equipment", &seed.equipment), ("spell", &seed.spells), ("skill", &seed.skills)] {
        for s in slugs {
            if let Some(id) = package.find(kind, s) {
                add(&mut held, id, HeldRule::default());
            }
        }
    }
    for key in &seed.race_traits {
        if let Some(id) = package.find("race_trait", &slug(key)) {
            add(&mut held, id, HeldRule::default());
        }
    }
    for (class, explanation_id) in &seed.class_features {
        let rest = explanation_id.strip_prefix("class_feature.").unwrap_or(explanation_id);
        let segs: Vec<&str> = rest.split('.').collect();
        let mut found = None;
        'outer: for i in 0..segs.len() {
            let tail = segs[i..].join("_");
            for candidate in [format!("{class}_{tail}"), tail.clone()] {
                if let Some(id) = package.find("class_feature", &candidate) {
                    found = Some(id.clone());
                    break 'outer;
                }
            }
        }
        if let Some(id) = found {
            add(&mut held, &id, HeldRule { holder_class: Some(class.clone()), ..Default::default() });
        }
    }
    for id in &seed.rule_ids {
        if package.rule(id).is_some() {
            add(&mut held, id, HeldRule::default());
        }
    }

    // 2. The fixpoint.
    for _round in 0..64 {
        let before = held.rules.len() + held.declared.len() + held.counts_as.len() + held.removed.len();
        let mut candidates: BTreeSet<RuleId> = BTreeSet::new();
        for id in held.rules.keys() {
            if let Some(granted) = package.grants_from_rule.get(id) {
                candidates.extend(granted.iter().cloned());
            }
        }
        candidates.extend(package.fact_granted.iter().cloned());
        let mut additions: Vec<(RuleId, HeldRule)> = Vec::new();
        for id in candidates {
            if held.rules.contains_key(&id) {
                continue;
            }
            let Some(rule) = package.rule(&id) else { continue };
            let mut entry: Option<HeldRule> = None;
            for grant in &rule.granted_by {
                let (satisfied, via) = match &grant.by {
                    Granter::Rule(g) => (held.holds(g), held.rules.get(g).cloned()),
                    Granter::Class { id: class, at_level } => {
                        let lvl = seed.classes.iter().filter(|(c, _)| c == class).map(|(_, l)| *l).sum::<i64>();
                        (lvl >= i64::from(*at_level), Some(HeldRule { holder_class: Some(class.clone()), ..Default::default() }))
                    }
                    Granter::ClassSpellList { .. } => (false, None),
                    Granter::Race(r) => (facts.race.as_deref() == Some(slug(r).as_str()), None),
                    Granter::Deity(d) => (facts.deity.as_ref() == Some(d), None),
                    Granter::Choice(c) => {
                        let (_, _, my_slug) = split_rule_id(&id);
                        (facts.choices.get(c).is_some_and(|v| v.iter().any(|(o, _)| o == my_slug || o == &id)), None)
                    }
                };
                if !satisfied {
                    continue;
                }
                let ctx = EvalContext {
                    holder_class: via.as_ref().and_then(|v| v.holder_class.clone()),
                    spell_level: via.as_ref().and_then(|v| v.spell_level).unwrap_or(0),
                    item_tags: Vec::new(),
                };
                let ev = Evaluator::new(package, &held, facts, ctx);
                if ev.applies(&grant.when).includes() && ev.applies(&rule.applies).includes() {
                    let mut e = via.unwrap_or_default();
                    e.via = match &grant.by {
                        Granter::Rule(g) => Some(g.clone()),
                        _ => None,
                    };
                    entry = Some(e);
                    break;
                }
            }
            if let Some(e) = entry {
                additions.push((id, e));
            }
        }
        for (id, e) in additions {
            add(&mut held, &id, e);
        }
        // 3. Effects of everything held.
        let ids: Vec<RuleId> = held.rules.keys().cloned().collect();
        for id in ids {
            let Some(rule) = package.rule(&id) else { continue };
            for effect in &rule.grants {
                match effect {
                    Effect::FactDeclare { name, value } => {
                        held.declared.insert((name.clone(), value.clone()));
                    }
                    Effect::CountsAs(CountsAs::Rule(r)) => {
                        held.counts_as.insert(r.clone());
                    }
                    Effect::Waives(r) | Effect::Revokes(r) => {
                        held.removed.insert(r.clone());
                    }
                    _ => {}
                }
            }
        }
        let after = held.rules.len() + held.declared.len() + held.counts_as.len() + held.removed.len();
        if after == before {
            break;
        }
    }
    held
}

/// Every held, printed rule's line, grouped by kind then label: the "Rules and features" section.
/// A `#bonusN` sibling is a bonus line with its own gate (`applies`), held alongside its
/// principal: it prints only when that gate includes -- an unbroken chain shirt's "Broken"
/// line, a Climb bonus gated on a climb speed, a Skill Focus line on a character without the
/// feat stay off the sheet.
pub fn render_sheet(package: &SheetRulePackage, seed: &HeldSeed, facts: &CharacterFacts) -> Vec<SheetLine> {
    let held = held_set(package, seed, facts);
    let mut lines: Vec<SheetLine> = held
        .rules
        .iter()
        .filter(|(id, _)| !held.removed.contains(*id))
        .filter_map(|(id, entry)| package.rule(id).map(|r| (r, entry)))
        .filter(|(r, _)| r.print)
        .filter_map(|(r, entry)| {
            let ctx = EvalContext {
                holder_class: entry.holder_class.clone(),
                spell_level: entry.spell_level.unwrap_or(0),
                item_tags: if r.subject == Subject::Item { r.tags.clone() } else { Vec::new() },
            };
            if r.id.contains('#') && !Evaluator::new(package, &held, facts, ctx.clone()).applies(&r.applies).includes() {
                return None;
            }
            Some(evaluate(r, &held, package, facts, ctx))
        })
        .collect();
    lines.sort_by(|a, b| a.kind.cmp(&b.kind).then(a.label.cmp(&b.label)).then(a.id.cmp(&b.id)));
    lines
}

/// AT-35-E2-002 -- the evaluator, proven once per value form on real records and once per
/// kind over the live package (`epic-breakdown.md` AT-35-E2-002 Evidence).
#[cfg(test)]
mod evaluate_tests {
    use super::*;
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};
    use crate::rules_core::corpus_loader::load_sheet_rules;
    use crate::rules_core::pilot_compute::{compute_pilot_base_chassis, PilotBaseChassisComputation};
    use std::path::PathBuf;
    use std::sync::OnceLock;

    const FIGHTER_FIXTURE: &str =
        include_str!("../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

    fn repo() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// The live package, loaded once per test process from `data/sheet_rules/`.
    fn package() -> &'static SheetRulePackage {
        static P: OnceLock<SheetRulePackage> = OnceLock::new();
        P.get_or_init(|| {
            let started = std::time::Instant::now();
            let load = load_sheet_rules(&repo().join("data/sheet_rules"));
            assert!(load.diagnostics.is_empty(), "every package file parses: {:?}", &load.diagnostics[..load.diagnostics.len().min(3)]);
            assert!(load.rule_files > 40_000, "the package is generated (cargo run --locked --bin sheet_rule_convert): {} files", load.rule_files);
            eprintln!(
                "sheet_rules package: {} rule files, {} var files, {} rules, loaded in {:?}",
                load.rule_files,
                load.var_files,
                load.package.rules.len(),
                started.elapsed()
            );
            load.package
        })
    }

    fn fighter() -> (CharacterInput, PilotBaseChassisComputation) {
        let input = load_character_input_fixture(FIGHTER_FIXTURE).character_input.expect("the deterministic fixture loads");
        let computation = compute_pilot_base_chassis(&input);
        (input, computation)
    }

    fn fighter_facts() -> CharacterFacts {
        let (input, computation) = fighter();
        CharacterFacts::from_character(&input, &computation)
    }

    fn rule_with_value(value: SheetValue) -> SheetRule {
        SheetRule {
            id: "core_rulebook:race_trait:probe".into(),
            label: "Probe".into(),
            value,
            also: vec![],
            prose: vec![],
            applies: Applies::Always,
            target: None,
            bonus_type: None,
            print: true,
            pool: "special_ability".into(),
            tags: vec![],
            subject: Subject::Character,
            repeatable: false,
            granted_by: vec![],
            offers: None,
            grants: vec![],
            provenance: Provenance::default(),
        }
    }

    #[test]
    fn rat_is_exact_and_truncates_toward_zero_once() {
        let two_thirds = Rat::int(2).div(Rat::int(3));
        // 300 * 2/3 is exactly 200 -- no float drift, no intermediate truncation.
        assert_eq!(Rat::int(300).mul(two_thirds).trunc(), 200);
        // (7 / 2) * 2 = 7 exactly; a per-step floor would have given 6.
        assert_eq!(Rat::int(7).div(Rat::int(2)).mul(Rat::int(2)).trunc(), 7);
        assert_eq!(Rat::int(-7).div(Rat::int(2)).trunc(), -3, "toward zero");
        assert_eq!(Rat::int(-7).div(Rat::int(2)).floor(), Rat::int(-4));
        assert_eq!(Rat::int(7).div(Rat::int(2)).ceil(), Rat::int(4));
        assert_eq!(Rat::int(5).div(Rat::ZERO), Rat::ZERO, "division by zero yields 0");
        assert!(Rat::int(1).div(Rat::int(3)) < Rat::int(1).div(Rat::int(2)));
    }

    /// The racial SLA rule: `10 + spell level + Charisma modifier` prints as one DC. On the real
    /// APG Ill Omen record (spell level 1, folded to `Const(1)` by the converter) at Charisma 14
    /// that is `DC 13`; on the design's own shape (`SpellLevel` as a leaf) at spell level 3 it
    /// is `Resolved(15)`.
    #[test]
    fn racial_sla_save_dc_evaluates_to_one_final_number() {
        let package = package();
        let mut facts = fighter_facts();
        facts.ability_mods[ability_index(Ability::Cha)] = 2; // Charisma 14
        let held = HeldSet::default();

        let rule = package.rule("advanced_players_guide:race_trait:racial_sla_ill_omen").expect("the APG Ill Omen SLA converted");
        let line = evaluate(rule, &held, package, &facts, EvalContext::default());
        assert_eq!(line.kind, "race_trait");
        assert_eq!(line.label, "Ill Omen");
        assert!(line.also.iter().any(|(printed, v)| printed == "DC 13" && *v == SheetLineValue::Resolved(13)), "{:?}", line.also);
        assert!(line.also.iter().any(|(printed, v)| printed == "CL 1" && *v == SheetLineValue::Resolved(1)), "caster level = character level 1: {:?}", line.also);
        assert!(line.also.iter().any(|(printed, _)| printed == "1/day"), "{:?}", line.also);
        assert_eq!(line.value, SheetLineValue::Resolved(1), "the principal value is the SLA count");

        let shape = rule_with_value(SheetValue::Number(Expr::Sum(vec![Expr::Const(10), Expr::SpellLevel, Expr::AbilityMod(Ability::Cha)])));
        let line = evaluate(&shape, &held, package, &facts, EvalContext { spell_level: 3, ..Default::default() });
        assert_eq!(line.value, SheetLineValue::Resolved(15));
        assert_eq!(line.printed, "15");
    }

    /// The weapon: dice stay dice. The real CRB Longsword is `1d8`; with a `Const(2)` modifier
    /// it folds to `1d8+2`; a size step moves it along the damage ladder.
    #[test]
    fn weapon_evaluates_to_dice_in_final_form() {
        let package = package();
        let facts = fighter_facts();
        let held = HeldSet::default();
        let rule = package.rule("core_rulebook:equipment:longsword").expect("the CRB Longsword converted");
        let line = evaluate(rule, &held, package, &facts, EvalContext::default());
        assert_eq!(line.value, SheetLineValue::Dice("1d8".into()));
        assert_eq!(line.printed, "1d8");
        assert!(line.prose.contains("Critical threat: 2-20"), "{}", line.prose);

        let plus_two = rule_with_value(SheetValue::Dice { dice: "1d8".into(), modifier: Some(Expr::Const(2)), size_steps: None });
        assert_eq!(evaluate(&plus_two, &held, package, &facts, EvalContext::default()).value, SheetLineValue::Dice("1d8+2".into()));
        let str_mod = rule_with_value(SheetValue::Dice { dice: "1d8".into(), modifier: Some(Expr::AbilityMod(Ability::Str)), size_steps: None });
        assert_eq!(evaluate(&str_mod, &held, package, &facts, EvalContext::default()).printed, "1d8+4", "Strength 16 + the Human +2 the chassis applied = 18");
        let stepped = rule_with_value(SheetValue::Dice { dice: "1d8".into(), modifier: None, size_steps: Some(Expr::Const(1)) });
        assert_eq!(evaluate(&stepped, &held, package, &facts, EvalContext::default()).printed, "1d10");
        assert_eq!(step_die("1d8", -1), "1d6");
        assert_eq!(step_die("2d6", 1), "2d8");
        assert_eq!(fold_dice_modifier("1d8", -1), "1d8-1");
    }

    /// The choice trait: a term the player has not settled prints as words, and the chosen
    /// name fills the slot once the choice is made.
    #[test]
    fn choice_trait_evaluates_to_words() {
        let package = package();
        let mut facts = fighter_facts();
        let held = HeldSet::default();
        let rule = package.rule("advanced_players_guide:trait:trait_magical_knack").expect("the APG Magical Knack trait converted");
        let line = evaluate(rule, &held, package, &facts, EvalContext::default());
        assert_eq!(line.value, SheetLineValue::Words);
        assert_eq!(line.printed, "");
        assert!(line.prose.contains("(choice not yet made)"), "{}", line.prose);

        facts.choices.insert(rule.id.clone(), vec![("wizard".into(), "Wizard".into())]);
        let line = evaluate(rule, &held, package, &facts, EvalContext::default());
        assert!(line.prose.contains("Wizard"), "{}", line.prose);
        assert!(!line.prose.contains("(choice not yet made)"));

        let unmade = rule_with_value(SheetValue::Number(Expr::Sum(vec![Expr::Const(1), Expr::Choice("nobody:offers:this".into())])));
        assert_eq!(evaluate(&unmade, &held, package, &facts, EvalContext::default()).value, SheetLineValue::Words, "an unmade numeric choice prints as words");
    }

    /// The per-kind gate: every rule of every kind in the live package evaluates for a probe
    /// character to a well-formed line -- a number, dice, or words -- with its label. All 19
    /// kinds are present. Counts per kind are printed for the receipt.
    #[test]
    fn every_kind_in_the_package_evaluates_to_a_well_formed_line() {
        let package = package();
        let (input, computation) = fighter();
        let facts = CharacterFacts::from_character(&input, &computation);
        let held = held_set(package, &HeldSeed::from_character(&input, &computation), &facts);
        let mut per_kind: BTreeMap<String, [usize; 3]> = BTreeMap::new();
        for rule in package.rules.values() {
            let line = evaluate(rule, &held, package, &facts, EvalContext::default());
            assert!(!line.label.is_empty(), "{}: a line has a label", rule.id);
            let slot = per_kind.entry(line.kind.clone()).or_default();
            match &line.value {
                SheetLineValue::Resolved(n) => {
                    let expected = if rule.target.is_some() { format!("{n:+}") } else { n.to_string() };
                    assert_eq!(line.printed, expected, "{}", rule.id);
                    slot[0] += 1;
                }
                SheetLineValue::Dice(s) => {
                    assert_eq!(&line.printed, s, "{}", rule.id);
                    assert!(s.contains('d'), "{}: dice stay dice: {s}", rule.id);
                    slot[1] += 1;
                }
                SheetLineValue::Words => {
                    assert_eq!(line.printed, "", "{}", rule.id);
                    slot[2] += 1;
                }
            }
            for (printed, v) in &line.also {
                assert!(!printed.is_empty(), "{}: an `also` value prints ({v:?})", rule.id);
            }
        }
        for (kind, [n, d, w]) in &per_kind {
            eprintln!("kind={kind} number={n} dice={d} words={w}");
        }
        assert_eq!(per_kind.len(), 19, "every kind the converter wrote: {:?}", per_kind.keys().collect::<Vec<_>>());
    }

    /// The held set is a fixpoint: a granted rule joins when its grant's `when` and its own
    /// `applies` include, an effect on a held rule declares a fact another gate reads, and a
    /// waiver drops a rule from the sheet.
    #[test]
    fn held_set_fixpoint_grants_and_effects() {
        let mut package = SheetRulePackage::new();
        let mut a = rule_with_value(SheetValue::Text);
        a.id = "core_rulebook:feat:seed".into();
        a.grants = vec![Effect::FactDeclare { name: "Trained".into(), value: "true".into() }];
        let mut b = rule_with_value(SheetValue::Number(Expr::Const(2)));
        b.id = "core_rulebook:class_feature:granted".into();
        b.granted_by = vec![Grant { by: Granter::Rule(a.id.clone()), when: Applies::Compare { lhs: Expr::Level, op: Cmp::Gte, rhs: Expr::Const(3) } }];
        let mut c = rule_with_value(SheetValue::Text);
        c.id = "core_rulebook:class_feature:by_fact".into();
        c.granted_by = vec![Grant { by: Granter::Rule(b.id.clone()), when: Applies::Always }];
        c.applies = Applies::Holds { what: Holdable::Fact { name: "Trained".into(), value: "true".into() }, count: 1 };
        let mut d = rule_with_value(SheetValue::Text);
        d.id = "core_rulebook:class_feature:waived".into();
        d.granted_by = vec![Grant { by: Granter::Rule(a.id.clone()), when: Applies::Always }];
        let mut e = rule_with_value(SheetValue::Text);
        e.id = "core_rulebook:feat:waiver".into();
        e.grants = vec![Effect::Waives(d.id.clone())];
        for r in [a, b, c, d, e] {
            package.insert_rule(r);
        }
        package.finish();
        let seed = HeldSeed { feats: vec!["seed".into(), "waiver".into()], ..Default::default() };

        let facts = CharacterFacts { level: 1, ..Default::default() };
        let held = held_set(&package, &seed, &facts);
        assert!(held.holds("core_rulebook:feat:seed"));
        assert!(!held.holds("core_rulebook:class_feature:granted"), "level 1 does not satisfy the grant's `when`");
        assert!(held.declared.contains(&("Trained".into(), "true".into())));
        assert!(held.removed.contains("core_rulebook:class_feature:waived"));

        let facts = CharacterFacts { level: 3, ..Default::default() };
        let held = held_set(&package, &seed, &facts);
        assert!(held.holds("core_rulebook:class_feature:granted"));
        assert!(held.holds("core_rulebook:class_feature:by_fact"), "the fact the seed declared satisfies the gate");
        let lines = render_sheet(&package, &seed, &facts);
        let ids: Vec<&str> = lines.iter().map(|l| l.id.as_str()).collect();
        assert_eq!(ids, vec!["core_rulebook:class_feature:by_fact", "core_rulebook:class_feature:granted", "core_rulebook:feat:seed", "core_rulebook:feat:waiver"], "grouped by kind then label; the waived rule is off the sheet");
        assert_eq!(lines[1].printed, "2");
    }

    /// The `Var` fold: two Racial +2s to one variable stack (4), two Enhancement bonuses take
    /// the max (3), a `Replace` contribution competes with the plain fold, and a variable
    /// with no held declarer is 0.
    #[test]
    fn var_fold_stacks_by_bonus_type() {
        let mut package = SheetRulePackage::new();
        let mut holder = rule_with_value(SheetValue::Number(Expr::Var("v1".into())));
        holder.id = "core_rulebook:feat:holder".into();
        let mut other = rule_with_value(SheetValue::Text);
        other.id = "core_rulebook:feat:other".into();
        let mut unheld = rule_with_value(SheetValue::Text);
        unheld.id = "core_rulebook:feat:unheld".into();
        for r in [holder, other, unheld] {
            package.insert_rule(r);
        }
        let typed = |name: &str, mode: StackMode| Some(BonusType { name: name.into(), mode });
        let contribution = |rule: &str, n: i32, bonus_type: Option<BonusType>| VarContribution {
            rule_id: format!("core_rulebook:feat:{rule}"),
            expr: Expr::Const(n),
            bonus_type,
            when: Applies::Always,
        };
        package.insert_var(VarTable {
            var: "v1".into(),
            declared_by: vec!["core_rulebook:feat:holder".into()],
            contributions: vec![
                contribution("holder", 2, typed("Racial", StackMode::Plain)),
                contribution("other", 2, typed("Racial", StackMode::Plain)),
                contribution("holder", 2, typed("Enhancement", StackMode::Plain)),
                contribution("other", 3, typed("Enhancement", StackMode::Plain)),
                contribution("holder", 1, None),
                contribution("unheld", 50, None),
            ],
            provenance: VarProvenance::default(),
        });
        package.insert_var(VarTable { var: "v2".into(), declared_by: vec!["core_rulebook:feat:unheld".into()], contributions: vec![contribution("holder", 9, None)], provenance: VarProvenance::default() });
        package.finish();
        let seed = HeldSeed { feats: vec!["holder".into(), "other".into()], ..Default::default() };
        let facts = CharacterFacts::default();
        let lines = render_sheet(&package, &seed, &facts);
        let holder_line = lines.iter().find(|l| l.id == "core_rulebook:feat:holder").unwrap();
        assert_eq!(holder_line.value, SheetLineValue::Resolved(4 + 3 + 1), "Racial stacks, Enhancement takes the max, untyped sums, the unheld contribution is ignored");

        let held = held_set(&package, &seed, &facts);
        let no_declarer = rule_with_value(SheetValue::Number(Expr::Var("v2".into())));
        assert_eq!(evaluate(&no_declarer, &held, &package, &facts, EvalContext::default()).value, SheetLineValue::Resolved(0));
        let replace = VarTable {
            var: "v3".into(),
            declared_by: vec!["core_rulebook:feat:holder".into()],
            contributions: vec![
                contribution("holder", 2, typed("Enhancement", StackMode::Plain)),
                contribution("other", 5, typed("Enhancement", StackMode::Replace)),
            ],
            provenance: VarProvenance::default(),
        };
        package.insert_var(replace);
        let held = held_set(&package, &seed, &facts);
        let r = rule_with_value(SheetValue::Number(Expr::Var("v3".into())));
        assert_eq!(evaluate(&r, &held, &package, &facts, EvalContext::default()).value, SheetLineValue::Resolved(5), "max(plain, replace)");
    }

    /// The deterministic Human Fighter renders real lines from the live package: its feats, its
    /// longsword's dice, the racial traits the caller's race resolver names, and the class
    /// features the chassis grounded -- grouped by kind. Acrobatic's skill bonus is the
    /// converter's own `Sum`/`Min`/`Max` shape evaluated against real ranks.
    #[test]
    fn a_fixture_fighter_renders_lines_grouped_by_kind() {
        let package = package();
        let (mut input, _) = fighter();
        input.chosen.selected_feats.push("feat:acrobatic".into());
        input.chosen.skill_allocations.push(crate::rules_core::character_input::SkillAllocation { skill_id: "skill:acrobatics".into(), ranks: 10 });
        let computation = compute_pilot_base_chassis(&input);
        let computation = computation.with_sheet_rules(&input, package, &["Human ~ Bonus Feat".to_string(), "Human ~ Skilled".to_string()]);
        let lines = &computation.sheet_lines;
        assert!(!lines.is_empty());
        let facts = CharacterFacts::from_character(&input, &computation);
        let by_id = |id: &str| lines.iter().find(|l| l.id == id).unwrap_or_else(|| panic!("{id} in {:?}", lines.iter().map(|l| &l.id).collect::<Vec<_>>()));
        assert_eq!(by_id("core_rulebook:feat:acrobatic").printed, "+4", "10 ranks in Acrobatics: the +2 becomes +4");
        assert_eq!(by_id("core_rulebook:feat:acrobatic#bonus1").printed, "+2", "no ranks in Fly");
        assert_eq!(by_id("core_rulebook:feat:power_attack").kind, "feat");
        assert_eq!(by_id("core_rulebook:equipment:longsword").printed, "1d8");
        assert_eq!(by_id("core_rulebook:race_trait:human_bonus_feat").kind, "race_trait");
        assert!(lines.iter().any(|l| l.kind == "class_feature" && l.id.contains("fighter")), "a fighter class feature the chassis grounded: {:?}", lines.iter().filter(|l| l.kind == "class_feature").map(|l| &l.id).collect::<Vec<_>>());
        let kinds: Vec<&str> = lines.iter().map(|l| l.kind.as_str()).collect();
        let mut sorted = kinds.clone();
        sorted.sort();
        assert_eq!(kinds, sorted, "grouped by kind");
        assert_eq!(facts.level, 1);
        assert_eq!(facts.class_levels, vec![("fighter".to_string(), 1)]);
        assert_eq!(facts.ability_mods[ability_index(Ability::Str)], 4, "Strength 16 + the Human +2 = 18");
        assert!(!lines.iter().any(|l| l.kind == "template"), "no template is held that nothing granted: {:?}", lines.iter().filter(|l| l.kind == "template").map(|l| &l.id).collect::<Vec<_>>());
        assert!(!lines.iter().any(|l| l.kind == "monster_ability"), "no monster ability is held that nothing granted");
        eprintln!("fixture fighter lines: {} across kinds {:?}", lines.len(), lines.iter().map(|l| l.kind.as_str()).collect::<BTreeSet<_>>());
        assert_eq!(facts.size, 4, "Human is Medium");
        assert_eq!(facts.skill_ranks.get("acrobatics"), Some(&10));
        assert_eq!(facts.race.as_deref(), Some("human"));
    }

    /// A `#bonusN` sibling is a bonus line with its own gate: it prints, and folds into its
    /// target, only when that gate includes. The deterministic fighter has no Climb speed and
    /// no Skill Focus, so `core_rulebook:skill:climb`'s three gated siblings (+8 Racial with a
    /// climb speed, +3 / +6 Skill Focus) stay off the sheet and the class-skill +3 is the only
    /// line on the target; PCGen's `SKILL.n.MISC` for the same character carries exactly that
    /// +3 (AT-35-E2-005 cycle 1, disagreements 1-4). A sibling whose gate includes still
    /// prints (Acrobatic's Fly half is `Always`).
    #[test]
    fn a_sibling_line_prints_only_when_its_own_gate_includes() {
        let package = package();
        let (mut input, _) = fighter();
        input.chosen.selected_feats.push("feat:acrobatic".into());
        let computation = compute_pilot_base_chassis(&input);
        let computation = computation.with_sheet_rules(&input, package, &[]);
        let lines = &computation.sheet_lines;
        let climb: Vec<&str> = lines.iter().filter(|l| l.id.starts_with("core_rulebook:skill:climb")).map(|l| l.id.as_str()).collect();
        assert_eq!(climb, vec!["core_rulebook:skill:climb"], "the gated siblings stay off the sheet");
        let swim: Vec<&str> = lines.iter().filter(|l| l.id.starts_with("core_rulebook:skill:swim")).map(|l| l.id.as_str()).collect();
        assert_eq!(swim, vec!["core_rulebook:skill:swim"]);
        assert!(lines.iter().any(|l| l.id == "core_rulebook:feat:acrobatic#bonus1"), "an `Always` sibling prints");
        assert!(
            !lines.iter().any(|l| l.id == "core_rulebook:equipment:chain_shirt#bonus1"),
            "the Broken-armor sibling of an unbroken chain shirt stays off the sheet"
        );
        let held = held_set(package, &HeldSeed::from_character(&input, &computation), &CharacterFacts::from_character(&input, &computation));
        assert!(held.holds("core_rulebook:skill:climb#bonus2"), "the sibling is still HELD (its gate is a bonus condition, not a holding condition)");
    }

    /// A `Var` declared by a `class` kind rule is that class's own level variable. The converter
    /// refuses every `class` record today (unmapped `STARTSKILLPTS` / `SPELLSTAT` / ... --
    /// AT-35-E4-001), so no class rule is in the package to seed; the character still HOLDS
    /// the class by its levels, so the declarer counts as held and its `ClassLevel`
    /// contribution folds (AT-35-E2-005 cycle 1, disagreements 5-8). Another class's levels do
    /// not hold it.
    #[test]
    fn a_class_level_var_folds_for_the_class_the_character_holds() {
        let mut package = SheetRulePackage::new();
        let mut feature = rule_with_value(SheetValue::Number(Expr::Var("vlvl".into())));
        feature.id = "core_rulebook:class_feature:bard_thing".into();
        package.insert_rule(feature);
        package.insert_var(VarTable {
            var: "vlvl".into(),
            declared_by: vec!["core_rulebook:class:bard".into()],
            contributions: vec![VarContribution {
                rule_id: "core_rulebook:class:bard".into(),
                expr: Expr::ClassLevel("bard".into()),
                bonus_type: None,
                when: Applies::Always,
            }],
            provenance: VarProvenance::default(),
        });
        package.finish();
        for (class, expected) in [("bard", 5), ("fighter", 0)] {
            let seed = HeldSeed {
                classes: vec![(class.to_string(), 5)],
                rule_ids: vec!["core_rulebook:class_feature:bard_thing".into()],
                ..Default::default()
            };
            let facts = CharacterFacts { class_levels: vec![(class.to_string(), 5)], ..Default::default() };
            let lines = render_sheet(&package, &seed, &facts);
            assert_eq!(lines.len(), 1, "{class}: the class rule itself is not a line (no record)");
            assert_eq!(lines[0].value, SheetLineValue::Resolved(expected), "{class}");
            let held = held_set(&package, &seed, &facts);
            assert_eq!(held.holds("core_rulebook:class:bard"), class == "bard");
            assert!(!held.holds("core_rulebook:class:bard#bonus1"), "a class sibling is not implied by levels");
        }
    }

    /// The live package, on the parity roster's Human Bard: Bardic Performance's rounds per day
    /// is 4 + Cha + 2 per level beyond 1st in the book, `2 + Cha + 2 * BardLevel` in the converted
    /// contribution -- 7 at level 1 and 25 at level 10 with Charisma 16, PCGen's own substituted
    /// DESCRIPTION for the same characters (`oracle-parity/exports/human_bard_l{1,10}.txt`).
    #[test]
    fn bardic_performance_rounds_read_the_bard_level_from_the_held_class() {
        let package = package();
        for (level, expected) in [(1, 7), (10, 25)] {
            let text = format!(
                "case_id=parity-human_bard_l{level}\nsource_package_id=pf1.core_rulebook\nrace_id=race:human\nclass_level=class:bard:{level}\n\
                 ability=strength:12\nability=dexterity:10\nability=constitution:8\nability=intelligence:18\nability=wisdom:14\nability=charisma:16\n\
                 choice=choice:human_ability_bonus:ability:strength\n"
            );
            let input = load_character_input_fixture(&text).character_input.expect("the roster member loads");
            let computation = compute_pilot_base_chassis(&input).with_sheet_rules(&input, package, &[]);
            let line = computation
                .sheet_lines
                .iter()
                .find(|l| l.id == "core_rulebook:class_feature:bard_bardic_performance")
                .unwrap_or_else(|| panic!("bardic performance is held at bard {level}: {:?}", computation.sheet_lines.iter().map(|l| &l.id).collect::<Vec<_>>()));
            let rounds = line.also.iter().find(|(printed, _)| printed.contains("rounds")).map(|(_, v)| v.clone());
            assert_eq!(rounds, Some(SheetLineValue::Resolved(expected)), "bard {level}: {:?}", line.also);
        }
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
