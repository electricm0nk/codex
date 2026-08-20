//! Per-record `class_feature` GRANT-fact ingest (SD-31 wave 22, rebuild of
//! wave 21's `SD31-W21-CF-GRANT-001`, which was `GAMED-REJECTED`,
//! `OPEN-ISSUES.md` row 334 -- read that row and
//! `worktree-wf_a45ece26-3fc-1:src/rules_core/cache_gen/class_feature_grants.rs`
//! before touching this file again).
//!
//! ## The fact this module ingests, and why nothing ingested it before
//!
//! A `class_feature` corpus record (e.g. `Wizard ~ Spells`) carries its own
//! row: `KEY:`, `DESC:`, `CATEGORY:`. It does **not** carry the fact "a
//! Wizard gains this at level 1" -- that fact lives on a physically
//! SEPARATE row elsewhere in the same file, an `ABILITY:` grant TOKEN.
//! This module reads every such token, resolves it to `(class, key,
//! level)`, and writes the result to `data/class_feature_grants/` -- never
//! to `data/corpus/` (see "Output location" below), and never consumed by
//! `pilot_compute`/`wiring_class.rs` (both untouched, out of this lane's
//! file scope).
//!
//! ## Why wave 21's version was rejected, and what is different here
//!
//! `OPEN-ISSUES.md` row 334, re-derived independently in this cycle
//! against the SAME oracle row it cites
//! (`adventurers_guide/ag_abilities_class.lst:882`,
//! `Sigilus ~ Inscribe Sihedron`): the prior module read `ABILITY:`
//! token segment 2 alone (the `<key>`) and split the class off the KEY's
//! own `<X> ~ <Y>` text -- `"Sigilus"`, an ARCHETYPE name, not a class.
//! The real grant is gated `PRECLASS:1,Magus=7`, which this prior version
//! never read at all (it recognised only `PREVARGTEQ:`), so it silently
//! fell through to its ungated level-1 default -- fabricating a level-1
//! Magus feature that the game actually grants at level 7, and crediting
//! it to a class ("Sigilus") that does not exist as a playable class.
//!
//! **This version never derives class or level from the grant token's own
//! `<key>` text.** Two independent, oracle-verified sources supply
//! `(class, level)`, and this module refuses rather than guesses when
//! neither applies:
//!
//! 1. **`PRECLASS:<count>,<Class>=<Level>` on the grant token itself.**
//!    `adventurers_guide:882`'s own gate names both facts directly:
//!    `PRECLASS:1,Magus=7` IS "Magus, level 7" -- no inference needed. Used
//!    only when `count == 1` and exactly one class clause parses with an
//!    explicit `=<Level>` (a genuine single-class prerequisite); a
//!    multi-class gate (`PRECLASS:2,Cavalier=1,Ranger=1`, confirmed
//!    present at `apg_abilities_class.lst:3725` and siblings -- "must be
//!    BOTH classes", not "granted by either") is refused, not resolved,
//!    because no single class can be named as sole owner.
//! 2. **The row's own `CATEGORY=Class|<Class>[ ~ <Variant>].MOD` target**
//!    (`core_rulebook:316`'s `CATEGORY=Class|Wizard.MOD`,
//!    `core_rulebook:145`'s `CATEGORY=Class|Barbarian ~ Standard Class
//!    Full.MOD`) -- this line's field 0 states, in PCGen's own syntax,
//!    "modify class object `<Class>`", so every `ABILITY:` token the SAME
//!    row carries is granted BY that class. Combined with the token's own
//!    `PREVARGTEQ:<Var>,<N>` gate when present (level `N`, explicit), or
//!    with NO gate at all (level 1, implicit -- verified independently of
//!    Pathfinder Unchained against `core_rulebook:148`'s ungated `Barbarian
//!    ~ Uncanny Dodge Tracker`, a Core Rulebook row PU never touches: the
//!    Barbarian chassis grants class levels from 1st, so an ungated grant
//!    on ITS OWN class's `.MOD` row means level 1, PCGen's own semantics,
//!    not an assumption borrowed from PU's convention).
//!
//! If a `PRECLASS:<Class>=<Level>` gate names a DIFFERENT class than the
//! row's own `CATEGORY=Class|<Class>.MOD` target, this module refuses
//! rather than pick one -- a real, if rare, contradiction worth a human
//! looking at, not a coin flip.
//!
//! **Every other gate shape is refused, not defaulted**: `PREABILITY`
//! (conditioned on possessing another ability, no level stated),
//! `PREDOMAIN`/`PRESKILL`/`PRETEMPLATE`/`PRESIZEEQ`/`PREARMORTYPE` (not a
//! class-level gate at all), `PREVAREQ`-only (a binary flag, not a level
//! threshold), any negated `!PRE*` gate, and an `ABILITY:` token that sits
//! on neither a `CATEGORY=Class.MOD` row nor carries its own resolvable
//! `PRECLASS:`. [`GenerationReport::unresolved`] carries every refusal
//! with its reason, sized as a headline number, per this wave's own
//! standing instruction to report the honest ceiling rather than an
//! inflated ceiling.
//!
//! ## A second, independently-confirmed prior defect: dropped sibling keys
//!
//! One `ABILITY:` token can grant MORE than one key sharing the same gate
//! -- `core_rulebook:384`'s
//! `ABILITY:Pathfinder Chronicler Class Feature|AUTOMATIC|Pathfinder
//! Chronicler ~ Bardic Knowledge|Pathfinder Chronicler ~ Deep
//! Pockets|Pathfinder Chronicler ~ Master Scribe|PREVARGTEQ:PathfinderChronicler_CFP_Level,1`
//! names THREE keys before its single trailing gate. The prior module read
//! only segment 2, silently dropping `Deep Pockets` and `Master Scribe`.
//! [`split_ability_token`] collects every ` ~ `-shaped segment between the
//! key position and the first `PRE*`-prefixed segment, so all three keys
//! resolve, all sharing the one gate -- not a fabrication risk (nothing
//! was invented), but a real, silent, coverage-losing bug this module no
//! longer has.
//!
//! ## The two-classes-share-a-feature case, verified against the oracle
//!
//! `core_rulebook:206`: `CATEGORY=Class|Bard.MOD` grants `Bard ~ Bardic
//! Knowledge` at level 1 (`PREVARGTEQ:Bard_CFP_Level,1`).
//! `core_rulebook:384`: `CATEGORY=Class|Pathfinder Chronicler.MOD` grants
//! `Pathfinder Chronicler ~ Bardic Knowledge` at level 1
//! (`PREVARGTEQ:PathfinderChronicler_CFP_Level,1`) -- a DIFFERENT corpus
//! `KEY:`, owned by a different class, both real, both correctly resolved
//! as two separate `GrantFact`s (`bard_and_pathfinder_chronicler_...` test
//! below). Confirms the module preserves `(class, feature-key, level)` as
//! a fact, never collapsing to `(feature-key, level)` -- the exact shape
//! the wave-21 rejection named as necessary and this key-per-token,
//! class-per-row design gives for free, by construction.
//!
//! ## Cross-check against the real corpus, not a same-file heuristic
//!
//! A resolved grant fact's `key` is checked against the REAL, already-
//! ingested `class_feature` corpus (`data/corpus/*/class_feature/**/*.json`,
//! `data.key`), not merely "does a `KEY:` line exist somewhere in this same
//! raw file" -- some grant tokens live in one book's primary file but the
//! feature they grant is corpus-owned by ANOTHER book (`advanced_players_guide`'s
//! grant token for `Druid ~ Wild Shape` at level 6; the record itself
//! lives under `core_rulebook`), confirmed live this cycle. A grant whose
//! key matches no real corpus record anywhere (191 of 3,618 resolved facts,
//! this cycle's own re-derivation -- overwhelmingly not-yet-ingested
//! archetype replacement features, e.g. `ultimate_combat`'s
//! `Dervish Dancer ~ ...` block) is written with `corpus_record_exists:
//! false` rather than dropped silently: the grant fact itself is real and
//! resolvable; it simply has nothing to attach to yet, which is a
//! different, honestly-reported condition from an unresolved gate shape.
//!
//! ## A related, pre-existing, OUT-OF-SCOPE defect this cycle found and did NOT fix
//!
//! `cache_gen::class_feature::ClassFeatureData.class` (a file this lane may
//! write but was not asked to touch, and a wholesale `data/corpus/` regen
//! is forbidden mid-wave) is populated by the SAME `key`-prefix split this
//! module's predecessor used, per that module's own doc comment ("split
//! off `key`'s ` ~ ` separator... the same split `class_feature_owner`
//! ... already perform"). Confirmed live: the shipped corpus record
//! `data/corpus/adventurers_guide/class_feature/sigilus/inscribe_rune.json`
//! carries `"class": "Sigilus"` for the exact same reason wave 21 was
//! rejected -- "Sigilus" is an archetype, not a class; the real class is
//! Magus. This module's own `GrantFact::class` for that same key resolves
//! correctly (`Magus`, via `PRECLASS:1,Magus=2`) and is NOT written back
//! into that field -- doing so would be a `data/corpus/` regen, out of
//! this lane's scope and this wave's regen discipline. Logged for a future
//! cycle, not corrected here.
//!
//! ## Output location: `data/class_feature_grants/`, NOT `data/corpus/`
//!
//! Confirmed by wave 21's own history: `data/corpus/` is a CLOSED set of
//! Shape B v1 records (`tests/sd27_license_stripping_shape_v1.rs`,
//! `tests/sd26_cache_core_rulebook.rs`), and an array-of-facts file broke
//! both on the first full-suite run when tried there. This module's output
//! lives under `data/class_feature_grants/<book>/<class-slug>.json`
//! instead, a sibling tree, never touched by either gate.
//!
//! ## Deliberately NOT touched
//!
//! This module writes data. It does not decide `wiring_class`, does not
//! compute a doneness verdict, and is never called from `pilot_compute` or
//! `wiring_class.rs`. "A grant fact resolves" and "a record is done" are
//! different questions -- conflating them is the Decision 1(a) violation
//! this package forbids.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Serialize;

