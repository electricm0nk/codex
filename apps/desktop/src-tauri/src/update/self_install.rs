//! First-run self-install for the Linux AppImage.
//!
//! A downloaded AppImage runs from wherever the browser saved it. When it is started from anywhere
//! but `~/Codex/Codex.AppImage`, the app offers once to copy itself there, add a menu entry and
//! icon, and point `installed-state.json` at the copy, so in-app updates replace a file at a fixed
//! path. The decision and the file work are plain functions over paths; `run_at_startup` is the
//! Tauri glue (dialog, relaunch, exit). Not an AppImage (`$APPIMAGE` unset): nothing happens.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

use super::seed::{gather_running_install, overwrite_installed_state, BUILD_CHANNEL};
use super::transaction::{config_update_dir, resolve_config_root, InstalledState};

pub const INSTALL_DIR_NAME: &str = "Codex";
pub const INSTALLED_APPIMAGE_NAME: &str = "Codex.AppImage";
/// Matches the AppImage's own bundled entry (`StartupWMClass=codex-desktop`, `Icon=codex-desktop`).
pub const DESKTOP_FILE_NAME: &str = "codex-desktop.desktop";
pub const ICON_NAME: &str = "codex-desktop";
const ICON_SIZE_DIR: &str = "512x512";
/// The 512x512 source icon the bundler also packs into the AppImage.
const ICON_PNG: &[u8] = include_bytes!("../../icons/icon.png");
/// One declined AppImage path per line, under the update config dir.
pub const DECLINED_FILENAME: &str = "self-install-declined.txt";
const JUST_RUN_LABEL: &str = "Just run this time";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfInstallCase {
    /// `$APPIMAGE` (or `$HOME`) unset: a .deb, dev, Windows or macOS build.
    NotAppImage,
    /// Running from `~/Codex/Codex.AppImage`: keep the menu entry and icon current, ask nothing.
    AlreadyInstalled { installed: PathBuf },
    /// The user chose "Just run this time" for this file before.
    Declined,
    /// No install yet: offer Install.
    Fresh { running: PathBuf, target: PathBuf },
    /// `~/Codex/Codex.AppImage` exists and this is a different file: offer Replace.
    ReplaceExisting { running: PathBuf, target: PathBuf },
}

pub fn install_target(home: &Path) -> PathBuf {
    home.join(INSTALL_DIR_NAME).join(INSTALLED_APPIMAGE_NAME)
}

/// The path with symlinks resolved, or as given when it cannot be (a deleted download).
fn canonical(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

pub fn decide(appimage_env: Option<PathBuf>, home: Option<&Path>, declined: &[PathBuf]) -> SelfInstallCase {
    let Some(running) = appimage_env.filter(|path| !path.as_os_str().is_empty()) else {
        return SelfInstallCase::NotAppImage;
    };
    let Some(home) = home.filter(|home| home.is_absolute()) else {
        return SelfInstallCase::NotAppImage;
    };
    let target = install_target(home);
    let running_real = canonical(&running);
    if running_real == canonical(&target) {
        return SelfInstallCase::AlreadyInstalled { installed: target };
    }
    if declined.iter().any(|path| canonical(path) == running_real) {
        return SelfInstallCase::Declined;
    }
    if target.exists() {
        SelfInstallCase::ReplaceExisting { running, target }
    } else {
        SelfInstallCase::Fresh { running, target }
    }
}

/// `(title, message, accept label)` for the dialog, or `None` when nothing is asked.
pub fn prompt_text(case: &SelfInstallCase) -> Option<(String, String, String)> {
    let (title, message, accept) = match case {
        SelfInstallCase::Fresh { running, target } => (
            "Install Codex",
            format!(
                "Install Codex to ~/Codex?\n\nCodex will be copied to {} and added to your applications menu. \
                 Updates install there.\n\nAfterwards you can delete the downloaded file {}.",
                target.display(),
                running.display()
            ),
            "Install",
        ),
        SelfInstallCase::ReplaceExisting { running, target } => (
            "Replace installed Codex",
            format!(
                "Codex is already installed at {}.\n\nReplace it with this copy?\n\n\
                 Afterwards you can delete the downloaded file {}.",
                target.display(),
                running.display()
            ),
            "Replace",
        ),
        SelfInstallCase::NotAppImage | SelfInstallCase::AlreadyInstalled { .. } | SelfInstallCase::Declined => return None,
    };
    Some((title.to_string(), message, accept.to_string()))
}

fn declined_file(config_root: &Path) -> PathBuf {
    config_update_dir(config_root).join(DECLINED_FILENAME)
}

/// A missing or unreadable file means nothing was declined: the worst case is one more question.
pub fn read_declined(config_root: &Path) -> Vec<PathBuf> {
    fs::read_to_string(declined_file(config_root))
        .map(|text| text.lines().filter(|line| !line.is_empty()).map(PathBuf::from).collect())
        .unwrap_or_default()
}

pub fn remember_declined(config_root: &Path, running: &Path) -> Result<(), String> {
    let mut declined = read_declined(config_root);
    let running = canonical(running);
    if declined.contains(&running) {
        return Ok(());
    }
    declined.push(running);
    let path = declined_file(config_root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("cannot create {}: {err}", parent.display()))?;
    }
    let text: String = declined.iter().map(|path| format!("{}\n", path.display())).collect();
    fs::write(&path, text).map_err(|err| format!("cannot write {}: {err}", path.display()))
}

