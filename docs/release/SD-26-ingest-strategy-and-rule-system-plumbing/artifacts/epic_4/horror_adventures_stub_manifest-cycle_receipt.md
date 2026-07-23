# Cycle horror_adventures_stub_manifest — Epic 4 / Criterion 4.11

- **Card ID:** (see kanban step, below)
- **Commit SHA:** (filled in after push — see `progress.md` for the landed SHA)
- **Files touched:**
  - `data/stubs/horror_adventures.json` (new — per-book stub manifest)
  - `docs/governance/wired-integration-stubs-registry.md` (added entry #0013 for `horror_adventures`)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criteria 4.2..4.22 — "One cycle per future-state book. Per cycle: writes `data/stubs/<book>.json` + Stubs Registry entry." (`epic-breakdown.md` §Criteria 4.2..4.22). This cycle's dispatch brief named it "Criterion 4.10 — bonus_bestiary," but see Notes for the pivot to criterion 4.11 / book `horror_adventures`.
- **Status:** complete
- **Notes:**
  - **Dispatch collision, resolved by re-deriving live state (same pattern as the 4.9/4.10
    cycles' documented pivots).** This cycle's brief assigned book `bonus_bestiary` under the
    label "criterion 4.10." After the mandatory pre-work `git fetch origin tranche/5-4 && git
    rebase origin/tranche/5-4` (loop-instruction.md §6 step 1), both `bonus_bestiary` (landed as
    entry #0011 / criterion 4.9, commit `bdaf39b`) and the "criterion 4.10" label itself
    (claimed by `core_essentials`, entry #0012, commit `cf7b84f`) were already present on disk in
    `data/stubs/` and in the registry — confirmed by `grep`/`ls` before writing anything, per the
    brief's own instruction to "use the next free number after whatever is actually on disk at
    edit time, not a number you assumed in advance."
  - **Re-derivation.** Re-read `content-unit-inventory.md §2.2`'s canonical 21-book list
    (`advanced_race_guide, adventurers_guide, beginner_box, bestiary_2, bestiary_3, bestiary_4,
    bestiary_5, bestiary_6, bonus_bestiary, core_essentials, horror_adventures, monster_codex,
    mythic_adventures, occult_adventures, pathfinder_unchained, ultimate_campaign,
    ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic,
    ultimate_wilderness`), cross-checked every name against `ls data/stubs/` (10 already present:
    all of the list through `core_essentials`), and landed on `horror_adventures` — the first
    genuinely unclaimed book in list order. Re-fetched/rebased a second time immediately before
    editing the shared registry file (step 6/7 discipline) and re-confirmed `horror_adventures`
    was still unclaimed and the highest existing registry heading was still `#0012` before
    writing entry `#0013`. No collision encountered on this cycle's single push attempt.
  - **Book identity verified against real source, not guessed.**
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/horror_adventures/*.pcc`
    confirms `CAMPAIGN:Horror Adventures`, `SOURCELONG:Horror Adventures`, `SOURCESHORT:HA`,
    `SOURCEDATE:2016-08` — a real Paizo PF1 supplement. `data/corpus/horror_adventures/` does not
    exist (confirmed via `ls`), consistent with `content_kind_counts: null` in the manifest.
  - **JSON manifest shape.** Followed the pilot (`data/stubs/advanced_race_guide.json`) and every
    landed sibling (`#0003`-`#0012`) exactly, no deviation: `{book_id: "horror_adventures",
    book_name: "Horror Adventures", planned_resolution_bundle: "SD-27+ (unscheduled)",
    content_kind_counts: null, registered_at: "2026-07-23T02:33:17Z"}`. Verified via
    `python3 -c "import json; ... assert list(d.keys())==[...]"` — valid JSON, correct key order
    and key set.
  - **Registry entry.** Entry `#0013` follows `#0003`-`#0012`'s five-sentence-field template
    exactly (Book/manifest path, What's missing, Justification, Audit-grep impact,
    Bundle-of-record, Remediation cycle, Status), same operator-verbatim citation
    (`README.md §3`, operator directive 2026-07-21 17:39:26), same `SD-27+ (unscheduled)`
    remediation-cycle value matching the manifest's own field.
  - No cargo test suite applies (markdown/JSON authoring criterion, consistent with every prior
    4.x cycle). Verification was JSON-schema shape assertion + structural-pattern comparison
    against `#0003`/`#0012`, plus the dual-audit grep gate (both `OK_*`).
- **Discovery forwards:** none.
- **Next-cycle plan:** 10 future-state books remain unclaimed after this cycle
    (`monster_codex, mythic_adventures, occult_adventures, pathfinder_unchained,
    ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic,
    ultimate_wilderness`); the next cycle should re-derive live state the same way rather than
    trusting any assigned book/criterion-number label, since concurrent siblings land
    continuously.
