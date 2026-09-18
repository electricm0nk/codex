//! GE08 desktop authoring workbench snapshot and command adapter.
//!
//! This module bridges the Tauri desktop shell and the headless GE08 authoring/preview
//! substrate, providing a bounded workbench snapshot contract for loading and previewing
//! the first proof package.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use codex::homebrew_authoring::package_manifest::PackageValidationState;
use codex::homebrew_authoring::package_store::PackageStore;
use codex::homebrew_authoring::preview_bridge::{ArmorClassPreview, PreviewBridge, PreviewStatus};

const PACKAGED_RESOURCE_PREFIX: &str = "resources/";
const LINUX_PRODUCT_RESOURCE_DIR_NAME: &str = "Codex";
const LINUX_DEB_RESOURCE_DIR_NAME: &str = "codex";
const LINUX_BINARY_RESOURCE_DIR_NAME: &str = "codex";

/// Request to load the GE08 authoring workbench snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoringWorkbenchRequest {
    /// Repo-root-relative path to the package bundle.
    pub package_root: String,
    /// Optional stable ID to focus a specific authored record.
    pub active_record_ref: Option<String>,
}

/// Package manifest information surfaced in the workbench snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageManifest {
    pub package_id: String,
    pub package_title: String,
    pub package_version: String,
    pub depends_on: Vec<String>,
    pub supported_object_kinds: Vec<String>,
}

/// Authored record (feat/effect/prerequisite) information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoredRecord {
    pub stable_id: String,
    pub owning_feat_id: Option<String>,
    pub display_name: String,
    pub object_kind: String,
    pub target_family: Option<String>,
    pub modifier_type: Option<String>,
    pub modifier_value: Option<i16>,
    pub predicate: Option<String>,
}

/// Selected slot resolution for the Human bonus feat substitution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedSlotResolution {
    pub slot: String,
    pub removed: String,
    pub added: String,
    pub resolved_feat_id: String,
}

/// Baseline armor class preview result (computed or blocked).
///
/// The `kind` tag stays PascalCase (`Computed` / `Blocked`): the TS boundary
/// (`loadAuthoringWorkbench.ts`) matches on those exact strings, and a
/// container-level `rename_all` would rename the variant tags, not the fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum BaselineArmorClass {
    Computed { value: i16 },
    Blocked { reason: String },
}

/// Package validation diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub class: String,
    pub severity: String, // "Error" or "Warning"
    pub message: String,
    pub subject_ref: String,
    pub claim_blocking: bool,
}

/// Provenance/source reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceRef {
    pub stable_id: String,
    pub source_package_id: String,
    pub authored_path: String,
}

/// Explanation graph node reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationRef {
    pub node_kind: String,
    pub ref_id: String,
    pub detail: String,
}

/// Oracle dimension status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleDimensionStatus {
    pub dimension: String,
    pub status: String,
}

/// Lifecycle gate state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleGateState {
    pub save_allowed: bool,
    pub preview_allowed: bool,
    pub export_allowed: bool,
    pub diff_mode: String,
}

/// Complete GE08 authoring workbench snapshot response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoringWorkbenchSnapshot {
    pub package_root: String,
    pub package_state: String, // "draft", "valid", "invalid", "deferred"
    pub package_manifest: PackageManifest,
    pub active_record_ref: Option<String>,
    pub authored_records: AuthoredRecords,
    pub preview: PreviewEnvelope,
    pub lifecycle_gate_state: LifecycleGateState,
    pub data_source: String, // "headless-preview-bridge" or "tauri-unavailable"
    pub note: String,
}

/// Authored records container.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoredRecords {
    pub feat: Option<AuthoredRecord>,
    pub effect: Option<AuthoredRecord>,
    pub prerequisite: Option<AuthoredRecord>,
}

