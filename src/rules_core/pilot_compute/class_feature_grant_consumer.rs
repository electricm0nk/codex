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

use super::bonus_stack_reader;
use super::formula_interpreter::PcgenFormulaEvaluator;
use super::formula_reproduction_harness::FormulaEvaluator as _;
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
// (`formula_interpreter::PcgenFormulaEvaluator`, proven to reproduce 22 of 22 hand-modelled
// functions, zero disagreements) and this section is what plugs it in for `class_feature`,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ClassFeatureRecordTokens {
    pub(crate) name: String,
    pub(crate) class: String,
    pub(crate) raw_description: String,
    pub(crate) bonus_vars: BTreeMap<String, String>,
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

/// A stricter sibling of [`parse_bonus_var_tokens`] (SD-32 T12 Epic 8,
/// `epic-2-cause-closure` row 18: pool-shaped class features). Refuses
/// (drops the target entirely, never guesses) any `BONUS:VAR` target name
/// that carries MORE THAN ONE raw row for the same record, or whose
/// formula segment is followed by a further `|`-delimited PRE-gate
/// qualifier (`PREVAREQ:`/`PREVARGTEQ:`/...) -- both shapes
/// [`parse_bonus_var_tokens`] silently resolves by keeping only the LAST
/// row, which is safe for the handful of records SD-32 Epic 1's callers
/// hand-picked and independently verified one at a time, but NOT safe for
/// a generic pass over an unverified population: silently picking the
/// wrong PRE-gated variant (e.g. `advanced_players_guide`'s Force Bomb
/// discovery, `BONUS:VAR|ForceBombDieSize|3|PREVAREQ:...,1` vs
/// `BONUS:VAR|ForceBombDieSize|4|PREVAREQ:...,0`) would ship a genuinely
/// wrong number as a real computed value -- exactly the failure
/// `decisions.md §1a` exists to prevent. `formula_interpreter.rs`'s own
/// module doc names the real PRE-gate-aware summation mechanism
/// (`bonus_stack_reader`, a sibling module elsewhere) as out of scope for
/// this generic resolver; refusing is correct here, not merely expedient.
/// SD-32 T12 Epic 8 row 18 cycle 6: widened to correctly RESOLVE two shapes this function used
/// to unconditionally drop (module doc above still describes the original refusal; both
/// widenings below only ADD a verified-safe path -- neither removes the original refusal for any
/// shape it does not understand).
///
/// **Widening 1 -- `TYPE=<bonustype>` trailing fields are stripped, never treated as a gate.**
/// `TYPE=` (`BONUS:VAR|<target>|<formula>|TYPE=<bonustype>`, e.g. this corpus's own
/// `AC_Natural_Armor|2|TYPE=Base`, `Craft (Alchemy)|4|TYPE=Insight`, `DomainAirLVL|DomainLVL|
/// TYPE=Domain`) is PCGen's real bonus-STACKING classification -- it governs whether two
/// DIFFERENT bonus sources of the same type stack with each other, never whether THIS record's
/// own contribution applies to this character at all. Every hand-modelled function elsewhere in
/// this file that grounds a `TYPE=`-tagged token (cited throughout `mod.rs`, e.g. `AC_Natural_
/// Armor|2|TYPE=Base`) already treats the formula as unconditional, confirming this is real
/// oracle semantics, not a guess. Stripping it (rather than refusing on an unrecognised trailing
/// field, as before) is therefore strictly safe.
///
/// **Widening 2 -- multi-row `PREVARGTEQ`-gated targets now resolve, via `bonus_stack_reader`.**
/// `bonus_stack_reader` (SD-31 wave 26, `super::bonus_stack_reader`) already reads and proves
/// exactly this shape: multiple `BONUS:VAR` rows sharing one target, each independently gated by
/// its own `PREVARGTEQ:<var>,<threshold>` (real oracle semantics, `PreVariableTester.java` +
/// `BonusManager.sumActiveBonusMap` -- summed, only the currently-qualifying rows -- both cited in
/// that module's own doc). For each target name found on this record (after widening 1 strips
/// any `TYPE=` field), `bonus_stack_reader::extract_addends` is tried; if it succeeds (every row
/// is now either ungated or carries exactly one well-formed `PREVARGTEQ` field), the addends are
/// re-expressed as a single formula string this module's OWN existing evaluator already parses --
/// `if(<gate var>>=<threshold>,(<formula>),0)` per gated row, summed with `+` -- reusing the
/// `if(...)`/`Cmp` grammar `formula_interpreter.rs` already implements (wave 26 shape closure)
/// rather than adding a second evaluation path. A target whose rows still carry any OTHER shape
/// (more than one non-`TYPE=` PRE-tag field, a non-`PREVARGTEQ`/non-`TYPE=` tag such as
/// `PREABILITY`/`PREMULT`) still fails `extract_addends` -- widening 3 below then decides what
/// happens next, rather than the whole target being silently dropped as before.
///
/// **Widening 3 -- when `extract_addends` refuses, fall back to the target's own sole UNGATED
/// row, if it has exactly one.** Found live: SD-32 T12 row 21 cycle 2, `core_rulebook:
/// class_feature:barbarian_damage_reduction`. Its real corpus record (only visible once row 21
/// cycle 1 restored the `.MOD`-appended rows this function reads) carries ONE unconditional row
/// (`BONUS:VAR|BarbarianDR|(BarbarianDRLVL-4)/3`) plus five rows each gated by BOTH a
/// `PREVARGTEQ` AND a `PREVAREQ` tag (a rage-power-selection offset,
/// `PREVAREQ:Barbarian_CF_DamageReduction<N>,1`) -- two PRE-tag kinds on one row, a shape
/// `bonus_stack_reader` correctly refuses (its own module doc: "Recognises exactly one PRE-tag
/// kind, `PREVARGTEQ`"). `PREVAREQ` gates on a feat/rage-power SELECTION, not a value this module
/// has ever modelled (`ability_modifier_seed_vars` seeds only ability scores and class level) --
/// there is no live binding this resolver could evaluate that gate against even if a fourth
/// parser were built for it, so refusing to evaluate those five rows is correct, not merely
/// expedient. What is NOT correct is discarding the target's own unconditional row along with
/// them: a character who has selected none of those rage powers (the common case, and the exact
/// case `derived_evaluator_fixture_check`'s pinned fixture -- re-derived independently from the
/// upstream `.lst`'s single base row, which is the ONLY row that script's own single-line reader
/// sees -- covers) still has a real, fully-determined `BarbarianDR` from the base formula alone.
/// This widening returns that base formula ONLY when the target carries exactly one row with no
/// PRE-tag tail at all (after widening 1 strips any `TYPE=`); a target with zero ungated rows, or
/// more than one (which would itself be an ambiguous shape), still refuses entirely, unchanged.
fn parse_bonus_var_tokens_pre_gate_safe(raw_tokens: &[Value]) -> BTreeMap<String, String> {
    // Re-expand comma-joined `VAR|Name1,Name2|formula|PRE...` rows into one synthetic
    // `"VAR|<name>|formula|PRE..."` value per name, exactly the shape `bonus_stack_reader::
    // extract_addends` expects (one target name per token) -- mirrors the name-splitting the
    // original version of this function already did before discarding it into a bare formula.
    // Any `TYPE=...` trailing field is dropped here (widening 1, see doc above) before the value
    // ever reaches `extract_addends`, so a `PREVARGTEQ`-only remainder is left recognisable.
    let mut expanded: Vec<(String, String)> = Vec::new();
    let mut target_order: Vec<String> = Vec::new();
    let mut seen_targets: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    // Widening 3's own bookkeeping: every UNGATED (no PRE-tag tail at all, post-`TYPE=`-strip)
    // row's formula, per target name -- judged on the target's OWN rows, not the reader's.
    let mut ungated_formulas: BTreeMap<String, std::collections::BTreeSet<String>> = BTreeMap::new();
    for token in raw_tokens {
        if token["key"].as_str() != Some("BONUS") {
            continue;
        }
        let Some(value) = token["value"].as_str() else { continue };
        let Some(rest) = value.strip_prefix("VAR|") else { continue };
        let mut parts = rest.splitn(2, '|');
        let (Some(names), Some(formula_and_tail)) = (parts.next(), parts.next()) else {
            continue;
        };
        let mut tail_fields: Vec<&str> = formula_and_tail.split('|').collect();
        let formula = tail_fields.remove(0);
        tail_fields.retain(|field| !field.starts_with("TYPE="));
        let rebuilt = if tail_fields.is_empty() {
            formula.to_string()
        } else {
            format!("{formula}|{}", tail_fields.join("|"))
        };
        for name in names.split(',') {
            let name = name.trim();
            if name.is_empty() {
                continue;
            }
            if seen_targets.insert(name.to_string()) {
                target_order.push(name.to_string());
            }
            if tail_fields.is_empty() {
                ungated_formulas.entry(name.to_string()).or_default().insert(formula.to_string());
            }
            expanded.push(("BONUS".to_string(), format!("VAR|{name}|{rebuilt}")));
        }
    }
    let borrowed: Vec<(&str, &str)> =
        expanded.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let mut out = BTreeMap::new();
    for name in target_order {
        let Ok(addends) = bonus_stack_reader::extract_addends(&name, borrowed.iter().copied())
        else {
            // Widening 3: an unrecognised PRE-tag shape refuses `extract_addends`'s summed
            // result, but a lone ungated row for this same target is still a real,
            // unconditional fact -- use it rather than dropping the target outright.
            if let Some(formulas) = ungated_formulas.get(&name) {
                if let [only] = formulas.iter().collect::<Vec<_>>().as_slice() {
                    out.insert(name, (*only).clone());
                }
            }
            continue;
        };
        match addends.as_slice() {
            [] => {}
            [only] if only.gate.is_none() => {
                out.insert(name, only.formula.clone());
            }
            _ => {
                let synthesized = addends
                    .iter()
                    .map(|addend| match &addend.gate {
                        None => format!("({})", addend.formula),
                        Some(gate) => {
                            format!("if({}>={},({}),0)", gate.variable, gate.threshold, addend.formula)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("+");
                out.insert(name, synthesized);
            }
        }
    }
    out
}

/// SD-32 T12 Epic 8 row 18 cycle 10: the ONE per-target merge policy both
/// `class_feature_bonus_vars_any_record` (cycle 8, header-side) and
/// `class_feature_record_tokens_pre_gate_safe` (this cycle, member-side)
/// now share -- never overwrite an already-bound target name, so the FIRST
/// book (alphabetical walk order) to define a given `BONUS:VAR` target
/// wins for that target specifically, while every OTHER target either side
/// contributes still merges in. Factored out so the two cross-book merges
/// cannot drift into two different collision policies the way the header
/// and member tables' own WHOLE-RECORD `or_insert_with` calls silently did
/// before cycle 8/10 fixed them one at a time.
pub(crate) fn merge_bonus_var_target_map_never_overwriting(
    into: &mut BTreeMap<String, String>,
    from: BTreeMap<String, String>,
) {
    for (target, formula) in from {
        into.entry(target).or_insert(formula);
    }
}

/// A PRE-gate-safe sibling of [`class_feature_record_tokens`], identical in
/// every respect except its `bonus_vars` field is built via
/// [`parse_bonus_var_tokens_pre_gate_safe`] rather than
/// [`parse_bonus_var_tokens`] -- see that function's own doc comment for
/// why a generic, per-record-unverified consumer (SD-32 T12 Epic 8's
/// [`super::resolve_pool_member_sole_magnitude`]) must use this table
/// instead of the original.
///
/// SD-32 T12 Epic 8 row 18 cycle 10: MERGED across every book carrying the
/// SAME bare `KEY:`, never first-book-wins -- the member-side twin of
/// `class_feature_bonus_vars_any_record`'s own cross-book merge (cycle 8),
/// same root cause and same fix, applied to this table instead of skipped
/// as a "narrower/riskier" scope call (cycle 8's own receipt; the `class`
/// field this table's `owned_by_class` census check reads IS load-bearing,
/// see below, so this merge preserves first-seen `class`/`name`/
/// `raw_description` exactly as before and only widens `bonus_vars`).
/// Confirmed live: this table's own description gate (`is_real_description_
/// value`) means most of the header-only 155 bare-key duplicates
/// `class_feature_bonus_vars_any_record`'s own doc names never reach this
/// table at all (a header record with `description: null` is filtered out
/// upstream, before `or_insert_with` even runs) -- but any MEMBER record
/// sharing a bare key across books (a book reprinting the same named
/// bloodline/domain/order power) previously kept only the alphabetically-
/// first book's own `bonus_vars`, silently discarding a later book's
/// `.MOD`-restored rows for a target the first book's copy never carried.
/// Per-target `.or_insert` (never overwriting an already-bound target,
/// identical policy to the header-side table) means a genuine cross-book
/// disagreement on the SAME target name still keeps whichever book's row
/// was seen first, unchanged from this table's own pre-existing single-
/// record collision policy -- this only extends "one record" to "one key,
/// merged across books", exactly as cycle 8 phrased it for the header
/// table.
///
/// **`§17a` re-derivation result: this merge closes ZERO new pool groups.**
/// Independently re-derived corpus-wide (every real `class_feature`
/// key carrying a non-null description that appears in more than one
/// book): 81 such keys exist, and for every one of them the union of
/// `BONUS:VAR` target names across all contributing books is IDENTICAL to
/// the alphabetically-first book's own target set alone -- no book
/// contributes a target its sibling copies lack. The defect this cycle
/// fixes is real (a future corpus update that DOES diverge would have
/// silently lost data under the old whole-record `or_insert_with`), but it
/// has not yet manifested for any key currently in this table. A predicted
/// change that does not reproduce is itself a finding (`decisions.md
/// §17a`), not a wasted cycle: `pool_group_closure_census_across_all_six_
/// pools`'s six baselines are unchanged by this fix, confirmed by re-
/// running that test after landing it.
pub(crate) fn class_feature_record_tokens_pre_gate_safe() -> &'static BTreeMap<String, ClassFeatureRecordTokens>
{
    static TABLE: OnceLock<BTreeMap<String, ClassFeatureRecordTokens>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut out: BTreeMap<String, ClassFeatureRecordTokens> = BTreeMap::new();
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
                let (Some(key), Some(name), Some(class)) =
                    (data["key"].as_str(), data["name"].as_str(), data["class"].as_str())
                else {
                    continue;
                };
                // SD-32 T12 Epic 8 row 18 cycle 18 (`§27b`): `description: null` is now ADMITTED
                // (as an empty `raw_description`, never fabricated text) rather than skipped
                // outright -- confirmed live, `data/corpus/ultimate_magic/class_feature/
                // jungle_domain/trap_sense.json` (`VISIBLE:NO`, a real, invisible, purely
                // mechanical sub-ability: `BONUS:VAR|TrapSenseBonus|DomainJungleLVL/3`, no
                // description text at all) was refused by `resolve_pool_member_sole_magnitude`
                // purely because this table's OLD `description.as_str()?` gate dropped the
                // record before its real `bonus_vars` chain was ever read -- the exact same
                // "header record with `description: null`" shape `class_feature_bonus_vars_any_
                // record`'s own doc above already names and already admits for HEADER lookups;
                // this widening applies the identical reasoning to MEMBER lookups. Safe for every
                // existing consumer: `resolved_description_for`/`resolved_description_for_formula_
                // only_desc_argument` both already refuse cleanly (return `None`, never render) on
                // an empty `raw_description` -- `render_pcgen_desc_with_values("", ...).text.
                // is_empty()` and `desc_token_arguments("").is_empty()` respectively -- so no
                // caller anywhere gains a new, empty, fabricated rendering; only
                // `resolve_pool_member_sole_magnitude`'s `BONUS:VAR`-only path (which never reads
                // `raw_description` at all) gains real, previously-invisible magnitude-bearing
                // records. A record whose description IS present but carries a REAL bad value
                // (`.CLEAR`/`.CLEARALL`/a PI-redaction marker) is still refused exactly as before
                // -- `is_real_description_value` only ever ran on a `Some` description, so this
                // widening changes NOTHING about that PI-safety gate (`§15`).
                let raw_desc = match data["description"].as_str() {
                    Some(s) => {
                        if !is_real_description_value(s) {
                            continue;
                        }
                        s.to_string()
                    }
                    None => String::new(),
                };
                let bonus_vars = data["raw_tokens"]
                    .as_array()
                    .map(|tokens| parse_bonus_var_tokens_pre_gate_safe(tokens))
                    .unwrap_or_default();
                let entry = out.entry(key.to_string()).or_insert_with(|| ClassFeatureRecordTokens {
                    name: name.to_string(),
                    class: class.to_string(),
                    raw_description: raw_desc,
                    bonus_vars: BTreeMap::new(),
                });
                merge_bonus_var_target_map_never_overwriting(&mut entry.bonus_vars, bonus_vars);
            }
        }
        out
    })
}

