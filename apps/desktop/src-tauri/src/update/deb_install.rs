//! Self-update for installs that came from the .deb (`InstallKind::Deb`).
//!
//! `/usr/bin/codex-desktop` is root-owned and package-managed, so the AppImage staged-replace
//! transaction cannot apply. Instead the release's `.deb` is downloaded, verified against the
//! manifest (size, sha256) and against its own package metadata (name and version), and handed
//! to the package manager under `pkexec`. The installer is root, so every check runs before it.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::transaction::{
    config_update_dir, sha256_of_file, InstallKind, InstalledState, RelaunchPrompt,
    INSTALLED_STATE_FILENAME,
};

/// Only artifacts published on this repository's releases may be installed as root.
pub const ALLOWED_URL_PREFIX: &str = "https://github.com/electricm0nk/codex/releases/download/";
const PACKAGE_NAME: &str = "codex";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebInstallRequest {
    pub version: String,
    pub name: String,
    pub url: String,
    pub sha256: String,
    pub size_bytes: u64,
}

/// What `dpkg-deb -f` reports about a .deb.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebMetadata {
    pub package: String,
    pub version: String,
}

/// The side-effecting edges, injected so the verification logic is testable without root,
/// network or dpkg.
pub trait DebSystem {
    fn download(&self, url: &str, dest: &Path) -> Result<(), String>;
    fn inspect(&self, deb: &Path) -> Result<DebMetadata, String>;
    fn install(&self, deb: &Path) -> Result<(), String>;
}

pub fn parse_deb_install_request(manifest: &Value) -> Result<DebInstallRequest, String> {
    let version = manifest
        .get("version")
        .and_then(Value::as_str)
        .filter(|v| !v.is_empty())
        .ok_or("manifest has no version")?;
    let deb = manifest
        .get("linux_deb")
        .ok_or("this release publishes no .deb artifact (manifest has no linux_deb)")?;
    let field = |key: &str| {
        deb.get(key)
            .and_then(Value::as_str)
            .filter(|v| !v.is_empty())
            .map(str::to_string)
            .ok_or(format!("manifest linux_deb.{key} is missing"))
    };
    let size_bytes = deb
        .get("size_bytes")
        .and_then(Value::as_u64)
        .ok_or("manifest linux_deb.size_bytes is missing")?;
    Ok(DebInstallRequest {
        version: version.to_string(),
        name: field("name")?,
        url: field("url")?,
        sha256: field("sha256")?.to_ascii_lowercase(),
        size_bytes,
    })
}

fn validate_request(req: &DebInstallRequest) -> Result<(), String> {
    if !req.url.starts_with(ALLOWED_URL_PREFIX) {
        return Err(format!("refusing to install from {}: not a codex release asset URL", req.url));
    }
    if req.name.contains('/') || req.name.contains('\\') || req.name.contains("..") || !req.name.ends_with(".deb") {
        return Err(format!("refusing artifact name {:?}: must be a bare .deb file name", req.name));
    }
    Ok(())
}

/// Download, verify and install the release's .deb. Returns the relaunch prompt on success.
pub fn install_deb_update(
    config_dir: &Path,
    installed: &InstalledState,
    req: &DebInstallRequest,
    system: &dyn DebSystem,
) -> Result<RelaunchPrompt, String> {
    if installed.install_kind != InstallKind::Deb {
        return Err("this install is not a .deb install".to_string());
    }
    validate_request(req)?;

    let update_dir = config_update_dir(config_dir);
    let staging = update_dir.join("staging");
    fs::create_dir_all(&staging).map_err(|e| format!("cannot create {}: {e}", staging.display()))?;
    let staged: PathBuf = staging.join(&req.name);
    let _ = fs::remove_file(&staged);

    let outcome = (|| {
        system.download(&req.url, &staged)?;
        let size = fs::metadata(&staged).map_err(|e| format!("downloaded file missing: {e}"))?.len();
        if size != req.size_bytes {
            return Err(format!("downloaded {size} bytes, manifest says {}", req.size_bytes));
        }
        let actual = sha256_of_file(&staged).map_err(|e| format!("cannot hash download: {e}"))?;
        if actual != req.sha256 {
            return Err(format!("sha256 mismatch: manifest {} downloaded {actual}", req.sha256));
        }
        let meta = system.inspect(&staged)?;
        if meta.package != PACKAGE_NAME {
            return Err(format!("package is {:?}, expected {PACKAGE_NAME:?}", meta.package));
        }
        if meta.version != req.version {
            return Err(format!("package version {} does not match release {}", meta.version, req.version));
        }
        system.install(&staged)
    })();

    // The staged copy is single-use whether or not it installed.
    let _ = fs::remove_file(&staged);
    outcome?;

    Ok(RelaunchPrompt {
        pending_update_path: update_dir.join(INSTALLED_STATE_FILENAME),
        managed_executable_path: installed.managed_executable_path.clone(),
        from_version: installed.version.clone(),
        to_version: req.version.clone(),
        artifact_sha256: req.sha256.clone(),
    })
}

