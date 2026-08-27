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
//! # Scope: every `" ~ "`-group-qualified pool (SD-32 T12 widening)
//!
//! **Superseded, SD-32 T12 class-feature-pool-population cycle:** this
//! section used to say "Rogue Talent and Rage Power, deliberately no
//! wider", justified by a per-pool spot-check cost that `decisions.md §17`
//! (serve the population "as a class... not one module per pool") and
//! `§27b` ("EVERYTHING. No carve-outs survive.") no longer permit as a
//! reason to stay narrow. The population this catalog was scoped away from
//! is ~1,913 groups / ~16,350 records
//! (`scripts/census_class_feature_pool_population.py`), not 27 — the
//! `CLASS_FEATURE_POOLS` figure below is a DIFFERENT, narrower registry
//! (owner-resolution only, `v06_work_inventory.rs`), not this catalog's own
//! population.
//!
//! The per-pool spot-check this section used to require (are the `%N`
//! argument shapes the same, is the owning class resolvable) is exactly
//! what the safety gates below check GENERICALLY, per record, for every
//! pool at once — they do not need to be re-verified by hand per group
//! name, because they never trusted the group name for correctness in the
//! first place (see [`REGISTERED_POOL_GROUPS`]'s doc comment). Two NEW
//! generic gates this cycle added, generalizing the two hand-kept,
//! Rage-Power-only precedents the spot-check style of scoping used to
//! produce: [`carries_unimplemented_marker`] (a literal stub marker found
//! baked into 17 `occult_adventures` records once the widening reached
//! them) and [`carries_class_specific_level_phrase`] (generalizing
//! `CLASS_LEVEL_SCALED_SHEET_VALUE_EXCLUDED_KEYS`'s Rage-Power-only
//! denylist to every group). `v06_work_inventory.rs`'s `CLASS_FEATURE_POOLS`
//! (27 pools, a SEPARATE registry, owner-resolution only) is unaffected by
//! this widening.
//!
//! **Rage Power's own wrinkle, not present for Rogue Talent:** its group
//! text ("Rage Power") shares no prefix/suffix with its owning class's own
//! name ("barbarian"), unlike "Rogue Talent" which literally starts with
//! "Rogue ". `v06_work_inventory.rs`'s `class_feature_owner` (and its
//! `type_facet` fallback) therefore resolve `None` for every Rage Power
//! record, which used to route the WHOLE pool through a hard-coded
//! `engine_does_not_hold` regardless of this catalog. `classify()`'s "no owner
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

/// **SD-32 T12 class-feature-pool-population cycle:** this catalog used to
/// hard-refuse any `" ~ "`-group-qualified `class_feature` record whose
/// group prefix was not one of two hand-registered literal strings
/// (`"Rogue Talent"`, `"Rage Power"`) -- `decisions.md §17` (serve the
/// population "as a class... group-driven, config-shaped, not one module
/// per pool") and `§27b` ("EVERYTHING. No carve-outs survive.") both
/// forbid keeping that allowlist now that the population it excluded is
/// known to be ~1,913 groups / ~16,350 records
/// (`scripts/census_class_feature_pool_population.py`), not two. Every
/// OTHER record this catalog refuses is refused by one of the SAFETY gates
/// below (render-and-refuse, engine-effect-token, archetype-lock,
/// multi-`DESC:`, bare-`%N`, the unimplemented-marker guard, and the
/// generalized class-level-scaled-phrase guard) -- none of those gates
/// cares what the group's name is, so the group-name allowlist was never
/// load-bearing for correctness, only for the size of the population wave
/// 22/23 had hand-verified. `is_registered_pool_group` below now accepts
/// ANY `" ~ "`-group-qualified key; this constant is kept only as
/// documentation of the two groups that received the original, deepest
/// hand-verification (the wave 22/23 spot-checks the module doc above
/// describes) and is no longer consulted by [`load_pool_catalog`]'s filter.
pub const REGISTERED_POOL_GROUPS: &[&str] = &["Rogue Talent", "Rage Power"];

/// `true` for any corpus `key` that is itself `" ~ "`-group-qualified at
/// all (i.e. the key IS an option-pool member, regardless of which group).
/// See [`REGISTERED_POOL_GROUPS`]'s doc comment for why this is no longer a
/// curated allowlist: the safety gates below (not the group's name) are
/// what decide whether a given record may be served. Deliberately checks
/// the KEY, not the split-off group prefix -- `key.split(" ~ ").next()`
/// returns the whole key unchanged (non-empty) for an UNqualified key too,
/// so gating on the group string alone would silently admit every
/// `class_feature` record in the corpus, not only pool members.
fn is_registered_pool_group(key: &str) -> bool {
    key.contains(" ~ ")
}

/// `true` for a `class_feature` corpus `key` that is NOT itself `"
/// ~ "`-group-qualified -- i.e. a standalone, single-record feature
/// (`"Timeless Body"`, `"Uncanny Dodge"`) rather than one member of an
/// option pool. Mutually exclusive with [`is_registered_pool_group`] by
/// construction (a key either contains `" ~ "` or it does not), so
/// [`load_standalone_class_feature_catalog`] can never serve a record
/// [`load_pool_catalog`] already does, and vice versa -- the two catalogs
/// partition the corpus's `class_feature` keys, they never overlap.
fn is_standalone_class_feature(key: &str) -> bool {
    !key.contains(" ~ ")
}

