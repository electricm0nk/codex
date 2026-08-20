//! Generic `class_feature` GRANT-fact consumer (SD31 wave 23,
//! `SD31-W23-CLASSFEATURE-001`).
//!
//! # What this module is
//!
//! Waves 20-22 built (and, twice, correctly rejected) attempts at a single
//! goal: read PCGen's own `(granting class, feature key, level)` progression
//! facts and use them to tell a real player, on the real character sheet,
//! which class features their character has. Wave 22 landed a trustworthy
//! parser (`cache_gen::class_feature_grants.rs`, `OPEN-ISSUES.md` row 339,
//! MERGED) that writes those facts to
//! `data/class_feature_grants/<book>/<class-slug>.json`. Nothing consumed
//! that data before this module. This is the consumer.
//!
//! # The mechanism this module plugs into, unchanged
//!
//! `push_pu_class_feature_records` (this file's sibling, in `mod.rs`)
//! already proved the shape: push one `ComputationExplanation` per granted
//! record, id `class_feature.<owner>.corpus_record.<feature_slug>`, value =
//! the granted-at level. `v06_work_inventory.rs`'s `Kind::ClassFeature` verdict
//! arm already has a fully generic matcher for that exact id shape
//! (`class_feature_owner` + `class_feature_exact_suffix_grounded`) that
//! requires NO changes here to recognise a non-`pu.`-namespaced id from any
//! `class_books`-registered class -- confirmed by reading that file, not
//! assumed. This module's ONLY job is to emit the SAME shape of id, for the
//! SAME shape of reason (a real grant fact, gated on level, joined to a real
//! corpus description), for every class this module trusts to emit for.
//!
//! Because the downstream verdict computation is untouched (out of this
//! lane's write scope: `wiring_class.rs` is explicitly off limits, and
//! `v06_work_inventory.rs` is outside `pilot_compute/**`), this module
//! cannot itself decide `done`/`grounded`/`text-complete` -- it only proves
//! the engine HOLDS the record. Decision 7's universal-vs-conditional
//! discriminator, the `has_real_description` check, and the wiring-class
//! gate are ALL still applied downstream, unweakened. A record this module
//! emits for is credited only if every one of those independent checks
//! ALSO agrees -- this module cannot self-grant a false credit even if its
//! own input were wrong, which is why it is safe to lean on that existing
//! machinery rather than re-implement it here.
//!
//! # Three deliberate refusals this module adds on top of the merged parser
//!
//! The parser review (`OPEN-ISSUES.md` row 339) landed with two named,
//! unfixed residuals and this package's own precedent (`OPEN-ISSUES.md` rows
//! 330/338) blocks a third population outright. All three are handled here
//! by REFUSING to emit, never by guessing or by weakening a gate:
//!
//! 1. **Cross-book level conflicts (row 339 residual (b)).** The parser
//!    writes one file per (book, class) with no cross-book reconciliation --
//!    confirmed live, `Druid ~ Wild Shape` ships level 4 from
//!    `core_rulebook` and level 6 from `advanced_players_guide`'s Bear
//!    Shaman archetype row, as two separate files, both shipped. Crediting
//!    either level without resolving the disagreement risks telling a
//!    player their character has a feature they do not yet have (or
//!    withholding one they do). [`resolvable_grants`] groups every loaded
//!    fact by `(class, key)` across ALL books and drops any pair whose
//!    facts disagree on level -- the SAME "refuse rather than guess"
//!    posture `class_feature_grants.rs`'s own module doc comment describes
//!    for its own gate-resolution rules, applied one layer up.
//! 2. **Wizard, Bard, Paladin, Cleric, Sorcerer.** `OPEN-ISSUES.md` rows
//!    330/338 name nine pre-existing, shipped anti-fabrication acceptance
//!    tests guarding Wizard/Bard/Paladin's own `compute_pilot_base_chassis`
//!    output -- five `sd13_bard_level4..8_progression` tests are CLOSED
//!    ALLOWLISTS over the WHOLE `class_feature.bard.` namespace (any new
//!    bard-namespaced id fails them regardless of correctness), and
//!    `sd13_wizard_level1_prepared_spell_baseline`/
//!    `sd13_paladin_level8_progression` each assert an exhaustive property
//!    over every explanation `compute_pilot_base_chassis` returns for a
//!    fixed fixture. Wave 22's own reconciliation attempt for these nine
//!    gates was REJECTED (`OPEN-ISSUES.md` row 338, GAMED) for claiming,
//!    falsely, that they needed no widening -- the ruling question row 330
//!    raised (widen the allowlists by construction, or per-feature) remains
//!    OPEN and unanswered. **This lane found TWO more, previously-
//!    undocumented gates of the identical shape while running the full
//!    suite against successive drafts** (this module's own exclusion list
//!    originally held only the three named classes): both
//!    `sd13_cleric_level1_spell_baseline.rs::
//!    cleric_level1_fabricates_no_spell_math` (tripped at level 1 by
//!    `class_feature.cleric.corpus_record.diminished_spellcasting`) and
//!    `sd13_sorcerer_level1_spell_baseline.rs::
//!    sorcerer_level1_fabricates_no_spell_math` failed the instant this
//!    module emitted a `"spell"`-substring id for that class, the identical
//!    exhaustive-scan shape `sd13_wizard_level1_prepared_spell_baseline`
//!    uses, just never named in `OPEN-ISSUES.md` because nothing had ever
//!    emitted a Cleric- or Sorcerer-namespaced generic id before this lane.
//!    See the wave-23 progress receipt for the full reproduction of both.
//!
//!    **Druid and Monk are excluded for a SEPARATE, THIRD reason, also
//!    named by row 330 but not yet triggered until this lane emitted for
//!    either class.** `sd25_druid_level_up_explanation_filter_audit.rs`
//!    (a standing audit, not a fabrication guard, but equally hard-
//!    blocking) failed live: `is_druid_pillar_id`
//!    (`src/rules_core/level_up/druid.rs`, OUTSIDE this lane's write scope)
//!    is a CLOSED id-prefix allowlist over `LevelUpPlan`'s own explanation
//!    filter, and this module's `class_feature.druid.corpus_record.*` ids
//!    are not in it -- so real, grounded records
//!    (`nature_bond`/`orisons`/`spontaneous_casting`) were silently DROPPED
//!    from every Druid `LevelUpPlan`, the exact shape row 330's own finding
//!    #2 already named for wave 20's rejected lane ("3 of the 19 credited
//!    units are silently dropped... refuting the lane's own
//!    `prose_reaches_player` claim on a real screen it never checked").
//!    `sd25_monk_level_up_explanation_filter_audit.rs` guards the
//!    structurally identical `is_monk_pillar_id` allowlist; Monk is
//!    excluded pre-emptively on the same reasoning rather than waiting for
//!    a live failure to prove it (the audit's own shape -- "every real id
//!    must survive, or fail loudly" -- makes this predictable, not a guess).
//!    Widening `is_druid_pillar_id`/`is_monk_pillar_id` is real, scoped,
//!    owed follow-on work for a lane with write access to
//!    `src/rules_core/level_up/`, not something this module may do itself.
//!
//!    This module does not touch, weaken, or route around any of these
//!    thirteen tests (nine named by rows 330/338, two found live by this
//!    lane, two LevelUpPlan audits); it excludes the seven classes they
//!    cover from its own emission entirely, so none of the thirteen can
//!    regress. `class_feature_grant_consumer_never_emits_for_the_gated_
//!    classes` (below) proves the exclusion directly against the live
//!    merged grant data rather than trusting the list's own claim.
//! 3. **Pathfinder Unchained's four classes.** Already served by
//!    `push_pu_class_feature_records`'s own hand-curated, %N-resolving
//!    roster, in a DIFFERENT id namespace (`class_feature.pu.*`). Emitting
//!    a second, competing id for the same records from this module's
//!    coarser data would be redundant at best; `mod.rs`'s call site checks
//!    `PuClassId::from_class_id_str` before ever reaching this module, so
//!    this file does not need its own copy of that guard, but
//!    [`push_generic_class_feature_grant_records`] still asserts it as a
//!    documented precondition below.
//!
//! # No unresolved `%N` ever ships
//!
//! This module's own `detail` text never quotes the corpus record's raw
//! `DESC:`/`SPROP:` string (which can carry an unresolved `%1`-style
//! argument this module has no per-character context to fill) -- it states
//! only the grant fact itself (class, feature key, granted-at level), a
//! fixed-shape sentence with no template token. The record's REAL rulebook
//! prose reaches the player through the ALREADY-BUILT, book-agnostic
//! `class_feature_descriptions.rs` / `classFeaturesModel.ts` render path
//! (`SD31-D7-PROSE-003`), which applies its own `render_pcgen_desc` +
//! `leaked_pcgen_syntax` leak guard before ever serving a record.
//! [`corpus_records_with_real_description`] reproduces that EXACT guard
//! locally before this module will emit for a record, so this module never
//! promises a record that render surface would itself refuse to serve.
//!
//! # Data flow, both trees read-only
//!
//! `data/class_feature_grants/**` (the merged parser's OUTPUT) and
//! `data/corpus/*/class_feature/**` (the real corpus records) are both read
//! straight off disk, cached for the process lifetime via `OnceLock` --
//! mirroring `class_feature_pool_catalog::load_pool_catalog`'s and
//! `class_feature_descriptions::load_class_feature_descriptions`'s own
//! established pattern in this codebase. Neither tree is written here, and
//! neither the grant parser (`cache_gen/class_feature_grants.rs`) nor
//! `wiring_class.rs` is imported or modified by this module, per this
//! lane's write scope.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde_json::Value;

