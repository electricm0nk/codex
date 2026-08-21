//! Option-pool `class_feature` browsable catalog (SD31-W22-POOLMEMBER-001).
//!
//! # Why this module exists
//!
//! `class_feature_effect_wired` / `CLASS_FEATURE_POOLS`
//! (`v06_work_inventory.rs`) already prove, per record, whether SELECTING a
//! specific option-pool member (a rogue talent, a bloodline power, a witch
//! hex, ...) moves an observable engine fact. That answers "is this record's
//! magnitude computed" for the minority of pool members whose selection
//! changes something the engine renders.
//!
//! It cannot answer Decision 7's question for the majority: a genuinely
//! prose-only, zero-magnitude pool member (e.g. Rogue Talent ~ Ledge
//! Walker: "you move along narrow surfaces at full speed") never changes any
//! observable fact whether or not it is selected — there is nothing to
//! compute — so the consumer-delta probe correctly reports
//! `NoConsumerDelta`/"not held", and `Kind::ClassFeature`'s own doc comment
//! (`v06_work_inventory.rs`, the `class_feature_owner_matched_by_name_but_
//! record_not_held_by_engine` branch) names exactly the missing precondition
//! for `text-complete`: **"no generic class_feature catalog exists anywhere
//! in this engine, unlike feat/spell/equipment"** (`decisions.md §42`,
//! `SD28-E24`). `feat`/`spell`/`equipment` each have a real catalog that
//! serves every record's description to a player regardless of whether that
//! record is currently held — a browsable reference, not a per-character
//! computation. This module is that catalog for `class_feature` option-pool
//! records, built for a small, deliberately-widened set of pools
//! (`REGISTERED_POOL_GROUPS`) as the dispatch briefs asked: a precise answer
//! per pool, not a stub across all of them.
//!
//! # Scope: Rogue Talent and Rage Power, deliberately no wider
//!
//! `v06_work_inventory.rs`'s `CLASS_FEATURE_POOLS` registers 27 pools.
//! Widening `REGISTERED_POOL_GROUPS` to all of them is mechanical — the same
//! walk, the same render-and-refuse gate — but each pool's corpus rows would
//! need the same one-pool spot-check wave 22 ran on Rogue Talent and this
//! cycle (`SD31-W23-POOLMEMBER-002`) ran on Rage Power (are the `%N`
//! argument shapes the same, is `data.class` really the bare pool name for
//! every book that prints it) before being trusted at scale. Left named,
//! not built, per the dispatch's own "report what it would cost to extend"
//! ask.
//!
//! **Rage Power's own wrinkle, not present for Rogue Talent:** its group
//! text ("Rage Power") shares no prefix/suffix with its owning class's own
//! name ("barbarian"), unlike "Rogue Talent" which literally starts with
//! "Rogue ". `v06_work_inventory.rs`'s `class_feature_owner` (and its
//! `type_facet` fallback) therefore resolve `None` for every Rage Power
//! record, which used to route the WHOLE pool through a hard-coded
//! `not_ingested` regardless of this catalog. `classify()`'s "no owner
//! resolved" branch now ALSO consults `class_feature_pool_catalog_holds`
//! before falling back (`SD31-W23-POOLMEMBER-002`), mirroring the check the
//! "owner resolved" branch already had — without that fix, registering
//! "Rage Power" here would have changed nothing on the board. Any future
//! pool needs the SAME check first: does its group text share a
//! prefix/suffix with its class's name (Rogue Talent, Advanced Talents,
//! Versatile Performance shape) or not (Rage Power, Discovery, Hex,
//! Bloodline, Domain, Mystery, ... shape, per `v06_work_inventory.rs`'s own
//! `class_feature_option_pool_record_not_held_by_engine` comment) — both
//! shapes now reach the board, so this is no longer a blocker, only a fact
//! worth re-confirming per pool before assuming the owned-branch precedent
//! alone explains a movement.
//!
//! # The render-and-refuse gate is the whole safety property
//!
//! A pool member's corpus `description` is the RAW, unresolved `.lst` `DESC:`
//! string — for a record like `Rogue Talent ~ Bleeding Attack`, that string
//! is `"...take %1 additional points of damage...|SneakAttackDice"`:
//! `SneakAttackDice` is a bare cross-reference to a character-specific value
//! this catalog has no character to resolve against, so
//! `wiring_class::has_prose_formula_segment` (deliberately) leaves it
//! undetermined rather than guessing, and Decision 7's condition 2 ("nothing
//! to compute") genuinely fails for it — a player cannot read a complete
//! sentence without the engine computing a number this catalog is not given.
//! [`render_pcgen_desc`] already reports exactly this as a dropped `%N`
//! argument; this module refuses to serve any record whose render drops one,
//! which is simultaneously the leak guard every sibling catalog
//! (`monster_catalog`, `companion_catalog`, `class_feature_descriptions`)
//! already runs AND the correct Decision-7 disposition for a record that
//! genuinely still needs a computation. The two never conflict here.
//!
//! # PI screening
//!
//! Already discharged upstream, same trust boundary as
//! `class_feature_descriptions.rs`: `cache_gen::class_feature::generate`
//! screens NAME and DESCRIPTION (SD-30 `§52.3`/`§53.5`) before a record is
//! ever written to `data/corpus/`. This module reads only that
//! already-screened output and re-runs no PI check of its own.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::rules_core::pcgen_desc::{leaked_pcgen_syntax, render_pcgen_desc};

