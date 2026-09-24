//! The class base-attack-bonus / base-save chassis, read from the CONVERTED
//! rules (`data/sheet_rules/<book>/class/<slug>.json`) — SD-35 `AT-35-E6-001`
//! (`epic-breakdown.md` `### AT-35-E6-001`; `decisions.md` §1, §11).
//!
//! # What this replaces
//!
//! Three modules used to do the same job three times, each by reading a corpus
//! record's ingest-format token list, pulling the base-attack and base-save
//! formula STRINGS out of it, and running them through the ingest-format
//! formula interpreter at render time:
//! `pilot_compute::crb_untabled_class_chassis` (CRB's seven NPC/`Ex-*`
//! classes), `pilot_compute::generic_class_chassis` (61 conventional PC classes
//! across 14 books), and the desktop crate's own `class_catalog_generic`
//! (the same 61, mirrored for the reference-library browser).
//!
//! The ruling is that nothing on the live side reads an ingest-format token or
//! an ingest-format formula string. Conversion happens at ingest: `sheet_rule_convert` already
//! writes each class's BAB and three save progressions as converted [`Expr`]s
//! (`{"Number":{"Div":[{"ClassLevel":"commoner"},{"Const":2}]}}`), keyed by
//! [`BonusTarget::BaseAttack`] / [`BonusTarget::BaseSave`], and this cycle
//! added the class's own level ceiling to the converted `applies` gate
//! (`Compare { ClassLevel(slug), Lte, Const(n) }`) so the ceiling no longer has
//! to be re-read from a token either. This module is the ONE live reader of
//! that shape; all three callers above now go through it.
//!
//! # Arithmetic
//!
//! [`sheet_rule::evaluate_expr_from_facts`] evaluates exactly (`Rat`), and this
//! module truncates toward zero ONCE at the boundary — the same single
//! truncation `SheetValue::Number`'s own contract states, and the same result
//! the ingest format's per-operation integer division reaches for every shape this
//! population carries (`floor(x) + k == floor(x + k)` for the integer `k` a
//! good save adds).
//!
//! # Honest absence
//!
//! A class file that carries no `BaseAttack` row, or fewer than all three
//! `BaseSave` rows, is not a chassis-bearing class and is simply absent from
//! [`records`] — never half a chassis and never a guessed progression.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::support::paths::repo_root;

use crate::rules_core::sheet_rule::{
    evaluate_expr_from_facts, Applies, BonusTarget, CharacterFacts, Cmp, Expr, ProseFamily,
    ProsePiece, Rat, Save, SheetRule, SheetValue,
};

/// The default level ceiling for a class whose converted record states none:
/// 10 for a prestige class, 20 otherwise. Identical to the rule the corpus-side
/// readers applied before this module existed (no stated ceiling, or a record
/// stating explicitly that there is no limit — the class still stops at the
/// ordinary table's last row).
const DEFAULT_MAX_LEVEL: u8 = 20;
const DEFAULT_PRESTIGE_MAX_LEVEL: u8 = 10;

/// One class's converted chassis: the four progressions plus its ceiling.
#[derive(Debug, Clone)]
pub struct ClassChassis {
    pub book: String,
    /// The class slug, exactly as the converted record's file is named. This is
    /// the id the dispatchers key on (`"class:<slug>"`).
    pub slug: String,
    /// The class id `Expr::ClassLevel` actually spells inside THIS record's own
    /// converted expressions, which is not always [`Self::slug`]: a record
    /// whose class name is redacted carries a codex-neutral id
    /// (`codex_named_unit_class_<book>_<lst>_<line>`) while its file keeps the
    /// readable slug. Binding the wrong one makes every level evaluate to
    /// zero -- a confidently wrong number, which is worse than a refusal --
    /// so the binding is read off the expressions themselves.
    level_var: String,
    /// The converted rule's own `label` — the class's display name.
    pub display_name: String,
    /// `tags` off the principal rule (`"Base"`, `"PC"`, `"NPC"`, `"Prestige"`,
    /// `"Monster"` …), converted from the record's `TYPE:` heads.
    pub tags: Vec<String>,
    pub max_level: u8,
    base_attack: Expr,
    /// Fortitude, Reflex, Will.
    saves: [Expr; 3],
    /// Per save index: `true` when the record also carries a `BaseSave` row
    /// for that save whose value converted to WORDS (`SheetValue` other
    /// than `Number`) -- the words-not-`Expr` degradation symptom
    /// `generic_class_chassis.rs`'s population history documents (the old
    /// record-wide policy that printed a class's own clean save formula as
    /// its rule's words). [`Self::save_shape`] reports such a save
    /// `Degraded` rather than trusting whichever sibling row did convert.
    save_words: [bool; 3],
    /// This class's hit die size (`d10` -> `10`), read off the principal
    /// rule's `StatBlock "Hit die"` prose row — `None` for the 7 records
    /// (of 185) that carry no such row, never a fabricated value (F0-check
    /// finding 5; review finding 13 / `epic-f-class-completion.md` §0.4).
    /// A hard prerequisite of F3: HP cannot be computed without it.
    pub hit_die: Option<u8>,
    /// This class's skill ranks gained per level, read off the principal
    /// rule's `StatBlock "Skill ranks per level"` prose row the same way
    /// [`Self::hit_die`] reads `"Hit die"` — `None` when the row is absent,
    /// never a fabricated value. The converter writes the row from PCGen's
    /// class-line `STARTSKILLPTS` (SD-36 F3b2, `sheet_rule/convert.rs`):
    /// every chassis-bearing class record carries it
    /// (`skill_ranks_per_level_is_read_for_every_chassis_bearing_class`). Before
    /// F3b2 no class record did (F0-check finding 5,
    /// `docs/retro/events/sub-agent-f0-check-fix.jsonl`).
    pub skill_ranks_per_level: Option<u8>,
}

/// A class's base-save progression as read off its converted `Expr`
/// ([`ClassChassis::save_shape`]). PF1 prints two class-level save shapes,
/// good and poor, each in a base-class and a prestige-class table form;
/// anything else is named, never folded into one of them. The fold that
/// consumes a shape reads the class's own `Expr` values for the totals --
/// the enum names the category, not which table form produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveProgression {
    /// `level/2 + 2` (base class good save) or `(level+1)/2` (prestige
    /// class good save).
    Good,
    /// `level/3` (base class poor save) or `(level+1)/3` (prestige class
    /// poor save).
    Poor,
    /// The record shows the words-not-`Expr` symptom for this save: a
    /// `BaseSave` row whose value converted to words, or an unresolved
    /// `Choice` term (which prints as words) inside the `Expr` itself.
    Degraded,
    /// A clean `Expr` whose values match neither closed form over the
    /// class's own levels, or one that reads anything other than the
    /// class's own level and constants.
    Unrecognized,
}