/// `$XDG_DATA_HOME` when it is set to an absolute path, else `~/.local/share`.
pub fn data_home(xdg_data_home: Option<OsString>, home: &Path) -> PathBuf {
    match xdg_data_home.map(PathBuf::from) {
        Some(dir) if dir.is_absolute() => dir,
        _ => home.join(".local").join("share"),
    }
}

/// A desktop-entry string value: backslash is the escape character.
fn escape_value(text: &str) -> String {
    text.replace('\\', "\\\\")
}

/// An `Exec` argument: quoted when it holds a reserved character, with `"`, `` ` ``, `$` and `\`
/// escaped inside the quotes; `%` is doubled because it starts a field code.
fn exec_argument(text: &str) -> String {
    const RESERVED: &[char] = &[' ', '\t', '\n', '"', '\'', '\\', '>', '<', '~', '|', '&', ';', '$', '*', '?', '#', '(', ')', '`'];
    let arg = if text.contains(RESERVED) {
        let mut quoted = String::from("\"");
        for ch in text.chars() {
            if matches!(ch, '"' | '`' | '$' | '\\') {
                quoted.push('\\');
            }
            quoted.push(ch);
        }
        quoted.push('"');
        quoted
    } else {
        text.to_string()
    };
    escape_value(&arg.replace('%', "%%"))
}

pub fn desktop_entry_text(appimage: &Path) -> String {
    let path = appimage.to_string_lossy();
    format!(
        "[Desktop Entry]\nType=Application\nName=Codex\nExec={}\nTryExec={}\nIcon={ICON_NAME}\n\
         StartupWMClass={ICON_NAME}\nCategories=Game;RolePlaying;\nTerminal=false\n",
        exec_argument(&path),
        escape_value(&path)
    )
}

pub fn desktop_entry_path(data_home: &Path) -> PathBuf {
    data_home.join("applications").join(DESKTOP_FILE_NAME)
}

pub fn icon_path(data_home: &Path) -> PathBuf {
    data_home.join("icons").join("hicolor").join(ICON_SIZE_DIR).join("apps").join(format!("{ICON_NAME}.png"))
}

/// Write the menu entry and icon when missing or different. Returns whether anything changed.
pub fn ensure_desktop_integration(data_home: &Path, appimage: &Path) -> Result<bool, String> {
    let entry = write_if_different(&desktop_entry_path(data_home), desktop_entry_text(appimage).as_bytes())?;
    let icon = write_if_different(&icon_path(data_home), ICON_PNG)?;
    Ok(entry || icon)
}

fn write_if_different(path: &Path, bytes: &[u8]) -> Result<bool, String> {
    if fs::read(path).is_ok_and(|current| current == bytes) {
        return Ok(false);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("cannot create {}: {err}", parent.display()))?;
    }
    fs::write(path, bytes).map_err(|err| format!("cannot write {}: {err}", path.display()))?;
    Ok(true)
}

