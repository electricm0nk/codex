fn main() {
    // Without this, cargo only reruns build.rs when a file inside this crate
    // changes — a commit elsewhere in the repo wouldn't refresh the embedded
    // hash. .git/HEAD changes on every commit/checkout, so watching it keeps
    // the embedded commit accurate.
    println!("cargo:rerun-if-changed=../../../.git/HEAD");
    println!("cargo:rustc-env=CODEX_GIT_SHA={}", git_short_sha());
    tauri_build::build()
}

/// Short commit hash of the current checkout, embedded at compile time so the
/// running app can report exactly which commit it was built from. Falls back
/// to "unknown" for builds outside a git checkout (e.g. a source tarball)
/// rather than failing the build.
fn git_short_sha() -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|sha| sha.trim().to_string())
        .filter(|sha| !sha.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}