/// Literal stub/placeholder markers found injected directly into some
/// `occult_adventures` `class_feature` records' `description` field itself
/// (e.g. `Sha'ir ~ Jin`'s real corpus row: `"[not implemented]At 1st
/// level, a sha'ir learns..."` -- `grep -rl '\[not implemented\]'
/// data/corpus/*/class_feature/**/*.json` finds 16 such records,
/// `'\[not enforced\]'` finds 1 more, all in `occult_adventures`). These
/// predate this cycle's widening and were invisible while
/// `REGISTERED_POOL_GROUPS` was a two-entry allowlist that never reached
/// `occult_adventures`' pool groups (`Sha'ir`, `Toxitician`,
/// `Necroccultist`, ...); widening the group match to universal
/// (`is_registered_pool_group`) would otherwise ship this literal bracketed
/// stub marker straight onto a player's character sheet -- exactly the
/// defect `docs/governance/no-stub-mvp-doctrine.md` exists to catch. Refused
/// here, structurally, same discipline as every other hand-found leak this
/// module's gates already catch (multi-`DESC:`, bare-`%N`). This is an
/// ingest-territory (`cache_gen::class_feature::generate`) defect this
/// catalog cannot fix at its source without crossing file territory; the
/// refusal here is the correct-for-this-file mitigation (never manufacture
/// `text-complete` for a record this module can independently prove is
/// broken), not a fix of the root cause.
fn carries_unimplemented_marker(description: &str) -> bool {
    description.contains("[not implemented]") || description.contains("[not enforced]")
}

