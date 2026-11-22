# Hand-written replacements for tables whose compiled form is not a plain row slice, keyed by
# (module chain..., item name). Text is Rust, emitted into the mirror module.
EXCLUDE_MODS={'monster_subset_01','monster_subset_02','monster_subset_03','monster_subset_04','monster_subset_05',
              'monster_subset_06','monster_subset_07','monster_subset_08','monster_subset_09'}

def power_points(fn, modarg):
    return f'''pub fn {fn}(level: u8, {modarg}: i16) -> Option<i16> {{
    if level < 1 {{
        return None;
    }}
    let ladder = crate::rules_core::rules_catalog::rows::<crate::rules_core::rules_data_package::PowerPointsBaseRow>("ultimate_psionics/{{MOD}}/{fn}");
    let base = ladder.iter().find(|row| row.level == level.min(20))?.base_power_points;
    Some(base + ({modarg} * i16::from(level)) / 2)
}}'''

OVERRIDES={}
for cls,arg in [('psion','int_mod'),('wilder','cha_mod'),('vitalist','wis_mod'),('tactician','int_mod'),
                ('psychic_warrior','wis_mod'),('marksman','wis_mod'),('dread','cha_mod'),('cryptic','int_mod')]:
    OVERRIDES[('ultimate_psionics',f'{cls}_features',f'{cls}_power_points_total')]=power_points(f'{cls}_power_points_total',arg).replace('{MOD}',f'{cls}_features')

OVERRIDES[('beastiary1','monster_resolve')]='''pub fn monster_resolve(monster_id: MonsterId, rule_set: RuleSetId) -> Option<MonsterStatBlock> {
    if rule_set != RuleSetId::Bestiary1 {
        return None;
    }
    crate::rules_core::rules_catalog::rows::<crate::rules_core::rules_data_package::Beastiary1MonsterRow>("beastiary1/monster_resolve")
        .iter()
        .find(|row| row.monster == monster_id)
        .map(|row| row.stat_block.clone())
}'''

OVERRIDES[('pathfinder_unchained','class_chassis','class_chassis_resolve')]='''pub fn class_chassis_resolve(class_id: PuClassId, level: u8, rule_set: RuleSetId) -> Option<ClassTableRow> {
    if rule_set != RuleSetId::Pu {
        return None;
    }
    crate::rules_core::rules_catalog::rows::<crate::rules_core::rules_data_package::UnchainedClassRow>("pathfinder_unchained/class_chassis/class_chassis_resolve")
        .iter()
        .find(|row| row.class_id == class_id && row.row.level == level)
        .map(|row| row.row)
}'''

OVERRIDES[('monster_chassis','MONSTER_BOOKS')]='''pub static MONSTER_BOOKS: crate::rules_core::rules_catalog::Derived<MonsterBook> =
    crate::rules_core::rules_catalog::Derived::new(monster_books_from_package);

fn monster_books_from_package() -> Vec<MonsterBook> {
    use crate::rules_core::rules_catalog::rows;
    use crate::rules_core::rules_data_package::MonsterBookIndexRow;
    rows::<MonsterBookIndexRow>("monster_chassis/MONSTER_BOOKS")
        .iter()
        .map(|book| MonsterBook {
            corpus_book: book.corpus_book,
            monsters: if book.monsters.is_empty() { &[] } else { rows::<MonsterStatBlock>(book.monsters) },
            monster_abilities: if book.monster_abilities.is_empty() { &[] } else { rows::<MonsterAbilityRecord>(book.monster_abilities) },
            cross_table_owner_names: book.cross_table_owner_names,
        })
        .collect()
}'''
OVERRIDES[('monster_chassis','MONSTER_BOOK_REGISTRY')]='// MONSTER_BOOK_REGISTRY: the Pathfinder book list is MONSTER_BOOKS (package-backed); no other system registers a monster book yet.'
OVERRIDES[('monster_chassis','monster_book')]='''pub fn monster_book(corpus_book: &str) -> Option<&'static MonsterBook> {
    MONSTER_BOOKS.iter().find(|book| book.corpus_book == corpus_book)
}'''

OVERRIDES[('companion_chassis','COMPANION_BOOKS')]='''pub static COMPANION_BOOKS: crate::rules_core::rules_catalog::Derived<CompanionBook> =
    crate::rules_core::rules_catalog::Derived::new(companion_books_from_package);

fn companion_books_from_package() -> Vec<CompanionBook> {
    use crate::rules_core::rules_catalog::rows;
    use crate::rules_core::rules_data_package::CompanionBookIndexRow;
    rows::<CompanionBookIndexRow>("companion_chassis/COMPANION_BOOKS")
        .iter()
        .map(|book| CompanionBook {
            corpus_book: book.corpus_book,
            companions: if book.companions.is_empty() { &[] } else { rows::<CompanionRecord>(book.companions) },
            companion_abilities: if book.companion_abilities.is_empty() { &[] } else { rows::<CompanionAbilityRecord>(book.companion_abilities) },
            companion_classes: if book.companion_classes.is_empty() { &[] } else { rows::<CompanionClassRecord>(book.companion_classes) },
        })
        .collect()
}'''
OVERRIDES[('companion_chassis','COMPANION_BOOK_REGISTRY')]='// COMPANION_BOOK_REGISTRY: the Pathfinder book list is COMPANION_BOOKS (package-backed); no other system registers a companion book yet.'
OVERRIDES[('companion_chassis','companion_book')]='''pub fn companion_book(corpus_book: &str) -> Option<&'static CompanionBook> {
    COMPANION_BOOKS.iter().find(|book| book.corpus_book == corpus_book)
}'''

OVERRIDES[('class_spell_levels','STATIC_CLASS_SPELL_LISTS')]='''pub(crate) static STATIC_CLASS_SPELL_LISTS: crate::rules_core::rules_catalog::Derived<(&'static str, &'static [(&'static str, u8)], &'static str)> =
    crate::rules_core::rules_catalog::Derived::new(static_class_spell_lists_from_package);

fn static_class_spell_lists_from_package() -> Vec<(&'static str, &'static [(&'static str, u8)], &'static str)> {
    use crate::rules_core::rules_catalog::rows;
    use crate::rules_core::rules_data_package::ClassSpellListIndexRow;
    rows::<ClassSpellListIndexRow>("class_spell_levels/STATIC_CLASS_SPELL_LISTS")
        .iter()
        .map(|row| (row.class_id, rows::<(&'static str, u8)>(row.list), row.corpus_class_id))
        .collect()
}'''

OVERRIDES[('advanced_race_guide','class_spell_levels','ARG_CLASS_SPELL_LEVELS')]='''pub static ARG_CLASS_SPELL_LEVELS: crate::rules_core::rules_catalog::Derived<(&'static str, &'static [(&'static str, u8)])> =
    crate::rules_core::rules_catalog::Derived::new(arg_class_spell_levels_from_package);

fn arg_class_spell_levels_from_package() -> Vec<(&'static str, &'static [(&'static str, u8)])> {
    crate::rules_core::rules_catalog::rows::<(&'static str, Vec<(&'static str, u8)>)>("advanced_race_guide/class_spell_levels/ARG_CLASS_SPELL_LEVELS")
        .iter()
        .map(|(class_id, rows)| (*class_id, &*Box::leak(rows.clone().into_boxed_slice())))
        .collect()
}'''

ROOT_EXTRA=''
