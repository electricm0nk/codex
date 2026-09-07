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

use codex::rules_core::derived_evaluator_fixture_check::{
    format_companion_save_dc_formula, format_companion_skill_ability_diff,
    format_companion_strength_damage, parse_companion_save_dc_formula,
    parse_companion_skill_ability_diff, parse_companion_strength_damage,
};
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
        // SD-29 Epic 7 round 8. Core Rulebook — the book's SIXTH family, beside
        // the classes, races, spells, equipment and race traits already landed.
        // `CRB` is not a new code invented here: `race_catalog`, `spell_catalog`
        // and `equipment_catalog` each already declare `const BOOK_CRB: &str =
        // "CRB"`, so this row makes the companion catalog agree with the three
        // catalogs a player already reads rather than adding a fourth spelling.
        "core_rulebook" => "CRB",
        // SD-29 Epic 7 round 9 — the lane's final pass. None of these four codes
        // is invented here: `equipment_catalog` already declares
        // `const BOOK_UM: &str = "UM"`, `race_catalog` and `spell_catalog`
        // already declare `BOOK_ARG`/`BOOK_APG`, and `monster_catalog` already
        // declares `const BOOK_BOTD1: &str = "BOTD1"` for this book's monsters.
        // Every row here makes the companion catalog agree with a catalog the
        // player already reads rather than adding a second spelling.
        "ultimate_magic" => "UM",
        "advanced_race_guide" => "ARG",
        "advanced_players_guide" => "APG",
        "book_of_the_damned_volume_1" => "BOTD1",
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

/// One `BONUS:WEAPONPROF=<attack>|DAMAGE|<formula>` token the creature's row
/// states — extra damage on a named attack.
///
/// # Why the screen shows a rule and not a number
///
/// The dominant corpus formula is `max(0,(STR/2))`, PCGen's encoding of PF1 CRB
/// p.182's *"if a creature has only one natural attack, it adds 1-1/2 times its
/// Strength bonus on damage rolls"* — the base attack applies the full modifier
/// and this token adds the other half, clamped at zero because the rule is
/// stated about a Strength BONUS and a penalty is never multiplied. A catalog
/// browser has no character, so it has no Strength modifier to evaluate the
/// formula at; serving a number here would be inventing one. The engine's
/// `derived_evaluator_fixture_check::format_companion_strength_damage` renders
/// the rule in words instead, and it is the SAME parse
/// (`parse_companion_strength_damage`) whose evaluated values 117 committed
/// fixtures pin — so this column and that gate can never drift apart.
///
/// # `attack` is the token's own selector, not a join
///
/// It is NOT guaranteed to name one of `naturalAttacks`
/// (`companion_chassis::NaturalAttackDamageBonus`'s Parrot finding), so this is
/// served as its own list rather than folded into the attack rows — folding it
/// would silently drop what the corpus actually says.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompanionDamageBonusDto {
    /// The `WEAPONPROF=` selector verbatim: `"Bite"`, `"Claw"`, `"Slam"`, …
    pub attack: String,
    /// The rule in words: `"+1/2 Str modifier (minimum +0)"`, `"+Str modifier"`,
    /// `"+5"`, …
    pub bonus: String,
    /// The token's formula half verbatim, for a row whose shape the engine
    /// refuses to interpret (`STR/2`, `-(STR/2)` — an unclamped halving whose
    /// negative-odd rounding PCGen does not state). `None` once `bonus` carries
    /// the rendered rule; `Some` is the honest "the corpus says this and we
    /// will not guess what it means" state, and the screen prints it as the raw
    /// token it is.
    pub unparsed_formula: Option<String>,
}

/// One `BONUS:SKILL|<skills>|<A>-<B>` token the creature's row states — a
/// skill-check bonus computed as the DIFFERENCE between two ability
/// modifiers, rather than a flat number.
///
/// **A rule, not a number.** The dominant (and, corpus-wide, only) formula is
/// `DEX-STR`: familiars and small companions whose Dexterity typically
/// exceeds their Strength get Climb and Swim checks computed from the
/// difference between the two rather than from Strength alone, which is what
/// Climb and Swim otherwise key off. A catalog browser has no character and
/// therefore no modifiers to subtract, so the engine renders the rule in
/// words rather than inventing a total — the same posture
/// [`CompanionDamageBonusDto`] takes for its own formula.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompanionSkillBonusDto {
    /// Every skill the token names, e.g. `["Climb", "Swim"]`.
    pub skills: Vec<String>,
    /// The rule in words: `"Dex modifier − Str modifier"`.
    pub bonus: String,
    /// The token's formula half verbatim, for a shape the engine refuses to
    /// interpret. `None` once `bonus` carries the rendered rule.
    pub unparsed_formula: Option<String>,
}