/// The diagnostic id a class's HP contribution carries when its record
/// states no hit die ([`ClassChassis::hit_points`]).
pub const HIT_POINTS_UNKNOWN: &str = "class_chassis.hit_points.unknown";
/// The diagnostic id a class's skill-point contribution carries when its
/// record states no skill ranks per level ([`ClassChassis::skill_points`]).
pub const SKILL_POINTS_UNKNOWN: &str = "class_chassis.skill_points.unknown";

/// A sheet total the class's converted record cannot support: `id` is one of
/// [`HIT_POINTS_UNKNOWN`] / [`SKILL_POINTS_UNKNOWN`], `message` names the
/// class and the missing row. Printed as Unknown, never folded as 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChassisUnknown {
    pub id: &'static str,
    pub message: String,
}

/// One row of a class's printed progression table: `(level, base attack bonus,
/// Fortitude, Reflex, Will)`.
pub type ProgressionRow = (u8, i16, i16, i16, i16);

/// The process-wide `(book, slug)` cache [`record`] keeps.
type RecordCache = std::sync::Mutex<BTreeMap<(String, String), Option<&'static ClassChassis>>>;

/// One resolved level's row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassChassisRow {
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

impl ClassChassis {
    /// `true` when the record is a conventional PC/NPC class rather than a
    /// monster or companion pseudo-class. The converted `tags` carry the
    /// record's own `TYPE:` heads, so this is the same predicate the
    /// corpus-side `classify_class_record` applied to `TYPE`, read off the
    /// converted record instead of the token.
    pub fn is_conventional(&self) -> bool {
        !self.tags.iter().any(|t| t == "Monster")
    }

    /// This class's chassis row at `level`, or `None` when `level` is outside
    /// `1..=max_level` or a progression does not evaluate to a whole number
    /// that fits.
    pub fn row_at(&self, level: u8) -> Option<ClassChassisRow> {
        if level < 1 || level > self.max_level {
            return None;
        }
        let facts = CharacterFacts {
            level: i64::from(level),
            class_levels: vec![(self.level_var.clone(), i64::from(level))],
            ..CharacterFacts::default()
        };
        let at = |e: &Expr| i16::try_from(evaluate_expr_from_facts(e, &facts).trunc()).ok();
        Some(ClassChassisRow {
            base_attack_bonus: at(&self.base_attack)?,
            fort_save: at(&self.saves[0])?,
            ref_save: at(&self.saves[1])?,
            will_save: at(&self.saves[2])?,
        })
    }

    /// The shape of save `index` (0 Fortitude, 1 Reflex, 2 Will), or `None`
    /// for an index outside `0..3`. See [`SaveProgression`]. The shape is
    /// decided by the values the `Expr` takes at every level
    /// `1..=max_level`, compared against each closed form truncated the way
    /// [`Self::row_at`] truncates -- after ruling out degradation (checked
    /// structurally first) and any term other than this class's own level.
    pub fn save_shape(&self, index: usize) -> Option<SaveProgression> {
        let expr = self.saves.get(index)?;
        if self.save_words[index] || expr_has_choice(expr) {
            return Some(SaveProgression::Degraded);
        }
        if !reads_only_own_level(expr, &self.level_var) {
            return Some(SaveProgression::Unrecognized);
        }
        let value_at = |level: i64| {
            let facts = CharacterFacts {
                level,
                class_levels: vec![(self.level_var.clone(), level)],
                ..CharacterFacts::default()
            };
            evaluate_expr_from_facts(expr, &facts).trunc()
        };
        let matches = |form: fn(i64) -> i64| (1..=i64::from(self.max_level)).all(|l| value_at(l) == form(l));
        if SAVE_GOOD_FORMS.iter().any(|form| matches(*form)) {
            return Some(SaveProgression::Good);
        }
        if SAVE_POOR_FORMS.iter().any(|form| matches(*form)) {
            return Some(SaveProgression::Poor);
        }
        Some(SaveProgression::Unrecognized)
    }

    /// Save `index`'s EXACT value at `level` -- the converted `Expr` evaluated
    /// as a rational with no truncation (`level/2 + 2` at 1st is `5/2`), or
    /// `None` for an index outside `0..3` or a level outside
    /// `1..=max_level`. SD-36 F3b: the multiclass fold sums these across
    /// classes and floors once, so a class's own table form (base or
    /// prestige) is what it contributes, never a re-derived closed form.
    /// Callers check [`Self::save_shape`] first: only a `Good`/`Poor` save
    /// is folded.
    pub fn save_value_exact(&self, index: usize, level: u8) -> Option<Rat> {
        let expr = self.saves.get(index)?;
        if level < 1 || level > self.max_level {
            return None;
        }
        let facts = CharacterFacts {
            level: i64::from(level),
            class_levels: vec![(self.level_var.clone(), i64::from(level))],
            ..CharacterFacts::default()
        };
        Some(evaluate_expr_from_facts(expr, &facts))
    }

    /// The hit points `levels` levels of this class contribute: the full
    /// hit die for the level that is the character's 1st
    /// (`includes_first_character_level`), the non-rolling average
    /// ([`crate::rules_core::durability::average_hit_die_value`]) for every
    /// other, each level plus `constitution_modifier` and floored at 1 --
    /// the same per-level rule `durability::compute_max_hp` applies to the
    /// tabled classes. A record with no hit die is
    /// [`HIT_POINTS_UNKNOWN`], never 0.
    ///
    /// This is the per-class term of the sheet's HP total; no fold reads HP
    /// off a `ClassChassis` before SD-36 F3b (the multiclass fold), so this is
    /// where the Unknown is decided for every caller that does.
    pub fn hit_points(
        &self,
        levels: u8,
        includes_first_character_level: bool,
        constitution_modifier: i16,
    ) -> Result<i16, ChassisUnknown> {
        let die = self.hit_die.ok_or_else(|| ChassisUnknown {
            id: HIT_POINTS_UNKNOWN,
            message: format!(
                "{} ({}:class:{}): the converted record states no hit die, so this class's hit \
                 points are Unknown",
                self.display_name, self.book, self.slug
            ),
        })?;
        let mut total = 0_i16;
        for level in 1..=levels {
            let die_value = if level == 1 && includes_first_character_level {
                i16::from(die)
            } else {
                crate::rules_core::durability::average_hit_die_value(die)
            };
            total += (die_value + constitution_modifier).max(1);
        }
        Ok(total)
    }

    /// The skill points `levels` levels of this class contribute: skill ranks
    /// per level plus `intelligence_modifier`, at least 1 per level (PF1).
    /// Race and favored-class extras are not the class's and are not added
    /// here. A record with no skill-ranks-per-level row is
    /// [`SKILL_POINTS_UNKNOWN`], never 0 (see [`Self::skill_ranks_per_level`]).
    pub fn skill_points(&self, levels: u8, intelligence_modifier: i16) -> Result<i16, ChassisUnknown> {
        let ranks = self.skill_ranks_per_level.ok_or_else(|| ChassisUnknown {
            id: SKILL_POINTS_UNKNOWN,
            message: format!(
                "{} ({}:class:{}): the converted record states no skill ranks per level, so \
                 this class's skill points are Unknown",
                self.display_name, self.book, self.slug
            ),
        })?;
        Ok(i16::from(levels) * (i16::from(ranks) + intelligence_modifier).max(1))
    }