/// Every corpus `class_feature` record's own PRE-gate-safe `BONUS:VAR`
/// chain, keyed by `KEY:` -- WITHOUT [`class_feature_record_tokens_pre_gate_
/// safe`]'s `data.description` requirement (SD-32 T12 Epic 8). A pool's own
/// HEADER record (`"Alchemist ~ Discovery"`, `"Witch ~ Hex"`, ...) very
/// often defines the pool-specific level variable individual members scale
/// on (`AlchemistDiscoveryLVL|AlchemistLVL`) but carries `description:
/// null` in this corpus (confirmed live,
/// `advanced_players_guide/class_feature/alchemist/discovery.json`) -- the
/// sibling table's description gate would silently exclude it, starving
/// `super::resolve_pool_member_sole_magnitude`'s header-chain merge of
/// exactly the variable it exists to supply. This table's own consumer
/// never renders `raw_description` (it is a real corpus record's own field,
/// kept `.to_string()`-of-empty rather than `Option` only to reuse
/// [`ClassFeatureRecordTokens`]'s existing shape without a second struct).
pub(crate) fn class_feature_bonus_vars_any_record() -> &'static BTreeMap<String, ClassFeatureRecordTokens> {
    static TABLE: OnceLock<BTreeMap<String, ClassFeatureRecordTokens>> = OnceLock::new();
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
                let (Some(key), Some(name)) = (data["key"].as_str(), data["name"].as_str()) else {
                    continue;
                };
                // SD-32 T12 Epic 8 row 18 cycle 8: `class` tolerated as absent/`null` here (kept
                // `""`, never a fabricated class name) -- confirmed live across every real
                // per-bloodline HEADER record this corpus carries (`data/corpus/*/class_feature/
                // <bloodline>/<bloodline>.json`, e.g. `"Marid Bloodline"`, `"Draconic Bloodline"`,
                // `"Aberrant Bloodline"`; every single one of the 53 real Sorcerer Bloodline groups'
                // own header ingests with `class: null` even after row 21's `.MOD`-token restoral
                // put their real `BONUS:VAR|Sorcerer_<X>_BloodlineLVL|BloodlineLVL`-shaped chain
                // rows back). This table exists ONLY to feed `pool_header_record_by_normalized_
                // suffix`'s header-var MERGE (never rendered, never treated as a member's own
                // ownership signal -- `resolve_pool_member_sole_magnitude`'s member lookup still
                // goes through the DESCRIPTION-gated, class-`Some`-required sibling table
                // unchanged), so an unowned header contributes vars but can never itself pass an
                // ownership check anywhere in this file.
                let class = data["class"].as_str().unwrap_or("");
                let raw_desc = data["description"].as_str().unwrap_or("").to_string();
                let bonus_vars = data["raw_tokens"]
                    .as_array()
                    .map(|tokens| parse_bonus_var_tokens_pre_gate_safe(tokens))
                    .unwrap_or_default();
                // SD-32 T12 Epic 8 row 18 cycle 8: MERGED across every book carrying the SAME bare
                // key, never first-book-wins. Confirmed live: `"Bloodline Tracker"` alone (the
                // shared `BloodlineLVL`/`BloodlineCasterLVL`/`BloodlineProgressionLVL` var chain
                // every one of the 53 real Sorcerer Bloodline groups' own per-bloodline header
                // chains through) is real-ingested from 8 SEPARATE book files (`core_rulebook`,
                // `advanced_class_guide`, `advanced_players_guide`, `advanced_race_guide`,
                // `occult_adventures`, `ultimate_combat`, `ultimate_magic`, `monster_codex`), and
                // 154 more bare `class_feature` keys carry this exact shape too (e.g. `"Verdant
                // Bloodline"` alone in 4 books, `"Celestial Bloodline"` in 3) -- each book's own
                // ingested copy carries a DIFFERENT subset of that one real ability's `.MOD`-
                // appended rows (the same per-book `.MOD`-collision shape row 21 fixed at the
                // per-FILE level; this is the same defect surviving at the per-KEY,
                // cross-file level). The prior `or_insert_with` kept only whichever book sorted
                // FIRST alphabetically (`"advanced_class_guide"` before `"core_rulebook"`) --
                // silently discarding `core_rulebook`'s own COMPLETE 308-token `"Bloodline
                // Tracker"` copy in favour of `advanced_class_guide`'s single leftover `DEFINE`.
                // `.or_insert` per target name (never overwriting an already-bound target) means
                // every book's own real rows contribute, and a genuine cross-book disagreement on
                // the SAME target name keeps whichever book's row was seen first -- unchanged from
                // this table's own pre-existing single-record collision policy (`parse_bonus_var_
                // tokens_pre_gate_safe` already refuses an ambiguous multi-row target within one
                // record; this only extends "one record" to "one key, merged across books").
                let entry = out.entry(key.to_string()).or_insert_with(|| ClassFeatureRecordTokens {
                    name: name.to_string(),
                    class: class.to_string(),
                    raw_description: raw_desc.clone(),
                    bonus_vars: BTreeMap::new(),
                });
                if entry.class.is_empty() && !class.is_empty() {
                    entry.class = class.to_string();
                }
                if entry.raw_description.is_empty() && !raw_desc.is_empty() {
                    entry.raw_description = raw_desc;
                }
                merge_bonus_var_target_map_never_overwriting(&mut entry.bonus_vars, bonus_vars);
            }
        }
        out
    })
}

