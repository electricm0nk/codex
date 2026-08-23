//! SD-27 monster catalog browser — Tauri command adapter over Bestiary 1's
//! ingested monster stat blocks (`beastiary1::MonsterId::ALL`, 46 records as
//! of SD28-E16 subset 09, 2026-08-07; 41 when this module was authored).
//!
//! # The gap this closes
//!
//! `reach_gate.rs`'s `OPEN_FINDINGS` recorded it verbatim: *"Bestiary 1's 41
//! ingested monster stat blocks reach no surface. The only consumers are
//! `corpus_ingest_diagnostic` (a count) and `cache_gen::beastiary1` (a
//! build-time JSON generator); the React app contains no monster reference at
//! all."* It also named the remedy — *"a monster catalog command and browser,
//! mirroring `spell_catalog.rs` + SpellCatalogScreen.tsx"* — and that is what
//! this module is, deliberately following that file's shape (a pure
//! `build_*_catalog()` builder plus a thin `#[tauri::command]` wrapper over it)
//! rather than inventing a new one.
//!
//! The Pets tab does **not** count and never did: its companion stat block is
//! computed by `pilot_compute`'s own `ground_*_companion_stat_block`, not read
//! from these tables.
//!
//! # What is served, and what is deliberately absent
//!
//! Every field on [`MonsterStatBlock`](codex::rules_core::rules_tables::beastiary1::MonsterStatBlock)
//! crosses: name, challenge rating, size, land speed, creature type and
//! subtype, source page, and the natural-attack list.
//!
//! **Armor Class, hit points and saves are not served, because they are not
//! ingested.** That is a corpus fact carried forward from the ingest, stated in
//! `MonsterStatBlock`'s own doc comment: those values are PCGen-computed at
//! runtime from the `MONSTERCLASS:` hit-dice table and ability-score modifiers,
//! not literal tokens on the monster's `b1_races.lst` row. Rendering an empty
//! "AC" column for all 46 rows would be exactly the placeholder this repo's
//! wired-integration doctrine forbids, so the columns do not exist at all.
//!
//! **Natural-attack damage dice carry their provenance.** 12 of the 46 monsters
//! name their attacks with a bare `ABILITY:Internal|AUTOMATIC|<Name>`
//! cross-reference that resolves to a row carrying no dice at any hop, so their
//! dice were grounded from the published Bestiary 1 text under
//! `SD-26 decisions.md §11.5`. `beastiary1::natural_attack_provenance` already
//! records every one of those with its sources; this adapter passes that
//! distinction through as [`NaturalAttackDto::damage_dice_source`] so the
//! screen can say which dice are transcribed corpus tokens and which are
//! grounded from published text. Presenting the two as identical would be the
//! quieter lie.

use serde::{Deserialize, Serialize};

use codex::rules_core::rules_tables::beastiary1::{
    self, natural_attack_provenance, MonsterId, MonsterStatBlock,
};
use codex::rules_core::derived_evaluator_fixture_check::{
    spell_like_ability_caster_level, spell_like_ability_save_dc,
};
use codex::rules_core::rules_tables::monster_chassis::{self, MonsterBook};
use codex::rules_core::rules_tables::RuleSetId;

/// The one book this catalog serves. A wire code rather than a display label,
/// matching `spell_catalog.rs`/`equipment_catalog.rs`'s convention; the
/// frontend maps it to "Bestiary 1".
const BOOK_B1: &str = "B1";

/// Bonus Bestiary, the second book this catalog serves (SD-29 Epic 5 pilot).
/// Its wire code is the book's own `SOURCESHORT:BB`; the frontend maps it to
/// "Bonus Bestiary".
const BOOK_BB: &str = "BB";

/// Monster Codex, the third (SD-29 Epic 5 extend, round 1). Its wire code is
/// the book's own `SOURCESHORT:MC`.
const BOOK_MC: &str = "MC";

/// Book of the Damned, Volumes 1 and 2 -- the fourth and fifth (SD-29 Epic 5
/// extend, round 2). Wire codes are the books' own `SOURCESHORT:BOTD1` and
/// `SOURCESHORT:BOTD2`, so they are the first codes here longer than two
/// characters; nothing in the frontend's map assumes a width.
const BOOK_BOTD1: &str = "BOTD1";
const BOOK_BOTD2: &str = "BOTD2";

/// Inner Sea World Guide, the sixth (SD-29 Epic 5 extend, round 3). Its wire
/// code is the book's own `SOURCESHORT:ISWG`. The first book here served with
/// only PART of its rows: 5 of its 14 monster rows carry `NAMEISPI:YES` (the
/// corpus declaring its own name Product Identity) and 13 of its 30 ability
/// rows end up owned by no shipped monster. The catalog therefore shows 9
/// monsters and 14 abilities -- every record that is both shippable and
/// reachable. See `rules_tables::inner_sea_world_guide` for the derivation.
const BOOK_ISWG: &str = "ISWG";

/// Bestiary 2, the seventh (SD-29 Epic 5 extend, round 4) and the first that
/// serves more records than every book before it combined: 314 monsters and
/// (SD31-E6-F9-005, wave 12: 401 -> 493, +92 newly-transcribed records)
/// abilities. Its wire code is the book's own `SOURCESHORT:B2`, already used
/// by the companion catalog for the same book's familiars -- one code per
/// book, both catalogs. Of its 316 monster rows, 2 are `<Base>.COPY=<Variant>`
/// deltas that state no stat block of their own. The prior "65 orphans of 466
/// raw rows" figure is NOT restated here -- it described the raw-candidate
/// population before this wave's transcription pass and was not re-derived
/// against the new total; re-run `scripts/classify_monster_ability_rows.py`
/// before quoting it again. It is also the first book here whose abilities
/// have SEVERAL owners -- 19 of them do, and each is rendered under every
/// monster that claims it. See `rules_tables::bestiary_2` for the derivation.
const BOOK_B2: &str = "B2";

/// Bestiary 3, the eighth (SD-29 Epic 5 extend, round 5). Its wire code is the
/// book's own `SOURCESHORT:B3`. Every one of its 261 corpus monster rows ships
/// -- the first book in this catalog for which that is true -- and the 13
/// ability rows that do not are owned by no monster row of this book. See
/// `rules_tables::bestiary_3` for the derivation.
const BOOK_B3: &str = "B3";

/// Bestiary 4, the ninth (SD-29 Epic 5 extend, round 6). Its wire code is the
/// book's own `SOURCESHORT:B4`. It is the first book in this catalog to lose
/// monster rows to Product Identity: 14 of its 220 corpus rows declare
/// `NAMEISPI:YES` and do not ship, and that drop is also why 73 of its 225
/// excluded ability rows are excluded — they are well-formed and owned, and
/// unreachable only because their owner is one of the 14. See
/// `rules_tables::bestiary_4` for both derivations.
const BOOK_B4: &str = "B4";

/// Inner Sea Bestiary, the tenth (SD-29 Epic 5 extend, round 7). Its wire code
/// is the book's own `SOURCESHORT:ISB`. It is the first book in this catalog to
/// lose monster rows to the Product Identity of the abilities they NAME rather
/// than of their own name — a monster's emitted `ability_keys` array carries
/// each ability's key, so a row naming a deity-namespaced ability cannot ship
/// either. See `rules_tables::inner_sea_bestiary` for the derivation.
const BOOK_ISB: &str = "ISB";

/// Inner Sea Gods, the eleventh (SD-29 Epic 5 extend, round 9). Its wire code
/// is the book's own `SOURCESHORT:ISG`. It is the first book in this catalog
/// whose corpus rows are not all at the book root -- 3 of its 39 monsters come
/// from `support/isg_races_b4.lst`, loaded under
/// `PRECAMPAIGN:1,INCLUDES=Bestiary 4`. See `rules_tables::inner_sea_gods`.
const BOOK_ISG: &str = "ISG";

/// Ultimate Psionics, the twelfth (SD-29 Epic 5 extend, round 10) and the first
/// non-Paizo book in this catalog.
///
/// **This is the one wire code here that is NOT the book's own `SOURCESHORT`,
/// and the divergence is deliberate.** `ultimate_psionics.pcc:17` declares
/// `SOURCESHORT:UP`, but this app has served the same book's equipment under
/// `equipment_resolver::EQUIPMENT_BOOK_UPSI` = `"UPSI"` since SD-28 E29, and its
/// feats under the `Upsi` source token. The convention every other code here
/// follows exists to stop a code being *invented*; serving one book under `UP`
/// on the monster screen and `UPSI` on the equipment screen would produce
/// exactly the mislabelling that convention protects against, so the code the
/// app already ships for this book wins. Recorded rather than silently chosen —
/// `decisions.md §64.2`.
const BOOK_UPSI: &str = "UPSI";

/// Horror Adventures, the thirteenth (SD-29 Epic 5 extend, FINAL round) and the
/// smallest, at 3 monsters and 6 abilities.
///
/// Its wire code is the book's own `SOURCESHORT:HA`, and — unlike `BOOK_UPSI` —
/// that is also the code this app already serves the same book under on three
/// other screens: `race_catalog::BOOK_HA`, `companion_catalog`'s
/// `"horror_adventures" => "HA"`, and `reach_gate`'s race-trait and companion
/// rows. Convention and precedent agree here, which is why this constant needs
/// no ruling of its own.
const BOOK_HA: &str = "HA";

/// Ultimate Wilderness, Ultimate Intrigue, Ultimate Magic, Bestiary 6 and
/// Bestiary 5 -- `decisions.md §20` no_record-to-zero, round 3. All five have
/// **zero** monster rows of their own (`scripts/classify_monster_ability_
/// rows.py`'s "ZERO-monster books" line), so every `monster_ability` record
/// registered for them ships owner-less (`owners: &[]`) -- none reaches this
/// catalog's per-monster rendering, only `reach_gate.rs`'s pinned non-reach.
/// Each wire code is the same one this app already serves the book's OTHER
/// families under: `companion_catalog::book_wire_code` for `UW`/`UM`/`B6`/`B5`,
/// `equipment_catalog::BOOK_UI` for `UI` -- reused rather than invented, per
/// `BOOK_UPSI`'s own precedent above of never letting one book carry two codes.
const BOOK_UW: &str = "UW";
const BOOK_UI: &str = "UI";
const BOOK_UM: &str = "UM";
const BOOK_B6: &str = "B6";
const BOOK_B5: &str = "B5";

/// `decisions.md §20` no_record-to-zero, round 4: `pathfinder_unchained` and
/// `advanced_race_guide`, the last two of the original 8 zero-monster books.
/// Both already serve OTHER families under these same wire codes --
/// `equipment_catalog::BOOK_PU` for `PU`, `race_catalog`/`companion_catalog`'s
/// own `advanced_race_guide => "ARG"` arm for `ARG` -- reused, not invented.
const BOOK_PU: &str = "PU";
const BOOK_ARG: &str = "ARG";

/// `decisions.md §20` no_record-to-zero, round 5: `mythic_adventures`, the
/// last of the original 8 zero-monster books. Already serves its `equipment`/
/// `spell` families under this same wire code (`equipment_catalog`'s
/// `"MYTHIC"` assertions, `reach_gate`'s `("mythic_adventures", "equipment")`
/// arm) — reused, not invented.
const BOOK_MYTHIC: &str = "MYTHIC";

