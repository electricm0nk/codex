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

use crate::rules_core::sheet_rule::{
    evaluate_expr_from_facts, Applies, BonusTarget, CharacterFacts, Cmp, Expr, Save, SheetRule,
    SheetValue,
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

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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
    for rule in rules {
        let Some(BonusTarget::BaseSave(save)) = rule.target.as_ref() else { continue };
        let index = match save {
            Save::Fortitude => 0,
            Save::Reflex => 1,
            Save::Will => 2,
        };
        if saves[index].is_none() {
            saves[index] = number_of(rule);
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
    Some(ClassChassis {
        book: book.to_string(),
        slug: slug.to_string(),
        level_var,
        display_name: principal.label.clone(),
        tags,
        max_level,
        base_attack: base_attack.clone(),
        saves: [fort.clone(), refl.clone(), will.clone()],
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
        // Pathfinder Delver's converted expressions name a codex-neutral class
        // id, not the file slug. Binding the file slug would make every level
        // evaluate to zero and still LOOK like a resolved chassis -- so the
        // binding is read off the expressions. Moderate BAB (`level*3/4`) at
        // level 10 is 7, and the three saves (`(level+1)/2` Reflex, `(level+1)/3`
        // Fortitude and Will) are 5, 3 and 3.
        let delver =
            record("adventurers_guide", "pathfinder_delver").expect("the record is present");
        assert_ne!(delver.level_var, delver.slug, "this record's ids genuinely differ");
        let row = delver.row_at(10).expect("level 10 resolves");
        assert_ne!(row.base_attack_bonus, 0, "a zero here is the wrong-binding failure");
        assert_eq!(row.base_attack_bonus, 7);
    }

    #[test]
    fn a_record_with_no_chassis_rows_is_absent_rather_than_half_built() {
        // Inner Sea Gods' Evangelist converts no BaseAttack/BaseSave row at
        // all; it must not appear as a chassis with guessed progressions.
        assert!(record("inner_sea_gods", "evangelist").is_none());
    }
}
