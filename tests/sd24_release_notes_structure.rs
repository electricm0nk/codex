/// SD-24 Criterion 8.3 — Release notes structure validation
///
/// Verifies that docs/release/SD-24-beta-readiness-and-multiclass/release-notes.md
/// contains all required sections per the REQUIRED_NOTES_SECTIONS specification.
#[test]
fn release_notes_has_all_required_sections() {
    let release_notes_path = "docs/release/SD-24-beta-readiness-and-multiclass/release-notes.md";

    let content = std::fs::read_to_string(release_notes_path)
        .expect("release-notes.md should exist");

    // Per loop-instruction.md §2.3 step 7: "template's REQUIRED_NOTES_SECTIONS"
    // Per tools/release/check_release_manifest.py: Summary, User-Visible Changes,
    // Defects Fixed, Operational Notes, Verification Evidence, Known Issues, Update Eligibility

    let required_sections = vec![
        "## Summary",
        "## User-Visible Changes",
        "## Defects Fixed",
        "## Operational Notes",
        "## Verification Evidence",
        "## Known Issues",
        "## Update Eligibility",
    ];

    for section in required_sections {
        assert!(
            content.contains(section),
            "release-notes.md must contain section: {}",
            section
        );
    }

    // Verify that sections are not just placeholders
    assert!(!content.contains("(populated at Epic 8 closure)"),
        "release-notes.md sections should be populated, not placeholders");
}

/// Verify release notes mentions each epic
#[test]
fn release_notes_covers_all_epics() {
    let release_notes_path = "docs/release/SD-24-beta-readiness-and-multiclass/release-notes.md";

    let content = std::fs::read_to_string(release_notes_path)
        .expect("release-notes.md should exist");

    let epics = vec![
        "Epic 1",
        "Epic 2",
        "Epic 3",
        "Epic 4",
        "Epic 5",
        "Epic 6",
        "Epic 7",
        "Epic 8",
    ];

    for epic in epics {
        assert!(
            content.contains(epic),
            "release-notes.md should mention {}",
            epic
        );
    }
}