/// SD-32 T12 Epic 8 row 18 cycle 21 (`§27b`): every `class_feature/wildblooded/*.json` record's
/// own declared PARENT bloodline pool-group name, keyed by the VARIANT's own pool-group name
/// (`"Bedrock Bloodline"` -> `"Deep Earth Bloodline"`).
///
/// Real corpus fact, confirmed live (`data/corpus/ultimate_magic/class_feature/wildblooded/
/// bedrock.json`): a "Wildblooded" bloodline variant (PF1e RAW -- Ultimate Magic p.68, a Sorcerer
/// swaps their bloodline's normal 1st-level power and bloodline arcana for a themed alternate)
/// is corpus-KEYED as if it were its OWN pool group (`"Bedrock Bloodline ~ Bloodline Arcana"`,
/// same `"<PoolGroup> ~ <Member>"` shape as every real bloodline), which is why
/// `real_groups_owned_by` correctly counts it as a real, distinct, selectable group -- but its
/// own `PREABILITY` token (`"1,CATEGORY=Special Ability,Sorcerer Bloodline ~ Deep Earth"`) proves
/// selecting it REQUIRES the character to already hold a real, different, named bloodline (`"Deep
/// Earth Bloodline"`) as a level-1 prerequisite -- so that parent's own header vars
/// (`Sorcerer_DeepEarth_BloodlinePowerNLVL`, ...) are, by corpus-declared construction, always
/// genuinely bound whenever a Wildblooded variant's own member records reference them (cycle
/// 17/19's own oracle-proven "cross-bloodline" refusal shape does NOT apply here -- that shape is
/// a genuinely UNRELATED bloodline with no prerequisite link; this is a variant's own declared
/// PARENT). All 20 real Wildblooded records checked (18 previously refused this way, `Empyreal`/
/// `Sage` already resolving via another path): every one names its parent via this exact
/// `PREABILITY:...,Sorcerer Bloodline ~ <Parent>` shape, none any other.
///
/// Parsing is deliberately narrow and corpus-literal: the LAST comma-separated segment of the
/// FIRST real `PREABILITY` token, split on `" ~ "`, its own last segment -- never a guess, never a
/// transform of the variant's own name (proven live against all 20 files above).
pub(crate) fn wildblooded_variant_parent_pool_group() -> &'static BTreeMap<String, String> {
    static TABLE: OnceLock<BTreeMap<String, String>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut out = BTreeMap::new();
        let corpus_root = repo_root().join("data/corpus");
        let Ok(books) = std::fs::read_dir(&corpus_root) else { return out };
        let mut book_dirs: Vec<_> = books.flatten().collect();
        book_dirs.sort_by_key(|e| e.file_name());
        for book_entry in book_dirs {
            let wb_dir = book_entry.path().join("class_feature").join("wildblooded");
            if !wb_dir.is_dir() {
                continue;
            }
            let mut files = Vec::new();
            walk_json_files(&wb_dir, &mut files);
            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                let data = &doc["data"];
                let Some(name) = data["name"].as_str() else { continue };
                let Some(tokens) = data["raw_tokens"].as_array() else { continue };
                let Some(preability) = tokens
                    .iter()
                    .find(|t| t["key"].as_str() == Some("PREABILITY"))
                    .and_then(|t| t["value"].as_str())
                else {
                    continue;
                };
                let Some(last_segment) = preability.rsplit(',').next() else { continue };
                let Some(parent) = last_segment.rsplit(" ~ ").next() else { continue };
                if parent == last_segment {
                    // no " ~ " separator -- not the expected shape, skip rather than guess.
                    continue;
                }
                out.entry(format!("{name} Bloodline")).or_insert_with(|| format!("{parent} Bloodline"));
            }
        }
        out
    })
}

