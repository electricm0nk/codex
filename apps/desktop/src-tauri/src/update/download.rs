//! Shared edges for the self-update installers: which URLs and file names may be installed, and the
//! HTTPS download itself. The `.deb`, AppImage and Windows installers all fetch from this repository's
//! GitHub release assets and nowhere else, so the rule lives in one place.

use std::fs;
use std::io::Write;
use std::path::Path;

/// Only artifacts published on this repository's releases may be installed.
pub const ALLOWED_URL_PREFIX: &str = "https://github.com/electricm0nk/codex/releases/download/";

/// Refuses a URL outside this repository's release assets, and an artifact name that is not a bare
/// file name ending in `extension` (matched case-insensitively, `".AppImage"` style).
pub fn validate_asset(url: &str, name: &str, extension: &str) -> Result<(), String> {
    if !url.starts_with(ALLOWED_URL_PREFIX) {
        return Err(format!("refusing to install from {url}: not a codex release asset URL"));
    }
    let bare = !name.contains('/') && !name.contains('\\') && !name.contains("..") && !name.is_empty();
    if !bare || !name.to_ascii_lowercase().ends_with(&extension.to_ascii_lowercase()) {
        return Err(format!("refusing artifact name {name:?}: must be a bare {extension} file name"));
    }
    Ok(())
}

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_connect(std::time::Duration::from_secs(15))
        .timeout_read(std::time::Duration::from_secs(60))
        .build()
}

/// Streams `url` into `writer`, returning the byte count.
pub fn download_to_writer(url: &str, writer: &mut dyn Write) -> std::io::Result<u64> {
    let response = agent()
        .get(url)
        .call()
        .map_err(|e| std::io::Error::other(format!("download of {url} failed: {e}")))?;
    let mut reader = response.into_reader();
    std::io::copy(&mut reader, writer)
}

/// Streams `url` into a new file at `dest`.
pub fn download_to_file(url: &str, dest: &Path) -> Result<(), String> {
    let mut file = fs::File::create(dest).map_err(|e| format!("cannot create {}: {e}", dest.display()))?;
    download_to_writer(url, &mut file).map(|_| ()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD: &str = "https://github.com/electricm0nk/codex/releases/download/alpha-v1/Codex.AppImage";

    #[test]
    fn a_release_asset_url_and_bare_name_are_accepted() {
        assert!(validate_asset(GOOD, "Codex_1_amd64.AppImage", ".AppImage").is_ok());
        assert!(validate_asset(GOOD, "Codex_1_x64-setup.EXE", ".exe").is_ok(), "the extension match ignores case");
    }

    #[test]
    fn a_url_outside_this_repositorys_releases_is_refused() {
        for url in ["https://evil.example/codex/releases/download/x/a.exe", "http://github.com/electricm0nk/codex/releases/download/x/a.exe", "https://github.com/other/codex/releases/download/x/a.exe"] {
            let err = validate_asset(url, "a.exe", ".exe").expect_err(url);
            assert!(err.contains("not a codex release asset URL"), "{err}");
        }
    }

    #[test]
    fn a_name_that_is_not_a_bare_file_of_the_right_kind_is_refused() {
        for name in ["../a.exe", "dir/a.exe", "dir\\a.exe", "a.msi", "", "a.exe.sh"] {
            assert!(validate_asset(GOOD, name, ".exe").is_err(), "{name:?} must be refused");
        }
    }
}