    /// Every level `1..=max_level` as `(level, bab, fort, ref, will)`, or
    /// `None` when any level fails to resolve — a partial progression is never
    /// returned.
    pub fn full_progression(&self) -> Option<Vec<ProgressionRow>> {
        let mut rows = Vec::with_capacity(usize::from(self.max_level));
        for level in 1..=self.max_level {
            let row = self.row_at(level)?;
            rows.push((level, row.base_attack_bonus, row.fort_save, row.ref_save, row.will_save));
        }
        Some(rows)
    }
}

/// PF1's good class-level save progressions, as closed forms over the
/// class's own level (integer division truncates, as [`ClassChassis::row_at`]
/// does once at the boundary): the base-class table's `level/2 + 2` and the
/// prestige-class table's `(level+1)/2` (CRB prestige tables: +1 at 1st,
/// +5 at 10th).
const SAVE_GOOD_FORMS: [fn(i64) -> i64; 2] = [|l| l / 2 + 2, |l| (l + 1) / 2];
/// PF1's poor class-level save progressions: base `level/3`, prestige
/// `(level+1)/3` (+0 at 1st, +3 at 10th).
const SAVE_POOR_FORMS: [fn(i64) -> i64; 2] = [|l| l / 3, |l| (l + 1) / 3];

/// `true` when `e` contains an unresolved `Choice` term -- a term that
/// prints as words, not a number.
fn expr_has_choice(e: &Expr) -> bool {
    match e {
        Expr::Choice(_) => true,
        Expr::Sum(terms) => terms.iter().any(expr_has_choice),
        Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Min(a, b) | Expr::Max(a, b) => {
            expr_has_choice(a) || expr_has_choice(b)
        }
        Expr::Floor(a) | Expr::Ceil(a) => expr_has_choice(a),
        _ => false,
    }
}

/// `true` when `e` reads nothing but constants and `ClassLevel(level_var)`
/// through arithmetic -- the only inputs a class-level save progression has.
fn reads_only_own_level(e: &Expr, level_var: &str) -> bool {
    match e {
        Expr::Const(_) => true,
        Expr::ClassLevel(c) => c == level_var,
        Expr::Sum(terms) => terms.iter().all(|t| reads_only_own_level(t, level_var)),
        Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Min(a, b) | Expr::Max(a, b) => {
            reads_only_own_level(a, level_var) && reads_only_own_level(b, level_var)
        }
        Expr::Floor(a) | Expr::Ceil(a) => reads_only_own_level(a, level_var),
        _ => false,
    }
}

/// The number `applies` states as this class's own level ceiling: the
/// `Compare { ClassLevel(slug), Lte, Const(n) }` term the converter writes from
/// the record's own stated ceiling. `None` when the record states none.
fn ceiling_in(gate: &Applies, slug: &str) -> Option<u8> {
    match gate {
        Applies::Compare { lhs: Expr::ClassLevel(c), op: Cmp::Lte, rhs: Expr::Const(n) }
            if c == slug =>
        {
            u8::try_from(*n).ok()
        }
        Applies::All(terms) => terms.iter().find_map(|t| ceiling_in(t, slug)),
        _ => None,
    }
}

/// Every distinct class id `Expr::ClassLevel` names inside `e`.
fn class_level_ids(e: &Expr, out: &mut std::collections::BTreeSet<String>) {
    match e {
        Expr::ClassLevel(c) => {
            out.insert(c.clone());
        }
        Expr::Sum(terms) => terms.iter().for_each(|t| class_level_ids(t, out)),
        Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Min(a, b) | Expr::Max(a, b) => {
            class_level_ids(a, out);
            class_level_ids(b, out);
        }
        Expr::Floor(a) | Expr::Ceil(a) => class_level_ids(a, out),
        _ => {}
    }
}

fn number_of(rule: &SheetRule) -> Option<&Expr> {
    match &rule.value {
        SheetValue::Number(e) => Some(e),
        _ => None,
    }
}

/// The plain text of a `StatBlock` prose segment labeled `label` on
/// `rule`, or `None` when no such row exists OR its pieces are not ALL
/// plain `ProsePiece::Text` (a slot/choice/dice piece cannot be resolved
/// statically here — honest absence, never a guessed evaluation of a
/// character-dependent piece at read-record time). `pick_last`/`applies`
/// are ignored: every StatBlock row observed in this corpus today
/// (`"Hit die"`) carries neither, and reading only the first unconditional
/// match keeps this the same "principal row wins" precedent
/// `chassis_from_rules` already applies to `BaseAttack`.
fn stat_block_prose_text(rule: &SheetRule, label: &str) -> Option<String> {
    let segment = rule.prose.iter().find(|seg| match &seg.family {
        ProseFamily::StatBlock(l) => l == label,
        _ => false,
    })?;
    let mut text = String::new();
    for piece in &segment.pieces {
        match piece {
            ProsePiece::Text(s) => text.push_str(s),
            _ => return None,
        }
    }
    Some(text)
}

/// Parses a hit-die StatBlock row's text (`"d10"`) into its numeric size
/// (`10`). `None` for any shape other than a bare `d<digits>` — never a
/// guessed size for text this parser does not recognize.
fn parse_die_size(text: &str) -> Option<u8> {
    text.strip_prefix('d')?.parse().ok()
}

/// Parses a skill-ranks-per-level StatBlock row's text into its numeric
/// value, to the same "plain digits, nothing else" contract [`parse_die_size`]
/// uses — never a guessed rank count for a shape this parser does not
/// recognize.
fn parse_skill_ranks(text: &str) -> Option<u8> {
    text.trim().parse().ok()
}

