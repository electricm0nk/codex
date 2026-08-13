//! SD-29 Epic 7 companion catalog browser — Tauri command adapter over the
//! ingested `companion` tables (`companion_chassis::COMPANION_BOOKS`).
//!
//! # The gap this closes
//!
//! Before this lane, the whole `companion` kind reached no surface at all: all
//! 1,696 corpus units read `companion_content_has_no_engine_table` or
//! `no_compiled_rule_set_for_book`, and the engine's only companion content was
//! two hand-grounded species — `pilot_compute::ground_wolf_companion_stat_block`
//! and `ground_horse_companion_stat_block` — whose values are Rust constants
//! chosen for the pilot vertical slice, not corpus reads. The character sheet's
//! Pets tab renders those two and nothing else; it is a *computed* companion for
//! the character in front of you, not a browsable catalog of what the corpus
//! contains, and it can never show a Griffon or a Clockwork Spy.
//!
//! This module is deliberately `monster_catalog.rs`'s shape (a pure
//! `build_*_catalog()` builder plus a thin `#[tauri::command]` wrapper over it),
//! for the reason that file states about `spell_catalog.rs`: the second catalog
//! of a kind should not invent a third convention.
//!
//! # One kind, two record shapes
//!
//! `v06_work_inventory::file_kind` types both a book's `*_races_companion.lst`
//! creature rows and its `*_abilities_companion.lst` ability rows as
//! `Kind::Companion`. The wire keeps that shape: an ability is served **attached
//! to the creature that owns it**, exactly as a `monster_ability` is served
//! under its monster, and both share one `<book>:companion:<slug>` key space
//! because the corpus files them under one kind.
//!
//! # What is served, and what is deliberately absent
//!
//! Every field on `CompanionRecord` crosses: name, size, movement modes, reach,
//! creature type and subtype, the `MONSTERCLASS:` token, the `TYPE:` segments,
//! natural attacks, `BONUS:STAT` adjustments, natural armor and source page.
//!
//! **Armor class, hit points and saves are not served, because they are not
//! ingested.** PCGen computes them at runtime from the `MONSTERCLASS:` hit-dice
//! table and the companion's ability scores; they are not literal tokens on the
//! creature's row. The same corpus fact `monster_catalog` states for the same
//! token, and the columns do not exist here either.
//!
//! **`BONUS:STAT` values are labelled adjustments, never scores.** A Griffon's
//! row carries `BONUS:STAT|STR|6` and a Griffon's Strength is not 6. The wire
//! carries the ability abbreviation and the signed adjustment, and the screen
//! labels the block as adjustments; presenting them as ability scores would be
//! the quieter lie.

use serde::{Deserialize, Serialize};

use codex::rules_core::rules_tables::companion_chassis::{self, CompanionRecord};

/// Wire code for a companion book's corpus directory.
///
/// A hard panic rather than a fallback, for the reason `monster_catalog`'s twin
/// states: a book registered in `companion_chassis::COMPANION_BOOKS` with no
/// wire code here would be served to the frontend under an empty or guessed
/// label, which is exactly the silent mislabelling this program has paid for
/// before.
fn book_wire_code(corpus_book: &str) -> &'static str {
    match corpus_book {
        "inner_sea_combat" => "ISC",
        "monster_codex" => "MC",
        "inner_sea_intrigue" => "ISI",
        "horror_adventures" => "HA",
        // SD-29 Epic 7 round 2. The corpus directories are `bestiary_2` /
        // `bestiary_5` / `bestiary_6`; the wire codes are the book's own
        // shorthand, matching `SOURCESHORT:` rather than the directory.
        "bestiary_5" => "B5",
        "bestiary_6" => "B6",
        "bestiary_2" => "B2",
        // SD-29 Epic 7 round 3. The corpus directory is the misspelled
        // `beastiary`; the wire code is the book's real shorthand.
        "beastiary" => "B1",
        // SD-29 Epic 7 round 4. Bestiary 3's companions and familiars — the
        // book's second family, beside the monsters the monster lane landed in
        // `9595bd82`. Same wire code either way: it names the BOOK.
        "bestiary_3" => "B3",
        // SD-29 Epic 7 round 5. Bestiary 4's companions and familiars — the
        // book's second family, beside the monsters the monster lane landed in
        // `52da4bc3`. Same wire code either way: it names the BOOK.
        "bestiary_4" => "B4",
        // SD-29 Epic 7 round 6. Ultimate Wilderness — the book's second family,
        // beside the 136 feats SD-28 Epic 26 landed. Same wire code either way.
        "ultimate_wilderness" => "UW",
        // SD-29 Epic 7 round 7. Core Essentials — the book's second family,
        // beside the 64 heritage race traits the race-trait lane landed in
        // `SD29-E6-F2-005`. Same wire code either way: it names the BOOK, and
        // `CE` is already what `reach_gate`'s `race_traits` claim passes.
        "core_essentials" => "CE",
        other => panic!(
            "companion_catalog: no wire code for companion book {other:?}. Add one here and its \
             display label in the frontend's book map before registering the book."
        ),
    }
}

