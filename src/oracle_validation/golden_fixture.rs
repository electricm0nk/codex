//! Golden-case fixture schema for the GE05-E2-F1 oracle-validation slice.
//!
//! This module defines the smallest typed shape that can represent the PF1 Core
//! Rulebook Human Fighter level 1 pilot case as an oracle-comparison fixture. It
//! binds old-system PCGen *runtime* evidence to an as-yet-unresolved Codex output
//! without claiming parity has passed.
//!
//! Deliberate non-goals (deferred to later GE-05 slices): no comparator, no
//! normalization rule engine, no parity-report writer, and no PCGen runner. This
//! module only loads/represents the fixture and reports claim-blocking diagnostics
//! when required evidence is missing. It mirrors the GE04
//! `rules_core::character_input` loader conventions: a simple `key=value` text
//! parser and structured, claim-blocking diagnostics.

/// A loaded golden-case fixture binding old-system oracle evidence to an
/// (unresolved) Codex output for one bounded comparison case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoldenCaseFixture {
    pub case_id: String,
    pub case_version: u32,
    pub scope: String,
    pub source_package: SourcePackage,
    /// Reference to (or compact summary of) the chosen character input. This slice
    /// stores a reference; it does not re-model the GE04 character-input record.
    pub character_input_ref: String,
    pub legacy_oracle: LegacyOracleEvidence,
    pub codex_output: CodexOutputEvidence,
    pub dimensions: Vec<ComparisonDimension>,
    /// Normalization declarations are references/placeholders only. No
    /// normalization behavior is implemented in this slice.
    pub normalization_refs: Vec<String>,
    pub known_gap_refs: Vec<String>,
    pub claim_target: ClaimTier,
    pub current_claim_status: ClaimTier,
    pub provisional_assumptions: Vec<ProvisionalAssumption>,
}

impl GoldenCaseFixture {
    /// Returns true only when the current claim status is `OracleChecked`. The
    /// fixture must be able to represent unresolved/blocked outputs without this
    /// returning true.
    pub fn parity_claimed(&self) -> bool {
        self.current_claim_status == ClaimTier::OracleChecked
    }

    /// Looks up a provisional assumption value by key.
    pub fn provisional_assumption(&self, key: &str) -> Option<&str> {
        self.provisional_assumptions
            .iter()
            .find(|assumption| assumption.key == key)
            .map(|assumption| assumption.value.as_str())
    }
}

/// Game system / source-package / campaign identity for the case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePackage {
    pub system: String,
    pub package: String,
    pub campaign: String,
    pub game_mode: String,
}

/// Old-system (PCGen) oracle evidence reference and capture metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyOracleEvidence {
    pub route: String,
    pub evidence_kind: OracleEvidenceKind,
    pub raw_output_ref: String,
    pub raw_output_retention: RawOutputRetention,
    pub raw_output_sha256: String,
    pub reduced_facts_ref: String,
}

/// Trust classification for the old-system evidence (TR-05-003). The pilot uses
/// `RuntimeBehaviorEvidence`; static source files alone must not satisfy parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OracleEvidenceKind {
    StaticSourceTruth,
    RuntimeBehaviorEvidence,
    GuiDerivedEvidence,
    Unknown,
}

/// Retention posture for the captured raw old-system output (TR-05-015).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawOutputRetention {
    LocalGeneratedOnly,
}

/// New-system (Codex) output evidence. For this slice it is unresolved or absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodexOutputEvidence {
    pub state: CodexOutputState,
    pub output_ref: Option<String>,
    pub diagnostics: Vec<String>,
}

/// Codex output state. There is intentionally no "resolved/passed" variant in
/// this slice: the comparator and capture adapter are out of scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodexOutputState {
    Unresolved,
    Absent,
}

/// A compared output dimension and its current (non-passing) status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonDimension {
    pub id: String,
    pub status: DimensionStatus,
}

/// Status of a comparison dimension. There is no `Passed` variant: passing
/// belongs to the GE05-E4 comparator, which is out of scope here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionStatus {
    Candidate,
    Blocked,
    NotYetGrounded,
}

/// A provisional, non-canonical assumption carried from the GE05-E1-F2 runtime
/// receipt. These must not be promoted into GE-06 pilot truth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvisionalAssumption {
    pub key: String,
    pub value: String,
}

/// Compatibility claim tier. Mirrors the quality-gate claim-tier model closely
/// enough for this slice. A case may target `OracleChecked` while its current
/// status remains `NotYetGrounded`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimTier {
    NotYetGrounded,
    Computed,
    OracleChecked,
}

/// Result of attempting to load a golden-case fixture. The fixture is produced
/// only when no claim-blocking diagnostics were raised.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureLoadResult {
    pub fixture: Option<GoldenCaseFixture>,
    pub diagnostics: Vec<FixtureDiagnostic>,
}

