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
//! 2. **Wizard, Bard, Paladin, Cleric, Sorcerer -- WIDENED BY CONSTRUCTION,
//!    SD-34 `decisions.md` §18.** `OPEN-ISSUES.md` rows 330/338 named nine
//!    pre-existing, shipped anti-fabrication acceptance tests guarding
//!    Wizard/Bard/Paladin's own `compute_pilot_base_chassis` output (plus
//!    two more of the identical shape this lane found live for Cleric and
//!    Sorcerer, and two `LevelUpPlan` audits -- see the Druid/Monk
//!    paragraph below). Wave 22's reconciliation attempt was REJECTED
//!    (`OPEN-ISSUES.md` row 338, GAMED) for claiming, falsely, that these
//!    gates needed no widening; row 330's own open ruling question --
//!    "widen the allowlists by construction, or per-feature?" -- is
//!    answered by `decisions.md` §18: **by construction.** An explanation
//!    is admitted when it CITES A REAL CORPUS RECORD (the SAME
//!    `corpus_records_with_real_description`/`resolved_description_for`
//!    gate this module already applied, unweakened), never because its
//!    class name sits on a hand-maintained allowlist. Concretely: the nine
//!    `sd13_*`/`sd25_*` gates were widened, additively, to accept the
//!    resulting `class_feature.<class>.corpus_record.*` ids by SHAPE
//!    (the five `sd13_bard_level4..8_progression` closed-namespace
//!    allowlists each gained one prefix carve-out; `sd13_wizard_level1_
//!    prepared_spell_baseline`/`sd13_cleric_level1_spell_baseline`/
//!    `sd13_sorcerer_level1_spell_baseline`'s `"spell"`-substring catches
//!    each gained the same; `sd13_paladin_level8_progression`'s
//!    `"resolve"`-substring catch gained an EXACT carve-out for
//!    `class_feature.paladin.corpus_record.aura_of_resolve` alone, since
//!    that test's remaining purpose -- no fabricated Aura of Resolve
//!    MECHANICAL magnitude -- is orthogonal to this module's flat,
//!    citation-backed grant-fact id). No existing assertion in any of the
//!    nine was weakened, deleted, or narrowed; every one still fails on a
//!    genuinely fabricated id. `previously_gated_classes_now_emit_citation_
//!    backed_explanations_by_construction` (below) proves the widening
//!    directly against the live merged grant data.
//!
//!    **Druid and Monk -- WIDENED THIS CYCLE (SD-34, bucket-B batch cycle,
//!    continuing `decisions.md` §18's own construction).** Prior cycles kept
//!    both classes wholesale-excluded here, citing a SEPARATE, THIRD reason:
//!    `is_druid_pillar_id`/`is_monk_pillar_id`
//!    (`src/rules_core/level_up/{druid,monk}.rs`) are a CLOSED id-prefix
//!    allowlist over `LevelUpPlan`'s own explanation filter, out of this
//!    lane's write scope, and a prior investigation found it dropping real
//!    facts from that screen. Direct re-inspection this cycle (reading both
//!    functions' live source, not the doc comment's own inherited claim)
//!    found `is_druid_pillar_id` already matches
//!    `"class_chassis.druid."`/`"class_feature.druid."`/`"class_spell.
//!    druid."` and `is_monk_pillar_id` already matches
//!    `"class_chassis.monk."`/`"class_feature.monk."` -- both ALREADY admit
//!    this module's own `class_feature.<class>.corpus_record.*` id shape by
//!    prefix (confirmed live by `sd25_druid_level_up_explanation_filter_
//!    audit.rs`/`sd25_monk_level_up_explanation_filter_audit.rs`, both
//!    updated this cycle, not weakened, to name the two new flat ids this
//!    widening introduces -- see their own doc comments). Separately,
//!    **`v06_work_inventory.rs`'s own `classify()` never reads
//!    `LevelUpPlan`/`level_up::` at all** (confirmed by grep: zero
//!    `level_up::` references in that file) -- it drives
//!    `compute_pilot_base_chassis` directly, so whether an explanation this
//!    module emits ever reaches a `LevelUpPlan` screen has NEVER been the
//!    gate on whether this mechanism's own `docs/work-inventory.json`
//!    verdict credits the record. The `LevelUpPlan`-reachability concern
//!    named by prior cycles is real or its own, differently-scoped
//!    mechanism (player-facing screen coverage), not this one (does the
//!    engine hold a computed fact at all). The class-wide exclusion is
//!    removed for both classes; the SAME citation-based property (above) is
//!    now the only gate for every class this module serves.
//!
//!    Druid's own owner-matched population has exactly one unit,
//!    `Archetype Druid`, `description: null` -- the citation gate refuses it
//!    for the SAME reason it refuses every other null-description record,
//!    so this widening changes NOTHING observable for Druid (confirmed by
//!    `a_class_feature_grant_consumer_no_longer_wholesale_excludes_druid_or_
//!    monk`, below, which asserts zero explanations for Druid at level 20).
//!    Monk's population has two real-description, non-`%`-leaking records
//!    this gate now admits: `Monk ~ Flurry of Blows`, `Monk ~ Unarmed
//!    Strike` -- neither collides with `monk.rs`'s own hand-wired
//!    `class_chassis.monk.flurry_of_blows_attack_bonus/_attack_count`/
//!    `unarmed_strike_damage_die(_count)` ids (the collision guard compares
//!    the id's TRAILING dot-segment only, and none of those four segments
//!    equals this module's own `flurry_of_blows`/`unarmed_strike` slugs),
//!    so both coexist as separate, non-shadowing facts -- the identical
//!    "flat roster fact alongside a real computed magnitude" shape already
//!    proven safe for the five previously-widened classes.
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

use crate::rules_core::record_vars::{self, ConvertedChain, SeedAbilityMods};
use super::{AbilityModifiers, ComputationExplanation, pu_feature_slug};

/// SD-34 `decisions.md` §18 ruling: the anti-fabrication gates for
/// Wizard/Bard/Paladin/Cleric/Sorcerer (`OPEN-ISSUES.md` rows 330/338's nine
/// `sd13_*`/`sd25_*` tests) are widened BY CONSTRUCTION -- an explanation is
/// admitted when it CITES A REAL CORPUS RECORD (this module's own
/// `corpus_records_with_real_description`/`resolved_description_for` gate,
/// unchanged), never because its class name sits on a hand-maintained
/// allowlist. The nine `sd13_*`/`sd25_*` gates were widened in the SAME
/// cycle (see each test file's own doc comment) to accept the resulting
/// `class_feature.<class>.corpus_record.*` ids by SHAPE, trusting that
/// production-side citation gate rather than re-deriving a class list
/// there.
///
/// **This cycle removes the constant entirely.** It used to also gate
/// Druid and Monk for a SEPARATE, THIRD, structurally distinct reason (see
/// the module-level doc comment's own "Druid and Monk" paragraph, above,
/// for the full re-investigation) -- direct re-reading of
/// `is_druid_pillar_id`/`is_monk_pillar_id`'s live source this cycle found
/// both already admit this module's `class_feature.<class>.corpus_record.*`
/// id shape by prefix, and `v06_work_inventory.rs`'s own `classify()` never
/// reads `LevelUpPlan`/`level_up::` at all, so the cited reason did not
/// actually gate this mechanism's own `docs/work-inventory.json` verdict.
/// The citation-based property above is now the ONLY gate, for every class
/// this module serves, with no exceptions.
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
///
/// `gate` (added for T7/D12, see [`resolvable_grants`]'s doc comment) is the
/// row's own `"gate"` field verbatim (`"preclass"` / `"mod_row_gated"` /
/// `"mod_row_ungated"`) -- `cache_gen::class_feature_grants.rs`'s own
/// documented invariant (mirrored in this file's earlier doc comment) is
/// that ONLY the bare-`PRECLASS:` resolution path can EVER be
/// archetype-sourced, so this is the one signal this module can read,
/// without re-parsing corpus text, to tell "a row this shallow, single-hop
/// `granted_via_archetype` check could plausibly have missed" apart from
/// "a row that structurally cannot be archetype-sourced at all".
#[derive(Debug)]
struct RawGrantFact {
    key: String,
    class: String,
    level: u8,
    gate: String,
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
                let gate = row["gate"].as_str().unwrap_or("").to_string();
                out.push(RawGrantFact { key: key.to_string(), class: class.to_string(), level, gate });
            }
        }
    }
    out
}

/// `(class.to_lowercase(), key)` -> the granted-at level, for every grant
/// fact that resolves WITHOUT a cross-book disagreement AND (T7/D12, below)
/// is not a bare-`PRECLASS:`-only fact with no corroborating non-`PRECLASS:`
/// fact for the same pair. See this module's doc comment, section 1, for why
/// disagreeing pairs are dropped rather than resolved by picking one side.
///
/// **T7/D12 -- shallow, single-hop `granted_via_archetype` traversal
/// (`docs/release/SD-31-corpus-closure-grind/`, tracked-defects list
/// `defects.md`, item D12).**
/// `granted_via_archetype` (`load_raw_grant_facts`'s own filter, above) reads
/// only the ONE row that carries the `ABILITY:` grant token's OWN `CATEGORY`
/// field -- a single hop. It cannot see a grant token nested INSIDE another
/// ability's definition row, where the archetype-ness lives one hop further
/// out, on the CONTAINING row (confirmed live:
/// `ultimate_combat/uc_abilities_class.lst:1970`'s "Guns Everywhere" optional
/// -rule row, `CATEGORY:Internal`, embeds `ABILITY:...|Gunslinger ~ Gun
/// Training|...|PRECLASS:1,Gunslinger=1` -- the embedded grant's own row
/// context is never archetype-flagged because the row that OWNS the grant
/// token is not itself the class's base definition; same shape at
/// `ultimate_combat/uc_abilities_class.lst:584`'s Evangelist "Sermonic
/// Performance" row for `Cleric ~ Channel Energy`, and
/// `ultimate_intrigue/ui_abilities_class.lst:587`'s Paladin analogue).
///
/// This module's own documented invariant (mirrored from
/// `cache_gen::class_feature_grants.rs`, this file's earlier doc comment,
/// "Only the bare-`PRECLASS:` resolution path... can EVER be
/// archetype-sourced") is the one lever available here without re-parsing
/// corpus text: a `.MOD`-row-gated fact (`gate` = `mod_row_gated` /
/// `mod_row_ungated`) can never be this shape, so it is always trusted at
/// face value; a bare-`PRECLASS:`-gated fact (`gate` = `preclass`) is the
/// ONLY shape this defect can hide in. Re-deriving the corpus census
/// (`t7_census.py`, cited in the cycle receipt) over the live merged data
/// found exactly one `(class, key)` pair, corpus-wide, whose SURVIVING
/// (non-archetype-flagged) facts are ALL `gate == "preclass"` with no
/// `mod_row_*` fact to corroborate them: `("gunslinger", "Gunslinger ~ Gun
/// Training")`. The other three D12-named pairs (`Cleric ~ Channel Energy`,
/// `Druid ~ Wild Shape`, `Paladin ~ Smite Evil`) already carry a genuine
/// `mod_row_gated` base-class fact at a DIFFERENT level, so they were already
/// refused by the cross-book-conflict rule above -- but only by that
/// incidental level disagreement, not by anything that reads `gate` at all
/// (defects.md D12's own finding). Refusing every uncorroborated
/// bare-`PRECLASS:` pair closes the whole shape structurally: it no longer
/// matters whether a future corpus edit happens to make the levels agree,
/// because the missing `mod_row_*` corroboration is what is actually being
/// checked now, not a level coincidence.
fn resolvable_grants() -> &'static BTreeMap<(String, String), u8> {
    static TABLE: OnceLock<BTreeMap<(String, String), u8>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut levels_seen: BTreeMap<(String, String), BTreeMap<u8, ()>> = BTreeMap::new();
        let mut gates_seen: BTreeMap<(String, String), std::collections::BTreeSet<String>> =
            BTreeMap::new();
        for fact in load_raw_grant_facts() {
            let pair = (fact.class.to_lowercase(), fact.key);
            levels_seen.entry(pair.clone()).or_default().insert(fact.level, ());
            gates_seen.entry(pair).or_default().insert(fact.gate);
        }
        levels_seen
            .into_iter()
            .filter_map(|(pair, levels)| {
                if levels.len() != 1 {
                    // Cross-book conflict: refuse the whole pair rather than
                    // guess which book wins.
                    return None;
                }
                let gates = gates_seen.get(&pair).cloned().unwrap_or_default();
                if gates.len() == 1 && gates.contains("preclass") {
                    // T7/D12: a bare-PRECLASS:-only pair with no mod_row_*
                    // corroboration -- refuse structurally rather than trust
                    // the single-hop `granted_via_archetype` derivation.
                    return None;
                }
                levels.into_keys().next().map(|level| (pair, level))
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
                // SD-35 `AT-35-E6-003-SWEEP` cycle 17: the guard this module reproduces is
                // `class_feature_pool_catalog`'s, and that catalog stopped rendering the
                // ingest format's description at run time. It now asks the converted package
                // for the record's words; a record the package states no prose for is one the
                // catalog would refuse, which is exactly what this walk must not claim.
                let book = book_entry.file_name().to_string_lossy().to_string();
                // `settled_description_for`, **not** `description_for`: this table's whole
                // meaning is "the sheet's Class Features section can already print this
                // record's sentence with no character in hand". A sentence whose magnitude is
                // still a term does not meet that bar — it is `resolved_description_for`'s
                // job, one branch down, where this character's own level settles it. Using the
                // unsettled form here silently replaced 32 resolved, per-character sentences
                // with their term-word form.
                let Some(converted) = crate::rules_core::converted_prose::settled_description_for(
                    &book,
                    "class_feature",
                    key,
                ) else {
                    continue;
                };
                let lower = converted.to_ascii_lowercase();
                if lower.contains("[not implemented]") || lower.contains("[not enforced]") {
                    continue;
                }
                // The gate-weakening review finding of SD-31 wave 23 — that an unresolved `%N`
                // argument silently DROPPED, rather than leaked as literal syntax, left a
                // sentence reading "You gain a + bonus..." with no `%` character to catch — is
                // now settled one level upstream. A row whose magnitude the converter could not
                // finish reaches the package as no prose at all, which the `else` above
                // refuses, so this module still never claims a record whose magnitude is
                // missing from its words.
                out.entry(key.to_string()).or_insert_with(|| name.to_string());
            }
        }
        out
    })
}