/// Corpus `data.class` values this catalog recognises as an option-pool
/// group rather than a real engine-modelled class. See the module doc for
/// why the list is only these two entries today.
pub const REGISTERED_POOL_GROUPS: &[&str] = &["Rogue Talent", "Rage Power"];

/// `raw_tokens` keys that carry a real, player-facing engine effect --
/// wave-22 adversarial review CONFIRMED (finding, severity high) that 9 of
/// the lane's 88 originally-banked records carry one of these alongside a
/// clean-rendering description (e.g. `Finesse Rogue`'s own `ABILITY:FEAT|
/// VIRTUAL|Weapon Finesse`, `Skill Mastery`'s `SELECT:3+INT`). Decision 7
/// condition 1 ("prose only, not a mechanic") and condition 2 ("nothing to
/// compute") both fail for a record carrying any of these -- the render-
/// and-refuse gate above only catches an UNRESOLVED `%N` inside the prose
/// itself, never a wholly separate mechanical token the description text
/// never mentions at all. Refused here, at the corpus-row level, per
/// Decision 7's own binding PROXY WARNING (hand-verify the WHOLE row, not
/// a magnitude-token proxy, before banking a zero-magnitude unit).
const ENGINE_EFFECT_TOKEN_KEYS: &[&str] =
    &["ABILITY", "CSKILL", "SELECT", "AUTO", "SAB", "BONUS", "DEFINE", "ADD", "SPELLS", "DR", "SR"];

/// SD31-W29-INTEGRATE (Ruling §18, `OPERATOR-RULINGS-2026-08-21.md`):
/// *"we need to show only valid choices."* A PCGen `PRE*` token (
/// `PREABILITY`, `PREVARGTEQ`, `PREMULT`, `PRESKILL`, `PRELEVEL`,
/// `PREFACT`, `PRECLASS`, `PREFEAT`, ...) is a prerequisite gate: the
/// option is not available to every character of the pool's owning class,
/// only to the subset who satisfies it (an archetype, a minimum level, a
/// skill rank, ...). This catalog has no character to check a prerequisite
/// against -- it is a book-wide reference walk, not a per-character
/// query -- so it cannot tell a qualifying character from a disqualified
/// one. Wave 29's adversarial review found 126 already-registered records
/// (both Rogue Talent and Rage Power) carry a `PRE*` token and are served
/// wholesale regardless, which is exactly the "listing a pool wholesale"
/// shape Ruling §18 forbids ("the next pool-touching cycle must confirm
/// the shipped catalog honours that rather than listing a pool wholesale.
/// That check has not been done" -- it has now). Refusing here is the
/// doctrinal fix: an option this catalog cannot prove is a VALID choice is
/// not served, full stop, same posture as [`has_no_engine_effect_token`]
/// refusing a record with a live mechanical token this catalog cannot
/// safely narrate. A future cycle that wants these records back needs
/// real per-character prerequisite evaluation in the picker itself
/// (`class_feature_pool_picker.rs`), not a wider catalog.
fn has_no_prerequisite_token(raw_tokens: &Value) -> bool {
    let Some(tokens) = raw_tokens.as_array() else { return true };
    !tokens.iter().any(|t| {
        t.get("key").and_then(|k| k.as_str()).is_some_and(|k| k.starts_with("PRE"))
    })
}

/// `true` when `raw_tokens` carries no [`ENGINE_EFFECT_TOKEN_KEYS`] entry --
/// i.e. the record is genuinely prose-only, not merely prose-renders-clean.
fn has_no_engine_effect_token(raw_tokens: &Value) -> bool {
    let Some(tokens) = raw_tokens.as_array() else { return true };
    !tokens.iter().any(|t| {
        t.get("key").and_then(|k| k.as_str()).is_some_and(|k| ENGINE_EFFECT_TOKEN_KEYS.contains(&k))
    })
}