/// Best effort: menus and icon themes pick the new files up without a re-login. Either tool may be
/// absent, and the files are already in place, so failures are ignored.
pub fn refresh_desktop_caches(data_home: &Path) {
    use std::process::{Command, Stdio};
    let applications = data_home.join("applications");
    let hicolor = data_home.join("icons").join("hicolor");
    let runs: [(&str, Vec<&std::ffi::OsStr>); 2] = [
        ("update-desktop-database", vec![applications.as_os_str()]),
        ("gtk-update-icon-cache", vec!["-q".as_ref(), "-t".as_ref(), "-f".as_ref(), hicolor.as_os_str()]),
    ];
    for (tool, args) in runs {
        let _ = Command::new(tool).args(args).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status();
    }
}

/// Copy `source` to `target` through a temp file next to `target`, then rename; mode 0755.
pub fn copy_appimage(source: &Path, target: &Path) -> Result<(), String> {
    let dir = target.parent().ok_or_else(|| format!("{} has no parent directory", target.display()))?;
    fs::create_dir_all(dir).map_err(|err| format!("cannot create {}: {err}", dir.display()))?;
    let partial = dir.join(format!(".{INSTALLED_APPIMAGE_NAME}.partial-{}", std::process::id()));
    let result = copy_then_rename(source, &partial, target);
    if result.is_err() {
        let _ = fs::remove_file(&partial);
    }
    result
}