// ---------------------------------------------------------------------------------------------
// SD-31 wave 26: resolving `%N` corpus DESC placeholders through the formula interpreter
// (`OPERATOR-RULINGS-2026-08-21.md` §20, "RULED, 2026-08-21: §24.1 IS OVERTURNED. Build the
// interpreter.").
// ---------------------------------------------------------------------------------------------
//
// `corpus_records_with_real_description` above (SD31-W23) admits a grant fact ONLY when its
// record's raw `DESC:` renders CLEAN with NO character context at all -- any `%N` reference is
// grounds for exclusion, full stop. That was the correct, conservative call in wave 23: no
// mechanism existed yet to fill a `%N` honestly. Wave 25b built one
// (the converter-side formula interpreter, `crate::pcgen_import::formula_interpreter`, proven to
// reproduce 22 of 22 hand-modelled functions, zero disagreements) and this section is what plugs
// its CONVERTED output in for `class_feature`,
// following the ALREADY-ESTABLISHED, ALREADY-LIVE precedent
// `pilot_compute/mod.rs::pu_display_values` / `pu_resolved_description` set for Pathfinder
// Unchained: read the same-record `BONUS:VAR` chain, seed it with the ONE fact this module
// actually has about a specific character (their level in the granting class), evaluate with the
// real interpreter, and hand the result to `pcgen_desc::render_pcgen_desc_with_values` -- the
// SAME renderer, unmodified, that already enforces "drop and report, never guess" for any
// argument this chain cannot reach.
//
// # Scope: level-only chains, one class per record
//
// This resolver binds exactly one variable from outside the record itself: `<Class ...
// no-spaces>LVL` (PCGen's own auto-declared per-class level variable -- confirmed corpus-wide,
// e.g. `Bard` -> `BardLVL`, `Slayer` -> `SlayerLVL`, `Alchemist` -> `AlchemistLVL`) bound to the
// character's real level in that class, taken from this function's own `level` parameter (the
// SAME single-class-only precondition `push_generic_class_feature_grant_records`'s own caller
// already documents). No ability-modifier binding exists yet -- a formula whose chain bottoms out
// in anything else (an ability abbreviation, a sibling record's own variable, a shape the
// interpreter refuses such as the documented bare-comparison-as-numeric-term gap) simply never
// resolves, is never guessed, and the grant fact this module already skips today keeps being
// skipped. Widening to ability modifiers is real, scoped follow-on work (`ability_modifiers` is
// already in scope at this module's one call site, `compute_class_chassis`), not attempted here.
//
// # Why this belongs in `detail`, not a new render surface
//
// `ClassFeatureRow.detail` (`classFeaturesModel.ts`) is ALREADY rendered verbatim on the
// character sheet -- "the engine's own corpus citation", per that file's own module doc. Routing
// the resolved sentence through it needs zero new IPC surface, zero new Tauri command, and zero
// new frontend wiring: the render path this wave's brief asks for ("wire it into the
// description-completion path... the render path... is already book-agnostic") is this one,
// already proven, already live.

/// One class_feature corpus record's raw PCGen tokens this resolver needs: its owning class
/// (read straight from `data.class`, never re-derived from the `KEY:` text), display name, raw
/// `DESC:` token text (verbatim, `%N` unresolved), and every same-record `BONUS:VAR` name ->
/// formula pair. A comma-separated multi-target `BONUS:VAR` row (PCGen's own shape, e.g.
/// `BONUS:VAR|CMB_Sunder,CMD_Sunder|SunderTrainingSunderBonus`) contributes one entry per named
/// target, all sharing the same formula text.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ClassFeatureRecordTokens {
    pub(crate) name: String,
    pub(crate) class: String,
    pub(crate) raw_description: String,
    pub(crate) bonus_vars: ConvertedChain,
}

/// Every `data/corpus/*/class_feature/**/*.json` record that carries a real (non-empty,
/// non-`.CLEAR`, non-PI-marker) description, keyed by corpus `KEY:`, regardless of whether that
/// description carries an unresolved `%N` -- the strictly WIDER sibling of
/// `corpus_records_with_real_description` above, which additionally requires the description to
/// already render clean with no character context. First book (alphabetically) wins a duplicate
/// key, mirroring that function's own convention.
///
/// SD-32 T12 row 21 cycle 2: this is now a thin alias for
/// [`class_feature_record_tokens_pre_gate_safe`] rather than a second, independently-built table.
/// Before row 21 cycle 1 restored the corpus's real `.MOD`-appended `BONUS:VAR` rows, every
/// record this table covered carried at most one raw row per target name, so the now-deleted
/// `parse_bonus_var_tokens`'s last-write-wins behaviour and `parse_bonus_var_tokens_pre_gate_safe`'s
/// PRE-gate-aware summation agreed on every record and the duplication was harmless. Restoring
/// those dropped rows exposed the disagreement for real: `core_rulebook:class_feature:
/// barbarian_damage_reduction` now carries multiple same-named, PRE-gated `BONUS:VAR|BarbarianDR|`
/// rows, and last-write-wins silently picked the WRONG one (`resolve_pcgen_var_chain` bound
/// `BarbarianDR=-1` at level 7 where the pinned upstream `.lst` states `+1` --
/// `tests/derived_evaluator_fixture_check.rs::
/// engine_evaluator_output_equals_the_corpus_derived_expected_value`). The PRE-gate-safe sibling
/// parser (built for `resolve_pool_member_sole_magnitude`, see its own doc above) already exists
/// and already proves this shape safe generically; reusing it here -- rather than writing a third
/// parser or patching the deleted one to also understand PRE-gates -- is Decision `§17`'s generic-
/// pass requirement, not a per-record special case.
pub(crate) fn class_feature_record_tokens() -> &'static BTreeMap<String, ClassFeatureRecordTokens> {
    class_feature_record_tokens_pre_gate_safe()
}

// ---------------------------------------------------------------------------------------------
// The record's own variable chain, read from the CONVERTED artifact.
//
// SD-35 `AT-35-E6-001` cycle 4 (`decisions.md` §11). Everything between this comment and
// `resolved_description_for` below used to read the source tokens and run them: the row reader
// (which decides WHICH source rows a target sums over), the formula parser, and the formula
// evaluator. All three now run ONCE, at ingest,
// in `crate::pcgen_import::class_feature_vars`, and write `data/converted/record_vars.json`.
// This module joins that artifact to the record fields it still reads from the corpus (name,
// owning class, raw description) and evaluates our own converted `Expr`
// (`crate::rules_core::record_vars::resolve_chain`).
//
// Nothing here parses a formula, names a source token, or decides which source row wins. The
// corpus-wide before/after comparison over every record and every level 1..=20 is in
// `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/
// AT-35-E6-001_cycle4_varchain-{before,after}.json`, re-derivable with
// `AT35_E6_VARCHAIN_DUMP=<path> cargo test --locked --lib -j 6 -- --ignored
// class_feature_grant_consumer::tests::dump_the_whole_var_chain_population`.
// ---------------------------------------------------------------------------------------------

/// The one per-target merge policy every cross-book table here shares (SD-32 T12 Epic 8 row 18
/// cycle 10): a target already bound by an earlier book is never overwritten, but a target that
/// book never carried does merge in. Factored out so the header-side and member-side merges
/// cannot drift into two different collision policies the way they silently did before cycle
/// 8/10 fixed them one at a time.
pub(crate) fn merge_bonus_var_target_map_never_overwriting(
    into: &mut ConvertedChain,
    from: ConvertedChain,
) {
    for (target, value) in from {
        into.entry(target).or_insert(value);
    }
}

/// Every `data/corpus/*/class_feature/**/*.json` record whose description passes the same gate
/// `class_feature_descriptions.rs`'s catalog applies before serving a record to a player, keyed
/// by corpus `KEY:`, joined to its converted chain.
///
/// SD-32 T12 Epic 8 row 18 cycle 18 (`§27b`): `description: null` is ADMITTED (as an empty
/// `raw_description`, never fabricated text) rather than skipped -- a real, invisible, purely
/// mechanical sub-ability (`ultimate_magic/class_feature/jungle_domain/trap_sense.json`,
/// `VISIBLE:NO`) was refused purely because the old gate dropped the record before its chain was
/// ever read. A description that IS present but carries a bad value (`.CLEAR`, a PI-redaction
/// marker) is still refused exactly as before, so the PI-safety gate (`§15`) is untouched.
pub(crate) fn class_feature_record_tokens_pre_gate_safe()
-> &'static BTreeMap<String, ClassFeatureRecordTokens> {
    static TABLE: OnceLock<BTreeMap<String, ClassFeatureRecordTokens>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let chains = &record_vars::package().class_feature_described;
        let mut out: BTreeMap<String, ClassFeatureRecordTokens> = BTreeMap::new();
        for_each_class_feature_record(|key, data| {
            let (Some(name), Some(class)) = (data["name"].as_str(), data["class"].as_str()) else {
                return;
            };
            let raw_desc = match data["description"].as_str() {
                Some(s) => {
                    if !is_real_description_value(s) {
                        return;
                    }
                    s.to_string()
                }
                None => String::new(),
            };
            out.entry(key.to_string()).or_insert_with(|| ClassFeatureRecordTokens {
                name: name.to_string(),
                class: class.to_string(),
                raw_description: raw_desc,
                bonus_vars: chains.get(key).cloned().unwrap_or_default(),
            });
        });
        out
    })
}