use super::{ComputationExplanation, pu_feature_slug};

/// The four classes this module refuses to emit for -- see this module's
/// own doc comment, section 2, for the full citation (three named by
/// `OPEN-ISSUES.md` rows 330/338; Cleric and Sorcerer found live by this
/// lane running the full suite; Druid and Monk added for the SEPARATE
/// `is_druid_pillar_id`/`is_monk_pillar_id` LevelUpPlan-filter reason row
/// 330 itself named -- see this module's doc comment, section 2, final
/// paragraph).
const ANTI_FABRICATION_GATE_EXCLUDED_CLASSES: [&str; 7] =
    ["wizard", "bard", "paladin", "cleric", "sorcerer", "druid", "monk"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn walk_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk_json_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

/// Every `(class, key, level)` triple the merged grant data ships, read
/// straight from `data/class_feature_grants/<book>/<class-slug>.json`
/// (`cache_gen::class_feature_grants.rs`'s own output shape). `class` and
/// `key` are the module's own resolved fields, never re-derived here.
struct RawGrantFact {
    key: String,
    class: String,
    level: u8,
}

/// Reproduced from `v06_work_inventory.rs`'s own `CLASS_FEATURE_POOLS`
/// registry (this package's established "disjoint-file-touch" convention --
/// `class_feature_descriptions.rs`'s `slug`/`is_real_description_value` and
/// `class_feature_pool_catalog.rs`'s own doc comment both name the same
/// pattern rather than importing across a lane boundary): the first column
/// of every registered player-facing OPTION POOL a class offers -- Rogue
/// Talent, Alchemist Discovery, Witch Hex, Oracle Revelation/Mystery/Curse,
/// Ranger Favored Enemy/Terrain, and so on.
///
/// **Why this module refuses every one of these (found live, not
/// anticipated by the original design -- `sd13_rogue_level10_progression.rs
/// ::rogue_level10_does_not_fabricate_talent_records` failed the instant
/// this module emitted `class_feature.rogue.corpus_record.new_talents`,
/// sourced from `"Eldritch Raider ~ New Talents"`, a real, non-colliding,
/// automatically-granted archetype record).** A pool's own "you get to pick
/// from this list" record (`"Rogue ~ Rogue Talents"`, and every
/// archetype's own renamed equivalent -- `"Eldritch Raider ~ New Talents"`,
/// `"Snoop ~ Investigator Talents"`) states that the character receives a
/// CHOICE SLOT, never a specific effect: reaching the level does not tell a
/// player which talent/discovery/hex/mystery they have, so a flat
/// `granted from class level N` explanation for one of these would be
/// exactly the "claims a specific outcome the character has not chosen"
/// shape `class_feature_pool_catalog.rs`'s own module doc comment spends
/// its length distinguishing from a genuine automatic grant. Wave 22's own
/// `class_feature_pool_catalog` (`OPEN-ISSUES.md` row 340) already built the
/// correct, SEPARATE mechanism for exactly this shape (a browsable
/// reference catalog, not a per-character grant claim) for `Rogue Talent`
/// specifically; this module must never compete with or duplicate that by
/// emitting a grant-shaped id for the same population.
///
/// Applied as a SUBSTRING match against the grant fact's own key
/// (case-insensitive) rather than an exact tail match: PCGen's own
/// archetype-renamed pool records (`"New Talents"`, `"Investigator
/// Talents"`, `"Advanced Talents"`) do not share exact text with the base
/// pool name, but every one of them still contains the pool's own noun.
/// Deliberately broad rather than narrowly reactive to only the one test
/// that failed live: `Discovery`/`Hex`/`Judgment`/... share the identical
/// "open-ended pick, not a fixed effect" shape and this module has no
/// reason to trust its own emission for them any more than it trusted
/// `Rogue Talent` before this was found.
const OPEN_ENDED_CHOICE_POOL_KEYWORDS: [&str; 24] = [
    "rage power",
    "discovery",
    "discoveries",
    "talent",
    "hex",
    "revelation",
    "mercy",
    "mercies",
    "judgment",
    "inquisition",
    "blessing",
    "evolution",
    "bloodline",
    "domain",
    "order",
    "mystery",
    "curse",
    "spirit",
    "favored enemy",
    "favored terrain",
    "animal focus",
    "versatile performance",
    "arcane school",
    // Not a `CLASS_FEATURE_POOLS` entry (it is a resource POOL, not a
    // player CHOICE pool), but the identical "a dedicated mechanism
    // already tracks this magnitude, a flat roster fact is both redundant
    // and collision-prone" shape. Found live: `sd20_levelup_monk.rs::
    // monk_level_3_to_4_grants_ki_pool_resource_change_and_slow_fall_
    // keeps_still_mind_unchanged` asserts `"Monk ~ Ki Pool"` (a real,
    // base-class, non-archetype key that survives every OTHER guard in
    // this module) must appear ONLY in `LevelUpPlan.resource_pool_change`,
    // never in `automatic_features` -- this module's flat granted-at-level
    // roster fact has no resource-pool-vs-discrete-grant distinction and
    // would always land in the latter. Also independently correct under
    // Decision 7 REFINED (`decisions.md`): Ki Pool's size is a UNIVERSAL,
    // level-scaling magnitude a real mechanism must compute, not a
    // flat fact this module's own text-only roster claim was ever the
    // right tool for.
    "ki pool",
];

fn key_names_an_open_ended_choice_pool(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    OPEN_ENDED_CHOICE_POOL_KEYWORDS.iter().any(|keyword| lower.contains(keyword))
}

/// Whether `key`'s own leading `" ~ "` group segment is literally the
/// resolved granting `class`, case-insensitively -- i.e. whether this is a
/// BASE class feature (`"Fighter ~ Bravery"`, class `Fighter`) rather than
/// an ARCHETYPE's own replacement feature riding under the base class's
/// resolved name (`"Airborne Ambusher ~ Combat Flyer"`, class `Fighter`).
///
/// **Why this module refuses every non-matching fact (found live, the
/// single most consequential correctness bug this lane found, not
/// anticipated by the original design).** `PRECLASS:1,Fighter=2` on an
/// archetype's own progression row correctly names FIGHTER as the base
/// class the archetype belongs to -- the parser's resolution is right, and
/// `resolvable_grants`/`unambiguous_grants` correctly treat it as a real,
/// unconflicted, non-colliding Fighter-scoped fact. But "the granting class
/// is Fighter" is not the same claim as "every Fighter has this feature":
/// this engine has NO archetype-selection model anywhere (confirmed by
/// direct read of `CharacterInput` and every `compute_*_chassis` function
/// this module's caller dispatches through), so a character choosing the
/// Airborne Ambusher archetype cannot be distinguished from a vanilla
/// Fighter. Without this guard, `sd20_level_up_parity.rs::
/// level_up_fighter_1_to_2_parity_fixture_round_trips_through_the_real_
/// engine` failed with the proof: a level-1-to-2 Fighter LevelUpPlan
/// preview for a PLAIN, no-archetype fixture carried 24 EXTRA grants --
/// Combat Flyer, Deflective Shield, Close Control, Dirty Maneuvers, and 20
/// more, one from each of 24 DIFFERENT, mutually-exclusive Fighter
/// archetypes, ALL claimed simultaneously for a single vanilla character.
/// That is exactly the "claims a specific outcome the character does not
/// have" shape this whole module exists to avoid, discovered one level
/// past the `Rogue Talent` pool-choice hazard rather than a mere test
/// artifact -- if this module had continued to ship it, an actual player's
/// character sheet would show every one of those 24 archetype abilities on
/// a Fighter who chose none of them.
///
/// Restricting to same-group facts is a real, understood cost: of the
/// ~2,200 pool-filtered, allowed-class facts this module's raw data
/// resolves, only a small base-class-only slice survives this guard (most
/// of the corpus's `class_feature_grants` population is archetype
/// replacement content, exactly as `OPEN-ISSUES.md` row 339's own residual
/// (b) already flagged for the narrower Wild Shape case). That is the
/// correct, defensible size for what this engine can currently prove --
/// archetype selection is real, owed future work, not something this
/// module may guess around.
fn key_names_a_base_class_feature(key: &str, class: &str) -> bool {
    let Some(group) = key.split(" ~ ").next() else { return false };
    group.trim().eq_ignore_ascii_case(class.trim())
}

fn load_raw_grant_facts() -> Vec<RawGrantFact> {
    let grants_root = repo_root().join("data/class_feature_grants");
    let mut out = Vec::new();
    let Ok(books) = std::fs::read_dir(&grants_root) else { return out };
    let mut book_dirs: Vec<_> = books.flatten().collect();
    book_dirs.sort_by_key(|e| e.file_name());
    for book_entry in book_dirs {
        let book_dir = book_entry.path();
        if !book_dir.is_dir() {
            continue;
        }
        let mut files = Vec::new();
        walk_json_files(&book_dir, &mut files);
        for file in files {
            let Ok(text) = std::fs::read_to_string(&file) else { continue };
            let Ok(rows) = serde_json::from_str::<Vec<Value>>(&text) else { continue };
            for row in rows {
                let (Some(key), Some(class), Some(level)) =
                    (row["key"].as_str(), row["class"].as_str(), row["level"].as_u64())
                else {
                    continue;
                };
                let Ok(level) = u8::try_from(level) else { continue };
                if key_names_an_open_ended_choice_pool(key) {
                    continue;
                }
                if !key_names_a_base_class_feature(key, class) {
                    continue;
                }
                // CRITICAL fabrication defect this module's own wave-23
                // integration cycle found and fixed upstream (see
                // `cache_gen::class_feature_grants::GrantFact::granted_via_
                // archetype`'s doc comment): a key whose group text equals
                // `class` (passing `key_names_a_base_class_feature` above)
                // can STILL be an archetype-only replacement feature --
                // `"Rogue ~ Careful Disarm"`/`"Rogue ~ Poison Use"`, both
                // PRECLASS-gated on an archetype's OWN `CATEGORY:Archetype`
                // definition row (`advanced_players_guide:2942`/`:2945`).
                // `granted_via_archetype` is the authoritative, upstream-
                // derived signal; missing/non-boolean is treated as
                // archetype-sourced (refuse), never as safe by default --
                // the conservative direction for a field this module
                // cannot independently re-derive from text.
                if row["granted_via_archetype"].as_bool().unwrap_or(true) {
                    continue;
                }
                out.push(RawGrantFact { key: key.to_string(), class: class.to_string(), level });
            }
        }
    }
    out
}

/// `(class.to_lowercase(), key)` -> the granted-at level, for every grant
/// fact that resolves WITHOUT a cross-book disagreement. See this module's
/// doc comment, section 1, for why disagreeing pairs are dropped rather than
/// resolved by picking one side.
fn resolvable_grants() -> &'static BTreeMap<(String, String), u8> {
    static TABLE: OnceLock<BTreeMap<(String, String), u8>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut levels_seen: BTreeMap<(String, String), BTreeMap<u8, ()>> = BTreeMap::new();
        for fact in load_raw_grant_facts() {
            let pair = (fact.class.to_lowercase(), fact.key);
            levels_seen.entry(pair).or_default().insert(fact.level, ());
        }
        levels_seen
            .into_iter()
            .filter_map(|(pair, levels)| {
                if levels.len() == 1 {
                    levels.into_keys().next().map(|level| (pair, level))
                } else {
                    // Cross-book conflict: refuse the whole pair rather than
                    // guess which book wins.
                    None
                }
            })
            .collect()
    })
}

