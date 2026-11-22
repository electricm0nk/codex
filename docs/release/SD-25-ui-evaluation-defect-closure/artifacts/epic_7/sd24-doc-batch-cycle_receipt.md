# Cycle SD-25-E7-DOC-BATCH — Epic 7 / Criterion 7.P (SD-24 Documentation-Staleness Batch)

- **Card ID:** t_b8c34aba
- **Commit SHA:** 4d4d18e
- **Files touched:**
  - `../SD-24-beta-readiness-and-multiclass/content-unit-inventory.md` (B1, B2, B3, B9)
  - `../SD-24-beta-readiness-and-multiclass/technical-design.md` (B10, B11)
  - `../SD-24-beta-readiness-and-multiclass/epic-breakdown.md` (B4, B14)
  - `../SD-24-beta-readiness-and-multiclass/acceptance-and-verification.md` (B9)

- **Identifier audit result:** OK_TRIVIAL_NO_PRODUCT_CODE (docs-only changes; grep excludes documentation paths)
- **Wired-integration audit result:** OK_TRIVIAL_NO_PRODUCT_CODE (docs-only changes; grep excludes documentation paths)

- **Acceptance criterion:** Per cycle doc: "All 10 in-scope items corrected; receipt lists item → file → correction applied; no other files touched."

- **Status:** complete

- **Corrections applied:**

| Item | File | Correction |
|------|------|-----------|
| **B1** | `content-unit-inventory.md §3.2` (line 82) | Clarified CRB uses split structure (`class_tables.rs` shared + `level_up/<class>.rs` per-class), not single per-class files like APG/ACG. Added note: "(corrected 2026-07-22 per SD-25 criterion 7.P: CRB uses split structure...)" |
| **B2** | `content-unit-inventory.md §3.2` (line 84) | Clarified APG uses 6 separate per-class files (`apg/class_alchemist.rs`...`apg/class_witch.rs`) + shared `apg/mod.rs`, opposite pattern from CRB. Added note. |
| **B3** | `content-unit-inventory.md §3.2` (line 84) | Clarified ACG uses one file per class (`acg/class_arcanist.rs`...`acg/class_warpriest.rs`) + shared `acg/mod.rs`. Combined into same note as B2. |
| **B4** | `epic-breakdown.md` criterion 4.3 (line 83) | Fixed ACG class roster: removed "Alchemist-side" (APG-only class, not ACG), added "Slayer" (real ACG class). Correct 10-class roster: Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest. Added note. |
| **B6** | N/A — no planning-doc citations found | SD-24 carry-forward register notes measurement error (675 CRB spells → 652 actual due to `.COPY=` variant miscounts + header line). No citations of "675" exist in `epic-breakdown.md` or `content-unit-inventory.md`. Progress.md documents the correction; planning docs do not cite the wrong number. Verify-only, no correction needed. |
| **B7** | N/A — no planning-doc citations found | SD-24 register notes measurement errors (APG: 341 → 338 equipment, 298 → 297 spells due to `SOURCELONG:` header miscounts + duplicates). No citations of these numbers exist in planning docs. Progress.md documents corrections; planning docs do not cite wrong numbers. Verify-only, no correction needed. |
| **B9** | `content-unit-inventory.md` (lines 92-94) + `acceptance-and-verification.md` (line 35) | Corrected test file references: Tauri command tests are inline using `#[cfg(test)] mod tests`, not standalone `tests/sd24_characterhub_append.rs` / `tests/sd24_characterhub_recompute.rs` / `tests/sd24_characterhub_resave.rs` files (no standalone `apps/desktop/src-tauri/tests/` directory exists). Added note clarifying inline pattern. |
| **B10** | `technical-design.md §3.1` (line 50) | Clarified the 7.1 gap was narrower than initially framed: SD-23 already landed `level_up_character`, `add_equipment_selection`, `add_spell_selection` (per duracon `e74f3fa`, `f203df8`); the real 7.1 gap was batch-append + real-corpus validation only. Added note. |
| **B11** | `technical-design.md §3.2` (lines 55-64) | Corrected `reSaveCharacter` request shape from sketch `{ characterId, character: CharacterSnapshot }` (which doesn't exist as a wire type) to real implementation `{ characterId, expectedRevisionId, savedAt, ruleSystemId }`. No `CharacterSnapshot` serde impl exists; real request uses revision-conflict-guard shape per duracon 2026-07-18 18:20:41. Added note. |
| **B14** | `epic-breakdown.md` criterion 7.5 (lines 162-165) | Fixed file path: criterion 7.5 "Files touched" was `apps/desktop/src-tauri/src/characterHub/characterHubRuntime.ts` (doesn't exist); real path is `apps/desktop/src/characterHub/characterHubRuntime.ts` (frontend TS, not src-tauri). Also clarified `compose_character_input` is a Rust function at `character_hub.rs:211`, not in the TS file. Added note. |

- **Notes:**

1. **B5, B8, B12 verify-only:** Per cycle doc, items B5 (crb/equipment_tables.rs doc comment stale), B8 (apg/equipment_tables.rs doc comment stale), B12 (7.4 RED text stale) were already corrected in-cycle during SD-24. Not re-corrected; noted here for audit trail. No changes made to product code files for these items.

2. **B6, B7 finding verification:** Both items reference measurement errors discovered during SD-24's Epic 6 cycles (inventory miscounts due to `.COPY=` variant deduplication, `SOURCELONG:` header rows, duplicate records). These corrections are recorded in SD-24's `progress.md` (rows 45-46) and in the actual ingested corpus (spell/equipment counts are correct in Rust tables). Planning docs (`epic-breakdown.md`, `content-unit-inventory.md`, `technical-design.md`, `acceptance-and-verification.md`) do not cite the wrong numbers (675/341/298) anywhere. No corrections to planning docs were needed for B6-B7.

3. **B13 (open question Q6):** Register notes B13 as "open question, default no action" — no correction needed. Deferred to operator decision.

4. **Dual-audit gated pass:** Both identifier-discipline and wired-integration audits pass trivially on docs-only changes. Product-code audits excluded documentation paths per normal operation. No bundle-tag identifiers or forbidden tokens present.

- **Discovery forwards:** none

- **Next-cycle plan:** Epic 7 cycle progression continues. This batch cycle completes the documentation-staleness corrections for all 41 DISCOVERED items from SD-24 (per SD-24 carry-forward register's 33 real follow-on work items + 8 process/tooling lessons). SD-25 Epic 7 proceeds to next priority item in queue.

---

**Verification checklist:**

- [x] All 10 in-scope items (B1-B4, B6-B7, B9-B11, B14) reviewed against register §B
- [x] Each item verified against live repo before correction written
- [x] B6-B7 (no planning-doc citations) identified and noted for audit trail
- [x] B5, B8, B12 (already corrected) noted for completeness
- [x] Dual-audit gate passed (docs-only changes, trivial OK)
- [x] Commit SHA recorded
- [x] Files touched listed per item
- [x] Correction notes appended with 2026-07-22 date per correction-style mandate
- [x] No product code touched; file-touch grant honored
