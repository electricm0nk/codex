//! Corpus-literal byte-equality sweep for the shipped JSON corpus records
//! under `data/corpus/**/*.json`.
//!
//! **The bar this closes.** A record whose `wiring_class` is `static` carries
//! nothing but literal magnitudes (`wiring_class::classify`'s
//! `literal_magnitudes_only` fallback). Its bar is therefore already known and
//! needs no consumer-delta probe: the shipped record must byte-match the
//! corpus literal it was transcribed from. Until this module existed there was
//! no check that performed that comparison, so every such unit sat at `held` —
//! "as done as the current instruments can prove" — for want of the one
//! instrument that could prove it. This is that instrument. It does **not**
//! decide any unit's doneness verdict; it supplies the evidence a verdict
//! would need, and it is deliberately capable of reporting that the evidence
//! is absent.
//!
//! **What byte-equality means here, precisely.** A shipped record's
//! `data.raw_tokens` is a list of `{key, value}` pairs transcribed off a
//! PCGen `.lst` row. Reconstituted as `KEY:VALUE` each one must appear, byte
//! for byte, as a tab-separated field of that record's **token closure** —
//! its base corpus row plus every `.MOD` row in the same book targeting its
//! name or key. The closure, not the base row alone, is the correct
//! comparand: `wiring_class` itself is derived from the closure
//! (`wiring_class::token_closure_rows`), and a record whose gate token lives
//! on a `.MOD` row in a sibling file is transcription-correct, not drifted.
//! Comparing against the base row alone reports 25 such records as mismatches
//! that are in fact faithful — measured 2026-08-13, and the reason this
//! module reuses `wiring_class`'s own `build_mod_index` rather than forking a
//! narrower rule.
//!
//! **Synthesized tokens.** `ingest_races::parse_trait` appends one token the
//! trait row did not state, under the key `GLOBALVAR:ABILITY`, when a race's
//! suppression gate is declared in the book's `*globalvar*` file instead of
//! on the trait row. That key deliberately contains a `:` so it cannot be
//! mistaken for an LST token name. It is still held to a corpus literal, just
//! a different one: its value must appear verbatim as an `ABILITY:` field
//! somewhere in the same book's corpus. Every key carrying a `:` that is not
//! on [`SYNTHESIZED_TOKEN_KEYS`] is reported as
//! [`Finding::UnknownSynthesizedKey`] rather than waved through — a new
//! synthesized key must be added here with its own corpus rule, not inherit
//! an exemption.
//!
//! **Provenance.** Independently of the tokens, every record claiming a
//! `source.sha256` is checked against the real digest of the corpus file it
//! names. That is what catches the corpus moving underneath a shipped record
//! whose tokens happen to still be found on some line.
//!
//! Pure comparison only — every function here takes the corpus material it
//! needs as an argument. The `corpus_literal_sweep` binary does the walking
//! and the I/O.

use std::collections::{BTreeMap, BTreeSet};

use crate::rules_core::pi_screening::{blacklist_term_hit_including_concatenated, classify_field};

use crate::rules_core::shape_b_v1::{License, REDACTED_PI_MARKER};

/// Token keys this repo synthesizes rather than transcribes, each with the
/// LST token name its value must be found under in the book's corpus.
///
/// Kept as an explicit two-column table, never as "any key containing a
/// colon": a blanket rule would let a future synthesized key inherit an
/// exemption nobody reviewed, which is the shape of gate defect this repo has
/// already shipped twice (an audit implementing 3 of its 4 patterns, and an
/// open-handle check dead behind a SIGPIPE).
pub const SYNTHESIZED_TOKEN_KEYS: &[(&str, &str)] = &[("GLOBALVAR:ABILITY", "ABILITY")];

/// One `{key, value}` pair as the shipped record carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShippedToken {
    pub key: String,
    pub value: String,
}

impl ShippedToken {
    /// The token as it would appear on a `.lst` row: `KEY:VALUE`.
    pub fn joined(&self) -> String {
        format!("{}:{}", self.key, self.value)
    }
}

/// One shipped JSON corpus record, reduced to what the sweep compares.
///
/// `PartialEq` only, not `Eq`: `cost_gp`/`weight_lbs` are `Option<f64>`,
/// which has no total `Eq` impl.
#[derive(Debug, Clone, PartialEq)]
pub struct ShippedRecord {
    /// Repo-relative path of the JSON file, for the report.
    pub record_path: String,
    /// `source.path`: the corpus-relative `.lst` file this was read from.
    pub source_path: String,
    /// `source.line`, 1-indexed.
    pub source_line: usize,
    /// `source.sha256`, when the record claims one.
    pub source_sha256: Option<String>,
    /// Every name this record answers to (`data.key`, `data.name`,
    /// `source.record_key`) — the identities a `.MOD` row can target.
    pub identities: BTreeSet<String>,
    /// `data.raw_tokens`, in order.
    pub tokens: Vec<ShippedToken>,
    /// `true` when this record's `license` is `"PI-REDACTED"` and
    /// `pi_field` is `"description"` -- the declared-PI reader
    /// (`SD-30 decisions.md §53.5`) redacted `data.description` to
    /// [`REDACTED_PI_MARKER`]. When true, a `raw_tokens` entry whose value
    /// is ALSO exactly the marker is expected to differ from the real
    /// corpus row (that IS the redaction) and is exempted from the
    /// byte-match check in [`compare_tokens`] -- see that function's own
    /// doc comment for why this is a narrow, declared exemption and not a
    /// loosened rule.
    pub pi_redacted_description: bool,
    /// `data.codex_generated_name` (`SD-32 decisions.md §24`): `true` when
    /// this record's own name IS Product Identity and it was therefore
    /// ingested under a Codex-generated neutral name
    /// (`rules_core::codex_neutral_name`), per `§24b`-3's binding condition
    /// that the record carry a visible marker rather than look native.
    /// `ingest_ability.py`'s docstring records the companion fact this field
    /// exists to gate: on such a record, ANY `raw_tokens` entry whose VALUE
    /// restates the original PI name/key (not only `DESC`) is redacted to
    /// [`REDACTED_PI_MARKER`] -- `KEY:`, `TYPE:`, `PREMULT:`, `BONUS:` and
    /// others, not a fixed set of keys. Read directly off the record's own
    /// top-level JSON field, never inferred from the filename or directory
    /// (`codex_named_unit_*` is a naming convention some writers use, not
    /// proof) -- see [`compare_tokens`]'s doc comment for the exemption this
    /// gates.
    pub codex_generated_name: bool,
    /// `data.cost_gp`, when the record's typed schema carries one. Read
    /// independently of `raw_tokens` -- see [`compare_tokens`]'s doc
    /// comment for why this field is checked against the corpus closure
    /// at all (`OPEN-ISSUES.md` row 91).
    pub cost_gp: Option<f64>,
    /// `data.weight_lbs`, the `WT:` sibling of [`Self::cost_gp`].
    pub weight_lbs: Option<f64>,
}

/// One way a shipped record failed to byte-match its corpus literal.
///
/// Every variant carries the record path, because a finding a reader cannot
/// locate is a finding nobody acts on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Finding {
    /// The corpus file named by `source.path` does not exist under the
    /// corpus root.
    CorpusFileMissing { record: String, source_path: String },
    /// `source.line` is past the end of the corpus file.
    CorpusLineOutOfRange { record: String, source_path: String, line: usize, file_lines: usize },
    /// `source.sha256` does not match the corpus file's real digest.
    DigestDrift { record: String, source_path: String, claimed: String, actual: String },
    /// A transcribed token is not a byte-identical field of the record's
    /// token closure.
    TokenNotInClosure { record: String, token: String },
    /// A synthesized token's value is not a byte-identical field of the
    /// book's corpus under its declared LST token name.
    SynthesizedTokenNotInCorpus { record: String, token: String },
    /// A token key carries a `:` but is not on [`SYNTHESIZED_TOKEN_KEYS`].
    UnknownSynthesizedKey { record: String, key: String },
    /// A record's own typed field (`data.cost_gp`/`data.weight_lbs`) is not
    /// byte-derivable from any `<lst_key>:<value>` entry in its token
    /// closure -- the systemic gap `raw_tokens`-only comparison could never
    /// catch, since a typed field can be populated from a wholly different
    /// source than `raw_tokens` (`OPEN-ISSUES.md` row 91).
    TypedFieldNotInClosure { record: String, field: &'static str, shipped_value: String, lst_key: &'static str },
}

impl Finding {
    /// The record whose comparison produced this finding.
    pub fn record(&self) -> &str {
        match self {
            Finding::CorpusFileMissing { record, .. }
            | Finding::CorpusLineOutOfRange { record, .. }
            | Finding::DigestDrift { record, .. }
            | Finding::TokenNotInClosure { record, .. }
            | Finding::SynthesizedTokenNotInCorpus { record, .. }
            | Finding::UnknownSynthesizedKey { record, .. }
            | Finding::TypedFieldNotInClosure { record, .. } => record,
        }
    }