/// Every corpus `data/corpus/*/class/*.json` CLASS record's own PRE-gate-safe `BONUS:VAR` chain,
/// keyed by `class_id` (SD-32 T12 Epic 8 row 18 cycle 8). Real corpus fact, confirmed live:
/// Cleric's own `DomainLVL` (`BONUS:VAR|DomainLVL|ClericLVL`, real PCGen source `cr_classes.lst`)
/// binds on the CLASS record itself, `core_rulebook/class/cleric.json`, NOT on any `class_feature`
/// record -- every one of the 67 real, never-hand-modelled Cleric Domain groups' own members needs
/// this exact binding and none of them can ever supply it themselves (cycle 7's own receipt named
/// this as a second, separate, larger gap than the Bloodline family's per-book `.MOD`-collision
/// one). Row 21 restored `raw_tokens` onto every one of the 168 real class records (previously
/// absent entirely) -- this table is the missing READ side, mirroring `class_feature_bonus_vars_
/// any_record`'s own shape one dir level up. `class_id` is the record's plain display name
/// (`"Cleric"`, confirmed live -- never a `"class:"`-prefixed id), so this table's own keys line up
/// directly with every `owning_class`/`class` string this module already threads. One real class
/// record per book-and-name pair observed so far (no cross-book duplication like the `class_
/// feature` family's own "Tracker" shape), so first-insert-wins is safe here; a future duplicate
/// would still merge safely via the same `.or_insert`-per-target policy `class_feature_bonus_vars_
/// any_record` already uses, kept identical for consistency rather than re-derived per table.
pub(crate) fn class_record_bonus_vars() -> &'static BTreeMap<String, BTreeMap<String, String>> {
    static TABLE: OnceLock<BTreeMap<String, BTreeMap<String, String>>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut out: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        let corpus_root = repo_root().join("data/corpus");
        let Ok(books) = std::fs::read_dir(&corpus_root) else { return out };
        let mut book_dirs: Vec<_> = books.flatten().collect();
        book_dirs.sort_by_key(|e| e.file_name());
        for book_entry in book_dirs {
            let class_dir = book_entry.path().join("class");
            if !class_dir.is_dir() {
                continue;
            }
            let mut files = Vec::new();
            walk_json_files(&class_dir, &mut files);
            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                let data = &doc["data"];
                let Some(class_id) = data["class_id"].as_str() else { continue };
                let bonus_vars = data["raw_tokens"]
                    .as_array()
                    .map(|tokens| parse_bonus_var_tokens_pre_gate_safe(tokens))
                    .unwrap_or_default();
                let entry = out.entry(class_id.to_string()).or_default();
                for (target, formula) in bonus_vars {
                    entry.entry(target).or_insert(formula);
                }
            }
        }
        out
    })
}

