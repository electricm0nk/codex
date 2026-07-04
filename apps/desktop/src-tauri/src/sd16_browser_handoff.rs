//! SD-16 E5-F3b — browser-open handoff seam.
//!
//! This module owns the Rust/Tauri half of the defect-submission "browser-open"
//! boundary (AV-PAY-1 browser-open half, AV-PAY-7 URL-half). It builds a
//! prefilled GitHub "new issue" URL from a shell-supplied request, re-validates
//! the URL as defense-in-depth, and returns it to the React layer.
//!
//! ## Two honest deviations from the F3b handoff (documented in the PR body)
//!
//! 1. **F3a not shipped.** The handoff imports `IssueUrlRequest`,
//!    `IssueUrlError`, `build_github_issue_url`, and `validate_github_issues_url`
//!    from `crate::issue_payload` (F3a). That module does NOT exist on
//!    `origin/develop`, and F3b's write scope forbids creating it. F3b therefore
//!    inlines the minimum URL builder + validator + types here. F3a remains a
//!    sibling slice that may later replace these inlined symbols.
//! 2. **Tauri 2 shell::open.** The handoff calls `tauri::api::shell::open`,
//!    which Tauri 2 moved into the separate `tauri-plugin-shell` crate (not in
//!    the dep tree; adding it requires a forbidden `Cargo.toml` edit). F3b's
//!    command instead performs the build + validate and returns
//!    `IssueUrlResponse { url, opened: true }` once the URL is validated —
//!    `opened` here is the "URL built and validated, ready for the OS to open"
//!    readiness signal. The actual OS shell open is the F4 wiring slice's job,
//!    which will replace this boolean with the real `tauri-plugin-shell` result
//!    and emit the TS `BROWSER_OPENED` event only after the OS confirms.
//!
//! The unit-testable seam is the pure `prepare_url` helper (build + validate,
//! no OS interaction), exercised by the inline tests below.

use serde::{Deserialize, Serialize};

/// Upper bound on the generated GitHub issue URL length (AV-PAY-6 boundary).
pub const MAX_URL_LENGTH: usize = 8192;

/// GitHub username rule: 1-39 chars.
const OWNER_MAX_LEN: usize = 39;
/// Repository name upper bound.
const REPO_MAX_LEN: usize = 100;
/// Issue title upper bound.
const TITLE_MAX_LEN: usize = 256;
/// Per-label length upper bound.
const LABEL_MAX_LEN: usize = 50;
/// Maximum number of labels.
const LABELS_MAX: usize = 100;

/// Request ferried from the shell: the pieces needed to build a prefilled
/// GitHub "new issue" URL. The diagnostic payload is sanitized upstream (TS
/// redaction layer) before it reaches this command; this seam trusts the input
/// and only shape-validates.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueUrlRequest {
    pub owner: String,
    pub repo: String,
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub labels: Vec<String>,
}

/// Response surfaced to the React layer via IPC. `opened` is the F3b
/// browser-open-readiness signal: `true` once the URL is built AND re-validated.
/// The F4 wiring slice replaces it with the actual OS shell-open result.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IssueUrlResponse {
    pub url: String,
    pub opened: bool,
}

/// Failure modes of the browser-open handoff. Internally tagged so the React
/// layer can switch on `kind` (kebab-case) without positional parsing.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum IssueUrlError {
    InvalidOwner,
    InvalidRepo,
    EmptyTitle,
    TitleTooLong { max: usize },
    TooManyLabels { max: usize },
    LabelTooLong { max: usize },
    BodyTooLong { max: usize },
    UrlValidation { reason: String },
}

/// Minimal RFC 3986 percent-encoder: every byte outside the unreserved set
/// (ALPHA / DIGIT / `-` / `.` / `_` / `~`) is emitted as `%XX`. Operates on
/// raw bytes so multi-byte UTF-8 sequences are encoded byte-by-byte, keeping
/// the output ASCII-safe. (F3a owns the `percent-encoding` crate; F3b adds no
/// dependency and inlines this narrow encoder instead.)
fn percent_encode(input: &str) -> String {
    const HEX: &[u8] = b"0123456789ABCDEF";
    let mut output = String::with_capacity(input.len() * 3);
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                output.push(byte as char);
            }
            _ => {
                output.push('%');
                output.push(HEX[(byte >> 4) as usize] as char);
                output.push(HEX[(byte & 0xf) as usize] as char);
            }
        }
    }
    output
}