/// One companion ABILITY's save DC, stated entirely in a `DESC:` argument —
/// PCGen's `DESC:...%1...|<base>[+HD/2]+<ability>` encoding.
///
/// **A rule, not a number**, same posture [`CompanionSkillBonusDto`] and
/// [`CompanionDamageBonusDto`] both take: a catalog browser has no character
/// and therefore no Hit Dice or ability modifier to add, so the engine
/// renders the rule in words. This is the ONLY place the DC reaches a
/// player at all — `render_pcgen_desc` (`decisions.md §24`, no formula
/// interpreter) drops the `%1` placeholder from `description` entirely, so
/// without this field the DC number is silently missing from the ability's
/// own prose.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompanionSaveDcDto {
    /// The rule in words: `"10 + 1/2 HD + Con modifier"`.
    pub formula: String,
    /// The token's raw argument, for a shape the engine refuses to
    /// interpret. `None` once `formula` carries the rendered rule.
    pub unparsed_formula: Option<String>,
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
    /// Every DESC-embedded save-DC formula this row states (from
    /// `description`'s own argument list and every `description_variants`
    /// entry's). Empty for most rows — see [`CompanionSaveDcDto`].
    pub save_dc_formulas: Vec<CompanionSaveDcDto>,
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
    /// Every `BONUS:WEAPONPROF=<attack>|DAMAGE|` token on the creature's row.
    /// Empty for most rows, which is a real corpus state — see
    /// [`CompanionDamageBonusDto`].
    pub natural_attack_damage_bonuses: Vec<CompanionDamageBonusDto>,
    /// Every `BONUS:SKILL|<skills>|<A>-<B>` token on the creature's row.
    /// Empty for most rows — see [`CompanionSkillBonusDto`].
    pub skill_ability_diff_bonuses: Vec<CompanionSkillBonusDto>,
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
    /// The book's reference-pool ability groups (SD-32 row 19 cycle 3) --
    /// `" ~ "`-qualified companion `Ability` records that no creature row of
    /// their own book owns, so they cannot be flattened under `entries`' own
    /// `abilities` field the way an owned ability is. See
    /// `companion_pool_catalog.rs`'s module doc for the shape.
    pub pool_groups: Vec<crate::companion_pool_catalog::CompanionPoolGroupDto>,
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
/// **A closed set, deliberately.** These were the three token kinds Ultimate
/// Wilderness's 22 multi-`DESC:` rows carry, derived rather than guessed; round
/// 9 added a fourth and its negation for Ultimate Magic (see the
/// `PREABILITY` arm).
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
        // Widened deliberately, same discipline as `PREVARGTEQ`/`PREVARLT` above: the
        // one real corpus shape this catalog has ever carried is `PREHD:MIN=<n>` (the
        // Griffon's +1 HP-per-Hit-Die companion advancement,
        // `rules_tables::crb::companion_data.rs`'s own `PREHD:MIN=3`). PCGen's `PREHD`
        // also supports a `MAX=`/bare-range form this catalog has never seen -- refused
        // below rather than guessed, matching this function's own established style.
        "PREHD" => {
            let bound = body.strip_prefix("MIN=").unwrap_or_else(|| {
                panic!(
                    "companion DESC condition {token:?} uses gate kind \"PREHD\" with body \
                     {body:?}; this catalog renders only the \"MIN=<n>\" form seen in the real \
                     corpus. Widen deliberately rather than shipping the raw token"
                )
            });
            format!("{bound} Hit Dice or higher")
        }
        // SD-29 Epic 7 round 9 (`decisions.md §69.3`). Ultimate Magic is the
        // second book to carry conditional `DESC:` tokens and the first to gate
        // them on something other than a variable or an alignment: its three
        // vermin-companion rows state their poison/acid/blood-drain text once
        // for a companion that HAS taken its advancement package and once for
        // one that has not.
        //
        // ```text
        // PREABILITY:1,CATEGORY=Special Ability,Companion Advancement (Leech (Giant))
        // !PREABILITY:1,CATEGORY=Special Ability,Companion Advancement (Leech (Giant))
        // ```
        //
        // Widened DELIBERATELY, which is what the panic arm below asks for, and
        // widened to the NEGATED form too: dropping `!PREABILITY` would leave
        // the reader holding the "after" text under no condition at all, which
        // is worse than either showing both or refusing both.
        //
        // The `CATEGORY=` segment is not rendered. It names PCGen's internal
        // ability category, not anything a reader looks up; the ability's own
        // name is the identifier on the page. Everything else in the token
        // reaches the reader verbatim.
        "PREABILITY" | "!PREABILITY" => {
            let (count, rest) = body.split_once(',').unwrap_or_else(|| {
                panic!("companion DESC condition {token:?} states no ability after its count")
            });
            assert_eq!(
                count, "1",
                "companion DESC condition {token:?} requires {count} abilities; this catalog \
                 renders only the single-ability form. Widen deliberately."
            );
            let ability = rest.strip_prefix("CATEGORY=").map_or(rest, |after| {
                after.split_once(',').map_or(after, |(_category, name)| name)
            });
            assert!(
                !ability.contains(','),
                "companion DESC condition {token:?} names several abilities; this catalog \
                 renders only the single-ability form. Widen deliberately."
            );
            let preposition = if kind == "PREABILITY" { "with" } else { "without" };
            format!("{preposition} {ability}")
        }
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

/// Every save-DC formula a companion ability's `DESC:` token(s) state, in
/// words.
///
/// Unlike [`CompanionDamageBonusDto`]/[`CompanionSkillBonusDto`] (whose
/// tokens are a syntactically distinct `BONUS:` family regardless of
/// whether the specific formula parses), a `DESC:` argument carries no
/// independent marker saying "this one is a save DC" — the ONLY signal this
/// screen has that a given argument belongs to this rule family is that
/// [`parse_companion_save_dc_formula`] actually parses it as one. So, unlike
/// those two siblings, this field never serves an `unparsed_formula` row:
/// doing so for an arbitrary DESC argument this parser does not recognise
/// (a damage die reference, a duration, an unrelated named variable) would
/// mislabel it as a save DC it may not be. Deduplicated across
/// `description`'s own argument list and every `description_variants`
/// entry's, since a record commonly states the identical formula twice, once
/// per companion-advancement-tier gate (Assassin Bug (Giant) ~ Poison).
fn serve_save_dc_formulas(
    record: &companion_chassis::CompanionAbilityRecord,
) -> Vec<CompanionSaveDcDto> {
    let mut candidates: Vec<&'static str> = record.description_variables.to_vec();
    for variant in record.description_variants {
        candidates.extend(variant.variables.iter().copied());
    }
    let mut rendered: Vec<String> = candidates
        .iter()
        .filter_map(|c| parse_companion_save_dc_formula(c))
        .map(format_companion_save_dc_formula)
        .collect();
    rendered.sort();
    rendered.dedup();
    rendered
        .into_iter()
        .map(|formula| CompanionSaveDcDto { formula, unparsed_formula: None })
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
        save_dc_formulas: serve_save_dc_formulas(record),
        source_page: record.source_page.map(str::to_owned),
    }
}

