# Cycle ultimate_magic_stub_manifest — Epic 4 / Criterion 4.21

- **Card ID:** (see kanban step, below)
- **Commit SHA:** `06222fa` (landed on `tranche/5-4`; also see `progress.md` for the recorded SHA)
- **Files touched:**
  - `data/stubs/ultimate_magic.json` (new — per-book stub manifest, following the #0003 pilot's
    exact shape)
  - `docs/governance/wired-integration-stubs-registry.md` (added entry `#0022` — `book_stub`:
    `ultimate_magic` — following #0003-#0021's field-by-field template exactly; updated the
    reserved-entries footer note to `0023-000n` / "remaining 1")
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.21 — Book stub manifest for `ultimate_magic`
  (`epic-breakdown.md` §"Criteria 4.2..4.22 — One cycle per future-state book"): writes
  `data/stubs/ultimate_magic.json` + the corresponding Stubs Registry entry.
- **Status:** complete
- **Notes:**
  - **Replicated, did not redesign.** Per the brief, this cycle strictly copies criterion 4.1's
    landed pattern (`research_book_stub_kind-cycle_receipt.md`, entry #0003, and
    `data/stubs/advanced_race_guide.json`), following the same field-by-field template every
    intervening cycle (#0004-#0021) also used, substituting only `book_id`/`book_name`/entry
    number/criterion number/date/manifest path. No new field-shape or wording decisions were
    made.
  - **Book identity verified against real source data, not guessed.** Checked
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_magic/` directly:
    `_ultimate_magic.pcc` confirms `CAMPAIGN:Ultimate Magic`, `SOURCELONG:Ultimate Magic`,
    `SOURCESHORT:UM`, a Paizo Pathfinder RPG supplement (`BOOKTYPE:Supplement`),
    `SOURCEDATE:2011-05`, with real LST content files present (`um_classes.lst`,
    `um_feats.lst`, `um_spells.lst`, `um_abilities_class.lst`, `um_abilities_race.lst`,
    `um_abilities_companion.lst`, `um_abilities_wordsofpower.lst`, `um_domains.lst`,
    `um_equip_arms_armor.lst`, `um_equip_general.lst`, `um_kits.lst`, `um_kits_companion.lst`,
    `um_templates.lst`, `um_races_companion.lst`, `um_profs_shield.lst`, `um_profs_weapon.lst`,
    etc.) — confirming the book genuinely exists and covers class/feat/spell/domain/
    ability/equipment/kit/template/companion content, none of which has been ingested into
    `data/corpus/ultimate_magic/` in this repo. The PCC's `DESC:` field ("Unlock the magical
    mysteries of the Pathfinder RPG... comprehensive 256-page hardcover reference... arcane
    secrets... ki-tricks... alchemist mutagens... oracle mysteries... channel energy options")
    was used to write the registry entry's one-line description. Used `book_name: "Ultimate
    Magic"` (matching the PCC's `SOURCELONG` exactly).
  - **`registered_at` timestamp.** Used the actual current UTC time at manifest-write
    (`2026-07-23T03:19:03Z`), not a copy of the pilot's or a sibling's timestamp — each
    book_stub manifest records its own real registration moment.
  - **Entry number determined at edit time, not pre-assumed.** Re-fetched + rebased onto
    `origin/tranche/5-4` immediately before editing the shared registry file (a sibling cycle's
    `ultimate_intrigue` progress.md update had landed in between the first and second fetch),
    then grepped the file on disk for the highest existing `### NNNN` heading. At edit time
    entries `#0003` through `#0021` had already landed (covering `advanced_race_guide` through
    `ultimate_intrigue`) and `ultimate_magic` was not yet registered — confirmed `#0022` was the
    next free number and `ultimate_magic` genuinely unclaimed before writing.
  - **`content_kind_counts: null` (not `0`).** Per the pilot's established rationale, carried
    forward unchanged: `null` honestly signals "no counting pass has run," `0` would falsely
    claim a completed count of zero.
  - **Footer-note judgment call: criterion-to-book count mismatch flagged, not resolved.**
    After this cycle, 20 of the 21 future-state books are registered; only `ultimate_wilderness`
    remains unregistered, but two criterion numbers remain open (`4.12`, `4.22`) — one too many
    for one remaining book. `content-unit-inventory.md`'s "Pre-existing count discrepancy"
    section documents that the bundle originally scoped 22 future-state books/criteria and was
    corrected to 21 books but the correction did not fully renumber criteria, leaving `4.12`
    without a mapped book in every entry landed so far (`#0003`-`#0022` map only to `4.1`,
    `4.2`-`4.11`, `4.13`-`4.21`). Updated the reserved-entries footer note to flag this
    explicitly (one of `{4.12, 4.22}` is expected to register `ultimate_wilderness`; the other is
    a likely no-op left over from the miscount) rather than silently picking one, since which
    criterion number is the "real" one is an operator call, not something this cycle's brief
    authorized deciding. Not self-healable inline; flagged for the next cycle / operator.
  - **JSON shape verified programmatically.** Ran a Python assertion confirming the manifest's
    key set and value types/format match `data/stubs/advanced_race_guide.json`'s pilot shape
    exactly (RED: no `ultimate_magic.json` and no registry entry existed before this cycle;
    GREEN: both written, assertion + grep both pass).
- **Discovery forwards:** the criterion-4.12-orphan / `ultimate_wilderness`-mapping ambiguity
  noted above — forwarding to `progress.md` `## DISCOVERED` for the next cycle / operator to
  resolve which of `{4.12, 4.22}` actually registers `ultimate_wilderness`.
- **Next-cycle plan:** the one remaining future-state book, `ultimate_wilderness`, should pick the
  next free `data/stubs/<book>.json` + next free registry entry number (`#0023`) at edit time,
  under whichever of criteria `4.12`/`4.22` the operator (or the next cycle's own judgment,
  absent operator input) resolves to, following this cycle's (and #0003-#0022's) template. After
  that, all 21 future-state books are registered and Epic 4's dynamic per-book criteria are
  closed out.