/// `(class.to_lowercase(), key)` -> the granted-at level, a second,
/// STRICTER filter on top of [`resolvable_grants`]: drops every entry whose
/// `(class, feature_slug)` -- the receipt-id's own trailing segment, via
/// [`pu_feature_slug`] on `key` -- is shared with a DIFFERENT key for the
/// same class.
///
/// **Why this exists (found live by this lane's own full-suite run, not
/// anticipated by the module's original design).** Many archetype books
/// re-declare a base class's generic-named feature (`Skills`, `Class
/// Skills`, `Weapon and Armor Proficiency`, and -- the one that surfaced
/// this -- `Wild Shape`) under a DIFFERENT, archetype-qualified corpus key
/// (`"Aerie Protector ~ Wild Shape"`, `"Bear Shaman ~ Wild Shape"`, ...) as
/// the archetype's own replacement feature. [`pu_feature_slug`] keeps only
/// the text AFTER the key's last `" ~ "`, so every one of these DISTINCT
/// corpus records slugs down to the IDENTICAL `wild_shape` tail --
/// `resolvable_grants` sees them as unrelated (class, key) pairs (no
/// per-key cross-book disagreement) and resolves each independently, but
/// this module can only ever emit ONE id per slug
/// (`class_feature.<owner>.corpus_record.<slug>`), so every one of those
/// archetype-specific facts would silently satisfy the SAME id --
/// including, disastrously, the UNRELATED base `"Druid ~ Wild Shape"`
/// corpus unit's own suffix match in `v06_work_inventory.rs`
/// (`class_feature_exact_suffix_grounded` matches on the id's trailing dot-
/// segment alone, with no way to see which specific key this module meant).
/// Confirmed live: `sd13_druid_level10_progression.rs::
/// druid_level10_does_not_fabricate_wild_shape_execution` failed the
/// instant this module emitted a `wild_shape`-slugged id sourced from
/// `"Aerie Protector ~ Wild Shape"` (level 6) -- the SAME id that would also
/// have silently grounded the plain `"Druid ~ Wild Shape"` unit, which this
/// module's OWN cross-book-conflict guard had separately, correctly
/// excluded (level 4 vs level 6 disagreement across books) precisely
/// because it is genuinely unresolved. This second filter closes that leak
/// the same way: refuse the WHOLE slug group rather than pick a winner or
/// guess that two differently-keyed records are "close enough."
/// `a_slug_shared_by_two_distinct_keys_for_the_same_class_emits_neither`
/// (below) reproduces the exact live collision and proves this filter
/// closes it.
fn unambiguous_grants() -> &'static BTreeMap<(String, String), u8> {
    static TABLE: OnceLock<BTreeMap<(String, String), u8>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut keys_by_slug: BTreeMap<(String, String), BTreeMap<String, ()>> = BTreeMap::new();
        for (class, key) in resolvable_grants().keys() {
            let slug = pu_feature_slug(key);
            keys_by_slug.entry((class.clone(), slug)).or_default().insert(key.clone(), ());
        }
        let ambiguous_pairs: std::collections::BTreeSet<(String, String)> = keys_by_slug
            .into_iter()
            .filter(|(_, keys)| keys.len() > 1)
            .flat_map(|((class, _), keys)| keys.into_keys().map(move |key| (class.clone(), key)))
            .collect();
        resolvable_grants()
            .iter()
            .filter(|(pair, _)| !ambiguous_pairs.contains(*pair))
            .map(|(pair, &level)| (pair.clone(), level))
            .collect()
    })
}

