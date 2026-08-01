//! Rust-side PCGen runner wrapper (SD-26 Epic 2, Criterion 2.4).
//!
//! Wraps the two real scripts SD-25's Epic 4 PCGen-runner scaffolding
//! already ships into one Rust call:
//!
//! - `scripts/pcgen-run-character.sh` — drives the real PCGen Gradle wrapper
//!   in headless batch-export mode against a real `.pcg` character file,
//!   producing raw XML.
//! - `scripts/pcgen-normalize-output.py` — normalizes that raw XML into the
//!   typed `case_id`/`dimensions`/`diagnostics` JSON shape consumed by
//!   `src/oracle_validation/{golden_fixture,selected_parity_dimensions}.rs`.
//!
//! This module does not reimplement either script's logic; it shells out to
//! both real scripts in sequence (runner, then normalizer) and parses their
//! composed real output into typed Rust values, surfacing real failures
//! (non-zero exit, spawn failure, malformed output) as typed
//! [`PcgenRunnerError`] variants rather than swallowing them. No PCGen output
//! is mocked, stubbed, or fabricated here.
//!
//! `scripts/pcgen-normalize-output.py` already reduces raw PCGen XML into
//! typed (`value_string` XOR `value_i16`) dimension values — the same shape
//! [`crate::oracle_validation::comparator::NormalizedDimensionValue`] uses —
//! so [`PcgenRunOutput::to_normalized_output`] is a direct field-for-field
//! carry into Criterion 2.1's comparator input, not a second normalization
//! pass. Criterion 2.2's `normalization.rs` rule engine remains available
//! separately for raw, not-yet-typed text captures that do not come through
//! this script pair.
//!
//! Concurrency: serial (per `epic-breakdown.md` Criterion 2.4).

use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

use crate::oracle_validation::comparator::{NormalizedDimensionValue, NormalizedOutput};

/// Default PCGen repo checkout location relative to `$HOME`, mirroring
/// `scripts/pcgen-run-character.sh`'s own `DEFAULT_PCGEN_DIR`.
///
/// Kept HOME-relative on purpose: the operator keeps `workspace/` in the
/// home directory and syncs it across machines, so `$HOME/workspace/...`
/// is correct on every box while an absolute `/home/<someone>/...` is
/// correct on exactly one.
const DEFAULT_PCGEN_REPO_DIR_REL: &str = "workspace/repos/pcgen";
const RUNNER_SCRIPT_REL: &str = "scripts/pcgen-run-character.sh";
const NORMALIZER_SCRIPT_REL: &str = "scripts/pcgen-normalize-output.py";

/// Resolves this codex checkout's root, for locating the two scripts this
/// module wraps. `CODEX_REPO_ROOT` (an operator/launcher override) wins when
/// set; otherwise the compile-time `CARGO_MANIFEST_DIR` (this crate's own
/// root, which is the repo root) is used — the same order-of-truth
/// `apps/desktop/src-tauri/src/ge08_workbench.rs::codex_repo_root` already
/// establishes elsewhere in this repo.
fn codex_repo_root() -> PathBuf {
    std::env::var("CODEX_REPO_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

/// Resolves the PCGen repo checkout this wrapper invokes against.
/// `PCGEN_REPO_DIR` wins when set (matching
/// `scripts/pcgen-run-character.sh`'s own `-w`/`$PCGEN_REPO_DIR` contract);
/// otherwise `$HOME/workspace/repos/pcgen`, the same HOME-relative default
/// that script falls back to.
fn default_pcgen_repo_dir() -> PathBuf {
    if let Ok(configured) = std::env::var("PCGEN_REPO_DIR") {
        return PathBuf::from(configured);
    }
    let home = std::env::var("HOME")
        .expect("HOME must be set to locate the default PCGen repo checkout");
    PathBuf::from(home).join(DEFAULT_PCGEN_REPO_DIR_REL)
}

/// Resolves the absolute path to `scripts/pcgen-run-character.sh` in this
/// checkout.
pub fn default_runner_script() -> PathBuf {
    codex_repo_root().join(RUNNER_SCRIPT_REL)
}

/// Resolves the absolute path to `scripts/pcgen-normalize-output.py` in this
/// checkout.
pub fn default_normalizer_script() -> PathBuf {
    codex_repo_root().join(NORMALIZER_SCRIPT_REL)
}

/// Options for one [`run_pcgen_character`] invocation.
#[derive(Debug, Clone)]
pub struct PcgenRunOptions {
    /// PCGen repo root (must contain an executable `./gradlew`). Passed to
    /// `scripts/pcgen-run-character.sh -w`.
    pub pcgen_repo_dir: PathBuf,
    /// Golden-case fixture `case_id` to stamp on the normalized output.
    pub case_id: String,
    /// Source package id to stamp on each normalized dimension.
    pub source_package_id: String,
    /// Legacy-oracle route description to record on the normalized output.
    pub legacy_route: String,
}

impl PcgenRunOptions {
    /// Builds options for `case_id`/`source_package_id`/`legacy_route`,
    /// resolving `pcgen_repo_dir` from `$PCGEN_REPO_DIR` or the runner
    /// script's own documented default.
    pub fn new(
        case_id: impl Into<String>,
        source_package_id: impl Into<String>,
        legacy_route: impl Into<String>,
    ) -> Self {
        Self {
            pcgen_repo_dir: default_pcgen_repo_dir(),
            case_id: case_id.into(),
            source_package_id: source_package_id.into(),
            legacy_route: legacy_route.into(),
        }
    }
}

/// One normalized dimension value, mirroring
/// `scripts/pcgen-normalize-output.py`'s `SelectedDimension.to_json()` shape
/// field for field.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PcgenDimensionOutput {
    pub id: String,
    pub value_string: Option<String>,
    pub value_i16: Option<i16>,
    pub source_package_id: String,
}

/// Full parsed result of one PCGen-runner + normalizer pipeline invocation,
/// mirroring `scripts/pcgen-normalize-output.py`'s
/// `NormalizationResult.to_json()` shape field for field.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PcgenRunOutput {
    pub case_id: String,
    pub source_package_id: String,
    pub legacy_route: String,
    pub claim_tier_floor: String,
    pub dimensions: Vec<PcgenDimensionOutput>,
    pub diagnostics: Vec<String>,
}