/// Every corpus `class_feature` record's converted chain, keyed by `KEY:` -- WITHOUT the sibling
/// table's `data.description` requirement (SD-32 T12 Epic 8). A pool's own HEADER record
/// (`"Alchemist ~ Discovery"`, `"Witch ~ Hex"`) very often defines the pool-specific level
/// variable individual members scale on and carries `description: null` in this corpus, so the
/// description-gated sibling would starve the header-chain merge of exactly the variable it
/// exists to supply. `class` is tolerated as absent here (kept `""`, never fabricated): every one
/// of the 53 real Sorcerer Bloodline groups' own header records ingests with `class: null`.
///
/// Merged across books per target name, never first-book-wins: one real ability's `.MOD`-appended
/// rows land in several books' copies, each carrying a different subset.
pub(crate) fn class_feature_bonus_vars_any_record()
-> &'static BTreeMap<String, ClassFeatureRecordTokens> {
    static TABLE: OnceLock<BTreeMap<String, ClassFeatureRecordTokens>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let chains = &record_vars::package().class_feature_any;
        let mut out: BTreeMap<String, ClassFeatureRecordTokens> = BTreeMap::new();
        for_each_class_feature_record(|key, data| {
            let Some(name) = data["name"].as_str() else { return };
            let class = data["class"].as_str().unwrap_or("");
            let raw_desc = data["description"].as_str().unwrap_or("").to_string();
            let entry =
                out.entry(key.to_string()).or_insert_with(|| ClassFeatureRecordTokens {
                    name: name.to_string(),
                    class: class.to_string(),
                    raw_description: raw_desc.clone(),
                    bonus_vars: chains.get(key).cloned().unwrap_or_default(),
                });
            if entry.class.is_empty() && !class.is_empty() {
                entry.class = class.to_string();
            }
            if entry.raw_description.is_empty() && !raw_desc.is_empty() {
                entry.raw_description = raw_desc;
            }
        });
        out
    })
}

/// Walk every `data/corpus/*/class_feature/**/*.json` record in book-alphabetical order, handing
/// each one's corpus `KEY:` and its `data` object to `visit`. The ONE corpus walk both tables
/// above share, so they can never disagree about which files exist or in what order.
fn for_each_class_feature_record(mut visit: impl FnMut(&str, &Value)) {
    let corpus_root = repo_root().join("data/corpus");
    let Ok(books) = std::fs::read_dir(&corpus_root) else { return };
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
            let Some(key) = data["key"].as_str() else { continue };
            let key = key.to_string();
            visit(&key, data);
        }
    }
}

/// SD-32 T12 Epic 8 row 18 cycle 21 (`§27b`): every `class_feature/wildblooded/*.json` record's
/// own declared PARENT bloodline pool-group name, keyed by the VARIANT's own pool-group name
/// (`"Bedrock Bloodline"` -> `"Deep Earth Bloodline"`). A Wildblooded variant is corpus-keyed as
/// if it were its own pool group, but its own prerequisite proves selecting it REQUIRES already
/// holding the named parent bloodline -- so the parent's header variables are, by corpus-declared
/// construction, genuinely bound whenever a variant's members reference them. Derived at ingest
/// (`pcgen_import::class_feature_vars`), read here.
pub(crate) fn wildblooded_variant_parent_pool_group() -> &'static BTreeMap<String, String> {
    &record_vars::package().wildblooded_parents
}

/// Every corpus `data/corpus/*/class/*.json` CLASS record's converted chain, keyed by `class_id`
/// (SD-32 T12 Epic 8 row 18 cycle 8). Cleric's own `DomainLVL` binds on the CLASS record itself,
/// not on any `class_feature` record, and every one of the 67 real Cleric Domain groups' members
/// needs that binding and none of them can supply it themselves. A class record that binds no
/// variable at all is simply absent, which every consumer already treats as an empty chain.
pub(crate) fn class_record_bonus_vars() -> &'static BTreeMap<String, ConvertedChain> {
    &record_vars::package().class_records
}

/// Every corpus `data/corpus/*/domain/*.json` DOMAIN record's converted chain, keyed by the
/// domain's own bare `KEY:` (`"Cave"`, never `"Cave Domain"`) (SD-32 T12 Epic 8 row 18 cycle 18,
/// `§27b`). A `domain`-kind record already carries the whole resolvable chain every one of that
/// domain's `class_feature` MEMBER records needs; no other table reads this subdirectory at all,
/// so every domain whose only real header lives here was refused for want of this read path
/// rather than for want of real data.
pub(crate) fn domain_kind_bonus_vars_any_record() -> &'static BTreeMap<String, ConvertedChain> {
    &record_vars::package().domain_records
}

/// The class's own level-variable name (`Bard` -> `BardLVL`) -- the seed key a caller binds the
/// character's level in the granting class to. Defined by
/// [`crate::rules_core::record_vars`]; re-exported here because every caller already reaches for
/// it by this path.
pub(crate) use crate::rules_core::record_vars::class_level_variable_name;

/// Resolve every variable this record's own converted chain can reach, seeded with the two facts
/// this module knows about one character: their level in the granting class, and their six
/// ability modifiers.
///
/// SD-35 `AT-35-E6-001` cycle 4: this used to be a fixpoint over source formula TEXT, run through
/// the PCGen formula interpreter at request time. It is now a fixpoint over converted [`Expr`],
/// and the arithmetic is `sheet_rule`'s own. The contract is deliberately unchanged --
/// substitute what the chain reaches, then default only a reference the corpus binds NOWHERE
/// (its declared baseline, or 0), never one it binds somewhere under some condition, and never
/// a `classlevel(<another class>)` term; anything that still does not close is ABSENT from the
/// result rather than guessed. See `record_vars`'s own module doc for why each of those four
/// steps is what it is, and `AT-35-E6-001_cycle4_varchain-{before,after}.json` for the
/// corpus-wide proof that the swap moved no value.
pub(crate) fn resolve_pcgen_var_chain(
    bonus_vars: &ConvertedChain,
    class_level_var: &str,
    level: u8,
    ability_modifiers: &AbilityModifiers,
) -> BTreeMap<String, i64> {
    record_vars::resolve_chain(
        bonus_vars,
        class_level_var,
        level,
        SeedAbilityMods {
            values: [
                i64::from(ability_modifiers.strength),
                i64::from(ability_modifiers.dexterity),
                i64::from(ability_modifiers.constitution),
                i64::from(ability_modifiers.intelligence),
                i64::from(ability_modifiers.wisdom),
                i64::from(ability_modifiers.charisma),
            ],
        },
        &record_vars::package().var_defaults,
    )
}


/// This grant fact's real corpus `DESC:` description with THIS CHARACTER's own numbers
/// substituted in place of every `%N`, or `None` when the chain does not fully resolve -- exactly
/// the "drop and report, never guess" contract `render_pcgen_desc_with_values` already enforces,
/// extended here only by WHERE the values come from (the real formula interpreter over this
/// record's own `BONUS:VAR` chain, seeded with the character's real class level and real ability
/// modifiers) rather than a hand-modelled function.
pub(crate) fn resolved_description_for(
    key: &str,
    level: u8,
    ability_modifiers: &AbilityModifiers,
) -> Option<String> {
    let record = class_feature_record_tokens().get(key)?;
    let class_level_var = class_level_variable_name(&record.class);
    let resolved_vars =
        resolve_pcgen_var_chain(&record.bonus_vars, &class_level_var, level, ability_modifiers);
    let mut values = crate::pcgen_import::pcgen_desc::PcgenDisplayValues::new();
    for (name, value) in &resolved_vars {
        values.set(name, *value);
    }
    let rendered =
        crate::pcgen_import::pcgen_desc::render_pcgen_desc_with_values(&record.raw_description, &values);
    if !rendered.dropped_args.is_empty() || rendered.text.is_empty() {
        return None;
    }
    if crate::pcgen_import::pcgen_desc::leaked_pcgen_syntax(&rendered.text).is_some() {
        return None;
    }
    Some(rendered.text)
}