/// `decisions.md §27b` — EVERYTHING: `occult_adventures`'s 5 `monster_ability`
/// units, overturning the repeatedly-reconfirmed "correctly out of scope"
/// disposition (a reachability finding about a negated `PRECAMPAIGN` gate,
/// not an ingest exemption). Already serves its `equipment`/`spell` families
/// under this same wire code (`spell_catalog::BOOK_OA`, `reach_gate`'s
/// `("occult_adventures", "equipment")` arm) — reused, not invented.
const BOOK_OA: &str = "OA";

/// Wire code for a chassis book's corpus directory.
///
/// A hard panic rather than a fallback: a book registered in
/// `monster_chassis::MONSTER_BOOKS` with no wire code here would be served to
/// the frontend under an empty or guessed label, which is exactly the silent
/// mislabelling this program has paid for before. Adding a book to the registry
/// and forgetting its label fails loudly on the first call rather than shipping
/// an unlabelled row.
/// The book's name as a reader sees it, for prose the catalog serves.
///
/// Separate from [`book_wire_code`] on purpose: the wire code is an identifier
/// the frontend maps, and a sentence in a grounding note is read by a player
/// who has never seen "BB". Both are exhaustive over the registry and both
/// panic on an unregistered book rather than guessing.
fn book_display_name(corpus_book: &str) -> &'static str {
    match corpus_book {
        "bonus_bestiary" => "Bonus Bestiary",
        "monster_codex" => "Monster Codex",
        "book_of_the_damned_volume_1" => "Book of the Damned, Volume 1",
        "book_of_the_damned_volume_2" => "Book of the Damned, Volume 2",
        "inner_sea_world_guide" => "Inner Sea World Guide",
        "bestiary_2" => "Bestiary 2",
        "bestiary_3" => "Bestiary 3",
        "bestiary_4" => "Bestiary 4",
        // SD-29 Epic 5 extend, round 8. The chassis half of Bestiary 1 — the
        // 284 rows `rules_tables::beastiary1` does not hold. It serves under
        // the SAME display name and the SAME wire code as that table, because
        // it is the same book: a player filtering the catalog by "Bestiary 1"
        // must see all 330 creatures, not 46 under one label and 284 under
        // another. `decisions.md §58.3`.
        "beastiary" => "Bestiary 1",
        "inner_sea_bestiary" => "Inner Sea Bestiary",
        "inner_sea_gods" => "Inner Sea Gods",
        "ultimate_psionics" => "Ultimate Psionics",
        "horror_adventures" => "Horror Adventures",
        "ultimate_wilderness" => "Ultimate Wilderness",
        "ultimate_intrigue" => "Ultimate Intrigue",
        "ultimate_magic" => "Ultimate Magic",
        "bestiary_6" => "Bestiary 6",
        "bestiary_5" => "Bestiary 5",
        "pathfinder_unchained" => "Pathfinder Unchained",
        "advanced_race_guide" => "Advanced Race Guide",
        "mythic_adventures" => "Mythic Adventures",
        "occult_adventures" => "Occult Adventures",
        other => panic!(
            "monster_catalog: no display name for chassis book {other:?}. Add one here before \
             registering the book, or a player reads a sentence naming the wrong book."
        ),
    }
}

fn book_wire_code(corpus_book: &str) -> &'static str {
    match corpus_book {
        "bonus_bestiary" => BOOK_BB,
        "monster_codex" => BOOK_MC,
        "book_of_the_damned_volume_1" => BOOK_BOTD1,
        "book_of_the_damned_volume_2" => BOOK_BOTD2,
        "inner_sea_world_guide" => BOOK_ISWG,
        "bestiary_2" => BOOK_B2,
        "bestiary_3" => BOOK_B3,
        "bestiary_4" => BOOK_B4,
        // Bestiary 1's chassis half, under Bestiary 1's own wire code — see
        // `book_display_name`. This is the only corpus book in the registry
        // whose wire code is shared with a table outside the registry.
        "beastiary" => BOOK_B1,
        "inner_sea_bestiary" => BOOK_ISB,
        "inner_sea_gods" => BOOK_ISG,
        "ultimate_psionics" => BOOK_UPSI,
        "horror_adventures" => BOOK_HA,
        "ultimate_wilderness" => BOOK_UW,
        "ultimate_intrigue" => BOOK_UI,
        "ultimate_magic" => BOOK_UM,
        "bestiary_6" => BOOK_B6,
        "bestiary_5" => BOOK_B5,
        "pathfinder_unchained" => BOOK_PU,
        "advanced_race_guide" => BOOK_ARG,
        "mythic_adventures" => BOOK_MYTHIC,
        "occult_adventures" => BOOK_OA,
        other => panic!(
            "monster_catalog: no wire code for chassis book {other:?}. Add one here and its \
             display label in the frontend's book map before registering the book."
        ),
    }
}

/// Reads a `CR:` token into the DTO's `f32`.
///
/// The corpus spells fractional challenge ratings `1/2`, `1/3`, `1/4`, and
/// Monster Codex's `Bat (Sootwing)` is the first such row this catalog has ever
/// served -- every previously ingested monster carried an integer CR, so the
/// bare `str::parse::<f32>` this function replaces was sufficient and would now
/// panic on a perfectly correct corpus value. The table keeps the token
/// verbatim; only this wire projection reads it as a number.
fn parse_challenge_rating(book: &str, key: &str, token: &str) -> f32 {
    if let Some((numerator, denominator)) = token.split_once('/') {
        let numerator: f32 = numerator.trim().parse().unwrap_or_else(|e| {
            panic!("{book}/{key}: CR: token {token:?} has an unreadable numerator: {e}")
        });
        let denominator: f32 = denominator.trim().parse().unwrap_or_else(|e| {
            panic!("{book}/{key}: CR: token {token:?} has an unreadable denominator: {e}")
        });
        assert!(denominator != 0.0, "{book}/{key}: CR: token {token:?} divides by zero");
        return numerator / denominator;
    }
    token.trim().parse().unwrap_or_else(|e| {
        panic!("{book}/{key}: CR: token {token:?} is not a number: {e}")
    })
}

/// Where one natural attack's `damage_dice` came from.
///
/// Wire form of `natural_attack_provenance::AttackSource`, flattened to the
/// distinction a reader of the screen actually needs: is this die expression a
/// corpus token, or a value grounded from the published book because the corpus
/// genuinely has none?
const DICE_FROM_MONSTER_ROW: &str = "monsterRowToken";
const DICE_FROM_CORPUS_CROSS_REFERENCE: &str = "corpusCrossReferenceToken";
const DICE_FROM_PUBLISHED_TEXT: &str = "publishedText";
/// The monster's row names the attack but the corpus carries no die
/// expression for it anywhere, and this book's ingest did not ground one from
/// published text. The row prints the attack's name alone; it never prints a
/// stand-in value. 13 of Bonus Bestiary's 14 natural attacks are in this state
/// -- see `rules_tables::bonus_bestiary`'s module doc comment.
const DICE_ABSENT_FROM_CORPUS: &str = "notInCorpus";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NaturalAttackDto {
    /// The attack's name, e.g. `"Bite"`.
    pub name: String,
    /// The die expression only, with no Strength modifier (`"1d6"`, not
    /// `"1d6+1"`). `"0"` is a real attack that deals no damage — Cave Fisher's
    /// Filament, whose own corpus token ends `,*1,0`.
    /// `None` means the corpus states no dice at all, which is emphatically
    /// not the same thing as the real `"0"` above
    /// (`damage_dice_source == "notInCorpus"`).
    pub damage_dice: Option<String>,
    /// One of [`DICE_FROM_MONSTER_ROW`], [`DICE_FROM_CORPUS_CROSS_REFERENCE`]
    /// or [`DICE_FROM_PUBLISHED_TEXT`].
    pub damage_dice_source: String,
    /// For a grounded attack, the verbatim published "Melee" line the dice were
    /// read from, kept so a reader can re-check the transcription. `None` for an
    /// attack transcribed from the monster's own row, which has nothing extra to
    /// show.
    pub grounding_note: Option<String>,
}

/// One movement mode from the monster's `MOVE:` token.
///
/// Served alongside [`MonsterCatalogEntryDto::speed_ft`] rather than replacing
/// it: Bestiary 1's ingest only ever captured the land speed, so its rows have
/// exactly one mode to offer and the existing field stays the truth for them.
/// Bonus Bestiary's ingest captured the whole token, and a row whose only
/// movement is `Fly,30` would otherwise reach the screen as "no speed".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SpeedDto {
    /// The PCGen movement mode verbatim: `"Walk"`, `"Fly"`, `"Swim"`, ...
    pub mode: String,
    pub feet: u32,
}