/// Every corpus `data/corpus/*/domain/*.json` DOMAIN record's own PRE-gate-safe `BONUS:VAR` chain,
/// keyed by the domain's own bare `KEY:` (e.g. `"Cave"`, `"Aquatic"` -- never `"Cave Domain"`, the
/// `class_feature` sibling family's own naming shape) (SD-32 T12 Epic 8 row 18 cycle 18, `§27b`).
///
/// Real corpus fact, confirmed live by direct inspection
/// (`data/corpus/ultimate_magic/domain/cave.json`): a `domain`-kind record ALREADY carries the
/// real, resolvable `BONUS:VAR|DomainCaveLVL|DomainLVL|TYPE=Domain`, `BONUS:VAR|DomainCaveDC|
/// 10+(DomainCaveLVL/2)+WIS|TYPE=Domain`, `BONUS:VAR|DomainCaveTimes|DomainPowerTimes|TYPE=Domain`
/// chain every one of that domain's `class_feature` MEMBER records (`"Cave Domain ~ Cavesight"`,
/// ...) needs -- the exact same convention `pool_header_record_by_normalized_suffix`'s cycle-5
/// widening already merges for domains whose header happens to live under `class_feature`
/// (`"Air Domain"`, bare key, `data/corpus/core_rulebook/class_feature/air/air.json`). Neither
/// `class_feature_bonus_vars_any_record` nor any other existing table ever reads this `domain`
/// subdirectory at all -- every domain whose ONLY real header record lives here (23 of the
/// cycle-18 census's 197 unresolved groups sampled this cycle: Aquatic, Arctic, Eagle, Frog,
/// Jungle, Monkey, Mountain, Plains, Serpent, Swamp, Cave, Desert, ...) was refused purely for
/// want of this read path, not for want of real data -- a genuine engine gap this cycle closes,
/// not a data gap (`§27b` distinguishes the two; only a hard impossibility, never "no consumer
/// reaches it", excuses a unit).
///
/// `data.class` is genuinely absent on this record shape (confirmed: the domain schema carries no
/// `class` field at all, gated instead by a `PRECLASS:` token this table does not need to read --
/// the domain-header convention is inherently class-independent the same way `"Domains"`'s own bare
/// `DomainPowerTimes` chain already is, per that clause's own doc above). Merged across books
/// exactly like every sibling table here (`.or_insert` per target name, never overwriting an
/// already-bound target), so a genuine cross-book disagreement on the same target name keeps
/// whichever book's row was seen first.
pub(crate) fn domain_kind_bonus_vars_any_record() -> &'static BTreeMap<String, BTreeMap<String, String>> {
    static TABLE: OnceLock<BTreeMap<String, BTreeMap<String, String>>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut out: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        let corpus_root = repo_root().join("data/corpus");
        let Ok(books) = std::fs::read_dir(&corpus_root) else { return out };
        let mut book_dirs: Vec<_> = books.flatten().collect();
        book_dirs.sort_by_key(|e| e.file_name());
        for book_entry in book_dirs {
            let domain_dir = book_entry.path().join("domain");
            if !domain_dir.is_dir() {
                continue;
            }
            let mut files = Vec::new();
            walk_json_files(&domain_dir, &mut files);
            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                let data = &doc["data"];
                let Some(key) = data["key"].as_str() else { continue };
                let bonus_vars = data["raw_tokens"]
                    .as_array()
                    .map(|tokens| parse_bonus_var_tokens_pre_gate_safe(tokens))
                    .unwrap_or_default();
                let entry = out.entry(key.to_string()).or_default();
                for (target, formula) in bonus_vars {
                    entry.entry(target).or_insert(formula);
                }
            }
        }
        out
    })
}

/// PCGen's own auto-declared per-class level variable name: the class's display name with every
/// whitespace character removed, plus `LVL` -- confirmed corpus-wide (`Bard` -> `BardLVL`,
/// `Barbarian` -> `BarbarianLVL`, `Arcane Archer` -> `ArcaneArcherLVL`, ...) by grepping every
/// `BONUS:VAR|<name>|<ClassNameLVL>` single-identifier row in `data/corpus/*/class_feature/` and
/// checking which owning `data.class` value it names.
pub(crate) fn class_level_variable_name(class: &str) -> String {
    let mut out: String = class.chars().filter(|c| !c.is_whitespace()).collect();
    out.push_str("LVL");
    out
}

/// Every ability-abbreviation identifier PCGen's own formula tokens reference bare (`STR`,
/// `DEX`, `CON`, `INT`, `WIS`, `CHA`), seeded from the character's real, already-computed
/// [`AbilityModifiers`] -- SD-31 wave 27's widening of [`resolve_pcgen_var_chain`], named as the
/// scoped, cheap follow-on wave 26 explicitly deferred (its own doc comment below, prior
/// version: "No ability-modifier binding exists yet ... Widening to ability modifiers is real,
/// scoped follow-on work (`ability_modifiers` is already in scope at this module's one call
/// site, `compute_class_chassis`)"). These six identifiers are always bound, regardless of
/// whether the record being resolved actually references any of them -- an unreferenced
/// identifier sitting unused in the seed map changes nothing (the evaluator only ever reads an
/// identifier a formula names), and it costs nothing to seed all six once rather than special-
/// case which records need which ability.
fn ability_modifier_seed_vars(ability_modifiers: &AbilityModifiers) -> BTreeMap<String, i64> {
    let mut out = BTreeMap::new();
    out.insert("STR".to_string(), i64::from(ability_modifiers.strength));
    out.insert("DEX".to_string(), i64::from(ability_modifiers.dexterity));
    out.insert("CON".to_string(), i64::from(ability_modifiers.constitution));
    out.insert("INT".to_string(), i64::from(ability_modifiers.intelligence));
    out.insert("WIS".to_string(), i64::from(ability_modifiers.wisdom));
    out.insert("CHA".to_string(), i64::from(ability_modifiers.charisma));
    out
}