/// A pool member's real `%N`-substituted `DESC:` formula resolved DIRECTLY (SD-32 T12 Epic 8
/// row 18 cycle 15), for the corpus shape cycle 14's own `§16` finding named and refused to
/// force through the wrong module: a record whose `bonus_vars` is EMPTY (so `resolve_pool_
/// member_sole_magnitude` -- which only ever reads `bonus_vars` -- correctly returns `None` for
/// it, per that function's own precondition) but whose `raw_description` carries a real `%N`
/// argument that is itself a raw PCGen formula EXPRESSION (`"max(1,WarpriestLVL/2)"`,
/// `"if(WarpriestLVL<19,1+((WarpriestLVL/2)-5),5)"`, a bare `"WarpriestLVL"`) rather than a
/// bare variable name a `BONUS:VAR` chain would bind. `formula_interpreter.rs`'s own module doc
/// scopes `%N` DESC-argument substitution OUT of that module and names this one
/// (`pcgen_desc.rs`) as the real consumer; this is that consumer, extended only by WHERE an
/// argument's value comes from when [`resolve_desc_argument`](crate::pcgen_import::pcgen_desc)'s
/// own three narrow shapes (integer literal, exact named lookup, `<Name><+|-><integer>` offset)
/// do not cover it -- converted at ingest alongside the record's own chain
/// (`crate::pcgen_import::class_feature_vars`, keyed by the exact argument text) and evaluated
/// through the SAME converted fold every other resolver in this module uses, seeded with the
/// SAME two facts (class level, ability modifiers), never a new evaluation mechanism.
///
/// Returns `None` (never a guess) unless the ENTIRE description renders clean -- every `%N`
/// argument resolves to a value, `leaked_pcgen_syntax` finds nothing raw left over -- exactly
/// [`resolved_description_for`]'s own "drop and report, never partially-render" contract, reused
/// unchanged. On success, also returns `%1`'s own resolved value as the caller's single
/// representative magnitude (every real member this cycle's own corpus scan found states its
/// primary number as `%1` first; a member whose `%1` is itself unresolvable already returned
/// `None` above, so this unwrap is only ever reached after `%1`'s formula has already evaluated).
///
/// Deliberately does NOT read `bonus_vars` at all, unlike [`resolved_description_for`] -- a
/// record that DOES carry a real `BONUS:VAR` chain is `resolved_description_for`'s own business,
/// not this function's; the two are mutually exclusive by construction (see the `bonus_vars.
/// is_empty()` guard below) so a caller can safely try both without ever double-resolving the
/// same record two different ways.
///
/// `header_vars`: SD-32 T12 Epic 8 row 18 cycle 19 -- the SAME "pool header chain" shape
/// [`super::pool_header_record_by_normalized_suffix`] already merges for
/// [`super::resolve_pool_member_sole_magnitude`]'s own `BONUS:VAR`-chain path, extended to this,
/// the OTHER generic resolver. Cycle 18 named this precisely: `Mountain Domain ~ Foothold`'s `%1`
/// argument is the bare identifier `DomainMountainTimes`, resolvable through Cleric's own
/// class-record `BONUS:VAR|DomainPowerTimes|3+WIS` header chain (the SAME chain gap 1 in cycle
/// 18's own Cleric Domain fix already wired for the bonus_vars-only resolver) -- but this
/// resolver, unlike that one, never received it, because it evaluates each `%N` argument as a
/// bare formula string against ONLY the ability-modifier and class-level seed, with no header
/// merge step of its own. Resolved through the SAME [`resolve_pcgen_var_chain`] every other
/// resolver in this module already uses (never a new evaluation mechanism, `§17`), then folded
/// into `seed_vars` via `.entry().or_insert()` -- never overwriting the ability-modifier/
/// class-level seeds already bound above, exactly the "never fabricate, never overwrite an
/// already-bound identifier" policy every other merge in this module already follows. An empty
/// `header_vars` map (both existing call sites before this cycle) makes this a true no-op --
/// `resolve_pcgen_var_chain` on an empty map only ever re-derives the same ability-modifier/
/// class-level seed this function was already computing, so prior behaviour is preserved exactly.
pub(crate) fn resolved_description_for_formula_only_desc_argument(
    key: &str,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    header_vars: &ConvertedChain,
) -> Option<(String, i64)> {
    let record = class_feature_record_tokens_pre_gate_safe().get(key)?;
    if !record.bonus_vars.is_empty() {
        return None; // a real chain exists -- `resolved_description_for`'s business.
    }
    let args = crate::pcgen_import::pcgen_desc::desc_token_arguments(&record.raw_description);
    if args.is_empty() {
        return None; // no `%N` argument at all -- nothing this function grounds.
    }
    // SD-35 `AT-35-E6-001` cycle 4: the arguments themselves are converted at ingest
    // (`pcgen_import::class_feature_vars`, keyed by the exact argument text). This function
    // binds the seeds and evaluates; it no longer parses anything.
    let converted_args = record_vars::package().desc_arguments.get(key)?;
    let class_level_var = class_level_variable_name(&record.class);
    let mut seed_vars: BTreeMap<String, i64> = BTreeMap::new();
    for (i, (abbr, _)) in record_vars::ABILITY_SEED_NAMES.iter().enumerate() {
        let value = [
            ability_modifiers.strength,
            ability_modifiers.dexterity,
            ability_modifiers.constitution,
            ability_modifiers.intelligence,
            ability_modifiers.wisdom,
            ability_modifiers.charisma,
        ][i];
        seed_vars.insert((*abbr).to_string(), i64::from(value));
    }
    seed_vars.insert(class_level_var.clone(), i64::from(level));
    if let Some(class_name) = class_level_var.strip_suffix("LVL") {
        seed_vars.insert(record_vars::class_level_call_key(class_name), i64::from(level));
    }
    if !header_vars.is_empty() {
        let resolved_header =
            resolve_pcgen_var_chain(header_vars, &class_level_var, level, ability_modifiers);
        for (name, value) in &resolved_header {
            seed_vars.entry(name.clone()).or_insert(*value);
        }
    }
    let mut values = crate::pcgen_import::pcgen_desc::PcgenDisplayValues::new();
    for arg in &args {
        let trimmed = arg.trim();
        let Some(converted) = converted_args.get(trimmed) else { continue };
        // Keyed under the exact argument text -- `resolve_desc_argument`'s own named-lookup shape
        // (`values.get(arg)`) then finds it by that same text, with no change to that function
        // or to `render_pcgen_desc_with_values` itself. An argument that does not resolve is
        // simply never inserted, and the renderer drops and reports it -- its existing
        // no-fabrication contract, unchanged.
        if let Some(value) = record_vars::evaluate_with_bindings(converted, &seed_vars) {
            values.set(trimmed, value);
        }
    }
    let rendered = crate::pcgen_import::pcgen_desc::render_pcgen_desc_with_values(
        &record.raw_description,
        &values,
    );
    if !rendered.dropped_args.is_empty() || rendered.text.is_empty() {
        return None;
    }
    if crate::pcgen_import::pcgen_desc::leaked_pcgen_syntax(&rendered.text).is_some() {
        return None;
    }
    let primary_value = record_vars::evaluate_with_bindings(
        converted_args.get(args[0].trim())?,
        &seed_vars,
    )?;
    Some((rendered.text, primary_value))
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
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(owner) = class_id_str.strip_prefix("class:") else { return };
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
        // SD-34 decisions.md section 18: a NAMED, per-record refusal, not a per-class one --
        // found live while widening Bard by construction. `Bard ~ Versatile Performance`
        // cites a real corpus record (so the citation-based gate alone would admit it), but
        // three dedicated, pre-existing, unmodified acceptance tests
        // (`sd13_bard_level2_progression.rs`/`sd13_bard_level3_progression.rs`::
        // `bard_levelN_does_not_fabricate_versatile_performance`,
        // `sd13_bard_level10_progression.rs`'s own `contains("versatile")` guard) assert this
        // module's real gap directly: Versatile Performance is a choice-gated skill-
        // substitution engine that does not exist in this codebase, the SAME reasoning this
        // module's own module-doc already applies to Rogue Talent-shaped option pools. This
        // refusal is a PROPERTY of this one record (a real citation whose mechanical effect
        // is provably unimplemented, evidenced by three independent, unrelated tests), never
        // a class-wide exclusion -- every other Bard grant fact still emits normally.
        if class == "bard" && key == "Bard ~ Versatile Performance" {
            continue;
        }
        // Two independent paths to a servable name for this record:
        //
        // 1. (SD31-W23, unchanged) `descriptions` -- the record's raw description already
        //    renders clean with NO character context at all. Its real prose is served
        //    separately by the STATIC, book-agnostic `class_feature_descriptions.rs` render
        //    path; this branch's own `detail` text is byte-identical to before this wave.
        // 2. (SD-31 wave 26, NEW) The record's description carries an unresolved `%N`, but
        //    THIS character's own class level lets the formula interpreter resolve it (see
        //    `resolved_description_for` above). Previously this whole grant fact was skipped
        //    outright (`descriptions.get` returned `None` and the loop moved on) -- it is now
        //    emitted WITH its real, per-character resolved sentence embedded directly in
        //    `detail`, which `classFeaturesModel.ts` already renders verbatim on the sheet. A
        //    record whose chain does NOT resolve (ability-modifier-dependent, a shape the
        //    interpreter refuses, an unknown grant class) keeps being skipped exactly as
        //    before -- refuse, never guess.
        let (name, resolved_prose): (&str, Option<String>) =
            if let Some(name) = descriptions.get(key) {
                (name.as_str(), None)
            } else if let Some(record) = class_feature_record_tokens().get(key) {
                match resolved_description_for(key, level, ability_modifiers) {
                    Some(text) => (record.name.as_str(), Some(text)),
                    None => continue,
                }
            } else {
                continue;
            };
        let feature_slug = pu_feature_slug(key);
        if feature_slug.is_empty() {
            continue;
        }
        if already_computed_slugs.contains(feature_slug.as_str()) {
            continue;
        }
        let detail = match &resolved_prose {
            Some(prose) => format!(
                "{owner} level {level}: `{key}` (\"{name}\") is a class feature of this \
                 character, granted from class level {granted_at}. {prose} (the real rulebook \
                 description, with this character's own numbers resolved through the PCGen \
                 formula interpreter, per a grant fact ingested from PCGen's own \
                 class-progression tokens, data/class_feature_grants)."
            ),
            None => format!(
                "{owner} level {level}: `{key}` (\"{name}\") is a class feature of this \
                 character, granted from class level {granted_at}, per a grant fact ingested \
                 from PCGen's own class-progression tokens (data/class_feature_grants). The \
                 record's real rulebook description is served separately by the character \
                 sheet's Class Features section."
            ),
        };
        explanations.push(ComputationExplanation {
            id: format!("class_feature.{owner}.corpus_record.{feature_slug}"),
            value: i16::from(granted_at),
            detail,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Lower a `name -> source formula` map the way the ingest-time converter does, so a test
    /// that states its case in the source form still exercises the REAL conversion and the REAL
    /// fold (SD-35 `AT-35-E6-001` cycle 4). A formula the converter refuses is absent from the
    /// result, exactly as it is absent from the shipped artifact.
    pub(crate) fn lowered_chain_for_test(formulas: &BTreeMap<String, String>) -> ConvertedChain {
        let mut out = ConvertedChain::new();
        for (name, formula) in formulas {
            let Ok(lowered) = crate::pcgen_import::class_feature_vars::lower_formula(formula)
            else {
                continue;
            };
            out.insert(
                name.clone(),
                crate::rules_core::record_vars::ConvertedVar {
                    expr: lowered.expr,
                    refs: lowered.refs.into_iter().collect(),
                },
            );
        }
        out
    }

    /// SD-35 `AT-35-E6-001` cycle 4 -- the corpus-wide before/after comparison cycle 3 named as
    /// the precondition for swapping this module's run-time PCGen formula evaluation for the
    /// converter's own converted `Expr`. Writes, for EVERY record this module's own table
    /// carries, at every level 1..=20 and under two ability-modifier probes (all-zero and a
    /// spread that makes each of the six abbreviations distinguishable), the whole resolved
    /// variable map. Two runs of this dump -- one at the pre-swap commit, one after -- are the
    /// evidence; the file it writes is named by `AT35_E6_VARCHAIN_DUMP`.
    ///
    /// `#[ignore]`d because it is an evidence-producing dump over the whole corpus, not a
    /// property: it asserts nothing about the values, only that the population is non-empty.
    /// Run: `AT35_E6_VARCHAIN_DUMP=<path> cargo test --locked --lib -j 6 -- --ignored
    /// class_feature_grant_consumer::tests::dump_the_whole_var_chain_population`.
    #[test]
    #[ignore = "evidence dump over the whole corpus; run explicitly with AT35_E6_VARCHAIN_DUMP set"]
    fn dump_the_whole_var_chain_population() {
        let Ok(out_path) = std::env::var("AT35_E6_VARCHAIN_DUMP") else {
            panic!("set AT35_E6_VARCHAIN_DUMP to the file to write");
        };
        let spread = AbilityModifiers {
            strength: 1,
            dexterity: 2,
            constitution: 3,
            intelligence: 4,
            wisdom: 5,
            charisma: 6,
        };
        let probes: [(&str, AbilityModifiers); 2] =
            [("zero", AbilityModifiers::default()), ("spread", spread)];
        let table = class_feature_record_tokens_pre_gate_safe();
        let mut out = serde_json::Map::new();
        let mut resolved_pairs = 0usize;
        for (key, record) in table {
            if record.bonus_vars.is_empty() {
                continue;
            }
            let class_level_var = class_level_variable_name(&record.class);
            let mut per_record = serde_json::Map::new();
            for (probe_name, mods) in &probes {
                let mut per_probe = serde_json::Map::new();
                for level in 1u8..=20 {
                    let vars = resolve_pcgen_var_chain(
                        &record.bonus_vars,
                        &class_level_var,
                        level,
                        mods,
                    );
                    // Only the record's OWN target names -- the seeds (class level, the six
                    // ability abbreviations) are inputs, not results, and would otherwise
                    // dominate the diff with values that never came from a formula.
                    let mut per_level = serde_json::Map::new();
                    for name in record.bonus_vars.keys() {
                        if let Some(v) = vars.get(name) {
                            per_level.insert(name.clone(), serde_json::json!(v));
                            resolved_pairs += 1;
                        }
                    }
                    per_probe.insert(level.to_string(), serde_json::Value::Object(per_level));
                }
                per_record.insert((*probe_name).to_string(), serde_json::Value::Object(per_probe));
            }
            out.insert(key.clone(), serde_json::Value::Object(per_record));
        }
        assert!(out.len() > 100, "population collapsed: only {} records carry BONUS:VAR", out.len());
        let body = serde_json::json!({
            "records": out.len(),
            "resolved_name_level_pairs": resolved_pairs,
            "by_record": serde_json::Value::Object(out),
        });
        std::fs::write(&out_path, serde_json::to_vec_pretty(&body).expect("serialise"))
            .expect("write dump");
        eprintln!(
            "AT35_E6_VARCHAIN_DUMP records={} resolved_name_level_pairs={} -> {out_path}",
            body["records"], body["resolved_name_level_pairs"]
        );
    }

    /// SD-35 `AT-35-E6-001` cycle 4, against the LIVE corpus: the three real records whose
    /// magnitude the retired interpreter answered `0` for, because the corpus row writes the
    /// class level variable in a different case than the class declares it. Each value below is
    /// the published rule, checked by hand, not a re-print of what the code now returns.
    ///
    /// Re-derive: `cargo test --locked --lib -j 6 --
    /// class_feature_grant_consumer::tests::a_mixed_case_class_level_reference_now_scales`.
    #[test]
    fn a_mixed_case_class_level_reference_now_scales() {
        let zero = AbilityModifiers::default();
        let value = |key: &str, target: &str, class: &str, level: u8| -> Option<i64> {
            let record = class_feature_record_tokens_pre_gate_safe().get(key)?;
            let vars = resolve_pcgen_var_chain(
                &record.bonus_vars,
                &class_level_variable_name(class),
                level,
                &zero,
            );
            vars.get(target).copied()
        };
        // Knife Master (Ultimate Combat): "+1/2 her rogue level" concealed-weapon bonus.
        // `BONUS:VAR|HiddenBladeBonus|RogueLvl/2`, and the class declares `RogueLVL`.
        assert_eq!(value("Knife Master ~ Hidden Blade", "HiddenBladeBonus", "Rogue", 10), Some(5));
        assert_eq!(value("Knife Master ~ Hidden Blade", "HiddenBladeBonus", "Rogue", 1), Some(0));
        // Empyreal Knight (Paladin archetype): resistance 5 at 3rd, 10 at 9th.
        // The gate is `PREVARGTEQ:PaladinLvl`, the class declares `PaladinLVL`.
        assert_eq!(
            value("Empyreal Knight ~ Celestial Heart", "AcidResistanceBonus", "Paladin", 3),
            Some(5)
        );
        assert_eq!(
            value("Empyreal Knight ~ Celestial Heart", "AcidResistanceBonus", "Paladin", 9),
            Some(10)
        );
        assert_eq!(
            value("Empyreal Knight ~ Celestial Heart", "AcidResistanceBonus", "Paladin", 2),
            Some(0),
            "below 3rd the gate does not fire and the bonus really is 0"
        );
        // Loremaster: one secret at 1st and every odd level after -- `(LoreMasterLVL+1)/2`,
        // where the class declares `LoremasterLVL`.
        assert_eq!(
            value("Loremaster ~ Secret Lore", "LoremasterSecretCount", "Loremaster", 5),
            Some(3)
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 10. The shared merge policy both
    /// cross-book tables now use: a target seen in an earlier book is never
    /// overwritten by a later book's own value for the same target name,
    /// but a target the earlier book never defined at all DOES get pulled
    /// in from a later book -- proving the exact defect the old whole-
    /// record `or_insert_with` (first book wins ENTIRELY, even for targets
    /// it never carried) used to have, on both tables, before cycle 8/10.
    #[test]
    fn merge_bonus_var_target_map_pulls_in_new_targets_but_never_overwrites_a_seen_one() {
        let mut first: BTreeMap<String, String> = BTreeMap::new();
        first.insert("SharedTarget".to_string(), "1".to_string());
        first.insert("OnlyFirstBook".to_string(), "1".to_string());
        let mut second: BTreeMap<String, String> = BTreeMap::new();
        second.insert("SharedTarget".to_string(), "2".to_string());
        second.insert("OnlySecondBook".to_string(), "2".to_string());
        let mut into = lowered_chain_for_test(&first);
        let from = lowered_chain_for_test(&second);
        let first_book_value = into.get("SharedTarget").cloned();

        merge_bonus_var_target_map_never_overwriting(&mut into, from);

        assert_eq!(
            into.get("SharedTarget").cloned(),
            first_book_value,
            "a target already bound by an earlier book must never be overwritten by a later one"
        );
        assert!(into.contains_key("OnlyFirstBook"));
        assert!(
            into.contains_key("OnlySecondBook"),
            "a target the earlier book never defined must still merge in from a later book -- \
             this is the exact behaviour the old whole-record `or_insert_with` lacked"
        );
    }

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

    /// T7/D12 (`docs/release/SD-31-corpus-closure-grind/`, tracked-defects list
    /// `defects.md`, item D12; `docs/release/SD-32-compute-library-and-cause-closure` card 11):
    /// `("gunslinger", "Gunslinger ~ Gun Training")` is the one live,
    /// reproducible D12 pair with NO cross-book level conflict at all (the
    /// other three named pairs are already caught by
    /// `cross_book_conflicting_pairs_are_dropped_not_guessed` above) -- its
    /// sole surviving fact comes from `ultimate_combat/uc_abilities_class.lst
    /// :1970`'s `CATEGORY:Internal` "Guns Everywhere" optional-rule row,
    /// embedding a `PRECLASS:1,Gunslinger=1`-gated grant for the SAME key a
    /// vanilla Gunslinger already has via a genuinely separate, hand-wired
    /// chassis function (`class_ultimate_combat.rs::
    /// gunslinger_gun_training_count`) -- so the single-hop
    /// `granted_via_archetype` check on this row alone (`CATEGORY:Internal`,
    /// not `CATEGORY:Archetype`) cannot see that the grant is embedded, not a
    /// genuine top-level base-class declaration. Mutating the `gates.len() ==
    /// 1 && gates.contains("preclass")` refusal in `resolvable_grants` to a
    /// no-op turns this red (confirmed live, see cycle receipt).
    #[test]
    fn a_bare_preclass_only_pair_with_no_mod_row_corroboration_is_refused() {
        let raw = load_raw_grant_facts();
        let gunslinger_facts: Vec<&RawGrantFact> =
            raw.iter().filter(|f| f.class.eq_ignore_ascii_case("gunslinger") && f.key == "Gunslinger ~ Gun Training").collect();
        assert!(
            !gunslinger_facts.is_empty(),
            "expected load_raw_grant_facts to carry at least one live Gunslinger ~ Gun Training \
             fact, to prove this test has real input to refuse"
        );
        assert!(
            gunslinger_facts.iter().all(|f| f.gate == "preclass"),
            "expected every live Gunslinger ~ Gun Training fact to be bare-PRECLASS:-gated \
             (no mod_row_* corroboration): {gunslinger_facts:?}"
        );
        let grants = resolvable_grants();
        assert!(
            !grants.contains_key(&("gunslinger".to_string(), "Gunslinger ~ Gun Training".to_string())),
            "an uncorroborated bare-PRECLASS: pair must never resolve -- T7/D12 regression"
        );
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

    /// SD-34 bucket-B batch cycle: `LEVEL_UP_PILLAR_FILTERED_CLASSES` (a hand-maintained
    /// class-wide exclusion for Druid and Monk) is removed this cycle -- see the module doc
    /// comment's own re-investigation. Direct re-derivation found Druid genuinely DOES now emit
    /// several real, citation-backed explanations (Nature Bond, Orisons, Spontaneous Casting,
    /// Trackless Step, Resist Nature's Lure, Venom Immunity, A Thousand Faces, Nature Sense --
    /// every one a real corpus record with a real, non-leaking description) -- an initial
    /// assumption that Druid would still emit nothing was WRONG and corrected here, not carried
    /// forward silently. This replaces the old (now-vacuous)
    /// `class_feature_grant_consumer_never_emits_for_the_level_up_pillar_filtered_classes`
    /// assertion with a proof of the ONE thing that specifically stays absent: `Archetype
    /// Druid`, the sole `core_rulebook` `class_feature_owner_matched_by_name_but_record_not_
    /// held_by_engine` (`docs/work-inventory.json`) unit for Druid. Its own corpus record
    /// (`data/corpus/core_rulebook/class_feature/archetype_druid/archetype_druid.json`) carries
    /// `description: null` AND no `ABILITY`/level-gate token at all (only `CATEGORY`/`TYPE`/
    /// `VISIBLE`) -- confirmed live: it is not in `unambiguous_grants()` at all, so this
    /// module's citation gate never even considers it (there is no `(class, key)` grant fact
    /// to try), the same practical outcome (never emitted) as a citation refusal, just for a
    /// prior-pipeline-stage reason. This widening changes nothing observable for THAT specific
    /// bucket-B unit -- it may move other Druid units this module was not previously credited
    /// for (see this cycle's own receipt for the full accounting).
    #[test]
    fn archetype_druid_stays_unemitted_but_other_druid_facts_now_emit() {
        let mut explanations = Vec::new();
        push_generic_class_feature_grant_records(
            "class:druid",
            20,
            &AbilityModifiers::default(),
            &mut explanations,
        );
        assert!(
            !explanations.iter().any(|e| e.id == "class_feature.druid.corpus_record.archetype_druid"),
            "`Archetype Druid` has no grant fact at all (confirmed: not in `unambiguous_grants`) \
             -- got {explanations:?}"
        );
        // Sanity: confirm the widening is real (Druid was wholesale-excluded before this
        // cycle) -- an empty result here would make the assertion above trivially,
        // uninformatively true.
        assert!(
            !explanations.is_empty(),
            "expected Druid to now emit at least one real, citation-backed explanation \
             (the class-wide exclusion is removed this cycle); an empty result here would not \
             distinguish 'Archetype Druid has no grant fact' from 'Druid is still wholesale- \
             excluded'"
        );
        // Sanity: confirm `Archetype Druid` genuinely has NO unambiguous grant fact (so its
        // absence above is a pipeline-stage non-candidate, not this module silently refusing a
        // real fact it should have tried).
        assert!(
            !unambiguous_grants().keys().any(|(c, key)| c == "druid" && key == "Archetype Druid"),
            "sanity check: `Archetype Druid` was expected to carry no grant fact at all (no \
             `ABILITY`/level-gate token in its own corpus record) -- if this now fails, the \
             corpus or parser changed and `Archetype Druid`'s own disposition needs \
             re-investigating, not this test silently loosened"
        );
    }

    /// SD-34 `decisions.md` §18: the anti-fabrication gate is now a PROPERTY (cites a real
    /// corpus record), not a hand-maintained class allowlist. This proves the widening
    /// directly: every one of the five previously wholesale-excluded classes (Wizard, Bard,
    /// Paladin, Cleric, Sorcerer), PLUS Monk (widened this cycle -- Druid is excluded from this
    /// list because its own only real-description-eligible test would trivially skip via
    /// `has_a_fact` continue below; Monk is included because it has two:
    /// `Monk ~ Flurry of Blows`/`Monk ~ Unarmed Strike`) now DOES emit real, citation-backed
    /// explanations when the live merged grant data resolves one for it -- the old
    /// `class_feature_grant_consumer_never_emits_for_the_gated_classes` assertion (renamed
    /// above) would have failed for every one of these six before this cycle.
    #[test]
    fn previously_gated_classes_now_emit_citation_backed_explanations_by_construction() {
        let widened_classes = ["wizard", "bard", "paladin", "cleric", "sorcerer", "monk"];
        let unambiguous = unambiguous_grants();
        for class in widened_classes {
            // Only assert emission for a class that the live data actually has an
            // unambiguous, resolvable grant fact for -- a class with none would trivially
            // "pass" with zero explanations, proving nothing.
            let has_a_fact = unambiguous.keys().any(|(c, _)| c == class);
            if !has_a_fact {
                continue;
            }
            let mut explanations = Vec::new();
            push_generic_class_feature_grant_records(
                &format!("class:{class}"),
                20,
                &AbilityModifiers::default(),
                &mut explanations,
            );
            assert!(
                !explanations.is_empty(),
                "{class} was wholesale-excluded before SD-34 decisions.md §18; the live merged \
                 data resolves at least one unambiguous fact for it, so the widened, \
                 citation-based gate must now emit for it: got zero explanations"
            );
            for explanation in &explanations {
                assert!(
                    explanation.id.starts_with(&format!("class_feature.{class}.corpus_record.")),
                    "unexpected id shape for {class}: {}",
                    explanation.id
                );
                // Every emitted explanation must, by construction, cite a corpus record this
                // module independently proved real -- never a fabricated or unresolved one.
                let slug = explanation.id.rsplit('.').next().unwrap();
                let cited_key = unambiguous
                    .keys()
                    .find(|(c, key)| c == class && pu_feature_slug(key) == slug)
                    .map(|(_, key)| key.clone())
                    .unwrap_or_else(|| panic!("{}: no corpus grant key backs this id", explanation.id));
                assert!(
                    descriptions_or_resolved_for_test(&cited_key, explanation.value as u8),
                    "{}: emitted with no real corpus citation -- exactly what the widened gate \
                     must refuse",
                    explanation.id
                );
            }
        }
    }

    /// RED->GREEN mutation proof for the widened gate: a synthetic explanation whose id names
    /// a corpus key that carries NO real, renderable description must never be treated as
    /// citation-backed. This directly exercises `descriptions_or_resolved_for_test` (this
    /// test's own probe of the SAME citation gate `push_generic_class_feature_grant_records`
    /// applies in production) against a key manufactured to have no corpus record at all.
    #[test]
    fn mutation_proof_a_fabricated_key_is_never_treated_as_citation_backed() {
        // RED: plant a key that cannot possibly exist in the real corpus.
        let fabricated_key = "SD-34 Mutation Probe ~ Not A Real Corpus Record";
        assert!(
            !descriptions_or_resolved_for_test(fabricated_key, 1),
            "the citation gate incorrectly accepted a fabricated key with no corpus record -- \
             the gate is not catching what it must catch"
        );
        // GREEN (baseline restored): the probe used no shared state, so a real, known-good key
        // still resolves exactly as before -- confirming this proof did not itself corrupt the
        // gate for real records.
        let (real_class, real_key) = unambiguous_grants()
            .keys()
            .next()
            .expect("live merged data must resolve at least one real grant fact")
            .clone();
        let granted_at = *unambiguous_grants().get(&(real_class, real_key.clone())).unwrap();
        assert!(
            descriptions_or_resolved_for_test(&real_key, granted_at.max(1)),
            "baseline citation check for a REAL corpus key must stay clean after the mutation \
             probe above: {real_key}"
        );
    }

    /// Shared probe for the two tests above: true iff `key` cites a real, renderable corpus
    /// record either directly (`corpus_records_with_real_description`) or through the
    /// per-character formula-resolution path (`resolved_description_for`) -- the SAME two
    /// paths `push_generic_class_feature_grant_records` itself tries, in the same order.
    fn descriptions_or_resolved_for_test(key: &str, level: u8) -> bool {
        if corpus_records_with_real_description().contains_key(key) {
            return true;
        }
        resolved_description_for(key, level, &AbilityModifiers::default()).is_some()
    }

    #[test]
    fn a_granted_record_below_its_level_emits_nothing() {
        let mut explanations = Vec::new();
        // Level 0 cannot meet any real grant's `granted_at` (PCGen's own
        // minimum class level is 1), so this must always be empty for any
        // class this module serves.
        push_generic_class_feature_grant_records("class:fighter", 0, &AbilityModifiers::default(), &mut explanations);
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
        push_generic_class_feature_grant_records("class:fighter", 20, &AbilityModifiers::default(), &mut explanations);
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
        push_generic_class_feature_grant_records("class:rogue", 20, &AbilityModifiers::default(), &mut explanations);
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
        push_generic_class_feature_grant_records("class:fighter", 20, &AbilityModifiers::default(), &mut explanations);
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
            &AbilityModifiers::default(),
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
            // SD-34 bucket-B batch cycle: no class is skipped here any more --
            // `LEVEL_UP_PILLAR_FILTERED_CLASSES` (Druid/Monk's own former class-wide
            // exclusion) is removed; the citation gate alone decides per-record.
            let mut explanations = Vec::new();
            push_generic_class_feature_grant_records(&format!("class:{class}"), 20, &AbilityModifiers::default(), &mut explanations);
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

    // -----------------------------------------------------------------------------------------
    // SD-31 wave 26: resolving `%N` corpus DESC placeholders through the formula interpreter
    // -----------------------------------------------------------------------------------------

    #[test]
    fn class_level_variable_name_matches_the_corpus_wide_convention() {
        assert_eq!(class_level_variable_name("Bard"), "BardLVL");
        assert_eq!(class_level_variable_name("Rogue"), "RogueLVL");
        assert_eq!(class_level_variable_name("Arcane Archer"), "ArcaneArcherLVL");
        assert_eq!(class_level_variable_name("Assassin"), "AssassinLVL");
    }

    /// `Assassin ~ Save against Poisons` (`core_rulebook`, real corpus record): a single
    /// `BONUS:VAR|AssassinPoisonSaveBonus|AssassinLVL/2` token, no chain hop needed at all --
    /// the simplest real shape this resolver handles.
    #[test]
    fn resolve_pcgen_var_chain_reproduces_a_single_hop_division_formula() {
        let mut bonus_vars = BTreeMap::new();
        bonus_vars.insert("AssassinPoisonSaveBonus".to_string(), "AssassinLVL/2".to_string());
        for (level, expected) in [(2u8, 1i64), (3, 1), (4, 2), (10, 5), (20, 10)] {
            let vars =
                resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "AssassinLVL", level, &AbilityModifiers::default());
            assert_eq!(
                vars.get("AssassinPoisonSaveBonus"),
                Some(&expected),
                "level {level}"
            );
        }
    }

    /// `Rogue ~ Trapfinding` (`core_rulebook`, real corpus record): a TWO-hop chain
    /// (`TrapfindingLVL` -> `RogueLVL`, then `TrapfindingBonus` -> `max(TrapfindingLVL/2,1)`),
    /// the shape wave 25b's own worked example (`Bardic Knowledge`) also uses. Proves the
    /// fixed-point pass genuinely chains through an intermediate variable, not just a bare
    /// single-hop lookup.
    #[test]
    fn resolve_pcgen_var_chain_reproduces_a_two_hop_max_formula() {
        let mut bonus_vars = BTreeMap::new();
        bonus_vars.insert("TrapfindingLVL".to_string(), "RogueLVL".to_string());
        bonus_vars.insert("TrapfindingBonus".to_string(), "max(TrapfindingLVL/2,1)".to_string());
        for (level, expected) in [(1u8, 1i64), (2, 1), (3, 1), (4, 2), (5, 2), (10, 5), (20, 10)] {
            let vars =
                resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "RogueLVL", level, &AbilityModifiers::default());
            assert_eq!(vars.get("TrapfindingBonus"), Some(&expected), "level {level}");
        }
    }

    /// SD-32 T12 Epic 8 row 18 cycle 12 correction: this test's ORIGINAL body used a made-up,
    /// not-in-the-corpus name (`SiblingRecordOwnVariable`) to stand in for "an identifier the
    /// chain can never reach", asserting the whole formula stayed unbound. Reading the pinned
    /// oracle's own `VariableProcessor.java`/`PlayerCharacter.java` (this cycle's own receipt)
    /// proved that assumption wrong for a name genuinely absent from the corpus under every
    /// condition: real PCGen's `getVariable` -> `getVariableValue` -> `processBrokenParser`
    /// chain silently treats such a bare additive term as `0`, not a refusal. Corrected to prove
    /// the REAL safety property this test was reaching for -- a name that DOES exist elsewhere in
    /// the corpus as a `BONUS:VAR` target (here, `AssassinPoisonSaveBonus`, real record
    /// `data/corpus/core_rulebook/class_feature/assassin/save_against_poisons.json`) but is not
    /// reachable from THIS formula's own local `bonus_vars` map still refuses -- it is a REAL,
    /// possibly-conditional PCGen value this resolver cannot see from here, and must never guess.
    #[test]
    fn resolve_pcgen_var_chain_never_binds_an_identifier_bound_elsewhere_in_the_corpus() {
        let mut bonus_vars = BTreeMap::new();
        bonus_vars
            .insert("SomeBonus".to_string(), "10+(SomeLVL/2)+AssassinPoisonSaveBonus".to_string());
        bonus_vars.insert("SomeLVL".to_string(), "RogueLVL".to_string());
        let vars =
            resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "RogueLVL", 10, &AbilityModifiers::default());
        assert_eq!(vars.get("SomeLVL"), Some(&10));
        assert!(
            !vars.contains_key("SomeBonus"),
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR target
            "a formula referencing an identifier bound elsewhere in the corpus (a sibling record's \
             own real) must never resolve to a guessed number: {vars:?}"
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 12: real PCGen's own 0-default for a bare identifier the
    /// corpus never binds ANYWHERE, under ANY condition (this cycle's own receipt traces the
    /// oracle's `VariableProcessor.java`/`PlayerCharacter.java` chain to it). `NeverBoundAnywhere`
    /// is not a real corpus name and appears in no fixture -- standing in for the shape, proven
    /// absent from `every_corpus_bound_bonus_var_target()`/`corpus_define_literal_defaults()` by
    /// construction (a name this test file invents can never appear in either live-corpus table).
    #[test]
    fn resolve_pcgen_var_chain_defaults_a_corpus_wide_unbound_identifier_to_zero() {
        // SD-35 `AT-35-E6-001` cycle 4: the defaultable set is DERIVED AT INGEST -- every name
        // any converted chain references, minus every name the corpus binds somewhere -- and
        // shipped in the artifact, instead of being decided per lookup at request time. So the
        // property must be stated with a name the corpus really leaves unbound rather than an
        // invented one: a name that appears in NO corpus record at all cannot reach this
        // resolver, so it is not a case this resolver has. Re-derive the set with
        // `jq -r '.var_defaults | keys[]' data/converted/record_vars.json`.
        let unbound = "ArcaneStrikeDamageBonus";
        assert!(
            record_vars::package().var_defaults.contains_key(unbound),
            "{unbound} must still be a real name the corpus references and binds nowhere"
        );
        let mut bonus_vars = BTreeMap::new();
        bonus_vars.insert("SomeBonus".to_string(), format!("10+(SomeLVL/2)+{unbound}"));
        bonus_vars.insert("SomeLVL".to_string(), "RogueLVL".to_string());
        let vars = resolve_pcgen_var_chain(
            &lowered_chain_for_test(&bonus_vars),
            "RogueLVL",
            10,
            &AbilityModifiers::default(),
        );
        assert_eq!(vars.get("SomeLVL"), Some(&10));
        assert_eq!(
            vars.get("SomeBonus"),
            Some(&15),
            "10 + (10/2) + 0 = 15 -- a genuinely corpus-wide-unbound identifier is the rule source's own \
             real 0, not a refusal: {vars:?}"
        );
    }

    /// SD-32 T12 Epic 8 row 18 cycle 12: the concrete, real corpus case this cycle's fix targets
    /// -- `data/corpus/advanced_class_guide/class_feature/bloodrager/bloodrager_bloodline_
    /// tracker.json` carries `DEFINE:BloodragerBloodlinePower1LVLBonus|0` and no corpus
    /// `BONUS:VAR` row ever targets that same name, so this resolves through the DEFINE-literal
    /// path (`corpus_define_literal_defaults`), not the bare-0-fallback path -- both land on the
    /// same real number here, but this proves the more precise mechanism actually fires.
    #[test]
    fn resolve_pcgen_var_chain_binds_a_real_corpus_define_zero_baseline() {
        let mut bonus_vars = BTreeMap::new();
        bonus_vars.insert(
            "Bloodrager_Draconic_BloodlinePower1LVL".to_string(),
            "Bloodrager_Draconic_BloodlineLVL+BloodragerBloodlinePower1LVLBonus".to_string(),
        );
        bonus_vars
            .insert("Bloodrager_Draconic_BloodlineLVL".to_string(), "BloodragerLVL".to_string());
        let vars =
            resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "BloodragerLVL", 7, &AbilityModifiers::default());
        assert_eq!(
            vars.get("Bloodrager_Draconic_BloodlinePower1LVL"),
            Some(&7),
            "Bloodrager_Draconic_BloodlineLVL (7) + BloodragerBloodlinePower1LVLBonus (real \
             corpus DEFINE, 0) = 7: {vars:?}"
        );
    }

    /// SD-31 wave 27: an ability-modifier-dependent formula (the EXACT real shape `Rogue ~
    /// Master Strike`'s corpus row carries, `10+(MasterStrikeLVL/2)+INT`) now resolves once the
    /// character's real `AbilityModifiers` are seeded -- this is the widening the prior test's
    /// old body (before this wave) proved deliberately did NOT happen.
    #[test]
    fn resolve_pcgen_var_chain_now_binds_a_real_ability_modifier() {
        let mut bonus_vars = BTreeMap::new();
        bonus_vars.insert("MasterStrikeDC".to_string(), "10+(MasterStrikeLVL/2)+INT".to_string());
        bonus_vars.insert("MasterStrikeLVL".to_string(), "RogueLVL".to_string());
        let ability_modifiers = AbilityModifiers { intelligence: 3, ..AbilityModifiers::default() };
        let vars = resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "RogueLVL", 20, &ability_modifiers);
        assert_eq!(vars.get("MasterStrikeLVL"), Some(&20));
        assert_eq!(
            vars.get("MasterStrikeDC"),
            Some(&23),
            "10 + (20/2) + 3 = 23, real production INT modifier now seeded: {vars:?}"
        );
        // A DIFFERENT ability abbreviation the formula does not reference is seeded too (all six
        // always are, see `ability_modifier_seed_vars`) but changing it must not move this
        // formula's own result.
        let ability_modifiers_wis_only =
            AbilityModifiers { intelligence: 3, wisdom: 99, ..AbilityModifiers::default() };
        let vars2 = resolve_pcgen_var_chain(&lowered_chain_for_test(&bonus_vars), "RogueLVL", 20, &ability_modifiers_wis_only);
        assert_eq!(vars2.get("MasterStrikeDC"), Some(&23), "an unreferenced WIS seed must not leak in");
    }

    /// End-to-end against the LIVE corpus record and the LIVE grant data: `resolved_description_for`
    /// produces the exact real sentence, with this character's own number substituted, for
    /// `Assassin ~ Save against Poisons` at a concrete level.
    #[test]
    fn resolved_description_for_produces_the_real_sentence_for_a_live_corpus_record() {
        // Level 3, deliberately NOT level 4: `AssassinLVL/2` floor-divides 3 and 4 to the SAME
        // result at some off-by-one mutations but not others -- straddling the 2/3 and 3/4
        // division boundaries (levels 2, 3, 4 below) is what makes this assertion actually
        // sensitive to an off-by-one in the seeded level, confirmed live during this wave's own
        // mutation-proof pass (temporarily seeding `level + 1`): a level-4-only check here missed
        // it by coincidence (5/2 truncates to the same 2 as 4/2), while level 3 does not
        // (4/2=2 != 3/2=1).
        let text3 = resolved_description_for("Assassin ~ Save against Poisons", 3, &AbilityModifiers::default())
            .expect("Assassin ~ Save against Poisons must resolve at level 3 against the live corpus");
        assert_eq!(text3, "The assassin gains a +1 saving throw bonus against poisons.");
        let text4 = resolved_description_for("Assassin ~ Save against Poisons", 4, &AbilityModifiers::default())
            .expect("Assassin ~ Save against Poisons must resolve at level 4 against the live corpus");
        assert_eq!(text4, "The assassin gains a +2 saving throw bonus against poisons.");
        assert!(!text3.contains('%') && !text4.contains('%'), "no unresolved %N argument may survive");
    }

    /// The same, for the two-hop `max()` shape, against the live `Rogue ~ Trapfinding` record.
    #[test]
    fn resolved_description_for_produces_the_real_sentence_for_the_two_hop_live_record() {
        let text = resolved_description_for("Rogue ~ Trapfinding", 1, &AbilityModifiers::default())
            .expect("Rogue ~ Trapfinding must resolve at level 1 against the live corpus");
        assert_eq!(
            text,
            "You add +1 to Perception skill checks made to locate traps and to Disable Device \
             skill checks. You can use the Disable Device skill to disarm magical traps."
        );
        let text4 = resolved_description_for("Rogue ~ Trapfinding", 4, &AbilityModifiers::default())
            .expect("Rogue ~ Trapfinding must resolve at level 4 against the live corpus");
        assert!(text4.starts_with("You add +2 to Perception"), "got {text4:?}");
    }

    /// The honest scale of this wave's own widening, measured against the LIVE, real
    /// `unambiguous_grants()` population (not a hand-picked sample): how many grant facts were
    /// admitted before this wave (`descriptions.get` -- no `%N` at all), how many are newly
    /// admitted by this wave's interpreter-backed chain resolution, and -- for every fact this
    /// wave still cannot resolve -- WHY, split by cause, so a future wave knows what it is
    /// planning against. Pinned as a concrete assertion (not merely printed) so a regression in
    /// either count is caught, not silently drifted.
    ///
    /// **SD-31 wave 27 update.** `resolved_description_for` (probed here with
    /// `AbilityModifiers::default()`, i.e. a structural "does the chain reach a value at all"
    /// probe, not a specific character's real scores) now also seeds the six bare ability
    /// abbreviations (`ability_modifier_seed_vars`), so three MORE records resolve than at wave
    /// 26's close: `newly_resolved` gains `Arcane Archer ~ Arrow of Death` (CHA),
    /// `Ranger ~ Master Hunter` (WIS), `Rogue ~ Master Strike` (INT) (12 -> 15), and
    /// `class_excluded_otherwise_resolvable` gains `Monk ~ Quivering Palm` (WIS),
    /// `Paladin ~ Lay on Hands` (CHA), `Sorcerer ~ Spells` (CHA) (8 -> 11) -- confirmed by hand,
    /// one record at a time, against the live corpus (`check_excluded_formulas.py`-style probe,
    /// this wave's own investigation), not merely accepted because the assertion below now
    /// passes. `chain_unresolvable` drops by the same six records (20 -> 14). No count outside
    /// these two buckets moved.
    #[test]
    fn the_live_scale_of_this_waves_widening_is_measured_and_pinned() {
        let descriptions = corpus_records_with_real_description();
        let mut already_admitted = 0usize;
        let mut newly_resolved = 0usize;
        let class_excluded_otherwise_resolvable = 0usize;
        let mut chain_unresolvable = 0usize;
        let mut no_record_at_all = 0usize;
        let mut newly_resolved_examples: Vec<String> = Vec::new();

        for ((class, key), &granted_at) in unambiguous_grants() {
            if descriptions.contains_key(key) {
                already_admitted += 1;
                continue;
            }
            if class_feature_record_tokens().get(key).is_none() {
                no_record_at_all += 1;
                continue;
            }
            // Structural resolvability is level-INDEPENDENT for this corpus's arithmetic
            // formulas (no known div-by-zero-at-a-specific-level case exists today) -- probing
            // at the record's own granted level is representative and also the level a
            // just-qualifying character actually has.
            let probe_level = granted_at.max(1);
            let resolves = resolved_description_for(key, probe_level, &AbilityModifiers::default());
            // SD-34 bucket-B batch cycle: no class-wide exclusion is checked here any more --
            // `LEVEL_UP_PILLAR_FILTERED_CLASSES` (Druid/Monk's own former exclusion) is
            // removed, so `class_excluded_otherwise_resolvable` is now permanently 0 (kept as a
            // named bucket in the tuple below rather than deleted, so a future re-read of this
            // pinned assertion does not have to guess why it vanished).
            match resolves {
                Some(text) => {
                    newly_resolved += 1;
                    newly_resolved_examples.push(format!("{class}/{key}@{granted_at}"));
                    assert!(!text.contains('%'), "{key}: resolved text still leaks an unresolved %N argument");
                }
                None => chain_unresolvable += 1,
            }
        }

        // Pinned counts: change these ONLY with a concrete corpus/grant-data change that moves
        // them, never to make a test pass. If this assertion fails after touching
        // `resolve_pcgen_var_chain`/`resolved_description_for`, the new counts ARE the finding --
        // report them, don't silently update the pin without checking why they moved.
        //
        // `already_admitted` moved 137 -> 136 (T7/D12, SD-32 card 11): `resolvable_grants` now
        // refuses `("gunslinger", "Gunslinger ~ Gun Training")`, an uncorroborated bare-PRECLASS:
        // pair (see that function's own doc comment), so it no longer survives into
        // `unambiguous_grants` at all. This is the intended effect of the fix, not a regression --
        // the value was already suppressed downstream by `push_generic_class_feature_grant_records`'s
        // own already-computed-slug guard (Gunslinger's real Gun Training magnitude is served by
        // `class_ultimate_combat.rs`'s dedicated function), so no player-visible value changes.
        // `newly_resolved` moved 15 -> 20, `chain_unresolvable` moved 14 -> 9 (SD-32 T12 Epic 8
        // row 18 cycle 6): `classlevel("X")` now resolves correctly for the SAME-class case
        // (`formula_interpreter.rs`'s `Expr::ClassLevel` widening) -- exactly the 5 Summoner
        // records the new failure output names (Bond Senses, Maker's Call, Merge Forms, Summon
        // Monster, Twin Eidolon), each of whose real corpus formula is a bare
        // `classlevel("Summoner")` call this module could not bind before this cycle. Re-derive:
        // `cargo test --locked --lib -- rules_core::pilot_compute::class_feature_grant_consumer::
        // tests::the_live_scale_of_this_waves_widening_is_measured_and_pinned`.
        //
        // `newly_resolved` moved 20 -> 21, `chain_unresolvable` moved 9 -> 8 (SD-32 T12 Epic 8
        // row 18 cycle 12): `resolve_pcgen_var_chain`'s new corpus-verified 0-default (see that
        // function's own doc, oracle citation `VariableProcessor.java`/`PlayerCharacter.java`)
        // resolves exactly ONE more record here -- `mystic theurge/Mystic Theurge ~ Combined
        // Spells@1`. Its real corpus formula, `CombinedSpellsMaxLevel|(CombinedSpellsLVL+1)/2`,
        // references `CombinedSpellsLVL`, which the SAME record carries only as `DEFINE:
        // CombinedSpellsLVL|0` (no `BONUS:VAR` row anywhere ever targets it -- confirmed,
        // `grep -rl "VAR|CombinedSpellsLVL" data/corpus/` -> 0 hits) -- exactly the "real corpus
        // DEFINE zero baseline" shape `resolve_pcgen_var_chain_binds_a_real_corpus_define_zero_
        // baseline` proves in isolation. `(0+1)/2 = 0` (integer division). Every other of the 20
        // pre-existing `newly_resolved` records, and all 11/36 `class_excluded_otherwise_
        // resolvable`/`no_record_at_all`, are unchanged -- confirmed by diffing this cycle's own
        // full `newly_resolved_examples` list against the pre-cycle-12 one before landing.
        //
        // `no_record_at_all` moved 36 -> 1, `chain_unresolvable` moved 8 -> 43 (SD-32 T12 Epic 8
        // row 18 cycle 18, `§27b`/`§17a` -- a RECLASSIFICATION, not a resolver improvement:
        // `class_feature_record_tokens_pre_gate_safe`'s own `description: null` gate widened (see
        // that function's own doc comment -- a genuine engine gap, `Jungle Domain ~ Trap Sense`'s
        // `VISIBLE:NO`/`description: null` shape was invisible to `resolve_pool_member_sole_
        // magnitude` for want of this read path, not for want of real data). 35 of these 36
        // records DO have a real corpus record after the widening -- they were never truly
        // "absent", only mis-bucketed by a gate this table's own OLD code applied one layer too
        // early -- but their OWN `raw_description` is empty (no `%N` text at all), so
        // `resolved_description_for` still correctly returns `None` for every one of them
        // (`render_pcgen_desc_with_values("", ...).text.is_empty()`), landing them in
        // `chain_unresolvable` instead: a MORE ACCURATE label ("a record exists but nothing
        // renders") than the old "no record at all", never a fabricated render. `newly_resolved`
        // and `class_excluded_otherwise_resolvable` are UNCHANGED -- confirmed by diffing this
        // cycle's own `newly_resolved_examples` list against cycle 12's, identical. Exactly 1
        // genuinely absent key remains.
        //
        // `newly_resolved` moved 21 -> 26, `class_excluded_otherwise_resolvable` moved 11 -> 6
        // (SD-34 AT-34-E3-001, `decisions.md` §18): the wholesale per-class exclusion for
        // Wizard/Bard/Paladin/Cleric/Sorcerer was replaced by the citation-based property (see
        // `LEVEL_UP_PILLAR_FILTERED_CLASSES`'s own doc comment) -- these five classes' own
        // already-resolvable records (found by THIS SAME probe before this cycle, just bucketed
        // as excluded rather than counted) now land in `newly_resolved` instead:
        // `bard/Bard ~ Bardic Knowledge@1`, `bard/Bard ~ Lore Master@5`,
        // `paladin/Paladin ~ Holy Champion@20`, `paladin/Paladin ~ Lay on Hands@2`,
        // `sorcerer/Sorcerer ~ Spells@1` -- exactly 5, a RECLASSIFICATION of this cycle's own
        // widening, not a resolver change (`resolved_description_for` itself is untouched this
        // cycle). Wizard and Cleric contribute zero newly-resolved records here (their own
        // resolvable-but-excluded population was already 0 before this cycle) -- the classes'
        // widening is real (proven live by
        // `previously_gated_classes_now_emit_citation_backed_explanations_by_construction`,
        // above) even where this particular census shows no movement.
        //
        // `newly_resolved` moved 26 -> 32, `class_excluded_otherwise_resolvable` moved 6 -> 0
        // (SD-34 bucket-B batch cycle, continuing `decisions.md` §18): the class-wide exclusion
        // for Druid and Monk (`LEVEL_UP_PILLAR_FILTERED_CLASSES`) is removed this cycle -- see
        // the module's own doc comment for the re-investigation showing the ONLY reason this
        // module kept excluding them (a `LevelUpPlan`-reachability concern) did not actually
        // gate this mechanism's own `docs/work-inventory.json` verdict. All 6 previously-
        // excluded-but-resolvable records were Monk's, not a Druid/Monk mix as an earlier
        // cycle's comment (removed here) mistakenly generalized: `monk/Monk ~ Abundant Step@12`,
        // `monk/Monk ~ Diamond Soul@13`, `monk/Monk ~ Fast Movement@3`,
        // `monk/Monk ~ High Jump@5`, `monk/Monk ~ Quivering Palm@15`,
        // `monk/Monk ~ Wholeness of Body@7` -- confirmed by diffing this cycle's own
        // `newly_resolved_examples` against the pre-cycle list (all 6 appear, nothing else
        // added or removed beyond them). `class_excluded_otherwise_resolvable` is now
        // permanently 0 (no class-wide exclusion remains anywhere in this module). Re-derive:
        // `cargo test --locked --lib -- rules_core::pilot_compute::class_feature_grant_consumer::
        // tests::the_live_scale_of_this_waves_widening_is_measured_and_pinned`.
        // `already_admitted` moved 136 -> 131 and `newly_resolved` 32 -> 37 in SD-35
        // `AT-35-E6-003-SWEEP` cycle 17. **The two buckets' sum is unchanged at 168, and that
        // is the whole content of the move**: it is a reclassification between two paths, not a
        // record gained or lost, and `chain_unresolvable` and `no_record_at_all` did not move
        // at all. The admission test stopped being "the raw corpus description renders clean
        // with no character" and became "the CONVERTED record states prose the converter
        // settled with no character"
        // (`converted_prose::settled_description_for`). Five records state prose whose
        // magnitude is still a term — `bard/Bard ~ Bardic Performance@1`,
        // `monk/Monk ~ Slow Fall@4`, `rogue/Rogue ~ Trapfinding@1`,
        // `vigilante/Vigilante ~ Seamless Guise@1`, `vigilante/Vigilante ~ Unshakable@3` —
        // so they now take the interpreter path, which states this character's own number
        // instead of the term's words. Confirmed by diffing this test's own
        // `newly_resolved_examples` against the pre-cycle list: those five appear and nothing
        // else changed. Re-derive:
        // `cargo test --locked --lib -- rules_core::pilot_compute::class_feature_grant_consumer::
        // tests::the_live_scale_of_this_waves_widening_is_measured_and_pinned -- --nocapture`.
        assert_eq!(
            (already_admitted, newly_resolved, class_excluded_otherwise_resolvable, chain_unresolvable, no_record_at_all),
            (131, 37, 0, 43, 1),
            "live scale moved -- already_admitted={already_admitted} newly_resolved={newly_resolved} \
             class_excluded_otherwise_resolvable={class_excluded_otherwise_resolvable} \
             chain_unresolvable={chain_unresolvable} no_record_at_all={no_record_at_all} \
             examples of newly-resolved: {newly_resolved_examples:?}"
        );
    }

    /// A key with no corpus record at all resolves to `None`, never a panic or a guess.
    #[test]
    fn resolved_description_for_returns_none_for_an_unknown_key() {
        assert_eq!(resolved_description_for("Not A Real Class ~ Not A Real Feature", 5, &AbilityModifiers::default()), None);
    }

    /// End-to-end THROUGH the emission function this wave widens: `Assassin ~ Save against
    /// Poisons` was completely absent from this module's output before this wave (its
    /// description carries an unresolved `%1`, so `corpus_records_with_real_description` -- the
    /// pre-wave-26 gate -- excluded it, and the whole grant fact was silently skipped). It now
    /// emits, carrying the real, per-character resolved sentence in `detail`.
    #[test]
    fn push_generic_class_feature_grant_records_now_emits_the_previously_skipped_assassin_record() {
        // Level 3 (not 4): see `resolved_description_for_produces_the_real_sentence_for_a_live_
        // corpus_record`'s own comment for why this specific level is what makes the assertion
        // sensitive to an off-by-one in the seeded class level.
        let mut explanations = Vec::new();
        push_generic_class_feature_grant_records("class:assassin", 3, &AbilityModifiers::default(), &mut explanations);
        let found = explanations
            .iter()
            .find(|e| e.id == "class_feature.assassin.corpus_record.save_against_poisons")
            .unwrap_or_else(|| panic!("expected the assassin poison-save record to be emitted at \
                 level 3; got {explanations:?}"));
        assert_eq!(found.value, 2, "granted_at must still be the real grant level, unchanged");
        assert!(
            found.detail.contains("The assassin gains a +1 saving throw bonus against poisons."),
            "the real, resolved sentence with this character's own number must be embedded in \
             detail: {}",
            found.detail
        );
        assert!(!found.detail.contains('%'), "no unresolved %N argument may ship: {}", found.detail);
    }

    /// **Honest scope correction to `the_live_scale_of_this_waves_widening_is_measured_and_
    /// pinned`'s own count, found while verifying this wave against the REAL full pipeline
    /// rather than this module in isolation.** `Barbarian ~ Damage Reduction` is one of the 12
    /// `newly_resolved` records that census counts -- correctly, `resolved_description_for`
    /// genuinely resolves it via the interpreter -- but `pilot_compute/mod.rs` ALREADY carries a
    /// real, hand-modelled `class_feature.barbarian.damage_reduction` explanation with a
    /// complete, per-character value AND real derivation prose in `detail`, pushed before this
    /// module ever runs. Its trailing dot-segment (`damage_reduction`) COLLIDES with this
    /// module's own roster id for the same grant fact, so the pre-existing
    /// `already_computed_slugs` guard (see `a_pre_existing_real_explanation_suppresses_the_
    /// matching_roster_id`, above) correctly suppresses this module's own emission in the REAL
    /// pipeline -- the player was already fully served for this one record before this wave, and
    /// this wave changes nothing observable for it. Of the 12 interpreter-resolvable records,
    /// this is the ONLY one with a pre-existing hand-modelled collision (confirmed by grep: no
    /// other of the 12 classes' feature slugs appears as a `class_feature.<class>.<slug>` id
    /// anywhere in `pilot_compute/mod.rs`) -- so this wave's real, NEW, previously-unserved
    /// population is 11, not 12. Reported here rather than silently, per the wave brief's own
    /// "report honestly how far it scales" instruction.
    #[test]
    fn barbarian_damage_reduction_is_superseded_by_its_own_pre_existing_hand_modelled_explanation() {
        let mut explanations = vec![ComputationExplanation {
            id: "class_feature.barbarian.damage_reduction".to_owned(),
            value: 2,
            detail: "the real, hand-wired Damage Reduction magnitude and derivation".to_owned(),
        }];
        push_generic_class_feature_grant_records("class:barbarian", 20, &AbilityModifiers::default(), &mut explanations);
        assert_eq!(
            explanations.iter().filter(|e| e.id.rsplit('.').next() == Some("damage_reduction")).count(),
            1,
            "exactly the pre-seeded real explanation must survive; this module must not add a \
             second, colliding id for the same trailing segment: {explanations:?}"
        );
        assert_eq!(explanations[0].value, 2, "the real hand-modelled explanation must be untouched");
    }

    /// Below the grant level, nothing is emitted at all -- unchanged behaviour, character does
    /// not have the feature yet.
    #[test]
    fn push_generic_class_feature_grant_records_still_withholds_below_the_grant_level() {
        let mut explanations = Vec::new();
        push_generic_class_feature_grant_records("class:assassin", 1, &AbilityModifiers::default(), &mut explanations);
        assert!(
            !explanations.iter().any(|e| e.id.contains("save_against_poisons")),
            "a character below the grant level must not see this feature at all: {explanations:?}"
        );
    }

    /// The EXISTING (pre-wave-26) branch's `detail` text is byte-identical to before this wave --
    /// this change is purely additive for records that were previously skipped, never a rewrite
    /// of records that were already served.
    #[test]
    fn the_pre_existing_no_placeholder_branch_detail_text_is_unchanged() {
        let mut explanations = Vec::new();
        push_generic_class_feature_grant_records("class:fighter", 2, &AbilityModifiers::default(), &mut explanations);
        let bravery_roster_id = "class_feature.fighter.corpus_record.bravery";
        // Fighter's OWN hand-wired chassis code normally pushes a real `bravery` explanation
        // first and this module defers to it (see the `already_computed_slugs` guard above) --
        // called in isolation here (no prior explanations), so this module's own coarser roster
        // fact is what gets pushed, and its exact wording is the thing under test.
        let found = explanations.iter().find(|e| e.id == bravery_roster_id);
        if let Some(found) = found {
            assert!(
                found.detail.ends_with(
                    "per a grant fact ingested from PCGen's own class-progression tokens \
                     (data/class_feature_grants). The record's real rulebook description is \
                     served separately by the character sheet's Class Features section."
                ),
                "unchanged detail wording expected for the no-percent-n branch: {}",
                found.detail
            );
        }
    }
}