/// One movement mode from the creature's `MOVE:` token.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompanionSpeedDto {
    /// The PCGen movement mode verbatim: `"Walk"`, `"Fly"`, `"Swim"`, ...
    pub mode: String,
    pub feet: u32,
}

/// One natural attack named by the creature's row.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompanionAttackDto {
    pub name: String,
    /// The die expression only. `None` means the corpus names the attack and
    /// prices it nowhere — the screen prints the name alone, never a stand-in.
    pub damage_dice: Option<String>,
}

/// One `BONUS:STAT` token. An adjustment, never a score — see the module doc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompanionStatAdjustmentDto {
    /// `"STR"`, `"DEX"`, ... the corpus abbreviation verbatim.
    pub ability: String,
    pub amount: i16,
}

/// One conditional `DESC:` token of an ability row that states its rules text
/// more than once.
///
/// PCGen serves the token whose `PRE…` gate the character meets. This catalog
/// has no character, so it serves them ALL, each labelled with its condition —
/// the only rendering that is true for every reader. Picking one would be the
/// same defect as picking one by position, which is what the transcriber
/// refused outright until Ultimate Wilderness made the refusal load-bearing
/// (`decisions.md §61.1`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompanionDescriptionVariantDto {
    /// The variant's rules text, rendered for a player by the same renderer and
    /// under the same leak guard as `description`.
    pub text: String,
    /// The gate, in prose (`"master level 15 or higher"`), several gates joined
    /// with `" and "`. `"unconditionally"` for a row that carries SEVERAL
    /// ungated tokens — Ultimate Wilderness's two `Breath Weapon` rows each
    /// carry nine `DESC:` tokens of which two are ungated, so neither can be
    /// promoted to `description` and both are served here.
    ///
    /// Never empty. The single ungated token of a row that has exactly one is
    /// promoted to `description` and is NOT repeated here.
    pub condition: String,
}