/// GitHub owner rule: non-empty, <= 39 chars, ASCII alphanumeric plus `-`/`_`.
fn is_valid_owner(owner: &str) -> bool {
    !owner.is_empty()
        && owner.len() <= OWNER_MAX_LEN
        && owner
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Repository rule: non-empty, <= 100 chars, ASCII alphanumeric plus `-`/`_`/`.`.
fn is_valid_repo(repo: &str) -> bool {
    !repo.is_empty()
        && repo.len() <= REPO_MAX_LEN
        && repo
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

/// Build the prefilled GitHub issue URL after shape-validating each field.
/// Owner/repo are placed into the path unencoded because they are validated to
/// a URL-path-safe character set; the free-text title/body/labels are
/// percent-encoded into the query string.
pub fn build_github_issue_url(req: &IssueUrlRequest) -> Result<String, IssueUrlError> {
    if !is_valid_owner(&req.owner) {
        return Err(IssueUrlError::InvalidOwner);
    }
    if !is_valid_repo(&req.repo) {
        return Err(IssueUrlError::InvalidRepo);
    }
    if req.title.is_empty() {
        return Err(IssueUrlError::EmptyTitle);
    }
    if req.title.chars().count() > TITLE_MAX_LEN {
        return Err(IssueUrlError::TitleTooLong { max: TITLE_MAX_LEN });
    }
    if req.body.len() > MAX_URL_LENGTH {
        return Err(IssueUrlError::BodyTooLong { max: MAX_URL_LENGTH });
    }
    if req.labels.len() > LABELS_MAX {
        return Err(IssueUrlError::TooManyLabels { max: LABELS_MAX });
    }
    for label in &req.labels {
        if label.chars().count() > LABEL_MAX_LEN {
            return Err(IssueUrlError::LabelTooLong { max: LABEL_MAX_LEN });
        }
    }

    let mut url = format!(
        "https://github.com/{}/{}/issues/new?title={}&body={}",
        req.owner,
        req.repo,
        percent_encode(&req.title),
        percent_encode(&req.body),
    );
    if !req.labels.is_empty() {
        url.push_str("&labels=");
        url.push_str(&percent_encode(&req.labels.join(",")));
    }
    Ok(url)
}

/// Re-validate a built URL as defense-in-depth: it must be an
/// `https://github.com/<owner>/<repo>/issues/new` URL and stay within
/// `MAX_URL_LENGTH`. Only the path is checked; the query string is ignored.
pub fn validate_github_issues_url(url: &str) -> Result<(), IssueUrlError> {
    const PREFIX: &str = "https://github.com/";

    if url.len() > MAX_URL_LENGTH {
        return Err(IssueUrlError::UrlValidation {
            reason: format!(
                "url length {} exceeds MAX_URL_LENGTH {MAX_URL_LENGTH}",
                url.len()
            ),
        });
    }

    let rest = match url.strip_prefix(PREFIX) {
        Some(rest) => rest,
        None => {
            return Err(IssueUrlError::UrlValidation {
                reason: "url must start with https://github.com/".to_string(),
            })
        }
    };

    let path = rest.split(['?', '#']).next().unwrap_or("");
    let segments: Vec<&str> = path.split('/').collect();
    let well_formed = segments.len() == 4
        && !segments[0].is_empty()
        && !segments[1].is_empty()
        && segments[2] == "issues"
        && segments[3] == "new";
    if !well_formed {
        return Err(IssueUrlError::UrlValidation {
            reason: format!("path must be /<owner>/<repo>/issues/new, got /{path}"),
        });
    }

    Ok(())
}

/// Pure, OS-free seam: build then re-validate the issue URL. This is the
/// unit-testable core of the command (the F3b handoff's god-emperor defensive
/// note names this helper `prepare_url`). The Tauri command is a thin wrapper.
fn prepare_url(req: &IssueUrlRequest) -> Result<String, IssueUrlError> {
    let url = build_github_issue_url(req)?;
    validate_github_issues_url(&url)?;
    Ok(url)
}

/// Tauri command consumed by the shell when the user clicks "Report this
/// defect". Builds + re-validates the GitHub issue URL and returns it with the
/// `opened: true` readiness signal. F3b explicitly refuses to claim submission
/// without proof: the actual OS browser open (and the TS `BROWSER_OPENED`
/// event that advances the reducer to `confirmed`) is the F4 wiring slice's
/// responsibility.
#[tauri::command]
pub fn sd16_browser_handoff(req: IssueUrlRequest) -> Result<IssueUrlResponse, IssueUrlError> {
    let url = prepare_url(&req)?;
    Ok(IssueUrlResponse { url, opened: true })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_request() -> IssueUrlRequest {
        IssueUrlRequest {
            owner: "electricm0nk".to_string(),
            repo: "codex".to_string(),
            title: "Shell defect: crash on open".to_string(),
            body: "version=2.5.0".to_string(),
            labels: vec!["codex-shell-defect".to_string(), "tranche-2.5".to_string()],
        }
    }

    #[test]
    fn prepare_url_returns_well_formed_url_for_valid_request() {
        let url = prepare_url(&valid_request()).expect("valid request must build");
        assert!(
            url.starts_with("https://github.com/electricm0nk/codex/issues/new?"),
            "url shape: {url}"
        );
        assert!(url.contains("title=Shell%20defect%3A%20crash%20on%20open"), "title enc: {url}");
        assert!(url.contains("body=version%3D2.5.0"), "body enc: {url}");
        assert!(url.contains("labels=codex-shell-defect%2Ctranche-2.5"), "labels enc: {url}");
        assert!(validate_github_issues_url(&url).is_ok(), "built url must validate: {url}");
    }

    #[test]
    fn prepare_url_returns_error_for_invalid_owner() {
        let mut req = valid_request();
        req.owner = "..".to_string();
        assert_eq!(prepare_url(&req), Err(IssueUrlError::InvalidOwner));
    }

    #[test]
    fn prepare_url_returns_error_for_empty_title() {
        let mut req = valid_request();
        req.title = String::new();
        assert_eq!(prepare_url(&req), Err(IssueUrlError::EmptyTitle));
    }

    #[test]
    fn prepare_url_returns_error_for_body_too_long() {
        let mut req = valid_request();
        req.body = "x".repeat(MAX_URL_LENGTH + 1);
        assert_eq!(
            prepare_url(&req),
            Err(IssueUrlError::BodyTooLong { max: MAX_URL_LENGTH })
        );
    }

    #[test]
    fn validate_url_accepts_well_formed_github_url() {
        let url = "https://github.com/electricm0nk/codex/issues/new?title=x&body=y";
        assert!(validate_github_issues_url(url).is_ok());
    }

    #[test]
    fn validate_url_rejects_non_https() {
        let url = "http://github.com/electricm0nk/codex/issues/new";
        assert!(matches!(
            validate_github_issues_url(url),
            Err(IssueUrlError::UrlValidation { .. })
        ));
    }

    #[test]
    fn validate_url_rejects_non_github_host() {
        let url = "https://gitlab.com/electricm0nk/codex/issues/new";
        assert!(matches!(
            validate_github_issues_url(url),
            Err(IssueUrlError::UrlValidation { .. })
        ));
    }

    #[test]
    fn validate_url_rejects_wrong_path() {
        let url = "https://github.com/electricm0nk/codex/pulls/new";
        assert!(matches!(
            validate_github_issues_url(url),
            Err(IssueUrlError::UrlValidation { .. })
        ));
    }

    #[test]
    fn handoff_command_returns_opened_true_after_validate_passes() {
        let res = sd16_browser_handoff(valid_request()).expect("valid request");
        assert!(res.opened, "opened must be true once url built+validated");
        assert!(res.url.starts_with("https://github.com/electricm0nk/codex/issues/new?"));
    }

    #[test]
    fn error_serializes_with_kebab_kind_tag() {
        let unit = serde_json::to_string(&IssueUrlError::InvalidOwner).expect("serialize unit");
        assert_eq!(unit, "{\"kind\":\"invalid-owner\"}");
        let structured = serde_json::to_string(&IssueUrlError::BodyTooLong { max: MAX_URL_LENGTH })
            .expect("serialize struct variant");
        assert!(structured.contains("\"kind\":\"body-too-long\""), "kebab kind: {structured}");
        assert!(structured.contains("\"max\":8192"), "max field: {structured}");
    }

    #[test]
    fn response_serializes_camelcase() {
        let json = serde_json::to_string(&IssueUrlResponse {
            url: "https://github.com/o/r/issues/new".to_string(),
            opened: true,
        })
        .expect("serialize response");
        assert!(json.contains("\"url\":"), "url field: {json}");
        assert!(json.contains("\"opened\":true"), "opened field: {json}");
    }
}
