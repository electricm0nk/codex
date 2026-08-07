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
use codex::rules_core::rules_tables::RuleSetId;

/// The one book this catalog serves. A wire code rather than a display label,
/// matching `spell_catalog.rs`/`equipment_catalog.rs`'s convention; the
/// frontend maps it to "Bestiary 1".
const BOOK_B1: &str = "B1";

/// Where one natural attack's `damage_dice` came from.
///
/// Wire form of `natural_attack_provenance::AttackSource`, flattened to the
/// distinction a reader of the screen actually needs: is this die expression a
/// corpus token, or a value grounded from the published book because the corpus
/// genuinely has none?
const DICE_FROM_MONSTER_ROW: &str = "monsterRowToken";
const DICE_FROM_CORPUS_CROSS_REFERENCE: &str = "corpusCrossReferenceToken";
const DICE_FROM_PUBLISHED_TEXT: &str = "publishedText";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NaturalAttackDto {
    /// The attack's name, e.g. `"Bite"`.
    pub name: String,
    /// The die expression only, with no Strength modifier (`"1d6"`, not
    /// `"1d6+1"`). `"0"` is a real attack that deals no damage — Cave Fisher's
    /// Filament, whose own corpus token ends `,*1,0`.
    pub damage_dice: String,
    /// One of [`DICE_FROM_MONSTER_ROW`], [`DICE_FROM_CORPUS_CROSS_REFERENCE`]
    /// or [`DICE_FROM_PUBLISHED_TEXT`].
    pub damage_dice_source: String,
    /// For a grounded attack, the verbatim published "Melee" line the dice were
    /// read from, kept so a reader can re-check the transcription. `None` for an
    /// attack transcribed from the monster's own row, which has nothing extra to
    /// show.
    pub grounding_note: Option<String>,
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
        damage_dice: attack.damage_dice.clone(),
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
    }
}

/// Build the full monster catalog response, in `MonsterId::ALL`'s own
/// declaration order (subsets 01-08, i.e. ascending challenge rating then
/// alphabetical within a band). A thin, testable wrapper behind the Tauri
/// command below, mirroring `build_spell_catalog`'s command/pure-fn split.
pub fn build_monster_catalog() -> MonsterCatalogResponse {
    MonsterCatalogResponse {
        entries: MonsterId::ALL.iter().copied().map(map_monster).collect(),
    }
}

#[tauri::command]
pub fn list_monster_catalog() -> MonsterCatalogResponse {
    build_monster_catalog()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_catalog_serves_every_ingested_bestiary_1_monster() {
        let response = build_monster_catalog();
        assert_eq!(response.entries.len(), MonsterId::ALL.len());
        assert_eq!(
            response.entries.len(),
            46,
            "Bestiary 1's ingested roster is 46 stat blocks (subsets 01-09, SD28-E16); if the \
             roster grew, re-derive this from the corpus rather than relaxing it"
        );
        for entry in &response.entries {
            assert_eq!(entry.book, BOOK_B1);
        }
    }

    /// The key derivation is checked against the engine's own resolver rather
    /// than trusted, so a served key can never be one nothing can look up.
    #[test]
    fn every_served_key_resolves_back_to_its_record() {
        for entry in &build_monster_catalog().entries {
            let resolved = beastiary1::monster_key_resolve(&entry.key, RuleSetId::Bestiary1)
                .unwrap_or_else(|| panic!("served key `{}` resolves to no record", entry.key));
            assert_eq!(resolved.name, entry.name);
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
    #[test]
    fn every_row_carries_the_fields_the_screen_renders() {
        for entry in &build_monster_catalog().entries {
            assert!(!entry.name.trim().is_empty(), "{} has no name", entry.key);
            assert!(!entry.size.trim().is_empty(), "{} has no size", entry.key);
            assert!(
                !entry.race_type.trim().is_empty(),
                "{} has no creature type",
                entry.key
            );
            assert!(
                !entry.source_page.trim().is_empty(),
                "{} has no source page",
                entry.key
            );
            assert!(
                entry.challenge_rating > 0.0,
                "{} has no challenge rating",
                entry.key
            );
        }
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
        let landless: Vec<&str> = entries
            .iter()
            .filter(|entry| entry.speed_ft == 0)
            .map(|entry| entry.name.as_str())
            .collect();
        assert_eq!(landless, vec!["Shark", "Squid", "Vargouille", "Shadow"]);
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
        assert_eq!(ankheg.natural_attacks[0].damage_dice, "2d6");
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

        assert_eq!(
            from_row + cross_reference + published,
            attacks.len(),
            "every served attack must carry one of the three known provenance labels"
        );
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
                check(&entry.key, "attack damage_dice", &attack.damage_dice);
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
        let entries = build_monster_catalog().entries;
        let with_subtype: Vec<&MonsterCatalogEntryDto> =
            entries.iter().filter(|entry| entry.race_subtype.is_some()).collect();
        let multi = with_subtype
            .iter()
            .filter(|entry| entry.race_subtype.as_deref().is_some_and(|s| s.contains(", ")))
            .count();

        assert_eq!(with_subtype.len(), 20, "rows carrying a RACESUBTYPE token");
        assert_eq!(multi, 2, "rows whose RACESUBTYPE token is multi-valued");
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