/// A structured diagnostic about the fixture, consistent with the GE04
/// character-input diagnostic shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureDiagnostic {
    pub class: DiagnosticClass,
    pub severity: DiagnosticSeverity,
    pub subject_ref: String,
    pub message: String,
    pub claim_blocking: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticClass {
    MissingFixtureField,
    InvalidFixtureField,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
}

#[derive(Default)]
struct ParsedFixture {
    case_id: Option<String>,
    case_version: Option<u32>,
    scope: Option<String>,
    source_system: Option<String>,
    source_package: Option<String>,
    source_campaign: Option<String>,
    source_game_mode: Option<String>,
    character_input_ref: Option<String>,
    legacy_route: Option<String>,
    legacy_evidence_kind: Option<OracleEvidenceKind>,
    legacy_raw_output_ref: Option<String>,
    legacy_raw_output_retention: Option<RawOutputRetention>,
    legacy_raw_output_sha256: Option<String>,
    legacy_reduced_facts_ref: Option<String>,
    codex_output_state: Option<CodexOutputState>,
    codex_output_ref: Option<String>,
    codex_diagnostics: Vec<String>,
    dimensions: Vec<ComparisonDimension>,
    normalization_refs: Vec<String>,
    known_gap_refs: Vec<String>,
    claim_target: Option<ClaimTier>,
    current_claim_status: Option<ClaimTier>,
    provisional_assumptions: Vec<ProvisionalAssumption>,
    diagnostics: Vec<FixtureDiagnostic>,
}

/// Parse a narrow text fixture into a [`GoldenCaseFixture`].
///
/// The format is a deliberately simple, pilot-local `key=value` form consistent
/// with the GE04 character-input fixture loader. Repeatable keys (`dimension`,
/// `provisional`, `normalization_ref`, `known_gap_ref`) may appear multiple times.
pub fn load_golden_case_fixture(input: &str) -> FixtureLoadResult {
    let mut parsed = ParsedFixture::default();

    for raw_line in input.lines() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            parsed.diagnostics.push(invalid(
                "fixture_line",
                format!("invalid golden-case fixture line missing '=': {raw_line}"),
            ));
            continue;
        };

        apply_field(key.trim(), value.trim(), &mut parsed);
    }

    parsed.add_required_field_diagnostics();

    if parsed.diagnostics.is_empty() {
        FixtureLoadResult {
            fixture: Some(parsed.into_fixture()),
            diagnostics: Vec::new(),
        }
    } else {
        FixtureLoadResult {
            fixture: None,
            diagnostics: parsed.diagnostics,
        }
    }
}

fn apply_field(key: &str, value: &str, parsed: &mut ParsedFixture) {
    match key {
        "case_id" => parsed.case_id = Some(value.to_owned()),
        "case_version" => apply_case_version(value, parsed),
        "scope" => parsed.scope = Some(value.to_owned()),
        "source_system" => parsed.source_system = Some(value.to_owned()),
        "source_package" => parsed.source_package = Some(value.to_owned()),
        "source_campaign" => parsed.source_campaign = Some(value.to_owned()),
        "source_game_mode" => parsed.source_game_mode = Some(value.to_owned()),
        "character_input_ref" => parsed.character_input_ref = Some(value.to_owned()),
        "legacy_route" => parsed.legacy_route = Some(value.to_owned()),
        "legacy_evidence_kind" => apply_evidence_kind(value, parsed),
        "legacy_raw_output_ref" => parsed.legacy_raw_output_ref = Some(value.to_owned()),
        "legacy_raw_output_retention" => apply_retention(value, parsed),
        "legacy_raw_output_sha256" => parsed.legacy_raw_output_sha256 = Some(value.to_owned()),
        "legacy_reduced_facts_ref" => parsed.legacy_reduced_facts_ref = Some(value.to_owned()),
        "codex_output_state" => apply_codex_state(value, parsed),
        "codex_output_ref" => parsed.codex_output_ref = Some(value.to_owned()),
        "codex_diagnostic" => parsed.codex_diagnostics.push(value.to_owned()),
        "dimension" => apply_dimension(value, parsed),
        "normalization_ref" => parsed.normalization_refs.push(value.to_owned()),
        "known_gap_ref" => parsed.known_gap_refs.push(value.to_owned()),
        "claim_target" => apply_claim_tier(value, parsed, ClaimTierField::Target),
        "current_claim_status" => apply_claim_tier(value, parsed, ClaimTierField::Current),
        "provisional" => apply_provisional(value, parsed),
        unknown => parsed.diagnostics.push(invalid(
            unknown,
            format!("invalid golden-case fixture field '{unknown}' is unsupported"),
        )),
    }
}