/// One `monster_ability` record on a monster's chassis.
///
/// This is the wire half of `corpus-work-channels.md` §9.2's ruling: monster
/// abilities are to a monster what race traits are to a race, so they are
/// served attached to the chassis they belong to rather than as a free-floating
/// second catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MonsterAbilityDto {
    /// The corpus `KEY:` token — the identity, which for 6 of Bonus Bestiary's
    /// 17 records is namespaced (`Caryatid Column ~ Immunity to Magic`) and
    /// differs from `name`.
    pub key: String,
    /// The display name, which is not unique across books and is never an
    /// identity.
    pub name: String,
    /// `"SpecialAttack"` or `"SpecialQuality"`, verbatim from the row's
    /// `TYPE:` token.
    pub facet: String,
    /// `"Supernatural"` / `"Extraordinary"` / `"SpellLike"`, or `None` where
    /// the row does not say.
    pub delivery: Option<String>,
    /// The row's `DESC:` text. `None` for the one record that carries none —
    /// an absence the screen states, never an empty paragraph.
    pub description: Option<String>,
    pub source_page: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MonsterCatalogEntryDto {
    /// The record's corpus identity — its canonical
    /// `beastiary1:monster:<slug>` key, the same shape
    /// `beastiary1::monster_key_resolve` accepts. Unique across the catalog.
    pub key: String,
    /// `"B1"` — Bestiary 1 is the only ingested book with monster records.
    pub book: String,
    /// The monster's display name, verbatim from its `b1_races.lst` row.
    pub name: String,
    /// `CR:` verbatim. Fractional for the sub-CR-1 rows the roster does not
    /// currently hold, hence `f32` rather than an integer.
    pub challenge_rating: f32,
    /// `SIZE:` verbatim — a single PCGen size code (`"M"`, `"L"`, ...).
    pub size: String,
    /// The `MOVE:Walk,N` pair verbatim, in feet.
    ///
    /// **`0` is a real value, not a missing one.** Three records — Shark,
    /// Squid and Vargouille — carry no `Walk` pair on their row at all
    /// (`MOVE:Swim,60`, `MOVE:Swim,60,Jet,240`, `MOVE:Fly,30`), which is the
    /// published "Speed 0 ft., swim 60 ft." stat line. The screen renders that
    /// as *no land speed* rather than as "0 ft".
    ///
    /// Only the land speed is ingested; the swim/climb/fly/jet pairs on these
    /// rows were explicitly out of scope for the Bestiary 1 ingest, so this
    /// catalog has none to show and says so rather than implying a monster has
    /// only the speed printed here.
    pub speed_ft: u32,
    /// `RACETYPE:` verbatim, e.g. `"Magical Beast"`.
    pub race_type: String,
    /// The row's `RACESUBTYPE:` subtypes as readable prose, rendered by
    /// [`serve_race_subtype`], or `None` where the row carries no such token.
    /// A genuine absence — never substituted with the type.
    ///
    /// **Not verbatim, and deliberately so.** `RACESUBTYPE:` is a
    /// PCGen *multi-value* token whose separator is `|`; two Bestiary 1 rows
    /// carry more than one subtype, and serving the token verbatim put the
    /// separator itself on screen — `MonsterCatalogScreen.tsx`'s
    /// `formatCreatureType` rendered Hell Hound as
    /// `Outsider (Evil|Extraplanar|Fire|Lawful)`. That is the same class of
    /// defect as an unrendered `DESC:` placeholder — internal corpus syntax
    /// reaching a player — but it is a *display join*, not a `DESC` render, so
    /// [`render_pcgen_desc`](codex::rules_core::pcgen_desc) is the wrong tool:
    /// it has no `|` treatment and would pass the string through untouched.
    pub race_subtype: Option<String>,
    /// `SOURCEPAGE:` verbatim, e.g. `"p.15"`.
    pub source_page: String,
    /// Every natural attack on the record, in the row's own token order.
    /// Empty for a monster whose row declares none — Gnoll fights with a
    /// manufactured longspear, and an empty list says so honestly.
    pub natural_attacks: Vec<NaturalAttackDto>,
    /// Every movement mode on the row. For a Bestiary 1 record this is the
    /// single land-speed pair `speed_ft` already carries (empty when that
    /// record has no `Walk` pair at all); for a Bonus Bestiary record it is the
    /// whole `MOVE:` token.
    pub speeds: Vec<SpeedDto>,
    /// The `MONSTERCLASS:` token (`"Undead:4"`) — what PCGen computes AC, hit
    /// points and saves from, served verbatim in place of totals this ingest
    /// deliberately does not compute. `None` for Bestiary 1, whose ingest did
    /// not capture it.
    pub monster_class: Option<String>,
    /// The `monster_ability` records this book defines for this monster, in row
    /// order. For a Bestiary 1 (SD-22-half) row this is the CROSS-TABLE-OWNED
    /// subset only (`SD31-W23-MONSTER-001`, `decisions.md §58.3`) -- the
    /// legacy monster's own ability rows, resolved by owner NAME out of the
    /// `bestiary` chassis table because that monster's `MonsterStatBlock`
    /// ships from a different table than its abilities do. Still empty for a
    /// legacy monster whose row names none, honestly, not as a blanket "not
    /// ingested" default.
    pub abilities: Vec<MonsterAbilityDto>,
    /// Ability names the row cites that the book does not define (universal
    /// monster rules such as `Grab` or `Scent`). Kept so the screen can say the
    /// creature has them without this catalog pretending to carry their text.
    pub external_ability_refs: Vec<String>,
    /// PF1's "Spell-Like Abilities" universal monster rule (caster level = Hit
    /// Dice), computed by
    /// `derived_evaluator_fixture_check::spell_like_ability_caster_level` —
    /// the FIRST production caller that function has ever had (SD31-E6-F1-002,
    /// `OPEN-ISSUES.md` row 44: the seam that built it had zero, which is why
    /// this field exists rather than the function being called from a test
    /// alone).
    ///
    /// `None` for a monster with no `BONUS:VAR|SLA_CL|` token on its row at
    /// all (has no spell-like abilities to attach a caster level to — never shown as a
    /// bare number with nothing behind it), and for every record served by
    /// [`map_monster`]'s Bestiary 1 half, whose ingest does not capture
    /// abilities at all and so cannot honestly answer either way.
    pub spell_like_ability_caster_level: Option<i32>,
    /// Every spell this creature's row grants as a spell-like ability, in row
    /// order, each carrying the derived spell level
    /// `derived_evaluator_fixture_check::spell_like_ability_save_dc` reads out
    /// of the row's own save-DC token (SD31-W15-MONSTER-SLA-001).
    ///
    /// Empty for every record served by [`map_monster`]'s Bestiary 1 half,
    /// whose SD-22-era ingest captures no `SPELLS:` tokens at all — an empty
    /// list there says "this half of the ingest does not carry them", the same
    /// honest-absence posture `abilities` already takes.
    pub spell_like_abilities: Vec<MonsterSpellLikeAbilityDto>,
}

/// One granted spell-like ability, as the screen renders it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MonsterSpellLikeAbilityDto {
    /// The spell's name as the corpus row states it, scope qualifiers
    /// included (`Invisibility (self only)`).
    pub spell: String,
    /// How often, verbatim from the row's `TIMES=` segment (`3`, `ATWILL`),
    /// paired with `time_unit` when the row states one.
    pub times: Option<String>,
    pub time_unit: Option<String>,
    /// The row's `CASTERLEVEL=` value verbatim — a flat literal or a PCGen
    /// formula. Never resolved here; a formula is shown as the row states it
    /// rather than as an invented number.
    pub caster_level_token: Option<String>,
    /// The row's save-DC token verbatim (`15+CHA`). `None` for a spell the
    /// row states no save for.
    pub save_dc_token: Option<String>,
    /// The spell's own level, derived from `save_dc_token` by PF1's
    /// Spell-Like Abilities universal monster rule (`DC = 10 + spell level +
    /// ability modifier`). `None` when the row states no DC, or states one
    /// this repo refuses to read rather than guess at.
    pub derived_spell_level: Option<i32>,
    /// The ability whose modifier the DC scales with (`CHA`, or `INT` for the
    /// monsters whose rows exercise the rule's "unless otherwise noted"
    /// clause). Deliberately NOT resolved to a number: a monster's ability
    /// scores are not a corpus-stated fact in this repo (`SD31-E6-F1-002`),
    /// so the screen shows the formula, never a fabricated DC.
    pub save_dc_ability: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonsterCatalogResponse {
    pub entries: Vec<MonsterCatalogEntryDto>,
}

/// The canonical `beastiary1:monster:<slug>` key for a stat block.
///
/// Delegates to `beastiary1::monster_key` (SD28-E16, `decisions.md` §36
/// instance 9) rather than re-implementing the same lowercase/underscore
/// formula in this crate -- a second copy of this exact derivation used to
/// live here, silently able to drift from the engine's own, and the drift
/// would have surfaced only as a served key nothing could resolve. Now
/// there is one formula. Still not blindly trusted:
/// `every_served_key_resolves_back_to_its_record` feeds every key produced
/// here back through `beastiary1::monster_key_resolve` and requires the
/// same record to come back.
fn monster_key(block: &MonsterStatBlock) -> String {
    beastiary1::monster_key(&block.name)
}

/// Renders one `RACESUBTYPE:` token into the prose this catalog is allowed to
/// serve: its `|`-separated subtypes joined as a readable list.
///
/// Two of Bestiary 1's 46 rows are multi-valued — Vargouille
/// (`Evil|Extraplanar`) and Hell Hound (`Evil|Extraplanar|Fire|Lawful`) — and
/// both were reaching the screen with the raw separator in them. The other 11
/// subtype-bearing rows are single-valued and pass through unchanged, which is
/// why the defect survived: 39 of 41 rows looked perfectly fine.
///
/// Empty segments are dropped rather than rendered as a stray `, `, so a
/// malformed `A||B` degrades to `A, B` instead of putting the corpus's own
/// defect on screen in a second form.
fn serve_race_subtype(raw: &str) -> String {
    raw.split('|')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>()
        .join(", ")
}

/// Projects one grounded-attack provenance row onto the wire.
fn map_natural_attack(
    monster_key: &str,
    attack: &beastiary1::NaturalAttack,
) -> NaturalAttackDto {
    let grounded = natural_attack_provenance::provenance_for(monster_key)
        .into_iter()
        .find(|row| row.attack_name == attack.name);

    let (damage_dice_source, grounding_note) = match grounded {
        None => (DICE_FROM_MONSTER_ROW, None),
        Some(row) => match row.source {
            natural_attack_provenance::AttackSource::LstToken { path, line, .. } => (
                DICE_FROM_CORPUS_CROSS_REFERENCE,
                Some(format!(
                    "Damage dice read from a real corpus token in {path}:{line}, reached through \
                     this monster's own `{}` cross-reference rather than a `NATURALATTACKS:` \
                     token on its row. Published text: {}",
                    row.corpus_name_token, row.published_melee_text
                )),
            ),
            natural_attack_provenance::AttackSource::WebSecondSource { urls, fetched_at, .. } => (
                DICE_FROM_PUBLISHED_TEXT,
                Some(format!(
                    "This monster's row names the attack with `{}` and supplies no dice at any \
                     hop, so the dice are grounded from the published Bestiary 1 text \
                     (\"{}\"), corroborated against {} on {fetched_at}.",
                    row.corpus_name_token,
                    row.published_melee_text,
                    urls.join(" and "),
                )),
            ),
        },
    };

    NaturalAttackDto {
        name: attack.name.clone(),
        damage_dice: Some(attack.damage_dice.clone()),
        damage_dice_source: damage_dice_source.to_owned(),
        grounding_note,
    }
}

fn map_monster(monster_id: MonsterId) -> MonsterCatalogEntryDto {
    // `monster_resolve` is scoped to `RuleSetId::Bestiary1` and returns `None`
    // for any other rule set. `MonsterId::ALL` is Bestiary 1's own roster, so
    // this is total by construction, and the expect names the invariant rather
    // than hiding a possible `None`.
    let block = beastiary1::monster_resolve(monster_id, RuleSetId::Bestiary1)
        .expect("every MonsterId::ALL variant resolves under RuleSetId::Bestiary1");
    let key = monster_key(&block);
    let natural_attacks = block
        .natural_attacks
        .iter()
        .map(|attack| map_natural_attack(&key, attack))
        .collect();

    // `SD31-W23-MONSTER-001`: the cross-table-owner remedy `decisions.md
    // §58.3` named and left unbuilt. This half of the ingest still captures
    // no `monster_ability` records of its OWN, but 55 real, owned ability
    // rows across this book's 46 legacy monsters ship from the `bestiary`
    // chassis table (`scripts/transcribe_monster_tables.py bestiary`'s own
    // cross-table-owner screen), keyed to their real owner's NAME rather than
    // to any `MonsterStatBlock` in that table -- `abilities_owned_by_name`
    // reads exactly that, so this monster's OWN ability rows (when it has
    // any) now reach the screen through the SAME render path
    // (`map_chassis_ability` / `serve_ability_description`) every other
    // book's abilities already use. A monster with none of its 55 stays
    // correctly empty -- this is a real lookup, not a blanket fill.
    let bestiary_table = monster_chassis::monster_book("beastiary")
        .expect("the `bestiary` chassis (this book's OTHER table) is registered in MONSTER_BOOKS");
    let abilities: Vec<MonsterAbilityDto> = bestiary_table
        .abilities_owned_by_name(&block.name)
        .into_iter()
        .map(|ability| map_chassis_ability(bestiary_table.corpus_book, ability))
        .collect();

    MonsterCatalogEntryDto {
        key,
        book: BOOK_B1.to_owned(),
        name: block.name,
        challenge_rating: block.challenge_rating,
        size: block.size,
        speed_ft: block.speed_ft,
        race_type: block.race_type,
        race_subtype: block.race_subtype.as_deref().map(serve_race_subtype),
        source_page: block.source_page,
        natural_attacks,
        speeds: if block.speed_ft > 0 {
            vec![SpeedDto { mode: "Walk".to_owned(), feet: block.speed_ft }]
        } else {
            Vec::new()
        },
        monster_class: None,
        abilities,
        external_ability_refs: Vec::new(),
        // Bestiary 1's SD-22 half still captures no `SPELLS:`/spell-like-
        // ability tokens of its own (only the CROSS-TABLE `abilities` above,
        // resolved from the OTHER table, are populated), so this catalog
        // still cannot tell a monster with no spell-like abilities from one
        // whose abilities were simply never captured. `None` is the honest
        // answer to a question this half of the ingest cannot answer, not a
        // claim that none exist.
        spell_like_ability_caster_level: None,
        // Same reason: Bestiary 1's SD-22 half captures no `SPELLS:` tokens,
        // so an empty list is the honest answer rather than a claim the
        // creature grants none.
        spell_like_abilities: Vec::new(),
    }
}