impl PcgenRunOutput {
    /// Carries this run's already-typed dimensions directly into Criterion
    /// 2.1's comparator input shape (see this module's doc comment for why
    /// this is a direct carry, not a second normalization pass).
    pub fn to_normalized_output(&self) -> NormalizedOutput {
        NormalizedOutput {
            dim_values: self
                .dimensions
                .iter()
                .map(|d| NormalizedDimensionValue {
                    id: d.id.clone(),
                    value_string: d.value_string.clone(),
                    value_i16: d.value_i16,
                })
                .collect(),
        }
    }
}

/// Errors from the PCGen runner + normalizer wrapper. Every variant carries
/// enough of the real underlying script's real failure (exit status,
/// stderr) to debug without re-running by hand.
#[derive(Debug)]
pub enum PcgenRunnerError {
    /// The requested `.pcg` character file does not exist.
    CharacterFileNotFound(PathBuf),
    /// `scripts/pcgen-run-character.sh` is missing at the resolved path.
    RunnerScriptNotFound(PathBuf),
    /// `scripts/pcgen-normalize-output.py` is missing at the resolved path.
    NormalizerScriptNotFound(PathBuf),
    /// A scratch directory for the run's intermediate XML/JSON could not be
    /// created.
    ScratchDirUnavailable(String),
    /// The OS failed to spawn the runner script itself (not a
    /// script-internal failure).
    RunnerSpawnFailed(String),
    /// The runner script ran but exited non-zero.
    RunnerFailed {
        status: Option<i32>,
        stderr: String,
    },
    /// The OS failed to spawn `python3` for the normalizer.
    NormalizerSpawnFailed(String),
    /// The normalizer ran but exited non-zero.
    NormalizerFailed {
        status: Option<i32>,
        stderr: String,
    },
    /// The normalizer's output file could not be read back.
    OutputUnreadable(String),
    /// The normalizer's output was not parseable as the expected JSON shape.
    OutputParseError(String),
}

impl std::fmt::Display for PcgenRunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CharacterFileNotFound(p) => {
                write!(f, "PCGen character file not found: {}", p.display())
            }
            Self::RunnerScriptNotFound(p) => {
                write!(f, "pcgen-run-character.sh not found at {}", p.display())
            }
            Self::NormalizerScriptNotFound(p) => {
                write!(f, "pcgen-normalize-output.py not found at {}", p.display())
            }
            Self::ScratchDirUnavailable(e) => write!(f, "could not create scratch directory: {e}"),
            Self::RunnerSpawnFailed(e) => write!(f, "failed to spawn pcgen-run-character.sh: {e}"),
            Self::RunnerFailed { status, stderr } => write!(
                f,
                "pcgen-run-character.sh failed (exit status {status:?}): {stderr}"
            ),
            Self::NormalizerSpawnFailed(e) => {
                write!(f, "failed to spawn pcgen-normalize-output.py (python3): {e}")
            }
            Self::NormalizerFailed { status, stderr } => write!(
                f,
                "pcgen-normalize-output.py failed (exit status {status:?}): {stderr}"
            ),
            Self::OutputUnreadable(e) => {
                write!(f, "could not read pcgen-normalize-output.py's output: {e}")
            }
            Self::OutputParseError(e) => write!(
                f,
                "failed to parse pcgen-normalize-output.py output as JSON: {e}"
            ),
        }
    }
}