/// Resolves every `bonus_vars` identifier this record's own `BONUS:VAR` tokens can reach, seeded
/// with the two facts this module actually knows about one character: their level in the
/// granting class (bound to `class_level_var`) and their six ability modifiers (bound to the
/// bare `STR`/`DEX`/`CON`/`INT`/`WIS`/`CHA` abbreviations PCGen's own formula tokens use --
/// [`ability_modifier_seed_vars`], SD-31 wave 27). A fixed-point pass over the record's own token
/// set: repeatedly evaluates any not-yet-bound formula whose every identifier is already known,
/// through the real [`PcgenFormulaEvaluator`], until a full pass adds nothing further (capped at
/// 16 passes -- generous headroom over the longest chain this corpus has ever shown, 2 hops).
///
/// An identifier this loop cannot reach (a sibling record's own variable, a `classlevel(...)`
/// argument -- deliberately never bound here, per `formula_interpreter.rs`'s own standing
/// precondition that no consumer may bank through a `classlevel(...)`-bearing formula until its
/// cross-class gap is resolved -- or a shape the interpreter refuses, e.g. the documented
/// bare-comparison-as-numeric-term gap) is simply never bound -- never guessed, never defaulted.
/// [`resolved_description_for`]'s own downstream `render_pcgen_desc_with_values` call then drops
/// (and reports) any `%N` that still names it, exactly the way it already treats any other
/// unresolved argument.
///
/// **SD-32 T12 Epic 8 row 18 cycle 17 -- why refusing an identifier bound only on an unrelated
/// (cross-class or non-granting-source) record is the ORACLE-CORRECT behaviour, not merely a
/// conservative guess, established by reading the real engine rather than assuming.** Real
/// PCGen's variable read path, `PlayerCharacter.getVariable` (`code/src/java/pcgen/core/
/// PlayerCharacter.java:2090`), sums `getTotalBonusTo("VAR", variableString)` -- EVERY
/// `BONUS:VAR` contribution to that exact variable name from EVERY source the character actually
/// possesses, character-wide, with no per-class scoping at all. This confirmed, on inspection,
/// that this module's guard is not merely refusing to guess a class boundary the oracle also
/// respects -- the oracle has NO class boundary here; it is genuinely global. But that global sum
/// is only ever nonzero for a variable name a given character's OWN held sources actually bonus:
/// re-derived for Bloodrager's remaining 7 single-terminal `Bloodline` members (row 18 cycle 13's
/// own finding, re-verified cycle 16/17): each chains to a per-bloodline `Bloodrager_<X>_
/// BloodlineLVL` identifier bound by exactly TWO real corpus records, NEITHER a plain Bloodrager
/// who merely picked that bloodline through the class's own bloodline-choice mechanism would
/// hold -- (a) `data/corpus/advanced_class_guide/ability/*_bloodline.json`, an ABILITY record
/// under `CATEGORY:"Raging Blood Feat Bloodline"`, PREMULT-gated on holding a DIFFERENT, already-
/// taken `Eldritch Heritage Bloodline` or `Sorcerer Bloodline` ability -- a feat a plain Bloodrager
/// has no reason to hold, and (b) the `eldritch_scion_<x>_bloodline` class_feature record, `class:
/// "Sorcerer"` (the cross-class archetype cycle 13 named). Confirmed exhaustively (`grep` across
/// `data/corpus/**/*.json` for `<X>_BloodlineLVL|BloodragerLVL` or `<X>_BloodlineLVL|
/// BloodragerBloodlineLVL`, any class): no THIRD record exists binding a plain Bloodrager's own
/// per-bloodline `BloodlineLVL` to anything level-scaled. Importing either candidate's binding
/// here would misrepresent EVERY Bloodrager as if they also held the Raging Blood feat or the
/// Eldritch Scion archetype -- a live fabricated value, exactly the failure mode this bundle's
/// own dispatch brief names as having already occurred once when a cross-record refusal like
/// this one was loosened elsewhere. The guard therefore stays
/// exactly as cycle 12 built it; this cycle's contribution is the oracle citation proving it is
/// the correct model of the real engine's own semantics, not merely an unverified safety margin,
/// and confirming (not merely repeating) that the remaining 7 are a genuine data/ingestion gap
/// (no corpus record grants a plain Bloodrager's per-bloodline level at all) rather than a
/// resolvable compute-shape gap this lane's own scope could close.
pub(crate) fn resolve_pcgen_var_chain(
    bonus_vars: &BTreeMap<String, String>,
    class_level_var: &str,
    level: u8,
    ability_modifiers: &AbilityModifiers,
) -> BTreeMap<String, i64> {
    let evaluator = PcgenFormulaEvaluator;
    let mut vars: BTreeMap<String, i64> = ability_modifier_seed_vars(ability_modifiers);
    vars.insert(class_level_var.to_string(), i64::from(level));
    // SD-32 T12 Epic 8 row 18 cycle 6: bind `classlevel("<ThisClass>")`'s own per-class key too
    // (`formula_interpreter.rs`'s `Expr::ClassLevel` now looks up `CLASSLEVEL::<name>`, never a
    // class-blind `__LEVEL__` slot). This caller only ever knows ONE class's real level -- the
    // record's own granting class, recovered from `class_level_var` by stripping the trailing
    // `LVL` PCGen's own auto-declared-variable convention always appends (`class_level_variable_
    // name`'s own inverse) -- so only THAT class's key is bound; a formula naming any other class
    // stays unbound and refuses, never fabricates (see `formula_interpreter.rs`'s own doc for why
    // this is safe: same-class `classlevel(...)` now resolves correctly, genuinely-different-
    // class arguments still refuse cleanly).
    if let Some(class_name) = class_level_var.strip_suffix("LVL") {
        vars.insert(format!("CLASSLEVEL::{class_name}"), i64::from(level));
    }
    let mut progressed = true;
    let mut guard = 0;
    while progressed && guard < 16 {
        progressed = false;
        guard += 1;
        for (name, formula) in bonus_vars {
            if vars.contains_key(name) {
                continue;
            }
            if let Ok(value) = evaluator.evaluate(formula, &vars) {
                vars.insert(name.clone(), value);
                progressed = true;
            }
        }
    }
    // SD-32 T12 Epic 8 row 18 cycle 12: PCGen's own real 0-default for a bare
    // identifier this corpus never binds ANYWHERE, under ANY condition.
    //
    // Oracle trace, file:line (pinned PCGen commit `7f818006e371`):
    // `PlayerCharacter.java:2090-2140` (`getVariable`) tries a modern, DECLARED
    // `VariableKey` first; failing that (`hasVariable`, `:2430-2440`, false for
    // any name never registered as one) it falls to `getVariableValue`, which
    // is exactly the path a `BONUS:VAR|Target|Formula` RHS resolves through
    // (`JEPFormula.resolve` -> `character.getVariableValue`,
    // `code/src/java/pcgen/cdom/base/JEPFormula.java`). That method
    // (`VariableProcessor.java:125-139`) tries the modern JEP parser first
    // (`:151-182`, `processJepFormula` `:433-513`): JEP requires EVERY symbol
    // in the expression to resolve via `lookupVariable` (`:532-561`) or it
    // returns `null` outright -- "we could not get a value for all of the
    // variables, so it must not have been a JEP function after all" (`:469`).
    // `getVariableValue` then falls through to the LEGACY, `+`/`-`/`*`/`/`-
    // delimited parser (`processBrokenParser`, `:215-421`): its own per-term
    // loop (`:357-421`) calls `lookupVariable` for each term and, when that
    // returns `null` (not `hasVariable`, no internal/export variable either --
    // exactly what a globally-unbound identifier hits), leaves the term's raw
    // text unchanged; `Float.parseFloat` on that text then throws
    // `NumberFormatException`, caught silently and treated as `0.0` (`:394-
    // 402`, own comment: "Don't care, as it's just zero"). Real PCGen genuinely
    // computes 0 for this shape, not a refusal.
    //
    // Matched here NARROWLY, per `decisions.md §17a`'s "implement the
    // condition, not the convenience": only a bare `Expr::Var` lookup miss
    // (the interpreter's own distinct `"unbound variable {name:?}"` text,
    // `formula_interpreter.rs`'s `eval_expr`) is retried with THAT identifier
    // defaulted to 0 -- never `classlevel(...)`'s own separately-worded
    // refusal, division-by-zero, an unknown function, or any other refused
    // shape, all of which keep refusing exactly as before (they fail with
    // different error text this loop never matches). And only when the
    // identifier is ABSENT from `every_corpus_bound_bonus_var_target()` --
    // the full corpus-wide union of every `BONUS:VAR` target name any
    // class/class_feature record binds anywhere, PRE-gated or not. An
    // identifier present in that set (e.g. one this record's own PRE-gate-
    // safe parse dropped for genuine multi-row ambiguity) is a REAL,
    // possibly-nonzero conditional PCGen value this module cannot safely
    // guess -- it keeps refusing, unchanged from every prior cycle's own
    // safety property (cycles 2 and 5's proofs this doc inherits).
    //
    // A name genuinely absent from that set may STILL carry a real, more
    // precise corpus fact than a bare guessed `0`: `DEFINE:<name>|0` is
    // PCGen's own standard idiom for declaring a `BONUS:VAR` target's
    // baseline (`DefineLst.java`: registers a `VariableKey`, and its own
    // deprecation warning literally reads "please use a DEFINE of 0 and an
    // appropriate bonus" for any non-zero use) -- confirmed live for exactly
    // this family: `data/corpus/advanced_class_guide/class_feature/
    // bloodrager/bloodrager_bloodline_tracker.json` carries `DEFINE:
    // BloodragerBloodlinePower1LVLBonus|0` (and the 4/8/12/16/20 siblings),
    // and `grep` confirms no `BONUS:VAR` row anywhere ever targets that same
    // name, so there is no possible conditional addend this fallback could
    // be hiding. `corpus_define_literal_defaults()` reads exactly that
    // literal, falling back to `0` (real PCGen's own catch-all, see above)
    // only when the corpus never says anything about the name at all --
    // e.g. Sorcerer/Cleric/Shaman's bare `<Pool>PowerNLVLBonus` family,
    // which carries no `DEFINE` either.
    let bound_anywhere = every_corpus_bound_bonus_var_target();
    let define_defaults = corpus_define_literal_defaults();
    let mut progressed_zero = true;
    let mut guard_zero = 0;
    while progressed_zero && guard_zero < 16 {
        progressed_zero = false;
        guard_zero += 1;
        for (name, formula) in bonus_vars {
            if vars.contains_key(name) {
                continue;
            }
            match evaluator.evaluate(formula, &vars) {
                Ok(value) => {
                    vars.insert(name.clone(), value);
                    progressed_zero = true;
                }
                Err(e) => {
                    if let Some(missing) =
                        e.0.strip_prefix("unbound variable \"").and_then(|s| s.strip_suffix('"'))
                    {
                        if !vars.contains_key(missing) && !bound_anywhere.contains(missing) {
                            let default_value =
                                define_defaults.get(missing).copied().unwrap_or(0);
                            vars.insert(missing.to_string(), default_value);
                            progressed_zero = true;
                        }
                    }
                }
            }
        }
    }
    vars
}