/// One companion ability record, served attached to the creature that owns it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompanionAbilityDto {
    /// The canonical `<book>:companion:<slug>` identity. Unique across the
    /// catalog, so it is safe as a list key.
    pub key: String,
    /// The display name, which is not unique — Inner Sea Intrigue defines
    /// `Tinkering` twice — and is never an identity.
    pub name: String,
    /// `"CompanionAdvancement"` / `"SpecialQuality"` / `"SpecialAttack"`, or
    /// `None` for a row whose `TYPE:` states no facet the chassis models. Three
    /// Inner Sea Intrigue rows are in that state and the screen shows their
    /// `typeSegments` instead, rather than an invented label.
    pub facet: Option<String>,
    /// `"Supernatural"` / `"Extraordinary"` / `"SpellLike"`, or `None`.
    pub delivery: Option<String>,
    /// Every `TYPE:` segment of the row verbatim, so an unmodelled shape is
    /// visible rather than lost.
    pub type_segments: Vec<String>,
    /// The row's rules text, rendered for a player. `None` where the corpus
    /// carries none — an absence the screen states, never an empty paragraph.
    ///
    /// Also `None` where the row states its text ONLY conditionally, which is
    /// not the same absence: `descriptionVariants` is then non-empty and the
    /// screen renders those instead.
    pub description: Option<String>,
    /// The row's conditional rules texts, each with the condition that selects
    /// it rendered into prose. Empty for the ordinary row. Ultimate Wilderness
    /// is the first book to carry any (`decisions.md §61.1`).
    pub description_variants: Vec<CompanionDescriptionVariantDto>,
    /// The `BONUS:STAT` tokens this advancement package applies.
    pub stat_adjustments: Vec<CompanionStatAdjustmentDto>,
    pub source_page: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompanionCatalogEntryDto {
    /// The canonical `<book>:companion:<slug>` identity. Unique.
    pub key: String,
    /// The book's wire code — `"ISC"`, `"MC"`, `"ISI"`, `"HA"`.
    pub book: String,
    pub name: String,
    /// A single PCGen size code (`"M"`, `"L"`, `"T"`), or `None` where the row
    /// states none in either token shape.
    pub size: Option<String>,
    /// Every movement mode on the row. Empty is a real state, not a missing
    /// one; the screen says *no movement stated* rather than "0 ft".
    pub speeds: Vec<CompanionSpeedDto>,
    /// The `REACH:` token in feet. `Some(0)` is a real corpus value — Inner Sea
    /// Intrigue's two Tiny familiars both carry `REACH:0` — and is emphatically
    /// not the same as `None`.
    pub reach_feet: Option<u32>,
    pub race_type: Option<String>,
    /// The row's `RACESUBTYPE:` subtypes as readable prose, `|`-joined into a
    /// list rather than served with the separator on screen. Same treatment
    /// `monster_catalog::serve_race_subtype` gives the same token, and for the
    /// same reason: the raw separator reaching a player is internal corpus
    /// syntax on the sheet.
    pub race_subtype: Option<String>,
    /// The `MONSTERCLASS:` token verbatim (`"Companion:2"`), served in place of
    /// the hit points, AC and saves this ingest deliberately does not compute.
    pub monster_class: Option<String>,
    /// Every `TYPE:` segment verbatim. Empty for the 9 registered rows that
    /// carry no `TYPE:` token at all.
    pub type_segments: Vec<String>,
    pub natural_attacks: Vec<CompanionAttackDto>,
    /// `BONUS:STAT` adjustments from the creature's own row.
    pub stat_adjustments: Vec<CompanionStatAdjustmentDto>,
    /// `BONUS:VAR|AC_Natural_Armor|n|TYPE=Base`, when the row carries one.
    pub natural_armor: Option<i16>,
    pub source_page: Option<String>,
    /// The abilities this book defines for this creature, in creature-row order.
    pub abilities: Vec<CompanionAbilityDto>,
    /// Ability names the row cites that its own book does not define (`Scent`,
    /// `Flight Maneuverability`). Kept so the screen can say the creature has
    /// them without this catalog pretending to carry their text.
    pub external_ability_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanionCatalogResponse {
    pub entries: Vec<CompanionCatalogEntryDto>,
}

/// The canonical corpus identity of a companion record, in the same
/// `<book>:<kind>:<slug>` shape every other ingested kind uses, so
/// `reach_gate`'s corpus denominator joins the served rows without a
/// translation table.
///
/// The slug formula is `gen_book_cache::slugify`'s, reproduced here for the
/// same reason `monster_catalog::chassis_key` reproduces it: this crate does
/// not depend on that binary. `every_served_key_matches_a_corpus_record_file`
/// is the guard that keeps the two from drifting — it compares this output
/// against the real file names on disk rather than against a second copy of the
/// formula.
fn companion_key(book: &str, corpus_key: &str) -> String {
    let lowered: String = corpus_key
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    let mut collapsed = String::with_capacity(lowered.len());
    for c in lowered.chars() {
        if c == '_' && collapsed.ends_with('_') {
            continue;
        }
        collapsed.push(c);
    }
    format!("{book}:companion:{}", collapsed.trim_matches('_'))
}

/// Renders one `RACESUBTYPE:` token into the prose this catalog serves.
fn serve_race_subtype(raw: &str) -> String {
    raw.split('|')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>()
        .join(", ")
}

/// Renders one ability's `DESC:` token into text a player may read.
///
/// Same treatment and same hard panic `monster_catalog::serve_ability_description`
/// carries: `render_pcgen_desc` drops a `%N` formula placeholder rather than
/// guessing it (`decisions.md §24` — there is no formula interpreter), and a
/// token shape the renderer cannot handle stops here rather than reaching a
/// screen with PCGen syntax in it.
fn serve_ability_description(
    record: &companion_chassis::CompanionAbilityRecord,
) -> Option<String> {
    let raw = record.description?;
    let rendered = render_desc_token(record.key, raw, record.description_variables);
    Some(rendered)
}

/// Renders one `DESC:` prose + argument-list pair the way PCGen states it.
///
/// **The two halves are rejoined before rendering, and that is the fix rather
/// than an implementation detail.** `transcribe_companion_tables.parse_desc`
/// splits a `DESC:` token into its prose and its `%N` argument list and the
/// chassis stores them in two fields, so every earlier caller handed
/// `render_pcgen_desc` the prose ALONE — a token whose argument list is
/// missing. For the `%N` placeholders every registered book carries that made
/// no difference (all of their arguments are formulas this engine cannot
/// evaluate, so the placeholder is dropped either way, and the rendered text is
/// byte-identical: `grep -rho 'description_variables: &\[[^]]*\]'
/// src/rules_core/rules_tables/*/companion_data.rs` returns no integer
/// literal). For Ultimate Wilderness's `%%1` rows it made all the difference,
/// because the renderer decides whether `%%N` is an escape or an argument by
/// asking whether argument N exists — and it never did (`decisions.md §61.3`).
fn render_desc_token(key: &str, prose: &str, variables: &[&str]) -> String {
    let raw = if variables.is_empty() {
        prose.to_owned()
    } else {
        format!("{prose}|{}", variables.join("|"))
    };
    let rendered = codex::rules_core::pcgen_desc::render_pcgen_desc(&raw);
    if let Some(leak) = codex::rules_core::pcgen_desc::leaked_pcgen_syntax(&rendered.text) {
        panic!(
            "companion ability {key:?}: rendered description still carries {leak}. Raw token: {raw:?}"
        );
    }
    rendered.text
}

/// Renders one PCGen `PRE…` gate on a conditional `DESC:` token into prose.
///
/// **A closed set, deliberately.** These are the three token kinds Ultimate
/// Wilderness's 22 multi-`DESC:` rows actually carry, derived rather than
/// guessed:
///
/// ```text
/// python3 - <<'PY'   # over the book's own rows, round 6
/// Counter({'PREVARGTEQ': 36, 'PREVARLT': 12, 'PREALIGN': 5})
/// PY
/// ```
///
/// Anything else stops here rather than reaching a screen, the same discipline
/// `serve_ability_description`'s leak panic states: a gate this function cannot
/// read is a gate a player would be shown wrong, and inventing a fallback
/// ("some condition applies") would hide the next book's new shape instead of
/// surfacing it.
fn serve_desc_condition(token: &str) -> String {
    let (kind, body) = token.split_once(':').unwrap_or_else(|| {
        panic!("companion DESC condition {token:?} carries no ':' — not a PCGen PRE token")
    });
    match kind {
        "PREVARGTEQ" | "PREVARLT" => {
            let (variable, bound) = body.rsplit_once(',').unwrap_or_else(|| {
                panic!("companion DESC condition {token:?} states no comparison bound")
            });
            let comparison = if kind == "PREVARGTEQ" { "or higher" } else { "below" };
            if kind == "PREVARGTEQ" {
                format!("{} {bound} {comparison}", spell_out_variable(variable))
            } else {
                format!("{} {comparison} {bound}", spell_out_variable(variable))
            }
        }
        "PREALIGN" => format!("{} alignment", spell_out_alignment(body)),
        other => panic!(
            "companion DESC condition {token:?} uses gate kind {other:?}, which this catalog does \
             not render. Widen serve_desc_condition deliberately rather than shipping the raw token"
        ),
    }
}

/// `MasterLevel` -> `master level`. A mechanical camel-case split, so the
/// variable's own words reach the reader and nothing is invented: PCGen's
/// variable names ARE English words concatenated, and the alternative — a
/// hand-written label table — would need an entry per book and would silently
/// mislabel the one it lacked.
fn spell_out_variable(variable: &str) -> String {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    for c in variable.chars() {
        if c.is_ascii_uppercase() && !current.is_empty() {
            words.push(std::mem::take(&mut current));
        }
        current.push(c.to_ascii_lowercase());
    }
    if !current.is_empty() {
        words.push(current);
    }
    words.join(" ")
}

/// The nine PCGen alignment codes. A closed table because the set IS closed —
/// unlike the variable names above, these are codes rather than words, and
/// `TN` split mechanically reads "t n".
fn spell_out_alignment(code: &str) -> &'static str {
    match code {
        "LG" => "lawful good",
        "LN" => "lawful neutral",
        "LE" => "lawful evil",
        "NG" => "neutral good",
        "TN" => "true neutral",
        "NE" => "neutral evil",
        "CG" => "chaotic good",
        "CN" => "chaotic neutral",
        "CE" => "chaotic evil",
        other => panic!("companion DESC condition names alignment code {other:?}, which is not one of PCGen's nine"),
    }
}