fn apply_case_version(value: &str, parsed: &mut ParsedFixture) {
    match value.parse::<u32>() {
        Ok(version) => parsed.case_version = Some(version),
        Err(_) => parsed.diagnostics.push(invalid(
            "case_version",
            format!("invalid golden-case fixture case_version '{value}' is not a number"),
        )),
    }
}

fn apply_evidence_kind(value: &str, parsed: &mut ParsedFixture) {
    let kind = match value {
        "static_source_truth" => OracleEvidenceKind::StaticSourceTruth,
        "runtime_behavior_evidence" => OracleEvidenceKind::RuntimeBehaviorEvidence,
        "gui_derived_evidence" => OracleEvidenceKind::GuiDerivedEvidence,
        "unknown" => OracleEvidenceKind::Unknown,
        other => {
            parsed.diagnostics.push(invalid(
                "legacy_evidence_kind",
                format!("invalid golden-case fixture legacy_evidence_kind '{other}' is unsupported"),
            ));
            return;
        }
    };
    parsed.legacy_evidence_kind = Some(kind);
}

fn apply_retention(value: &str, parsed: &mut ParsedFixture) {
    match value {
        "local_generated_only" => {
            parsed.legacy_raw_output_retention = Some(RawOutputRetention::LocalGeneratedOnly);
        }
        other => parsed.diagnostics.push(invalid(
            "legacy_raw_output_retention",
            format!(
                "invalid golden-case fixture legacy_raw_output_retention '{other}' is unsupported"
            ),
        )),
    }
}

fn apply_codex_state(value: &str, parsed: &mut ParsedFixture) {
    let state = match value {
        "unresolved" => CodexOutputState::Unresolved,
        "absent" => CodexOutputState::Absent,
        other => {
            parsed.diagnostics.push(invalid(
                "codex_output_state",
                format!("invalid golden-case fixture codex_output_state '{other}' is unsupported"),
            ));
            return;
        }
    };
    parsed.codex_output_state = Some(state);
}

fn apply_dimension(value: &str, parsed: &mut ParsedFixture) {
    let Some((id, status_text)) = value.rsplit_once(':') else {
        parsed.diagnostics.push(invalid(
            "dimensions",
            format!("invalid golden-case fixture dimension '{value}' is missing a status"),
        ));
        return;
    };

    let status = match status_text {
        "candidate" => DimensionStatus::Candidate,
        "blocked" => DimensionStatus::Blocked,
        "not_yet_grounded" => DimensionStatus::NotYetGrounded,
        other => {
            parsed.diagnostics.push(invalid(
                "dimensions",
                format!("invalid golden-case fixture dimension status '{other}' is unsupported"),
            ));
            return;
        }
    };

    parsed.dimensions.push(ComparisonDimension {
        id: id.to_owned(),
        status,
    });
}

enum ClaimTierField {
    Target,
    Current,
}

fn apply_claim_tier(value: &str, parsed: &mut ParsedFixture, field: ClaimTierField) {
    let (subject, message_field) = match field {
        ClaimTierField::Target => ("claim_target", "claim_target"),
        ClaimTierField::Current => ("current_claim_status", "current_claim_status"),
    };

    let tier = match value {
        "not_yet_grounded" => ClaimTier::NotYetGrounded,
        "computed" => ClaimTier::Computed,
        "oracle_checked" => ClaimTier::OracleChecked,
        other => {
            parsed.diagnostics.push(invalid(
                subject,
                format!("invalid golden-case fixture {message_field} '{other}' is unsupported"),
            ));
            return;
        }
    };

    match field {
        ClaimTierField::Target => parsed.claim_target = Some(tier),
        ClaimTierField::Current => parsed.current_claim_status = Some(tier),
    }
}

fn apply_provisional(value: &str, parsed: &mut ParsedFixture) {
    let Some((key, assumption_value)) = value.split_once(':') else {
        parsed.diagnostics.push(invalid(
            "provisional_assumptions",
            format!("invalid golden-case fixture provisional assumption '{value}' is missing a value"),
        ));
        return;
    };

    parsed.provisional_assumptions.push(ProvisionalAssumption {
        key: key.trim().to_owned(),
        value: assumption_value.trim().to_owned(),
    });
}