/// The canonical corpus identity of a Bonus Bestiary record, in the same
/// `<book>:<kind>:<slug>` shape Bestiary 1's monsters already use, so both
/// books' keys read the same way and `reach_gate`'s corpus denominator joins
/// them without a translation table.
fn chassis_key(book: &str, kind: &str, corpus_key: &str) -> String {
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
    format!("{book}:{kind}:{}", collapsed.trim_matches('_'))
}

/// Renders one ability's `DESC:` token into text a player may read.
///
/// **Caught on screen, not by a test** (SD-29 Epic 5, DoD item 8): the first
/// version served `record.description` verbatim and the catalog printed
/// *"must succeed on a DC %1 Will save"* — a raw PCGen substitution
/// placeholder, the same class of defect as the `RACESUBTYPE:` `|` separator
/// this file already documents. `render_pcgen_desc` owns the treatment
/// (`decisions.md §24`: a formula `%N` is DROPPED, never guessed, because
/// there is no formula interpreter and `Babble`'s DC is genuinely
/// `10+(HD/2)+CHA` — a number this ingest does not compute).
///
/// The leak check is the same guard `gen_book_cache.rs`'s own
/// `render_player_facing_description` carries, kept as a hard panic: a token
/// shape this renderer cannot handle must stop here rather than reach a
/// screen.
fn serve_ability_description(record: &monster_chassis::MonsterAbilityRecord) -> Option<String> {
    let raw = record.description?;
    let rendered = codex::rules_core::pcgen_desc::render_pcgen_desc(raw);
    if let Some(leak) = codex::rules_core::pcgen_desc::leaked_pcgen_syntax(&rendered.text) {
        panic!(
            "monster ability {:?}: rendered description still carries {leak}. Raw token: {raw:?}",
            record.key
        );
    }
    Some(rendered.text)
}

fn map_chassis_ability(
    book: &str,
    record: &monster_chassis::MonsterAbilityRecord,
) -> MonsterAbilityDto {
    MonsterAbilityDto {
        key: chassis_key(book, "monster_ability", record.key),
        name: record.name.to_owned(),
        facet: record.facet.corpus_token().to_owned(),
        delivery: record.delivery.map(|d| d.corpus_token().to_owned()),
        description: serve_ability_description(record),
        source_page: record.source_page.map(str::to_owned),
    }
}

/// Projects one Bonus Bestiary stat block onto the same wire shape Bestiary 1's
/// rows use.
///
/// The `challenge_rating` parse is the one place this book's verbatim-token
/// discipline meets the DTO's `f32`. It is a hard panic rather than a silent
/// `0.0` default: a CR this cannot read is a transcription defect, and serving
/// "CR 0" for it would put a wrong number on screen instead of failing.
/// `every_bonus_bestiary_row_states_a_readable_challenge_rating` exercises all
/// 14 rows.
fn map_chassis_monster(
    table: &MonsterBook,
    block: &monster_chassis::MonsterStatBlock,
) -> MonsterCatalogEntryDto {
    let book = table.corpus_book;
    let challenge_rating = parse_challenge_rating(
        book,
        block.key,
        block
            .challenge_rating
            .unwrap_or_else(|| panic!("{book}/{} carries no CR: token", block.key)),
    );

    MonsterCatalogEntryDto {
        key: chassis_key(book, "monster", block.key),
        book: book_wire_code(book).to_owned(),
        name: block.name.to_owned(),
        challenge_rating,
        size: block.size.unwrap_or_default().to_owned(),
        speed_ft: block
            .speeds
            .iter()
            .find(|s| s.mode == "Walk")
            .map(|s| s.feet)
            .unwrap_or(0),
        race_type: block.race_type.unwrap_or_default().to_owned(),
        race_subtype: block.race_subtype.map(serve_race_subtype),
        source_page: block.source_page.unwrap_or_default().to_owned(),
        natural_attacks: block
            .natural_attacks
            .iter()
            .map(|attack| NaturalAttackDto {
                name: attack.name.to_owned(),
                damage_dice: attack.damage_dice.map(str::to_owned),
                damage_dice_source: if attack.damage_dice.is_some() {
                    DICE_FROM_MONSTER_ROW.to_owned()
                } else {
                    DICE_ABSENT_FROM_CORPUS.to_owned()
                },
                // Caught ON SCREEN, not by a test (SD-29 Epic 5 extend,
                // round 1, DoD item 8): this sentence hard-coded both the
                // book name and the token shape, so Monster Codex's Seru
                // rendered "the Bonus Bestiary corpus carries no die
                // expression" for a Monster Codex row whose attack is named
                // by `NATURALATTACKS:Venom,...,Poison`, not by
                // `ABILITY:Internal|AUTOMATIC|Venom`. Two false statements in
                // one sentence, both player-visible, neither reachable by any
                // test that did not read the words.
                //
                // It now names the real book and stops asserting a token
                // shape the table does not record. The shape is recoverable
                // from the record's own `source` citation in
                // `data/corpus/<book>/monster/`; asserting it here was a
                // detail this projection never had.
                grounding_note: if attack.damage_dice.is_some() {
                    None
                } else {
                    Some(format!(
                        "This monster's row names the attack, and the {} corpus \
                         carries no die expression for it at any hop. No value is shown \
                         because none was ingested.",
                        book_display_name(book)
                    ))
                },
            })
            .collect(),
        speeds: block
            .speeds
            .iter()
            .map(|s| SpeedDto { mode: s.mode.to_owned(), feet: s.feet })
            .collect(),
        monster_class: block.monster_class.map(str::to_owned),
        abilities: table
            .abilities_of(block)
            .into_iter()
            .map(|ability| map_chassis_ability(book, ability))
            .collect(),
        external_ability_refs: block
            .external_ability_refs
            .iter()
            .map(|r| (*r).to_owned())
            .collect(),
        spell_like_ability_caster_level: spell_like_ability_caster_level(block),
        spell_like_abilities: block
            .spell_like_abilities
            .iter()
            .map(|sla| {
                let derived = spell_like_ability_save_dc(sla);
                MonsterSpellLikeAbilityDto {
                    spell: sla.spell.to_owned(),
                    times: sla.times.map(str::to_owned),
                    time_unit: sla.time_unit.map(str::to_owned),
                    caster_level_token: sla.caster_level_token.map(str::to_owned),
                    save_dc_token: sla.save_dc_token.map(str::to_owned),
                    derived_spell_level: derived.as_ref().map(|d| d.spell_level),
                    save_dc_ability: derived.map(|d| d.ability),
                }
            })
            .collect(),
    }
}

/// Build the full monster catalog response, in `MonsterId::ALL`'s own
/// declaration order (subsets 01-08, i.e. ascending challenge rating then
/// alphabetical within a band). A thin, testable wrapper behind the Tauri
/// command below, mirroring `build_spell_catalog`'s command/pure-fn split.
pub fn build_monster_catalog() -> MonsterCatalogResponse {
    let mut entries: Vec<MonsterCatalogEntryDto> =
        MonsterId::ALL.iter().copied().map(map_monster).collect();
    for table in monster_chassis::MONSTER_BOOKS {
        entries.extend(table.monsters.iter().map(|block| map_chassis_monster(table, block)));
    }
    MonsterCatalogResponse { entries }
}