/// Renders one conditional `DESC:` variant for the wire.
///
/// Same renderer and same leak panic as `serve_ability_description` — a variant
/// is rules text a player reads, so nothing about it may be laxer than the
/// unconditional case.
fn serve_desc_variant(
    record: &companion_chassis::CompanionAbilityRecord,
    variant: &companion_chassis::CompanionDescriptionVariant,
) -> CompanionDescriptionVariantDto {
    let text = render_desc_token(record.key, variant.text, variant.variables);
    let condition = if variant.conditions.is_empty() {
        "unconditionally".to_owned()
    } else {
        variant
            .conditions
            .iter()
            .map(|c| serve_desc_condition(c))
            .collect::<Vec<String>>()
            .join(" and ")
    };
    CompanionDescriptionVariantDto { text, condition }
}

/// Every conditional variant the screen should show, in row order.
///
/// The single ungated token of a row that has exactly one is already served as
/// `description`, so it is dropped here rather than shown twice. Everything
/// else is served — including the ungated tokens of a row with several, which
/// have nowhere else to go.
fn serve_desc_variants(
    record: &companion_chassis::CompanionAbilityRecord,
) -> Vec<CompanionDescriptionVariantDto> {
    let promoted = record.description.is_some();
    record
        .description_variants
        .iter()
        .filter(|v| !(promoted && v.conditions.is_empty()))
        .map(|v| serve_desc_variant(record, v))
        .collect()
}