fn copy_then_rename(source: &Path, partial: &Path, target: &Path) -> Result<(), String> {
    fs::copy(source, partial).map_err(|err| format!("cannot copy {} to {}: {err}", source.display(), partial.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(partial, fs::Permissions::from_mode(0o755))
            .map_err(|err| format!("cannot make {} executable: {err}", partial.display()))?;
    }
    fs::rename(partial, target).map_err(|err| format!("cannot move {} to {}: {err}", partial.display(), target.display()))
}

/// Install the running AppImage to `~/Codex/Codex.AppImage`: copy it, point the update record at the
/// copy, and write the menu entry and icon. Returns the installed path.
pub fn perform_self_install(
    running: &Path,
    home: &Path,
    data_home: &Path,
    config_root: &Path,
    version: &str,
) -> Result<(PathBuf, InstalledState), String> {
    let target = install_target(home);
    copy_appimage(running, &target)?;
    // The record must name the copy: `reconcile_installed_state` never rewrites an AppImage record,
    // so one naming the download would make every in-app update refuse the copy as a foreign file.
    let installed = gather_running_install(version, &target, Some(target.clone()), env!("CODEX_GIT_SHA"), BUILD_CHANNEL)?;
    let record = overwrite_installed_state(config_root, &installed)?;
    ensure_desktop_integration(data_home, &target)?;
    Ok((target, record))
}

/// Start the installed copy as its own process. The AppImage runtime variables of this process are
/// removed so the new runtime sets its own instead of inheriting a mount that is about to go away.
fn launch_detached(appimage: &Path) -> Result<(), String> {
    use std::process::{Command, Stdio};
    let mut command = Command::new(appimage);
    command
        .env_remove("APPIMAGE")
        .env_remove("APPDIR")
        .env_remove("ARGV0")
        .env_remove("OWD")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    command.spawn().map(|_| ()).map_err(|err| format!("cannot start {}: {err}", appimage.display()))
}

/// Startup hook. Runs before the installed-state seed. The question is asked from a helper thread:
/// called inside `setup`, before the event loop runs, the GTK dialog backend raced Tauri's own GTK
/// start-up and could hang the main window. From another thread the dialog is posted to the running
/// event loop, the same way a command shows one.
pub fn run_at_startup(app: &tauri::AppHandle, version: &str) {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let config_root = resolve_config_root();
    let case = decide(std::env::var_os("APPIMAGE").map(PathBuf::from), home.as_deref(), &read_declined(&config_root));
    let Some(home) = home else { return };
    let data = data_home(std::env::var_os("XDG_DATA_HOME"), &home);

    if let SelfInstallCase::AlreadyInstalled { installed } = &case {
        match ensure_desktop_integration(&data, installed) {
            Ok(true) => refresh_desktop_caches(&data),
            Ok(false) => {}
            Err(err) => eprintln!("Cannot update the Codex menu entry: {err}"),
        }
        return;
    }
    let (Some((title, message, accept)), SelfInstallCase::Fresh { running, .. } | SelfInstallCase::ReplaceExisting { running, .. }) =
        (prompt_text(&case), &case)
    else {
        return;
    };
    let running = running.clone();
    let version = version.to_string();
    let handle = app.clone();
    std::thread::spawn(move || {
        handle
            .dialog()
            .message(message)
            .title(title)
            .buttons(MessageDialogButtons::OkCancelCustom(accept, JUST_RUN_LABEL.to_string()))
            .show(move |accepted| {
                if !accepted {
                    if let Err(err) = remember_declined(&config_root, &running) {
                        eprintln!("Cannot remember the declined install: {err}");
                    }
                    return;
                }
                let outcome = perform_self_install(&running, &home, &data, &config_root, &version).and_then(|(target, _)| {
                    refresh_desktop_caches(&data);
                    launch_detached(&target)
                });
                match outcome {
                    Ok(()) => handle.exit(0),
                    Err(err) => {
                        eprintln!("Codex self-install failed: {err}");
                        handle
                            .dialog()
                            .message(format!("Codex could not be installed to ~/Codex:\n\n{err}\n\nCodex keeps running from {}.", running.display()))
                            .title("Install failed")
                            .kind(MessageDialogKind::Error)
                            .show(|_| {});
                    }
                }
            });
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::update::transaction::{sha256_of_file, InstallKind, INSTALLED_STATE_FILENAME};

    fn temp_dir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("codex-self-install-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn download(dir: &Path, bytes: &[u8]) -> PathBuf {
        let path = dir.join("Downloads").join("Codex_0.16.141_amd64.AppImage");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, bytes).unwrap();
        path
    }

    #[test]
    fn without_appimage_or_home_nothing_happens() {
        let home = temp_dir("not-appimage");
        assert_eq!(decide(None, Some(&home), &[]), SelfInstallCase::NotAppImage);
        assert_eq!(decide(Some(PathBuf::new()), Some(&home), &[]), SelfInstallCase::NotAppImage);
        assert_eq!(decide(Some(download(&home, b"x")), None, &[]), SelfInstallCase::NotAppImage);
    }

    #[test]
    fn running_from_the_install_path_is_already_installed() {
        let home = temp_dir("installed");
        let target = install_target(&home);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(&target, b"x").unwrap();
        assert_eq!(decide(Some(target.clone()), Some(&home), &[]), SelfInstallCase::AlreadyInstalled { installed: target.clone() });
        // The same file reached through a symlinked path is still the install.
        #[cfg(unix)]
        {
            let link = home.join("link-home");
            std::os::unix::fs::symlink(&home, &link).unwrap();
            let via_link = install_target(&link);
            assert!(matches!(decide(Some(via_link), Some(&home), &[]), SelfInstallCase::AlreadyInstalled { .. }));
        }
    }

    #[test]
    fn a_download_with_no_install_is_offered_install() {
        let home = temp_dir("fresh");
        let running = download(&home, b"x");
        assert_eq!(
            decide(Some(running.clone()), Some(&home), &[]),
            SelfInstallCase::Fresh { running, target: install_target(&home) }
        );
    }

    #[test]
    fn a_declined_path_is_not_asked_again_but_a_different_download_is() {
        let home = temp_dir("declined");
        let config = home.join("config");
        let running = download(&home, b"x");
        remember_declined(&config, &running).unwrap();
        let declined = read_declined(&config);
        assert_eq!(decide(Some(running.clone()), Some(&home), &declined), SelfInstallCase::Declined);
        let other = running.with_file_name("Codex_0.16.142_amd64.AppImage");
        fs::write(&other, b"y").unwrap();
        assert!(matches!(decide(Some(other), Some(&home), &declined), SelfInstallCase::Fresh { .. }));
    }

    #[test]
    fn declined_paths_accumulate_and_a_missing_file_reads_as_none() {
        let config = temp_dir("declined-file").join("config");
        assert!(read_declined(&config).is_empty());
        remember_declined(&config, Path::new("/home/u/Downloads/a.AppImage")).unwrap();
        remember_declined(&config, Path::new("/home/u/Downloads/b.AppImage")).unwrap();
        remember_declined(&config, Path::new("/home/u/Downloads/a.AppImage")).unwrap();
        assert_eq!(
            read_declined(&config),
            vec![PathBuf::from("/home/u/Downloads/a.AppImage"), PathBuf::from("/home/u/Downloads/b.AppImage")]
        );
        assert!(config_update_dir(&config).join(DECLINED_FILENAME).is_file());
    }

    #[test]
    fn an_existing_install_from_another_file_is_offered_replace() {
        let home = temp_dir("replace");
        let target = install_target(&home);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(&target, b"old").unwrap();
        let running = download(&home, b"new");
        assert_eq!(
            decide(Some(running.clone()), Some(&home), &[]),
            SelfInstallCase::ReplaceExisting { running, target }
        );
    }

    #[test]
    fn the_dialog_names_the_action_and_says_the_download_can_be_deleted() {
        let running = PathBuf::from("/home/u/Downloads/Codex_0.16.141_amd64.AppImage");
        let target = PathBuf::from("/home/u/Codex/Codex.AppImage");
        let (title, message, accept) =
            prompt_text(&SelfInstallCase::Fresh { running: running.clone(), target: target.clone() }).unwrap();
        assert_eq!(accept, "Install");
        assert!(title.contains("Install"), "{title}");
        assert!(message.contains("Install Codex to ~/Codex?"), "{message}");
        assert!(message.contains("/home/u/Codex/Codex.AppImage"), "{message}");
        assert!(message.contains("/home/u/Downloads/Codex_0.16.141_amd64.AppImage") && message.contains("delete"), "{message}");

        let (_, message, accept) = prompt_text(&SelfInstallCase::ReplaceExisting { running, target }).unwrap();
        assert_eq!(accept, "Replace");
        assert!(message.contains("already installed"), "{message}");
        assert!(message.contains("delete"), "{message}");

        for quiet in [
            SelfInstallCase::NotAppImage,
            SelfInstallCase::Declined,
            SelfInstallCase::AlreadyInstalled { installed: PathBuf::from("/home/u/Codex/Codex.AppImage") },
        ] {
            assert_eq!(prompt_text(&quiet), None, "{quiet:?}");
        }
    }

    #[test]
    fn data_home_follows_xdg_only_when_it_is_absolute() {
        let home = Path::new("/home/u");
        assert_eq!(data_home(None, home), PathBuf::from("/home/u/.local/share"));
        assert_eq!(data_home(Some("".into()), home), PathBuf::from("/home/u/.local/share"));
        assert_eq!(data_home(Some("relative/share".into()), home), PathBuf::from("/home/u/.local/share"));
        assert_eq!(data_home(Some("/data/share".into()), home), PathBuf::from("/data/share"));
    }

    #[test]
    fn the_menu_entry_launches_the_installed_appimage() {
        let text = desktop_entry_text(Path::new("/home/u/Codex/Codex.AppImage"));
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines[0], "[Desktop Entry]");
        for expected in [
            "Type=Application",
            "Name=Codex",
            "Exec=/home/u/Codex/Codex.AppImage",
            "TryExec=/home/u/Codex/Codex.AppImage",
            "Icon=codex-desktop",
            "StartupWMClass=codex-desktop",
            "Categories=Game;RolePlaying;",
            "Terminal=false",
        ] {
            assert!(lines.contains(&expected), "missing {expected:?} in\n{text}");
        }
        assert!(text.ends_with('\n'));
    }

    #[test]
    fn a_home_with_a_space_is_quoted_in_exec_only() {
        let text = desktop_entry_text(Path::new("/home/Jo Smith/Codex/Codex.AppImage"));
        assert!(text.contains("\nExec=\"/home/Jo Smith/Codex/Codex.AppImage\"\n"), "{text}");
        assert!(text.contains("\nTryExec=/home/Jo Smith/Codex/Codex.AppImage\n"), "{text}");
    }

    #[test]
    fn menu_entry_and_icon_are_written_once_and_rewritten_when_they_drift() {
        let data = temp_dir("integration").join("share");
        let appimage = Path::new("/home/u/Codex/Codex.AppImage");
        assert!(ensure_desktop_integration(&data, appimage).unwrap(), "first run writes");
        assert_eq!(fs::read_to_string(desktop_entry_path(&data)).unwrap(), desktop_entry_text(appimage));
        assert_eq!(fs::read(icon_path(&data)).unwrap(), ICON_PNG);
        assert!(icon_path(&data).ends_with("icons/hicolor/512x512/apps/codex-desktop.png"));
        assert!(!ensure_desktop_integration(&data, appimage).unwrap(), "unchanged files are left alone");

        fs::write(desktop_entry_path(&data), "[Desktop Entry]\nExec=/old/path\n").unwrap();
        assert!(ensure_desktop_integration(&data, appimage).unwrap());
        assert_eq!(fs::read_to_string(desktop_entry_path(&data)).unwrap(), desktop_entry_text(appimage));
        fs::remove_file(icon_path(&data)).unwrap();
        assert!(ensure_desktop_integration(&data, appimage).unwrap());
        assert_eq!(fs::read(icon_path(&data)).unwrap(), ICON_PNG);
    }

    #[test]
    fn the_copy_is_complete_executable_and_leaves_the_download_in_place() {
        let dir = temp_dir("copy");
        let source = download(&dir, b"appimage-bytes");
        let target = install_target(&dir.join("home"));
        copy_appimage(&source, &target).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"appimage-bytes");
        assert_eq!(fs::read(&source).unwrap(), b"appimage-bytes", "the download stays");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(fs::metadata(&target).unwrap().permissions().mode() & 0o777, 0o755);
        }
        let leftovers: Vec<_> = fs::read_dir(target.parent().unwrap()).unwrap().map(|e| e.unwrap().file_name()).collect();
        assert_eq!(leftovers, vec![OsString::from(INSTALLED_APPIMAGE_NAME)], "no temp file is left behind");

        // Replace: an existing install is overwritten with the running file's bytes.
        fs::write(&source, b"newer-bytes").unwrap();
        copy_appimage(&source, &target).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"newer-bytes");
    }

    #[test]
    fn a_failed_copy_reports_the_path_and_leaves_no_install() {
        let dir = temp_dir("copy-missing");
        let target = install_target(&dir.join("home"));
        let err = copy_appimage(&dir.join("missing.AppImage"), &target).unwrap_err();
        assert!(err.contains("missing.AppImage"), "{err}");
        assert!(!target.exists());
    }

    #[test]
    fn install_points_the_update_record_at_the_copy_even_over_an_appimage_record() {
        let dir = temp_dir("perform");
        let home = dir.join("home");
        let data = home.join(".local/share");
        let config = home.join(".config");
        let running = download(&dir, b"appimage-bytes");
        // The download was run once before: an AppImage record names the download path.
        let first = super::super::seed::gather_running_install("0.16.141", &running, Some(running.clone()), "157873a67e80", "alpha").unwrap();
        super::super::seed::reconcile_installed_state(&config, &first).unwrap();

        let (target, record) = perform_self_install(&running, &home, &data, &config, "0.16.141").unwrap();
        assert_eq!(target, install_target(&home));
        let on_disk: InstalledState =
            serde_json::from_slice(&fs::read(config_update_dir(&config).join(INSTALLED_STATE_FILENAME)).unwrap()).unwrap();
        assert_eq!(on_disk, record);
        assert_eq!(record.managed_executable_path, target);
        assert_eq!(record.artifact_sha256, sha256_of_file(&target).unwrap());
        assert_eq!(record.install_kind, InstallKind::AppImage);
        assert_eq!(record.version, "0.16.141");
        assert!(record.update_eligible);
        assert_eq!(fs::read_to_string(desktop_entry_path(&data)).unwrap(), desktop_entry_text(&target));
        assert!(icon_path(&data).is_file());
        assert!(running.is_file(), "the download is left in place");
    }
}
