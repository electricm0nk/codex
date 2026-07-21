# Cycle release-notes-cycle — Epic 8 / Criterion 8.3

- **Card ID:** t_3b47a9fa
- **Commit SHA:** 3ecced2
- **Files touched:** `docs/release/SD-24-beta-readiness-and-multiclass/release-notes.md`, `tests/sd24_release_notes_structure.rs`

## Acceptance criterion

Criterion 8.3 — Release notes generated at `./release-notes.md` per the template's REQUIRED_NOTES_SECTIONS: Summary, User-Visible Changes, Defects Fixed, Operational Notes, Verification Evidence, Known Issues, Update Eligibility.

## Status

Complete.

## Implementation notes

**RED (TDD):** Wrote `tests/sd24_release_notes_structure.rs` with two tests:
1. `release_notes_has_all_required_sections()` — verifies all 7 required markdown sections exist and are not placeholders.
2. `release_notes_covers_all_epics()` — verifies all 8 epics are mentioned in the release notes.

Ran against the placeholder `release-notes.md`; both tests failed as expected (sections were placeholders, no epic mentions present).

**GREEN:** Implemented comprehensive release notes at `./release-notes.md` covering:
- **Summary:** Overview of SD-24's 8-epic delivery (multiclass, equipment corpus, Tauri commands, architecture refresh).
- **User-Visible Changes:** Multiclass advancement (Fighter+Wizard levels 1-10), equipment corpus ingestion (CRB 100%, APG 100%, ACG 100%), Tauri command surface (appendToCharacter, recomputeCharacter, reSaveCharacter), item picker wiring.
- **Defects Fixed:** Epic 1 identifier cleanup (6 modules, 13 constants), Epic 4 Wizard spell baseline gap, Epic 5 Wizard multiclass integration, Epic 6 spell/corpus measurement corrections, Epic 7 loadout hardcoding removal.
- **Operational Notes:** Breaking changes (none), architecture integration (multiclass dispatch, equipment schema, Tauri surface), data regeneration notes, deferred work (formal APG/ACG multiclass deferral).
- **Verification Evidence:** Test results (3992 passed / 0 failed root; 113 passed / 0 failed Tauri; 441 test binaries), dual-audit gate results (OK_NO_BUNDLE_TAGS, pre-existing benign findings), per-cycle artifact evidence (all 8 epics' cycle logs + receipts).
- **Known Issues:** Returned-to-backlog items (CRB equipment description 61.2%, APG equipment description 0%, APG spell full text 87.9% — all real corpus limitations, not code defects), deferred work (APG/ACG multiclass, Bestiary 1 equipment).
- **Update Eligibility:** Supported configs (Fighter+Wizard multiclass levels 1-20, chassis-only for all other classes), installation/upgrade notes (no breaking changes, existing characters can re-save), rollback safety.

Re-ran both tests after implementation: `release_notes_has_all_required_sections ... ok`, `release_notes_covers_all_epics ... ok`.

**Dual-audit gate (post-implementation):**
- Identifier audit: `OK_NO_BUNDLE_TAGS` (no bundle-tag leaks in the new test file or release notes).
- Wired-integration audit: pre-existing 4 benign finds from prior cycles (Epic 6 spell/equipment completion: "placeholder" doc comments, "hack" in Plant Growth real spell text), zero new violations from this cycle.

**Regression test suite (refreshed at release-notes finalize pass):**
- Root `cargo test --locked --tests`: 444 test binaries, 4018 passed (3994 prior + 24 new since receipt draft; 48 ignored pre-existing), 0 failed.
- Tauri `cargo test --locked`: 113 passed, 0 failed (unaffected).

**Note on test-count drift:** The release notes and the receipt's "Regression test suite" line were authored against the test counts at their respective commit times (release-notes.md at 439/3992; this receipt at 441/3994). Both have been refreshed at this release-notes finalize cycle to the current 444/4018 truth, so a reader of the closure PR sees a single, current count. Receipt retains its prior figure inline as provenance for the two tests this cycle's structural-validation suite added.

## Notes

This cycle is **documentation-only** (per `acceptance-and-verification.md` row 33: verification method "artifact inspection"). No source code, no production tests beyond the structural validation tests. The release notes synthesize facts from the full cycle log (progress.md's ## DONE + ## Cycle log + ## Open blockers) and the 35 prior criteria's 40+ cycle receipts.

The release-notes.md file satisfies the tool-checked REQUIRED_NOTES_SECTIONS regex (verified against `tools/release/check_release_manifest.py`'s own expectation: all 7 sections present, no placeholder text).

## Discovery forwards

None. This cycle's scope is bounded to the release-notes.md artifact and its structural tests.

## Next-cycle plan

Criterion 8.4 (Build version increment) is the final criterion in SD-24 after 8.3 (both fire at the tail end). No hard-stop preconditions. Could dispatch concurrently.
