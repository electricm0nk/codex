# Cycle ultimate_equipment_stub_manifest — Epic 4 / Criterion 4.19

- **Card ID:** (see kanban step, below)
- **Commit SHA:** `ee142b1` (manifest + registry entry #0020) — landed on `tranche/5-4` after rebase/push (first push attempt succeeded, no non-fast-forward retries needed).
- **Files touched:**
  - `data/stubs/ultimate_equipment.json` (new — per-book stub manifest for `ultimate_equipment`)
  - `docs/governance/wired-integration-stubs-registry.md` (added registry entry #0020 for `ultimate_equipment`; updated the reserved-entries footer note to reflect 3 remaining books/#0021-000n)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.19 — Book stub manifest for `ultimate_equipment`. Replicate criterion 4.1's landed `book_stub` pattern exactly (not redesigned): write `data/stubs/ultimate_equipment.json` in the pilot's exact shape and add the next-numbered registry entry following #0003's template.
- **Status:** complete
- **Notes:**
  - **Replicated, did not redesign.** Used the pilot's exact shape:
    `{book_id, book_name, planned_resolution_bundle, content_kind_counts: null,
    registered_at: <ISO-8601>}`. Verified by direct key-order/type comparison against the pilot
    `data/stubs/advanced_race_guide.json` (`python3 -c "import json; ..."` — see Verification
    below).
  - **Book identity verified against real source, not guessed.** Checked
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_equipment/ultimate_equipment.pcc`:
    `CAMPAIGN:Ultimate Equipment`, `SOURCELONG:Ultimate Equipment`, `SOURCESHORT:UE`,
    `PUBNAMELONG:Paizo Publishing LLC`, `SOURCEDATE:2012-08`, `PRECAMPAIGN:1,INCLUDES=Advanced
    Player's Guide` — a real Paizo PF1 supplement (per the `.pcc` file's own dependency on the
    Advanced Player's Guide, and the real Paizo product page URL embedded in `SOURCEWEB`). The
    directory also has real LST content files confirming coverage: `ue_abilities.lst`,
    `ue_abilitycategories.lst`, `ue_equip_arms_armor.lst`, `ue_equip_general.lst`,
    `ue_equip_magic_items.lst`, `ue_equipmods.lst`, `ue_kits.lst`, `ue_profs_armor.lst`,
    `ue_profs_weapon.lst`, `ue_skills.lst`, `ue_spells.lst`, `ue_templates.lst`, plus a `_pfs/`
    subdirectory. The `.pcc` `DESC` confirms the book is a 400-page all-in-one equipment/magic-item
    reference. `book_name` set to `"Ultimate Equipment"` (the `SOURCELONG`/`CAMPAIGN` value,
    matching the convention every prior `book_stub` entry used). None of this content is ingested
    into `data/corpus/ultimate_equipment/` (directory does not exist), confirming the stub is
    honest.
  - **Registry entry number (#0020).** Re-fetched + rebased immediately before implementing and
    again immediately before editing the shared registry file (per step 6/§5). At both checks, the
    highest registry heading on disk was `#0019` (`ultimate_combat`, criterion 4.18) and
    `ultimate_equipment` was unclaimed, so this cycle used `#0020` — matching the plan brief's
    expectation. No sibling collision encountered; push succeeded on the first attempt.
  - **`planned_resolution_bundle` value.** Used `"SD-27+ (unscheduled)"`, matching the pilot's
    (#0003's) value and every subsequent `book_stub` entry (#0004-#0019).
- **Verification:**
  - `python3 -c "import json; ..."` — confirmed `data/stubs/ultimate_equipment.json` is valid JSON
    with the exact same key set and key order as the pilot `advanced_race_guide.json` (`book_id`,
    `book_name`, `planned_resolution_bundle`, `content_kind_counts`, `registered_at`),
    `content_kind_counts` is JSON `null` (not `0` or a string), `registered_at` parses as a valid
    ISO-8601 UTC timestamp, and `book_id` / `book_name` / `planned_resolution_bundle` hold the
    expected literal values.
  - `grep -n "ultimate_equipment" docs/governance/wired-integration-stubs-registry.md` — confirms
    the new #0020 entry landed with the `book_stub` structural pattern (all seven fields: Book /
    manifest path, What's missing, Justification, Audit-grep impact, Bundle-of-record,
    Remediation cycle, Status) matching #0003-#0019's template field-by-field.
  - Dual-audit gate (§6 identifier + wired-integration greps) re-run against
    `${BASE_BRANCH}...HEAD` after committing and pushing: both `OK_*` (`OK_NO_BUNDLE_TAGS`,
    `OK_NO_TOKENS`).
- **Discovery forwards:** None new this cycle.
- **Next-cycle plan:** Criteria covering the remaining 3 unclaimed books (tracked in the registry
  footer note) continue the same mechanical replication: pick the next unclaimed book from
  `content-unit-inventory.md §2.2`, write `data/stubs/<book_id>.json` in this exact shape, add the
  next-numbered registry entry (re-verifying the highest entry number on disk immediately before
  editing, per the concurrent-write protocol), and update the reserved-entries footer note.
