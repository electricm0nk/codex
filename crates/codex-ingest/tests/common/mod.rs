//! Shared helpers for the codex-ingest integration tests that drive the real PCGen engine.
//!
//! Not every suite uses every item, so dead-code warnings are off for this module.
#![allow(dead_code)]

use std::ffi::OsStr;
use std::path::Path;

/// `Ok(())` iff a real PCGen headless export can run from `repo_dir`; otherwise `Err` naming the
/// first missing piece. One guard for every suite that shells out to `pcgen-run-character.sh`.
///
/// CI's oracle is a cone-mode sparse checkout (`scripts/fetch-pcgen-oracle.sh`). Cone mode always
/// keeps root files, so `gradlew` is there; the cone also carries `code/testsuite/PCGfiles`, which
/// brings `code/testsuite/base-xml.ftl` with it. Neither proves an engine: the Gradle wrapper jar
/// and the Java sources stay outside the cone. Checked in order:
/// executable `gradlew`, `gradle/wrapper/gradle-wrapper.jar`, `code/src/java`,
/// `code/testsuite/base-xml.ftl`, and a `java` binary on `PATH`.
pub fn pcgen_engine_runnable(repo_dir: &Path) -> Result<(), String> {
    pcgen_engine_runnable_with_path(repo_dir, std::env::var_os("PATH").as_deref())
}

/// [`pcgen_engine_runnable`] with an explicit `PATH`, so the guard's own unit tests can control
/// the `java` lookup without mutating process-wide environment.
pub fn pcgen_engine_runnable_with_path(repo_dir: &Path, path: Option<&OsStr>) -> Result<(), String> {
    let gradlew = repo_dir.join("gradlew");
    if !is_executable_file(&gradlew) {
        return Err(format!("no executable gradlew at {}", gradlew.display()));
    }
    let wrapper_jar = repo_dir.join("gradle/wrapper/gradle-wrapper.jar");
    if !wrapper_jar.is_file() {
        return Err(format!("no Gradle wrapper jar at {}", wrapper_jar.display()));
    }
    let java_sources = repo_dir.join("code/src/java");
    if !java_sources.is_dir() {
        return Err(format!("no PCGen Java source tree at {}", java_sources.display()));
    }
    let template = repo_dir.join("code/testsuite/base-xml.ftl");
    if !template.is_file() {
        return Err(format!("no batch export template at {}", template.display()));
    }
    let on_path = path
        .map(|p| std::env::split_paths(p).any(|dir| is_executable_file(&dir.join("java"))))
        .unwrap_or(false);
    if !on_path {
        return Err("no java binary on PATH".to_string());
    }
    Ok(())
}

fn is_executable_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(path)
            .map(|meta| (meta.permissions().mode() & 0o111) != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        true
    }
}