/// The real edges: HTTPS download, `dpkg-deb` inspection, `pkexec apt-get` installation.
pub struct SystemDebInstaller;

impl DebSystem for SystemDebInstaller {
    fn download(&self, url: &str, dest: &Path) -> Result<(), String> {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(std::time::Duration::from_secs(15))
            .timeout_read(std::time::Duration::from_secs(60))
            .build();
        let response = agent.get(url).call().map_err(|e| format!("download of {url} failed: {e}"))?;
        let mut reader = response.into_reader();
        let mut file = fs::File::create(dest).map_err(|e| format!("cannot create {}: {e}", dest.display()))?;
        std::io::copy(&mut reader, &mut file).map_err(|e| format!("download of {url} interrupted: {e}"))?;
        Ok(())
    }

    fn inspect(&self, deb: &Path) -> Result<DebMetadata, String> {
        let field = |name: &str| -> Result<String, String> {
            let out = std::process::Command::new("dpkg-deb")
                .args(["-f"])
                .arg(deb)
                .arg(name)
                .output()
                .map_err(|e| format!("cannot run dpkg-deb: {e}"))?;
            if !out.status.success() {
                return Err(format!("dpkg-deb -f {name} failed: {}", String::from_utf8_lossy(&out.stderr).trim()));
            }
            Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
        };
        Ok(DebMetadata { package: field("Package")?, version: field("Version")? })
    }

    fn install(&self, deb: &Path) -> Result<(), String> {
        // pkexec raises the desktop's polkit password prompt; apt resolves dependencies.
        let out = std::process::Command::new("pkexec")
            .args(["apt-get", "install", "-y"])
            .arg(deb)
            .output()
            .map_err(|e| format!("cannot run pkexec (is polkit installed?): {e}"))?;
        if out.status.success() {
            return Ok(());
        }
        let stderr = String::from_utf8_lossy(&out.stderr);
        match out.status.code() {
            Some(126) => Err("installation was cancelled at the authorization prompt".to_string()),
            Some(127) => Err("authorization could not be requested (no polkit agent running)".to_string()),
            _ => Err(format!("apt-get install failed ({}): {}", out.status, stderr.trim())),
        }
    }
}

