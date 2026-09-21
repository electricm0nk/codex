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
    ProsePiece, Save, SheetRule, SheetValue,
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
    /// This class's hit die size (`d10` -> `10`), read off the principal
    /// rule's `StatBlock "Hit die"` prose row — `None` for the 7 records
    /// (of 185) that carry no such row, never a fabricated value (F0-check
    /// finding 5; review finding 13 / `epic-f-class-completion.md` §0.4).
    /// A hard prerequisite of F3: HP cannot be computed without it.
    pub hit_die: Option<u8>,
    /// This class's skill ranks gained per level, read off the principal
    /// rule's `StatBlock "Skill ranks per level"` prose row the same way
    /// [`Self::hit_die`] reads `"Hit die"` — `None` when the row is absent.
    /// Measured over the full corpus at authoring time (F0-check finding
    /// 5): NO class record in this corpus carries a `StatBlock` prose row
    /// under this or any other label besides `"Hit die"` (`grep -rho
    /// '"StatBlock":"[^"]*"' data/sheet_rules/*/class/*.json | sort | uniq
    /// -c` -> `178 "StatBlock":"Hit die"`, nothing else, across all 185
    /// class files) -- this reader is real and wired, but today returns
    /// `None` for every one of the 185 records, an honest absence rather
    /// than a fabricated skill-point figure (`docs/governance/
    /// no-stub-mvp-doctrine.md`). See `docs/retro/events/
    /// sub-agent-f0-check-fix.jsonl` for the correction against §2's
    /// assumption that this row already exists like `"Hit die"` does.
    pub skill_ranks_per_level: Option<u8>,
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
/// value. The corpus carries no such row today (see
/// [`ClassChassis::skill_ranks_per_level`]'s own doc comment) so this is
/// exercised by no real record yet, but is written to the same "plain
/// digits, nothing else" contract [`parse_die_size`] uses — never a
/// guessed rank count for a shape this parser does not recognize.
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
        // Evangelist used to be this test's example: before SD-36 Epic E
        // CONV-05, its converted record's own BAB/save formulas were wiped
        // to WORDS by an unrelated degrading token elsewhere on the same
        // record (`convert.rs`'s old record-wide degradation policy). CONV-05
        // fixed that -- Evangelist now correctly resolves a real chassis (¾
        // BAB, good Reflex; see `generic_class_chassis.rs`'s
        // `all_seventy_eight_conventional_classes_resolve`) -- so it no
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
    fn skill_ranks_per_level_is_an_honest_absence_over_the_whole_corpus_today() {
        // F0-check finding 5: §2's territory list assumed
        // `skill_ranks_per_level` is converted the same way `hit_die` is
        // (a `StatBlock` prose row, `None` only for the same 7 named
        // exceptions). Measured directly instead (`grep -rho
        // '"StatBlock":"[^"]*"' data/sheet_rules/*/class/*.json | sort |
        // uniq -c` -> only `178 "StatBlock":"Hit die"`, no other label at
        // all, across all 185 class files): this data does not exist in
        // the corpus yet. The reader is real and wired (same
        // `stat_block_prose_text` helper `hit_die` uses, generalized to
        // any label) -- it is simply never fed a matching row today. This
        // test pins that honest absence directly, over the full corpus,
        // rather than letting a silently-`None` field look untested.
        let books = crate::rules_core::class_census::prestige_scan_books();
        let book_refs: Vec<&str> = books.iter().map(String::as_str).collect();
        let all = records(&book_refs);
        assert_eq!(all.len(), 177, "measured chassis-bearing population moved off 177");
        assert!(
            all.values().all(|chassis| chassis.skill_ranks_per_level.is_none()),
            "a class record now carries a Skill ranks per level StatBlock row -- this test (and \
             the F0-check finding 5 correction logged at docs/retro/events/\
             sub-agent-f0-check-fix.jsonl) must be updated, not left silently green"
        );
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