fn map_ability(
    book: &str,
    record: &companion_chassis::CompanionAbilityRecord,
) -> CompanionAbilityDto {
    CompanionAbilityDto {
        key: companion_key(book, record.key),
        name: record.name.to_owned(),
        facet: record.facet.map(|f| f.corpus_token().to_owned()),
        delivery: record.delivery.map(|d| d.corpus_token().to_owned()),
        type_segments: record.type_segments.iter().map(|s| (*s).to_owned()).collect(),
        description: serve_ability_description(record),
        description_variants: serve_desc_variants(record),
        stat_adjustments: record
            .stat_adjustments
            .iter()
            .map(|a| CompanionStatAdjustmentDto {
                ability: a.ability.to_owned(),
                amount: a.amount,
            })
            .collect(),
        source_page: record.source_page.map(str::to_owned),
    }
}

fn map_companion(
    book: &companion_chassis::CompanionBook,
    record: &CompanionRecord,
) -> CompanionCatalogEntryDto {
    CompanionCatalogEntryDto {
        key: companion_key(book.corpus_book, record.key),
        book: book_wire_code(book.corpus_book).to_owned(),
        name: record.name.to_owned(),
        size: record.size.map(str::to_owned),
        speeds: record
            .speeds
            .iter()
            .map(|s| CompanionSpeedDto { mode: s.mode.to_owned(), feet: s.feet })
            .collect(),
        reach_feet: record.reach_feet,
        race_type: record.race_type.map(str::to_owned),
        race_subtype: record.race_subtype.map(serve_race_subtype),
        monster_class: record.monster_class.map(str::to_owned),
        type_segments: record.type_segments.iter().map(|s| (*s).to_owned()).collect(),
        natural_attacks: record
            .natural_attacks
            .iter()
            .map(|a| CompanionAttackDto {
                name: a.name.to_owned(),
                damage_dice: a.damage_dice.map(str::to_owned),
            })
            .collect(),
        stat_adjustments: record
            .stat_adjustments
            .iter()
            .map(|a| CompanionStatAdjustmentDto {
                ability: a.ability.to_owned(),
                amount: a.amount,
            })
            .collect(),
        natural_armor: record.natural_armor,
        source_page: record.source_page.map(str::to_owned),
        abilities: book
            .abilities_of(record)
            .into_iter()
            .map(|ability| map_ability(book.corpus_book, ability))
            .collect(),
        external_ability_refs: record
            .external_ability_refs
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
    }
}

/// Every ingested companion creature, in registry order then corpus row order.
///
/// Registry-driven: nothing here names a book, so registering a book in
/// `companion_chassis::COMPANION_BOOKS` is what makes it reach this catalog.
pub fn build_companion_catalog() -> CompanionCatalogResponse {
    let entries = companion_chassis::COMPANION_BOOKS
        .iter()
        .flat_map(|book| book.companions.iter().map(move |record| map_companion(book, record)))
        .collect();
    CompanionCatalogResponse { entries }
}