/// Builds one class's chassis from its converted rule file's rows, or `None`
/// when the file is not a chassis-bearing class record.
fn chassis_from_rules(book: &str, slug: &str, rules: &[SheetRule]) -> Option<ClassChassis> {
    // `rules` arrives in file order, which is id order: the principal rule
    // (`<book>:class:<slug>`) first, then its `#bonus<N>` siblings. A record
    // that states more than one base-attack progression states the alternates
    // behind their own extra gate -- Ultimate Intrigue's Vigilante is the only
    // one in this corpus, whose principal row is the default and whose sibling
    // is the vigilante-specialisation toggle. Taking the principal (first) row
    // is the same choice the corpus-side reader made by looking for the
    // toggle-off `,0` gate, made without reading a token.
    let base_attack = rules.iter().find(|r| r.target.as_ref() == Some(&BonusTarget::BaseAttack)).and_then(number_of)?;
    let mut saves: [Option<&Expr>; 3] = [None, None, None];
    let mut save_words = [false; 3];
    for rule in rules {
        let Some(BonusTarget::BaseSave(save)) = rule.target.as_ref() else { continue };
        let index = match save {
            Save::Fortitude => 0,
            Save::Reflex => 1,
            Save::Will => 2,
        };
        match number_of(rule) {
            Some(expr) if saves[index].is_none() => saves[index] = Some(expr),
            Some(_) => {}
            None => save_words[index] = true,
        }
    }
    let (Some(fort), Some(refl), Some(will)) = (saves[0], saves[1], saves[2]) else {
        return None;
    };
    let principal = rules.first()?;
    let tags = principal.tags.clone();
    // The id this record's own expressions bind their level to -- see
    // `ClassChassis::level_var`. Exactly one for every chassis-bearing record
    // in this corpus; a record naming none (or more than one) keeps the file
    // slug, which is what the dispatchers already pass.
    let mut ids = std::collections::BTreeSet::new();
    for e in [base_attack, fort, refl, will] {
        class_level_ids(e, &mut ids);
    }
    let level_var =
        if ids.len() == 1 { ids.into_iter().next().unwrap_or_default() } else { slug.to_string() };
    let max_level = rules
        .iter()
        .find_map(|r| ceiling_in(&r.applies, &level_var))
        .unwrap_or(if tags.iter().any(|t| t == "Prestige") {
            DEFAULT_PRESTIGE_MAX_LEVEL
        } else {
            DEFAULT_MAX_LEVEL
        });
    let hit_die = stat_block_prose_text(principal, "Hit die").and_then(|text| parse_die_size(&text));
    let skill_ranks_per_level =
        stat_block_prose_text(principal, "Skill ranks per level").and_then(|text| parse_skill_ranks(&text));
    Some(ClassChassis {
        book: book.to_string(),
        slug: slug.to_string(),
        level_var,
        display_name: crate::rules_core::sheet_rule::display_label(principal),
        tags,
        max_level,
        base_attack: base_attack.clone(),
        saves: [fort.clone(), refl.clone(), will.clone()],
        save_words,
        hit_die,
        skill_ranks_per_level,
    })
}

fn load_book(root: &Path, book: &str, out: &mut BTreeMap<(String, String), ClassChassis>) {
    let dir = root.join("data/sheet_rules").join(book).join("class");
    let Ok(entries) = std::fs::read_dir(&dir) else { return };
    let mut files: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    files.sort();
    for file in files {
        let Some(slug) = file.file_stem().and_then(|s| s.to_str()).map(str::to_string) else {
            continue;
        };
        let Ok(text) = std::fs::read_to_string(&file) else { continue };
        let Ok(rules) = serde_json::from_str::<Vec<SheetRule>>(&text) else { continue };
        if let Some(chassis) = chassis_from_rules(book, &slug, &rules) {
            out.insert((book.to_string(), slug), chassis);
        }
    }
}

/// Every chassis-bearing class record under `books`, keyed by `(book, slug)`.
/// Loaded once per process.
pub fn records(books: &[&str]) -> BTreeMap<(String, String), ClassChassis> {
    let root = repo_root();
    let mut out = BTreeMap::new();
    for book in books {
        load_book(&root, book, &mut out);
    }
    out
}