/// A silent-truncation defect found by on-screen DoD-8 inspection while
/// widening this catalog to Rage Power (`SD31-W23-POOLMEMBER-002`), present
/// in neither the render-and-refuse gate nor the engine-effect-token gate:
/// PCGen ships a handful of records with MULTIPLE `DESC:` tab fields on the
/// same row -- a lead-in clause plus several `PREVAREQ:`-gated continuation
/// clauses, one per "which element/condition did the character pick" branch
/// (e.g. `Rage Power ~ Elemental Blood (Greater)`'s real oracle row: `DESC:
/// While raging, the barbarian gains` followed by four separate `DESC:
/// ...a burrow speed of 30 feet.|PREVAREQ:BloodRage Acid,1` / `...a swim
/// speed of 60 feet.|PREVAREQ:BloodRage Cold,1` / ... segments). The
/// upstream ingestion this module reads (`cache_gen::class_feature`,
/// outside this module's file territory) keeps only the FIRST `DESC:`
/// token's text as `data.description` -- for most such records that first
/// segment already carries an unresolvable `%N` (caught by the existing
/// render-and-refuse gate) or a real engine-effect token (caught by
/// [`has_no_engine_effect_token`]), but `Elemental Blood, Greater`'s lead-in
/// clause is a plain, syntax-clean sentence FRAGMENT with neither -- it
/// rendered "While raging, the barbarian gains" verbatim on a live character
/// sheet, a truncated sentence with no missing-`%N`/leaked-syntax signal at
/// all. Refused structurally here: any record whose row carries more than
/// one `DESC:` field is, by construction, showing only a fragment of what
/// the oracle actually states, regardless of whether that fragment happens
/// to read as a complete sentence.
fn raw_tokens_carry_more_than_one_desc_segment(raw_tokens: &Value) -> bool {
    let Some(tokens) = raw_tokens.as_array() else { return false };
    tokens.iter().filter(|t| t.get("key").and_then(|k| k.as_str()) == Some("DESC")).count() > 1
}

/// A gap in `render_pcgen_desc`'s own `dropped_args` reporting, found while
/// widening this catalog to Rage Power (`SD31-W23-POOLMEMBER-002`; NOT
/// present in the Rogue Talent population wave 22 checked -- confirmed by
/// scanning all 170 real Rage Power records, only one carries this exact
/// shape): `dropped_args` only records an unresolved `%N` when the raw
/// `DESC:` token's OWN `|`-tail supplies a named argument for it
/// (`pcgen_desc.rs`'s own `split_prose_and_args` doc comment: "the unmatched
/// `%N` then has no argument at all... the honest outcome"). When the token
/// carries a bare `%N` reference and NO `|`-tail at all (`Rage Power ~
/// Knockback`'s real oracle row: `"...target takes %1 points of damage..."`
/// with no trailing `|<Var>`), the digit is silently deleted from the
/// rendered text -- no leaked syntax, no reported drop, just a grammatical
/// hole ("the target takes  points of damage") that neither
/// `dropped_args.is_empty()` nor `leaked_pcgen_syntax` can see. Decision 7
/// condition 2 ("nothing to compute") fails here exactly as it does for a
/// named unresolved argument: `render_pcgen_desc` is always called with
/// EMPTY `PcgenDisplayValues` in this reference catalog (no character
/// exists to resolve against), so any `%N` reference with no `|`-tail at
/// all can never resolve, ever. Checked independently of `pcgen_desc.rs`'s
/// own logic, deliberately -- fixing the shared renderer to ALSO populate
/// `dropped_args` for this case would widen every one of its callers'
/// behaviour at once, well outside this pool-membership lane's file
/// territory; this is a narrow, additional guard scoped to this catalog
/// alone, refusing conservatively where the shared renderer stays silent.
fn raw_desc_has_a_bare_percent_reference_no_pipe_tail_can_resolve(raw: &str) -> bool {
    if raw.contains('|') {
        // A `|`-tail exists; `render_pcgen_desc`'s own `dropped_args` (via
        // the existing `!rendered.dropped_args.is_empty()` gate below)
        // already catches an unresolved named argument in this shape.
        return false;
    }
    let chars: Vec<char> = raw.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '%' {
            if chars.get(i + 1) == Some(&'%') {
                i += 2; // `%%` is a literal-percent escape, never a reference.
                continue;
            }
            if chars.get(i + 1).is_some_and(|c| c.is_ascii_digit() && *c != '0') {
                return true;
            }
        }
        i += 1;
    }
    false
}