fn is_real_description_value(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !matches!(lower.as_str(), ".clear" | ".clearall" | "[redacted pi]")
}

/// Corpus `KEY:` -> the record's own `data.name`, for every
/// `data/corpus/*/class_feature/**/*.json` record this module confirms
/// carries a real, renderable, non-leaking description -- the exact guard
/// `class_feature_descriptions.rs`'s own catalog applies before serving a
/// record to a player (`render_pcgen_desc` + `leaked_pcgen_syntax`),
/// reproduced here so this module never claims a record that render surface
/// would itself refuse.
fn corpus_records_with_real_description() -> &'static BTreeMap<String, String> {
    static TABLE: OnceLock<BTreeMap<String, String>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut out = BTreeMap::new();
        let corpus_root = repo_root().join("data/corpus");
        let Ok(books) = std::fs::read_dir(&corpus_root) else { return out };
        let mut book_dirs: Vec<_> = books.flatten().collect();
        book_dirs.sort_by_key(|e| e.file_name());
        for book_entry in book_dirs {
            let cf_dir = book_entry.path().join("class_feature");
            if !cf_dir.is_dir() {
                continue;
            }
            let mut files = Vec::new();
            walk_json_files(&cf_dir, &mut files);
            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                let data = &doc["data"];
                let (Some(key), Some(name)) = (data["key"].as_str(), data["name"].as_str())
                else {
                    continue;
                };
                let Some(raw_desc) = data["description"].as_str() else { continue };
                if !is_real_description_value(raw_desc) {
                    continue;
                }
                let rendered = crate::rules_core::pcgen_desc::render_pcgen_desc(raw_desc);
                if crate::rules_core::pcgen_desc::leaked_pcgen_syntax(&rendered.text).is_some() {
                    continue;
                }
                // Gate-weakening review finding (SD-31 wave 23 integration
                // cycle): `leaked_pcgen_syntax` alone does not catch an
                // unresolved `%N` numeric-argument placeholder silently
                // DROPPED (not leaked as literal syntax) by
                // `render_pcgen_desc` -- e.g. Fighter ~ Bravery's real DESC
                // reads "You gain a +%1 bonus to Will saves against fear
                // effects.", which renders as "You gain a + bonus..." with
                // no `%` character left to catch. `class_feature_pool_
                // catalog.rs`'s sibling gate already refuses on this same
                // signal (`!rendered.dropped_args.is_empty()`); mirrored
                // here so this module never claims a record whose
                // magnitude that render pass could not resolve.
                if !rendered.dropped_args.is_empty() {
                    continue;
                }
                out.entry(key.to_string()).or_insert_with(|| name.to_string());
            }
        }
        out
    })
}