use crate::rules_core::cache_gen::class_feature::{BOOK_PRIMARY_FILES, sha256_file};
use crate::rules_core::pi_screening;
use crate::rules_core::shape_b_v1::License;

// ---------------------------------------------------------------------
// Gate resolution
// ---------------------------------------------------------------------

/// Which rule resolved a [`GrantFact`]'s `(class, level)` -- carried for
/// audit, never re-interpreted downstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GateKind {
    /// Resolved from the grant token's own `PRECLASS:1,<Class>=<N>` gate.
    Preclass,
    /// Resolved from the row's `CATEGORY=Class|<Class>...MOD` target plus
    /// the token's own `PREVARGTEQ:<Var>,<N>` gate.
    ModRowGated,
    /// Resolved from the row's `CATEGORY=Class|<Class>...MOD` target with
    /// no `PRE*` gate at all (implicit level 1).
    ModRowUngated,
}

/// One resolved (class, feature, level) GRANT fact. See the module doc
/// comment for the token shape and resolution rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrantFact {
    /// The record's own corpus `KEY:`-shaped identity, verbatim.
    pub key: String,
    /// `key`'s owning class -- read from the grant token's own gate or its
    /// row's `.MOD` target, NEVER from `key`'s own text. See module doc
    /// comment.
    pub class: String,
    /// The class level this grant activates at.
    pub level: u8,
    /// Whether `level` came from an explicit gate (`true`) or is the
    /// ungated level-1 default (`false`).
    pub level_explicit: bool,
    /// 1-based line this grant token was read from.
    pub source_line: u32,
    /// Which rule resolved this fact.
    pub gate: GateKind,
}

/// A grant-shaped `ABILITY:` token (or one of its sibling keys) this
/// module explicitly refused to resolve, with the reason -- reported as a
/// headline number, never silently dropped or defaulted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedGrant {
    pub key: String,
    pub source_line: u32,
    pub reason: String,
}

/// One `ABILITY:` token's pipe-segments, already split into its sibling
/// keys (everything ` ~ `-shaped before the gate boundary) and its gate
/// segments (everything from the first `PRE*`/`!PRE*`-prefixed segment
/// onward).
struct AbilityToken<'a> {
    keys: Vec<&'a str>,
    gate_segments: Vec<&'a str>,
}