/// One option-pool member's real corpus row, with a description proven to
/// render with nothing missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolCatalogEntry {
    /// The corpus book directory this record was read from.
    pub book: String,
    /// The registered pool group this record belongs to (`data.class`,
    /// verbatim — e.g. `"Rogue Talent"`).
    pub pool_group: String,
    /// The corpus `KEY:` token verbatim (e.g. `"Rogue Talent ~ Ledge
    /// Walker"`).
    pub key: String,
    pub name: String,
    /// Rendered through [`render_pcgen_desc`], with every unsubstituted
    /// `%N` argument refused rather than served (see the module doc's
    /// "render-and-refuse" section). Never empty, `.CLEAR`/`.CLEARALL`, or
    /// the PI-redaction marker — those never reach this struct at all.
    pub description: String,
}

/// Reproduced from `v06_work_inventory.rs`/`class_feature_descriptions.rs`'s
/// own copies — this crate's disjoint-file-touch convention, so a
/// consumer-territory module never has to coordinate an edit with an
/// ingest-territory one for a three-line predicate.
fn is_real_description_value(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !matches!(lower.as_str(), ".clear" | ".clearall" | "[redacted pi]")
}

/// Hand-verified exclusion: a [`REGISTERED_POOL_GROUPS`] record whose real
/// rulebook description carries a magnitude that scales on the
/// CHARACTER'S OWN class level (`"1/2 her barbarian level"`, `"per four
/// barbarian levels"`, ...) and applies to a value THIS ENGINE ALREADY
/// COMPUTES for that character (an activation state such as `active_
/// barbarian_rage_bonus` -- `pilot_compute/mod.rs` -- proves "while
/// raging" is not merely narrative flavor here) -- SD-31 wave 23
/// integration-cycle review finding (`corrected_units`): `wiring_class.rs`'s
/// `prose_scaling_phrases` list recognises `"your class level"`/`"your
/// character level"` but not a class-specific phrasing
/// (`"barbarian level"`, `"character's level"`), so these 16 records
/// cleared the `!carries_prose_magnitude` check that decides `text_only`
/// by an accident of that phrase list, not because Decision 7's
/// conditions were actually satisfied. Every entry was hand-read against
/// its shipped `data/corpus` description before listing here (see the
/// review's own `evidence` field); the Linnorm Death Curse variants this
/// SAME review deliberately left OFF this list (their scaled save DC
/// applies to an ATTACKER, never a value on this character's own sheet)
/// remain served, pending the operator ruling the review's own
/// `needs_ruling` records.
///
/// This is a targeted, hand-kept list rather than a broadened automatic
/// phrase scan -- widening `wiring_class.rs`'s shared phrase list would
/// ripple to every OTHER kind that list gates (feat, spell, monster_
/// ability, ...), a blast radius this integration cycle has no budget to
/// re-verify; a class_feature_pool_catalog-local denylist is exactly the
/// same "narrow, disjoint-file-touch correction" precedent this module's
/// other hand-kept guards (`raw_tokens_carry_more_than_one_desc_segment`,
/// the render-and-refuse gate) already establish.
const CLASS_LEVEL_SCALED_SHEET_VALUE_EXCLUDED_KEYS: [&str; 16] = [
    "Rage Power ~ Chaos Totem (Greater)",
    "Rage Power ~ Energy Resistance",
    "Rage Power ~ Guarded Life",
    "Rage Power ~ Guarded Life (Greater)",
    "Rage Power ~ Primal Scent",
    "Rage Power ~ Regenerative Vigor",
    "Rage Power ~ Renewed Life",
    "Rage Power ~ Renewed Vitality",
    "Rage Power ~ Hive Totem",
    "Rage Power ~ Hive Totem Resilience",
    "Rage Power ~ Liquid Courage",
    "Rage Power ~ Roaring Drunk",
    "Rage Power ~ Staggering Drunk",
    "Rage Power ~ Crippling Blow",
    "Rage Power ~ Eater of Magic",
    "Rage Power ~ Spell Sunder",
];

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

