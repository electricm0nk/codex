/// Version increment test for SD-24 criterion 8.4
/// Verifies that the desktop app version is bumped to 0.5.98
/// RED: test fails against 0.5.97
/// GREEN: test passes after version increment to 0.5.98

use std::fs;
use std::path::Path;

#[test]
fn desktop_version_is_correctly_incremented() {
    // Read package.json
    let package_json_path = Path::new("apps/desktop/package.json");
    assert!(package_json_path.exists(), "apps/desktop/package.json not found");

    let package_content = fs::read_to_string(package_json_path)
        .expect("Failed to read apps/desktop/package.json");

    // Parse JSON
    let package_json: serde_json::Value = serde_json::from_str(&package_content)
        .expect("Failed to parse apps/desktop/package.json as JSON");

    let package_version = package_json["version"].as_str()
        .expect("Failed to extract version from package.json");

    // Read tauri.conf.json
    let tauri_conf_path = Path::new("apps/desktop/src-tauri/tauri.conf.json");
    assert!(tauri_conf_path.exists(), "apps/desktop/src-tauri/tauri.conf.json not found");

    let tauri_content = fs::read_to_string(tauri_conf_path)
        .expect("Failed to read apps/desktop/src-tauri/tauri.conf.json");

    let tauri_conf: serde_json::Value = serde_json::from_str(&tauri_content)
        .expect("Failed to parse apps/desktop/src-tauri/tauri.conf.json as JSON");

    let tauri_version = tauri_conf["version"].as_str()
        .expect("Failed to extract version from tauri.conf.json");

    // Both must be 0.5.98
    assert_eq!(package_version, "0.5.98",
        "apps/desktop/package.json version should be 0.5.98 but found {}", package_version);

    assert_eq!(tauri_version, "0.5.98",
        "apps/desktop/src-tauri/tauri.conf.json version should be 0.5.98 but found {}", tauri_version);

    // Both must match
    assert_eq!(package_version, tauri_version,
        "Version mismatch: package.json={} vs tauri.conf.json={}", package_version, tauri_version);
}