/// Parses one `ABILITY:` field into its keys + gate segments, or `None`
/// when the field is not an `ABILITY:` token, is not `AUTOMATIC`, or names
/// no ` ~ `-shaped key at all (not class-feature-shaped -- a bare feat
/// grant, an `Internal` bookkeeping token with no tilde key, or similar).
fn split_ability_token(field: &str) -> Option<AbilityToken<'_>> {
    if !field.starts_with("ABILITY:") {
        return None;
    }
    // Segments of the FULL token, matching the documented
    // `ABILITY:<pool>|<nature>|<key>|<pre...>` shape: segs[0] == the
    // "ABILITY:<pool>" segment, segs[1] == nature, segs[2..] == key(s) and
    // gate.
    let segs: Vec<&str> = field.split('|').collect();
    if segs.len() < 3 {
        return None;
    }
    if segs[1] != "AUTOMATIC" {
        return None;
    }
    let mut keys = Vec::new();
    let mut gate_start = segs.len();
    for (i, s) in segs.iter().enumerate().skip(2) {
        if s.starts_with("PRE") || s.starts_with("!PRE") {
            gate_start = i;
            break;
        }
        if let Some((left, right)) = s.split_once(" ~ ") {
            // Both halves must be non-empty -- "Barbarian ~ " (no feature
            // half) is not a real key, just a malformed/placeholder token.
            if !left.is_empty() && !right.is_empty() {
                keys.push(*s);
            }
        }
    }
    if keys.is_empty() {
        return None;
    }
    Some(AbilityToken { keys, gate_segments: segs[gate_start..].to_vec() })
}

/// The class named by a row's own `CATEGORY=Class|<Class>[ ~
/// <Variant>].MOD` field-0 target, or `None` when field 0 is not that
/// shape (an ordinary ability/archetype DEFINITION row, whose field 0 is
/// the feature's own display name, not a modify-target).
fn mod_row_class(field0: &str) -> Option<String> {
    let rest = field0.strip_prefix("CATEGORY=Class|")?;
    let rest = rest.strip_suffix(".MOD")?;
    let class = rest.split(" ~ ").next()?;
    if class.is_empty() { None } else { Some(class.to_string()) }
}

/// One gate's parsed PRE* content, kept only as far as this module's
/// resolution rules need.
#[derive(Default)]
struct Gate {
    /// `Some((count, [(class, level)]))` for a non-negated `PRECLASS:`.
    preclass: Option<(u32, Vec<(String, Option<u8>)>)>,
    /// `Some((var, level))` for a non-negated `PREVARGTEQ:`.
    prevargteq: Option<(String, u8)>,
    /// Every other gate kind seen, negation-prefixed, for diagnostics.
    other_kinds: Vec<String>,
}

fn parse_gate(gate_segments: &[&str]) -> Gate {
    let mut gate = Gate::default();
    for seg in gate_segments {
        let negated = seg.starts_with('!');
        let body = seg.trim_start_matches('!');
        let (name, rest) = body.split_once(':').unwrap_or((body, ""));
        match name {
            "PRECLASS" if !negated => {
                let mut parts = rest.split(',');
                let count: u32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                let clauses: Vec<(String, Option<u8>)> = parts
                    .map(|p| match p.split_once('=') {
                        Some((cls, lvl)) => (cls.to_string(), lvl.parse::<u8>().ok()),
                        None => (p.to_string(), None),
                    })
                    .collect();
                gate.preclass = Some((count, clauses));
            }
            "PREVARGTEQ" if !negated => {
                if let Some((var, n)) = rest.split_once(',') {
                    if let Ok(n) = n.trim().parse::<u8>() {
                        gate.prevargteq = Some((var.to_string(), n));
                    }
                }
            }
            _ => gate.other_kinds.push(format!("{}{name}", if negated { "!" } else { "" })),
        }
    }
    gate
}

/// Resolves one `ABILITY:` token's keys against its row's own `field0` and
/// its own gate, per the priority rules the module doc comment states.
/// Every key shares the token's one resolution (or one refusal) -- never a
/// mix, since they share the same gate by construction.
fn resolve_token(field0: &str, token: &AbilityToken<'_>, source_line: u32) -> Vec<Result<GrantFact, UnresolvedGrant>> {
    let gate = parse_gate(&token.gate_segments);
    let mod_class = mod_row_class(field0);

    let resolution: Result<(String, u8, bool, GateKind), String> = match &gate.preclass {
        Some((count, clauses)) if *count == 1 && clauses.len() == 1 && clauses[0].1.is_some() => {
            let (cls, lvl) = (&clauses[0].0, clauses[0].1.unwrap());
            match &mod_class {
                Some(mc) if mc != cls => Err(format!("preclass_vs_modrow_class_conflict({mc}!={cls})")),
                _ => Ok((cls.clone(), lvl, true, GateKind::Preclass)),
            }
        }
        Some(_) => Err("preclass_multiclass_or_unparsed_gate".to_string()),
        None => match &mod_class {
            Some(mc) => {
                // `PREVARGTEQ:` gates the LEVEL. `PREVAREQ:<var>,<n>` gates
                // nothing about level at all -- it is PCGen's own
                // duplicate-grant guard ("only fire if this bookkeeping
                // flag still equals its starting value"), confirmed live:
                // `pu_abilities_class.lst:131`'s "Unchained Barbarian ~
                // Weapon and Armor Proficiency" carries two `PREVAREQ:`
                // guards and NO `PREVARGTEQ:` at all, and the hand-curated
                // PU table (this module's own reproduction proof) expects
                // it ungated at level 1 -- refusing it as
                // "unrecognized_gate" would have wrongly shrunk this
                // module's own correctness proof. A plain (non-negated)
                // `PREVAREQ:` therefore does not block the ungated-level-1
                // resolution on a trusted `.MOD` row; anything else
                // (negated `!PREVAREQ:`, `PREABILITY:`, `PREDOMAIN:`, ...)
                // still does, because none of those are proven
                // level-neutral the way `PREVAREQ:` is here.
                let only_prevareq = gate.other_kinds.iter().all(|k| k == "PREVAREQ");
                if let Some((_, lvl)) = &gate.prevargteq {
                    Ok((mc.clone(), *lvl, true, GateKind::ModRowGated))
                } else if only_prevareq {
                    Ok((mc.clone(), 1, false, GateKind::ModRowUngated))
                } else {
                    Err(format!(
                        "modrow_unrecognized_gate({})",
                        if gate.other_kinds.is_empty() { "unparsed".to_string() } else { gate.other_kinds.join("+") }
                    ))
                }
            }
            None => {
                let desc = if gate.prevargteq.is_some() {
                    "prevargteq_without_modrow_or_preclass".to_string()
                } else if gate.other_kinds.is_empty() {
                    "ungated_without_modrow_or_preclass".to_string()
                } else {
                    format!("unrecognized_gate_without_modrow_or_preclass({})", gate.other_kinds.join("+"))
                };
                Err(desc)
            }
        },
    };

    match resolution {
        Ok((class, level, level_explicit, gate_kind)) => token
            .keys
            .iter()
            .map(|k| Ok(GrantFact { key: (*k).to_string(), class: class.clone(), level, level_explicit, source_line, gate: gate_kind }))
            .collect(),
        Err(reason) => token
            .keys
            .iter()
            .map(|k| Err(UnresolvedGrant { key: (*k).to_string(), source_line, reason: reason.clone() }))
            .collect(),
    }
}