    /// A single-line report form, stable enough to grep for in a gate log.
    pub fn describe(&self) -> String {
        match self {
            Finding::CorpusFileMissing { record, source_path } => {
                format!("{record}: corpus file missing: {source_path}")
            }
            Finding::CorpusLineOutOfRange { record, source_path, line, file_lines } => {
                format!("{record}: {source_path} has {file_lines} lines, record claims line {line}")
            }
            Finding::DigestDrift { record, source_path, claimed, actual } => {
                format!("{record}: {source_path} digest drift: record claims {claimed}, file is {actual}")
            }
            Finding::TokenNotInClosure { record, token } => {
                format!("{record}: token not byte-present in corpus token closure: {token}")
            }
            Finding::SynthesizedTokenNotInCorpus { record, token } => {
                format!("{record}: synthesized token not byte-present in book corpus: {token}")
            }
            Finding::UnknownSynthesizedKey { record, key } => {
                format!("{record}: token key is not an LST token name and is not a declared synthesized key: {key}")
            }
            Finding::TypedFieldNotInClosure { record, field, shipped_value, lst_key } => {
                format!(
                    "{record}: typed field {field}={shipped_value} is not byte-derivable from any {lst_key}: entry in the corpus token closure"
                )
            }
        }
    }
}

/// How much a sweep actually examined. Reported alongside the findings so a
/// run that matched nothing cannot be read as a run that found nothing —
/// the zero-cases-ran failure mode `verify.sh`'s `reach` and `audit-selftest`
/// stages each guard against.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SweepTally {
    /// JSON files read under `data/corpus`.
    pub records_seen: usize,
    /// Records in the sweep's population: `source.kind == "lst_token"` with a
    /// `data.raw_tokens` array.
    pub records_examined: usize,
    /// Individual `raw_tokens` entries compared byte-for-byte.
    pub tokens_compared: usize,
    /// Of those, the ones held to the synthesized-token rule.
    pub synthesized_tokens_compared: usize,
    /// Records whose `source.sha256` claim was checked against the real file.
    pub digests_checked: usize,
    /// Typed fields (`cost_gp`/`weight_lbs`) checked against the token
    /// closure -- reported so a run that never exercised the typed-field
    /// check cannot be misread as one that exercised it and found nothing
    /// (`OPEN-ISSUES.md` row 91's own fix).
    pub typed_fields_compared: usize,
    /// `raw_tokens` entries excused from the byte-match check because they
    /// belong to a `codex_generated_name: true` record (`decisions.md §24`)
    /// and carry EXACTLY [`REDACTED_PI_MARKER`] -- the `§24b`-2 redaction
    /// itself, not a transcription defect. Counted (not just skipped)
    /// because `§22`'s divergence-must-be-visible condition and `§24b`-4
    /// both require the exemption be reported, never silent: a sweep that
    /// waves tokens through without saying how many is indistinguishable
    /// from one that stopped checking them.
    pub codex_generated_name_tokens_exempted: usize,
    /// Distinct records that had at least one token excused this way, so a
    /// reader can see how widely the exemption reached, not only how many
    /// tokens it touched.
    pub codex_generated_name_records_exempted: BTreeSet<String>,
}

/// The tab-separated fields of a `.lst` row that can carry tokens: field 0 is
/// the record's own name/`.MOD` head, never a `KEY:VALUE` token, and PCGen
/// rows are padded with long runs of empty fields.
pub fn tab_tokens(line: &str) -> Vec<&str> {
    line.trim_end_matches(['\r']).split('\t').skip(1).filter(|f| !f.is_empty()).collect()
}

/// Every token in one record's closure: its base row, each `.MOD` row
/// targeting any of its identities, and — when the base row is itself a
/// `.COPY=` declaration — the base record's own row it copies from.
///
/// `mod_index` is `wiring_class::build_mod_index`'s output narrowed to one
/// book — this module does not fork the `.MOD` discovery rule.
///
/// `copy_base_row` (`SD31-E6-F6-001`, `OPEN-ISSUES.md` rows 70/103's
/// `.COPY=` inheritance recovery, generalized): a `.COPY=<name>` row states
/// only what it overrides; every OTHER field a record built from it ships
/// (e.g. `equipment_gap_tables`'s `.COPY=` inheritance) comes from the row
/// it copies, never from the `.COPY=` line itself. Without this, a
/// genuinely inherited, corpus-real value (`BOWSTR`'s `cost_gp: 0`, real
/// per `cr_equipmods.lst:34`'s `COST:0`) reads as unprovable — not because
/// it is wrong, but because the closure this function built never looked at
/// the row that actually states it. Caller resolves the base row (by the
/// same `KEY:`-token-or-bare-name identity `.COPY=` itself resolves
/// against) and passes it here; `None` when the base row is a plain
/// declaration or its base could not be resolved (never fabricated).
pub fn token_closure(
    base_row: &str,
    identities: &BTreeSet<String>,
    mod_index: &BTreeMap<String, Vec<String>>,
    copy_base_row: Option<&str>,
) -> BTreeSet<String> {
    let mut closure: BTreeSet<String> =
        tab_tokens(base_row).into_iter().map(str::to_string).collect();
    for identity in identities {
        for row in mod_index.get(identity).into_iter().flatten() {
            closure.extend(tab_tokens(row).into_iter().map(str::to_string));
        }
    }
    if let Some(row) = copy_base_row {
        closure.extend(tab_tokens(row).into_iter().map(str::to_string));
    }
    closure
}

/// Formats an `f64` the way it must appear in an `<LST_KEY>:<value>` token:
/// an integral value with no trailing `.0` (PCGen corpus rows write bare
/// integers, e.g. `COST:800`, not `COST:800.0`), a fractional value with its
/// significant digits and nothing else. Used only to build the finding's own
/// human-readable `shipped_value` string -- the actual comparison in
/// [`compare_tokens`] parses the closure's own token text back to `f64` and
/// compares numerically, so this formatting never drives a pass/fail
/// decision, only what a failure reads as.
fn format_lst_number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

/// Every value of `closure` entries shaped `<lst_key>:<value>`, parsed as
/// `f64` -- multiple entries under the same key are all returned, since a
/// record whose closure includes a `.MOD` override can legitimately carry
/// more than one.
fn closure_numeric_values(closure: &BTreeSet<String>, lst_key: &str) -> Vec<f64> {
    let prefix = format!("{lst_key}:");
    closure.iter().filter_map(|t| t.strip_prefix(prefix.as_str())).filter_map(|v| v.parse::<f64>().ok()).collect()
}

/// Compares one typed numeric field (`cost_gp`/`weight_lbs`) against the
/// closure's own `lst_key:` entries. `None` on the record side is always
/// accepted without comparison (a corpus row that states no cost/weight is
/// not this check's population -- the population is scoped to fields the
/// record itself claims a value for). `Some` on the record side with NO
/// matching numeric token anywhere in the closure is the finding this
/// function exists to raise; a record whose closure carries the key under a
/// DIFFERENT numeric value is caught the same way (no candidate matches).
fn compare_typed_numeric_field(
    record_path: &str,
    field: &'static str,
    lst_key: &'static str,
    shipped: Option<f64>,
    closure: &BTreeSet<String>,
    tally: &mut SweepTally,
) -> Option<Finding> {
    let shipped = shipped?;
    tally.typed_fields_compared += 1;
    let candidates = closure_numeric_values(closure, lst_key);
    if candidates.iter().any(|c| (*c - shipped).abs() < f64::EPSILON) {
        return None;
    }
    Some(Finding::TypedFieldNotInClosure {
        record: record_path.to_string(),
        field,
        shipped_value: format_lst_number(shipped),
        lst_key,
    })
}