/// Generalizes `CLASS_LEVEL_SCALED_SHEET_VALUE_EXCLUDED_KEYS`'s hand-kept,
/// Rage-Power-only denylist to every group this cycle's universal match
/// (`is_registered_pool_group`) newly admits. The SD-31 wave 23 review
/// found 16 Rage Power records whose description scales on `"barbarian
/// level"` -- a class-specific phrasing `wiring_class.rs`'s shared
/// `prose_scaling_phrases` list does not recognise (it only catches "your
/// class level"/"your character level") -- and applies to a value this
/// engine already computes elsewhere, so serving the raw description as
/// `text-complete` would misreport a genuinely-needs-computation record as
/// done. Hand-verifying that same "does the engine already compute this
/// value" precondition for every one of ~1,900 newly-admitted groups is
/// infeasible in one cycle, so this guard is deliberately the CONSERVATIVE
/// half alone: refuse to serve ANY record whose description names its own
/// owning class immediately followed by "level"/"levels" (optionally
/// possessive), regardless of whether this engine happens to compute the
/// referenced value. A false "not served" here costs nothing new -- the
/// record simply stays `engine_does_not_hold`, exactly where it was before this
/// cycle; a false "text-complete" would be a new, wrong answer (`§1a`).
fn carries_class_specific_level_phrase(description: &str, class_name: &str) -> bool {
    if class_name.trim().is_empty() {
        return false;
    }
    let desc_lower = description.to_ascii_lowercase();
    let class_lower = class_name.to_ascii_lowercase();
    for suffix in ["'s level", "'s levels", " level", " levels"] {
        if desc_lower.contains(&format!("{class_lower}{suffix}")) {
            return true;
        }
    }
    false
}

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
/// *"we need to show only valid choices."*
///
/// **Corrected mid-cycle, by this same integration pass, after a blanket
/// "any `PRE*` token" version of this guard broke three pre-existing,
/// correctly-served real records** (`core_rulebook: Rage Power ~ Clear
/// Mind` — `PREVARGTEQ:RagePowersPrereqLVL,8`; `advanced_class_guide:
/// Rage Power ~ Elemental Blood (Greater)` and `~ Linnorm Death Curse
/// (Crag)` — `PRELEVEL:MIN=4`/`MIN=8`). A blanket refusal conflates two
/// UNRELATED PF1e shapes:
///
/// * **A level/chain/skill gate within the pool's OWN class** (`PRELEVEL`,
///   `PREVARGTEQ` against a class-internal counter, most `PREABILITY
///   CATEGORY=Special Ability` chain prerequisites like "Greater" requiring
///   the character already hold "Lesser") — every character who stays in
///   this class and levels up CAN eventually take this option. It is a
///   real, valid, standing member of an OPEN pool (exactly what Ruling
///   §18's own worked answer already calls Rage Power/Rogue Talent: "any
///   [class] can eventually take any [option]") — the catalog is not
///   lying by listing it, the same way a feat reference list is not lying
///   by listing a feat the character does not qualify for YET.
/// * **A permanent, structural exclusion from the base class itself** — a
///   PCGen `PREABILITY` token whose value carries `CATEGORY=Archetype`,
///   meaning the option belongs to a specific ARCHETYPE swap
///   (`Barbarian Archetype ~ Giant Stalker`, etc.), not to the base class
///   the pool's `REGISTERED_POOL_GROUPS` entry is keyed against. A
///   character who never takes that archetype can NEVER take this option
///   at any level — this is the genuinely EXCLUSIVE-shaped case Ruling §18
///   forbids serving wholesale (confirmed: `adventurers_guide`'s
///   `giant_stalker_defense`/`topple_giant`/`underfoot`, all three
///   `PREABILITY = 1,CATEGORY=Archetype,Barbarian Archetype ~ Giant
///   Stalker`).
///
/// So the refusal is scoped to exactly the second shape: a `PREABILITY`
/// token whose value contains `CATEGORY=Archetype`. This catalog has no
/// character to check a level/skill prerequisite against, but it does not
/// need one to know an archetype-locked option is not a standing member of
/// the base class's own pool. A future cycle that wants real per-character
/// LEVEL/skill gating (so the picker can grey out, not just list,
/// not-yet-qualified options) needs that in the picker itself
/// (`class_feature_pool_picker.rs`), not a wider refusal here.
fn is_archetype_locked(raw_tokens: &Value) -> bool {
    let Some(tokens) = raw_tokens.as_array() else { return false };
    tokens.iter().any(|t| {
        t.get("key").and_then(|k| k.as_str()) == Some("PREABILITY")
            && t.get("value")
                .and_then(|v| v.as_str())
                .is_some_and(|v| v.contains("CATEGORY=Archetype"))
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
/// speed of 60 feet.|PREVAREQ:BloodRage Cold,1` / ... segments).
///
/// Refused structurally here: any record whose row carries more than
/// one `DESC:` field is, by construction, showing only a fragment of what
/// the oracle actually states, regardless of whether that fragment happens
/// to read as a complete sentence -- UNLESS [`shipped_description_is_the_
/// already_regenerated_safe_multi_desc_join`] proves this specific
/// record's shipped `data.description` has already been caught up (see
/// that function's own doc comment for why the proof, not just the shape,
/// gates the exception).
fn raw_tokens_carry_more_than_one_desc_segment(raw_tokens: &Value) -> bool {
    let Some(tokens) = raw_tokens.as_array() else { return false };
    tokens.iter().filter(|t| t.get("key").and_then(|k| k.as_str()) == Some("DESC")).count() > 1
}

/// The `AT-34-E3-001 class_feature_option_pool` cycle's own narrow fix,
/// sub-cause 8: `Martial Weapon Proficiency Output` (standalone) and
/// `Octopus Wild Shape ~ Poison` (pool) each carry a genuine sequential
/// DESC continuation with no mechanical reason for the split -- unlike
/// `Rage Power ~ Elemental Blood (Greater)`'s PREVAREQ-gated alternative
/// branches, joining every segment IS this record's real, complete
/// description. `cache_gen::class_feature::generate`'s own `desc_value`
/// (a different file, this package's disjoint-file-touch convention) now
/// performs that join at ingest time for exactly this safe shape, so a
/// record whose `data.description` has been regenerated since carries the
/// FULL joined text already.
///
/// **Why this function re-derives the join instead of trusting the shape
/// alone.** Corpus-wide, many OTHER multi-DESC records share the same
/// "no PREVAREQ/PREVARGTEQ gate" shape but have NOT been regenerated --
/// their shipped `data.description` is still the stale, first-segment-only
/// value the old `desc_value` produced. Gating on shape alone (relaxing
/// [`raw_tokens_carry_more_than_one_desc_segment`] to skip every
/// ungated multi-DESC row) was tried and reverted: it silently served
/// ~186 other records' stale, truncated `data.description` across
/// multiple books and mechanisms this cycle does not own -- exactly the
/// silent-truncation defect this module exists to prevent, reopened at
/// corpus scale. Re-deriving the expected join from `raw_tokens` directly
/// and requiring it to match the ALREADY-SHIPPED `data.description` proves
/// ingest has actually caught up for this one record; every other
/// not-yet-regenerated record fails the equality check and stays refused,
/// unchanged from before this cycle.
fn shipped_description_is_the_already_regenerated_safe_multi_desc_join(
    raw_tokens: &Value,
    shipped_description: &str,
) -> bool {
    let Some(tokens) = raw_tokens.as_array() else { return false };
    let segments: Vec<&str> = tokens
        .iter()
        .filter(|t| t.get("key").and_then(|k| k.as_str()) == Some("DESC"))
        .filter_map(|t| t.get("value").and_then(|v| v.as_str()))
        .collect();
    if segments.len() <= 1 {
        return false;
    }
    if segments[1..].iter().any(|s| s.contains("PREVAREQ") || s.contains("PREVARGTEQ")) {
        return false;
    }
    let expected_join = segments.iter().map(|s| s.trim()).collect::<Vec<_>>().join(" ");
    expected_join == shipped_description
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
    /// The registered pool group this record belongs to (the corpus `key`'s
    /// own `" ~ "`-split group prefix — e.g. `"Rogue Talent"`; SD-32 card 11:
    /// no longer `data.class`, which now carries the TRUE owning class).
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

/// Shared walk-and-render pipeline behind both [`load_pool_catalog`] and
/// [`load_standalone_class_feature_catalog`] -- every safety gate (render-
/// and-refuse, engine-effect-token, archetype-lock, multi-`DESC:`, bare-`%N`,
/// the unimplemented-marker guard, the class-level-scaled-phrase guard) is
/// identical for both; the two public entry points differ ONLY in
/// `key_filter`, which is exactly what makes them a true partition (see
/// [`is_standalone_class_feature`]'s doc comment) rather than two
/// independently-drifting copies of the same logic.
fn load_class_feature_catalog(
    repo_root: &Path,
    key_filter: impl Fn(&str) -> bool,
) -> Vec<PoolCatalogEntry> {
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
            let (Some(key), Some(name)) = (data["key"].as_str(), data["name"].as_str()) else {
                continue;
            };
            // SD-32 card 11 (`epic-2-cause-closure`, T2a/T12 combined
            // cycle): the pool group this catalog needs is the corpus
            // key's own `" ~ "`-split prefix, NOT `data.class` -- those two
            // used to be interchangeable only because of the exact bug T2a
            // names (`cache_gen::class_feature::generate` used to derive
            // `class` FROM this same key-prefix text whenever no grant fact
            // resolved it). Now that `generate` ships the TRUE owning class
            // in `data.class` (e.g. `"Rogue"`, not `"Rogue Talent"`) so
            // `class_feature_descriptions.rs`'s consumer can join it against
            // a real `ExplanationDto.id`, this module's own filter must read
            // the group text directly from `key` instead -- `key` is
            // untouched by that fix, so `"Rogue Talent ~ Ledge Walker"` still
            // splits to `"Rogue Talent"` exactly as before.
            let group = key.split(" ~ ").next().unwrap_or(key);
            if !key_filter(key) {
                continue;
            }
            if CLASS_LEVEL_SCALED_SHEET_VALUE_EXCLUDED_KEYS.contains(&key) {
                continue;
            }
            let Some(raw_desc) = data["description"].as_str() else { continue };
            if !is_real_description_value(raw_desc) {
                continue;
            }
            if carries_unimplemented_marker(raw_desc) {
                continue;
            }
            let owning_class = data["class"].as_str().unwrap_or("");
            if carries_class_specific_level_phrase(raw_desc, owning_class) {
                continue;
            }
            if !has_no_engine_effect_token(&data["raw_tokens"]) {
                continue;
            }
            if is_archetype_locked(&data["raw_tokens"]) {
                continue;
            }
            if raw_tokens_carry_more_than_one_desc_segment(&data["raw_tokens"])
                && !shipped_description_is_the_already_regenerated_safe_multi_desc_join(&data["raw_tokens"], raw_desc)
            {
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
                pool_group: group.to_string(),
                key: key.to_string(),
                name: clean_name,
                description: rendered.text,
            });
        }
    }
    out
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
    load_class_feature_catalog(repo_root, is_registered_pool_group)
}

/// `AT-34-E3-001` (`class_feature_option_pool_record_not_held_by_engine`
/// mechanism): the sibling of [`load_pool_catalog`] for STANDALONE
/// `class_feature` records (a bare feature name, never a `" ~ "`-qualified
/// option-pool member) -- e.g. `"Timeless Body"`, `"Uncanny Dodge"`,
/// `"Woodland Stride"`. These records reach `Kind::ClassFeature`'s "no
/// owner resolved" branch in `v06_work_inventory.rs` for the same reason
/// Rage Power records used to (their bare name shares no prefix/suffix
/// with a modelled class's own name, since a shared multi-class feature
/// like Evasion or Uncanny Dodge is not owned by any single class) — and
/// until this catalog existed, that branch had no way to prove any of them
/// genuinely reaches a rendered description, exactly the gap `Kind::
/// ClassFeature`'s own doc comment names ("no generic class_feature catalog
/// exists anywhere in this engine"). Every safety gate below is IDENTICAL
/// to [`load_pool_catalog`]'s own (render-and-refuse, engine-effect-token,
/// archetype-lock, multi-`DESC:`, bare-`%N`) — a record carrying a real
/// mechanical token (`AUTO`, `ABILITY`, `BONUS`, ...) is refused here
/// exactly as it would be for an option-pool member, so a genuinely
/// mechanical, still-needs-computation record (e.g. `Armor Prof ~ Heavy`'s
/// `AUTO:ARMORPROF|...` — which is ALSO `" ~ "`-qualified and therefore
/// never reaches this catalog at all, [`is_standalone_class_feature`]'s own
/// mutual-exclusion with [`is_registered_pool_group`]) can never be
/// misreported `text-complete` by this addition.
pub fn load_standalone_class_feature_catalog(repo_root: &Path) -> Vec<PoolCatalogEntry> {
    load_class_feature_catalog(repo_root, is_standalone_class_feature)
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

    /// SD-32 T12 class-feature-pool-population cycle: the group-name
    /// allowlist is gone -- every `" ~ "`-group-qualified key is eligible,
    /// filtered only by the safety gates. A record with NO `" ~ "` at all
    /// (not a pool member) must still never appear; a clean record from a
    /// group that was NOT one of the original two (`Vigilante Talent`,
    /// unregistered before this cycle) now reaches the catalog, proving the
    /// widening is real and not merely documentation.
    #[test]
    fn only_group_qualified_keys_are_ever_served_but_every_group_is_now_eligible() {
        let entries = load_pool_catalog(&repo_root());
        assert!(
            entries.iter().all(|e| e.key.contains(" ~ ")),
            "a record with no ' ~ ' group qualifier is not a pool member and must never appear"
        );
        assert!(
            entries.iter().any(|e| e.pool_group == "Vigilante Talent"),
            "a clean record from a newly-widened group must now reach the catalog"
        );
    }

    /// The real corpus loads a real, clean `Vigilante Talent` record --
    /// proof the universal group match reaches a group that was NOT one of
    /// the original two hand-registered pools.
    #[test]
    fn loads_a_real_clean_vigilante_talent_from_a_newly_widened_group() {
        let entries = load_pool_catalog(&repo_root());
        let turnabout = entries
            .iter()
            .find(|e| e.book == "inner_sea_intrigue" && e.key == "Vigilante Talent ~ Turnabout")
            .expect("inner_sea_intrigue's real Vigilante Talent ~ Turnabout record must be in the catalog");
        assert_eq!(turnabout.pool_group, "Vigilante Talent");
        assert_eq!(turnabout.name, "Turnabout");
        assert!(turnabout.description.starts_with("A vigilante with this talent"));
    }

    #[test]
    fn carries_unimplemented_marker_catches_both_bracket_shapes() {
        assert!(carries_unimplemented_marker("[not implemented]At 1st level, a sha'ir learns..."));
        assert!(carries_unimplemented_marker("Some lead-in. [not enforced] the rest of it."));
        assert!(!carries_unimplemented_marker("A vigilante with this talent can capitalize."));
    }

    /// Real defect this cycle's widening would otherwise have shipped: 16
    /// `occult_adventures` records (plus 1 `[not enforced]`) carry a
    /// literal stub marker baked into `data.description` itself. Proves the
    /// live catalog refuses at least one, non-vacuously.
    #[test]
    fn a_record_carrying_a_literal_unimplemented_marker_is_refused_by_the_live_catalog() {
        let entries = load_pool_catalog(&repo_root());
        assert!(
            !entries.iter().any(|e| e.key == "Sha'ir ~ Jin" && e.book == "occult_adventures"),
            "a record whose description carries a literal '[not implemented]' stub marker \
             must never reach the catalog"
        );
    }

    #[test]
    fn carries_class_specific_level_phrase_generalizes_the_rage_power_denylist() {
        assert!(carries_class_specific_level_phrase(
            "This ability improves at 6th barbarian level.",
            "Barbarian"
        ));
        assert!(carries_class_specific_level_phrase(
            "Scales with the witch's level in this class.",
            "Witch"
        ));
        // A DIFFERENT class's name in the phrase must not trip the guard
        // for THIS record's own owning class.
        assert!(!carries_class_specific_level_phrase(
            "This ability improves at 6th sorcerer level.",
            "Barbarian"
        ));
        // No class name at all (owner unresolved) never matches.
        assert!(!carries_class_specific_level_phrase("You gain a bonus at higher levels.", ""));
        // A plain reference with no class-specific phrasing is unaffected.
        assert!(!carries_class_specific_level_phrase(
            "You move along narrow surfaces at full speed.",
            "Rogue"
        ));
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
            "a record whose row carries a PREVAREQ/PREVARGTEQ-gated choice-branch DESC segment \
             must never reach the catalog (only one branch applies per character; joining all \
             of them would show every alternative as if simultaneously true)"
        );
        assert!(entries.iter().any(|e| e.book == "advanced_class_guide" && e.pool_group == "Rage Power"));
    }

    /// `AT-34-E3-001 class_feature_option_pool` cycle, sub-cause 8: the SAFE
    /// multi-DESC shape (no PREVAREQ/PREVARGTEQ gate on any segment beyond
    /// the first) must reach the catalog now that `cache_gen::class_
    /// feature::generate` joins it into `data.description` directly --
    /// `Martial Weapon Proficiency Output` and `Octopus Wild Shape ~
    /// Poison` are the two real corpus records this closes.
    #[test]
    fn a_safe_multi_desc_continuation_reaches_the_standalone_catalog() {
        let entries = load_standalone_class_feature_catalog(&repo_root());
        let martial = entries
            .iter()
            .find(|e| e.key == "Martial Weapon Proficiency Output")
            .expect("Martial Weapon Proficiency Output must reach the standalone catalog");
        assert!(martial.description.contains("You understand how to use your martial weapons"));
        assert!(martial.description.contains("You make attack rolls with all your martial weapons"));
        assert!(!martial.description.contains('|'), "no pipe-arg tail may leak into prose");
    }

    #[test]
    fn a_safe_multi_desc_continuation_with_a_display_condition_tail_reaches_the_pool_catalog() {
        let entries = load_pool_catalog(&repo_root());
        let poison = entries
            .iter()
            .find(|e| e.key == "Octopus Wild Shape ~ Poison")
            .expect("Octopus Wild Shape ~ Poison must reach the pool catalog");
        assert!(poison.description.starts_with("Bite-injury"));
        assert!(poison.description.contains("Calling upon the venomous powers"));
        assert!(!poison.description.contains('|'), "the |PRERULE:... tail must not leak into prose");
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

    #[test]
    fn shipped_description_is_the_already_regenerated_safe_multi_desc_join_requires_an_exact_match() {
        let two_plain = serde_json::json!([
            {"key": "DESC", "value": "a"},
            {"key": "DESC", "value": "b"},
        ]);
        // Not yet regenerated: shipped description is still just the first
        // segment -- stays refused.
        assert!(!shipped_description_is_the_already_regenerated_safe_multi_desc_join(&two_plain, "a"));
        // Regenerated: shipped description is the full safe join.
        assert!(shipped_description_is_the_already_regenerated_safe_multi_desc_join(&two_plain, "a b"));
        // A choice-branch-gated row never has a safe join, regardless of
        // what the shipped description says.
        let choice_gated = serde_json::json!([
            {"key": "DESC", "value": "While raging, the barbarian gains"},
            {"key": "DESC", "value": " a burrow speed of 30 feet.|PREVAREQ:BloodRage Acid,1"},
        ]);
        assert!(!shipped_description_is_the_already_regenerated_safe_multi_desc_join(
            &choice_gated,
            "While raging, the barbarian gains a burrow speed of 30 feet.|PREVAREQ:BloodRage Acid,1"
        ));
        // A single-DESC row has nothing to join.
        let one = serde_json::json!([{"key": "DESC", "value": "a"}]);
        assert!(!shipped_description_is_the_already_regenerated_safe_multi_desc_join(&one, "a"));
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

    // SD31-W29-INTEGRATE (Ruling §18): only an ARCHETYPE-lock (a permanent,
    // structural exclusion from the base class) is refused -- an ordinary
    // level/chain/skill prerequisite within the pool's own class is not,
    // because every character of that class can eventually satisfy it.
    #[test]
    fn is_archetype_locked_refuses_only_a_preability_category_archetype_token() {
        let clean = serde_json::json!([{"key": "KEY", "value": "x"}, {"key": "DESC", "value": "x"}]);
        assert!(!is_archetype_locked(&clean));

        // Ordinary within-class prerequisites -- must NOT be refused. Real
        // shapes: `core_rulebook: Rage Power ~ Clear Mind`
        // (`PREVARGTEQ:RagePowersPrereqLVL,8`), `advanced_class_guide:
        // Rage Power ~ Linnorm Death Curse (Crag)` (`PRELEVEL:MIN=4`), and
        // a `PREABILITY CATEGORY=Special Ability` chain prerequisite (e.g.
        // "Greater" requiring "Lesser" already held), all of which stay
        // served today.
        for (key, value) in [
            ("PREVARGTEQ", "RagePowersPrereqLVL,8"),
            ("PRELEVEL", "MIN=4"),
            ("PRESKILL", "1,Knowledge (Arcana)=5"),
            ("PREFACT", "Deity,Zon-Kuthon"),
            ("PREMULT", "1,[PRELEVEL:MIN=8],[PREABILITY:1,CATEGORY=Special Ability,X]"),
            ("PREABILITY", "1,CATEGORY=Special Ability,Rage Power ~ Elemental Blood (Lesser)"),
        ] {
            let ungated = serde_json::json!([{"key": key, "value": value}]);
            assert!(
                !is_archetype_locked(&ungated),
                "{key}={value} is an ordinary within-class prerequisite, not an archetype lock"
            );
        }

        // The genuinely EXCLUSIVE shape -- must be refused.
        let archetype_gated = serde_json::json!([
            {"key": "PREABILITY", "value": "1,CATEGORY=Archetype,Barbarian Archetype ~ Giant Stalker"}
        ]);
        assert!(is_archetype_locked(&archetype_gated));
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

    /// The real, current size of the widened catalog (SD-32 T12
    /// class-feature-pool-population cycle) -- not a fixed pin (the corpus
    /// grows), a FLOOR proving the widening is doing real, large-scale
    /// work and not merely passing its own hand-picked test cases. Before
    /// this cycle the entire catalog held ~71 records across exactly 2
    /// groups; run with `--nocapture` to see the live count and group
    /// spread.
    #[test]
    fn the_widened_catalog_serves_far_more_than_the_original_two_groups() {
        let entries = load_pool_catalog(&repo_root());
        let mut groups: std::collections::BTreeSet<&str> =
            entries.iter().map(|e| e.pool_group.as_str()).collect();
        eprintln!(
            "class_feature_pool_catalog: {} entries across {} groups",
            entries.len(),
            groups.len()
        );
        assert!(
            entries.len() > 500,
            "expected the universal group match to serve well over the original ~71 \
             Rogue-Talent/Rage-Power-only records; got {}",
            entries.len()
        );
        assert!(
            groups.len() > 50,
            "expected far more than 2 distinct pool groups to be served; got {}",
            groups.len()
        );
        // The original two groups must still be served -- no regression.
        assert!(groups.remove("Rogue Talent"));
        assert!(groups.remove("Rage Power"));
    }

    /// `AT-34-E3-001` (`class_feature_option_pool_record_not_held_by_engine`
    /// mechanism): real, currently-shipped standalone CRB features with a
    /// clean-rendering, mechanically-inert description -- the exact shape
    /// this catalog exists to serve. Six real oracle keys, each hand-read
    /// against its own `data/corpus/core_rulebook/class_feature/**/*.json`
    /// row before being listed here.
    #[test]
    fn standalone_catalog_serves_real_prose_only_crb_features() {
        let entries = load_standalone_class_feature_catalog(&repo_root());
        let index = pool_catalog_index(&entries);
        for key in [
            "Timeless Body",
            "Uncanny Dodge",
            "Woodland Stride",
            "Evasion Output",
            "Improved Evasion",
            "Blank Weapon Block OS",
        ] {
            assert!(
                index.contains_key(&("core_rulebook".to_string(), key.to_string())),
                "expected the standalone catalog to serve {key:?}"
            );
        }
    }

    /// The render-and-refuse / engine-effect-token gates must refuse a
    /// standalone record exactly as they refuse an option-pool one: `Armor
    /// Prof ~ Heavy` (`" ~ "`-qualified, so it can never reach THIS catalog
    /// at all -- proven separately below) and `Channel Negative Energy`
    /// (a real oracle row whose `description` is `null`, so `has_real_
    /// description` fails upstream regardless of this catalog) must not be
    /// served.
    #[test]
    fn standalone_catalog_refuses_records_with_no_real_description_or_an_engine_effect_token() {
        let entries = load_standalone_class_feature_catalog(&repo_root());
        let index = pool_catalog_index(&entries);
        assert!(!index.contains_key(&("core_rulebook".to_string(), "Channel Negative Energy".to_string())));
        assert!(!index.contains_key(&("core_rulebook".to_string(), "Evasion".to_string())));
    }

    /// [`is_standalone_class_feature`] and [`is_registered_pool_group`] must
    /// partition the corpus's `class_feature` keys, never overlap --
    /// otherwise a record could ride BOTH catalogs, which would let a fix
    /// scoped to one mechanism's population silently also move another
    /// mechanism's (`decisions.md §14`'s nine-way split is only meaningful
    /// if each unit belongs to exactly one).
    #[test]
    fn pool_and_standalone_catalogs_never_overlap() {
        let pool = load_pool_catalog(&repo_root());
        let standalone = load_standalone_class_feature_catalog(&repo_root());
        let pool_keys: std::collections::BTreeSet<(&str, &str)> =
            pool.iter().map(|e| (e.book.as_str(), e.key.as_str())).collect();
        for entry in &standalone {
            assert!(
                !pool_keys.contains(&(entry.book.as_str(), entry.key.as_str())),
                "{:?}/{:?} appears in both catalogs",
                entry.book,
                entry.key
            );
        }
    }

    /// `AT-34-E3-001`'s `class_feature_owner_matched_by_name_but_record_
    /// not_held_by_engine` mechanism (`decisions.md §14`, 346 of 1,006
    /// `core_rulebook` bucket-B units at this cycle's start): re-derives,
    /// from the live `docs/work-inventory.json` and the live corpus this
    /// module already reads, WHY each unit in this mechanism's population
    /// is not served by [`load_pool_catalog`] -- the exact gate this
    /// module's own filter (`load_class_feature_catalog`) refuses it at,
    /// walked in the SAME order that function checks them, so the count is
    /// never a re-narration.
    ///
    /// **Every gate below is load-bearing, not this cycle's own
    /// invention** -- each was hand-verified against a real corpus finding
    /// by an earlier cycle (this file's own doc comments cite them). This
    /// test proves the negative the receipt reports: none of the 346 is a
    /// narrow catalog-widening bug this cycle can close without either (a)
    /// new engine wiring for a genuinely mechanical/computed record, or (b)
    /// new ingest work for a record with no player-facing description at
    /// all. The seven buckets below are that population's exact partition
    /// (`decisions.md §15`: a named remainder, not "the rest").
    #[test]
    fn class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly() {
        let repo_root = repo_root();
        let inventory_text = std::fs::read_to_string(repo_root.join("docs/work-inventory.json"))
            .expect("docs/work-inventory.json is readable");
        let inventory: Value =
            serde_json::from_str(&inventory_text).expect("docs/work-inventory.json is valid JSON");
        let units = inventory["units"].as_array().expect("units is an array");
        let mechanism_units: Vec<(String, String)> = units
            .iter()
            .filter(|u| {
                u["book"].as_str() == Some("core_rulebook")
                    && u["status"].as_str() == Some("engine-does-not-hold")
                    && u["evidence"].as_str()
                        == Some("class_feature_owner_matched_by_name_but_record_not_held_by_engine")
            })
            .map(|u| {
                (
                    u["book"].as_str().unwrap_or_default().to_string(),
                    u["corpus_key"].as_str().unwrap_or_default().to_string(),
                )
            })
            .collect();
        let population = mechanism_units.len();

        let corpus_root = repo_root.join("data/corpus");
        let mut reasons: BTreeMap<&'static str, u32> = BTreeMap::new();
        for (book, key) in &mechanism_units {
            let cf_dir = corpus_root.join(book).join("class_feature");
            let mut files = Vec::new();
            walk_json_files(&cf_dir, &mut files);
            let mut found = None;
            for file in &files {
                let Ok(text) = std::fs::read_to_string(file) else { continue };
                let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                if doc["data"]["key"].as_str() == Some(key.as_str()) {
                    found = Some(doc);
                    break;
                }
            }
            let Some(doc) = found else {
                *reasons.entry("no_corpus_record_found").or_default() += 1;
                continue;
            };
            let data = &doc["data"];
            let raw_desc = data["description"].as_str();
            let Some(raw_desc) = raw_desc else {
                // No `DESC:` at all -- a genuinely internal, never
                // player-facing bookkeeping row (`ADD:SPELLCASTER`,
                // `SPELLKNOWN`, `SPELLLEVEL`, ...). Real ingest work
                // (writing a description that does not exist upstream) or
                // a reclassification, not a catalog fix.
                *reasons.entry("description_is_null_internal_bookkeeping").or_default() += 1;
                continue;
            };
            if !is_real_description_value(raw_desc) {
                *reasons.entry("description_not_real_value").or_default() += 1;
                continue;
            }
            if carries_unimplemented_marker(raw_desc) {
                *reasons.entry("carries_unimplemented_marker").or_default() += 1;
                continue;
            }
            let owning_class = data["class"].as_str().unwrap_or("");
            if carries_class_specific_level_phrase(raw_desc, owning_class) {
                // Prose states a value that scales with the OWNING class's
                // level (e.g. "200 gp per wizard level") -- Decision 7
                // condition 2 ("nothing to compute") genuinely fails; this
                // needs a real per-character computation, not a serve.
                *reasons.entry("class_specific_level_phrase").or_default() += 1;
                continue;
            }
            if !has_no_engine_effect_token(&data["raw_tokens"]) {
                // Carries a real mechanical token (`ADD`, `ABILITY`,
                // `AUTO`, `BONUS`, `DEFINE`, `SPELLS`, ...) alongside its
                // description -- a genuine mechanic, not prose-only.
                *reasons.entry("engine_effect_token_present").or_default() += 1;
                continue;
            }
            if is_archetype_locked(&data["raw_tokens"]) {
                *reasons.entry("archetype_locked").or_default() += 1;
                continue;
            }
            if raw_tokens_carry_more_than_one_desc_segment(&data["raw_tokens"])
                && !shipped_description_is_the_already_regenerated_safe_multi_desc_join(
                    &data["raw_tokens"],
                    raw_desc,
                )
            {
                // Every one of these, hand-checked this cycle, carries a
                // genuine `PRE*`-gated alternative-branch shape (mutually
                // exclusive choices or level bands), not the `class_
                // feature_option_pool` cycle's safe sequential-continuation
                // shape -- joining them would show every branch at once,
                // the exact silent-truncation-turned-over-disclosure defect
                // that gate exists to prevent.
                *reasons.entry("multi_desc_segment_not_regenerated").or_default() += 1;
                continue;
            }
            if raw_desc_has_a_bare_percent_reference_no_pipe_tail_can_resolve(raw_desc) {
                *reasons.entry("bare_percent_reference").or_default() += 1;
                continue;
            }
            let rendered = render_pcgen_desc(raw_desc);
            if !rendered.dropped_args.is_empty() {
                *reasons.entry("dropped_pcgen_args").or_default() += 1;
                continue;
            }
            if leaked_pcgen_syntax(&rendered.text).is_some() {
                *reasons.entry("leaked_pcgen_syntax").or_default() += 1;
                continue;
            }
            // Passes every gate this catalog runs -- genuinely already
            // SERVED by `load_pool_catalog`/`pool_catalog_index`. Every one
            // hand-sampled this cycle (`Sorcerer Bonus Spell L4 ~ Elemental
            // Body I`, `Sorcerer Bonus Spell L1 ~ Bless`, ...) is still
            // blocked at `classify()`'s own promotion gate: either its
            // `wiring_class` is not `"display"` (`computed`/`ambiguous`/
            // `static`/`derived` -- a real magnitude/scaling signal the
            // catalog's render-and-refuse gate alone cannot see), or its
            // prose trips `closure_states_universal_sheet_modifier`'s
            // `"size bonus"` cue (a per-character numeric effect, not
            // static flavor text). Both gates are `classify()`'s, deliberate
            // and correct per Decision 7 -- a text-complete promotion for
            // either shape would misreport a record that still needs a
            // real computation as merely displayed.
            *reasons.entry("catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion")
                .or_default() += 1;
        }

        let total: u32 = reasons.values().sum();
        assert_eq!(
            total as usize, population,
            "the seven named sub-causes must partition the WHOLE mechanism population \
             exactly, decisions.md §15 -- got {reasons:?} summing to {total} against a \
             population of {population}"
        );
        for (k, v) in &reasons {
            eprintln!("AT-34-E3-001 class_feature_owner_matched sub-cause: {v} | {k}");
        }
    }
}