/// Every class-feature-shaped `ABILITY:` grant token in `text`, resolved
/// or refused. Multiple grants per line, multiple keys per token, and
/// multiple lines per key are all preserved -- [`resolve_grants`] decides
/// which occurrence wins per key.
pub fn parse_grant_facts(text: &str) -> (Vec<GrantFact>, Vec<UnresolvedGrant>) {
    let mut facts = Vec::new();
    let mut unresolved = Vec::new();
    for (idx, raw_line) in text.lines().enumerate() {
        let source_line = (idx + 1) as u32;
        let line = raw_line.trim_end_matches('\r');
        let fields: Vec<&str> = line.split('\t').collect();
        let Some(field0) = fields.first().copied() else { continue };
        for field in fields.iter().skip(1) {
            if field.is_empty() {
                continue;
            }
            let Some(token) = split_ability_token(field) else { continue };
            for r in resolve_token(field0, &token, source_line) {
                match r {
                    Ok(f) => facts.push(f),
                    Err(u) => unresolved.push(u),
                }
            }
        }
    }
    (facts, unresolved)
}

/// Resolves possibly-duplicate/possibly-contradictory occurrences to one
/// verdict per key: a key that resolved successfully at least once keeps
/// its FIRST successful (file-order) resolution -- the file's own primary
/// progression block always precedes any alternate/Ex-Class block
/// physically in every book this cycle checked, the same tie-break the
/// hand-authored Pathfinder Unchained tables already made by construction
/// (verified by [`pu_class_feature_reproduction_barbarian`] and its
/// siblings). A key that NEVER resolved keeps its first refusal reason.
pub fn resolve_grants(
    facts: Vec<GrantFact>,
    unresolved: Vec<UnresolvedGrant>,
) -> (BTreeMap<String, GrantFact>, BTreeMap<String, UnresolvedGrant>) {
    let mut resolved: BTreeMap<String, GrantFact> = BTreeMap::new();
    for fact in facts {
        resolved.entry(fact.key.clone()).or_insert(fact);
    }
    let mut refused: BTreeMap<String, UnresolvedGrant> = BTreeMap::new();
    for u in unresolved {
        if resolved.contains_key(&u.key) {
            continue;
        }
        refused.entry(u.key.clone()).or_insert(u);
    }
    (resolved, refused)
}

// ---------------------------------------------------------------------
// Corpus write path
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct GrantCitation {
    pub kind: &'static str,
    pub path: String,
    pub sha256: String,
    pub line: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct GrantRecord {
    pub key: String,
    pub class: String,
    pub level: u8,
    pub level_explicit: bool,
    pub gate: GateKind,
    /// Whether a real `class_feature` corpus record with this exact `key`
    /// exists anywhere under `data/corpus/*/class_feature/**` (checked
    /// against the ACTUAL corpus, not a same-file heuristic -- module doc
    /// comment). `false` means the grant fact is real and resolved but has
    /// no corpus record to attach to yet (commonly an un-ingested
    /// archetype replacement feature) -- a different, honestly distinct
    /// condition from an unresolved gate shape.
    pub corpus_record_exists: bool,
    pub source: GrantCitation,
}

#[derive(Debug, Default)]
pub struct GenerationReport {
    pub written: usize,
    pub corpus_record_exists: usize,
    pub corpus_record_missing: usize,
    /// Grant facts whose `key` tripped the Product-Identity blacklist term
    /// scan -- skipped rather than written, same conservative default
    /// `cache_gen::class_feature` uses.
    pub pi_skipped: usize,
    /// Refused grant tokens, by reason -- the headline "what this module
    /// could NOT prove" number, per this wave's instruction to report it
    /// as prominently as coverage.
    pub unresolved: BTreeMap<String, usize>,
    pub books_written: Vec<String>,
}

fn slugify(name: &str) -> String {
    let mut slug: String =
        name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    while slug.contains("__") {
        slug = slug.replace("__", "_");
    }
    let slug = slug.trim_matches('_').to_string();
    if slug.is_empty() { "unnamed".to_string() } else { slug }
}

/// Recursively collects every real `class_feature` corpus record's `data.key`
/// under `corpus_root/*/class_feature/**/*.json` -- the cross-check this
/// module's own doc comment describes. Malformed/unreadable files are
/// skipped, never treated as a match (conservative: a key this cannot
/// confirm counts as `corpus_record_exists: false`, never `true`).
pub fn real_class_feature_keys(corpus_root: &Path) -> std::io::Result<BTreeSet<String>> {
    let mut keys = BTreeSet::new();
    let Ok(book_dirs) = std::fs::read_dir(corpus_root) else { return Ok(keys) };
    for book_entry in book_dirs.flatten() {
        let cf_dir = book_entry.path().join("class_feature");
        if !cf_dir.is_dir() {
            continue;
        }
        collect_keys_recursive(&cf_dir, &mut keys)?;
    }
    Ok(keys)
}

fn collect_keys_recursive(dir: &Path, keys: &mut BTreeSet<String>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)?.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_keys_recursive(&path, keys)?;
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
        if let Some(key) = value.get("data").and_then(|d| d.get("key")).and_then(|k| k.as_str()) {
            keys.insert(key.to_string());
        }
    }
    Ok(())
}