/// Preview envelope with all result families.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewEnvelope {
    pub case_id: String,
    pub preview_status: String, // "success", "blocked", "unsupported"
    pub selected_slot_resolution: SelectedSlotResolution,
    pub baseline_armor_class: BaselineArmorClass,
    pub diagnostics: Vec<Diagnostic>,
    pub provenance_refs: Vec<ProvenanceRef>,
    pub explanation_refs: Vec<ExplanationRef>,
    pub oracle_dimension_status: Vec<OracleDimensionStatus>,
    pub blocked_claims: Vec<String>,
}

/// The Tauri-resolved resource directory for a packaged build, set once from
/// `main.rs`'s `.setup()` hook via [`set_app_resource_dir`]. A test binary or
/// a dev build that never calls that setter leaves this unset, and every
/// reader here treats "unset" exactly like every other absent candidate
/// source — never a panic, never assumed.
static APP_RESOURCE_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Records the real resource directory Tauri computed for this running app
/// (`app.path().resource_dir()`), so [`packaged_resource_candidates`] (and
/// therefore [`codex_repo_root`]) can consult the one candidate this process
/// actually knows to be correct, ahead of the exe-relative Linux layout
/// guesses and the `CODEX_DESKTOP_RESOURCE_DIR` env override below. Setting it
/// twice is a no-op — the first call wins, matching every other
/// `OnceLock`-cached value in this crate.
pub fn set_app_resource_dir(dir: PathBuf) {
    let _ = APP_RESOURCE_DIR.set(dir);
}

/// Resolve the codex repo root for repo-relative package paths.
///
/// Order of truth:
/// 1. The `CODEX_REPO_ROOT` environment variable (set by an operator or
///    launcher when the app runs outside a source checkout).
/// 2. The first [`packaged_resource_candidates`] candidate for `data/corpus`
///    that actually exists on disk, taken as evidence that its parent
///    directory is a real resource root (the packaged-app case: every
///    catalog loader that calls this function needs `data/corpus` under the
///    root it gets back, and this is the check that used to be missing).
/// 3. The compile-time `CARGO_MANIFEST_DIR` walk that works for dev builds
///    and tests. The compile-time path is baked at build time and does not
///    exist on tester machines running a published bundle, which is why the
///    two steps above exist.
pub fn codex_repo_root() -> Result<PathBuf, String> {
    if let Ok(root) = std::env::var("CODEX_REPO_ROOT") {
        return Ok(PathBuf::from(root));
    }

    if let Some(root) = first_candidate_root_carrying_corpus(&packaged_resource_root_candidates()) {
        return Ok(root);
    }

    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .ok_or_else(|| "cannot determine codex repo root from CARGO_MANIFEST_DIR".to_string())
}

/// The first candidate root, in priority order, whose `data/corpus`
/// subdirectory actually exists on disk — the selection rule behind step 2
/// of [`codex_repo_root`]'s doc comment, split out as a pure function of an
/// explicit candidate list.
///
/// Kept separate from [`packaged_resource_root_candidates`] (which reads
/// process-global environment variables and `std::env::current_exe()`) so it
/// is unit-testable without mutating that shared, process-wide state — state
/// every other test in this binary can also read concurrently. An earlier
/// version of this fix's own test set `CODEX_DESKTOP_RESOURCE_DIR` directly
/// and was flaky by construction: doing so mid-test-run made an unrelated,
/// concurrently-running fixture test resolve `codex_repo_root()` into this
/// test's own tempdir. Testing this pure function instead needs no such
/// mutation.
fn first_candidate_root_carrying_corpus(candidates: &[PathBuf]) -> Option<PathBuf> {
    candidates.iter().find(|root| root.join("data/corpus").is_dir()).cloned()
}

/// Resolve a requested package root: absolute paths pass through, repo-relative
/// paths anchor at the codex repo root, and packaged resource paths anchor at
/// the Tauri resource directory (with a source-tree fallback for tests/dev).
pub fn resolve_package_path(package_root: &str) -> Result<PathBuf, String> {
    let requested = PathBuf::from(package_root);

    if requested.is_absolute() {
        return Ok(requested);
    }

    if package_root.starts_with(PACKAGED_RESOURCE_PREFIX) {
        return resolve_packaged_resource_path(package_root);
    }

    Ok(codex_repo_root()?.join(requested))
}