/// Reads every already-ingested `class_feature` cache record under
/// `<repo_root>/data/corpus/*/class_feature/**/*.json` whose `data.class`
/// names a [`REGISTERED_POOL_GROUPS`] entry, keeping only the ones whose
/// description renders with nothing missing (see the module doc's
/// render-and-refuse gate). Reads a NEW tree of nothing — every record
/// already lives in the committed `data/corpus/` cache
/// `cache_gen::class_feature::generate` writes; this module adds no new
/// corpus data of its own, only a new reading of what already exists.
pub fn load_pool_catalog(repo_root: &Path) -> Vec<PoolCatalogEntry> {
    let corpus_root = repo_root.join("data/corpus");
    let mut out = Vec::new();
    let Ok(books) = std::fs::read_dir(&corpus_root) else { return out };
    let mut book_dirs: Vec<_> = books.flatten().collect();
    book_dirs.sort_by_key(|e| e.file_name());
    for book_entry in book_dirs {
        let book_dir = book_entry.path();
        if !book_dir.is_dir() {
            continue;
        }
        let book = book_entry.file_name().to_string_lossy().to_string();
        let cf_dir = book_dir.join("class_feature");
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
            if !REGISTERED_POOL_GROUPS.contains(&class) {
                continue;
            }
            if CLASS_LEVEL_SCALED_SHEET_VALUE_EXCLUDED_KEYS.contains(&key) {
                continue;
            }
            let Some(raw_desc) = data["description"].as_str() else { continue };
            if !is_real_description_value(raw_desc) {
                continue;
            }
            if !has_no_engine_effect_token(&data["raw_tokens"]) {
                continue;
            }
            if !has_no_prerequisite_token(&data["raw_tokens"]) {
                continue;
            }
            if raw_tokens_carry_more_than_one_desc_segment(&data["raw_tokens"]) {
                continue;
            }
            if raw_desc_has_a_bare_percent_reference_no_pipe_tail_can_resolve(raw_desc) {
                continue;
            }
            let rendered = render_pcgen_desc(raw_desc);
            // The render-and-refuse gate: an unresolved `%N` means a real
            // computation this catalog cannot perform is still missing from
            // the sentence, which fails Decision 7's condition 2 (`nothing
            // to compute`) at the same time it would leak broken syntax.
            if !rendered.dropped_args.is_empty() {
                continue;
            }
            if leaked_pcgen_syntax(&rendered.text).is_some() {
                continue;
            }
            // Strip raw PCGen footnote markers (`**`, `*`) that leaked
            // into the shipped `name` field -- adversarial review
            // confirmed `leaked_pcgen_syntax` was applied to `description`
            // only, never `name` (e.g. "Deadly Sneak**" reaching the
            // sheet verbatim). `name` never carries `%N`/`|`-arg syntax,
            // only trailing footnote asterisks, so a plain trim suffices
            // here (the description's own render-and-refuse gate already
            // handles the richer PCGen syntax shapes).
            let clean_name = name.trim_end_matches('*').trim().to_string();
            out.push(PoolCatalogEntry {
                book: book.clone(),
                pool_group: class.to_string(),
                key: key.to_string(),
                name: clean_name,
                description: rendered.text,
            });
        }
    }
    out
}