/// Generates `<grants_root>/<book>/<class-slug>.json` for one book, from
/// the SAME primary file `BOOK_PRIMARY_FILES` already establishes for
/// `class_feature` itself. `oracle_root` is the PCGen raw-`.lst` checkout
/// (`PCGEN_CORPUS_ROOT`) the grant tokens are READ from; `real_keys` is
/// this repo's own ALREADY-INGESTED `data/corpus/*/class_feature/**` key
/// set the resolved facts are cross-checked AGAINST -- two different
/// trees, never conflated (see [`generate_all`]).
pub fn generate_for_book(
    oracle_root: &Path,
    grants_root: &Path,
    real_keys: &BTreeSet<String>,
    book: &str,
    rel_dir: &str,
    file_name: &str,
) -> std::io::Result<GenerationReport> {
    let mut report = GenerationReport::default();
    let book_dir = oracle_root.join(rel_dir);
    let file_path = book_dir.join(file_name);
    let text = std::fs::read_to_string(&file_path)?;
    let sha256 = sha256_file(&file_path)?;
    let (facts, unresolved) = parse_grant_facts(&text);
    let (resolved, refused) = resolve_grants(facts, unresolved);

    for u in refused.values() {
        *report.unresolved.entry(u.reason.clone()).or_insert(0) += 1;
    }

    let mut by_class: BTreeMap<String, Vec<&GrantFact>> = BTreeMap::new();
    for fact in resolved.values() {
        by_class.entry(fact.class.clone()).or_default().push(fact);
    }

    let grants_dir = grants_root.join(book);
    let mut any_written = false;
    for (class, mut facts) in by_class {
        facts.sort_by(|a, b| a.key.cmp(&b.key));
        let mut records = Vec::with_capacity(facts.len());
        for fact in facts {
            let (license, _, _, _) = pi_screening::classify_field("name", &fact.key);
            if license == License::PiRedacted {
                report.pi_skipped += 1;
                continue;
            }
            let exists = real_keys.contains(&fact.key);
            if exists {
                report.corpus_record_exists += 1;
            } else {
                report.corpus_record_missing += 1;
            }
            records.push(GrantRecord {
                key: fact.key.clone(),
                class: fact.class.clone(),
                level: fact.level,
                level_explicit: fact.level_explicit,
                gate: fact.gate,
                corpus_record_exists: exists,
                source: GrantCitation {
                    kind: "class_feature_grant",
                    path: format!("{rel_dir}/{file_name}"),
                    sha256: sha256.clone(),
                    line: fact.source_line,
                },
            });
        }
        if records.is_empty() {
            continue;
        }
        std::fs::create_dir_all(&grants_dir)?;
        let path = grants_dir.join(format!("{}.json", slugify(&class)));
        let json = serde_json::to_string_pretty(&records)
            .expect("GrantRecord is a plain-data shape; serialization cannot fail");
        std::fs::write(path, json)?;
        report.written += records.len();
        any_written = true;
    }
    if any_written {
        report.books_written.push(book.to_string());
    }
    Ok(report)
}