impl std::error::Error for PcgenRunnerError {}

/// Parses `scripts/pcgen-normalize-output.py`'s real JSON output shape into
/// [`PcgenRunOutput`]. Pure and deterministic (no process spawn, no
/// filesystem access) so the parsing/error-mapping contract is unit-testable
/// independent of a real PCGen invocation.
pub fn parse_normalized_output(json_text: &str) -> Result<PcgenRunOutput, PcgenRunnerError> {
    serde_json::from_str(json_text).map_err(|e| PcgenRunnerError::OutputParseError(e.to_string()))
}

/// Runs a real PCGen character file through the real PCGen engine
/// (`scripts/pcgen-run-character.sh`, which drives PCGen's real Gradle
/// wrapper) and normalizes the result (`scripts/pcgen-normalize-output.py`)
/// into typed Rust values — a serial, end-to-end wrapper around both real
/// scripts. No PCGen output is mocked, stubbed, or fabricated: every call
/// shells out to the real scripts and surfaces their real result.
pub fn run_pcgen_character(
    character_pcg: &Path,
    options: &PcgenRunOptions,
) -> Result<PcgenRunOutput, PcgenRunnerError> {
    if !character_pcg.is_file() {
        return Err(PcgenRunnerError::CharacterFileNotFound(
            character_pcg.to_path_buf(),
        ));
    }

    let runner_script = default_runner_script();
    if !runner_script.is_file() {
        return Err(PcgenRunnerError::RunnerScriptNotFound(runner_script));
    }

    let normalizer_script = default_normalizer_script();
    if !normalizer_script.is_file() {
        return Err(PcgenRunnerError::NormalizerScriptNotFound(normalizer_script));
    }

    let scratch_dir = std::env::temp_dir().join(format!(
        "codex-pcgen-runner-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    std::fs::create_dir_all(&scratch_dir)
        .map_err(|e| PcgenRunnerError::ScratchDirUnavailable(e.to_string()))?;

    let xml_output = scratch_dir.join(
        character_pcg
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "pcgen_run".to_string())
            + ".xml",
    );

    let runner_output = Command::new(&runner_script)
        .arg("-c")
        .arg(character_pcg)
        .arg("-o")
        .arg(&xml_output)
        .arg("-w")
        .arg(&options.pcgen_repo_dir)
        .output()
        .map_err(|e| PcgenRunnerError::RunnerSpawnFailed(e.to_string()))?;

    if !runner_output.status.success() {
        let _ = std::fs::remove_dir_all(&scratch_dir);
        return Err(PcgenRunnerError::RunnerFailed {
            status: runner_output.status.code(),
            stderr: String::from_utf8_lossy(&runner_output.stderr).into_owned(),
        });
    }

    let normalized_json_path = scratch_dir.join("normalized.json");
    let normalizer_output = Command::new("python3")
        .arg(&normalizer_script)
        .arg(&xml_output)
        .arg("-o")
        .arg(&normalized_json_path)
        .arg("--case-id")
        .arg(&options.case_id)
        .arg("--source-package-id")
        .arg(&options.source_package_id)
        .arg("--legacy-route")
        .arg(&options.legacy_route)
        .output()
        .map_err(|e| PcgenRunnerError::NormalizerSpawnFailed(e.to_string()))?;

    if !normalizer_output.status.success() {
        let _ = std::fs::remove_dir_all(&scratch_dir);
        return Err(PcgenRunnerError::NormalizerFailed {
            status: normalizer_output.status.code(),
            stderr: String::from_utf8_lossy(&normalizer_output.stderr).into_owned(),
        });
    }

    let normalized_text = std::fs::read_to_string(&normalized_json_path)
        .map_err(|e| PcgenRunnerError::OutputUnreadable(e.to_string()));

    let _ = std::fs::remove_dir_all(&scratch_dir);

    parse_normalized_output(&normalized_text?)
}