fn resolve_packaged_resource_path(package_root: &str) -> Result<PathBuf, String> {
    let candidates = packaged_resource_candidates(package_root);
    if candidates.is_empty() {
        return Err("cannot determine packaged resource directory".to_string());
    }

    for candidate in &candidates {
        if candidate.exists() {
            return Ok(candidate.clone());
        }
    }

    // Return the first governed packaged-resource candidate so a genuine miss
    // reports a useful installed-resource path rather than falling back to the
    // source-only tests/fixtures location that caused the alpha tester defect.
    Ok(candidates[0].clone())
}

/// The candidate resource-root directories themselves, in priority order —
/// before any particular package path is joined on. Shared by
/// [`packaged_resource_candidates`] (which joins a package-relative path onto
/// each one) and [`codex_repo_root`] (which instead asks each root directly
/// whether it carries `data/corpus`, so it can hand back the root itself
/// rather than a path one level too deep).
fn packaged_resource_root_candidates() -> Vec<PathBuf> {
    let mut roots = Vec::new();

    if let Some(app_resource_dir) = APP_RESOURCE_DIR.get() {
        roots.push(app_resource_dir.clone());
    }

    if let Ok(resource_dir) = std::env::var("CODEX_DESKTOP_RESOURCE_DIR") {
        roots.push(PathBuf::from(resource_dir));
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            // Tauri Linux resource layout from an installed deb resolves from
            // `/usr/bin/<binary>` to `/usr/lib/<product name>`; debug/dev builds
            // may also copy resources beside the binary.
            roots.push(exe_dir.join(format!("../lib/{LINUX_PRODUCT_RESOURCE_DIR_NAME}")));
            roots.push(exe_dir.join(format!("../lib/{LINUX_DEB_RESOURCE_DIR_NAME}")));
            roots.push(exe_dir.join(format!("../lib/{LINUX_BINARY_RESOURCE_DIR_NAME}")));
            roots.push(exe_dir.to_path_buf());
            roots.push(exe_dir.join("resources"));
        }
    }

    if let Ok(appdir) = std::env::var("APPDIR") {
        let appdir = Path::new(&appdir);
        roots.push(appdir.join(format!("usr/lib/{LINUX_PRODUCT_RESOURCE_DIR_NAME}")));
        roots.push(appdir.join(format!("usr/lib/{LINUX_DEB_RESOURCE_DIR_NAME}")));
        roots.push(appdir.join(format!("usr/lib/{LINUX_BINARY_RESOURCE_DIR_NAME}")));
    }

    // Debian/install fallback documented by Tauri for Linux resource_dir.
    roots.push(PathBuf::from(format!(
        "/usr/lib/{LINUX_PRODUCT_RESOURCE_DIR_NAME}"
    )));
    roots.push(PathBuf::from(format!(
        "/usr/lib/{LINUX_DEB_RESOURCE_DIR_NAME}"
    )));
    roots.push(PathBuf::from(format!(
        "/usr/lib/{LINUX_BINARY_RESOURCE_DIR_NAME}"
    )));

    // Source-tree/test fallback: Tauri copies resources from src-tauri/resources
    // for bundled builds, while cargo unit tests run directly from src-tauri.
    roots.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")));

    roots
}

fn packaged_resource_candidates(package_root: &str) -> Vec<PathBuf> {
    packaged_resource_root_candidates()
        .into_iter()
        .map(|root| root.join(package_root))
        .collect()
}