/// Runs [`generate_for_book`] over every book [`BOOK_PRIMARY_FILES`]
/// names. `oracle_root` is the PCGen raw-`.lst` checkout the grant tokens
/// are read from (`PCGEN_CORPUS_ROOT`); `repo_corpus_root` is THIS repo's
/// own already-shipped `data/corpus/` tree the resolved facts are
/// cross-checked against ([`real_class_feature_keys`]) -- passing the same
/// path for both would silently cross-check against raw `.lst` text
/// instead of real corpus JSON and always report every fact missing.
pub fn generate_all(oracle_root: &Path, repo_corpus_root: &Path, grants_root: &Path) -> std::io::Result<GenerationReport> {
    let real_keys = real_class_feature_keys(repo_corpus_root)?;
    let mut total = GenerationReport::default();
    for (book, rel_dir, file_name) in BOOK_PRIMARY_FILES {
        let report = generate_for_book(oracle_root, grants_root, &real_keys, book, rel_dir, file_name)?;
        total.written += report.written;
        total.corpus_record_exists += report.corpus_record_exists;
        total.corpus_record_missing += report.corpus_record_missing;
        total.pi_skipped += report.pi_skipped;
        for (reason, count) in report.unresolved {
            *total.unresolved.entry(reason).or_insert(0) += count;
        }
        total.books_written.extend(report.books_written);
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // Shape 1: CATEGORY=Class|<Class>.MOD row + explicit PREVARGTEQ gate.
    // -----------------------------------------------------------------

    #[test]
    fn mod_row_with_explicit_gate_resolves_class_and_level() {
        let field0 = "CATEGORY=Class|Wizard.MOD";
        let field = "ABILITY:Wizard Class Feature|AUTOMATIC|Wizard ~ Spells|PREVAREQ:Wizard_CF_Spells,0|PREVARGTEQ:Wizard_CFP_Level,1";
        let token = split_ability_token(field).unwrap();
        let results = resolve_token(field0, &token, 316);
        assert_eq!(results.len(), 1);
        let fact = results[0].as_ref().unwrap();
        assert_eq!(fact.key, "Wizard ~ Spells");
        assert_eq!(fact.class, "Wizard");
        assert_eq!(fact.level, 1);
        assert!(fact.level_explicit);
        assert_eq!(fact.gate, GateKind::ModRowGated);
    }

    #[test]
    fn mod_row_with_variant_suffix_still_resolves_the_base_class() {
        // core_rulebook:145 -- "Barbarian ~ Standard Class Full.MOD".
        let field0 = "CATEGORY=Class|Barbarian ~ Standard Class Full.MOD";
        let field = "ABILITY:Barbarian Class Feature|AUTOMATIC|Barbarian ~ Rage|PREVAREQ:Barbarian_CF_Rage,0|PREVARGTEQ:Barbarian_CFP_Level,1";
        let token = split_ability_token(field).unwrap();
        let fact = resolve_token(field0, &token, 149)[0].as_ref().unwrap().clone();
        assert_eq!(fact.class, "Barbarian");
        assert_eq!(fact.level, 1);
    }

    // -----------------------------------------------------------------
    // Shape 2: CATEGORY=Class.MOD row, fully ungated -> implicit level 1.
    // -----------------------------------------------------------------

    #[test]
    fn mod_row_fully_ungated_defaults_to_level_one_and_is_marked_implicit() {
        // core_rulebook:148 -- "Barbarian ~ Uncanny Dodge Tracker", no PRE*
        // at all, verified independently of Pathfinder Unchained.
        let field0 = "CATEGORY=Class|Barbarian ~ Standard Class Full.MOD";
        let field = "ABILITY:Barbarian Class Feature|AUTOMATIC|Barbarian ~ Uncanny Dodge Tracker";
        let token = split_ability_token(field).unwrap();
        let fact = resolve_token(field0, &token, 148)[0].as_ref().unwrap().clone();
        assert_eq!(fact.class, "Barbarian");
        assert_eq!(fact.level, 1);
        assert!(!fact.level_explicit);
        assert_eq!(fact.gate, GateKind::ModRowUngated);
    }

    // -----------------------------------------------------------------
    // Shape 3: PRECLASS gate on the token itself -- the wave-21 defect fix.
    // -----------------------------------------------------------------

    #[test]
    fn preclass_gate_resolves_class_and_level_directly_not_from_the_key_text() {
        // adventurers_guide:882, "Sigilus ~ Inscribe Sihedron" -- the
        // exact record OPEN-ISSUES.md row 334 names. Field 0 is the
        // ARCHETYPE's own definition row ("Sigilus"), not a CATEGORY=Class
        // MOD row -- the real class comes ONLY from PRECLASS.
        let field0 = "Sigilus";
        let field = "ABILITY:Special Ability|AUTOMATIC|Sigilus ~ Inscribe Sihedron|PRECLASS:1,Magus=7";
        let token = split_ability_token(field).unwrap();
        let fact = resolve_token(field0, &token, 882)[0].as_ref().unwrap().clone();
        assert_eq!(fact.key, "Sigilus ~ Inscribe Sihedron");
        assert_eq!(fact.class, "Magus", "must read PRECLASS's class, never key.split(' ~ ')[0] (\"Sigilus\")");
        assert_eq!(fact.level, 7, "must read PRECLASS's level, never default to 1");
        assert!(fact.level_explicit);
        assert_eq!(fact.gate, GateKind::Preclass);
    }

    #[test]
    fn preclass_multiclass_gate_is_refused_not_resolved() {
        // apg_abilities_class.lst:3725 shape -- "must be BOTH classes",
        // not "granted by either"; no single owner to name.
        let field0 = "Standard Choices ~ Cavalier Mount";
        let field = "ABILITY:Special Ability|AUTOMATIC|Mount ~ Ranger Variant|PRECLASS:2,Cavalier=1,Ranger=1";
        let token = split_ability_token(field).unwrap();
        let result = &resolve_token(field0, &token, 3725)[0];
        assert!(result.is_err(), "a 2-class PRECLASS gate must not resolve to either class");
    }

    #[test]
    fn preclass_conflicting_with_mod_row_class_is_refused() {
        let field0 = "CATEGORY=Class|Wizard.MOD";
        let field = "ABILITY:Special Ability|AUTOMATIC|Something ~ Weird|PRECLASS:1,Sorcerer=3";
        let token = split_ability_token(field).unwrap();
        let result = &resolve_token(field0, &token, 1)[0];
        assert!(result.is_err(), "a PRECLASS class contradicting the row's own .MOD target must not be resolved by picking one");
    }

    // -----------------------------------------------------------------
    // Shape 4: multiple sibling keys sharing one gate.
    // -----------------------------------------------------------------

    #[test]
    fn multiple_sibling_keys_share_the_same_gate_none_dropped() {
        // core_rulebook:384 -- the prior module read only segment 2 and
        // silently dropped "Deep Pockets"/"Master Scribe".
        let field0 = "CATEGORY=Class|Pathfinder Chronicler.MOD";
        let field = "ABILITY:Pathfinder Chronicler Class Feature|AUTOMATIC|Pathfinder Chronicler ~ Bardic Knowledge|Pathfinder Chronicler ~ Deep Pockets|Pathfinder Chronicler ~ Master Scribe|PREVARGTEQ:PathfinderChronicler_CFP_Level,1";
        let token = split_ability_token(field).unwrap();
        let results = resolve_token(field0, &token, 384);
        assert_eq!(results.len(), 3, "all three sibling keys must resolve, not just segment 2");
        let keys: Vec<&str> = results.iter().map(|r| r.as_ref().unwrap().key.as_str()).collect();
        assert!(keys.contains(&"Pathfinder Chronicler ~ Bardic Knowledge"));
        assert!(keys.contains(&"Pathfinder Chronicler ~ Deep Pockets"));
        assert!(keys.contains(&"Pathfinder Chronicler ~ Master Scribe"));
        for r in &results {
            let f = r.as_ref().unwrap();
            assert_eq!(f.class, "Pathfinder Chronicler");
            assert_eq!(f.level, 1);
        }
    }

    // -----------------------------------------------------------------
    // Shape 5: the same feature name genuinely granted by TWO different
    // classes as two SEPARATE corpus records -- (class, key, level), never
    // collapsed to (key, level).
    // -----------------------------------------------------------------

    #[test]
    fn bard_and_pathfinder_chronicler_each_grant_their_own_bardic_knowledge_record() {
        let text = "CATEGORY=Class|Bard.MOD\tABILITY:Bard Class Feature|AUTOMATIC|Bard ~ Bardic Knowledge|PREVAREQ:Bard_CF_BardicKnowledge,0|PREVARGTEQ:Bard_CFP_Level,1\n\
                    CATEGORY=Class|Pathfinder Chronicler.MOD\tABILITY:Pathfinder Chronicler Class Feature|AUTOMATIC|Pathfinder Chronicler ~ Bardic Knowledge|Pathfinder Chronicler ~ Deep Pockets|Pathfinder Chronicler ~ Master Scribe|PREVARGTEQ:PathfinderChronicler_CFP_Level,1\n";
        let (facts, unresolved) = parse_grant_facts(text);
        assert!(unresolved.is_empty(), "unresolved: {unresolved:?}");
        let (resolved, _) = resolve_grants(facts, Vec::new());
        let bard = &resolved["Bard ~ Bardic Knowledge"];
        assert_eq!(bard.class, "Bard");
        assert_eq!(bard.level, 1);
        let chronicler = &resolved["Pathfinder Chronicler ~ Bardic Knowledge"];
        assert_eq!(chronicler.class, "Pathfinder Chronicler");
        assert_eq!(chronicler.level, 1);
        assert_ne!(
            bard.key, chronicler.key,
            "same feature NAME, two separate corpus KEYs, two separate GrantFacts -- never (key, level) alone"
        );
    }

    // -----------------------------------------------------------------
    // Refused shapes -- every one must REFUSE, never default to level 1.
    // -----------------------------------------------------------------

    #[test]
    fn preability_gate_with_no_level_is_refused() {
        let field0 = "CATEGORY=Special Ability|Dragon Disciple ~ Breath Weapon.MOD";
        let field = "ABILITY:Special Ability|AUTOMATIC|Draconic Bloodrager Bloodline ~ Breath Weapon|PREABILITY:1,CATEGORY=Blood of Dragons Bloodline,TYPE.BloodragerBloodlineChoice";
        let token = split_ability_token(field).unwrap();
        assert!(resolve_token(field0, &token, 379)[0].is_err());
    }

    #[test]
    fn predomain_gate_is_refused_not_treated_as_a_class_grant() {
        let field0 = "Air";
        let field = "ABILITY:Special Ability|AUTOMATIC|Domain Power ~ Lightning Arc|PREDOMAIN:1,Air|PREVARGTEQ:DomainAirAbilityTriggerLVL,-1";
        let token = split_ability_token(field).unwrap();
        assert!(resolve_token(field0, &token, 1027)[0].is_err());
    }

    #[test]
    fn ungated_grant_on_a_non_mod_row_is_refused_not_defaulted_to_level_one() {
        // advanced_class_guide:106 -- ungated, but field0 is NOT a
        // CATEGORY=Class.MOD row, so there is no proven class owner.
        let field0 = "Familiar";
        let field = "ABILITY:Special Ability|AUTOMATIC|Arcane Bond ~ Familiar";
        let token = split_ability_token(field).unwrap();
        assert!(resolve_token(field0, &token, 106)[0].is_err());
    }

    #[test]
    fn negated_preclass_is_refused_not_treated_as_the_positive_gate() {
        let field0 = "Resistance";
        let field = "ABILITY:Special Ability|AUTOMATIC|Abjuration School ~ Resistance|!PRECLASS:1,Wizard=1";
        let token = split_ability_token(field).unwrap();
        assert!(resolve_token(field0, &token, 250)[0].is_err());
    }

    #[test]
    fn a_feat_grant_with_no_tilde_key_is_not_class_feature_shaped() {
        let field = "ABILITY:FEAT|AUTOMATIC|Endurance|PREVARGTEQ:Ranger_CFP_Level,3";
        assert!(split_ability_token(field).is_none());
    }

    #[test]
    fn a_non_automatic_nature_is_skipped() {
        let field = "ABILITY:Rogue Class Feature|VIRTUAL|Rogue ~ Something|PREVARGTEQ:Rogue_CFP_Level,4";
        assert!(split_ability_token(field).is_none());
    }

    // -----------------------------------------------------------------
    // Line/file-level parsing and the first-occurrence tie-break.
    // -----------------------------------------------------------------

    #[test]
    fn resolve_grants_keeps_the_first_successful_file_order_occurrence() {
        let facts = vec![
            GrantFact {
                key: "Barbarian ~ Weapon and Armor Proficiency".into(),
                class: "Barbarian".into(),
                level: 1,
                level_explicit: false,
                source_line: 131,
                gate: GateKind::ModRowUngated,
            },
            GrantFact {
                key: "Barbarian ~ Weapon and Armor Proficiency".into(),
                class: "Barbarian".into(),
                level: 1,
                level_explicit: true,
                source_line: 145,
                gate: GateKind::ModRowGated,
            },
        ];
        let (resolved, _) = resolve_grants(facts, Vec::new());
        let fact = &resolved["Barbarian ~ Weapon and Armor Proficiency"];
        assert_eq!(fact.source_line, 131);
    }

    #[test]
    fn parse_grant_facts_reads_field0_from_the_row_not_from_tab_tokens_skip_one() {
        // Regression guard for the root cause: the shared corpus_literal_sweep
        // helper's `tab_tokens` unconditionally skips field 0. This module
        // must never route through it for `ABILITY:` discovery.
        let text = "CATEGORY=Class|Wizard.MOD\tABILITY:Wizard Class Feature|AUTOMATIC|Wizard ~ Spells|PREVARGTEQ:Wizard_CFP_Level,1\n";
        let (facts, unresolved) = parse_grant_facts(text);
        assert!(unresolved.is_empty());
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].class, "Wizard");
    }

    #[test]
    fn a_key_with_no_feature_half_is_rejected() {
        let field = "ABILITY:Internal|AUTOMATIC|Barbarian ~ |PREVARGTEQ:Barbarian_CFP_Level,1";
        assert!(split_ability_token(field).is_none());
    }

    // -----------------------------------------------------------------
    // Mutation-proof: two mutations, both ACTUALLY RUN (not merely
    // described) against this exact test file with `--include-ignored`,
    // both reverted before committing.
    // -----------------------------------------------------------------
    //
    // Mutation 1 -- reproduced wave 21's own bug verbatim: in the
    // `Preclass` resolution arm, replaced `Ok((cls.clone(), lvl, true,
    // GateKind::Preclass))` with `Ok((cls.clone(), 1, false,
    // GateKind::Preclass))` (fabricate an ungated level-1 grant instead of
    // reading `PRECLASS`'s own level). RESULT: RED --
    // `preclass_gate_resolves_class_and_level_directly_not_from_the_key_text`
    // and the live-oracle `sigilus_inscribe_sihedron_resolves_against_the_live_oracle_row`
    // both failed (`left: 1, right: 7`), 20/22 passed. Reverted; re-ran
    // clean (22/22).
    //
    // Mutation 2 -- discarded the row's own class entirely: in the
    // `ModRowGated` arm, replaced `Ok((mc.clone(), *lvl, true,
    // GateKind::ModRowGated))` with a hardcoded `Ok(("Barbarian".to_string(),
    // *lvl, true, GateKind::ModRowGated))`. RESULT: RED -- 7 of 22 tests
    // failed, INCLUDING `pu_class_feature_reproduction_monk`/`_rogue`/
    // `_summoner` (`Monk/Unchained Monk ~ Weapon and Armor Proficiency:
    // class mismatch, expected Monk, got Barbarian`) -- the exact
    // reproduction gate wave 21 also ran, except THIS cycle's version of
    // that gate asserts `class` at all (wave 21's own `pu_expected` tuple
    // never carried `class`, which is precisely how its class-fabrication
    // bug shipped through a gate that "passed"). Reverted; re-ran clean
    // (22/22).
    //
    // Both mutations targeted the exact two defects `OPEN-ISSUES.md` row
    // 334 named (wrong class, wrong/fabricated level) and both were
    // caught by tests that are NOT `#[ignore]`d except where they read the
    // live oracle -- i.e. `cargo test --locked --lib class_feature_grants`
    // alone (no `PCGEN_CORPUS_ROOT` needed) already catches mutation 2 via
    // the synthetic-text tests, and mutation 1 via
    // `preclass_gate_resolves_class_and_level_directly_not_from_the_key_text`.

    // -----------------------------------------------------------------
    // Correctness proof: reproduce all four PU tables, INCLUDING class
    // attribution this time (wave 21's own `pu_expected` never checked
    // `class` -- exactly the blind spot that let it ship the bug).
    // -----------------------------------------------------------------

    fn pcgen_corpus_root() -> Option<std::path::PathBuf> {
        use std::path::PathBuf;
        std::env::var_os("PCGEN_CORPUS_ROOT")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join("workspace/repos/pcgen/data")))
    }

    fn pu_expected(class: &str) -> Vec<(String, Option<u8>, bool)> {
        use crate::rules_core::rules_tables::pathfinder_unchained::{
            barbarian_features, monk_features, rogue_features, summoner_features,
        };
        match class {
            "Barbarian" => {
                barbarian_features::features().iter().map(|f| (f.key.to_string(), f.min_level, f.is_granted)).collect()
            }
            "Monk" => monk_features::features().iter().map(|f| (f.key.to_string(), Some(f.min_level), true)).collect(),
            "Rogue" => rogue_features::UnchainedRogueFeature::ALL
                .iter()
                .map(|f| (f.key().to_string(), f.min_level(), f.min_level().is_some()))
                .collect(),
            "Summoner" => summoner_features::UnchainedSummonerFeature::ALL
                .iter()
                .map(|f| (f.key().to_string(), Some(f.min_level()), true))
                .collect(),
            other => panic!("no PU expectation table for {other}"),
        }
    }

    fn assert_reproduces_pu_table(class: &str) {
        let Some(corpus_root) = pcgen_corpus_root() else {
            eprintln!("skipping {class} reproduction: no PCGEN_CORPUS_ROOT/HOME");
            return;
        };
        let lst_path =
            corpus_root.join("pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_abilities_class.lst");
        let Ok(text) = std::fs::read_to_string(&lst_path) else {
            eprintln!("skipping {class} reproduction: {} unreadable", lst_path.display());
            return;
        };
        let (facts, _unresolved) = parse_grant_facts(&text);
        let (resolved, _) = resolve_grants(facts, Vec::new());

        let expected = pu_expected(class);
        for (key, expected_min_level, expected_is_granted) in &expected {
            let found = resolved.get(key);
            let actual_is_granted = found.is_some();
            assert_eq!(
                actual_is_granted, *expected_is_granted,
                "{class}/{key}: is_granted mismatch (expected {expected_is_granted}, found {actual_is_granted})"
            );
            if let Some(fact) = found {
                assert_eq!(fact.class, class, "{class}/{key}: class mismatch, expected {class}, got {}", fact.class);
                let actual_min_level = Some(fact.level);
                let expected_min_level_resolved = expected_min_level.or(Some(1));
                assert_eq!(
                    fact.level_explicit,
                    expected_min_level.is_some(),
                    "{class}/{key}: level_explicit mismatch (expected {}, found {} at line {})",
                    expected_min_level.is_some(),
                    fact.level_explicit,
                    fact.source_line
                );
                assert_eq!(
                    actual_min_level, expected_min_level_resolved,
                    "{class}/{key}: level mismatch (expected {expected_min_level_resolved:?}, parsed {actual_min_level:?} at line {})",
                    fact.source_line
                );
            }
        }
    }

    #[test]
    #[ignore = "reads the live pinned PCGen oracle checkout; run with PCGEN_CORPUS_ROOT set"]
    fn pu_class_feature_reproduction_barbarian() {
        assert_reproduces_pu_table("Barbarian");
    }

    #[test]
    #[ignore = "reads the live pinned PCGen oracle checkout; run with PCGEN_CORPUS_ROOT set"]
    fn pu_class_feature_reproduction_monk() {
        assert_reproduces_pu_table("Monk");
    }

    #[test]
    #[ignore = "reads the live pinned PCGen oracle checkout; run with PCGEN_CORPUS_ROOT set"]
    fn pu_class_feature_reproduction_rogue() {
        assert_reproduces_pu_table("Rogue");
    }

    #[test]
    #[ignore = "reads the live pinned PCGen oracle checkout; run with PCGEN_CORPUS_ROOT set"]
    fn pu_class_feature_reproduction_summoner() {
        assert_reproduces_pu_table("Summoner");
    }

    #[test]
    #[ignore = "reads the live pinned PCGen oracle checkout; run with PCGEN_CORPUS_ROOT set"]
    fn sigilus_inscribe_sihedron_resolves_against_the_live_oracle_row() {
        // Direct, oracle-gated regression test for OPEN-ISSUES.md row 334's
        // named record -- hand-verified above by literal text, this test
        // additionally confirms the LIVE file still carries the same shape.
        let Some(corpus_root) = pcgen_corpus_root() else {
            eprintln!("skipping: no PCGEN_CORPUS_ROOT/HOME");
            return;
        };
        let lst_path = corpus_root.join("pathfinder/paizo/roleplaying_game/adventurers_guide/ag_abilities_class.lst");
        let Ok(text) = std::fs::read_to_string(&lst_path) else {
            eprintln!("skipping: {} unreadable", lst_path.display());
            return;
        };
        let (facts, _) = parse_grant_facts(&text);
        let (resolved, _) = resolve_grants(facts, Vec::new());
        let fact = resolved.get("Sigilus ~ Inscribe Sihedron").expect("Sigilus ~ Inscribe Sihedron must resolve");
        assert_eq!(fact.class, "Magus");
        assert_eq!(fact.level, 7);
    }
}