impl ParsedFixture {
    fn add_required_field_diagnostics(&mut self) {
        let required_strings = [
            ("case_id", self.case_id.as_deref()),
            ("scope", self.scope.as_deref()),
            ("source_system", self.source_system.as_deref()),
            ("source_package", self.source_package.as_deref()),
            ("source_campaign", self.source_campaign.as_deref()),
            ("source_game_mode", self.source_game_mode.as_deref()),
            ("character_input_ref", self.character_input_ref.as_deref()),
            ("legacy_route", self.legacy_route.as_deref()),
            ("legacy_raw_output_ref", self.legacy_raw_output_ref.as_deref()),
            (
                "legacy_raw_output_sha256",
                self.legacy_raw_output_sha256.as_deref(),
            ),
            (
                "legacy_reduced_facts_ref",
                self.legacy_reduced_facts_ref.as_deref(),
            ),
        ];

        for (field, value) in required_strings {
            if value.is_none_or(str::is_empty) {
                self.diagnostics.push(missing(
                    field,
                    format!("missing required golden-case fixture field '{field}'"),
                ));
            }
        }

        if self.case_version.is_none() {
            self.diagnostics.push(missing(
                "case_version",
                "missing required golden-case fixture field 'case_version'",
            ));
        }
        if self.legacy_evidence_kind.is_none() {
            self.diagnostics.push(missing(
                "legacy_evidence_kind",
                "missing required golden-case fixture field 'legacy_evidence_kind'",
            ));
        }
        if self.legacy_raw_output_retention.is_none() {
            self.diagnostics.push(missing(
                "legacy_raw_output_retention",
                "missing required golden-case fixture field 'legacy_raw_output_retention'",
            ));
        }
        if self.codex_output_state.is_none() {
            self.diagnostics.push(missing(
                "codex_output_state",
                "missing required golden-case fixture field 'codex_output_state'",
            ));
        }
        if self.claim_target.is_none() {
            self.diagnostics.push(missing(
                "claim_target",
                "missing required golden-case fixture field 'claim_target'",
            ));
        }
        if self.current_claim_status.is_none() {
            self.diagnostics.push(missing(
                "current_claim_status",
                "missing required golden-case fixture field 'current_claim_status'",
            ));
        }
    }

    /// Consumes the parsed fixture into a [`GoldenCaseFixture`].
    ///
    /// Only called once [`add_required_field_diagnostics`] has confirmed there are
    /// no diagnostics, so every required field is present.
    fn into_fixture(self) -> GoldenCaseFixture {
        GoldenCaseFixture {
            case_id: self.case_id.expect("validated case_id"),
            case_version: self.case_version.expect("validated case_version"),
            scope: self.scope.expect("validated scope"),
            source_package: SourcePackage {
                system: self.source_system.expect("validated source_system"),
                package: self.source_package.expect("validated source_package"),
                campaign: self.source_campaign.expect("validated source_campaign"),
                game_mode: self.source_game_mode.expect("validated source_game_mode"),
            },
            character_input_ref: self
                .character_input_ref
                .expect("validated character_input_ref"),
            legacy_oracle: LegacyOracleEvidence {
                route: self.legacy_route.expect("validated legacy_route"),
                evidence_kind: self
                    .legacy_evidence_kind
                    .expect("validated legacy_evidence_kind"),
                raw_output_ref: self
                    .legacy_raw_output_ref
                    .expect("validated legacy_raw_output_ref"),
                raw_output_retention: self
                    .legacy_raw_output_retention
                    .expect("validated legacy_raw_output_retention"),
                raw_output_sha256: self
                    .legacy_raw_output_sha256
                    .expect("validated legacy_raw_output_sha256"),
                reduced_facts_ref: self
                    .legacy_reduced_facts_ref
                    .expect("validated legacy_reduced_facts_ref"),
            },
            codex_output: CodexOutputEvidence {
                state: self.codex_output_state.expect("validated codex_output_state"),
                output_ref: self.codex_output_ref,
                diagnostics: self.codex_diagnostics,
            },
            dimensions: self.dimensions,
            normalization_refs: self.normalization_refs,
            known_gap_refs: self.known_gap_refs,
            claim_target: self.claim_target.expect("validated claim_target"),
            current_claim_status: self
                .current_claim_status
                .expect("validated current_claim_status"),
            provisional_assumptions: self.provisional_assumptions,
        }
    }
}

fn missing(subject_ref: impl Into<String>, message: impl Into<String>) -> FixtureDiagnostic {
    FixtureDiagnostic {
        class: DiagnosticClass::MissingFixtureField,
        severity: DiagnosticSeverity::Error,
        subject_ref: subject_ref.into(),
        message: message.into(),
        claim_blocking: true,
    }
}

fn invalid(subject_ref: impl Into<String>, message: impl Into<String>) -> FixtureDiagnostic {
    FixtureDiagnostic {
        class: DiagnosticClass::InvalidFixtureField,
        severity: DiagnosticSeverity::Error,
        subject_ref: subject_ref.into(),
        message: message.into(),
        claim_blocking: true,
    }
}