/// Build the GE08 authoring workbench snapshot from the headless substrate.
///
/// This is the whole command behavior behind `load_authoring_workbench_snapshot`;
/// the Tauri command in `main.rs` is a thin wrapper so this mapping (including
/// the lifecycle gate derivation) stays testable without a webview.
pub fn build_authoring_workbench_snapshot(
    request: AuthoringWorkbenchRequest,
) -> Result<AuthoringWorkbenchSnapshot, String> {
    let package_path = resolve_package_path(&request.package_root)?;

    if !package_path.exists() {
        return Err(format!(
            "package root does not exist: {} (resolved to {})",
            request.package_root,
            package_path.display()
        ));
    }

    let envelope = PreviewBridge::preview_from_root(&package_path)
        .map_err(|e| format!("failed to load/preview package: {}", e))?;

    let package = PackageStore::load(&package_path)
        .map_err(|e| format!("failed to load package source: {}", e))?;

    let (actual_state, _diags) = package.recompute_validation();
    let baseline_ac = match envelope.baseline_armor_class {
        ArmorClassPreview::Computed(value) => BaselineArmorClass::Computed { value },
        ArmorClassPreview::Blocked(reason) => BaselineArmorClass::Blocked { reason },
    };

    let export_allowed = actual_state == PackageValidationState::Valid;
    let preview_allowed = export_allowed && envelope.preview_status != PreviewStatus::Blocked;

    Ok(AuthoringWorkbenchSnapshot {
        package_root: request.package_root,
        package_state: actual_state.as_str().to_string(),
        package_manifest: PackageManifest {
            package_id: package.manifest.package_id,
            package_title: package.manifest.package_title,
            package_version: package.manifest.package_version,
            depends_on: package.manifest.depends_on,
            supported_object_kinds: package.manifest.supported_object_kinds,
        },
        active_record_ref: request.active_record_ref,
        authored_records: AuthoredRecords {
            feat: package.feat.map(|f| AuthoredRecord {
                stable_id: f.stable_id,
                owning_feat_id: None,
                display_name: f.display_name,
                object_kind: f.object_kind,
                target_family: None,
                modifier_type: None,
                modifier_value: None,
                predicate: None,
            }),
            effect: package.effect.map(|e| AuthoredRecord {
                stable_id: e.stable_id,
                owning_feat_id: Some(e.owning_feat_id),
                display_name: e.target_family.clone(),
                object_kind: "effect".to_string(),
                target_family: Some(e.target_family),
                modifier_type: Some(e.modifier_type),
                modifier_value: Some(e.modifier_value),
                predicate: None,
            }),
            prerequisite: package.prerequisite.map(|p| AuthoredRecord {
                stable_id: p.stable_id,
                owning_feat_id: Some(p.owning_feat_id),
                display_name: p.predicate.clone(),
                object_kind: "prerequisite".to_string(),
                target_family: None,
                modifier_type: None,
                modifier_value: None,
                predicate: Some(p.predicate),
            }),
        },
        preview: PreviewEnvelope {
            case_id: envelope.case_id,
            preview_status: match envelope.preview_status {
                PreviewStatus::Success => "success".to_string(),
                PreviewStatus::Blocked => "blocked".to_string(),
                PreviewStatus::Unsupported => "unsupported".to_string(),
            },
            selected_slot_resolution: SelectedSlotResolution {
                slot: envelope.selected_slot_resolution.slot,
                removed: envelope.selected_slot_resolution.removed,
                added: envelope.selected_slot_resolution.added,
                resolved_feat_id: envelope.selected_slot_resolution.resolved_feat_id,
            },
            baseline_armor_class: baseline_ac,
            diagnostics: envelope
                .diagnostics
                .iter()
                .map(|d| Diagnostic {
                    class: d.class.clone(),
                    severity: d.severity.as_str().to_string(),
                    message: d.message.clone(),
                    subject_ref: d.subject_ref.clone(),
                    claim_blocking: d.claim_blocking,
                })
                .collect(),
            provenance_refs: envelope
                .provenance_refs
                .iter()
                .map(|p| ProvenanceRef {
                    stable_id: p.stable_id.clone(),
                    source_package_id: p.source_package_id.clone(),
                    authored_path: p.authored_path.clone(),
                })
                .collect(),
            explanation_refs: envelope
                .explanation_refs
                .iter()
                .map(|e| ExplanationRef {
                    node_kind: e.node_kind.clone(),
                    ref_id: e.ref_id.clone(),
                    detail: e.detail.clone(),
                })
                .collect(),
            oracle_dimension_status: envelope
                .oracle_dimension_status
                .iter()
                .map(|o| OracleDimensionStatus {
                    dimension: o.dimension.clone(),
                    status: o.status.clone(),
                })
                .collect(),
            blocked_claims: envelope.blocked_claims,
        },
        lifecycle_gate_state: LifecycleGateState {
            save_allowed: true,
            preview_allowed,
            export_allowed,
            diff_mode: "deferred".to_string(),
        },
        data_source: "headless-preview-bridge".to_string(),
        note: "Real GE-08 authoring workbench snapshot from headless substrate.".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The packaged-build fix this test pins: given a candidate root list
    /// like the one Tauri's own resolved resource directory would produce
    /// (via [`packaged_resource_root_candidates`]), `codex_repo_root()`'s
    /// selection rule picks the first candidate that actually carries
    /// `data/corpus/` on disk, and returns the root itself -- not one path
    /// segment too deep -- since every caller joins it with `data/corpus`
    /// again. Before this fix, nothing in `codex_repo_root()` ever asked a
    /// packaged resource candidate this question at all; it only ever
    /// consulted `CODEX_REPO_ROOT` or the compile-time `CARGO_MANIFEST_DIR`
    /// walk, which is exactly why a binary built or unpacked elsewhere
    /// (any packaged build) reproduced "No race could be read from the
    /// corpus.": every catalog loader's corpus root resolved to a directory
    /// with no `data/corpus` under it, and nothing pushed a diagnostic
    /// naming why.
    ///
    /// Tested here as a pure function of an explicit candidate list, not by
    /// mutating `CODEX_DESKTOP_RESOURCE_DIR` (the env var
    /// `packaged_resource_root_candidates` itself reads): that env var is
    /// process-global state every other test in this binary can also read
    /// concurrently, and an earlier version of this test mutated it directly
    /// -- proven flaky, not merely suspected, when doing so made an
    /// unrelated, concurrently-running fixture test below resolve
    /// `codex_repo_root()` into this test's own tempdir mid-run.
    #[test]
    fn first_candidate_root_carrying_corpus_picks_the_one_with_a_real_data_corpus_dir() {
        let tempdir = std::env::temp_dir().join(format!(
            "codex-desktop-authoring-workbench-test-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(tempdir.join("data/corpus/core_rulebook"))
            .expect("test fixture directory should be creatable");

        // A decoy candidate ahead of the real one in priority order, which
        // carries no `data/corpus` at all and must be skipped rather than
        // returned or treated as an error.
        let decoy = PathBuf::from("/definitely/not/a/real/codex-desktop-test-decoy-root");
        let candidates = vec![decoy, tempdir.clone()];

        let result = first_candidate_root_carrying_corpus(&candidates);

        let _ = std::fs::remove_dir_all(&tempdir);

        assert_eq!(
            result,
            Some(tempdir),
            "the first candidate that actually carries data/corpus must win, and win as itself \
             (not one path segment too deep), so callers can join it with data/corpus again"
        );
    }

    #[test]
    fn first_candidate_root_carrying_corpus_is_none_when_no_candidate_carries_it() {
        let candidates = vec![
            PathBuf::from("/definitely/not/a/real/codex-desktop-test-decoy-root-one"),
            PathBuf::from("/definitely/not/a/real/codex-desktop-test-decoy-root-two"),
        ];
        assert_eq!(first_candidate_root_carrying_corpus(&candidates), None);
    }

    fn snapshot_for(fixture: &str) -> AuthoringWorkbenchSnapshot {
        build_authoring_workbench_snapshot(AuthoringWorkbenchRequest {
            package_root: format!("tests/fixtures/authoring_workbench/{fixture}"),
            active_record_ref: None,
        })
        .unwrap_or_else(|err| panic!("fixture '{fixture}' should build a snapshot: {err}"))
    }

    #[test]
    fn valid_guard_stance_package_yields_success() {
        let snapshot = snapshot_for("guard-stance-package");

        assert_eq!(snapshot.package_state, "valid");
        assert_eq!(snapshot.preview.preview_status, "success");
        assert_eq!(
            snapshot.preview.baseline_armor_class,
            BaselineArmorClass::Computed { value: 17 }
        );
        assert_eq!(
            snapshot.preview.selected_slot_resolution.slot,
            "human_bonus_feat"
        );
        assert_eq!(
            snapshot.preview.selected_slot_resolution.resolved_feat_id,
            "feat.homebrew.guard_stance"
        );
        assert!(!snapshot.preview.provenance_refs.is_empty());
        assert!(!snapshot.preview.explanation_refs.is_empty());
        assert!(snapshot.lifecycle_gate_state.export_allowed);
        assert!(snapshot.lifecycle_gate_state.preview_allowed);
    }

    #[test]
    fn packaged_guard_stance_resource_yields_success_without_repo_fixture_path() {
        let snapshot = build_authoring_workbench_snapshot(AuthoringWorkbenchRequest {
            package_root: "resources/authoring_workbench/guard-stance-package".to_string(),
            active_record_ref: None,
        })
        .expect("packaged GE08 resource should build a snapshot without tests/fixtures");

        assert_eq!(snapshot.package_root, "resources/authoring_workbench/guard-stance-package");
        assert_eq!(snapshot.package_state, "valid");
        assert_eq!(snapshot.preview.preview_status, "success");
        assert_eq!(
            snapshot.preview.baseline_armor_class,
            BaselineArmorClass::Computed { value: 17 }
        );
    }

    #[test]
    fn missing_effect_yields_blocked_with_diagnostics() {
        let snapshot = snapshot_for("guard-stance-package-invalid-missing-effect");

        assert_eq!(snapshot.preview.preview_status, "blocked");
        assert!(
            snapshot
                .preview
                .diagnostics
                .iter()
                .any(|d| d.claim_blocking),
            "blocked preview must carry claim-blocking diagnostics"
        );
        assert!(
            matches!(
                snapshot.preview.baseline_armor_class,
                BaselineArmorClass::Blocked { .. }
            ),
            "blocked preview must not fabricate a computed armor class"
        );
        assert!(!snapshot.preview.blocked_claims.is_empty());
        assert!(!snapshot.lifecycle_gate_state.export_allowed);
    }

    #[test]
    fn widened_package_yields_unsupported() {
        let snapshot = snapshot_for("guard-stance-package-invalid-widened-preview");

        assert_eq!(snapshot.preview.preview_status, "unsupported");
        assert!(
            !snapshot.preview.diagnostics.is_empty(),
            "widened preview must preserve diagnostics"
        );
        assert!(!snapshot.lifecycle_gate_state.export_allowed);
    }

    #[test]
    fn lifecycle_gates_prevent_export_when_invalid() {
        for fixture in [
            "guard-stance-package-invalid-missing-effect",
            "guard-stance-package-invalid-widened-preview",
        ] {
            let snapshot = snapshot_for(fixture);
            assert_ne!(snapshot.package_state, "valid");
            assert!(
                !snapshot.lifecycle_gate_state.export_allowed,
                "invalid package '{fixture}' must refuse export"
            );
            assert!(
                !snapshot.lifecycle_gate_state.preview_allowed,
                "invalid package '{fixture}' must refuse preview gating"
            );
        }
    }

    #[test]
    fn baseline_armor_class_wire_shape_matches_the_ts_boundary() {
        // loadAuthoringWorkbench.ts matches kind === 'Computed' | 'Blocked';
        // this pins the serialized tag casing so the boundary cannot silently
        // fall through to the Blocked rendering branch for computed values.
        let computed = serde_json::to_value(BaselineArmorClass::Computed { value: 17 })
            .expect("computed AC should serialize");
        assert_eq!(
            computed,
            serde_json::json!({ "kind": "Computed", "value": 17 })
        );

        let blocked = serde_json::to_value(BaselineArmorClass::Blocked {
            reason: "missing effect".to_string(),
        })
        .expect("blocked AC should serialize");
        assert_eq!(
            blocked,
            serde_json::json!({ "kind": "Blocked", "reason": "missing effect" })
        );
    }
}