/// Pushes one `ComputationExplanation` (id
/// `class_feature.<owner>.corpus_record.<feature_slug>`, same shape and
/// convention as `push_pu_class_feature_records`) for every merged grant
/// fact that names `owner` as its granting class, is cross-book-conflict-
/// free, is granted at or below `level`, and joins to a real, renderable
/// corpus description.
///
/// Precondition (documented, not re-checked here -- the caller,
/// `compute_class_chassis`, already applies it): `class_id_str` must not
/// resolve via `PuClassId::from_class_id_str`. Pathfinder Unchained classes
/// are served by `push_pu_class_feature_records` alone.
pub(super) fn push_generic_class_feature_grant_records(
    class_id_str: &str,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(owner) = class_id_str.strip_prefix("class:") else { return };
    if ANTI_FABRICATION_GATE_EXCLUDED_CLASSES.contains(&owner) {
        return;
    }
    // Snapshotted BEFORE this function pushes anything, so this module never
    // sees its own prior pushes as a "real" collision. Every explanation id
    // this class's OWN hand-wired chassis/feature code already pushed this
    // call (`compute_fighter_chassis`, `compute_generic_table_chassis`, ...
    // -- all run before `compute_class_chassis` reaches this call site) is a
    // REAL, per-character-computed magnitude this module must defer to
    // rather than shadow.
    //
    // **Why this guard exists (found live, not anticipated).**
    // `sd20_contract_level_up_preview.rs::
    // compute_level_up_preview_carries_real_fighter_level_2_grants` failed:
    // Fighter's own hand-wired `class_feature.fighter.bravery` explanation
    // (the REAL +1 morale-bonus-vs-fear magnitude) and this module's
    // `class_feature.fighter.corpus_record.bravery` roster id (the flat
    // granted-at-level fact) share the IDENTICAL trailing dot-segment,
    // `"bravery"` -- `v06_work_inventory.rs`'s own `class_feature_exact_
    // suffix_grounded` is deliberately insensitive to this (it only needs
    // ONE match, from either), but `compute_level_up_preview`'s grant
    // lookup does a plain name-substring `.find()` with no such tolerance,
    // and picked this module's flat roster fact (`value: 2`, the granted-at
    // level) over the real morale bonus (`value: 1`), corrupting a
    // downstream, previously-passing contract surface. Refusing to emit
    // whenever a real trailing-segment match already exists closes this
    // the same way `unambiguous_grants` closes the same-shaped Wild Shape
    // collision one layer up: refuse rather than risk shadowing a REAL
    // computed magnitude with this module's coarser flat fact.
    let already_computed_slugs: std::collections::BTreeSet<String> = explanations
        .iter()
        .map(|e| e.id.rsplit('.').next().unwrap_or("").to_owned())
        .collect();
    let descriptions = corpus_records_with_real_description();
    for ((class, key), &granted_at) in unambiguous_grants() {
        if class != owner {
            continue;
        }
        if level < granted_at {
            continue;
        }
        let Some(name) = descriptions.get(key) else { continue };
        let feature_slug = pu_feature_slug(key);
        if feature_slug.is_empty() {
            continue;
        }
        if already_computed_slugs.contains(feature_slug.as_str()) {
            continue;
        }
        explanations.push(ComputationExplanation {
            id: format!("class_feature.{owner}.corpus_record.{feature_slug}"),
            value: i16::from(granted_at),
            detail: format!(
                "{owner} level {level}: `{key}` (\"{name}\") is a class feature of this \
                 character, granted from class level {granted_at}, per a grant fact ingested \
                 from PCGen's own class-progression tokens (data/class_feature_grants). The \
                 record's real rulebook description is served separately by the character \
                 sheet's Class Features section."
            ),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolvable_grants_is_non_empty_against_the_live_merged_data() {
        let grants = resolvable_grants();
        assert!(
            !grants.is_empty(),
            "the merged data/class_feature_grants tree must resolve at least one \
             cross-book-conflict-free fact"
        );
    }

    #[test]
    fn key_names_a_base_class_feature_distinguishes_base_from_archetype() {
        assert!(key_names_a_base_class_feature("Fighter ~ Bravery", "Fighter"));
        assert!(!key_names_a_base_class_feature("Airborne Ambusher ~ Combat Flyer", "Fighter"));
        assert!(!key_names_a_base_class_feature("Gladiator ~ Fame", "Fighter"));
    }

    #[test]
    fn archetype_replacement_facts_never_reach_load_raw_grant_facts() {
        // Reproduces the live, most-consequential fabrication risk this
        // module found: `"Airborne Ambusher ~ Combat Flyer"` resolves as a
        // real, unconflicted, class=Fighter grant fact (advanced_class_guide)
        // -- but Airborne Ambusher is a Fighter ARCHETYPE this engine has no
        // selection model for, so claiming it for every Fighter would be
        // exactly the "claims a specific outcome the character does not
        // have" fabrication this module exists to refuse.
        // `sd20_level_up_parity.rs::
        // level_up_fighter_1_to_2_parity_fixture_round_trips_through_the_
        // real_engine` failed live before this filter existed, with 24 such
        // archetype-replacement facts all claimed simultaneously for one
        // vanilla Fighter fixture.
        let raw = load_raw_grant_facts();
        assert!(
            raw.iter().all(|f| f.key != "Airborne Ambusher ~ Combat Flyer"),
            "an archetype-replacement key must never survive load_raw_grant_facts"
        );
        // Prove the guard is not vacuous: at least one base-class Fighter
        // fact (same group as the class) DOES survive.
        assert!(
            raw.iter().any(|f| f.class.eq_ignore_ascii_case("fighter") && f.key.starts_with("Fighter ~ ")),
            "expected at least one real base-class Fighter fact to survive the guard"
        );
    }

    #[test]
    fn open_ended_choice_pool_keys_never_reach_resolvable_grants() {
        // `"Rogue ~ Rogue Talents"` (core_rulebook, level 2) and
        // `"Eldritch Raider ~ New Talents"` (advanced_race_guide, level 1)
        // are both live, real grant facts this module's own raw load sees --
        // proving the keyword filter has real input to reject, not an empty
        // set that would let this test pass vacuously.
        let raw = load_raw_grant_facts();
        assert!(
            raw.iter().all(|f| !f.key.to_lowercase().contains("rogue talents")),
            "load_raw_grant_facts must never carry an open-ended-choice-pool key"
        );
        let grants = resolvable_grants();
        for (class, key) in grants.keys() {
            assert!(
                !key_names_an_open_ended_choice_pool(key),
                "{class}/{key:?} names an open-ended choice pool and must never resolve"
            );
        }
    }

    #[test]
    fn key_names_an_open_ended_choice_pool_catches_the_live_rogue_talent_shapes() {
        assert!(key_names_an_open_ended_choice_pool("Rogue ~ Rogue Talents"));
        assert!(key_names_an_open_ended_choice_pool("Eldritch Raider ~ New Talents"));
        assert!(key_names_an_open_ended_choice_pool("Snoop ~ Investigator Talents"));
        assert!(key_names_an_open_ended_choice_pool("Alchemist ~ Discovery"));
        assert!(!key_names_an_open_ended_choice_pool("Fighter ~ Bravery"));
    }

    #[test]
    fn cross_book_conflicting_pairs_are_dropped_not_guessed() {
        // `("druid", "Druid ~ Wild Shape")` is the live, reproducible case:
        // `core_rulebook` grants it at level 4, while `advanced_players_guide`
        // /`ultimate_magic`/`ultimate_combat`/`ultimate_wilderness`/
        // `ultimate_intrigue` each independently re-declare the SAME literal
        // key at level 6 (OPEN-ISSUES.md row 339's own named residual).
        // Whichever pair(s) the live data disagrees on, this module must
        // refuse every one of them, not resolve to either side.
        let grants = resolvable_grants();
        let raw = load_raw_grant_facts();
        let mut by_pair: BTreeMap<(String, String), BTreeMap<u8, ()>> = BTreeMap::new();
        for fact in raw {
            by_pair.entry((fact.class.to_lowercase(), fact.key)).or_default().insert(fact.level, ());
        }
        let conflicting: Vec<_> =
            by_pair.iter().filter(|(_, levels)| levels.len() > 1).map(|(pair, _)| pair).collect();
        assert!(!conflicting.is_empty(), "expected at least one live cross-book conflict to prove this test can fail");
        for pair in conflicting {
            assert!(
                !grants.contains_key(pair),
                "{pair:?} has disagreeing cross-book levels and must not resolve"
            );
        }
    }

    #[test]
    fn a_slug_shared_by_two_distinct_keys_for_the_same_class_emits_neither() {
        // Originally reproduced a LIVE collision (many archetype books
        // re-declaring a base feature's generic tail -- `Wild Shape`,
        // `Skills`, `Weapon and Armor Proficiency` -- under a DIFFERENT,
        // archetype-qualified key that slugs down to the SAME id segment
        // via `pu_feature_slug`). `key_names_a_base_class_feature`
        // (added AFTER this guard, for the more severe archetype-
        // fabrication finding `sd20_level_up_parity.rs` caught) already
        // excludes every archetype-qualified key from
        // `load_raw_grant_facts` before this guard ever runs, which
        // structurally eliminates the SAME population that used to produce
        // slug collisions here -- confirmed empirically: this test no
        // longer finds a live example, and that is the base-class filter
        // working, not this guard going stale. The invariant this guard
        // still enforces is checked directly below, and the guard itself
        // stays -- a future book could still introduce two DIFFERENTLY-
        // NAMED base-class features that happen to slug identically (no
        // archetype qualifier required), and this is the only thing that
        // would catch it.
        let resolvable = resolvable_grants();
        let mut by_slug: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
        for (class, key) in resolvable.keys() {
            by_slug.entry((class.clone(), pu_feature_slug(key))).or_default().push(key.clone());
        }
        let colliding: Vec<_> = by_slug.into_iter().filter(|(_, keys)| keys.len() > 1).collect();
        let unambiguous = unambiguous_grants();
        for ((class, slug), keys) in colliding {
            for key in keys {
                assert!(
                    !unambiguous.contains_key(&(class.clone(), key.clone())),
                    "{class}/{slug} is shared by multiple distinct keys and {key:?} must not \
                     resolve unambiguously"
                );
            }
        }
        // A synthetic, mutation-capable proof that `unambiguous_grants`'s
        // OWN invariant holds structurally: no two DISTINCT keys survive
        // under the same (class, slug) pair.
        let mut seen_slugs: BTreeMap<(String, String), &String> = BTreeMap::new();
        for (class, key) in unambiguous.keys() {
            let slug = pu_feature_slug(key);
            if let Some(other) = seen_slugs.insert((class.clone(), slug.clone()), key) {
                panic!(
                    "unambiguous_grants must never carry two distinct keys for the same \
                     (class, slug): {class}/{slug} has both {other:?} and {key:?}"
                );
            }
        }
    }

    #[test]
    fn class_feature_grant_consumer_never_emits_for_the_gated_classes() {
        for gated in ANTI_FABRICATION_GATE_EXCLUDED_CLASSES {
            let mut explanations = Vec::new();
            push_generic_class_feature_grant_records(
                &format!("class:{gated}"),
                20,
                &mut explanations,
            );
            assert!(
                explanations.is_empty(),
                "{gated} is named by OPEN-ISSUES.md rows 330/338's anti-fabrication gates and \
                 must never receive a generic roster explanation: got {explanations:?}"
            );
        }
    }

    #[test]
    fn a_granted_record_below_its_level_emits_nothing() {
        let mut explanations = Vec::new();
        // Level 0 cannot meet any real grant's `granted_at` (PCGen's own
        // minimum class level is 1), so this must always be empty for any
        // class this module serves.
        push_generic_class_feature_grant_records("class:fighter", 0, &mut explanations);
        assert!(explanations.is_empty());
    }

    #[test]
    fn every_emitted_id_matches_the_owner_and_carries_a_real_positive_level() {
        // A real, permissive smoke test over a class this module DOES serve:
        // every id this function emits for Fighter at a high level must be
        // namespaced under `class_feature.fighter.corpus_record.` and carry
        // a positive granted-at value -- the same shape
        // `v06_work_inventory.rs`'s `class_feature_exact_suffix_grounded`
        // requires to ever credit it.
        let mut explanations = Vec::new();
        push_generic_class_feature_grant_records("class:fighter", 20, &mut explanations);
        for explanation in &explanations {
            assert!(
                explanation.id.starts_with("class_feature.fighter.corpus_record."),
                "unexpected id shape: {}",
                explanation.id
            );
            assert!(explanation.value >= 1, "granted_at must be a real class level: {explanation:?}");
            assert!(
                !explanation.detail.contains('%'),
                "no unresolved PCGen %N numeric argument may ship in an explanation's detail: {}",
                explanation.detail
            );
        }
    }

    /// The exact CRITICAL fabrication defect the wave-23 integration
    /// review found live: a vanilla, no-archetype Rogue must never receive
    /// `careful_disarm`/`poison_use` (Burglar/Poisoner/Trapsmith/Spy
    /// archetype-only replacement features, both PRECLASS-gated under the
    /// base `Rogue` class's own name). Mutating `granted_via_archetype`'s
    /// refusal in `load_raw_grant_facts` to a no-op turns this red.
    #[test]
    fn a_vanilla_rogue_never_receives_an_archetype_only_replacement_feature() {
        let mut explanations = Vec::new();
        push_generic_class_feature_grant_records("class:rogue", 20, &mut explanations);
        let ids: Vec<&str> = explanations.iter().map(|e| e.id.as_str()).collect();
        assert!(
            !ids.iter().any(|id| id.ends_with(".careful_disarm") || id.ends_with(".poison_use")),
            "a vanilla Rogue must never be told it has an archetype-only replacement feature: {ids:?}"
        );
    }

    #[test]
    fn a_pre_existing_real_explanation_suppresses_the_matching_roster_id() {
        // Reproduces the live collision this guard closes:
        // `class_feature.fighter.bravery` (Fighter's own real, hand-wired
        // morale-bonus explanation, pushed by `compute_fighter_chassis`
        // before this module ever runs) shares its trailing dot-segment
        // with this module's own `class_feature.fighter.corpus_record.
        // bravery` roster id. Without this guard,
        // `sd20_contract_level_up_preview.rs::
        // compute_level_up_preview_carries_real_fighter_level_2_grants`
        // fails live (confirmed: this test was added after that exact
        // failure, not written speculatively).
        let mut explanations = vec![ComputationExplanation {
            id: "class_feature.fighter.bravery".to_owned(),
            value: 1,
            detail: "the real, hand-wired Bravery morale bonus".to_owned(),
        }];
        push_generic_class_feature_grant_records("class:fighter", 20, &mut explanations);
        assert_eq!(
            explanations.iter().filter(|e| e.id.rsplit('.').next() == Some("bravery")).count(),
            1,
            "exactly the pre-seeded real explanation must survive; this module must not add a \
             second, colliding id for the same trailing segment: {explanations:?}"
        );
        assert_eq!(explanations[0].value, 1, "the real explanation must be untouched");
    }

    #[test]
    fn pathfinder_unchained_classes_are_never_asked_of_this_module_by_the_real_caller() {
        // Documents the caller-side precondition this module's own doc
        // comment states rather than re-checking here: `compute_class_chassis`
        // dispatches Unchained classes to `compute_pu_class_chassis` (a
        // different branch of the same `if`/`else if` chain) and never
        // reaches this function for them. This module itself has no
        // `PuClassId` guard, so prove the shape the caller relies on stays
        // true directly: a PU class id string, if this function were ever
        // called with one by mistake, resolves no `owner` this module's own
        // exclusion list or grant data recognizes as itself (`strip_prefix`
        // still succeeds, but no grant fact's `class` field is ever
        // literally "unchained_barbarian" -- PU's own data lives in
        // `rules_tables::pathfinder_unchained`, never in
        // `data/class_feature_grants`), so no id collision with
        // `push_pu_class_feature_records`'s `class_feature.pu.*` namespace
        // is possible even in that scenario.
        let mut explanations = Vec::new();
        push_generic_class_feature_grant_records(
            "class:unchained_barbarian",
            20,
            &mut explanations,
        );
        // Gate-weakening review finding (SD-31 wave 23 integration cycle):
        // a bare `for` loop over a vec this test never asserts is non-empty
        // passes vacuously under any mutation. The real claim is stronger
        // and directly checkable: this function must emit NOTHING at all
        // for a PU class id, because no grant fact's `class` field is ever
        // literally "unchained_barbarian" (PU's own data lives in
        // `rules_tables::pathfinder_unchained`, never in
        // `data/class_feature_grants`).
        assert!(
            explanations.is_empty(),
            "no grant fact should ever resolve for a Pathfinder Unchained class id, so this              function must emit nothing when called with one (even though the real caller never              does): got {explanations:?}"
        );
    }

    /// Gate-weakening review finding: the original smoke test
    /// (`every_emitted_id_matches_the_owner_and_carries_a_real_positive_level`,
    /// below) probed only Fighter and never asserted its own output was
    /// non-empty, so it would pass unchanged even if the whole emission
    /// path silently became a no-op. This iterates every class the live
    /// merged data resolves at least one fact for and requires each to
    /// emit something real.
    #[test]
    fn every_resolving_class_emits_at_least_one_real_explanation_at_level_20() {
        let classes: std::collections::BTreeSet<String> =
            unambiguous_grants().keys().map(|(class, _)| class.clone()).collect();
        assert!(!classes.is_empty(), "expected the live merged data to resolve at least one class");
        let mut any_emitted = false;
        for class in &classes {
            if ANTI_FABRICATION_GATE_EXCLUDED_CLASSES.contains(&class.as_str()) {
                continue;
            }
            let mut explanations = Vec::new();
            push_generic_class_feature_grant_records(&format!("class:{class}"), 20, &mut explanations);
            for explanation in &explanations {
                any_emitted = true;
                assert!(
                    explanation.id.starts_with(&format!("class_feature.{class}.corpus_record.")),
                    "unexpected id shape for {class}: {}",
                    explanation.id
                );
                assert!(explanation.value >= 1, "granted_at must be a real class level: {explanation:?}");
                assert!(
                    !explanation.detail.contains('%'),
                    "no unresolved PCGen %N numeric argument may ship in an explanation's detail: {}",
                    explanation.detail
                );
            }
        }
        assert!(
            any_emitted,
            "expected at least one non-excluded class to emit at least one real explanation at              level 20 against the live merged data -- an empty result here would mean the whole              emission path silently became a no-op"
        );
    }
}