/// Compare one record's transcribed tokens, and its typed `cost_gp`/
/// `weight_lbs` fields, against its corpus closure.
///
/// `book_corpus_tokens` is every tab field of every `.lst` row in the record's
/// book — the wider surface a synthesized token is checked against, since by
/// construction it was read from a file other than the record's own row.
///
/// **The one declared exemption.** When `record.pi_redacted_description` is
/// `true` (its `license`/`pi_field` state a real, declared-PI redaction
/// already verified by `declared_pi_shipping_audit`'s CHECK A), a `DESC`
/// token whose value is EXACTLY [`REDACTED_PI_MARKER`] is expected to
/// differ from the real corpus row — that mismatch IS the redaction, not a
/// transcription defect — and is skipped rather than reported. Narrow by
/// construction: only the `DESC` key, only the literal marker byte string,
/// only when the record's own metadata already declares the redaction, so a
/// record that merely happens to carry that string coincidentally (and is
/// NOT `license: "PI-REDACTED"`) is still checked normally, and every OTHER
/// token on a redacted record still must byte-match.
///
/// **A second, narrower exemption for non-`DESC` tokens (`SD31-E6-F10-001`,
/// widened t9-onboarding cycle 2026-08-23 for `class_feature`'s
/// concatenated-blacklist redaction path).** `enrich_spell_raw_tokens.rs`'s
/// `enrich_one` redacts ANY token (not only `DESC`) whose value hits the
/// shared blacklist term scan, one field at a time -- so a `FACTSET:`-shaped
/// token naming a deity on an Inner Sea Gods spell legitimately stores
/// [`REDACTED_PI_MARKER`], the identical shape a redacted `DESC` uses, but
/// the record-level `pi_redacted_description` flag only ever describes
/// `DESC`. Rather than trust the marker string alone (which would let an
/// accidental literal `"[redacted PI]"` value hide a real transcription
/// defect), this token is exempt ONLY when the real corpus closure's own
/// same-key value RE-SCREENS as blacklisted through EITHER of the two scans
/// a write path in this repo actually uses: `pi_screening::classify_field`
/// (the older, bare-substring scan `enrich_spell_raw_tokens.rs` uses), OR
/// `pi_screening::blacklist_term_hit_including_concatenated` (the
/// word-bounded, OCR-normalized, concatenation-aware scan
/// `cache_gen::class_feature::redact_concatenated_blacklist_tokens` uses,
/// added when re-deriving `corpus_literal_sweep` against the pinned oracle
/// found `class_feature`'s KEY/PREDEITY tokens legitimately redacted for a
/// misspelled deity name -- `"Cayden Callean"`, an OCR-fold-equivalent typo
/// of the blacklisted `"Cayden Cailean"` -- that `classify_field`'s
/// unbounded literal substring check alone cannot see). Purely additive: a
/// token that neither scan's re-derivation backs is still reported exactly
/// as before, so this widening cannot hide a real transcription defect,
/// only recognise a second write path's own legitimate redaction shape --
/// so the exemption is re-derived against the oracle every sweep run, never
/// merely asserted by the shipped record.
///
/// **The typed-field cross-check (`OPEN-ISSUES.md` row 91).** Before this,
/// the sweep compared ONLY `data.raw_tokens` against the closure — and
/// because the `enrich_*_raw_tokens` binaries harvest `raw_tokens` FROM the
/// cited row, that comparison is tautological at write time whenever
/// `raw_tokens` and a record's OTHER typed fields (`cost_gp`/`weight_lbs`)
/// come from independent sources, exactly as `cache_gen::equipment_gap` and
/// `cache_gen::hand_authored_equipment` are shaped: `cost_gp`/`weight_lbs`
/// are read from a hand-transcribed Rust table, `raw_tokens` from whatever
/// row `find_citation` resolves — so a wrong citation could leave
/// `raw_tokens` sweep-CLEAN while `cost_gp` silently disagreed with the real
/// corpus. `cost_gp`/`weight_lbs`, when the record states one, must now be
/// byte-derivable from a `COST:`/`WT:` entry in the SAME closure `raw_tokens`
/// is checked against — closing the gap without touching the `raw_tokens`
/// check itself.
///
/// **A third, still-narrower exemption for `decisions.md §24`-renamed
/// records.** A unit whose own name IS Product Identity ships under a
/// Codex-generated neutral name (`codex_neutral_name`), and `§24b`-2 requires
/// the PI original appear nowhere that ships — not only in `DESC`, and not
/// only in tokens that independently re-screen as blacklisted, but in ANY
/// `raw_tokens` entry whose value restates the original name/key
/// (`ingest_ability.py`'s own docstring: a `KEY:` field can carry the row's
/// PI name a second time, and its value need not itself be a blacklisted
/// term — `KEY:Trait ~ Guardian of the Forge` redacts because it restates
/// the record's own original name, not because "Guardian of the Forge" is on
/// the blacklist scan). Neither exemption above covers that shape: the first
/// requires `pi_redacted_description` (declared only for a bare
/// `pi_field == "description"`, false for `§24`'s
/// `pi_field: "description,name,raw_tokens"`); the second requires the real
/// corpus row's own same-key value to independently re-screen as
/// blacklisted, which a non-PI phrase like "Guardian of the Forge" never
/// will. So: when `record.codex_generated_name` is `true` (read off the
/// record's own `data.codex_generated_name` field — never inferred from a
/// filename convention), a token whose value is EXACTLY
/// [`REDACTED_PI_MARKER`] is exempt, on ANY key, because `§24b`-2's
/// redaction is not confined to one field the way SD-30/31's precedents
/// were.
///
/// This is deliberately as narrow as the ruling that created it and no
/// narrower is safe, no broader is sound:
/// - **Only** a record carrying the record's own `codex_generated_name: true`
///   marker gets the exemption at all — an ordinary record that happens to
///   ship the literal string `"[redacted PI]"` in some field is NOT exempt
///   (see `the_redaction_exemption_does_not_cover_other_tokens_or_undeclared_records`
///   above, which already proves this for the general marker string).
/// - **Only** a token whose value is the exact sentinel is exempt — every
///   OTHER token on a `§24`-renamed record, including one that merely
///   happens to drift from the real corpus row without reading the marker,
///   still must byte-match exactly as before. A `§24` record is not exempt
///   from the sweep; one token in it is.
/// - The exemption is **counted**, never silent
///   ([`SweepTally::codex_generated_name_tokens_exempted`]/
///   [`SweepTally::codex_generated_name_records_exempted`]), so the gate log
///   states how many tokens it excused and in how many records
///   (`§22`'s divergence-must-be-visible condition, `§24b`-4).
///
/// **A fourth, sibling exemption for the same `§24` shape (t9-onboarding,
/// corpus-literal-sweep-remainder cycle): a self-referential token whose
/// redaction is the NEUTRAL NAME, not the bare marker.** `KEY`/`TYPE`/`DESC`
/// redact to the literal [`REDACTED_PI_MARKER`] sentinel (the third
/// exemption above), but a token whose semantic content IS the record's own
/// name used mechanically -- `BONUS:ABILITYPOOL|<name>|1|TYPE=Base` (the
/// ability's own pool, conventionally named after the ability),
/// `PREMULT:1,[PREABILITY:1,CATEGORY=...,<name>],...` (a prerequisite
/// referencing this same ability by name), `ABILITY:...|<parent name> ~
/// <child name>|...` (a namespaced child ability under this record's own
/// renamed parent key) -- cannot redact to the bare marker without shipping
/// a value the compute engine cannot read as a pool/ability name at all.
/// These generators correctly substitute the record's own neutral name in
/// place of the original, which by construction still differs from the real
/// corpus row's same-key value (the real row still carries the original PI
/// name there) -- reproduced live re-deriving this sweep against the pinned
/// oracle: 15 findings across 6 records
/// (`adventurers_guide`/`inner_sea_gods`/`inner_sea_magic`/
/// `inner_sea_world_guide`, `class_feature`/`trait_generic`/`feat_generic`),
/// every one a `codex_generated_name: true` record whose flagged token's
/// value contains that same record's own neutral name as a substring.
/// Exempt ONLY when the token's value contains one of the record's own
/// `identities` (`data.key`/`data.name`/`source.record_key`, all equal to
/// the neutral name on a `§24` record) as a substring -- narrower than the
/// third exemption in scope (a substring match, not exact) but identically
/// narrow in gating (only `codex_generated_name` records, only a
/// self-referential token, counted the same way, never silent). A token
/// that drifts for an unrelated reason and does not reference the record's
/// own renamed identity anywhere in its value is still compared and still
/// reported
/// (`a_codex_generated_name_record_still_catches_a_non_self_referential_drifted_token`).
pub fn compare_tokens(
    record: &ShippedRecord,
    closure: &BTreeSet<String>,
    book_corpus_tokens: &BTreeSet<String>,
    tally: &mut SweepTally,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for token in &record.tokens {
        tally.tokens_compared += 1;
        if record.pi_redacted_description && token.key == "DESC" && token.value == REDACTED_PI_MARKER {
            continue;
        }
        if token.key != "DESC"
            && token.value == REDACTED_PI_MARKER
            && closure.iter().any(|field| {
                field.split_once(':').is_some_and(|(key, raw_value)| {
                    key == token.key
                        && (classify_field(key, raw_value).0 != License::Ogl
                            || blacklist_term_hit_including_concatenated(raw_value).is_some())
                })
            })
        {
            continue;
        }
        // decisions.md §24 (see this function's doc comment): a
        // Codex-generated-neutral-name record's own redaction is not
        // confined to DESC or to tokens that independently re-screen as
        // blacklisted -- `§24b`-2 redacts ANY token restating the original
        // name/key. Exempt ONLY the exact sentinel, ONLY on a record
        // carrying the record's own `codex_generated_name: true` marker,
        // and count it -- the divergence stays visible, never silent.
        if record.codex_generated_name && token.value == REDACTED_PI_MARKER {
            tally.codex_generated_name_tokens_exempted += 1;
            tally.codex_generated_name_records_exempted.insert(record.record_path.clone());
            continue;
        }
        // A second §24 shape (t9-onboarding, corpus-literal-sweep-remainder
        // cycle): a self-referential token that names the record's OWN
        // ability/pool/prerequisite by its own key is not redacted to the
        // bare marker at all -- it is redacted the way `§24b` actually
        // requires content to ship usably: the neutral name is substituted
        // IN PLACE of the original name, embedded inside an otherwise
        // ordinary, mechanically-real token (`BONUS:ABILITYPOOL|<neutral
        // name>|1|TYPE=Base`, `PREMULT:1,[PREABILITY:1,CATEGORY=...,<neutral
        // name>],...`, `ABILITY:...|<neutral name> ~ Costume
        // Proficiency|...` for a child ability namespaced under the
        // renamed parent). Stamping the bare marker into `BONUS:`/`ABILITY:`
        // would ship a value the compute engine cannot read as a pool/ability
        // name at all -- substituting the neutral name is the correct
        // shape, not a defect, and the real corpus closure's same-key row
        // necessarily differs by exactly the name substring (the real row
        // still carries the original PI name there). Exempt ONLY when the
        // token's value contains one of the record's OWN identities
        // (`data.key`/`data.name`/`source.record_key`, all equal to the
        // neutral name on a codex_generated_name record) as a substring --
        // narrow by construction: a token that merely happens to drift for
        // an unrelated reason, with no self-reference to the record's own
        // renamed identity anywhere in its value, is still compared and
        // still reported (see
        // `a_codex_generated_name_record_still_catches_a_non_self_referential_drifted_token`).
        if record.codex_generated_name
            && record.identities.iter().any(|identity| !identity.is_empty() && token.value.contains(identity.as_str()))
        {
            tally.codex_generated_name_tokens_exempted += 1;
            tally.codex_generated_name_records_exempted.insert(record.record_path.clone());
            continue;
        }
        let joined = token.joined();
        if let Some((_, lst_name)) =
            SYNTHESIZED_TOKEN_KEYS.iter().find(|(key, _)| *key == token.key)
        {
            tally.synthesized_tokens_compared += 1;
            let as_corpus_token = format!("{lst_name}:{}", token.value);
            if !book_corpus_tokens.contains(&as_corpus_token) {
                findings.push(Finding::SynthesizedTokenNotInCorpus {
                    record: record.record_path.clone(),
                    token: joined,
                });
            }
            continue;
        }
        if token.key.contains(':') {
            findings.push(Finding::UnknownSynthesizedKey {
                record: record.record_path.clone(),
                key: token.key.clone(),
            });
            continue;
        }
        if !closure.contains(&joined) {
            findings.push(Finding::TokenNotInClosure {
                record: record.record_path.clone(),
                token: joined,
            });
        }
    }
    findings.extend(compare_typed_numeric_field(
        &record.record_path,
        "cost_gp",
        "COST",
        record.cost_gp,
        closure,
        tally,
    ));
    findings.extend(compare_typed_numeric_field(
        &record.record_path,
        "weight_lbs",
        "WT",
        record.weight_lbs,
        closure,
        tally,
    ));
    findings
}

