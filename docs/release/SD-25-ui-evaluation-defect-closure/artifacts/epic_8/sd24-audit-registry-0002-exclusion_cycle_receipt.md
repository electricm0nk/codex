# Cycle sd24-audit-registry-0002-exclusion — Epic 8 Closure Epilogue / Precursor (standing test fix)

- **Card ID:** t_6ffc2b84
- **Commit SHA:** `42f972bd027f436debee000a162b38b76d742711`
- **Files touched:**
  - `tests/sd24_wired_integration_audit.rs`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS (test-infra change itself carries no forbidden tokens; verified by running the audit tests themselves, see Notes)
- **Acceptance criterion:** Not a numbered SD-25 criterion — precursor fix for `cargo test` regression discovered ahead of Epic 8 closure. The standing repo-wide tripwire `tests/sd24_wired_integration_audit.rs` (SD-24, not scoped to any single diff) had two failing tests (`no_zero_tolerance_forbidden_tokens_in_shipping_source`, `no_would_strings_in_shipping_source`) because it had not been updated since SD-25 criteria 3.3/3.4 legitimately introduced the operator-approved `StubAdapter` "Would ...; not yet implemented" placeholder (`governance/wired-integration-stubs-registry.md` entry 0002). `cargo test` must pass at HEAD before Epic 8 closure can proceed.
- **Status:** complete
- **Notes:**
  - Read `governance/wired-integration-stubs-registry.md` entry 0002 for the exact approved scope: `apps/desktop/src-tauri/src/stub_adapter.rs` (whole file) plus the three Tauri command files' `resolve_rule_system_adapter` doc comments / `*_via_rule_system_routes_unknown_id_to_stub_adapter` test bodies in `apps/desktop/src-tauri/src/characterHub/{appendToCharacter,recomputeCharacter,reSaveCharacter}.rs`.
  - Added a named exclusion closure `is_registry_0002_stub_adapter_exception` to each of the two failing tests, following the file's existing precedent pattern (`is_plant_growth_full_spell_text`): matches by file path (whole-file for `stub_adapter.rs`) and by the exact matched substring (`"Would render for system ...; not yet implemented"`, or the doc comment's distinctive phrase) for the three characterHub files — never a blanket bypass.
  - **RED (before fix):**
    ```
    test no_would_strings_in_shipping_source ... FAILED
      (8 hits: stub_adapter.rs x7, appendToCharacter.rs:284, reSaveCharacter.rs:298, recomputeCharacter.rs:401)
    test no_zero_tolerance_forbidden_tokens_in_shipping_source ... FAILED
      (9 hits: stub_adapter.rs x8, appendToCharacter.rs:155+284, reSaveCharacter.rs:298, recomputeCharacter.rs:401)
    test result: FAILED. 3 passed; 2 failed
    ```
  - **GREEN (after fix):** `cargo test --test sd24_wired_integration_audit` → `test result: ok. 5 passed; 0 failed`.
  - **Overly-broad-exclusion probe (live-tested then reverted):** temporarily added an unrelated line `// TEMP PROBE (reverted): "Would render unrelated probe string; not yet implemented"` to `apps/desktop/src-tauri/src/main.rs`. Re-ran the two tests: both correctly FAILED, flagging only the injected `main.rs:2` line — proving the exclusion is scoped to registry entry 0002's named file set only, not a blanket bypass. Reverted the probe; `git diff --stat -- apps/desktop/src-tauri/src/main.rs` confirmed clean; re-ran to confirm GREEN again (5 passed).
  - Full `cargo test` run afterward: all suites pass, no other regressions.
  - Updated the test file's own module doc comment with a new "## Exclusion added 2026-07-22 (registry entry 0002, SD-25 criteria 3.3/3.4)" section documenting the change and rationale, following the file's existing documentation style.
- **Discovery forwards:** none — this was itself the fix for a previously-undocumented gap (standing test not updated after 3.3/3.4 landed); no new DISCOVERED items opened.
- **Next-cycle plan:** none required; `cargo test` is green at HEAD. Epic 8 closure work can proceed unblocked.