/// Every ability record, from ANY registered companion book, that names this
/// creature (`book_id`, `companion_key`) in its own `cross_book_owners` list
/// (`companion_chassis::CompanionAbilityRecord::cross_book_owners`,
/// `decisions.md` Shape 8) -- paired with the ability's own book id, since a
/// cross-book ability's wire `key` is built from the book that OWNS the
/// ability record, never the creature's book (the same rule same-book
/// abilities already follow via `map_ability(book.corpus_book, ability)`).
///
/// **Closes a real gap, not a cosmetic one.** `CompanionBook::abilities_of`
/// resolves a creature's `ability_keys` only against its OWN book's
/// `companion_abilities` table (`companion_ability_resolve` is a same-book
/// lookup) -- so an ability record that declares ownership only via
/// `cross_book_owners` (e.g. Core Rulebook's 14 generic `Familiar ~ <X>`
/// rows, cross-owned by `beastiary`'s Bat/Cat/Hawk/.../Weasel familiars)
/// never appeared under ANY creature's `abilities` list before this fix,
/// even though `companion_chassis.rs`'s own
/// `the_chassis_link_resolves_in_both_directions_for_every_book` test
/// already proves every such row names a real, registered owner on the data
/// side. `cross_book_owners` was read by `v06_work_inventory.rs` (for
/// reachability accounting) and by that root-lib consistency test, but by
/// nothing on the actual wire path -- `reach_gate::every_registered_
/// ability_reaches_the_wire_under_an_owner` is what caught the gap between
/// "the data model says this is owned" and "the screen actually shows it".
fn cross_book_abilities_of(
    book_id: &str,
    companion_key: &str,
) -> Vec<(&'static str, &'static companion_chassis::CompanionAbilityRecord)> {
    companion_chassis::COMPANION_BOOKS
        .iter()
        .flat_map(|owning_book| {
            owning_book.companion_abilities.iter().filter_map(move |ability| {
                ability
                    .cross_book_owners
                    .iter()
                    .any(|(owner_book, owner_key)| *owner_book == book_id && *owner_key == companion_key)
                    .then_some((owning_book.corpus_book, ability))
            })
        })
        .collect()
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
        natural_attack_damage_bonuses: record
            .natural_attack_damage_bonuses
            .iter()
            .map(|b| match parse_companion_strength_damage(b.formula) {
                Some(parsed) => CompanionDamageBonusDto {
                    attack: b.attack.to_owned(),
                    bonus: format_companion_strength_damage(parsed),
                    unparsed_formula: None,
                },
                // The engine refuses this shape rather than guessing at it
                // (`parse_companion_strength_damage`'s own doc). Serving the
                // token verbatim, labelled as unparsed, is the honest state —
                // dropping the row would tell the player the corpus says
                // nothing, which is false.
                None => CompanionDamageBonusDto {
                    attack: b.attack.to_owned(),
                    bonus: b.formula.to_owned(),
                    unparsed_formula: Some(b.formula.to_owned()),
                },
            })
            .collect(),
        skill_ability_diff_bonuses: record
            .skill_ability_diff_bonuses
            .iter()
            .map(|b| match parse_companion_skill_ability_diff(b.formula) {
                Some(parsed) => CompanionSkillBonusDto {
                    skills: b.skills.iter().map(|s| (*s).to_owned()).collect(),
                    bonus: format_companion_skill_ability_diff(parsed),
                    unparsed_formula: None,
                },
                // The engine refuses this shape rather than guessing at it.
                // Serving the token verbatim, labelled as unparsed, is the
                // honest state — dropping the row would tell the player the
                // corpus says nothing, which is false.
                None => CompanionSkillBonusDto {
                    skills: b.skills.iter().map(|s| (*s).to_owned()).collect(),
                    bonus: b.formula.to_owned(),
                    unparsed_formula: Some(b.formula.to_owned()),
                },
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
            .chain(
                cross_book_abilities_of(book.corpus_book, record.key)
                    .into_iter()
                    .map(|(owning_book, ability)| map_ability(owning_book, ability)),
            )
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
    let pool_groups = crate::companion_pool_catalog::load_companion_pool_groups(book_wire_code);
    CompanionCatalogResponse { entries, pool_groups }
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

    /// Every `BONUS:WEAPONPROF=…|DAMAGE|` token the shipped tables carry
    /// reaches the wire — count for count. Derived from the registry rather
    /// than pinned to a number, so a regen that dropped the transcription
    /// fails here as well as in the fixture bar check.
    #[test]
    fn every_transcribed_damage_bonus_reaches_the_wire() {
        let response = build_companion_catalog();
        let served: usize =
            response.entries.iter().map(|e| e.natural_attack_damage_bonuses.len()).sum();
        let expected: usize = companion_chassis::COMPANION_BOOKS
            .iter()
            .flat_map(|b| b.companions.iter())
            .map(|c| c.natural_attack_damage_bonuses.len())
            .sum();
        assert_eq!(served, expected);
        assert!(expected > 0, "a wire carrying zero damage bonuses asserts nothing");
    }

    /// The rendered text is the RULE, not a number, and it is the shared
    /// `derived_evaluator_fixture_check` parse that produces it — the same one
    /// the committed fixtures pin. Checked on a real, resolved record whose
    /// corpus row is quoted in `companion_chassis::NaturalAttackDamageBonus`.
    #[test]
    fn a_half_strength_damage_bonus_reaches_the_screen_as_the_rule_it_states() {
        let response = build_companion_catalog();
        let fox = response
            .entries
            .iter()
            .find(|e| e.key == "ultimate_wilderness:companion:arctic_fox")
            .expect("Ultimate Wilderness ships an Arctic Fox");
        assert_eq!(fox.natural_attack_damage_bonuses.len(), 1);
        let bonus = &fox.natural_attack_damage_bonuses[0];
        assert_eq!(bonus.attack, "Bite");
        assert_eq!(bonus.bonus, "+1/2 Str modifier (minimum +0)");
        assert_eq!(bonus.unparsed_formula, None);
    }

    /// A formula the engine refuses to interpret reaches the screen VERBATIM
    /// and labelled, never dropped and never rendered as if understood.
    #[test]
    fn an_uninterpretable_formula_reaches_the_wire_verbatim_and_labelled() {
        let response = build_companion_catalog();
        let unparsed: Vec<&CompanionDamageBonusDto> = response
            .entries
            .iter()
            .flat_map(|e| e.natural_attack_damage_bonuses.iter())
            .filter(|b| b.unparsed_formula.is_some())
            .collect();
        assert!(
            !unparsed.is_empty(),
            "the corpus carries formulas this engine refuses (STR/2, -(STR/2)); if none reaches \
             the wire, either the transcription or the refusal changed"
        );
        for b in unparsed {
            assert_eq!(b.unparsed_formula.as_deref(), Some(b.bonus.as_str()));
        }
    }

    /// A save-DC formula stated ONLY in a `DESC:` argument reaches the screen
    /// as the rule in words -- the same real record
    /// `run_companion_save_dc_bar_check`'s own scratch tests use. Without
    /// this field the DC is silently missing: `render_pcgen_desc` drops the
    /// `%1` placeholder entirely (`decisions.md §24`), so `description`
    /// alone reads "...save Fort DC ; frequency 1/round for 6 rounds...".
    #[test]
    fn a_desc_embedded_save_dc_formula_reaches_the_screen_as_the_rule_it_states() {
        let response = build_companion_catalog();
        let dimorphodon = response
            .entries
            .iter()
            .find(|e| e.key == "bestiary_4:companion:companion_dinosaur_dimorphodon")
            .expect("Bestiary 4 ships a Dimorphodon companion");
        let poison = dimorphodon
            .abilities
            .iter()
            .find(|a| a.name == "Poison")
            .expect("Dimorphodon carries a Poison ability");
        assert_eq!(poison.save_dc_formulas.len(), 1);
        assert_eq!(poison.save_dc_formulas[0].formula, "10 + 1/2 HD + Con modifier");
        assert_eq!(poison.save_dc_formulas[0].unparsed_formula, None);
        // The %1 placeholder must be gone from the rendered prose (the real
        // reason this field exists) -- but the surrounding text must still
        // be there, proving this is the ordinary drop-not-corrupt renderer.
        let description = poison.description.as_deref().unwrap_or("");
        assert!(!description.contains('%'), "description leaked raw PCGen syntax: {description:?}");
        assert!(
            description.contains("save Fort DC"),
            "description lost its surrounding prose: {description:?}"
        );
    }

    /// A record whose ability states the SAME formula on two conditional
    /// `description_variants` (Assassin Bug (Giant) ~ Poison, gated on
    /// companion-advancement tier) still serves exactly ONE deduplicated
    /// entry -- two identical rule-in-words captions would look like a
    /// second, different DC on the screen.
    #[test]
    fn a_formula_repeated_across_conditional_variants_is_served_once() {
        let response = build_companion_catalog();
        let bug = response
            .entries
            .iter()
            .find(|e| e.key == "ultimate_wilderness:companion:companion_assassin_bug_giant")
            .expect("Ultimate Wilderness ships a Giant Assassin Bug companion");
        let poison = bug
            .abilities
            .iter()
            .find(|a| a.name == "Poison")
            .expect("Assassin Bug (Giant) carries a Poison ability");
        assert_eq!(poison.save_dc_formulas.len(), 1, "{:?}", poison.save_dc_formulas);
        assert_eq!(poison.save_dc_formulas[0].formula, "10 + 1/2 HD + Con modifier");
    }

    /// Corpus-wide: every save-DC formula this parser recognises reaches SOME
    /// ability on the wire, and the total is neither zero (asserting
    /// nothing) nor drifted without this test noticing. **35, not the
    /// fixture's 25**: an ability is served once PER OWNER (this module's
    /// own doc comment, `every_registered_ability_reaches_the_wire_under_an_
    /// owner`'s own distinction) while the fixture pins one row per DISTINCT
    /// ability RECORD -- a formula whose ability has several owning
    /// creatures is counted once here per owner, honestly matching what the
    /// screen actually renders per creature page.
    #[test]
    fn every_recognised_save_dc_formula_reaches_the_wire() {
        let response = build_companion_catalog();
        let served: usize =
            response.entries.iter().flat_map(|e| e.abilities.iter()).map(|a| a.save_dc_formulas.len()).sum();
        assert_eq!(served, 35, "companion save-DC formula count on the wire moved");
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

    /// Corpus records that genuinely exist on disk under one `<book>/companion/`
    /// directory but are deliberately NOT part of `companion_chassis`' transcribed
    /// table -- named row for row against `rules_tables/beastiary1/companion_data.rs`'s
    /// own header comment (SD-32 row 19 cycle 2), so this stays a NAMED, evidenced
    /// exception set rather than a loosened gate: any record not on this list still
    /// fails `every_served_key_matches_a_corpus_record_file` the moment it appears
    /// unaccounted for. Confirmed by re-running `scripts/transcribe_companion_tables.py
    /// beastiary`, which reports the identical 27 refusals (`.COPY=`/`.MOD` delta rows
    /// this chassis has no second-citation mechanism to resolve, per
    /// `decisions.md §59.2`/`§63.1`) plus one owned-but-unmodelled `ASPECT:`-only row
    /// (`§61.2`), and produces a byte-identical `companion_data.rs`. Per `decisions.md
    /// §27b`: "needs a new mechanism" is grounds for sizing, not silent exclusion --
    /// this is escalated by coordinate in the row 19 cycle 2 receipt, not invented here.
    const KNOWN_UNTRANSCRIBED_COMPANION_RECORDS: &[(&str, &str)] = &[
        // 22 `.COPY=` creature rows: each states a delta (a `TEMPLATE:` token) on a
        // base creature this chassis already carries, not a standalone chassis of its
        // own -- resolving one needs a creature-template application engine (Celestial
        // Creature / Fiendish Creature), which does not exist anywhere in this program
        // today (`companion_data.rs:24-61`).
        ("beastiary", "bat_celestial"),
        ("beastiary", "bat_fiendish"),
        ("beastiary", "cat_celestial"),
        ("beastiary", "cat_fiendish"),
        ("beastiary", "hawk_celestial"),
        ("beastiary", "hawk_fiendish"),
        ("beastiary", "lizard_celestial"),
        ("beastiary", "lizard_fiendish"),
        ("beastiary", "monkey_celestial"),
        ("beastiary", "monkey_fiendish"),
        ("beastiary", "owl_celestial"),
        ("beastiary", "owl_fiendish"),
        ("beastiary", "rat_celestial"),
        ("beastiary", "rat_fiendish"),
        ("beastiary", "raven_celestial"),
        ("beastiary", "raven_fiendish"),
        ("beastiary", "toad_celestial"),
        ("beastiary", "toad_fiendish"),
        ("beastiary", "viper_celestial"),
        ("beastiary", "viper_fiendish"),
        ("beastiary", "weasel_celestial"),
        ("beastiary", "weasel_fiendish"),
        // 4 `.MOD` ability rows: each states a delta on an existing ability record;
        // this chassis carries no second citation to resolve a `.MOD` target
        // (`companion_data.rs:29-32`).
        ("beastiary", "universal_monster_rule_change_shape"),
        ("beastiary", "universal_monster_rule_disease_extraordinary"),
        ("beastiary", "universal_monster_rule_fast_healing"),
        ("beastiary", "universal_monster_rule_poison_extraordinary"),
        // 1 orphan ability: no creature row of this book owns it, so nothing could
        // ever reach it on screen (`companion_data.rs:15-22`, `decisions.md §50`/`§56.1`).
        ("beastiary", "summon"),
        // 1 owned-but-unmodelled ability: states only `ASPECT:`, which no companion
        // chassis in this program models yet (`companion_data.rs:63-69`, `§61.2`).
        ("beastiary", "tail"),
        // bestiary_4: 2 `.COPY=` ability rows, same delta shape as `beastiary`'s
        // 4 `.MOD` rows above -- no second-citation mechanism to resolve them
        // (`rules_tables/bestiary_4/companion_data.rs:14-20`, `decisions.md §59.2`).
        ("bestiary_4", "pooka_change_shape"),
        ("bestiary_4", "psychopomp_nosoi_change_shape"),
    ];

    /// The served key is the corpus record's own file name. This is the join
    /// `reach_gate` makes, and the only thing that proves the wire and the disk
    /// agree — a second copy of the slug formula would agree with itself.
    ///
    /// SD-32 row 19 cycle 3: `served_slugs` now also includes every `owners:
    /// []`, `origin: "declared"` record `companion_pool_catalog.rs` renders
    /// and serves under `CompanionCatalogResponse::pool_groups` — the shared
    /// reference-library shape cycle 2 named and quantified (434 records
    /// across `ultimate_wilderness`/`ultimate_magic`/`advanced_race_guide`/
    /// `book_of_the_damned_volume_1`) rather than exception-listing it.
    ///
    /// A residual record the pool catalog's render-and-refuse gate still
    /// declines to serve is NOT hand-named here (330 records across 6 books
    /// is exactly the volume `decisions.md §17a` forbids fabricating
    /// per-record findings for under time pressure) -- instead
    /// [`residual_is_structurally_explained`] re-derives, GENERICALLY, per
    /// residual record, whether one of the pool catalog's own three refusal
    /// reasons applies (empty/absent description, a non-`"declared"`
    /// `origin`, or an unresolved `%N`/leaked syntax). A residual record that
    /// satisfies NONE of the three is a real, unexplained gap and fails this
    /// test by name -- the same disposition an unexplained record always
    /// had, just proven structurally instead of by a stale literal list.
    #[test]
    fn every_served_key_matches_a_corpus_record_file() {
        let root = repo_root().join("data/corpus");
        let pool_response = build_companion_catalog();
        let mut mismatches: Vec<String> = Vec::new();
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
            let known_gaps: BTreeSet<&str> = KNOWN_UNTRANSCRIBED_COMPANION_RECORDS
                .iter()
                .filter(|(b, _)| *b == book.corpus_book)
                .map(|(_, slug)| *slug)
                .collect();
            for gap in &known_gaps {
                assert!(
                    on_disk.contains(*gap),
                    "{}: named exception {gap:?} no longer exists on disk -- remove it from \
                     KNOWN_UNTRANSCRIBED_COMPANION_RECORDS, it is stale",
                    book.corpus_book
                );
            }
            let on_disk_accounted_for: BTreeSet<String> = on_disk
                .difference(&known_gaps.iter().map(|s| s.to_string()).collect())
                .cloned()
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
            let wire = book_wire_code(book.corpus_book);
            served.extend(
                pool_response
                    .pool_groups
                    .iter()
                    .filter(|g| g.book == wire)
                    .flat_map(|g| g.abilities.iter().map(|a| a.key.clone())),
            );
            let served_slugs: BTreeSet<String> = served
                .iter()
                .map(|k| k.rsplit(':').next().expect("the key has a slug").to_owned())
                .collect();
            let unexplained: Vec<String> = on_disk_accounted_for
                .difference(&served_slugs)
                .filter(|slug| !residual_is_structurally_explained(&dir, slug))
                .cloned()
                .collect();
            if !unexplained.is_empty() {
                mismatches.push(format!(
                    "{}: {} record(s) reach neither `companion_chassis` nor the pool catalog with NO \
                     structural explanation on file: {unexplained:?}",
                    book.corpus_book,
                    unexplained.len()
                ));
            }
        }
        assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
    }

    /// Re-derives, from the corpus record itself, whether `companion_pool_
    /// catalog.rs`'s render-and-refuse gate had a real structural reason to
    /// decline serving it -- the same three checks that module runs, so a
    /// residual record is either provably one of those shapes or a genuine
    /// unexplained gap this test still fails on, by name.
    fn residual_is_structurally_explained(companion_dir: &std::path::Path, slug: &str) -> bool {
        let path = companion_dir.join(format!("{slug}.json"));
        let Ok(text) = std::fs::read_to_string(&path) else { return false };
        let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { return false };
        let data = &doc["data"];
        // Reason 1: no real description to serve at all (null, empty,
        // `.CLEAR`/`.CLEARALL`, or the PI-redaction marker).
        let desc = data["description"].as_str();
        let has_real_desc = desc.is_some_and(|d| {
            let t = d.trim();
            !t.is_empty() && !matches!(t.to_ascii_lowercase().as_str(), ".clear" | ".clearall" | "[redacted pi]")
        });
        if !has_real_desc {
            return true;
        }
        // Reason 2: a delta row (`.MOD`/`.COPY=`), never a standalone record
        // this catalog has a second citation to resolve against.
        if data["origin"].as_str() != Some("declared") {
            return true;
        }
        // Reason 3: an unresolved `%N` formula or leaked PCGen syntax --
        // exactly `companion_pool_catalog.rs`'s own render-and-refuse gate.
        let raw_desc = desc.expect("has_real_desc already proved this is Some");
        let rendered = codex::rules_core::pcgen_desc::render_pcgen_desc(raw_desc);
        if !rendered.dropped_args.is_empty() {
            return true;
        }
        if codex::rules_core::pcgen_desc::leaked_pcgen_syntax(&rendered.text).is_some() {
            return true;
        }
        false
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

    /// Same certification as `no_served_description_leaks_pcgen_syntax`, for
    /// the OTHER half of the render path: a row whose every `DESC:` token is
    /// conditional has no `description` at all, so a description-only check
    /// would silently pass it while showing a player nothing. Before this
    /// test, only one record's variants (`spitting_cobra_poison`, the
    /// `a_conditional_description_reaches_the_wire_once_per_condition` test
    /// above) were pinned by name — this exercises EVERY variant this catalog
    /// serves, corpus-wide, the same blanket-coverage shape the description
    /// check already had. Written for `SD31-E6-F7-001`'s render-readiness
    /// report to Epic 6/Epic 2's `Kind::Companion` done-bar rung: the
    /// zero-magnitude `grounded` population that rung targets includes units
    /// whose ONLY player-visible text lives here (9 of the 223 re-derived
    /// this cycle carry `description: None` with real `description_variants`
    /// instead), and this is the assertion that certifies the render side of
    /// that population is sound before the rung ever reads it.
    #[test]
    fn no_served_description_variant_leaks_pcgen_syntax() {
        let response = build_companion_catalog();
        let mut variants_seen = 0;
        for entry in &response.entries {
            for ability in &entry.abilities {
                for variant in &ability.description_variants {
                    variants_seen += 1;
                    assert!(!variant.text.is_empty(), "{}: a variant with empty text shows a player nothing", ability.key);
                    assert!(
                        codex::rules_core::pcgen_desc::leaked_pcgen_syntax(&variant.text).is_none(),
                        "{}: {}",
                        ability.key,
                        variant.text
                    );
                }
            }
        }
        assert!(variants_seen > 0, "no ability carried a description variant; the check proved nothing");
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
        // Round 8 (`decisions.md §65.2`): 132 -> 133 and 31 -> 32, both moved by
        // Core Rulebook's single `TYPE:NaturalAttack.NaturalAttackSecondary.
        // Secondary` row, `Crocodile ~ Tail Slap`. Equal deltas again, so this
        // record too is reached through exactly one owner.
        // Round 9 (`decisions.md §69.2`): 133 -> 136 and 32 -> 35, moved by
        // Advanced Race Guide's two `TYPE:RaceAbility.SpecialAbility` rows and
        // the Advanced Player's Guide's one `TYPE:SkillChoice` row. Equal
        // deltas a third time, so each of these three is reached through
        // exactly one owner too.
        //
        // `SD31-CE-COMPANION-001` (2026-08-18): 136 -> 141 and 35 -> 39, and
        // **this is the first time the two deltas are NOT equal** -- which is
        // exactly the statement this pin exists to make visible. `decisions.md
        // §9` retired the `core_essentials` book id; its 11 unmodelled-facet
        // rows moved to Bestiary 1 (10) and Ultimate Magic (1) with no change
        // to either count, because a re-attribution moves a row, not its owner
        // graph. What DID move is the Advanced Player's Guide: importing
        // `ce_races_familiar_apg.lst`'s 8 familiar creature rows gave four
        // previously-orphan evolution-choice ability rows an owner for the
        // first time (`TYPE:EvolutionChoice` x1, `TYPE:TempEvolutionChoice`
        // x3), and ONE of the four -- `Temp Evolution ~ Grab` -- is named by
        // TWO of this book's creature rows, so it produces two wire rows from
        // one record. +4 records, +5 rows. Derived twice, independently: the
        // live catalog reports 141, and a standalone scan of every
        // `rules_tables/*/companion_data.rs`'s `facet: None` rows joined to
        // each book's own `ability_keys` ownership graph reports 141 rows over
        // 39 records with `Temp Evolution ~ Grab` as the single new
        // two-owner row.
        //
        // `AT-34-E3-001` (2026-08-27, `decisions.md §66`): 141 -> 2193 and
        // 39 -> 93, and this time the deltas are wildly UNEQUAL by design,
        // not a wiring gap -- 54 new Core Rulebook records (the book-wide
        // generic Animal Companion progression table: 31
        // `TYPE:AnimalCompanionFeat` feat-pool rows, 14 `TYPE:AnimalTrick`
        // trick rows, 6 `TYPE:CompStatChoice` by-level stat rows, 3
        // `TYPE:CompChoice`/`TYPE:Special` rows) each name no single
        // creature -- upstream grants the whole table to EVERY companion a
        // core_rulebook class can have, so `companion_chassis`'s ownership
        // graph attaches each of the 54 records to every one of
        // core_rulebook's own companion-eligible creature rows, producing
        // +2052 wire rows from +54 records (2052 / 54 ≈ 38 average owners
        // per record -- book-wide-granted content, not a per-creature
        // ability). The 39 pre-existing records are untouched; `93` is
        // exactly `companion_chassis.rs`'s own
        // `an_ability_with_no_modelled_facet_still_states_its_type_segments`
        // pin (39 + 54), re-derived independently here on the wire side
        // rather than copied. None of the 54 is a feat, a special quality,
        // or a special attack the way `CompanionAbilityFacet` models those
        // concepts -- they are a level-progression TABLE, a different shape
        // this classifier does not model, the same "real content, no facet"
        // reason every earlier round's records carry `facet: None`.
        assert_eq!(unmodelled.len(), 2193);
        let mut keys: Vec<&str> = unmodelled.iter().map(|a| a.key.as_str()).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), 93, "93 distinct records (39 pre-existing + AT-34-E3-001's 54) behind the 2193 wire rows");
        // Named, so neither count above can be satisfied by a different record.
        // Asserted on the WIRE rather than only on the table, because the gap
        // this catches is a row that exists in `rules_tables` and never crosses
        // the boundary — which no `companion_chassis` test can see.
        //
        // The wire `key` is the corpus ID (`<book>:companion:<slug>`), NOT the
        // corpus `KEY:` token (`Crocodile ~ Tail Slap`) the chassis-side test
        // asserts on. Written the other way first, and this assertion failed
        // while both counts above passed — which is the useful half: had it been
        // left as a count-only bump, the difference between the two identifier
        // spaces would not have been established anywhere on this side of the
        // boundary.
        assert!(
            keys.contains(&"core_rulebook:companion:crocodile_tail_slap"),
            "Core Rulebook's one unmodelled-facet record must reach the wire: {keys:?}"
        );
        // Round 9's three, named on the wire for the same reason.
        for key in [
            "advanced_race_guide:companion:puffball_poison",
            "advanced_race_guide:companion:sapling_treant_double_damage",
            "advanced_players_guide:companion:eidolon_skills",
        ] {
            assert!(
                keys.contains(&key),
                "round 9's unmodelled-facet record {key} must reach the wire: {keys:?}"
            );
        }
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
            //
            // Round 8 adds the SIXTH shape (`decisions.md §65.2`):
            //
            // * `NaturalAttack.NaturalAttackSecondary.Secondary` (1 row, Core
            //   Rulebook's `Crocodile ~ Tail Slap`) — the first unmodelled shape
            //   that is neither a CATEGORY name, an upstream typo, nor a
            //   spell-like delivery. The row is a natural ATTACK: it carries
            //   four `BONUS:WEAPONPROF=Tail Slap` tokens and a
            //   `NATURALATTACKS:` declaration. `CompanionAbilityFacet` models
            //   `CompanionAdvancement`, `SpecialQuality` and `SpecialAttack`,
            //   and a secondary natural attack is none of the three — mapping it
            //   onto `SpecialAttack` would claim the creature has a special
            //   attack it does not.
            //
            // Round 7 learned this assertion's value the expensive way: its
            // `Weakness.Extraordinary` row got through a whole gate run because
            // the count pin above it was checked first and was stale. This round
            // hit this line on the FIRST desktop run instead, because the named
            // structural assertion added beside the counts fails before the
            // loop is even reached.
            let first = ability.type_segments.first().map(String::as_str);
            assert!(
                ability.type_segments == vec!["ClockworkFamiliarInstalledItem".to_owned()]
                    || ability.type_segments
                        == vec!["Communicate".to_owned(), "SpellLike".to_owned()]
                    || first == Some("SpecialQuaility")
                    || ability.type_segments
                        == vec!["Special Ability".to_owned(), "Extraordinary".to_owned()]
                    || ability.type_segments
                        == vec!["Weakness".to_owned(), "Extraordinary".to_owned()]
                    || ability.type_segments
                        == vec![
                            "NaturalAttack".to_owned(),
                            "NaturalAttackSecondary".to_owned(),
                            "Secondary".to_owned(),
                        ]
                    // Round 9 adds the SEVENTH and EIGHTH shapes:
                    //
                    // * `RaceAbility.SpecialAbility` (2 rows, Advanced Race
                    //   Guide's `Puffball ~ Poison` and `Sapling Treant ~
                    //   Double Damage`) — the leading segment is PCGen's
                    //   RACE-side ability class. These are the plant
                    //   companions' poison and double-damage attack, defined
                    //   on the race side because the companion IS a plant
                    //   creature. Neither `SpecialQuality` nor `SpecialAttack`
                    //   states that, and both rows carry `DESC:` prose a
                    //   player reads, so they ship rather than being dropped.
                    // * `SkillChoice` (1 row, the Advanced Player's Guide's
                    //   `Eidolon ~ Skills`) — a CHOICE the player makes, not a
                    //   quality the creature has. `CompanionAbilityFacet` has
                    //   no variant for it and inventing one from
                    //   `SpecialQuality` would claim the eidolon has a special
                    //   quality it does not.
                    || ability.type_segments
                        == vec!["RaceAbility".to_owned(), "SpecialAbility".to_owned()]
                    || ability.type_segments == vec!["SkillChoice".to_owned()]
                    // `SD31-CE-COMPANION-001` (2026-08-18) adds the NINTH and
                    // TENTH shapes, and this assertion did its job again: the
                    // count pins above had already been updated to 141/39 and
                    // would have accepted any five new rows; this line failed on
                    // the first desktop run and named the row.
                    //
                    // Both arrive from the same cause. Retiring the
                    // `core_essentials` book id (`decisions.md §9`) imported
                    // `ce_races_familiar_apg.lst`'s 8 familiar creature rows
                    // into the Advanced Player's Guide, and those owners reach
                    // four summoner evolution rows that had no owner before:
                    //
                    // * `EvolutionChoice.Extraordinary` (1 row,
                    //   `Evolution ~ Scent`)
                    // * `TempEvolutionChoice.Extraordinary` (3 rows,
                    //   `Temp Evolution ~ Scent` / `~ Constrict` / `~ Grab`)
                    //
                    // These are the summoner's EVOLUTION POOL -- the record
                    // type `decisions.md §65`/`§69` named as the one finding
                    // behind this book's 208-row shortfall, seen here from the
                    // other side: four of its rows are reachable after all,
                    // because a familiar owner names them. They are a CHOICE
                    // the player spends evolution points on, exactly like
                    // `SkillChoice` above and for the same reason: mapping
                    // either onto `SpecialQuality` would claim the eidolon has
                    // a quality it does not, and `CompanionAbilityFacet` models
                    // `CompanionAdvancement`/`SpecialQuality`/`SpecialAttack`
                    // and nothing else.
                    || ability.type_segments
                        == vec!["EvolutionChoice".to_owned(), "Extraordinary".to_owned()]
                    || ability.type_segments
                        == vec!["TempEvolutionChoice".to_owned(), "Extraordinary".to_owned()]
                    // `AT-34-E3-001` (2026-08-27, `decisions.md §66`) adds the
                    // ELEVENTH through FIFTEENTH shapes -- all five from Core
                    // Rulebook's book-wide generic Animal Companion
                    // progression table (`docs/release/SD-34-book-completion/
                    // artifacts/epic-6-closure/AT-34-E6-001_gate-lane-b_
                    // cycle_receipt.md`'s companion cascade, re-derived
                    // directly against the live table rather than copied:
                    // `grep -c 'type_segments: &\["<Shape>"\]'
                    // src/rules_core/rules_tables/crb/companion_data.rs`):
                    //
                    // * `AnimalCompanionFeat` (31 rows) -- the pool of bonus
                    //   feats an advancing companion may select, e.g.
                    //   `Animal Companion Feat ~ Dodge`. A choice, not a
                    //   quality the creature already has.
                    // * `AnimalTrick` (14 rows) -- the trained-trick pool,
                    //   e.g. `Animal Trick ~ Attack`. Same choice shape.
                    // * `CompStatChoice` (6 rows, `Companion Stat ~
                    //   STR/DEX/CON/INT/WIS/CHA`) -- the by-level ability-score
                    //   bump a player assigns, not a fixed quality.
                    // * `CompChoice` (2 rows: `+2 to Dexterity and
                    //   Constitution`, `Companion Skills`) and `Special` (1
                    //   row: `Companion Advancement`) -- the remaining
                    //   miscellaneous progression-table entries, the same
                    //   "a choice, not a quality" reason as the four above.
                    //
                    // None of these five is a feat the creature already
                    // knows, a special quality, or a special attack --
                    // `CompanionAbilityFacet` models exactly those three
                    // concepts and none of them is "a pool the player picks
                    // from as the companion advances", so all five ship
                    // unmodelled rather than mapped onto a facet that would
                    // claim something false.
                    || ability.type_segments == vec!["AnimalCompanionFeat".to_owned()]
                    || ability.type_segments == vec!["AnimalTrick".to_owned()]
                    || ability.type_segments == vec!["CompStatChoice".to_owned()]
                    || ability.type_segments == vec!["CompChoice".to_owned()]
                    || ability.type_segments == vec!["Special".to_owned()],
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
        // Round 9's fourth kind and its negation, on the exact tokens Ultimate
        // Magic's three vermin-companion rows carry. Both directions are
        // asserted, because rendering only the positive one is how the "after"
        // text would reach a reader under no condition at all.
        assert_eq!(
            serve_desc_condition(
                "PREABILITY:1,CATEGORY=Special Ability,Companion Advancement (Leech (Giant))"
            ),
            "with Companion Advancement (Leech (Giant))"
        );
        assert_eq!(
            serve_desc_condition(
                "!PREABILITY:1,CATEGORY=Special Ability,Companion Advancement (Leech (Giant))"
            ),
            "without Companion Advancement (Leech (Giant))"
        );
        // The multi-ability and count>1 forms are refused rather than
        // approximated, same discipline as the unknown kind below.
        let several = std::panic::catch_unwind(|| {
            serve_desc_condition("PREABILITY:1,CATEGORY=Special Ability,Alpha,Beta")
        });
        assert!(several.is_err(), "the multi-ability form must stop rather than reach a screen");

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