/// One record's claim that a named corpus file had a named digest when the
/// record was written.
///
/// Deliberately a SEPARATE population from [`ShippedRecord`]. Only records
/// carrying `raw_tokens` have transcribed tokens to compare, but every record
/// citing a `source.sha256` — spells, monsters, companions and feats included
/// — is making a falsifiable claim about the corpus, and there is no reason a
/// record without `raw_tokens` should get to make it unchecked. Scoping the
/// digest check to the token population would have verified 3,516 of the
/// 8,903 claims on the tree and reported that as full coverage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceClaim {
    pub record_path: String,
    pub source_path: String,
    pub sha256: String,
}

/// Compare one provenance claim against the corpus file's real digest.
pub fn compare_digest(claim: &ProvenanceClaim, actual: &str) -> Option<Finding> {
    if claim.sha256 == actual {
        return None;
    }
    Some(Finding::DigestDrift {
        record: claim.record_path.clone(),
        source_path: claim.source_path.clone(),
        claimed: claim.sha256.clone(),
        actual: actual.to_string(),
    })
}

/// Read one shipped JSON record into the sweep's shape.
///
/// `Ok(None)` means the record is outside the sweep's population — not an
/// `lst_token` citation, or carrying no `raw_tokens` array to compare. `Err`
/// means the file is malformed, which is a finding for the caller to report,
/// never something to skip.
pub fn parse_record(record_path: &str, text: &str) -> Result<Option<ShippedRecord>, String> {
    Ok(parse_document(record_path, text)?.record)
}

/// Both populations one shipped JSON file can belong to, from a single parse.
///
/// `PartialEq` only, not `Eq`: `ShippedRecord` carries `Option<f64>` fields.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParsedDocument {
    /// The provenance claim, when the record cites a file and a digest —
    /// checked for every record regardless of `source.kind`.
    pub provenance: Option<ProvenanceClaim>,
    /// The transcribed record, when there are tokens to compare.
    pub record: Option<ShippedRecord>,
}

/// Read one shipped JSON record into both of the sweep's populations.
pub fn parse_document(record_path: &str, text: &str) -> Result<ParsedDocument, String> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("{record_path}: invalid JSON: {e}"))?;
    let Some(source) = value.get("source").and_then(serde_json::Value::as_object) else {
        return Ok(ParsedDocument::default());
    };
    let provenance = match (
        source.get("path").and_then(serde_json::Value::as_str),
        source.get("sha256").and_then(serde_json::Value::as_str),
    ) {
        (Some(path), Some(sha256)) => Some(ProvenanceClaim {
            record_path: record_path.to_string(),
            source_path: path.to_string(),
            sha256: sha256.to_string(),
        }),
        _ => None,
    };
    let record = parse_transcription(record_path, source, &value)?;
    Ok(ParsedDocument { provenance, record })
}

