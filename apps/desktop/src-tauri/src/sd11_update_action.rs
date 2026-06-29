use serde::{Deserialize, Serialize};

/// Running-build identity handed to the SD-11 update-action command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sd11UpdateActionRequest {
    pub build_version: String,
    pub build_label: String,
    pub platform_label: String,
    pub tester_channel_label: String,
}

/// Release truth returned to the desktop consumer.
///
/// This runtime boundary is read-only and never authors release truth. A locally built desktop
/// binary carries no embedded governed SD-12 release manifest, checksum, or provenance handle, so
/// per SD-12 it is not an official tester release unit. The only honest answer for such a build is
/// `no-official-release`; this command never counterfeits `up-to-date`. The `governed-release`
/// variant is intentionally absent here because no governed manifest producer exists in repo truth
/// yet — when one lands, it (not this local runtime) is what would emit a governed manifest.
#[derive(Debug, Serialize, PartialEq)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Sd11UpdateReleaseTruth {
    #[serde(rename_all = "camelCase")]
    NoOfficialRelease {
        reason: String,
        build_label: String,
        version: String,
    },
    // Reserved wire-contract variant. The TypeScript boundary emits `check-failed` when the runtime
    // is unavailable or the command itself fails; this runtime path never fabricates one, so it is
    // intentionally not constructed here.
    #[allow(dead_code)]
    #[serde(rename_all = "camelCase")]
    CheckFailed {
        reason: String,
        build_label: String,
        version: String,
    },
}

/// Resolve the honest SD-11 update truth for the running build.
pub fn resolve_sd11_update_action(request: Sd11UpdateActionRequest) -> Sd11UpdateReleaseTruth {
    Sd11UpdateReleaseTruth::NoOfficialRelease {
        reason: format!(
            "No governed SD-12 release manifest, checksum, or provenance handle is embedded in this \
             {} desktop build on the {} channel. Official update truth comes only from governed \
             GitHub-backed release units, so this local/non-governed build has no official release to \
             check against.",
            request.platform_label, request.tester_channel_label
        ),
        build_label: request.build_label,
        version: request.build_version,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> Sd11UpdateActionRequest {
        Sd11UpdateActionRequest {
            build_version: "0.0.0".to_string(),
            build_label: "codex-desktop-shell-scaffold@0.0.0".to_string(),
            platform_label: "Linux".to_string(),
            tester_channel_label: "alpha".to_string(),
        }
    }

    #[test]
    fn local_build_resolves_to_no_official_release() {
        let truth = resolve_sd11_update_action(request());
        match truth {
            Sd11UpdateReleaseTruth::NoOfficialRelease {
                version, build_label, reason,
            } => {
                assert_eq!(version, "0.0.0");
                assert_eq!(build_label, "codex-desktop-shell-scaffold@0.0.0");
                assert!(reason.contains("governed"), "reason must explain non-governed truth: {reason}");
            }
            other => panic!("expected NoOfficialRelease, got {other:?}"),
        }
    }

    #[test]
    fn no_official_release_serializes_with_kebab_kind_and_camel_fields() {
        let truth = resolve_sd11_update_action(request());
        let json = serde_json::to_string(&truth).expect("serialize");
        assert!(json.contains("\"kind\":\"no-official-release\""), "kind tag: {json}");
        assert!(json.contains("\"buildLabel\":"), "camelCase buildLabel: {json}");
        assert!(json.contains("\"version\":\"0.0.0\""), "version: {json}");
    }
}