/// One class's chassis in one named book, cached across calls.
pub fn record(book: &str, slug: &str) -> Option<&'static ClassChassis> {
    static CACHE: OnceLock<RecordCache> = OnceLock::new();
    let cache = CACHE.get_or_init(|| std::sync::Mutex::new(BTreeMap::new()));
    let key = (book.to_string(), slug.to_string());
    let mut guard = cache.lock().expect("class chassis cache");
    if let Some(hit) = guard.get(&key) {
        return *hit;
    }
    let mut loaded = BTreeMap::new();
    load_book(&repo_root(), book, &mut loaded);
    let found: Option<&'static ClassChassis> =
        loaded.remove(&key).map(|c| &*Box::leak(Box::new(c)));
    guard.insert(key, found);
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warrior_full_bab_and_the_crb_save_progressions_come_off_the_converted_record() {
        let warrior = record("core_rulebook", "warrior").expect("CRB carries the Warrior class");
        assert_eq!(warrior.display_name, "Warrior");
        assert_eq!(warrior.max_level, 20, "MAXLEVEL:20 converts to the applies ceiling");
        let row = warrior.row_at(10).expect("level 10 is inside the ceiling");
        // Full BAB -> 10. Good Fortitude (`level/2 + 2`) -> 7. Poor Reflex and
        // Will (`level/3`) -> 3.
        assert_eq!(row.base_attack_bonus, 10);
        assert_eq!(row.fort_save, 7);
        assert_eq!(row.ref_save, 3);
        assert_eq!(row.will_save, 3);
    }

    #[test]
    fn commoner_half_bab_and_three_poor_saves_truncate_the_same_way_the_book_prints() {
        let commoner = record("core_rulebook", "commoner").expect("CRB carries the Commoner class");
        let row = commoner.row_at(9).expect("level 9 resolves");
        assert_eq!(row.base_attack_bonus, 4, "9/2 truncates to 4");
        assert_eq!(row.fort_save, 3, "9/3 = 3, all three saves poor");
        assert_eq!(row.ref_save, 3);
        assert_eq!(row.will_save, 3);
    }

    #[test]
    fn a_level_above_the_converted_ceiling_resolves_nothing() {
        let warrior = record("core_rulebook", "warrior").expect("CRB carries the Warrior class");
        assert!(warrior.row_at(21).is_none(), "MAXLEVEL:20 caps resolution");
        assert!(warrior.row_at(0).is_none(), "level 0 is not a level");
    }

    #[test]
    fn a_prestige_class_carries_its_own_ten_level_ceiling() {
        let delver = record("adventurers_guide", "pathfinder_delver")
            .expect("Adventurer's Guide carries this prestige class");
        assert_eq!(delver.max_level, 10);
        assert!(delver.row_at(11).is_none());
        assert!(delver.row_at(10).is_some());
    }

    #[test]
    fn a_record_whose_class_id_is_redacted_still_binds_its_own_level() {
        // Pathfinder Delver's name is product identity; its corpus record ships
        // a codex-named placeholder key. Until SD-36 F1c-3 its converted
        // expressions named `slug(placeholder)`, not the file slug, and binding
        // the file slug would have made every level evaluate to zero while
        // still LOOKING like a resolved chassis -- so the binding is read off
        // the expressions. Since F1c-3 the converter keys every class by its
        // unit slug (`ctx::own_class_id`), so the two agree; the binding is
        // still read off the expressions. Moderate BAB (`level*3/4`) at level
        // 10 is 7, and the three saves (`(level+1)/2` Reflex, `(level+1)/3`
        // Fortitude and Will) are 5, 3 and 3.
        let delver =
            record("adventurers_guide", "pathfinder_delver").expect("the record is present");
        assert_eq!(delver.level_var, delver.slug, "the class is keyed by its unit slug since F1c-3");
        let row = delver.row_at(10).expect("level 10 resolves");
        assert_ne!(row.base_attack_bonus, 0, "a zero here is the wrong-binding failure");
        assert_eq!(row.base_attack_bonus, 7);
    }

    #[test]
    fn a_record_with_no_chassis_rows_is_absent_rather_than_half_built() {
        // Evangelist used to be this test's example: before SD-36 Epic E
        // CONV-05, its converted record's own BAB/save formulas were wiped
        // to WORDS by an unrelated degrading token elsewhere on the same
        // record (`convert.rs`'s old record-wide degradation policy). CONV-05
        // fixed that -- Evangelist now correctly resolves a real chassis (¾
        // BAB, good Reflex; see `generic_class_chassis.rs`'s
        // `every_conventional_class_in_class_family_books_resolves`) -- so it no
        // longer exercises "no chassis rows at all". `occult_adventures`'s
        // Psychic Detective genuinely converts no `BaseAttack`/`BaseSave` row
        // (verified: `data/sheet_rules/occult_adventures/class/psychic_
        // detective.json` carries exactly one line, a `CasterLevel` Number);
        // it must not appear as a chassis with a guessed progression.
        assert!(record("occult_adventures", "psychic_detective").is_none());
    }

    // -----------------------------------------------------------------
    // F0-check finding 5 (RED first): the `hit_die`/`skill_ranks_per_level`
    // readers `epic-f-class-completion.md` §2's F0 territory list required
    // and F0's five landed commits never added.
    // -----------------------------------------------------------------

    #[test]
    fn hit_die_is_read_off_the_stat_block_prose_row() {
        let warrior = record("core_rulebook", "warrior").expect("CRB carries the Warrior class");
        assert_eq!(warrior.hit_die, Some(10), "Warrior's printed hit die is d10");
        let commoner = record("core_rulebook", "commoner").expect("CRB carries the Commoner class");
        assert_eq!(commoner.hit_die, Some(6), "Commoner's printed hit die is d6");
        let wizard = record("core_rulebook", "wizard").expect("CRB carries the Wizard class");
        assert_eq!(wizard.hit_die, Some(6), "Wizard's printed hit die is d6");
    }

    #[test]
    fn every_chassis_bearing_class_in_the_corpus_has_a_hit_die() {
        // §0.4 (review finding 13): 178 of 185 class records carry the
        // `StatBlock "Hit die"` prose row; the 7 that do not
        // (`occult_adventures/psychic_detective`, three
        // `ultimate_psionics/*`, `bestiary/sorcerer_cleric_arcane`, two
        // `ultimate_intrigue/*`) are named by id. Measured directly
        // (F0-check finding 5): every one of those same 7 records ALSO
        // carries no `BaseAttack`/`BaseSave` row at all, so `record()`
        // already returns `None` for every one of them
        // (`a_record_with_no_chassis_rows_is_absent_rather_than_half_built`,
        // above). A further, previously unnoted eighth exception measured
        // the same way: `core_rulebook/monk` carries the `"Hit die"` row
        // but its principal rule degraded entirely to `value: Text` (no
        // `BaseAttack`/`BaseSave` row either) -- same "record()
        // returns None, never a half-built chassis" outcome, for a
        // different, unrelated reason (degradation, not an absent row).
        // 185 - 7 - 1 = 177 is the real chassis-bearing population
        // measured today: there is no record in this corpus with a REAL
        // chassis (BAB + all three saves) but a missing hit die. This test
        // proves that fact directly, over the full corpus, rather than
        // trusting it stays true by construction.
        let books = crate::rules_core::class_census::prestige_scan_books();
        let book_refs: Vec<&str> = books.iter().map(String::as_str).collect();
        let all = records(&book_refs);
        assert_eq!(all.len(), 177, "measured chassis-bearing population moved off 177");
        let missing: Vec<String> = all
            .values()
            .filter(|chassis| chassis.hit_die.is_none())
            .map(|chassis| format!("{}:{}", chassis.book, chassis.slug))
            .collect();
        assert!(
            missing.is_empty(),
            "chassis-bearing class(es) with no hit die (must be named, not silently defaulted): \
             {missing:?}"
        );
    }

    #[test]
    fn the_seven_hit_die_absent_records_have_no_chassis_at_all() {
        // The other half of the same fact: none of §0.4's 7 named
        // exceptions appears in `records()` in the first place (they
        // convert no BaseAttack/BaseSave row either), so `hit_die` cannot
        // even be queried on them today -- named directly, by id, so a
        // future corpus change that gives one of these seven a real
        // chassis (and then needs a real hit die too) surfaces here.
        for (book, slug) in [
            ("occult_adventures", "psychic_detective"),
            ("ultimate_psionics", "gifted_blade"),
            ("ultimate_psionics", "gifted_blade_marksman_power_list"),
            ("ultimate_psionics", "unlocked_talent"),
            ("bestiary", "sorcerer_cleric_arcane"),
            ("ultimate_intrigue", "vwarlock"),
            ("ultimate_intrigue", "vcabalist"),
        ] {
            assert!(
                record(book, slug).is_none(),
                "{book}:{slug} was expected to carry no chassis at all (§0.4's 7 named \
                 hit-die-absent exceptions) -- it now resolves one, so its hit_die must be \
                 re-examined, not silently accepted as None"
            );
        }
    }

    #[test]
    fn skill_ranks_per_level_is_read_for_every_chassis_bearing_class() {
        // SD-36 Epic F3b2 replaces F0-check finding 5's "honest absence" pin:
        // the converter now carries PCGen's class-line `STARTSKILLPTS` onto the
        // class principal as a `StatBlock "Skill ranks per level"` row
        // (`crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs`), and
        // this reader (unchanged) reads it. 171 of the 177 chassis-bearing
        // class records now state their ranks; the six that do not are named.
        let books = crate::rules_core::class_census::prestige_scan_books();
        let book_refs: Vec<&str> = books.iter().map(String::as_str).collect();
        let all = records(&book_refs);
        assert_eq!(all.len(), 177, "measured chassis-bearing population moved off 177");
        let mut missing: Vec<String> = all
            .values()
            .filter(|chassis| chassis.skill_ranks_per_level.is_none())
            .map(|chassis| format!("{}:{}", chassis.book, chassis.slug))
            .collect();
        missing.sort();
        // The six whose `STARTSKILLPTS` is a formula, not a number, so the
        // converter writes no row and names them in
        // `data/sheet_rules/_defects/skill-ranks-unresolved.json`: the five
        // Bestiary creature-type classes (`0+BaseClassSkillPts`,
        // `ce_classes_race.lst`) and the Eidolon (`EidolonSkillPoints`, a
        // variable its own closure does not define, `apg_classes.lst:211`).
        // None is a census class.
        assert_eq!(
            missing,
            [
                "advanced_players_guide:eidolon",
                "bestiary:construct",
                "bestiary:ooze",
                "bestiary:plant",
                "bestiary:undead",
                "bestiary:vermin",
            ],
            "chassis-bearing class(es) with no skill ranks per level moved"
        );
        // Oracle (hand-read PF1, Core Rulebook class tables): Fighter 2 (p.55),
        // Rogue 8 (p.67), Wizard 2 (p.77), Bard 6 (p.35), Ranger 6 (p.64),
        // Loremaster 4 (p.385), Mystic Theurge 2 (p.387); Warrior 2 (p.449).
        for (slug, ranks) in [
            ("fighter", 2),
            ("rogue", 8),
            ("wizard", 2),
            ("bard", 6),
            ("ranger", 6),
            ("loremaster", 4),
            ("mystic_theurge", 2),
            ("warrior", 2),
        ] {
            let chassis = record("core_rulebook", slug).unwrap_or_else(|| panic!("CRB {slug}"));
            assert_eq!(chassis.skill_ranks_per_level, Some(ranks), "{slug}");
        }
    }

    // -----------------------------------------------------------------
    // SD-36 Epic F3a (spec §5, review finding 11; acceptance F3.0 context):
    // `save_shape` over every class chassis record the class registry knows.
    // -----------------------------------------------------------------

    /// Every `(record id, chassis)` the class registry knows: for each
    /// `class_census::census()` id (every canonical registry, the prestige
    /// sweep, and `generic_class_chassis` -- generic families including the
    /// CRB/APG prestige rows F2a appended, and every bespoke-owned class),
    /// the converted chassis record in each book that census row names.
    /// Ids whose books carry no chassis record are returned separately.
    fn registry_chassis_records() -> (Vec<(String, &'static ClassChassis)>, Vec<String>) {
        let mut found = Vec::new();
        let mut without = Vec::new();
        for (class_id, entry) in crate::rules_core::class_census::census() {
            let slug = class_id.strip_prefix("class:").unwrap_or(&class_id).to_string();
            let before = found.len();
            for book in &entry.books {
                if let Some(chassis) = record(book, &slug) {
                    found.push((format!("{book}:class:{slug}"), chassis));
                }
            }
            if found.len() == before {
                without.push(class_id);
            }
        }
        (found, without)
    }

    /// The save slots whose converted `Expr` is a faithful conversion of a
    /// source formula that is not a PF1 class-level save progression --
    /// named, one by one, with the source LST formula each converts
    /// (`data/corpus/<book>/class/<slug>.json`, `SAVE|BASE.<save>|...`).
    /// They stay `Unrecognized` (and so stay Blocked in a mix, F3b) rather
    /// than folding as good or poor; correcting them is a source-data
    /// question, not a classifier one. See
    /// `docs/release/SD-36-consolidation/artifacts/epic-f/stage-f2-f3/f3a-save-shapes.md`.
    const NAMED_UNRECOGNIZED_SAVES: [(&str, usize); 14] = [
        ("adventurers_guide:class:mammoth_rider", 0), // (classlevel+2)/2
        ("inner_sea_combat:class:pure_legion_enforcer", 0), // classlevel+3/2
        ("inner_sea_combat:class:pure_legion_enforcer", 1), // classlevel+1/3
        ("inner_sea_combat:class:pure_legion_enforcer", 2), // classlevel+3/2
        ("inner_sea_combat:class:ulfen_guard", 0), // classlevel+3/2
        ("inner_sea_combat:class:ulfen_guard", 1), // classlevel+1/3
        ("inner_sea_combat:class:ulfen_guard", 2), // classlevel+3/2
        ("inner_sea_gods:class:evangelist", 1),    // classlevel/3+1
        ("inner_sea_gods:class:exalted", 0),       // classlevel+1/3
        ("inner_sea_gods:class:exalted", 1),       // classlevel+1/3
        ("inner_sea_gods:class:exalted", 2),       // classlevel+1/2
        ("inner_sea_gods:class:sentinel", 0),      // classlevel+1/2
        ("inner_sea_gods:class:sentinel", 1),      // classlevel+1/3
        ("inner_sea_gods:class:sentinel", 2),      // classlevel+1/3
    ];

    #[test]
    fn every_generic_class_save_shape_is_recognized_or_named() {
        let (records, _) = registry_chassis_records();
        // 135 chassis records for 132 of the census's 137 ids (F3a
        // measurement; 3 ids resolve a record in two books, each listed per
        // book -- cyphermage, hellknight, red_mantis_assassin -- and the
        // 5 ids without one are named by
        // `registry_ids_without_a_chassis_record_are_named`).
        assert_eq!(records.len(), 135, "registry chassis-record population moved off 135");
        let mut not_good_or_poor = Vec::new();
        for (id, chassis) in &records {
            for index in 0..3 {
                let shape = chassis.save_shape(index);
                if !matches!(shape, Some(SaveProgression::Good | SaveProgression::Poor)) {
                    not_good_or_poor.push((id.clone(), index, shape));
                }
            }
        }
        let named: Vec<(String, usize, Option<SaveProgression>)> = NAMED_UNRECOGNIZED_SAVES
            .iter()
            .map(|(id, index)| (id.to_string(), *index, Some(SaveProgression::Unrecognized)))
            .collect();
        let unnamed: Vec<String> = not_good_or_poor
            .iter()
            .filter(|slot| !named.contains(slot))
            .map(|(id, index, shape)| format!("({id}, {index}, {shape:?})"))
            .collect();
        let vanished: Vec<String> = named
            .iter()
            .filter(|slot| !not_good_or_poor.contains(slot))
            .map(|(id, index, _)| format!("({id}, {index})"))
            .collect();
        assert!(
            unnamed.is_empty() && vanished.is_empty(),
            "{} of {} save slots ({} records x 3) are not Good/Poor.\n\
             NOT NAMED (classify by mechanism, fix the classifier or name it):\n{}\n\
             NAMED BUT NOW Good/Poor or gone (re-examine, then unpin):\n{}",
            not_good_or_poor.len(),
            records.len() * 3,
            records.len(),
            unnamed.join("\n"),
            vanished.join("\n")
        );
    }

    #[test]
    fn registry_ids_without_a_chassis_record_are_named() {
        // 5 of the census's 137 ids have no converted chassis record in any
        // book their census row names (their BAB/saves come from bespoke
        // tables, not a `ClassChassis`): Core Rulebook Monk, whose principal
        // rule degraded to words and carries no `BaseAttack` row (see
        // `every_chassis_bearing_class_in_the_corpus_has_a_hit_die`), and the
        // four Pathfinder Unchained classes, whose converted records
        // (`data/sheet_rules/pathfinder_unchained/class/*.json`) are one
        // rule each with no `BaseAttack`/`BaseSave` target at all.
        let (_, without) = registry_chassis_records();
        assert_eq!(
            without,
            [
                "class:monk",
                "class:unchained_barbarian",
                "class:unchained_monk",
                "class:unchained_rogue",
                "class:unchained_summoner",
            ]
        );
    }

    /// A minimal chassis-bearing rule set for `slug` with the three given
    /// save values (BAB = class level).
    fn synthetic_rules(slug: &str, saves: [SheetValue; 3]) -> Vec<SheetRule> {
        let mut principal: SheetRule = serde_json::from_str(FIGHTER_PRINCIPAL_RULE_JSON)
            .expect("fixture JSON must parse as a SheetRule");
        principal.value = SheetValue::Number(Expr::ClassLevel(slug.to_owned()));
        principal.applies = Applies::Always;
        let mut rules = vec![principal.clone()];
        for (save, value) in [Save::Fortitude, Save::Reflex, Save::Will].into_iter().zip(saves) {
            let mut rule = principal.clone();
            rule.target = Some(BonusTarget::BaseSave(save));
            rule.value = value;
            rules.push(rule);
        }
        rules
    }

    fn level(slug: &str) -> Box<Expr> {
        Box::new(Expr::ClassLevel(slug.to_owned()))
    }

    #[test]
    fn save_shape_reads_good_poor_and_names_the_rest() {
        let slug = "synthetic";
        let good = SheetValue::Number(Expr::Sum(vec![
            Expr::Div(level(slug), Box::new(Expr::Const(2))),
            Expr::Const(2),
        ]));
        let poor = SheetValue::Number(Expr::Div(level(slug), Box::new(Expr::Const(3))));
        // `classlevel+1/3`, the Ulfen Guard source shape: full level, not a save.
        let odd = SheetValue::Number(Expr::Sum(vec![
            Expr::ClassLevel(slug.to_owned()),
            Expr::Div(Box::new(Expr::Const(1)), Box::new(Expr::Const(3))),
        ]));
        let chassis = chassis_from_rules("core_rulebook", slug, &synthetic_rules(slug, [good, poor, odd]))
            .expect("three Number saves build a chassis");
        assert_eq!(chassis.save_shape(0), Some(SaveProgression::Good));
        assert_eq!(chassis.save_shape(1), Some(SaveProgression::Poor));
        assert_eq!(chassis.save_shape(2), Some(SaveProgression::Unrecognized));
        assert_eq!(chassis.save_shape(3), None, "there is no fourth save");

        // A save reading character level (not the class's own) is not a
        // class-level progression, whatever values it takes single-class.
        let char_level = SheetValue::Number(Expr::Div(Box::new(Expr::Level), Box::new(Expr::Const(3))));
        let poor = SheetValue::Number(Expr::Div(level(slug), Box::new(Expr::Const(3))));
        let chassis = chassis_from_rules(
            "core_rulebook",
            slug,
            &synthetic_rules(slug, [char_level, poor.clone(), poor]),
        )
        .expect("builds");
        assert_eq!(chassis.save_shape(0), Some(SaveProgression::Unrecognized));
    }

    #[test]
    fn save_shape_names_the_words_not_expr_symptom_degraded() {
        let slug = "synthetic";
        let poor = || SheetValue::Number(Expr::Div(level(slug), Box::new(Expr::Const(3))));
        // A Fortitude row that converted to WORDS, beside a sibling that did
        // convert: never trusted as whatever the sibling says.
        let mut rules = synthetic_rules(slug, [SheetValue::Text, poor(), poor()]);
        let mut sibling = rules[1].clone();
        sibling.value = poor();
        rules.push(sibling);
        let chassis = chassis_from_rules("core_rulebook", slug, &rules).expect("builds off the sibling");
        assert_eq!(chassis.save_shape(0), Some(SaveProgression::Degraded));
        assert_eq!(chassis.save_shape(1), Some(SaveProgression::Poor));

        // An unresolved `Choice` term prints as words.
        let choice = SheetValue::Number(Expr::Sum(vec![
            Expr::Div(level(slug), Box::new(Expr::Const(3))),
            Expr::Choice("pick".into()),
        ]));
        let chassis = chassis_from_rules("core_rulebook", slug, &synthetic_rules(slug, [poor(), poor(), choice]))
            .expect("builds");
        assert_eq!(chassis.save_shape(2), Some(SaveProgression::Degraded));
    }

    // -----------------------------------------------------------------
    // SD-36 Epic F3.0 (spec §5 review finding 3): a missing hit die or
    // skill-ranks-per-level is a named Unknown on HP / skill points, never
    // a silent 0.
    // -----------------------------------------------------------------

    fn synthetic_chassis_without_stat_block() -> ClassChassis {
        let slug = "synthetic";
        let poor = || SheetValue::Number(Expr::Div(level(slug), Box::new(Expr::Const(3))));
        chassis_from_rules("core_rulebook", slug, &synthetic_rules(slug, [poor(), poor(), poor()]))
            .expect("builds")
    }

    #[test]
    fn a_class_missing_hit_die_reports_hp_unknown() {
        // The Unknown arm, on a chassis whose record carries no
        // `StatBlock "Hit die"` row.
        let chassis = synthetic_chassis_without_stat_block();
        assert_eq!(chassis.hit_die, None);
        let unknown = chassis.hit_points(3, true, 2).expect_err("no hit die -> Unknown, never 0");
        assert_eq!(unknown.id, HIT_POINTS_UNKNOWN);
        assert!(unknown.message.contains(&chassis.display_name), "names the class: {}", unknown.message);

        // Oracle (hand-worked PF1, non-rolling average = die/2 + 1): Warrior
        // d10, 3 levels including character level 1, Con +2:
        // (10+2) + (6+2) + (6+2) = 28. Taken later (no maximized die): 3 x 8 = 24.
        // Commoner d6, 2 levels, Con -3: max(6-3,1) + max(4-3,1) = 3 + 1 = 4.
        let warrior = record("core_rulebook", "warrior").expect("CRB Warrior");
        assert_eq!(warrior.hit_points(3, true, 2), Ok(28));
        assert_eq!(warrior.hit_points(3, false, 2), Ok(24));
        let commoner = record("core_rulebook", "commoner").expect("CRB Commoner");
        assert_eq!(commoner.hit_points(2, true, -3), Ok(4));

        // Over every chassis record the registry knows: HP is Unknown exactly
        // where the hit die is absent (0 of 135 today), named by class.
        let (records, _) = registry_chassis_records();
        assert_eq!(records.len(), 135);
        let mut unknown_hp = Vec::new();
        for (id, chassis) in &records {
            match (chassis.hit_die, chassis.hit_points(1, true, 0)) {
                (Some(die), Ok(hp)) => assert_eq!(hp, i16::from(die), "{id}: level 1 = max die"),
                (None, Err(u)) if u.id == HIT_POINTS_UNKNOWN => unknown_hp.push(id.clone()),
                (die, got) => panic!("{id}: hit_die {die:?} but hit_points {got:?}"),
            }
        }
        assert!(unknown_hp.is_empty(), "classes whose HP is Unknown (name them): {unknown_hp:?}");
    }

    #[test]
    fn a_class_missing_skill_ranks_reports_skill_points_unknown() {
        // A chassis with no `StatBlock "Skill ranks per level"` row is the
        // Unknown case -- named, never 0. Since SD-36 F3b2 every real
        // chassis-bearing record states its ranks
        // (`skill_ranks_per_level_is_read_for_every_chassis_bearing_class`),
        // so the Unknown path is exercised on the synthetic chassis.
        let mut chassis = synthetic_chassis_without_stat_block();
        assert_eq!(chassis.skill_ranks_per_level, None);
        let unknown = chassis.skill_points(3, 1).expect_err("no ranks row -> Unknown, never 0");
        assert_eq!(unknown.id, SKILL_POINTS_UNKNOWN);
        assert!(unknown.message.contains(&chassis.display_name), "names the class: {}", unknown.message);

        // Oracle (hand-worked PF1: ranks + Int per level, minimum 1 per
        // level): 2 ranks, Int -2 over 3 levels -> 3 x max(0, 1) = 3;
        // 4 ranks, Int +1 over 5 levels -> 5 x 5 = 25.
        chassis.skill_ranks_per_level = Some(2);
        assert_eq!(chassis.skill_points(3, -2), Ok(3));
        chassis.skill_ranks_per_level = Some(4);
        assert_eq!(chassis.skill_points(5, 1), Ok(25));

        // The real Warrior: 2 ranks (CRB p.449), Int -2 over 3 levels -> 3.
        let warrior = record("core_rulebook", "warrior").expect("CRB Warrior");
        assert_eq!(warrior.skill_points(3, -2), Ok(3));

        // Over every chassis record the registry knows: skill points are
        // Known for 135 of 135 (Unknown for none; any Unknown is named).
        let (records, _) = registry_chassis_records();
        assert_eq!(records.len(), 135);
        let unknown: Vec<&String> = records
            .iter()
            .filter(|(_, c)| matches!(c.skill_points(1, 0), Err(u) if u.id == SKILL_POINTS_UNKNOWN))
            .map(|(id, _)| id)
            .collect();
        assert!(unknown.is_empty(), "skill points Unknown (name them): {unknown:?}");
    }

    #[test]
    fn stat_block_prose_text_ignores_a_row_with_a_non_text_piece() {
        // Direct unit coverage of the helper's own contract: a piece this
        // reader cannot resolve statically (here, a `Slot`) must make the
        // whole row unreadable, never a truncated guess -- built from the
        // real Fighter record's own JSON shape with only the `prose` field
        // replaced, rather than hand-listing every `SheetRule` field.
        let mut rule: SheetRule = serde_json::from_str(FIGHTER_PRINCIPAL_RULE_JSON)
            .expect("fixture JSON must parse as a SheetRule");
        rule.prose = vec![crate::rules_core::sheet_rule::ProseSegment {
            family: ProseFamily::StatBlock("Hit die".to_owned()),
            pieces: vec![
                ProsePiece::Text("d".to_owned()),
                ProsePiece::Slot(crate::rules_core::sheet_rule::Expr::Level),
            ],
            applies: None,
            pick_last: false,
            suppress_when_all_zero: false,
        }];
        assert_eq!(stat_block_prose_text(&rule, "Hit die"), None);

        // A row entirely of Text pieces DOES resolve, concatenated.
        rule.prose = vec![crate::rules_core::sheet_rule::ProseSegment {
            family: ProseFamily::StatBlock("Hit die".to_owned()),
            pieces: vec![ProsePiece::Text("d".to_owned()), ProsePiece::Text("10".to_owned())],
            applies: None,
            pick_last: false,
            suppress_when_all_zero: false,
        }];
        assert_eq!(stat_block_prose_text(&rule, "Hit die"), Some("d10".to_owned()));

        // A label that does not match any row resolves to None.
        assert_eq!(stat_block_prose_text(&rule, "Skill ranks per level"), None);
    }

    /// The real `core_rulebook:class:fighter` principal rule's own JSON
    /// shape (its `prose` field is overwritten by the test above), used so
    /// this test does not have to hand-list every `SheetRule` field.
    const FIGHTER_PRINCIPAL_RULE_JSON: &str = r#"{
        "id": "core_rulebook:class:fighter",
        "label": "Fighter",
        "value": {"Number": {"ClassLevel": "fighter"}},
        "prose": [],
        "applies": {"All": [
            {"Compare": {"lhs": {"ClassLevel": "fighter"}, "op": "Lte", "rhs": {"Const": 20}}}
        ]},
        "target": "BaseAttack",
        "print": true,
        "pool": "",
        "tags": ["Base", "PC"],
        "subject": "Character",
        "repeatable": false,
        "grants": [],
        "provenance": {
            "book": "core_rulebook",
            "kind": "class",
            "closure_rows": ["pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:139"],
            "oracle_pin": "0000000000000000000000000000000000000000",
            "converter_version": "sheet_rule_convert/0.15.0"
        }
    }"#;

    /// SD-36 Epic E engine-P1-4 (review-caught third path): a class whose principal record's
    /// real name was redacted as Product Identity carries the ingest pipeline's placeholder
    /// label. `ClassChassis::display_name` must resolve it through
    /// `crate::rules_core::sheet_rule::display_label`, never `principal.label.clone()` raw --
    /// the frontend prints `display_name` directly as the class's name.
    #[test]
    fn a_placeholder_principal_label_resolves_to_the_source_derived_name() {
        use crate::rules_core::sheet_rule::{Provenance, Subject};

        let redacted_label = crate::rules_core::codex_neutral_name::neutral_name(
            "class",
            "core_rulebook",
            "classes.lst",
            42,
        );
        let base = |target: BonusTarget, expr: Expr| SheetRule {
            id: "core_rulebook:class:order_of_the_rack".to_owned(),
            label: redacted_label.clone(),
            value: SheetValue::Number(expr),
            also: Vec::new(),
            prose: Vec::new(),
            applies: Applies::Always,
            target: Some(target),
            bonus_type: None,
            print: true,
            pool: String::new(),
            tags: Vec::new(),
            subject: Subject::Character,
            repeatable: false,
            granted_by: Vec::new(),
            offers: None,
            grants: Vec::new(),
            closure_complete: false,
            always_held: false,
            provenance: Provenance::default(),
        };
        let level = Expr::ClassLevel("order_of_the_rack".to_owned());
        let rules = vec![
            base(BonusTarget::BaseAttack, level.clone()),
            base(BonusTarget::BaseSave(Save::Fortitude), level.clone()),
            base(BonusTarget::BaseSave(Save::Reflex), level.clone()),
            base(BonusTarget::BaseSave(Save::Will), level),
        ];

        let chassis = chassis_from_rules("core_rulebook", "order_of_the_rack", &rules)
            .expect("a full BAB+saves row set builds a chassis");

        assert!(
            !chassis.display_name.contains(crate::rules_core::codex_neutral_name::NAME_PREFIX),
            "display_name must not carry the raw ingest placeholder: {}",
            chassis.display_name
        );
        assert_eq!(chassis.display_name, "Order Of The Rack");
    }
}