fn parse_transcription(
    record_path: &str,
    source: &serde_json::Map<String, serde_json::Value>,
    value: &serde_json::Value,
) -> Result<Option<ShippedRecord>, String> {
    if source.get("kind").and_then(serde_json::Value::as_str) != Some("lst_token") {
        return Ok(None);
    }
    let Some(data) = value.get("data").and_then(serde_json::Value::as_object) else {
        return Ok(None);
    };
    let Some(raw_tokens) = data.get("raw_tokens").and_then(serde_json::Value::as_array) else {
        return Ok(None);
    };
    let source_path = source
        .get("path")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| format!("{record_path}: source.kind is lst_token but source.path is absent"))?
        .to_string();
    let source_line = source
        .get("line")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| format!("{record_path}: source.kind is lst_token but source.line is absent"))?
        as usize;
    let mut tokens = Vec::with_capacity(raw_tokens.len());
    for entry in raw_tokens {
        let key = entry
            .get("key")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("{record_path}: a raw_tokens entry has no string key"))?;
        let value = entry
            .get("value")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("{record_path}: raw_tokens entry {key} has no string value"))?;
        tokens.push(ShippedToken { key: key.to_string(), value: value.to_string() });
    }
    let mut identities = BTreeSet::new();
    for candidate in [data.get("key"), data.get("name"), source.get("record_key")] {
        if let Some(name) = candidate.and_then(serde_json::Value::as_str) {
            identities.insert(name.to_string());
        }
    }
    // `pi_field` is a comma-separated LIST of redacted fields once more than
    // one field was redacted on the same record (`declared_pi_shipping_
    // audit.rs` established this same `split(',').any(...)` reading for the
    // identical reason: SD-32's newer ingest paths stamp
    // `pi_field: "description,raw_tokens"`-shaped values, not a bare
    // `"description"`, the moment a record's DESC redaction is accompanied
    // by another redacted raw_tokens entry). The prior exact-equality check
    // silently stopped recognising this record's real, declared DESC
    // redaction as the DECLARED exemption it is -- caught live re-deriving
    // this sweep against the pinned oracle: `inner_sea_magic/ability/
    // diplomatic_student.json` (`pi_field: "description,raw_tokens"`,
    // `DESCISPI:YES` on the real corpus row) went MISMATCH on its own DESC
    // token though the redaction is genuine.
    let pi_redacted_description = value.get("license").and_then(serde_json::Value::as_str) == Some("PI-REDACTED")
        && value
            .get("pi_field")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|f| f.split(',').any(|part| part == "description"));
    // `§24b`-3's own visible marker -- read off the record's own top-level
    // JSON field, never inferred from `record_path`/filename (a
    // `codex_named_unit_*` filename convention is not proof; see
    // `compare_tokens`'s doc comment).
    let codex_generated_name =
        value.get("codex_generated_name").and_then(serde_json::Value::as_bool).unwrap_or(false);
    let cost_gp = data.get("cost_gp").and_then(serde_json::Value::as_f64);
    let weight_lbs = data.get("weight_lbs").and_then(serde_json::Value::as_f64);
    Ok(Some(ShippedRecord {
        record_path: record_path.to_string(),
        source_path,
        source_line,
        source_sha256: source
            .get("sha256")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string),
        identities,
        tokens,
        pi_redacted_description,
        codex_generated_name,
        cost_gp,
        weight_lbs,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(tokens: &[(&str, &str)]) -> ShippedRecord {
        ShippedRecord {
            record_path: "data/corpus/x/equipment/general/thing.json".to_string(),
            source_path: "pathfinder/paizo/roleplaying_game/x/x_equip.lst".to_string(),
            source_line: 4,
            source_sha256: Some("aa".repeat(32)),
            identities: ["Thing".to_string()].into_iter().collect(),
            tokens: tokens
                .iter()
                .map(|(k, v)| ShippedToken { key: k.to_string(), value: v.to_string() })
                .collect(),
            pi_redacted_description: false,
            codex_generated_name: false,
            cost_gp: None,
            weight_lbs: None,
        }
    }

    fn closure_of(rows: &[&str], identities: &BTreeSet<String>) -> BTreeSet<String> {
        let mut index: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for row in &rows[1..] {
            let head = row.split('\t').next().unwrap_or("");
            if let Some(at) = head.find(".MOD") {
                index
                    .entry(crate::rules_core::wiring_class::mod_base_name(&head[..at]))
                    .or_default()
                    .push((*row).to_string());
            }
        }
        token_closure(rows[0], identities, &index, None)
    }

    // ---- the detection cases: each of these MUST go red on a corrupted record

    #[test]
    fn a_value_that_drifted_by_one_byte_is_a_finding() {
        let rec = record(&[("COST", "50"), ("WT", "3")]);
        let rows = ["Thing\tCOST:5\tWT:3"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(
            findings,
            vec![Finding::TokenNotInClosure {
                record: rec.record_path.clone(),
                token: "COST:50".to_string(),
            }],
            "a shipped COST of 50 over a corpus COST of 5 must be reported"
        );
        assert_eq!(tally.tokens_compared, 2);
    }

    /// `OPEN-ISSUES.md` row 48/49: a record whose description was
    /// genuinely PI-redacted (`license: "PI-REDACTED"`,
    /// `pi_field: "description"`) legitimately carries a `DESC` raw_token
    /// reading `[redacted PI]`, which by construction differs from the
    /// real corpus row's own `DESC:` value. That must NOT be reported.
    #[test]
    fn a_declared_pi_redacted_desc_token_is_exempt_from_the_byte_match() {
        let mut rec = record(&[("COST", "5"), ("DESC", "[redacted PI]")]);
        rec.pi_redacted_description = true;
        let rows = ["Thing\tCOST:5\tDESC:The real Golarion-specific prose."];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(findings, vec![], "a declared-redacted DESC token must not be flagged");
    }

    /// `SD31-E6-F10-001`: `enrich_spell_raw_tokens.rs` redacts ANY token
    /// (not only `DESC`) whose value hits the shared blacklist term scan
    /// (`enrich_one`'s `blacklisted = blacklist_license != License::Ogl`
    /// branch, applied per-token) -- so a real `FACTSET:` token naming a
    /// deity on an Inner Sea Gods spell (`FACTSET:Deity|Sarenrae`-shaped)
    /// legitimately stores `[redacted PI]`, exactly like a redacted `DESC`
    /// does. The pre-existing DESC-only exemption above does not cover it,
    /// so 51 real, correctly-redacted records went `MISMATCH` for a
    /// redaction that is legitimate, not a transcription defect
    /// (re-derived against the real pinned oracle,
    /// `cargo run --locked --bin corpus_literal_sweep`, before this fix).
    ///
    /// Narrower than the DESC exemption in one respect and safer in
    /// another: rather than trusting a record-level flag
    /// (`pi_redacted_description`, which only ever describes `DESC`), this
    /// exemption RE-SCREENS the real corpus row's own same-key value
    /// through the identical blacklist scan the enrichment write path used
    /// -- so a token that merely happens to read the marker string by
    /// coincidence, with no real blacklist hit backing it, is still
    /// reported (see the sibling test below).
    #[test]
    fn a_non_desc_token_whose_raw_corpus_value_independently_reconfirms_as_blacklisted_is_exempt() {
        let rec = record(&[("COST", "5"), ("FACTSET", "[redacted PI]")]);
        let rows = ["Thing\tCOST:5\tFACTSET:Deity|Sarenrae"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(
            findings,
            vec![],
            "a FACTSET token legitimately redacted for a real deity-name blacklist hit must \
             not be flagged"
        );
    }

    /// The re-screen genuinely runs: a token whose value happens to equal
    /// the marker string, but whose real corpus row carries a CLEAN
    /// (non-blacklisted) value under the same key, is not exempt -- proving
    /// this is a real re-derivation, not a blanket "trust the marker" hole.
    #[test]
    fn a_token_merely_reading_the_marker_string_with_no_real_blacklist_backing_it_is_still_flagged() {
        let rec = record(&[("COST", "5"), ("FACTSET", "[redacted PI]")]);
        let rows = ["Thing\tCOST:5\tFACTSET:SomeOrdinaryValue"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(
            findings,
            vec![Finding::TokenNotInClosure {
                record: rec.record_path.clone(),
                token: "FACTSET:[redacted PI]".to_string(),
            }],
            "a token reading the marker string with no independently-reconfirmed blacklist \
             hit on the real corpus row must still be reported"
        );
    }

    /// t9-onboarding cycle (2026-08-23), `corpus_literal_sweep` unblock: the
    /// widened re-screen must recognise a legitimate `class_feature`
    /// redaction that `classify_field`'s bare-substring scan alone cannot
    /// see -- the real, live shape: `inner_sea_combat/class_feature/
    /// ranger_combat_style/cayden_callean.json`'s KEY/PREDEITY tokens are
    /// genuinely redacted for "Cayden Callean", a misspelled (double-L)
    /// variant of the blacklisted "Cayden Cailean" that only the
    /// word-bounded, OCR-normalized scan catches (both spellings fold to
    /// the same canonical form under the l/1/!->i table).
    #[test]
    fn a_non_desc_token_whose_value_only_the_concatenated_scan_reconfirms_is_exempt() {
        let rec = record(&[("KEY", "[redacted PI]")]);
        let rows = ["Cayden Callean\tKEY:Ranger Combat Style ~ Cayden Callean"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(
            findings,
            vec![],
            "a KEY token legitimately redacted for a misspelled-deity-name \
             concatenated-scan hit must not be flagged"
        );
    }

    /// The widening is purely additive: it must not also cause a genuinely
    /// wrong token (real corpus value clean under BOTH scans) to be waved
    /// through -- proving `§1a` directly for the new disjunct, mirroring the
    /// sibling proof above for the pre-existing `classify_field` disjunct.
    #[test]
    fn the_concatenated_scan_disjunct_does_not_wave_through_a_genuinely_clean_value() {
        let rec = record(&[("KEY", "[redacted PI]")]);
        let rows = ["Thing\tKEY:Some Ordinary Non PI Value"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(
            findings,
            vec![Finding::TokenNotInClosure {
                record: rec.record_path.clone(),
                token: "KEY:[redacted PI]".to_string(),
            }],
            "a token reading the marker string with no independently-reconfirmed hit under \
             EITHER re-screen must still be reported"
        );
    }

    /// The exemption is narrow: it does not extend to any OTHER drifted
    /// token on a redacted record, and it does not fire at all unless the
    /// record's own metadata declares the redaction.
    #[test]
    fn the_redaction_exemption_does_not_cover_other_tokens_or_undeclared_records() {
        // Other tokens on a redacted record still must byte-match.
        let mut rec = record(&[("COST", "50"), ("DESC", "[redacted PI]")]);
        rec.pi_redacted_description = true;
        let rows = ["Thing\tCOST:5\tDESC:The real Golarion-specific prose."];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally);
        assert_eq!(
            findings,
            vec![Finding::TokenNotInClosure { record: rec.record_path.clone(), token: "COST:50".to_string() }],
            "COST drift on a redacted record must still be reported"
        );

        // The literal marker string on a record that is NOT declared
        // redacted is still checked normally (and fails, as it should).
        let rec2 = record(&[("DESC", "[redacted PI]")]);
        assert!(!rec2.pi_redacted_description);
        let rows2 = ["Thing\tDESC:The real prose."];
        let mut tally2 = SweepTally::default();
        let findings2 = compare_tokens(&rec2, &closure_of(&rows2, &rec2.identities), &BTreeSet::new(), &mut tally2);
        assert_eq!(
            findings2,
            vec![Finding::TokenNotInClosure {
                record: rec2.record_path.clone(),
                token: "DESC:[redacted PI]".to_string()
            }],
            "the marker string on an UNDECLARED record must still be checked normally"
        );
    }

    /// `decisions.md §24`: a `codex_generated_name: true` record's own
    /// redaction is not confined to `DESC`, and does not require the real
    /// corpus row's same-key value to independently re-screen as
    /// blacklisted (`KEY:Trait ~ Guardian of the Forge` -- neither term is
    /// on the blacklist scan, yet the value legitimately restates the
    /// record's original name and is redacted per `§24b`-2). Every such
    /// token, on ANY key, must be exempt -- and the exemption must be
    /// counted, not silent.
    #[test]
    fn a_codex_generated_name_records_multi_field_redaction_is_exempt_and_counted() {
        let mut rec = record(&[
            ("KEY", "[redacted PI]"),
            ("CATEGORY", "Special Ability"),
            ("TYPE", "[redacted PI]"),
            ("DESC", "[redacted PI]"),
        ]);
        rec.codex_generated_name = true;
        let rows = ["Thing\tKEY:Trait ~ Guardian of the Forge\tCATEGORY:Special Ability\tTYPE:Trait.ReligionTrait\tDESC:Torag's sacred duties..."];
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally);
        assert_eq!(
            findings,
            vec![],
            "every §24-redacted token, on any key, must be exempt from the byte-match"
        );
        assert_eq!(
            tally.codex_generated_name_tokens_exempted, 3,
            "KEY, TYPE, and DESC were exempted; CATEGORY was a real byte-match, not an exemption"
        );
        assert_eq!(tally.codex_generated_name_records_exempted.len(), 1);
    }

    /// The `§24` exemption is exactly as narrow as the ruling: a token on a
    /// `codex_generated_name: true` record that does NOT read the exact
    /// sentinel is still checked normally, and still fails when it drifts.
    /// A record cannot smuggle an unrelated defect through by merely
    /// claiming to be §24-renamed.
    #[test]
    fn a_codex_generated_name_record_still_catches_a_non_redacted_drifted_token() {
        let mut rec = record(&[("KEY", "[redacted PI]"), ("COST", "50")]);
        rec.codex_generated_name = true;
        let rows = ["Thing\tKEY:Some PI Name\tCOST:5"];
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally);
        assert_eq!(
            findings,
            vec![Finding::TokenNotInClosure {
                record: rec.record_path.clone(),
                token: "COST:50".to_string(),
            }],
            "a non-redacted drifted token on a §24-renamed record must still be reported"
        );
        assert_eq!(tally.codex_generated_name_tokens_exempted, 1, "only KEY was exempted");
    }

    /// An UNMARKED record (`codex_generated_name: false`, the default) gets
    /// no §24 exemption even if some token happens to read the sentinel and
    /// even if other tokens on the same record are legitimate --
    /// confirming the marker, not the value alone, gates the exemption.
    #[test]
    fn an_unmarked_record_gets_no_24_exemption_for_the_sentinel_value() {
        let rec = record(&[("KEY", "[redacted PI]"), ("COST", "5")]);
        assert!(!rec.codex_generated_name);
        let rows = ["Thing\tKEY:Some Real Value\tCOST:5"];
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally);
        assert_eq!(
            findings,
            vec![Finding::TokenNotInClosure {
                record: rec.record_path.clone(),
                token: "KEY:[redacted PI]".to_string(),
            }],
            "the sentinel value alone, on a record NOT marked codex_generated_name, must still be a finding"
        );
        assert_eq!(tally.codex_generated_name_tokens_exempted, 0);
    }

    /// The fourth `§24` exemption (t9-onboarding, corpus-literal-sweep-
    /// remainder cycle): a self-referential token whose redaction is the
    /// record's own NEUTRAL NAME embedded inside an otherwise-mechanical
    /// value, not the bare marker -- `BONUS:ABILITYPOOL|<name>|1|TYPE=Base`,
    /// a `PREMULT:` prerequisite naming this same ability, and an `ABILITY:`
    /// child namespaced under this record's own renamed parent key. All
    /// three reproduce the live shape found re-deriving this sweep against
    /// the pinned oracle (15 findings / 6 records before this fix).
    #[test]
    fn a_self_referential_token_containing_the_records_own_neutral_name_is_exempt() {
        let mut rec = record(&[
            ("KEY", "Codex-Named Unit (class_feature_x_1)"),
            ("BONUS", "ABILITYPOOL|Codex-Named Unit (class_feature_x_1)|1|TYPE=Base"),
            (
                "PREMULT",
                "1,[PREABILITY:1,CATEGORY=Special Ability,Codex-Named Unit (class_feature_x_1)]",
            ),
            (
                "ABILITY",
                "Special Ability|AUTOMATIC|Codex-Named Unit (class_feature_x_1) ~ Costume Proficiency|PRECLASS:1,Bard=5",
            ),
            ("COST", "5"),
        ]);
        rec.codex_generated_name = true;
        rec.identities = ["Codex-Named Unit (class_feature_x_1)".to_string()].into_iter().collect();
        let rows = [
            "Thing\tKEY:The Real PI Name\tBONUS:ABILITYPOOL|The Real PI Name|1|TYPE=Base\
             \tPREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,The Real PI Name]\
             \tABILITY:Special Ability|AUTOMATIC|The Real PI Name ~ Costume Proficiency|PRECLASS:1,Bard=5\
             \tCOST:5",
        ];
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally);
        assert_eq!(
            findings,
            vec![],
            "every token that merely restates the record's own renamed identity must be exempt"
        );
        assert_eq!(
            tally.codex_generated_name_tokens_exempted, 4,
            "KEY, BONUS, PREMULT, and ABILITY all self-reference the renamed identity; COST is a \
             real byte-match, not an exemption"
        );
        assert_eq!(tally.codex_generated_name_records_exempted.len(), 1);
    }

    /// Narrowness proof for the fourth exemption: a token on a
    /// `codex_generated_name: true` record that drifts for a reason
    /// UNRELATED to the record's own renamed identity -- it neither reads
    /// the bare marker nor contains the identity substring anywhere -- must
    /// still be reported. A `§24` record cannot smuggle an unrelated defect
    /// through by merely containing some OTHER self-reference elsewhere.
    #[test]
    fn a_codex_generated_name_record_still_catches_a_non_self_referential_drifted_token() {
        let mut rec = record(&[
            ("KEY", "Codex-Named Unit (class_feature_x_1)"),
            ("COST", "50"),
        ]);
        rec.codex_generated_name = true;
        rec.identities = ["Codex-Named Unit (class_feature_x_1)".to_string()].into_iter().collect();
        let rows = ["Thing\tKEY:The Real PI Name\tCOST:5"];
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally);
        assert_eq!(
            findings,
            vec![Finding::TokenNotInClosure {
                record: rec.record_path.clone(),
                token: "COST:50".to_string(),
            }],
            "a drifted COST token with no self-reference to the renamed identity must still be \
             reported, even though the record is §24-renamed"
        );
        assert_eq!(tally.codex_generated_name_tokens_exempted, 1, "only KEY self-referenced");
    }

    #[test]
    fn a_token_the_corpus_row_never_carried_is_a_finding() {
        let rec = record(&[("COST", "50"), ("SPELLFAILURE", "35")]);
        let rows = ["Thing\tCOST:50"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert!(matches!(&findings[0], Finding::TokenNotInClosure { token, .. } if token == "SPELLFAILURE:35"));
    }

    // ---- SD31-E6-F6-001: `.COPY=` inheritance closure ----
    //
    // `gen_equipment_gap_tables.rs`'s `.COPY=` inheritance (rows 70/103's
    // recovery, generalized to `cost_gp`/`weight_lbs`) ships a value that
    // genuinely appears in the corpus, but on the BASE row, not the `.COPY=`
    // row a record's citation names. Without `copy_base_row`, that value is
    // real but the closure this function builds can never prove it — the
    // exact "provable one record deep" bar this check exists to enforce.

    /// The reproduction, from the real corpus (`BOWSTR`): the cited row is a
    /// bare `.COPY=` declaration with no `COST:` token; the shipped
    /// `cost_gp` (inherited from the base row) is only provable once the
    /// base row's own tokens join the closure.
    #[test]
    fn a_copy_rows_closure_without_its_base_cannot_prove_an_inherited_cost() {
        let mut rec = record(&[]);
        rec.cost_gp = Some(0.0);
        let rows = ["Special Quality ~ Composite Bow Strength Rating.COPY=BOWSTR\tVISIBLE:NO"];
        let mut tally = SweepTally::default();
        let closure = token_closure(rows[0], &rec.identities, &BTreeMap::new(), None);
        let findings = compare_tokens(&rec, &closure, &BTreeSet::new(), &mut tally);
        assert_eq!(
            findings,
            vec![Finding::TypedFieldNotInClosure {
                record: rec.record_path.clone(),
                field: "cost_gp",
                shipped_value: "0".to_string(),
                lst_key: "COST",
            }],
            "a .COPY= row's empty own line must not silently prove an inherited value"
        );
    }

    /// The fix: passing the resolved base row lets the SAME inherited
    /// `cost_gp` prove clean — the value is real, only the closure needed
    /// widening to see where it is actually stated.
    #[test]
    fn a_copy_rows_closure_with_its_resolved_base_proves_the_inherited_cost() {
        let mut rec = record(&[]);
        rec.cost_gp = Some(0.0);
        let copy_row = "Special Quality ~ Composite Bow Strength Rating.COPY=BOWSTR\tVISIBLE:NO";
        let base_row = "Composite Bow Strength Rating\tKEY:Special Quality ~ Composite Bow Strength Rating\tCOST:0";
        let mut tally = SweepTally::default();
        let closure =
            token_closure(copy_row, &rec.identities, &BTreeMap::new(), Some(base_row));
        let findings = compare_tokens(&rec, &closure, &BTreeSet::new(), &mut tally);
        assert_eq!(findings, Vec::new(), "the base row's real COST:0 must now be found");
    }

    /// A plain (non-`.COPY=`) row is unaffected by a `copy_base_row` that
    /// happens to be passed anyway — the base's tokens still merge in
    /// (defensive; in practice the caller only resolves a base for a
    /// genuine `.COPY=` row), but a normal record's own closure is not
    /// narrowed or otherwise changed by this parameter's mere presence.
    #[test]
    fn a_plain_rows_own_tokens_are_unaffected_by_an_absent_copy_base() {
        let rec = record(&[("COST", "50")]);
        let rows = ["Thing\tCOST:50"];
        let mut tally = SweepTally::default();
        let closure = token_closure(rows[0], &rec.identities, &BTreeMap::new(), None);
        let findings = compare_tokens(&rec, &closure, &BTreeSet::new(), &mut tally);
        assert_eq!(findings, Vec::new());
    }

    // ---- OPEN-ISSUES.md row 91: the typed-field cross-check ----
    //
    // `raw_tokens` and `cost_gp`/`weight_lbs` are, for the modules this
    // reproduces the real defect from (`cache_gen::equipment_gap`/
    // `cache_gen::hand_authored_equipment`), populated from INDEPENDENT
    // sources: `cost_gp` from a hand-transcribed Rust table, `raw_tokens`
    // from whatever row `find_citation` resolves. Before this fix,
    // `compare_tokens` never read `cost_gp` at all, so a wrong citation
    // could leave `raw_tokens` sweep-CLEAN (it was harvested from the same
    // wrong row it is compared against) while `cost_gp` silently disagreed
    // with the real corpus -- exactly `OPEN-ISSUES.md` row 90's confirmed,
    // shipped defect (`catapult_standard.json`'s `cost_gp=800` citing
    // `uc_profs_weapon.lst`, which never states 800 anywhere).

    /// The reproduction: a record's `cost_gp` is real (800) but its
    /// `raw_tokens` are empty (so the OLD, `raw_tokens`-only comparison
    /// finds nothing to check) and its closure states a DIFFERENT cost.
    /// Byte-equality on `raw_tokens` alone is vacuously satisfied; the
    /// typed-field check must still catch the drift.
    #[test]
    fn a_cost_gp_the_closure_never_states_is_a_finding_even_with_empty_raw_tokens() {
        let mut rec = record(&[]);
        rec.cost_gp = Some(800.0);
        let rows = ["Thing\tKEY:Thing"]; // no COST: token anywhere
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally);
        assert_eq!(
            findings,
            vec![Finding::TypedFieldNotInClosure {
                record: rec.record_path.clone(),
                field: "cost_gp",
                shipped_value: "800".to_string(),
                lst_key: "COST",
            }]
        );
        assert_eq!(tally.typed_fields_compared, 1, "weight_lbs is None and must not count");
    }

    /// The real row 90 shape, precisely: `cost_gp` is real (800) but the
    /// closure (built from the WRONG cited row, a proficiency listing with
    /// no `COST:` field at all) cannot derive it.
    #[test]
    fn the_real_catapult_standard_shape_trips_the_typed_field_check_pre_fix() {
        let mut rec = record(&[]);
        rec.identities = ["Catapult (Standard)".to_string()].into_iter().collect();
        rec.cost_gp = Some(800.0);
        // the WRONG cited row (a proficiency listing, no COST:)
        let rows = ["Catapult\tKEY:Catapult (Standard)\tTYPE:Exotic.Ranged.SiegeEngine"];
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally);
        assert!(
            findings.iter().any(|f| matches!(
                f,
                Finding::TypedFieldNotInClosure { field, lst_key, .. } if *field == "cost_gp" && *lst_key == "COST"
            )),
            "{findings:?}"
        );
    }

    /// A `cost_gp` that DOES match a `COST:` entry in the closure is not a
    /// finding -- the check is real, not a blanket new failure mode.
    #[test]
    fn a_cost_gp_present_in_the_closure_is_not_a_finding() {
        let mut rec = record(&[]);
        rec.identities = ["Catapult (Standard)".to_string()].into_iter().collect();
        rec.cost_gp = Some(800.0);
        rec.weight_lbs = Some(12.0);
        let rows = ["Catapult (Standard)\tPROFICIENCY:WEAPON|Catapult (Standard)\tCOST:800\tWT:12"];
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally);
        assert_eq!(findings, vec![], "{findings:?}");
        assert_eq!(tally.typed_fields_compared, 2);
    }

    /// A record whose typed schema states no `cost_gp`/`weight_lbs` at all
    /// (`None`) is not in this check's population -- absence on the record
    /// side is not compared, only a stated value that cannot be derived.
    #[test]
    fn a_record_with_no_typed_fields_is_unaffected() {
        let rec = record(&[("SOURCEPAGE", "p.1")]);
        assert!(rec.cost_gp.is_none() && rec.weight_lbs.is_none());
        let rows = ["Thing\tSOURCEPAGE:p.1"];
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally);
        assert_eq!(findings, vec![]);
        assert_eq!(tally.typed_fields_compared, 0);
    }

    /// Fractional and negative values round-trip through the finding's own
    /// display string without a spurious mismatch (a formatting bug here
    /// would silently widen the check's blast radius past what row 91
    /// scoped).
    #[test]
    fn negative_and_fractional_typed_values_compare_correctly() {
        let mut rec = record(&[]);
        rec.cost_gp = Some(-150.0);
        rec.weight_lbs = Some(0.5);
        let rows = ["Thing\tCOST:-150\tWT:.5"];
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally);
        assert_eq!(findings, vec![], "{findings:?}");
    }

    #[test]
    fn whitespace_is_not_normalised_away() {
        // Byte-equality means byte-equality: a trailing space the corpus does
        // not have is a mismatch, not a formatting difference to forgive.
        let rec = record(&[("DAMAGE", "1d8 ")]);
        let rows = ["Thing\tDAMAGE:1d8"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(findings.len(), 1, "{findings:?}");
    }

    fn claim() -> ProvenanceClaim {
        ProvenanceClaim {
            record_path: "data/corpus/x/spell/thing.json".to_string(),
            source_path: "pathfinder/paizo/roleplaying_game/x/x.lst".to_string(),
            sha256: "aa".repeat(32),
        }
    }

    #[test]
    fn a_digest_that_no_longer_matches_the_corpus_file_is_a_finding() {
        let finding = compare_digest(&claim(), &"bb".repeat(32));
        assert!(
            matches!(finding, Some(Finding::DigestDrift { .. })),
            "a corpus file that changed under a shipped record must be reported, got {finding:?}"
        );
    }

    #[test]
    fn an_undeclared_namespaced_key_is_a_finding_not_an_exemption() {
        let rec = record(&[("INVENTED:ABILITY", "whatever")]);
        let rows = ["Thing"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(
            findings,
            vec![Finding::UnknownSynthesizedKey {
                record: rec.record_path.clone(),
                key: "INVENTED:ABILITY".to_string(),
            }],
            "a new synthesized key must not inherit GLOBALVAR:ABILITY's exemption"
        );
    }

    #[test]
    fn a_synthesized_token_absent_from_the_book_corpus_is_a_finding() {
        let rec = record(&[("GLOBALVAR:ABILITY", "Aasimar Racial Trait|AUTOMATIC|Aasimar ~ Size|PREVAREQ:Aasimar_ReplaceSize,0")]);
        let rows = ["Thing"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert!(matches!(&findings[0], Finding::SynthesizedTokenNotInCorpus { .. }));
        assert_eq!(tally.synthesized_tokens_compared, 1);
    }

    // ---- the acceptance cases: what a faithful record looks like

    #[test]
    fn a_faithful_record_produces_no_findings() {
        let rec = record(&[("COST", "50"), ("WT", "3"), ("DAMAGE", "1d8")]);
        let rows = ["Thing\t\t\tCOST:50\t\t\tWT:3\t\t\t\t\tDAMAGE:1d8"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(findings, vec![], "padded empty tab fields are PCGen's own formatting");
        assert_eq!(tally.tokens_compared, 3);
    }

    #[test]
    fn a_token_carried_by_a_mod_row_in_the_same_book_is_faithful() {
        // The 25-record correction: comparing against the base row alone
        // reports a `.MOD`-carried token as drift when it is transcribed
        // exactly right.
        let rec = record(&[("COST", "50"), ("SR", "13")]);
        let rows = ["Thing\tCOST:50", "CATEGORY=Special Ability|Thing.MOD\tSR:13"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(findings, vec![]);
    }

    #[test]
    fn a_synthesized_token_present_in_the_book_corpus_is_faithful() {
        let value = "Aasimar Racial Trait|AUTOMATIC|Aasimar ~ Size|PREVAREQ:Aasimar_ReplaceSize,0";
        let rec = record(&[("GLOBALVAR:ABILITY", value)]);
        let rows = ["Thing"];
        let book: BTreeSet<String> = [format!("ABILITY:{value}")].into_iter().collect();
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &book, &mut tally);
        assert_eq!(findings, vec![]);
    }

    #[test]
    fn a_matching_digest_produces_no_finding() {
        assert_eq!(compare_digest(&claim(), &"aa".repeat(32)), None);
    }

    #[test]
    fn a_record_outside_the_token_population_still_makes_a_checkable_digest_claim() {
        // The coverage gap this population split closes: a spell record cites
        // a corpus file and a digest but carries no `raw_tokens`. Scoping the
        // digest check to the token population would leave 5,387 of the
        // tree's 8,903 claims unverified while reporting full coverage.
        let text = r#"{"source":{"kind":"web_second_source","path":"a.lst","sha256":"ff"},
                      "data":{"key":"Fireball"}}"#;
        let parsed = parse_document("spell.json", text).unwrap();
        assert_eq!(parsed.record, None, "no tokens to compare");
        assert_eq!(
            parsed.provenance,
            Some(ProvenanceClaim {
                record_path: "spell.json".to_string(),
                source_path: "a.lst".to_string(),
                sha256: "ff".to_string(),
            }),
            "but its provenance claim is still falsifiable and must be checked"
        );
    }

    #[test]
    fn a_record_claiming_no_digest_makes_no_claim_to_check() {
        let text = r#"{"source":{"kind":"web_second_source","path":"a.lst"},"data":{"key":"K"}}"#;
        assert_eq!(parse_document("r.json", text).unwrap().provenance, None);
    }

    // ---- parsing

    #[test]
    fn a_record_with_no_raw_tokens_is_out_of_population_not_a_pass() {
        let text = r#"{"source":{"kind":"lst_token","path":"a.lst","line":1},"data":{"key":"K"}}"#;
        assert_eq!(parse_record("r.json", text).unwrap(), None);
    }

    #[test]
    fn a_corrected_ingest_record_is_out_of_population() {
        let text = r#"{"source":{"kind":"lst_corrected_ingest","path":"a.lst","line":1},
                      "data":{"key":"K","raw_tokens":[{"key":"COST","value":"1"}]}}"#;
        assert_eq!(parse_record("r.json", text).unwrap(), None);
    }

    #[test]
    fn parsing_collects_every_identity_a_mod_row_could_target() {
        let text = r#"{"source":{"kind":"lst_token","path":"a.lst","line":7,"sha256":"ff","record_key":"RK"},
                      "data":{"key":"K","name":"N","raw_tokens":[{"key":"COST","value":"1"}]}}"#;
        let rec = parse_record("r.json", text).unwrap().expect("in population");
        assert_eq!(rec.source_line, 7);
        assert_eq!(rec.source_sha256.as_deref(), Some("ff"));
        assert_eq!(
            rec.identities,
            ["K".to_string(), "N".to_string(), "RK".to_string()].into_iter().collect()
        );
    }

    /// `pi_field` is a comma-separated LIST once more than one field on a
    /// record is redacted -- `pi_field: "description,raw_tokens"` must still
    /// clear the DESC-redaction check, matching
    /// `declared_pi_shipping_audit.rs`'s identical `split(',').any(...)`
    /// reading of the same field.
    #[test]
    fn a_multi_field_pi_field_list_including_description_still_exempts_desc() {
        let text = r#"{"source":{"kind":"lst_token","path":"a.lst","line":1},
                      "data":{"key":"K","raw_tokens":[{"key":"DESC","value":"[redacted PI]"}]},
                      "license":"PI-REDACTED","pi_field":"description,raw_tokens"}"#;
        let rec = parse_record("r.json", text).unwrap().expect("in population");
        assert!(rec.pi_redacted_description, "a comma-list pi_field containing \"description\" must still count");
    }

    /// `§24b`-3: the marker is read off the record's own top-level
    /// `data.codex_generated_name` field, not inferred from anything else.
    /// Absent (the vast majority of records) parses as `false`.
    #[test]
    fn codex_generated_name_is_parsed_from_the_records_own_top_level_field() {
        let text = r#"{"source":{"kind":"lst_token","path":"a.lst","line":1},
                      "data":{"key":"K","raw_tokens":[{"key":"COST","value":"1"}]},
                      "codex_generated_name":true}"#;
        let rec = parse_record("r.json", text).unwrap().expect("in population");
        assert!(rec.codex_generated_name);

        let absent = r#"{"source":{"kind":"lst_token","path":"a.lst","line":1},
                      "data":{"key":"K","raw_tokens":[{"key":"COST","value":"1"}]}}"#;
        let rec2 = parse_record("r.json", absent).unwrap().expect("in population");
        assert!(!rec2.codex_generated_name, "absent field must default false, not be assumed true");
    }

    #[test]
    fn malformed_json_is_an_error_never_a_silent_skip() {
        assert!(parse_record("r.json", "{not json").is_err());
    }

    #[test]
    fn an_lst_token_record_missing_its_line_is_an_error_never_a_silent_skip() {
        let text = r#"{"source":{"kind":"lst_token","path":"a.lst"},
                      "data":{"key":"K","raw_tokens":[{"key":"COST","value":"1"}]}}"#;
        assert!(parse_record("r.json", text).is_err());
    }

    #[test]
    fn field_zero_is_never_read_as_a_token() {
        // `Heavy Pick (Base).COPY=Pick, Heavy` is a record head, not a
        // `COPY=` token, and must not enter the closure.
        let tokens = tab_tokens("Heavy Pick (Base).COPY=Pick, Heavy\tKEY:Pick (Heavy)");
        assert_eq!(tokens, vec!["KEY:Pick (Heavy)"]);
    }

    /// `SD31-E6-F10-004`: real corpus reproduction, `inner_sea_gods/
    /// isg_equip.lst:220`, `Safecamp Wagon`'s `DESC:` field -- the LAST
    /// tab-delimited field on the row, followed by a single literal
    /// trailing space before the newline (`cat -A` on the real corpus
    /// file confirms `...fire resistance 5. $`, one space before the
    /// line-ending `$`). `tab_tokens` (this sweep's own corpus-side
    /// reader) correctly keeps that trailing space -- byte-equality means
    /// byte-equality, per `whitespace_is_not_normalised_away` above; the
    /// corpus's own literal is not this sweep's to normalize. The real
    /// defect was upstream, in `src/pcgen_import/lst_parser/equipment.rs`:
    /// `enrich_equipment_raw_tokens.rs` shipped a TRIMMED value into
    /// `raw_tokens` (via `EquipmentToken::value`, documented to strip
    /// surrounding whitespace for its OTHER callers) instead of the
    /// byte-exact field text `EquipmentToken::raw_pair` is documented to
    /// carry -- fixed there, not here. This test pins the sweep's own
    /// unchanged, correct behavior: the corpus-side closure entry for this
    /// exact real row keeps its trailing space, so a shipped value that
    /// ALSO keeps it (the fixed parser's own output) matches, and a
    /// shipped value that drops it (the pre-fix parser's output) does not.
    #[test]
    fn a_trailing_space_the_real_corpus_row_carries_is_kept_not_normalised_away() {
        let tokens = tab_tokens(
            "Safecamp Wagon\t\t\t\tTYPE:Magic.Wondrous Item.SLOT_None\t\tCOST:3000\tWT:2\t\tDESC:...fire resistance 5. ",
        );
        assert_eq!(
            tokens.last(),
            Some(&"DESC:...fire resistance 5. "),
            "the corpus row's own literal trailing space must survive `tab_tokens` unchanged"
        );

        let rec = record(&[("COST", "3000"), ("DESC", "...fire resistance 5. ")]);
        let rows = ["Safecamp Wagon\tCOST:3000\tDESC:...fire resistance 5. "];
        let mut tally = SweepTally::default();
        assert_eq!(
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &BTreeSet::new(), &mut tally),
            vec![],
            "a shipped value that preserves the corpus's own trailing space must match"
        );

        let trimmed_rec = record(&[("COST", "3000"), ("DESC", "...fire resistance 5.")]);
        let mut tally2 = SweepTally::default();
        assert_eq!(
            compare_tokens(
                &trimmed_rec,
                &closure_of(&rows, &trimmed_rec.identities),
                &BTreeSet::new(),
                &mut tally2
            ),
            vec![Finding::TokenNotInClosure {
                record: trimmed_rec.record_path.clone(),
                token: "DESC:...fire resistance 5.".to_string(),
            }],
            "a shipped value that DROPPED the corpus's own trailing space (the pre-fix \
             `enrich_equipment_raw_tokens.rs` behavior) must still be reported by this sweep -- \
             the fix belongs upstream in the parser, never in loosening this comparison"
        );
    }
}