/// Run the deb self-update for the recorded install. `manifest` is the raw published manifest.
pub fn perform_deb_install(config_dir: &Path, manifest: &Value) -> Result<RelaunchPrompt, String> {
    let record = config_update_dir(config_dir).join(INSTALLED_STATE_FILENAME);
    let bytes = fs::read(&record).map_err(|e| format!("no installed-state record at {}: {e}", record.display()))?;
    let installed: InstalledState =
        serde_json::from_slice(&bytes).map_err(|e| format!("installed-state.json is unreadable: {e}"))?;
    let req = parse_deb_install_request(manifest)?;
    install_deb_update(config_dir, &installed, &req, &SystemDebInstaller)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    const GOOD_URL: &str = "https://github.com/electricm0nk/codex/releases/download/alpha-v0.16.141-abc/Codex_0.16.141_amd64.deb";

    struct Fake {
        bytes: Vec<u8>,
        meta: DebMetadata,
        install_result: Result<(), String>,
        calls: RefCell<Vec<String>>,
    }

    impl Fake {
        fn good() -> Self {
            Fake {
                bytes: b"deb-bytes".to_vec(),
                meta: DebMetadata { package: "codex".into(), version: "0.16.141".into() },
                install_result: Ok(()),
                calls: RefCell::new(vec![]),
            }
        }
        fn calls(&self) -> Vec<String> {
            self.calls.borrow().clone()
        }
    }

    impl DebSystem for Fake {
        fn download(&self, url: &str, dest: &Path) -> Result<(), String> {
            self.calls.borrow_mut().push(format!("download {url}"));
            fs::write(dest, &self.bytes).map_err(|e| e.to_string())
        }
        fn inspect(&self, _deb: &Path) -> Result<DebMetadata, String> {
            self.calls.borrow_mut().push("inspect".into());
            Ok(self.meta.clone())
        }
        fn install(&self, deb: &Path) -> Result<(), String> {
            self.calls.borrow_mut().push(format!("install {}", deb.display()));
            self.install_result.clone()
        }
    }

    fn sha(bytes: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
    }

    fn request() -> DebInstallRequest {
        DebInstallRequest {
            version: "0.16.141".into(),
            name: "Codex_0.16.141_amd64.deb".into(),
            url: GOOD_URL.into(),
            sha256: sha(b"deb-bytes"),
            size_bytes: 9,
        }
    }

    fn installed(kind: InstallKind) -> InstalledState {
        InstalledState {
            managed_executable_path: PathBuf::from("/usr/bin/codex-desktop"),
            install_kind: kind,
            channel: "alpha".into(),
            version: "0.16.140".into(),
            source_commit: "157873a67e80".into(),
            release_tag: "alpha/v0.16.140-157873a6".into(),
            manifest_hash: String::new(),
            artifact_sha256: "installed-binary-sha".into(),
            installed_at: "2026-10-06T00:00:00Z".into(),
            update_eligible: true,
            ineligible_reason: None,
        }
    }

    fn config(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("sd16-deb-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn staged_files(config: &Path) -> usize {
        fs::read_dir(config_update_dir(config).join("staging")).map(|d| d.count()).unwrap_or(0)
    }

    #[test]
    fn verified_deb_is_installed_and_prompts_relaunch() {
        let cfg = config("happy");
        let fake = Fake::good();
        let prompt = install_deb_update(&cfg, &installed(InstallKind::Deb), &request(), &fake).unwrap();
        assert_eq!(prompt.from_version, "0.16.140");
        assert_eq!(prompt.to_version, "0.16.141");
        assert_eq!(prompt.managed_executable_path, PathBuf::from("/usr/bin/codex-desktop"));
        assert_eq!(prompt.artifact_sha256, sha(b"deb-bytes"));
        let calls = fake.calls();
        assert_eq!(calls.len(), 3, "download, inspect, install exactly once each: {calls:?}");
        assert!(calls[2].starts_with("install ") && calls[2].ends_with("Codex_0.16.141_amd64.deb"));
        assert_eq!(staged_files(&cfg), 0, "staged copy is removed after install");
    }

    #[test]
    fn sha_mismatch_never_reaches_the_installer() {
        let cfg = config("sha");
        let mut fake = Fake::good();
        fake.bytes = b"tampered!".to_vec();
        let err = install_deb_update(&cfg, &installed(InstallKind::Deb), &request(), &fake).unwrap_err();
        assert!(err.contains("sha256 mismatch"), "{err}");
        assert!(!fake.calls().iter().any(|c| c.starts_with("install")));
        assert_eq!(staged_files(&cfg), 0);
    }

    #[test]
    fn size_mismatch_never_reaches_the_installer() {
        let cfg = config("size");
        let mut req = request();
        req.size_bytes = 10;
        let fake = Fake::good();
        let err = install_deb_update(&cfg, &installed(InstallKind::Deb), &req, &fake).unwrap_err();
        assert!(err.contains("bytes"), "{err}");
        assert!(!fake.calls().iter().any(|c| c.starts_with("install")));
    }

    #[test]
    fn a_deb_for_a_different_package_is_refused() {
        let cfg = config("pkg");
        let mut fake = Fake::good();
        fake.meta.package = "evil".into();
        let err = install_deb_update(&cfg, &installed(InstallKind::Deb), &request(), &fake).unwrap_err();
        assert!(err.contains("expected \"codex\""), "{err}");
        assert!(!fake.calls().iter().any(|c| c.starts_with("install")));
    }

    #[test]
    fn a_deb_whose_version_disagrees_with_the_release_is_refused() {
        let cfg = config("ver");
        let mut fake = Fake::good();
        fake.meta.version = "0.15.0".into();
        let err = install_deb_update(&cfg, &installed(InstallKind::Deb), &request(), &fake).unwrap_err();
        assert!(err.contains("does not match release"), "{err}");
        assert!(!fake.calls().iter().any(|c| c.starts_with("install")));
    }

    #[test]
    fn urls_outside_the_codex_release_namespace_are_refused_before_any_download() {
        let cfg = config("url");
        let mut req = request();
        req.url = "https://example.com/Codex_0.16.141_amd64.deb".into();
        let fake = Fake::good();
        assert!(install_deb_update(&cfg, &installed(InstallKind::Deb), &req, &fake).is_err());
        assert!(fake.calls().is_empty());
    }

    #[test]
    fn artifact_names_with_path_components_are_refused() {
        let cfg = config("name");
        for bad in ["../evil.deb", "a/b.deb", "x.sh", "..\\x.deb"] {
            let mut req = request();
            req.name = bad.into();
            let fake = Fake::good();
            assert!(install_deb_update(&cfg, &installed(InstallKind::Deb), &req, &fake).is_err(), "{bad}");
            assert!(fake.calls().is_empty(), "{bad}");
        }
    }

    #[test]
    fn installer_failure_is_surfaced_and_the_staged_file_removed() {
        let cfg = config("fail");
        let mut fake = Fake::good();
        fake.install_result = Err("pkexec: dismissed by user".into());
        let err = install_deb_update(&cfg, &installed(InstallKind::Deb), &request(), &fake).unwrap_err();
        assert!(err.contains("dismissed by user"), "{err}");
        assert_eq!(staged_files(&cfg), 0);
    }

    #[test]
    fn only_deb_installs_take_this_path() {
        let cfg = config("kind");
        let fake = Fake::good();
        assert!(install_deb_update(&cfg, &installed(InstallKind::AppImage), &request(), &fake).is_err());
        assert!(fake.calls().is_empty());
    }

    #[test]
    fn manifest_parsing_reads_the_published_linux_deb_block() {
        let manifest = serde_json::json!({
            "version": "0.16.141",
            "linux_deb": { "name": "Codex_0.16.141_amd64.deb", "url": GOOD_URL, "sha256": "AB".repeat(32), "size_bytes": 9 }
        });
        let req = parse_deb_install_request(&manifest).unwrap();
        assert_eq!(req.version, "0.16.141");
        assert_eq!(req.sha256, "ab".repeat(32));
        assert_eq!(req.size_bytes, 9);
    }

    #[test]
    fn manifest_without_linux_deb_is_an_error_naming_the_gap() {
        let err = parse_deb_install_request(&serde_json::json!({ "version": "1.0.0" })).unwrap_err();
        assert!(err.contains("linux_deb"), "{err}");
    }

    #[test]
    fn system_inspect_reads_a_real_deb_built_with_dpkg_deb() {
        let root = config("inspect").join("pkg");
        fs::create_dir_all(root.join("DEBIAN")).unwrap();
        fs::write(
            root.join("DEBIAN/control"),
            "Package: codex\nVersion: 9.9.9\nArchitecture: all\nMaintainer: t\nDescription: t\n",
        )
        .unwrap();
        let deb = root.with_extension("deb");
        let built = std::process::Command::new("dpkg-deb").arg("--build").arg(&root).arg(&deb).output();
        let Ok(built) = built else {
            eprintln!("dpkg-deb not installed: skipping real-inspect test");
            return;
        };
        assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
        let meta = SystemDebInstaller.inspect(&deb).unwrap();
        assert_eq!(meta, DebMetadata { package: "codex".into(), version: "9.9.9".into() });
    }

    #[test]
    fn perform_deb_install_without_an_installed_record_is_an_honest_error() {
        let cfg = config("norecord");
        let err = perform_deb_install(&cfg, &serde_json::json!({ "version": "1.0.0" })).unwrap_err();
        assert!(err.contains("installed-state"), "{err}");
    }
}