#[tauri::command]
pub fn list_monster_catalog() -> MonsterCatalogResponse {
    build_monster_catalog()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The SD-22 half of Bestiary 1's rows on the wire.
    ///
    /// `book == BOOK_B1` stopped identifying that table at SD-29 Epic 5 round
    /// 8, when the chassis half of the same book joined the same response under
    /// the same wire code (`decisions.md §58.3`). Every assertion below that was
    /// written about the hand-modelled 46 — its `speed_ft` scalar, its
    /// `monster_key_resolve` round trip, its `RACESUBTYPE` population — is a
    /// claim about that table and not about the book, so the filter is the key
    /// namespace. This is the same fix the Epic 5 pilot applied when Bonus
    /// Bestiary widened these denominators, one level finer: there a book code
    /// separated the two tables, here only the key does.
    fn hand_modelled_rows(entries: &[MonsterCatalogEntryDto]) -> Vec<&MonsterCatalogEntryDto> {
        entries
            .iter()
            .filter(|e| e.book == BOOK_B1 && e.key.starts_with("beastiary1:monster:"))
            .collect()
    }

    /// Bestiary 1 reaches the wire from TWO tables under one wire code since
    /// SD-29 Epic 5 round 8 (`decisions.md §58.3`), so `book == BOOK_B1` is no
    /// longer the SD-22 roster on its own. The two are told apart by their key
    /// namespaces — `beastiary1:monster:` for the hand-modelled 46,
    /// `beastiary:monster:` for the chassis's 284 — which is the same
    /// distinction `reach_gate::monsters_reach` joins on.
    ///
    /// Both halves are pinned, and the sum is pinned against the corpus, so a
    /// table that silently stopped reaching the wire fails here rather than
    /// being absorbed by the other's count.
    #[test]
    fn the_catalog_serves_every_ingested_bestiary_1_monster() {
        let response = build_monster_catalog();
        let b1: Vec<_> = response.entries.iter().filter(|e| e.book == BOOK_B1).collect();
        let hand_modelled: Vec<_> =
            b1.iter().filter(|e| e.key.starts_with("beastiary1:monster:")).collect();
        let chassis: Vec<_> =
            b1.iter().filter(|e| e.key.starts_with("beastiary:monster:")).collect();
        assert_eq!(hand_modelled.len(), MonsterId::ALL.len());
        assert_eq!(
            hand_modelled.len(),
            46,
            "Bestiary 1's SD-22 roster is 46 stat blocks (subsets 01-09, SD28-E16); if the \
             roster grew, re-derive this from the corpus rather than relaxing it"
        );
        assert_eq!(
            chassis.len(),
            280,
            "the chassis holds the book's complement less its 4 `.MOD` overlay rows -- see \
             `rules_tables::bestiary`"
        );
        assert_eq!(
            b1.len(),
            326,
            "326 of the book's 330 monster units reach the wire; the 4 that do not are \
             `.MOD` overlays, which state a delta rather than a stat block"
        );
    }

    /// One creature, one row. The two tables serving Bestiary 1 are disjoint by
    /// `rules_tables::bestiary`'s own test; this is the same claim made where a
    /// player would see it break, on the served response rather than on the
    /// tables.
    #[test]
    fn no_bestiary_1_creature_reaches_the_wire_twice() {
        let response = build_monster_catalog();
        let mut names: Vec<String> = response
            .entries
            .iter()
            .filter(|e| e.book == BOOK_B1)
            .map(|e| e.name.clone())
            .collect();
        names.sort_unstable();
        let before = names.len();
        names.dedup();
        assert_eq!(names.len(), before, "a Bestiary 1 creature is served twice");
    }

    /// Bonus Bestiary joins the same command rather than getting a second one:
    /// the screen already renders every row this response carries, so the
    /// book's records reach the player the moment they are served here.
    #[test]
    fn the_catalog_serves_every_ingested_bonus_bestiary_monster_and_its_abilities() {
        let response = build_monster_catalog();
        let bb: Vec<_> = response.entries.iter().filter(|e| e.book == BOOK_BB).collect();
        assert_eq!(
            bb.len(),
            14,
            "re-derived from the corpus this cycle: \
             `awk -F'\t' '!/^#/ && !/^SOURCELONG/ && NF>0' bb_races.lst | wc -l` -> 14"
        );
        let abilities: usize = bb.iter().map(|e| e.abilities.len()).sum();
        assert_eq!(
            abilities,
            17,
            "all 17 `bb_abilities_race.lst` rows reach the wire, each on its own monster"
        );
        for entry in &bb {
            assert!(!entry.race_type.is_empty(), "{} serves no creature type", entry.name);
            assert!(!entry.source_page.is_empty(), "{} serves no source page", entry.name);
            assert!(!entry.speeds.is_empty(), "{} serves no movement mode", entry.name);
            assert!(entry.monster_class.is_some(), "{} serves no MONSTERCLASS token", entry.name);
        }
    }

    /// `parse_challenge_rating` is the one lossy step on a chassis book's
    /// path; this is the guard that says it is total across every registered
    /// book. It runs the real parser, not `str::parse`: the old test asserted
    /// `cr.parse::<f32>().is_ok()`, which Monster Codex's `CR:1/2` fails while
    /// being a perfectly correct corpus token.
    ///
    /// **What it asserts is the token↔value correspondence, not a magnitude,
    /// and that is a correction Ultimate Psionics forced.** Until round 10 this
    /// read `parsed > 0.0` — which caught a token that failed to parse and fell
    /// back to zero, but only because no registered book had a row whose real
    /// CR *is* zero. Psicrystal's `up_races.lst:47` states `CR:0`, so the old
    /// form flagged a correct transcription as a defect. The sharper property
    /// is the one the old form was reaching for: a value of zero is admissible
    /// exactly when the corpus token is `"0"`, and never as a silent fallback.
    /// Same shape as the per-entry uniqueness correction Bestiary 2 forced on
    /// `every_ability_key_is_the_corpus_key` below — a guard that encoded an
    /// accidental property of the books registered when it was written.
    #[test]
    fn every_chassis_row_states_a_readable_challenge_rating() {
        for table in monster_chassis::MONSTER_BOOKS {
            for block in table.monsters {
                let cr = block.challenge_rating.expect("every row carries CR:");
                let parsed = parse_challenge_rating(table.corpus_book, block.key, cr);
                if parsed == 0.0 {
                    assert_eq!(
                        cr.trim(),
                        "0",
                        "{}/{} reads CR {cr:?} as 0 -- a token that is not literally \"0\" \
                         must never parse to zero, which is the silent-fallback defect this \
                         guard exists to catch",
                        table.corpus_book,
                        block.key
                    );
                } else {
                    assert!(
                        parsed > 0.0,
                        "{}/{} reads CR {cr:?} as {parsed}",
                        table.corpus_book,
                        block.key
                    );
                }
            }
        }
    }

    /// The fractional spelling itself, pinned on the row that introduced it.
    /// `1/2` must reach the wire as `0.5`, not as a panic and not as `1.0`.
    #[test]
    fn a_fractional_challenge_rating_reaches_the_wire_as_a_fraction() {
        let bat = build_monster_catalog()
            .entries
            .into_iter()
            .find(|e| e.key == "monster_codex:monster:bat_sootwing")
            .expect("Monster Codex's Sootwing Bat is served");
        assert_eq!(bat.challenge_rating, 0.5);
        assert_eq!(bat.book, BOOK_MC);
    }

    /// A served ability key is the corpus `KEY:`, so namespaced records stay
    /// distinguishable on the wire. Serving the display name would collapse
    /// `Caryatid Column ~ Immunity to Magic` onto any other `Immunity to Magic`.
    ///
    /// **The uniqueness asserted here is PER ENTRY, not global, and that is a
    /// correction this book forced.** Until Bestiary 2 this test asserted that
    /// no ability key was served twice anywhere in the response, and it passed
    /// for five books — because in every one of them each ability had exactly
    /// one owner. That was a property of those five books, not of the catalog:
    /// an ability is rendered underneath *each* monster that claims it, so a
    /// shared ability is served once per owner **by design**. Bestiary 2 is the
    /// first book with any, and the old assertion read 522 against 488 and
    /// failed on correct output. Re-derived over
    /// `rules_tables::bestiary_2`'s table: **19** ability records carry more
    /// than one owner and account for exactly **34** extra served rows.
    ///
    /// The two properties that DO hold are the ones the old assertion was really
    /// standing in for: a monster never lists the same ability twice, and the
    /// number of DISTINCT served keys equals the number of ability records the
    /// registry holds. A key collapsed to a display name, or a record served
    /// under two different keys, still fails here.
    #[test]
    fn bonus_bestiary_ability_keys_carry_the_namespace() {
        let response = build_monster_catalog();
        assert!(response.entries.iter().any(|e| e
            .abilities
            .iter()
            .any(|a| a.key == "bonus_bestiary:monster_ability:caryatid_column_immunity_to_magic")));

        for entry in &response.entries {
            let keys: std::collections::BTreeSet<&String> =
                entry.abilities.iter().map(|a| &a.key).collect();
            assert_eq!(
                keys.len(),
                entry.abilities.len(),
                "{} lists an ability twice",
                entry.key
            );
        }

        let distinct_served: std::collections::BTreeSet<&String> = response
            .entries
            .iter()
            .flat_map(|e| e.abilities.iter().map(|a| &a.key))
            .collect();
        // `decisions.md §20` (no_record-to-zero wave 2): the chassis now
        // holds `owner: &[]` records too (no monster row of their book
        // claims them; shipped for shape measurement, not for reach --
        // `bestiary::tests::every_owner_less_ability_is_a_named_and_pinned_
        // non_reach`, `reach_gate.rs`'s `("beastiary1", "monster_abilities")`
        // `OPEN_FINDINGS`/`UNREACHED_RECORD_FINDINGS` entries). Those never
        // reach `list_monster_catalog` (it only ever walks a monster's own
        // `ability_keys`), so this invariant now compares against the OWNED
        // subset only -- the owner-less count is asserted separately, so a
        // record silently losing its owner (which WOULD move it here) still
        // fails loudly, just under a different assertion.
        let owned_records_held: usize =
            codex::rules_core::rules_tables::monster_chassis::MONSTER_BOOKS
                .iter()
                .flat_map(|book| book.monster_abilities.iter())
                .filter(|ability| !ability.owners.is_empty())
                .count();
        let owner_less_records_held: usize =
            codex::rules_core::rules_tables::monster_chassis::MONSTER_BOOKS
                .iter()
                .flat_map(|book| book.monster_abilities.iter())
                .filter(|ability| ability.owners.is_empty())
                .count();
        assert_eq!(
            distinct_served.len(),
            owned_records_held,
            "every OWNED ability record the chassis holds reaches the wire under its own key, \
             once"
        );
        // 180 -> 881 -> 957 (`decisions.md §20`, no_record-to-zero round 3,
        // +76): five previously-unregistered ZERO-monster books
        // (`ultimate_wilderness` +2, `ultimate_intrigue` +6, `ultimate_magic`
        // +13, `bestiary_6` +16, `bestiary_5` +39 -- one owned row,
        // `Traits Output ~ Sahkil`, is a multi-DESC: parse refusal and does
        // NOT ship) registered via the identical owner-less-ship mechanism.
        // 957 -> 1027 (`decisions.md §20` round 4, +70): the last two of the
        // original 8 zero-monster books, `pathfinder_unchained` (+69, 3 of
        // its 72 orphan candidates refused during transcription as an
        // unscreenable multi-DESC: shape) and `advanced_race_guide` (+1).
        // Re-derived: `python3 scripts/shape_ledger.py --inventory
        // docs/work-inventory.json` -- `monster_ability` `no_record` 191 -> 121.
        // 1027 -> 1048 (`decisions.md §20` round 5, +21): the last of the
        // original 8 zero-monster books, `mythic_adventures` (+21, all 21
        // orphan candidates shipped, 0 refused). Re-derived: `python3
        // scripts/shape_ledger.py --inventory docs/work-inventory.json` --
        // `monster_ability` `no_record` 121 -> 100.
        // 1048 -> 1053 (`decisions.md §27b` — EVERYTHING, +5): `occult_
        // adventures`, registered for the first time, overturning four
        // cycles' worth of "correctly out of scope" for a REACHABILITY
        // finding (negated `PRECAMPAIGN` gate), not an ingest exemption. All
        // 5 orphan candidates shipped, 0 refused. Re-derived: `python3
        // scripts/transcribe_monster_tables.py occult_adventures 2>&1
        // >/dev/null` prints exactly these 5 keys as owner-less.
        // 1053 -> 1066 (`decisions.md §24`/round 7, +13): the T9
        // name-PI/desc-PI `monster_ability` group closes. 13 ability rows
        // whose own name/key matched the blacklist now ship under a
        // Codex-generated neutral name/key instead of being dropped, and
        // every one of the 13 is an orphan (`inner_sea_bestiary` +7,
        // `inner_sea_gods` +3, `inner_sea_world_guide` +3). The 2
        // description-only-PI rows this same group closes are OWNED, not
        // owner-less, so they do not move this count. Re-derived: `python3
        // scripts/shape_ledger.py --inventory docs/work-inventory.json` --
        // `monster_ability` `no_record` 98 -> 83 (before merging this
        // cycle with `§27b`'s own separate 5-unit closure).
        // 1066 -> 1076 (`decisions.md §27`/round 8, +10): the `TYPE:`-facet-
        // vocabulary-gap group closes via the provisional `SpecialQuality`
        // default. Of the 23 total defaulted rows (22 real `no_record`
        // population + 1 bonus `.COPY=` row already `text-complete` by
        // inventory evidence alone), 13 land OWNED (namespaced `<Monster> ~
        // <Ability>` keys whose owner resolves through the existing prefix
        // pass) and do not move this count; the remaining 10 are owner-less
        // (all 10 in `bestiary_3`: `Asurendra ~ None`, `Lunar/Royal/Water
        // Naga ~ Spells`, `Unfettered Eidolon ~
        // Str/Dex/Con/Int/Wis/Cha`). Re-derived: `python3
        // scripts/transcribe_monster_tables.py bestiary_3 2>&1 >/dev/null`.
        // 1076 -> 1126 (`decisions.md §27b` round 9, +50): the multi-DESC:
        // `PREVAREQ`/`PREVARGT`/`PRESIZE*`/`PREHD`/`PRERACE`/`PRETEMPLATE`/
        // `PREABILITY`-gated parse-refusal group closes via `parse_desc`'s
        // new generalised sixth branch across 8 books -- `bestiary` +17,
        // `bestiary_3` +10, `bestiary_4` +7, `horror_adventures` +9,
        // `inner_sea_bestiary` +3, `bestiary_5` +1, `pathfinder_unchained`
        // +3, all owner-less (shared reference-library text no single stat
        // block owns); `bestiary_2`'s 2 closed units are both OWNED and do
        // not move this count. Re-derived: `python3 scripts/shape_ledger.py
        // --inventory docs/work-inventory.json` -- `monster_ability`
        // `no_record` 56 -> 0.
        assert_eq!(
            owner_less_records_held, 1126,
            "the owner-less (shape-measured-but-not-reachable) record count moved -- re-derive \
             from each book's own `scripts/transcribe_monster_tables.py <book>` stderr and \
             update both this pin and `reach_gate.rs`'s matching entries"
        );
    }

    /// No ability description reaches the wire carrying PCGen substitution
    /// syntax. This is the defect the on-screen pass caught: `Babble` printed
    /// "a DC %1 Will save" until `serve_ability_description` was introduced.
    #[test]
    fn no_ability_description_serves_raw_pcgen_substitution_syntax() {
        let mut checked = 0usize;
        for entry in build_monster_catalog().entries.iter().filter(|e| e.book == BOOK_BB) {
            for ability in &entry.abilities {
                let Some(description) = ability.description.as_deref() else { continue };
                checked += 1;
                assert!(
                    !description.contains('%'),
                    "{}'s description serves a raw substitution placeholder: {description}",
                    ability.key
                );
                assert!(
                    codex::rules_core::pcgen_desc::leaked_pcgen_syntax(description).is_none(),
                    "{} leaks PCGen syntax",
                    ability.key
                );
            }
        }
        // 16 of the book's 17 ability rows carry `DESC:` text; the 17th
        // (`Magic Circle against Evil`) carries none at all.
        assert_eq!(checked, 16);
    }

    /// An attack with no corpus dice serves `None` plus the sentence saying why
    /// — never `"0"`, which is a real value elsewhere in this same response.
    #[test]
    fn an_attack_with_no_corpus_dice_serves_none_and_says_so() {
        let response = build_monster_catalog();
        let bb: Vec<_> = response.entries.iter().filter(|e| e.book == BOOK_BB).collect();
        let mut absent = 0;
        for entry in &bb {
            for attack in &entry.natural_attacks {
                if attack.damage_dice.is_none() {
                    absent += 1;
                    assert_eq!(attack.damage_dice_source, DICE_ABSENT_FROM_CORPUS);
                    assert!(attack.grounding_note.is_some());
                }
            }
        }
        assert_eq!(absent, 13, "13 of this book's 14 named attacks carry no dice");
        let allip = bb.iter().find(|e| e.name == "Allip").expect("Allip is served");
        assert_eq!(allip.natural_attacks[0].damage_dice.as_deref(), Some("0"));
        assert_eq!(allip.natural_attacks[0].damage_dice_source, DICE_FROM_MONSTER_ROW);
    }

    /// A grounding note names the book the row actually came from.
    ///
    /// **This test exists because the screen said otherwise.** Driving the app
    /// (DoD item 8) showed Monster Codex's Seru rendering *"the Bonus Bestiary
    /// corpus carries no die expression for it"* — the note hard-coded one
    /// book's name and one token shape, and both were wrong on the second book.
    /// A player reading that sentence would look the creature up in the wrong
    /// book.
    ///
    /// Pinned in both directions: every note must name its own book, and no
    /// note may name a book it does not belong to.
    #[test]
    fn a_grounding_note_never_names_another_books_corpus() {
        let mut checked = 0;
        for entry in build_monster_catalog().entries {
            // Bestiary 1 serves its SD-22 notes from published-text provenance,
            // which names a page rather than a book. Since round 8 that table's
            // rows share `BOOK_B1` with the chassis half, so the wire code no
            // longer separates them and the key namespace does.
            if entry.key.starts_with("beastiary1:monster:") {
                continue;
            }
            let Some(table) = monster_chassis::MONSTER_BOOKS
                .iter()
                .find(|t| book_wire_code(t.corpus_book) == entry.book)
            else {
                continue;
            };
            let own = book_display_name(table.corpus_book);
            for attack in &entry.natural_attacks {
                let Some(note) = attack.grounding_note.as_deref() else {
                    continue;
                };
                checked += 1;
                assert!(
                    note.contains(own),
                    "{}'s {} note does not name {own}: {note}",
                    entry.name,
                    attack.name
                );
                for other in monster_chassis::MONSTER_BOOKS {
                    if other.corpus_book == table.corpus_book {
                        continue;
                    }
                    let foreign = book_display_name(other.corpus_book);
                    assert!(
                        !note.contains(foreign),
                        "{}'s {} note names {foreign}, a book it did not come from: {note}",
                        entry.name,
                        attack.name
                    );
                }
            }
        }
        assert!(checked > 0, "no grounding note was examined, so this asserts nothing");
    }

    /// The key derivation is checked against the engine's own resolver rather
    /// than trusted, so a served key can never be one nothing can look up.
    #[test]
    fn every_served_key_resolves_back_to_its_record() {
        let all = build_monster_catalog().entries;
        for entry in hand_modelled_rows(&all) {
            let resolved = beastiary1::monster_key_resolve(&entry.key, RuleSetId::Bestiary1)
                .unwrap_or_else(|| panic!("served key `{}` resolves to no record", entry.key));
            assert_eq!(resolved.name, entry.name);
        }

        // Bonus Bestiary has no key resolver of its own -- its identity is the
        // corpus `KEY:` token -- so the round trip is the derivation itself:
        // every served key must be one `chassis_key` produces from a record
        // the table actually holds, and every record must produce one --
        // checked per registered book, so a book whose rows never reach the
        // response fails here rather than passing by being absent.
        let response = build_monster_catalog();
        // The keys served under a wire code by a table that is NOT in this
        // registry. Bestiary 1's SD-22 half is the only one, and it is
        // round-tripped above through its own resolver; without subtracting it
        // this comparison would demand that the chassis derive keys it does not
        // own. Subtracted by namespace rather than skipped by book, so a chassis
        // key that drifted into a foreign namespace still fails here.
        let foreign_namespaces = ["beastiary1:monster:"];
        for table in monster_chassis::MONSTER_BOOKS {
            let wire_code = book_wire_code(table.corpus_book);
            let served: std::collections::BTreeSet<String> = response
                .entries
                .iter()
                .filter(|e| e.book == wire_code)
                .filter(|e| !foreign_namespaces.iter().any(|ns| e.key.starts_with(ns)))
                .map(|e| e.key.clone())
                .collect();
            let derived: std::collections::BTreeSet<String> = table
                .monsters
                .iter()
                .map(|m| chassis_key(table.corpus_book, "monster", m.key))
                .collect();
            assert_eq!(served, derived, "{} served keys", table.corpus_book);
        }
    }

    #[test]
    fn no_key_is_served_twice_so_a_row_is_unambiguous() {
        let entries = build_monster_catalog().entries;
        let mut keys: Vec<String> = entries.iter().map(|entry| entry.key.clone()).collect();
        keys.sort();
        let total = keys.len();
        keys.dedup();
        assert_eq!(keys.len(), total, "the catalog serves a duplicate monster key");
    }

    /// Every row carries readable payload beyond its own identity — the bar
    /// `reach_gate.rs` applies, asserted here at the source so a regression
    /// names itself in this file first.
    /// **`source_page` is the one field here that the corpus does not always
    /// state**, and Bestiary 3 is the book that proved it. The invariant held
    /// for seven books because every row in all seven happened to carry a
    /// `SOURCEPAGE:` token — a property of those books' data, not of the
    /// format. Re-derived against the corpus rather than inferred from the
    /// failure:
    ///
    /// ```text
    /// sed -n '215p;265p' b3_races.lst | tr '\t' '\n' | grep -c SOURCEPAGE   -> 0
    /// ```
    ///
    /// The transcriber emits `None` for a token the row does not carry, which
    /// is exactly right — the alternative is inventing a citation. Both records
    /// state everything else the screen renders (name, size, type, challenge
    /// rating, speeds, natural attacks), so dropping them for a missing page
    /// reference would withhold real content over a bibliographic field. They
    /// ship, the screen omits the page clause for them (`MonsterCatalogScreen`
    /// renders it conditionally, as it has always done for ability rows), and
    /// the two are pinned here BY CORPUS LINE so a third one cannot appear
    /// silently.
    #[test]
    fn every_row_carries_the_fields_the_screen_renders() {
        // The corpus rows that state no `SOURCEPAGE:` token, by the book and
        // line each one is. Keyed by served key so a renamed record still
        // matches the line it came from.
        const NO_SOURCE_PAGE: &[&str] =
            &["bestiary_3:monster:owl_giant", "bestiary_3:monster:spider_ogre"];

        // The served rows whose challenge rating is genuinely zero, pinned the
        // same way and for the same reason: a CR of 0 used to be impossible
        // here, so `> 0.0` doubled as an "it parsed" check. Psicrystal states
        // `CR:0` on `up_races.lst:47`. Pinning the set keeps that check alive —
        // a row losing its rating still fails — while admitting the real value.
        const ZERO_CHALLENGE_RATING: &[&str] = &["ultimate_psionics:monster:psicrystal"];

        let mut seen_without_page: Vec<&str> = Vec::new();
        let mut seen_zero_cr: Vec<&str> = Vec::new();
        let response = build_monster_catalog();
        for entry in &response.entries {
            assert!(!entry.name.trim().is_empty(), "{} has no name", entry.key);
            assert!(!entry.size.trim().is_empty(), "{} has no size", entry.key);
            assert!(
                !entry.race_type.trim().is_empty(),
                "{} has no creature type",
                entry.key
            );
            if entry.source_page.trim().is_empty() {
                assert!(
                    NO_SOURCE_PAGE.contains(&entry.key.as_str()),
                    "{} has no source page, and is not one of the two corpus rows \
                     (`b3_races.lst:215`, `:265`) known to state none. Check the row before \
                     adding it here: a page that vanished from a row that used to have one is \
                     a transcription defect, not a corpus fact.",
                    entry.key
                );
                seen_without_page.push(entry.key.as_str());
            }
            if entry.challenge_rating == 0.0 {
                assert!(
                    ZERO_CHALLENGE_RATING.contains(&entry.key.as_str()),
                    "{} serves challenge rating 0, and is not one of the corpus rows known to \
                     state `CR:0`. Check the row before adding it here: a rating that vanished \
                     from a row that used to have one is a transcription defect, not a corpus \
                     fact.",
                    entry.key
                );
                seen_zero_cr.push(entry.key.as_str());
            } else {
                assert!(
                    entry.challenge_rating > 0.0,
                    "{} has no challenge rating",
                    entry.key
                );
            }
        }
        seen_without_page.sort_unstable();
        assert_eq!(
            seen_without_page, NO_SOURCE_PAGE,
            "the set of records serving no source page changed; a pinned one gaining a page is \
             as much a signal as a new one losing it"
        );
        seen_zero_cr.sort_unstable();
        assert_eq!(
            seen_zero_cr, ZERO_CHALLENGE_RATING,
            "the set of records serving challenge rating 0 changed; a pinned one gaining a \
             rating is as much a signal as a new one losing it"
        );
    }

    /// The three records whose land speed is genuinely `0`, pinned by name so
    /// the screen's "no land speed" wording can never quietly start applying to
    /// a monster that really does walk.
    ///
    /// Derived by scanning the live table, not from the ingest notes: these are
    /// the only `speed_ft == 0` records, and each one's own doc comment cites a
    /// `MOVE:` token with no `Walk` pair.
    #[test]
    fn a_land_speed_of_zero_is_a_real_corpus_value_on_exactly_four_records() {
        // SD28-E16 subset 09 (2026-08-07) added a fourth: Shadow
        // (`b1_races.lst:357`, `MOVE:Fly,40` with no `Walk` component at
        // all -- the first Bestiary 1 record with no walk speed token
        // whatsoever, not merely a walk speed of 0). `speed_ft` is 0
        // because the row states no walk movement, never a guessed value.
        let entries = build_monster_catalog().entries;
        let landless: Vec<&str> = hand_modelled_rows(&entries)
            .into_iter()
            .filter(|entry| entry.speed_ft == 0)
            .map(|entry| entry.name.as_str())
            .collect();
        assert_eq!(landless, vec!["Shark", "Squid", "Vargouille", "Shadow"]);

        // Bonus Bestiary's own instance, and the reason `speeds` exists: Allip
        // is `MOVE:Fly,30` with no `Walk` pair, so its land speed is genuinely
        // 0 -- but unlike a Bestiary 1 row it still serves a movement mode, so
        // the screen prints "No land speed, fly 30 ft." rather than nothing.
        let bb_landless: Vec<&MonsterCatalogEntryDto> = entries
            .iter()
            .filter(|entry| entry.book == BOOK_BB && entry.speed_ft == 0)
            .collect();
        assert_eq!(bb_landless.len(), 1);
        assert_eq!(bb_landless[0].name, "Allip");
        assert_eq!(bb_landless[0].speeds, vec![SpeedDto { mode: "Fly".to_owned(), feet: 30 }]);

        for entry in entries.iter().filter(|entry| entry.speed_ft > 0) {
            assert!(entry.speed_ft >= 5, "{} has an implausible land speed", entry.key);
        }
    }

    /// A real record, pinned end to end against its own corpus row
    /// (`b1_races.lst:18`, transcribed in `monster_subset_07::ankheg`).
    #[test]
    fn a_real_monster_reaches_the_catalog_with_its_corpus_fields() {
        let entries = build_monster_catalog().entries;
        let ankheg = entries
            .iter()
            .find(|entry| entry.key == "beastiary1:monster:ankheg")
            .expect("Ankheg is a real Bestiary 1 record");

        assert_eq!(ankheg.name, "Ankheg");
        assert_eq!(ankheg.challenge_rating, 3.0);
        assert_eq!(ankheg.size, "L");
        assert_eq!(ankheg.speed_ft, 30);
        assert_eq!(ankheg.race_type, "Magical Beast");
        assert_eq!(ankheg.race_subtype, None);
        assert_eq!(ankheg.source_page, "p.15");
        assert_eq!(ankheg.natural_attacks.len(), 1);
        assert_eq!(ankheg.natural_attacks[0].name, "Bite");
        assert_eq!(ankheg.natural_attacks[0].damage_dice.as_deref(), Some("2d6"));
    }

    /// `SD31-W23-MONSTER-001`: the cross-table-owner remedy `decisions.md
    /// §58.3` scoped and `SD31-W22-MONSTER-001` bounded but did not build.
    /// Ankheg's two ability rows (`b1_abilities_race.lst:90`/`91`) are
    /// well-formed and owned, but their owner's `MonsterStatBlock` ships from
    /// `rules_tables::beastiary1`, not from the `bestiary` chassis that holds
    /// the `MonsterAbilityRecord`s themselves -- exactly the split this test
    /// exercises end to end, through the real `map_monster` production path,
    /// not a chassis-layer-only unit test that a desktop rendering gap could
    /// still hide behind (`SD31-W22-MONSTER-001`'s own finding about
    /// `map_beastiary1_monster` hardcoding `Vec::new()`).
    #[test]
    fn a_bestiary_1_legacy_monster_carries_its_cross_table_owned_abilities() {
        let entries = build_monster_catalog().entries;
        let ankheg = entries
            .iter()
            .find(|entry| entry.key == "beastiary1:monster:ankheg")
            .expect("Ankheg is a real Bestiary 1 record");

        assert_eq!(
            ankheg.abilities.len(),
            2,
            "Ankheg's row names exactly two abilities in `b1_abilities_race.lst`, got {:?}",
            ankheg.abilities.iter().map(|a| &a.name).collect::<Vec<_>>()
        );
        let bite = ankheg
            .abilities
            .iter()
            .find(|a| a.name == "Acid Bite")
            .expect("Acid Bite reaches the catalog");
        assert_eq!(
            bite.description.as_deref(),
            Some(
                "An Ankheg's bite does an additional 1d4 acid damage unless it has recently \
                 used it's spit acid ability."
            )
        );
        let spit = ankheg
            .abilities
            .iter()
            .find(|a| a.name == "Spit Acid")
            .expect("Spit Acid reaches the catalog");
        // The row's own DC is a runtime formula (`10+(HD/2)+CON`), which this
        // ingest does not compute -- `render_pcgen_desc` drops the `%1`
        // placeholder rather than fabricate a number, so the rendered text
        // must not carry it. Guards against a regression `serve_ability_
        // description`'s own leak-panic would otherwise catch loudly, but a
        // silent value would not.
        assert!(
            !spit.description.as_deref().unwrap_or_default().contains('%'),
            "Spit Acid's rendered description leaks an unresolved placeholder: {:?}",
            spit.description
        );
        assert!(spit.description.as_deref().unwrap_or_default().contains("30-foot line of acid"));
    }

    /// The first production caller of
    /// `derived_evaluator_fixture_check::spell_like_ability_caster_level`
    /// (SD31-E6-F1-002, `OPEN-ISSUES.md` row 44 -- the wave-3 seam that built
    /// the function had zero). Demon (Balor) is one of the seam's own 7
    /// committed fixtures: `MONSTERCLASS:Outsider (Fort/Will):20` states 20
    /// Hit Dice, and its row carries `BONUS:VAR|SLA_CL|HD`
    /// (`b1_races.lst:93`), so PF1's Spell-Like Abilities universal monster
    /// rule gives it caster level 20 on the wire, not merely in a test.
    #[test]
    fn a_monster_with_spell_like_abilities_serves_its_universal_monster_rule_caster_level() {
        let entries = build_monster_catalog().entries;
        let balor = entries
            .iter()
            .find(|entry| entry.book == BOOK_B1 && entry.name == "Demon (Balor)")
            .expect("Demon (Balor) is a real chassis record served under Bestiary 1's wire code");
        assert_eq!(
            balor.spell_like_ability_caster_level,
            Some(20),
            "Balor's MONSTERCLASS states 20 Hit Dice and its row carries BONUS:VAR|SLA_CL|HD"
        );
    }

    /// The save-DC seam's own production reach (SD31-W15-MONSTER-SLA-001):
    /// a real record's `SPELLS:` grants reach the wire, each carrying the
    /// spell level PF1's Universal Monster Rule derives from the row's own DC
    /// constant.
    ///
    /// Hag (Annis) (`bb_races.lst:14`) states
    /// `SPELLS:Innate|TIMES=3|CASTERLEVEL=7|Disguise Self,11+CHA|Fog Cloud,12+CHA`.
    /// Disguise self is a 1st-level spell and fog cloud a 2nd — which is what
    /// `11 - 10` and `12 - 10` say AND what `cr_spells.lst`'s own `CLASSES:`
    /// tokens independently state; this record is one of the 63 the seam's
    /// committed fixture banks.
    #[test]
    fn a_monsters_spell_like_ability_grants_reach_the_wire_with_their_derived_spell_level() {
        let entries = build_monster_catalog().entries;
        let annis = entries
            .iter()
            .find(|entry| entry.name == "Hag (Annis)")
            .expect("Hag (Annis) is a real Bonus Bestiary chassis record");
        assert_eq!(
            annis.spell_like_abilities.len(),
            2,
            "the row grants exactly two spell-like abilities, got {:?}",
            annis.spell_like_abilities
        );
        let disguise = annis
            .spell_like_abilities
            .iter()
            .find(|sla| sla.spell == "Disguise Self")
            .expect("the row grants Disguise Self");
        assert_eq!(disguise.derived_spell_level, Some(1));
        assert_eq!(disguise.save_dc_ability.as_deref(), Some("CHA"));
        assert_eq!(
            disguise.save_dc_token.as_deref(),
            Some("11+CHA"),
            "the DC reaches the screen as the formula the row states -- a monster's ability \
             SCORES are not a corpus fact here, so a resolved number would be fabricated"
        );
        assert_eq!(disguise.times.as_deref(), Some("3"));
        assert_eq!(disguise.caster_level_token.as_deref(), Some("7"));
        let fog = annis
            .spell_like_abilities
            .iter()
            .find(|sla| sla.spell == "Fog Cloud")
            .expect("the row grants Fog Cloud");
        assert_eq!(fog.derived_spell_level, Some(2));
    }

    /// A spell the row states no save for must serve `None` for BOTH the DC
    /// token and the derived level — never a zero, which would read as a
    /// cantrip on screen.
    #[test]
    fn a_granted_spell_with_no_save_serves_no_dc_and_no_derived_level() {
        let entries = build_monster_catalog().entries;
        let served: Vec<&MonsterSpellLikeAbilityDto> = entries
            .iter()
            .flat_map(|entry| entry.spell_like_abilities.iter())
            .filter(|sla| sla.save_dc_token.is_none())
            .collect();
        assert!(
            !served.is_empty(),
            "the registry serves grants with no save DC; if this is empty the assertion below \
             is vacuous"
        );
        for sla in served {
            assert_eq!(
                sla.derived_spell_level, None,
                "{:?} states no save DC, so no spell level can be derived from one",
                sla.spell
            );
            assert_eq!(sla.save_dc_ability, None, "{:?}", sla.spell);
        }
    }

    /// A monster with a perfectly readable `MONSTERCLASS:` token but no
    /// `BONUS:VAR|SLA_CL|` token at all must not be served a caster level it
    /// has no spell-like abilities to attach to — a number with nothing
    /// behind it is exactly the class of defect this file's
    /// `serve_ability_description` leak-check exists to catch for a
    /// different field.
    #[test]
    fn a_monster_with_no_spell_like_abilities_serves_no_caster_level() {
        let entries = build_monster_catalog().entries;
        let animated_object = entries
            .iter()
            .find(|entry| entry.book == BOOK_B1 && entry.name == "Animated Object (Medium)")
            .expect("Animated Object (Medium) is a real chassis record");
        assert_eq!(
            animated_object.spell_like_ability_caster_level, None,
            "Animated Object (Medium)'s row (b1_races.lst:13) carries no SLA_CL token"
        );

        // And the SD-22 half of Bestiary 1, whose ingest does not capture
        // abilities at all, must answer the same honest `None` rather than
        // guessing from a `monster_class` it happens to have.
        let ankheg = entries
            .iter()
            .find(|entry| entry.key == "beastiary1:monster:ankheg")
            .expect("Ankheg is a real Bestiary 1 record");
        assert_eq!(
            ankheg.spell_like_ability_caster_level, None,
            "Bestiary 1's SD-22 half never ingested monster_ability records, so it cannot \
             honestly say whether Ankheg has spell-like abilities"
        );
    }

    /// A monster whose row genuinely declares no natural attack is served with
    /// an empty list, not an invented one.
    #[test]
    fn a_monster_with_no_natural_attack_token_is_served_with_an_empty_list() {
        let entries = build_monster_catalog().entries;
        let gnoll = entries
            .iter()
            .find(|entry| entry.key == "beastiary1:monster:gnoll")
            .expect("Gnoll is a real Bestiary 1 record");
        assert!(
            gnoll.natural_attacks.is_empty(),
            "Gnoll's row carries no NATURALATTACKS: token — it fights with a manufactured \
             longspear — so the list must be empty rather than filled in"
        );
    }

    /// The provenance distinction is real and both sides are populated: a
    /// version of this adapter that labelled everything the same way would pass
    /// every other test here.
    #[test]
    fn grounded_damage_dice_are_labelled_distinctly_from_corpus_row_dice() {
        let entries = build_monster_catalog().entries;
        let attacks: Vec<&NaturalAttackDto> = entries
            .iter()
            .flat_map(|entry| entry.natural_attacks.iter())
            .collect();

        let from_row = attacks
            .iter()
            .filter(|attack| attack.damage_dice_source == DICE_FROM_MONSTER_ROW)
            .count();
        let cross_reference = attacks
            .iter()
            .filter(|attack| attack.damage_dice_source == DICE_FROM_CORPUS_CROSS_REFERENCE)
            .count();
        let published = attacks
            .iter()
            .filter(|attack| attack.damage_dice_source == DICE_FROM_PUBLISHED_TEXT)
            .count();
        // The fourth label, added with Bonus Bestiary: the corpus states no
        // dice and none were grounded. Counted here rather than excluded, so
        // the "every attack is labelled" invariant stays total.
        let absent = attacks
            .iter()
            .filter(|attack| attack.damage_dice_source == DICE_ABSENT_FROM_CORPUS)
            .count();

        assert_eq!(
            from_row + cross_reference + published + absent,
            attacks.len(),
            "every served attack must carry one of the four known provenance labels"
        );
        assert!(absent > 0, "the no-dice label must be exercised by a real record");
        assert!(from_row > 0, "most attacks are transcribed corpus-row tokens");
        assert_eq!(
            cross_reference + published,
            natural_attack_provenance::GROUNDED_NATURAL_ATTACKS.len(),
            "every grounded-attack provenance row must reach exactly one served attack"
        );

        // A grounded attack always explains itself; a plain corpus-row one has
        // nothing extra to say and must not manufacture a note.
        for attack in &attacks {
            if attack.damage_dice_source == DICE_FROM_MONSTER_ROW {
                assert!(attack.grounding_note.is_none(), "{}", attack.name);
            } else {
                assert!(
                    attack.grounding_note.as_deref().is_some_and(|note| !note.trim().is_empty()),
                    "{} is grounded and must carry its provenance note",
                    attack.name
                );
            }
        }
    }

    /// Ankheg's Bite is the module doc comment's own worked example: named by a
    /// cross-reference on its row, dice grounded from published text.
    #[test]
    fn a_grounded_attack_carries_its_published_source_verbatim() {
        let entries = build_monster_catalog().entries;
        let bite = &entries
            .iter()
            .find(|entry| entry.key == "beastiary1:monster:ankheg")
            .expect("Ankheg is a real Bestiary 1 record")
            .natural_attacks[0];

        assert_eq!(bite.damage_dice_source, DICE_FROM_PUBLISHED_TEXT);
        let note = bite.grounding_note.as_deref().expect("a grounded attack carries its note");
        assert!(note.contains("ABILITY:Internal|AUTOMATIC|Bite"), "{note}");
        assert!(note.contains("bite +5 (2d6+4 plus 1d4 acid and grab)"), "{note}");
        assert!(note.contains("aonprd.com"), "{note}");
    }

    /// The two multi-subtype rows read as prose, byte-exact.
    ///
    /// Pinned as whole strings rather than "contains no pipe", because the
    /// point is not only that the separator left — it is that every subtype
    /// survived the join. A renderer that dropped everything after the first
    /// `|` would also pass a pipe-absence check.
    #[test]
    fn the_two_multi_subtype_monsters_read_as_prose_not_as_a_pcgen_token() {
        let entries = build_monster_catalog().entries;
        let subtype_of = |key: &str| -> String {
            entries
                .iter()
                .find(|entry| entry.key == key)
                .unwrap_or_else(|| panic!("{key} is a real Bestiary 1 record"))
                .race_subtype
                .clone()
                .unwrap_or_else(|| panic!("{key} carries a RACESUBTYPE token"))
        };

        // Was served as `Evil|Extraplanar` (monster_subset_06.rs:134).
        assert_eq!(subtype_of("beastiary1:monster:vargouille"), "Evil, Extraplanar");
        // Was served as `Evil|Extraplanar|Fire|Lawful` (monster_subset_08.rs:141).
        assert_eq!(
            subtype_of("beastiary1:monster:hell_hound"),
            "Evil, Extraplanar, Fire, Lawful"
        );
    }

    /// No served string field carries the PCGen multi-value separator.
    ///
    /// Swept across every field the screen renders, not only `race_subtype`,
    /// so the next token that turns out to be multi-valued fails here rather
    /// than shipping. `grounding_note` is exempt and says why in its own
    /// arm: it deliberately *quotes* corpus tokens
    /// (`ABILITY:Internal|AUTOMATIC|Bite`) as evidence of provenance, which is
    /// the one place the separator is content rather than syntax.
    #[test]
    fn no_rendered_field_serves_a_raw_pcgen_multi_value_separator() {
        let mut checked = 0usize;
        let mut check = |key: &str, field: &str, value: &str| {
            checked += 1;
            assert!(
                !value.contains('|'),
                "{key}'s {field} serves the raw PCGen multi-value separator to the player: {value}"
            );
        };

        for entry in &build_monster_catalog().entries {
            check(&entry.key, "name", &entry.name);
            check(&entry.key, "size", &entry.size);
            check(&entry.key, "race_type", &entry.race_type);
            check(&entry.key, "source_page", &entry.source_page);
            if let Some(subtype) = entry.race_subtype.as_deref() {
                check(&entry.key, "race_subtype", subtype);
            }
            for attack in &entry.natural_attacks {
                check(&entry.key, "attack name", &attack.name);
                // `damage_dice` is `None` for an attack the corpus never dices;
                // there is no string to sweep, and skipping it here is not a
                // gap -- the `checked` floor below still requires the sweep to
                // have walked the whole catalog.
                if let Some(dice) = attack.damage_dice.as_deref() {
                    check(&entry.key, "attack damage_dice", dice);
                }
            }
            for ability in &entry.abilities {
                check(&entry.key, "ability name", &ability.name);
                check(&entry.key, "ability facet", &ability.facet);
                if let Some(description) = ability.description.as_deref() {
                    check(&entry.key, "ability description", description);
                }
            }
        }

        // The sweep is only worth its green if it actually walked the catalog.
        // Derived from `MonsterId::ALL.len()` rather than re-pinning a
        // literal (SD28-E16, decisions.md §36 instance 7): a `>=` bound
        // against a stale literal still passes as the roster grows, silently
        // checking a shrinking fraction of the real catalog every time a
        // subset lands -- exactly the "hand-maintained number beside a
        // derivable one" shape this decision names, just inside an
        // assertion instead of a lookup table.
        assert!(
            checked >= MonsterId::ALL.len() * 4,
            "the guard inspected only {checked} fields; it is no longer covering the catalog"
        );
    }

    /// 20 of 46 rows carry a subtype at all (18 of 41 before SD28-E16 subset
    /// 09 added Ogre and Shadow), and exactly 2 of those are multi-valued.
    /// Derived here rather than asserted from memory, so the
    /// scale of the fix stays honest as the roster changes — this assertion
    /// was first written as "13 of 41" from a miscounted grep and was
    /// corrected by its own failure, which is the reason it exists.
    #[test]
    fn the_multi_subtype_population_is_exactly_two_of_twenty() {
        // SD28-E16 subset 09 (2026-08-07) added two more single-value
        // RACESUBTYPE rows -- Ogre ("Giant") and Shadow ("Incorporeal") --
        // moving the denominator 18 -> 20. The multi-valued (pipe-separated)
        // population is unchanged: still only Vargouille and Hell Hound.
        //
        // Scoped to Bestiary 1 (SD-29 Epic 5): Bonus Bestiary joined the same
        // response and carries 3 subtype rows of its own, one of them
        // multi-valued (Shadow Mastiff's `Evil|Extraplanar`). Widening the
        // denominators would have made this assertion say nothing about the
        // population it was written for, so the book filter is the fix and the
        // new book gets its own assertion below.
        let entries = build_monster_catalog().entries;
        let with_subtype: Vec<&MonsterCatalogEntryDto> = hand_modelled_rows(&entries)
            .into_iter()
            .filter(|entry| entry.race_subtype.is_some())
            .collect();
        let multi = with_subtype
            .iter()
            .filter(|entry| entry.race_subtype.as_deref().is_some_and(|s| s.contains(", ")))
            .count();

        assert_eq!(with_subtype.len(), 20, "rows carrying a RACESUBTYPE token");
        assert_eq!(multi, 2, "rows whose RACESUBTYPE token is multi-valued");

        // Bonus Bestiary's own population, derived the same way: Allip
        // (`Incorporeal`), Nixie (`Aquatic`) and Shadow Mastiff
        // (`Evil|Extraplanar`), the last of which must reach the wire already
        // joined as prose.
        let bb_with_subtype: Vec<&MonsterCatalogEntryDto> = entries
            .iter()
            .filter(|entry| entry.book == BOOK_BB && entry.race_subtype.is_some())
            .collect();
        assert_eq!(bb_with_subtype.len(), 3);
        let bb_multi: Vec<&str> = bb_with_subtype
            .iter()
            .filter_map(|entry| entry.race_subtype.as_deref())
            .filter(|s| s.contains(", "))
            .collect();
        assert_eq!(bb_multi, vec!["Evil, Extraplanar"]);
    }

    /// The renderer itself, on shapes the corpus does not currently contain.
    #[test]
    fn the_subtype_renderer_drops_empty_segments_rather_than_rendering_them() {
        assert_eq!(serve_race_subtype("Aquatic"), "Aquatic");
        assert_eq!(serve_race_subtype("Evil|Extraplanar"), "Evil, Extraplanar");
        assert_eq!(serve_race_subtype("A||B"), "A, B");
        assert_eq!(serve_race_subtype("A| B "), "A, B");
        assert_eq!(serve_race_subtype(""), "");
    }
}
