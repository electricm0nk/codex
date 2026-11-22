# Cycle adventurers_guide_stub_manifest — Epic 4 / Criterion 4.3

- **Card ID:** (see kanban step, below)
- **Commit SHA:** (filled in after push — see `progress.md` for the landed SHA)
- **Files touched:**
  - `data/stubs/adventurers_guide.json` (new — per-book stub manifest, following the #0003 pilot's
    exact shape)
  - `docs/governance/wired-integration-stubs-registry.md` (added entry `#0004` — `book_stub`:
    `adventurers_guide` — following #0003's field-by-field template exactly; updated the
    reserved-entries footer note to `0005-000n` / "remaining 19")
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.3 — Book stub manifest for `adventurers_guide`
  (`epic-breakdown.md` §"Criteria 4.2..4.22 — One cycle per future-state book"): writes
  `data/stubs/adventurers_guide.json` + the corresponding Stubs Registry entry.
- **Status:** complete
- **Notes:**
  - **Replicated, did not redesign.** Per the brief, this cycle strictly copies criterion 4.1's
    landed pattern (`research_book_stub_kind-cycle_receipt.md`, entry #0003, and
    `data/stubs/advanced_race_guide.json`) substituting only `book_id`/`book_name`/entry
    number/criterion number/date/manifest path. No new field-shape or wording decisions were
    made.
  - **Book identity verified against real source data, not guessed.** Checked
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/adventurers_guide/` directly:
    `_adventurers_guide.pcc` confirms `SOURCELONG:Adventurer's Guide`, a Paizo Pathfinder RPG
    supplement (`BOOKTYPE:Supplement`), `SOURCEDATE:2017-06`, with real LST content files present
    (`ag_classes.lst`, `ag_races.lst`, `ag_spells.lst`, `ag_feats.lst`, `ag_abilities.lst`,
    `ag_equip_*.lst`, `ag_templates.lst`, `ag_companionmods.lst`, etc.) — confirming the book
    genuinely exists and covers class/race/spell/feat/equipment/template/companion content, none
    of which has been ingested into `data/corpus/adventurers_guide/` in this repo. Used
    `book_name: "Adventurer's Guide"` (matching the PCC's `SOURCELONG` exactly, including the
    apostrophe) rather than a guessed title.
  - **`registered_at` timestamp.** Used the actual current UTC time at manifest-write
    (`2026-07-23T00:46:07Z`), not a copy of the pilot's timestamp — each book_stub manifest
    records its own real registration moment.
  - **Entry number determined at edit time, not pre-assumed.** Re-fetched + rebased onto
    `origin/tranche/5-4` immediately before editing the shared registry file, then grepped the
    file on disk for the highest existing `### NNNN` heading (`#0003`, the pilot) to confirm
    `#0004` was the next free number — per the brief's explicit instruction not to assume a
    number in advance, since concurrent sibling cycles for other books may land first. No
    sibling entry had landed yet at edit time.
  - **`content_kind_counts: null` (not `0`).** Per the pilot's established rationale, carried
    forward unchanged: `null` honestly signals "no counting pass has run," `0` would falsely
    claim a completed count of zero.
  - **No discrepancies found.** The `planned_resolution_bundle` value
    (`"SD-27+ (unscheduled)"`) and the operator-verbatim justification citation both matched the
    pilot's already-resolved defaults exactly; no new judgment call was required for this cycle.
- **Discovery forwards:** none new (the `decisions.md §10` / `planned_resolution_bundle` wording
  discrepancy was already forwarded by criterion 4.1's receipt).
- **Next-cycle plan:** the next future-state book in `content-unit-inventory.md §2.2`'s list not
  yet covered by a landed registry entry should pick the next free `data/stubs/<book>.json` +
  next free registry entry number at edit time, following this cycle's (and #0003's) template.