/// `(book, key) -> description` for every entry the catalog holds — the
/// shape `v06_work_inventory.rs`'s `EngineFacts` (and `Kind::ClassFeature`'s
/// classify arm) actually consults, mirroring `feat_served_descriptions`'
/// own `(book, key)` indexing.
pub fn pool_catalog_index(entries: &[PoolCatalogEntry]) -> BTreeMap<(String, String), String> {
    entries.iter().map(|e| ((e.book.clone(), e.key.clone()), e.description.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn bare_percent_reference_with_no_pipe_tail_is_flagged_only_when_no_pipe_exists() {
        // The exact real shape (`Rage Power ~ Knockback`'s oracle row):
        // a `%1` reference with no `|`-tail anywhere in the token.
        assert!(raw_desc_has_a_bare_percent_reference_no_pipe_tail_can_resolve(
            "the target takes %1 points of damage"
        ));
        // A `%N` WITH a pipe tail is left to `render_pcgen_desc`'s own
        // `dropped_args` reporting -- never double-refused here.
        assert!(!raw_desc_has_a_bare_percent_reference_no_pipe_tail_can_resolve(
            "you add +%1 on one check|StrengthSurgeBonus"
        ));
        // A literal `%%` escape is never a reference, pipe or no pipe.
        assert!(!raw_desc_has_a_bare_percent_reference_no_pipe_tail_can_resolve(
            "a 20%% chance of success"
        ));
        // No percent sign at all.
        assert!(!raw_desc_has_a_bare_percent_reference_no_pipe_tail_can_resolve(
            "you move along narrow surfaces at full speed"
        ));
    }

    #[test]
    fn is_real_description_value_refuses_empty_clear_and_the_pi_marker() {
        assert!(!is_real_description_value(""));
        assert!(!is_real_description_value("   "));
        assert!(!is_real_description_value(".CLEAR"));
        assert!(!is_real_description_value(".CLEARALL"));
        assert!(!is_real_description_value("[redacted PI]"));
        assert!(is_real_description_value("You gain a bonus."));
    }

    /// The real corpus loads real, clean Rogue Talent records — proven
    /// against the live `data/corpus/` checkout, not a fixture. `Ledge
    /// Walker` is genuinely prose-only in the pinned oracle (no `%N`
    /// substitution, no `BONUS:`/`DEFINE:` token) and must be served intact.
    #[test]
    fn loads_a_real_clean_rogue_talent_from_the_live_corpus() {
        let entries = load_pool_catalog(&repo_root());
        let ledge_walker = entries
            .iter()
            .find(|e| e.book == "core_rulebook" && e.key == "Rogue Talent ~ Ledge Walker")
            .expect("core_rulebook's real Rogue Talent ~ Ledge Walker record must be in the catalog");
        assert_eq!(ledge_walker.pool_group, "Rogue Talent");
        assert_eq!(ledge_walker.name, "Ledge Walker");
        assert!(ledge_walker.description.starts_with("This ability allows you to move"));
        assert!(!ledge_walker.description.contains('|'), "no pipe-arg tail may leak into prose");
        assert!(!ledge_walker.description.contains('%'), "no unsubstituted argument may leak into prose");
    }

    /// The render-and-refuse gate's whole point: `Bleeding Attack`'s only
    /// magnitude is a bare cross-reference (`SneakAttackDice`) this catalog
    /// cannot resolve, so it must never be served — refused, not shipped
    /// with a dropped `%1` or a guessed number.
    #[test]
    fn bleeding_attack_is_refused_for_an_unresolvable_percent_argument() {
        let entries = load_pool_catalog(&repo_root());
        assert!(
            !entries.iter().any(|e| e.key == "Rogue Talent ~ Bleeding Attack"),
            "a record whose render drops a %N argument must never reach the catalog"
        );
        // The refusal is scoped to the one record, not the whole book.
        assert!(entries.iter().any(|e| e.book == "core_rulebook"));
    }

    /// Only registered pool groups are served — a real `Bloodline` or
    /// `Hex` record (unregistered today) must never appear, proving the
    /// scope guard is a real filter and not merely documentation.
    #[test]
    fn unregistered_pool_groups_are_never_served() {
        let entries = load_pool_catalog(&repo_root());
        assert!(
            entries.iter().all(|e| e.pool_group == "Rogue Talent" || e.pool_group == "Rage Power"),
            "only REGISTERED_POOL_GROUPS may appear in the catalog"
        );
        assert!(
            !entries.iter().any(|e| e.key.starts_with("Bloodline ~ ") || e.key.starts_with("Hex ~ ")),
            "an unregistered pool's records must never leak into the catalog"
        );
    }

    /// The real corpus loads real, clean Rage Power records — proven
    /// against the live `data/corpus/` checkout, not a fixture. `Clear
    /// Mind` is genuinely prose-only in the pinned oracle (a `PREVARGTEQ:`
    /// level gate, no `%N` substitution, no `BONUS:`/`DEFINE:`/`ABILITY:`
    /// token) and must be served intact (`SD31-W23-POOLMEMBER-002`).
    #[test]
    fn loads_a_real_clean_rage_power_from_the_live_corpus() {
        let entries = load_pool_catalog(&repo_root());
        let clear_mind = entries
            .iter()
            .find(|e| e.book == "core_rulebook" && e.key == "Rage Power ~ Clear Mind")
            .expect("core_rulebook's real Rage Power ~ Clear Mind record must be in the catalog");
        assert_eq!(clear_mind.pool_group, "Rage Power");
        assert_eq!(clear_mind.name, "Clear Mind");
        assert!(clear_mind.description.starts_with("You may reroll a failed Will save."));
        assert!(!clear_mind.description.contains('|'), "no pipe-arg tail may leak into prose");
        assert!(!clear_mind.description.contains('%'), "no unsubstituted argument may leak into prose");
    }

    /// The render-and-refuse gate applies identically to Rage Power:
    /// `Knockback`'s only magnitude is a bare `%1` with no `DEFINE:`/
    /// `BONUS:` token anywhere in its row to resolve it against (confirmed
    /// directly against the pinned oracle's `cr_abilities_class.lst`), so it
    /// must never be served.
    #[test]
    fn knockback_is_refused_for_an_unresolvable_percent_argument() {
        let entries = load_pool_catalog(&repo_root());
        assert!(
            !entries.iter().any(|e| e.key == "Rage Power ~ Knockback"),
            "a record whose render drops a %N argument must never reach the catalog"
        );
        assert!(entries.iter().any(|e| e.book == "core_rulebook" && e.pool_group == "Rage Power"));
    }

    /// The engine-effect-token refusal (wave-22's own withdrawal fix)
    /// applies identically to Rage Power: `Terrifying Howl` carries a real
    /// `BONUS:VAR` token computing its save DC even though its prose renders
    /// clean, and must be refused for the same reason `Finesse Rogue` was.
    #[test]
    fn terrifying_howl_is_refused_for_a_real_engine_effect_token() {
        let entries = load_pool_catalog(&repo_root());
        assert!(
            !entries.iter().any(|e| e.key == "Rage Power ~ Terrifying Howl"),
            "Terrifying Howl carries a real BONUS:VAR engine-effect token and must not reach the catalog"
        );
    }

    /// SD-31 wave 23 integration-cycle review finding: all 16 hand-verified
    /// `CLASS_LEVEL_SCALED_SHEET_VALUE_EXCLUDED_KEYS` entries must be
    /// refused, non-vacuously (each key really is present in the raw,
    /// unfiltered corpus, so this proves the exclusion is doing real work,
    /// not testing an already-absent key).
    #[test]
    fn every_class_level_scaled_sheet_value_key_is_excluded_from_the_catalog() {
        let entries = load_pool_catalog(&repo_root());
        for key in CLASS_LEVEL_SCALED_SHEET_VALUE_EXCLUDED_KEYS {
            assert!(
                !entries.iter().any(|e| e.key == key),
                "{key} carries a class-level-scaled sheet-computed magnitude and must not reach the catalog"
            );
            // Non-vacuous: the record really exists in the raw corpus under
            // `rage_power/`, so an unrelated typo in the denylist (or a
            // future corpus-key rename that silently orphans an entry)
            // cannot masquerade as "already excluded".
            let mut slug: String = key
                .rsplit(" ~ ")
                .next()
                .unwrap()
                .to_ascii_lowercase()
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect();
            while slug.contains("__") {
                slug = slug.replace("__", "_");
            }
            let slug = slug.trim_matches('_');
            let found = std::fs::read_dir(repo_root().join("data/corpus"))
                .unwrap()
                .flatten()
                .any(|book| {
                    let candidate = book.path().join("class_feature/rage_power").join(format!("{slug}.json"));
                    candidate.is_file()
                });
            assert!(found, "{key} (slug {slug:?}) must exist in the raw corpus for this test to be meaningful");
        }
    }

    /// The counterpart negative case, restated from the review's own
    /// distinction: a Linnorm Death Curse variant's scaled save DC applies
    /// to an ATTACKER, never a value this character's own sheet computes,
    /// so the review deliberately left it OFF the denylist pending an
    /// operator ruling -- it must still be served today.
    #[test]
    fn a_linnorm_death_curse_variant_is_not_excluded_pending_the_operator_ruling() {
        let entries = load_pool_catalog(&repo_root());
        assert!(
            entries.iter().any(|e| e.key.starts_with("Rage Power ~ Linnorm")),
            "Linnorm Death Curse variants must remain served until the operator rules on              whether an opponent-facing save DC counts as a sheet-computed magnitude"
        );
    }

    /// A real defect caught by DoD-8 on-screen inspection, not by any
    /// automated check: `Elemental Blood (Greater)`'s real oracle row carries
    /// FIVE `DESC:` tab fields (a lead-in clause plus four `PREVAREQ:`-gated
    /// per-element continuations); the corpus ingestion this catalog reads
    /// keeps only the first, so `data.description` is the literal fragment
    /// `"While raging, the barbarian gains"` -- syntax-clean (no `%N`, no
    /// pipe, no engine-effect token) but a truncated sentence. Must never
    /// reach the catalog (`SD31-W23-POOLMEMBER-002`).
    #[test]
    fn elemental_blood_greater_is_refused_for_a_silently_truncated_multi_desc_row() {
        let entries = load_pool_catalog(&repo_root());
        assert!(
            !entries.iter().any(|e| e.key == "Rage Power ~ Elemental Blood (Greater)"),
            "a record whose row carries more than one DESC: field must never reach the catalog \
             (only the first segment is ingested, which can be a syntax-clean sentence fragment)"
        );
        assert!(entries.iter().any(|e| e.book == "advanced_class_guide" && e.pool_group == "Rage Power"));
    }

    #[test]
    fn raw_tokens_carry_more_than_one_desc_segment_counts_desc_keys_only() {
        let one = serde_json::json!([{"key": "KEY", "value": "x"}, {"key": "DESC", "value": "x"}]);
        assert!(!raw_tokens_carry_more_than_one_desc_segment(&one));
        let two = serde_json::json!([
            {"key": "DESC", "value": "a"},
            {"key": "DESC", "value": "b"},
        ]);
        assert!(raw_tokens_carry_more_than_one_desc_segment(&two));
        // A second occurrence of an unrelated key must never trip this check.
        let unrelated_repeat = serde_json::json!([
            {"key": "DESC", "value": "a"},
            {"key": "SOURCEPAGE", "value": "p.1"},
            {"key": "SOURCEPAGE", "value": "p.2"},
        ]);
        assert!(!raw_tokens_carry_more_than_one_desc_segment(&unrelated_repeat));
    }

    /// No served description leaks unresolved PCGen syntax onto the screen
    /// — the same certification every sibling catalog runs, over the real
    /// cache rather than a hand-picked sample.
    #[test]
    fn every_served_description_renders_without_a_pcgen_syntax_leak() {
        let entries = load_pool_catalog(&repo_root());
        let mut checked = 0;
        for entry in &entries {
            if let Some(leak) = leaked_pcgen_syntax(&entry.description) {
                panic!("{:?} ({}): leaked {leak}", entry.key, entry.book);
            }
            checked += 1;
        }
        assert!(checked > 10, "no real descriptions were checked; the check proved nothing");
    }

    /// Wave-22 integration fix: 9 of the lane's originally-banked 88
    /// records carry a real engine-effect `raw_tokens` entry alongside a
    /// clean-rendering description and must be withdrawn (adversarial
    /// review, confirmed finding, severity high). `Finesse Rogue` is one
    /// of the 9 named -- `ABILITY:FEAT|VIRTUAL|Weapon Finesse`.
    #[test]
    fn a_record_with_a_real_engine_effect_token_is_refused_even_though_its_prose_renders_clean() {
        let entries = load_pool_catalog(&repo_root());
        for withdrawn_key in [
            "Rogue Talent ~ Finesse Rogue",
            "Rogue Talent ~ Improved Evasion",
            "Rogue Talent ~ Skill Mastery",
            "Rogue Talent ~ Combat Swipe",
            "Rogue Talent ~ Strong Impression",
            "Rogue Talent ~ Survivalist",
            "Rogue Talent ~ Firearm Training",
            "Rogue Talent ~ Getaway Artist",
            "Rogue Talent ~ Thrill of the Chase",
        ] {
            assert!(
                !entries.iter().any(|e| e.key == withdrawn_key),
                "{withdrawn_key} carries a real engine-effect token and must not reach the catalog"
            );
        }
        // The refusal is scoped to these records, not the whole catalog.
        assert!(entries.iter().any(|e| e.book == "core_rulebook"));
    }

    #[test]
    fn has_no_engine_effect_token_refuses_ability_and_select_but_allows_a_plain_desc_only_record() {
        let clean = serde_json::json!([{"key": "KEY", "value": "x"}, {"key": "DESC", "value": "x"}]);
        assert!(has_no_engine_effect_token(&clean));
        let with_ability = serde_json::json!([{"key": "ABILITY", "value": "FEAT|VIRTUAL|Weapon Finesse"}]);
        assert!(!has_no_engine_effect_token(&with_ability));
        let with_select = serde_json::json!([{"key": "SELECT", "value": "3+INT"}]);
        assert!(!has_no_engine_effect_token(&with_select));
    }

    // SD31-W29-INTEGRATE (Ruling §18): a pool option gated by a prerequisite
    // the catalog cannot evaluate must not be served wholesale.
    #[test]
    fn has_no_prerequisite_token_refuses_any_pre_star_key_but_allows_a_plain_desc_only_record() {
        let clean = serde_json::json!([{"key": "KEY", "value": "x"}, {"key": "DESC", "value": "x"}]);
        assert!(has_no_prerequisite_token(&clean));
        for pre_key in ["PREABILITY", "PREVARGTEQ", "PREMULT", "PRESKILL", "PRELEVEL", "PREFACT"] {
            let gated = serde_json::json!([{"key": pre_key, "value": "1,whatever"}]);
            assert!(
                !has_no_prerequisite_token(&gated),
                "{pre_key} must be recognised as a prerequisite gate"
            );
        }
    }

    #[test]
    fn an_archetype_gated_rage_power_is_refused_by_the_live_catalog() {
        // Real oracle rows (`adventurers_guide`): each carries
        // `PREABILITY = 1,CATEGORY=Archetype,Barbarian Archetype ~ Giant
        // Stalker` (or the sibling archetype), so none of these three may
        // be served wholesale to every barbarian -- wave 29 adversarial
        // review, CONFIRMED finding.
        let entries = load_pool_catalog(&repo_root());
        for withdrawn_key in ["giant_stalker_defense", "topple_giant", "underfoot"] {
            assert!(
                !entries.iter().any(|e| e.book == "adventurers_guide" && e.key == withdrawn_key),
                "{withdrawn_key} carries an archetype prerequisite and must not reach the catalog"
            );
        }
    }

    #[test]
    fn pool_catalog_index_is_keyed_by_book_and_key() {
        let entries = load_pool_catalog(&repo_root());
        let index = pool_catalog_index(&entries);
        assert_eq!(
            index.get(&("core_rulebook".to_string(), "Rogue Talent ~ Ledge Walker".to_string())),
            Some(&"This ability allows you to move along narrow surfaces at full speed using the Acrobatics skill without penalty. In addition, you are not flat-footed when using Acrobatics to move along narrow surfaces.".to_string())
        );
        assert!(!index.contains_key(&("core_rulebook".to_string(), "Rogue Talent ~ Bleeding Attack".to_string())));
    }
}

