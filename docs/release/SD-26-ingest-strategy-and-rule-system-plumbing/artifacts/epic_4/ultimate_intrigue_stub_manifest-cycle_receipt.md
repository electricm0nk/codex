# Cycle ultimate_intrigue_stub_manifest — Epic 4 / Criterion 4.20

- **Card ID:** (see kanban step, below)
- **Commit SHA:** (filled in after push — see `progress.md` for the landed SHA)
- **Files touched:**
  - `data/stubs/ultimate_intrigue.json` (new — per-book stub manifest, following the #0003 pilot's
    exact shape)
  - `docs/governance/wired-integration-stubs-registry.md` (added entry `#0021` — `book_stub`:
    `ultimate_intrigue` — following #0003-#0020's field-by-field template exactly; updated the
    reserved-entries footer note to `0022-000n` / "remaining 2")
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.20 — Book stub manifest for `ultimate_intrigue`
  (`epic-breakdown.md` §"Criteria 4.2..4.22 — One cycle per future-state book"): writes
  `data/stubs/ultimate_intrigue.json` + the corresponding Stubs Registry entry.
- **Status:** complete
- **Notes:**
  - **Replicated, did not redesign.** Per the brief, this cycle strictly copies criterion 4.1's
    landed pattern (`research_book_stub_kind-cycle_receipt.md`, entry #0003, and
    `data/stubs/advanced_race_guide.json`), following the same field-by-field template every
    intervening cycle (#0004-#0020) also used, substituting only `book_id`/`book_name`/entry
    number/criterion number/date/manifest path. No new field-shape or wording decisions were
    made.
  - **Book identity verified against real source data, not guessed.** Checked
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue/` directly:
    `_ultimate_intrigue.pcc` confirms `CAMPAIGN:Ultimate Intrigue`, `SOURCELONG:Ultimate
    Intrigue`, a Paizo Pathfinder RPG supplement (`BOOKTYPE:Supplement`), `SOURCEDATE:2016-04`,
    with real LST content files present (`ui_classes.lst`, `ui_feats.lst`, `ui_spells.lst`,
    `ui_skills.lst`, `ui_abilities_class.lst`, `ui_abilities_race.lst`, `ui_equip_*.lst`,
    `ui_equipmods.lst`, `ui_templates.lst`, `ui_kits.lst`, `ui_companionmods.lst`,
    `ui_profs_weapon.lst`, etc.) — confirming the book genuinely exists and covers
    class/feat/spell/skill/ability/equipment/template/kit/companion content, none of which has
    been ingested into `data/corpus/ultimate_intrigue/` in this repo. Used `book_name: "Ultimate
    Intrigue"` (matching the PCC's `SOURCELONG` exactly).
  - **`registered_at` timestamp.** Used the actual current UTC time at manifest-write
    (`2026-07-23T03:14:54Z`), not a copy of the pilot's timestamp — each book_stub manifest
    records its own real registration moment.
  - **Entry number determined at edit time, not pre-assumed.** Re-fetched + rebased onto
    `origin/tranche/5-4` immediately before editing the shared registry file, then grepped the
    file on disk for the highest existing `### NNNN` heading. At edit time entries `#0003`
    through `#0020` had already landed (covering `advanced_race_guide` through
    `ultimate_equipment`, per sibling cycles) and `ultimate_intrigue` was not yet registered —
    confirmed `#0021` was the next free number and `ultimate_intrigue` genuinely unclaimed before
    writing.
  - **`content_kind_counts: null` (not `0`).** Per the pilot's established rationale, carried
    forward unchanged: `null` honestly signals "no counting pass has run," `0` would falsely
    claim a completed count of zero.
  - **No discrepancies found.** The `planned_resolution_bundle` value
    (`"SD-27+ (unscheduled)"`) and the operator-verbatim justification citation both matched the
    pilot's already-resolved defaults exactly; no new judgment call was required for this cycle.
  - **JSON shape verified programmatically.** Ran a Python assertion confirming the manifest's
    key set, key order, and value types match `data/stubs/advanced_race_guide.json`'s pilot shape
    exactly (RED: no `ultimate_intrigue.json` existed before this cycle; GREEN: file written,
    assertion passes).
- **Discovery forwards:** none new (the `decisions.md §10` / `planned_resolution_bundle` wording
  discrepancy was already forwarded by criterion 4.1's receipt).
- **Next-cycle plan:** the remaining future-state books in `content-unit-inventory.md §2.2`'s list
  not yet covered by a landed registry entry (criteria 4.12, 4.21-4.22) should each pick the next
  free `data/stubs/<book>.json` + next free registry entry number at edit time, following this
  cycle's (and #0003-#0021's) template.