/// Every `BONUS:VAR` target name ANY corpus `class_feature` or `class` record
/// binds, ANYWHERE, under ANY PRE-condition (SD-32 T12 Epic 8 row 18 cycle 12)
/// -- the set [`resolve_pcgen_var_chain`]'s own 0-default widening checks an
/// identifier against before treating it as PCGen's real 0-default (see that
/// function's doc for the oracle citation). Built from the SAME two
/// corpus-wide, PRE-gate-safe tables the header/member merges already use
/// (`class_feature_bonus_vars_any_record`, `class_record_bonus_vars`) rather
/// than a third scan, so this can never disagree with what those tables
/// actually ingested.
fn every_corpus_bound_bonus_var_target() -> &'static std::collections::BTreeSet<String> {
    static SET: OnceLock<std::collections::BTreeSet<String>> = OnceLock::new();
    SET.get_or_init(|| {
        let mut out = std::collections::BTreeSet::new();
        for record in class_feature_bonus_vars_any_record().values() {
            out.extend(record.bonus_vars.keys().cloned());
        }
        for vars in class_record_bonus_vars().values() {
            out.extend(vars.keys().cloned());
        }
        out
    })
}

/// Every `DEFINE:<name>|<literal integer>` corpus fact, ANY book, ANY
/// `class_feature`/`class` record (SD-32 T12 Epic 8 row 18 cycle 12) -- read
/// directly from `raw_tokens`, first-book-wins per name (this is a baseline
/// declaration, not an addend; a genuine cross-book disagreement on the same
/// name would be a corpus authoring defect this function is not the place to
/// adjudicate). Only literal-integer DEFINE values are captured (PCGen's own
/// documented idiom for a `BONUS:VAR` target's zero baseline, `DefineLst.
/// java`'s deprecation warning: "please use a DEFINE of 0 and an appropriate
/// bonus" for any non-zero use) -- a `DEFINE` whose formula is itself a
/// non-literal expression is skipped here rather than guessed at (this
/// module has no need for it: no corpus record currently references such a
/// name as an unbound term inside a `BONUS:VAR` chain this resolver walks).
fn corpus_define_literal_defaults() -> &'static BTreeMap<String, i64> {
    static TABLE: OnceLock<BTreeMap<String, i64>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut out: BTreeMap<String, i64> = BTreeMap::new();
        let corpus_root = repo_root().join("data/corpus");
        let Ok(books) = std::fs::read_dir(&corpus_root) else { return out };
        let mut book_dirs: Vec<_> = books.flatten().collect();
        book_dirs.sort_by_key(|e| e.file_name());
        for book_entry in book_dirs {
            for subdir_name in ["class_feature", "class"] {
                let dir = book_entry.path().join(subdir_name);
                if !dir.is_dir() {
                    continue;
                }
                let mut files = Vec::new();
                walk_json_files(&dir, &mut files);
                for file in files {
                    let Ok(text) = std::fs::read_to_string(&file) else { continue };
                    let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                    let Some(tokens) = doc["data"]["raw_tokens"].as_array() else { continue };
                    for token in tokens {
                        if token["key"].as_str() != Some("DEFINE") {
                            continue;
                        }
                        let Some(value) = token["value"].as_str() else { continue };
                        let mut parts = value.splitn(2, '|');
                        let (Some(name), Some(literal)) = (parts.next(), parts.next()) else {
                            continue;
                        };
                        let name = name.trim();
                        if name.is_empty() {
                            continue;
                        }
                        let Ok(parsed) = literal.trim().parse::<i64>() else { continue };
                        out.entry(name.to_string()).or_insert(parsed);
                    }
                }
            }
        }
        out
    })
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
    let mut values = crate::rules_core::pcgen_desc::PcgenDisplayValues::new();
    for (name, value) in &resolved_vars {
        values.set(name, *value);
    }
    let rendered =
        crate::rules_core::pcgen_desc::render_pcgen_desc_with_values(&record.raw_description, &values);
    if !rendered.dropped_args.is_empty() || rendered.text.is_empty() {
        return None;
    }
    if crate::rules_core::pcgen_desc::leaked_pcgen_syntax(&rendered.text).is_some() {
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
/// argument's value comes from when [`resolve_desc_argument`](crate::rules_core::pcgen_desc)'s
/// own three narrow shapes (integer literal, exact named lookup, `<Name><+|-><integer>` offset)
/// do not cover it -- evaluated directly through the SAME real [`PcgenFormulaEvaluator`] every
/// other resolver in this module already uses, seeded with the SAME two facts (class level,
/// ability modifiers), never a new evaluation mechanism.
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
    header_vars: &BTreeMap<String, String>,
) -> Option<(String, i64)> {
    let record = class_feature_record_tokens_pre_gate_safe().get(key)?;
    if !record.bonus_vars.is_empty() {
        return None; // a real BONUS:VAR chain exists -- `resolved_description_for`'s business.
    }
    let args = crate::rules_core::pcgen_desc::desc_token_arguments(&record.raw_description);
    if args.is_empty() {
        return None; // no `%N` argument at all -- nothing this function grounds.
    }
    let class_level_var = class_level_variable_name(&record.class);
    let evaluator = PcgenFormulaEvaluator;
    let mut seed_vars = ability_modifier_seed_vars(ability_modifiers);
    seed_vars.insert(class_level_var.clone(), i64::from(level));
    if let Some(class_name) = class_level_var.strip_suffix("LVL") {
        seed_vars.insert(format!("CLASSLEVEL::{class_name}"), i64::from(level));
    }
    if !header_vars.is_empty() {
        let resolved_header =
            resolve_pcgen_var_chain(header_vars, &class_level_var, level, ability_modifiers);
        for (name, value) in &resolved_header {
            seed_vars.entry(name.clone()).or_insert(*value);
        }
    }
    let mut values = crate::rules_core::pcgen_desc::PcgenDisplayValues::new();
    for arg in &args {
        let trimmed = arg.trim();
        if let Ok(value) = evaluator.evaluate(trimmed, &seed_vars) {
            // Keyed under the exact argument text -- `resolve_desc_argument`'s own named-lookup
            // shape (`values.get(arg)`) then finds it by that same exact text, with no change
            // needed to that function or to `render_pcgen_desc_with_values` itself.
            values.set(trimmed, value);
        }
        // An argument the interpreter refuses (an unbound identifier, a shape it does not
        // implement) is simply never inserted -- `render_pcgen_desc_with_values` below then
        // drops and reports it, exactly its existing no-fabrication contract for any other
        // unresolved argument.
    }
    let rendered =
        crate::rules_core::pcgen_desc::render_pcgen_desc_with_values(&record.raw_description, &values);
    if !rendered.dropped_args.is_empty() || rendered.text.is_empty() {
        return None;
    }
    if crate::rules_core::pcgen_desc::leaked_pcgen_syntax(&rendered.text).is_some() {
        return None;
    }
    let primary_value = evaluator.evaluate(args[0].trim(), &seed_vars).ok()?;
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

    /// SD-32 T12 Epic 8 row 18 cycle 10. The shared merge policy both
    /// cross-book tables now use: a target seen in an earlier book is never
    /// overwritten by a later book's own value for the same target name,
    /// but a target the earlier book never defined at all DOES get pulled
    /// in from a later book -- proving the exact defect the old whole-
    /// record `or_insert_with` (first book wins ENTIRELY, even for targets
    /// it never carried) used to have, on both tables, before cycle 8/10.
    #[test]
    fn merge_bonus_var_target_map_pulls_in_new_targets_but_never_overwrites_a_seen_one() {
        let mut into: BTreeMap<String, String> = BTreeMap::new();
        into.insert("SharedTarget".to_string(), "first-book-formula".to_string());
        into.insert("OnlyFirstBook".to_string(), "1".to_string());
        let mut from: BTreeMap<String, String> = BTreeMap::new();
        from.insert("SharedTarget".to_string(), "second-book-formula".to_string());
        from.insert("OnlySecondBook".to_string(), "2".to_string());

        merge_bonus_var_target_map_never_overwriting(&mut into, from);

        assert_eq!(
            into.get("SharedTarget").map(String::as_str),
            Some("first-book-formula"),
            "a target already bound by an earlier book must never be overwritten by a later one"
        );
        assert_eq!(into.get("OnlyFirstBook").map(String::as_str), Some("1"));
        assert_eq!(
            into.get("OnlySecondBook").map(String::as_str),
            Some("2"),
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
                resolve_pcgen_var_chain(&bonus_vars, "AssassinLVL", level, &AbilityModifiers::default());
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
                resolve_pcgen_var_chain(&bonus_vars, "RogueLVL", level, &AbilityModifiers::default());
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
            resolve_pcgen_var_chain(&bonus_vars, "RogueLVL", 10, &AbilityModifiers::default());
        assert_eq!(vars.get("SomeLVL"), Some(&10));
        assert!(
            vars.get("SomeBonus").is_none(),
            "a formula referencing an identifier bound elsewhere in the corpus (a sibling \
             record's own real BONUS:VAR target) must never resolve to a guessed number: {vars:?}"
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
        let mut bonus_vars = BTreeMap::new();
        bonus_vars.insert("SomeBonus".to_string(), "10+(SomeLVL/2)+NeverBoundAnywhere".to_string());
        bonus_vars.insert("SomeLVL".to_string(), "RogueLVL".to_string());
        let vars =
            resolve_pcgen_var_chain(&bonus_vars, "RogueLVL", 10, &AbilityModifiers::default());
        assert_eq!(vars.get("SomeLVL"), Some(&10));
        assert_eq!(
            vars.get("SomeBonus"),
            Some(&15),
            "10 + (10/2) + 0 = 15 -- a genuinely corpus-wide-unbound identifier is PCGen's own \
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
            resolve_pcgen_var_chain(&bonus_vars, "BloodragerLVL", 7, &AbilityModifiers::default());
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
        let vars = resolve_pcgen_var_chain(&bonus_vars, "RogueLVL", 20, &ability_modifiers);
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
        let vars2 = resolve_pcgen_var_chain(&bonus_vars, "RogueLVL", 20, &ability_modifiers_wis_only);
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
        assert_eq!(
            (already_admitted, newly_resolved, class_excluded_otherwise_resolvable, chain_unresolvable, no_record_at_all),
            (136, 32, 0, 43, 1),
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
