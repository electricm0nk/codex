//! Reports through the codex-web API (`decisions.md §23`): `POST /api/report` with the `x-codex-key`
//! header. The request is made here, in Rust, so the key never enters the web view's JS. The key is
//! stamped at build time from the `CODEX_REPORT_KEY` env var (the CI secret of the same name); a build
//! without it reports Send as unavailable with the reason, never a fake success.

use std::time::Duration;

use serde::{Deserialize, Serialize};

pub const REPORT_URL: &str = "https://campaign-codex.org/api/report";
/// `body` must be under 8 KB, counted in bytes (diagnostics included) before anything is sent.
pub const MAX_BODY_BYTES: usize = 8 * 1024;
const REPORT_KEY: Option<&str> = option_env!("CODEX_REPORT_KEY");
const NO_KEY_REASON: &str =
    "Anonymous reports are unavailable in this build: it was built without the report key (only release builds carry it).";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReportKind {
    Bug,
    Enhancement,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportRequest {
    pub kind: ReportKind,
    pub title: String,
    pub body: String,
}

/// Every reply the endpoint can give, plus the two failures that never reach it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", rename_all = "kebab-case")]
pub enum ReportOutcome {
    Sent { message: String },
    Unavailable { reason: String },
    /// Refused before sending (empty field, body too large); nothing left the machine.
    Invalid { reason: String },
    Rejected { status: u16, reason: String },
    NoNetwork { reason: String },
    TimedOut { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportAvailability {
    pub available: bool,
    pub reason: Option<String>,
}

fn usable_key(key: Option<&str>) -> Option<&str> {
    key.filter(|k| !k.trim().is_empty())
}

pub fn availability(key: Option<&str>) -> ReportAvailability {
    match usable_key(key) {
        Some(_) => ReportAvailability { available: true, reason: None },
        None => ReportAvailability { available: false, reason: Some(NO_KEY_REASON.into()) },
    }
}

/// `diagnostics` is appended to `body` and counts toward the limit.
pub fn send_report_to(
    url: &str,
    key: Option<&str>,
    request: &ReportRequest,
    version: &str,
    diagnostics: &str,
    timeout: Duration,
) -> ReportOutcome {
    let Some(key) = usable_key(key) else {
        return ReportOutcome::Unavailable { reason: NO_KEY_REASON.into() };
    };
    if request.title.trim().is_empty() || request.body.trim().is_empty() {
        return ReportOutcome::Invalid { reason: "A report needs a title and a description.".into() };
    }
    let body = if diagnostics.is_empty() { request.body.clone() } else { format!("{}\n\n{diagnostics}", request.body) };
    if body.len() >= MAX_BODY_BYTES {
        return ReportOutcome::Invalid {
            reason: format!(
                "The report is {} bytes with diagnostics; it must be under 8 KB ({MAX_BODY_BYTES} bytes). Shorten the description.",
                body.len()
            ),
        };
    }
    let agent = ureq::AgentBuilder::new().timeout_connect(timeout).timeout(timeout).build();
    let payload = serde_json::json!({ "type": request.kind, "title": request.title, "body": body, "version": version });
    let sent = agent.post(url).set("x-codex-key", key).set("Content-Type", "application/json").send_string(&payload.to_string());
    match sent {
        Ok(response) if response.status() == 201 => ReportOutcome::Sent { message: "sent for review".into() },
        Ok(response) => rejected(response.status()),
        Err(ureq::Error::Status(status, _)) => rejected(status),
        Err(ureq::Error::Transport(err)) if is_timeout(&err) => ReportOutcome::TimedOut {
            reason: format!("The report service did not answer within {} seconds. Try again later.", timeout.as_secs_f32()),
        },
        Err(ureq::Error::Transport(err)) => ReportOutcome::NoNetwork {
            reason: format!("Could not reach the report service ({}). Check your connection and try again.", err.kind()),
        },
    }
}

fn rejected(status: u16) -> ReportOutcome {
    let reason = match status {
        400 => "The report service refused the report as malformed (400).".to_string(),
        403 => "The report service refused this build's report key (403). Update the app, or report through GitHub.".to_string(),
        413 => "The report service refused the report as too large (413). Shorten the description.".to_string(),
        other => format!("The report service answered {other}; the report was not accepted. Try again later."),
    };
    ReportOutcome::Rejected { status, reason }
}

fn is_timeout(err: &ureq::Transport) -> bool {
    let mut source = std::error::Error::source(err);
    while let Some(e) = source {
        if let Some(io) = e.downcast_ref::<std::io::Error>() {
            return matches!(io.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock);
        }
        source = e.source();
    }
    false
}

/// Build version, OS, install kind and channel (`decisions.md §30` Q-13.3), appended to `body`.
fn diagnostics(version: &str) -> String {
    use crate::update::seed::{detect_install_kind, RunningInstall, BUILD_CHANNEL};
    let appimage = std::env::var_os("APPIMAGE").map(std::path::PathBuf::from);
    let exe = std::env::current_exe().unwrap_or_default();
    let running = RunningInstall {
        managed_executable_path: appimage.clone().unwrap_or(exe),
        artifact_sha256: String::new(),
        version: version.to_string(),
        source_commit: env!("CODEX_GIT_SHA").to_string(),
        channel: BUILD_CHANNEL.to_string(),
        appimage_path: appimage,
    };
    format!(
        "---\nVersion: {version} ({})\nOS: {} {}\nInstall: {:?}\nChannel: {BUILD_CHANNEL}",
        running.source_commit,
        std::env::consts::OS,
        std::env::consts::ARCH,
        detect_install_kind(&running)
    )
}

#[tauri::command]
pub fn report_api_availability() -> ReportAvailability {
    availability(REPORT_KEY)
}

#[tauri::command(async)]
pub fn send_report(app: tauri::AppHandle, request: ReportRequest) -> ReportOutcome {
    let version = app.package_info().version.to_string();
    send_report_to(REPORT_URL, REPORT_KEY, &request, &version, &diagnostics(&version), Duration::from_secs(20))
}

#[cfg(test)]
mod report_api_tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::sync::mpsc;
    use std::thread;

    const KEY: &str = "test-key-not-a-secret";

    fn request(body: &str) -> ReportRequest {
        ReportRequest { kind: ReportKind::Bug, title: "Sheet crashes".into(), body: body.into() }
    }

    /// Reads one HTTP request (headers + Content-Length body) off `stream`.
    fn read_request(stream: &mut TcpStream) -> String {
        let mut buf = Vec::new();
        let mut chunk = [0u8; 4096];
        loop {
            let n = stream.read(&mut chunk).unwrap();
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..n]);
            let text = String::from_utf8_lossy(&buf).to_string();
            if let Some(split) = text.find("\r\n\r\n") {
                let len = text[..split]
                    .lines()
                    .find_map(|l| l.to_ascii_lowercase().strip_prefix("content-length:").map(|v| v.trim().parse::<usize>().unwrap()))
                    .unwrap_or(0);
                if buf.len() >= split + 4 + len {
                    break;
                }
            }
        }
        String::from_utf8(buf).unwrap()
    }

    /// Test-only stub of codex-web: answers one request with `status`, hands back what it received.
    fn stub_server(status: u16, reply: &'static str) -> (String, mpsc::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/api/report", listener.local_addr().unwrap());
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let received = read_request(&mut stream);
            let response = format!(
                "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{reply}",
                reply.len()
            );
            stream.write_all(response.as_bytes()).unwrap();
            tx.send(received).unwrap();
        });
        (url, rx)
    }

    fn send(url: &str, body: &str) -> ReportOutcome {
        send_report_to(url, Some(KEY), &request(body), "0.43.0", "OS: linux", Duration::from_millis(1500))
    }

    #[test]
    fn report_api_201_is_sent_for_review_with_key_header_and_contract_body() {
        let (url, rx) = stub_server(201, r#"{"ok":true}"#);
        assert_eq!(send(&url, "It crashed."), ReportOutcome::Sent { message: "sent for review".into() });
        let received = rx.recv().unwrap();
        let (head, payload) = received.split_once("\r\n\r\n").unwrap();
        assert!(head.starts_with("POST /api/report "), "{head}");
        assert!(head.to_ascii_lowercase().contains(&format!("x-codex-key: {KEY}")), "{head}");
        let json: serde_json::Value = serde_json::from_str(payload).unwrap();
        assert_eq!(json["type"], "bug");
        assert_eq!(json["title"], "Sheet crashes");
        assert_eq!(json["version"], "0.43.0");
        let body = json["body"].as_str().unwrap();
        assert!(body.starts_with("It crashed.") && body.ends_with("OS: linux"), "diagnostics appended: {body}");
        assert!(!format!("{:?}", send(&url_refused(), "x")).contains("issue created"));
    }

    #[test]
    fn report_api_400_is_rejected_as_malformed() {
        let (url, _rx) = stub_server(400, r#"{"error":"title required"}"#);
        match send(&url, "x") {
            ReportOutcome::Rejected { status: 400, reason } => assert!(reason.contains("malformed"), "{reason}"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn report_api_403_names_the_refused_key() {
        let (url, _rx) = stub_server(403, r#"{"error":"forbidden"}"#);
        match send(&url, "x") {
            ReportOutcome::Rejected { status: 403, reason } => assert!(reason.contains("key"), "{reason}"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn report_api_413_says_too_large() {
        let (url, _rx) = stub_server(413, "");
        match send(&url, "x") {
            ReportOutcome::Rejected { status: 413, reason } => assert!(reason.contains("too large"), "{reason}"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn report_api_other_status_is_rejected_with_the_code() {
        let (url, _rx) = stub_server(500, "boom");
        match send(&url, "x") {
            ReportOutcome::Rejected { status: 500, reason } => assert!(reason.contains("500"), "{reason}"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn report_api_timeout_is_timed_out() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/api/report", listener.local_addr().unwrap());
        // Accepts and never answers.
        let hold = thread::spawn(move || {
            let conn = listener.accept().unwrap();
            thread::sleep(Duration::from_millis(2500));
            drop(conn);
        });
        let outcome = send_report_to(&url, Some(KEY), &request("x"), "0.43.0", "", Duration::from_millis(300));
        assert!(matches!(outcome, ReportOutcome::TimedOut { .. }), "{outcome:?}");
        hold.join().unwrap();
    }

    fn url_refused() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/api/report", listener.local_addr().unwrap());
        drop(listener);
        url
    }

    #[test]
    fn report_api_no_network_is_no_network() {
        let outcome = send(&url_refused(), "x");
        assert!(matches!(outcome, ReportOutcome::NoNetwork { .. }), "{outcome:?}");
    }

    #[test]
    fn report_api_9kb_body_is_refused_before_sending() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}/api/report", listener.local_addr().unwrap());
        match send(&url, &"a".repeat(9 * 1024)) {
            ReportOutcome::Invalid { reason } => assert!(reason.contains("8 KB"), "{reason}"),
            other => panic!("{other:?}"),
        }
        assert!(listener.accept().is_err(), "nothing may be sent for an oversize body");
    }

    #[test]
    fn report_api_diagnostics_count_toward_the_limit() {
        let body = "a".repeat(MAX_BODY_BYTES - 4);
        let outcome = send_report_to(&url_refused(), Some(KEY), &request(&body), "v", "OS: linux", Duration::from_millis(300));
        assert!(matches!(outcome, ReportOutcome::Invalid { .. }), "{outcome:?}");
    }

    #[test]
    fn report_api_empty_title_or_body_is_refused_before_sending() {
        let blank_title = ReportRequest { kind: ReportKind::Enhancement, title: "  ".into(), body: "b".into() };
        let o = send_report_to(&url_refused(), Some(KEY), &blank_title, "v", "", Duration::from_millis(300));
        assert!(matches!(o, ReportOutcome::Invalid { .. }), "{o:?}");
        let o = send_report_to(&url_refused(), Some(KEY), &request(" "), "v", "", Duration::from_millis(300));
        assert!(matches!(o, ReportOutcome::Invalid { .. }), "{o:?}");
    }

    #[test]
    fn report_api_without_a_key_is_unavailable_with_the_reason() {
        let o = send_report_to(&url_refused(), None, &request("x"), "v", "", Duration::from_millis(300));
        assert_eq!(o, ReportOutcome::Unavailable { reason: NO_KEY_REASON.into() });
        assert_eq!(availability(None), ReportAvailability { available: false, reason: Some(NO_KEY_REASON.into()) });
        assert_eq!(availability(Some("")), availability(None), "an empty stamped secret is no key");
        assert_eq!(availability(Some(KEY)), ReportAvailability { available: true, reason: None });
    }
}