#[tauri::command]
pub fn list_companion_catalog() -> CompanionCatalogResponse {
    build_companion_catalog()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .expect("the repo root resolves")
    }

    /// Every registered creature reaches the wire. Derived from the registry
    /// rather than pinned to a number, so a book added to
    /// `COMPANION_BOOKS` without reaching the catalog fails here.
    #[test]
    fn the_catalog_serves_every_registered_companion_creature() {
        let response = build_companion_catalog();
        let expected: usize = companion_chassis::COMPANION_BOOKS
            .iter()
            .map(|b| b.companions.len())
            .sum();
        assert_eq!(response.entries.len(), expected);
        assert!(expected > 0, "a catalog serving zero rows asserts nothing");
    }

    /// Every registered ability reaches the wire too, and reaches it exactly
    /// once per owner. An ability with two owners is served under both, which
    /// is what the screen renders; the assertion is on the distinct key set so
    /// that is not mistaken for a duplicate.
    #[test]
    fn every_registered_ability_reaches_the_wire_under_an_owner() {
        let response = build_companion_catalog();
        let served: BTreeSet<String> = response
            .entries
            .iter()
            .flat_map(|entry| entry.abilities.iter().map(|a| a.key.clone()))
            .collect();
        let expected: BTreeSet<String> = companion_chassis::COMPANION_BOOKS
            .iter()
            .flat_map(|book| {
                book.companion_abilities
                    .iter()
                    .map(move |a| companion_key(book.corpus_book, a.key))
            })
            .collect();
        assert_eq!(served, expected, "an ability row reaches no creature on the wire");
    }

    /// The served key is the corpus record's own file name. This is the join
    /// `reach_gate` makes, and the only thing that proves the wire and the disk
    /// agree — a second copy of the slug formula would agree with itself.
    #[test]
    fn every_served_key_matches_a_corpus_record_file() {
        let root = repo_root().join("data/corpus");
        for book in companion_chassis::COMPANION_BOOKS {
            let dir = root.join(book.corpus_book).join("companion");
            let on_disk: BTreeSet<String> = std::fs::read_dir(&dir)
                .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
                .flatten()
                .filter_map(|entry| {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    name.strip_suffix(".json").map(str::to_owned)
                })
                .collect();
            let mut served: BTreeSet<String> = book
                .companions
                .iter()
                .map(|c| companion_key(book.corpus_book, c.key))
                .collect();
            served.extend(
                book.companion_abilities
                    .iter()
                    .map(|a| companion_key(book.corpus_book, a.key)),
            );
            let served_slugs: BTreeSet<String> = served
                .iter()
                .map(|k| k.rsplit(':').next().expect("the key has a slug").to_owned())
                .collect();
            assert_eq!(
                served_slugs, on_disk,
                "{}: the served keys and the corpus record files disagree",
                book.corpus_book
            );
        }
    }

    /// The pilot book's flagship row, end to end: the values the screen shows
    /// are the values the corpus row states.
    #[test]
    fn the_griffon_crosses_the_boundary_with_its_corpus_values() {
        let response = build_companion_catalog();
        let griffon = response
            .entries
            .iter()
            .find(|e| e.key == "inner_sea_combat:companion:companion_griffon")
            .expect("the Griffon reaches the catalog");
        assert_eq!(griffon.book, "ISC");
        assert_eq!(griffon.size.as_deref(), Some("L"));
        assert_eq!(griffon.race_type.as_deref(), Some("Magical Beast"));
        assert_eq!(griffon.monster_class.as_deref(), Some("Companion:2"));
        assert_eq!(griffon.natural_armor, Some(4));
        assert_eq!(
            griffon.speeds,
            vec![
                CompanionSpeedDto { mode: "Walk".to_owned(), feet: 30 },
                CompanionSpeedDto { mode: "Fly".to_owned(), feet: 40 },
            ]
        );
        assert_eq!(griffon.external_ability_refs, vec!["Scent".to_owned()]);
        let names: Vec<&str> = griffon.abilities.iter().map(|a| a.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["Unable to carry a rider while flying", "Companion Advancement (Griffon)"]
        );
        // The adjustments are labelled as adjustments on the wire: the row
        // states `BONUS:STAT|STR|6` and a Griffon's Strength is not 6.
        assert_eq!(
            griffon.stat_adjustments.first(),
            Some(&CompanionStatAdjustmentDto { ability: "STR".to_owned(), amount: 6 })
        );
    }

    /// No served description carries PCGen syntax. The renderer panics on a
    /// leak, so this test's value is that it EXERCISES every record — the panic
    /// only fires on a path something actually walks.
    #[test]
    fn no_served_description_leaks_pcgen_syntax() {
        let response = build_companion_catalog();
        let mut described = 0;
        for entry in &response.entries {
            for ability in &entry.abilities {
                let Some(text) = ability.description.as_deref() else { continue };
                described += 1;
                assert!(
                    codex::rules_core::pcgen_desc::leaked_pcgen_syntax(text).is_none(),
                    "{}: {text}",
                    ability.key
                );
            }
        }
        assert!(described > 0, "no ability carried a description; the check proved nothing");
    }

    /// `Some(0)` reach is a real corpus value on the two Tiny familiars, and it
    /// must not be flattened to `None` on the way to the wire — a screen that
    /// cannot tell "reach 0" from "no reach stated" is showing a different fact.
    #[test]
    fn a_zero_reach_survives_the_boundary_as_a_value_not_an_absence() {
        let response = build_companion_catalog();
        let spy = response
            .entries
            .iter()
            .find(|e| e.key == "inner_sea_intrigue:companion:familiar_clockwork_spy")
            .expect("the Clockwork Spy reaches the catalog");
        assert_eq!(spy.reach_feet, Some(0));
        let griffon = response
            .entries
            .iter()
            .find(|e| e.key == "inner_sea_combat:companion:companion_griffon")
            .expect("the Griffon reaches the catalog");
        assert_eq!(griffon.reach_feet, None);
    }

    /// The unmodelled-facet rows reach the player carrying their verbatim
    /// `TYPE:` segments, so the screen has something true to show where a facet
    /// label would go.
    ///
    /// **This counts WIRE ROWS, not records, and the two numbers differ.** The
    /// catalog nests abilities under each owning creature, so a record with two
    /// owners appears twice — which is what round 5 discovered here: Bestiary 4's
    /// two `TYPE:Communicate.SpellLike` rows are each owned by BOTH
    /// `Familiar (Pipefox)`/`Pipefox` and `Familiar (Ratling)`/`Ratling`, so 5
    /// records become 7 rows. `companion_chassis`'s
    /// `an_ability_with_no_modelled_facet_still_states_its_type_segments` is the
    /// per-RECORD count (5); this is the per-ROW one, and asserting they are the
    /// same number would be asserting that no record ever has two owners.
    #[test]
    fn an_unmodelled_facet_reaches_the_wire_with_its_type_segments() {
        let response = build_companion_catalog();
        let unmodelled: Vec<&CompanionAbilityDto> = response
            .entries
            .iter()
            .flat_map(|e| e.abilities.iter())
            .filter(|a| a.facet.is_none())
            .collect();
        // Round 7 (`decisions.md §62`): 121 -> 132 and 20 -> 31, both moved by
        // Core Essentials's 11 unmodelled-facet records. The two deltas are
        // EQUAL, and that is the statement worth pinning rather than the totals
        // alone: 11 new records producing 11 new rows means each is reached
        // through exactly ONE owner, unlike Bestiary 4's two rows below.
        assert_eq!(unmodelled.len(), 132);
        let mut keys: Vec<&str> = unmodelled.iter().map(|a| a.key.as_str()).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), 31, "31 distinct records behind the 132 wire rows");
        for ability in unmodelled {
            assert!(
                !ability.type_segments.is_empty(),
                "{}: an unmodelled facet with no segments shows the player nothing",
                ability.key
            );
            // Three known shapes, and the third is an UPSTREAM TYPO rather than
            // an unmodelled concept: 15 Ultimate Wilderness rows spell the
            // segment `SpecialQuaility`, one letter off the `SpecialQuality`
            // this chassis models, so `read_facet_and_delivery` correctly does
            // not recognise it. It is NOT silently corrected into the modelled
            // facet — the transcriber emits substrings of the cited row and
            // never an inferred value — and `type_segments` carries the corpus's
            // own spelling to the screen, which is what makes the typo visible
            // to a reader rather than laundered (`decisions.md §61.4`).
            // Round 7 adds the fourth and fifth shapes, both Core Essentials's
            // (`decisions.md §63`), and both genuinely unmodelled concepts
            // rather than typos:
            //
            // * `Special Ability.Extraordinary` (10 rows) — PCGen's CATEGORY
            //   name appearing in a `TYPE:` token. It is not a misspelling of
            //   `SpecialQuality` or `SpecialAttack`; it is a different axis, and
            //   collapsing it onto either would state something the row does
            //   not.
            // * `Weakness.Extraordinary` (1 row, `King Crab ~ Water
            //   Dependency`) — a concept `CompanionAbilityFacet` has no variant
            //   for at all. A companion's *weaknesses* are as real to a player
            //   as its special attacks, and the honest rendering is the corpus's
            //   own segment.
            //
            // This assertion is why they are named rather than absorbed: the
            // count pins above moved to 132/31 and would have been satisfied by
            // ANY eleven new rows. Listing the shapes is what makes the counts
            // mean something.
            let first = ability.type_segments.first().map(String::as_str);
            assert!(
                ability.type_segments == vec!["ClockworkFamiliarInstalledItem".to_owned()]
                    || ability.type_segments
                        == vec!["Communicate".to_owned(), "SpellLike".to_owned()]
                    || first == Some("SpecialQuaility")
                    || ability.type_segments
                        == vec!["Special Ability".to_owned(), "Extraordinary".to_owned()]
                    || ability.type_segments
                        == vec!["Weakness".to_owned(), "Extraordinary".to_owned()],
                "{} carries an unrecognised unmodelled shape: {:?}",
                ability.key,
                ability.type_segments
            );
        }
    }

    /// The gate renderer, over the three token kinds the corpus actually
    /// carries and one it does not.
    #[test]
    fn a_desc_gate_reaches_the_player_as_prose_and_an_unknown_one_stops_here() {
        assert_eq!(serve_desc_condition("PREVARGTEQ:MasterLevel,15"), "master level 15 or higher");
        assert_eq!(serve_desc_condition("PREVARLT:MasterLevel,9"), "master level below 9");
        assert_eq!(
            serve_desc_condition("PREVARGTEQ:CompanionAdvancement,1"),
            "companion advancement 1 or higher",
            "the variable's own words reach the reader; nothing is invented for it"
        );
        assert_eq!(
            serve_desc_condition("PREVARGTEQ:DraconicCompanionAcidAffinity,1"),
            "draconic companion acid affinity 1 or higher"
        );
        assert_eq!(serve_desc_condition("PREALIGN:TN"), "true neutral alignment");

        let unknown = std::panic::catch_unwind(|| serve_desc_condition("PRERACE:1,Elf"));
        assert!(unknown.is_err(), "an unrendered gate kind must stop rather than reach a screen");
    }

    /// A row that states its rules text once per condition reaches the wire
    /// with EVERY text and no duplication — the property that separates this
    /// from picking one by position.
    #[test]
    fn a_conditional_description_reaches_the_wire_once_per_condition() {
        let response = build_companion_catalog();
        let cobra = response
            .entries
            .iter()
            .flat_map(|e| e.abilities.iter())
            .find(|a| a.key == "ultimate_wilderness:companion:spitting_cobra_poison")
            .expect("the Spitting Cobra's poison reaches the catalog");

        assert_eq!(
            cobra.description, None,
            "every one of this row's DESC: tokens is conditional, so it has no unconditional text"
        );
        assert_eq!(cobra.description_variants.len(), 2);
        assert_eq!(cobra.description_variants[0].condition, "companion advancement below 1");
        assert_eq!(cobra.description_variants[1].condition, "companion advancement 1 or higher");
        assert!(cobra.description_variants[0].text.contains("blurred vision"));
        assert!(cobra.description_variants[1].text.contains("effect blindness"));
        for variant in &cobra.description_variants {
            assert_eq!(
                codex::rules_core::pcgen_desc::leaked_pcgen_syntax(&variant.text),
                None,
                "a variant is rules text a player reads and is held to the same guard"
            );
        }

        // The corpus row writes `Fort DC %1|10+HD/2+CON`, a formula this engine
        // does not evaluate, so the `%N` reference is DROPPED rather than
        // guessed — the same treatment the unconditional path already gives it.
        assert!(
            cobra.description_variants[0].text.ends_with("Fort DC"),
            "an unresolvable `%N` formula reference must not reach the screen: {:?}",
            cobra.description_variants[0].text
        );
        assert!(
            cobra.description_variants[0].text.contains("[20% miss chance]"),
            "and the row's LITERAL per cent sign, one clause earlier, must survive intact: {:?}",
            cobra.description_variants[0].text
        );
    }

    /// Every registered book has a wire code, and no two books share one.
    /// A duplicate would merge two books' rows under one filter on the screen.
    #[test]
    fn every_registered_book_has_a_distinct_wire_code() {
        let mut codes: Vec<&str> = companion_chassis::COMPANION_BOOKS
            .iter()
            .map(|b| book_wire_code(b.corpus_book))
            .collect();
        let before = codes.len();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), before, "two companion books share a wire code");
    }
}
