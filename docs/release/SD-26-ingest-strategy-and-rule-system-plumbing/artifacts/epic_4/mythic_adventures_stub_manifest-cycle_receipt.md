# Cycle mythic_adventures_stub_manifest — Epic 4 / Criterion 4.14

- **Card ID:** t_76778792 (board `codex-tranche-5`, assignee `operator`)
- **Commit SHA:** 7575aa8 (manifest + registry entry), e353ea1 (progress.md)
- **Files touched:**
  - `data/stubs/mythic_adventures.json` (new — per-book stub manifest)
  - `docs/governance/wired-integration-stubs-registry.md` (added entry #0015 for `mythic_adventures`)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.14 — "Book stub manifest for `mythic_adventures`." One cycle per future-state book; per cycle writes `data/stubs/<book>.json` + Stubs Registry entry (`epic-breakdown.md` §Criteria 4.2..4.22).
- **Status:** complete
- **Notes:**
  - **Live-state re-derivation before claiming a number (per the documented pattern from 4.9-4.13's
    collision fixes).** After `git fetch origin tranche/5-4 && git rebase origin/tranche/5-4`,
    confirmed via `ls data/stubs/` and `grep` on the registry that 12 books were already landed
    (`advanced_race_guide` through `horror_adventures`, plus `monster_codex` at entry #0014 /
    criterion 4.13) and the highest registry heading on disk was `#0014`. `mythic_adventures`
    itself was unclaimed. Re-fetched/rebased a second time immediately before editing the shared
    registry file and re-confirmed `#0014` was still the highest heading before writing entry
    `#0015` — no collision on this cycle's single push attempt.
  - **Book identity verified against real source, not guessed.**
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/mythic_adventures/_mythic_adventures.pcc`
    confirms `CAMPAIGN:Mythic Adventures`, `SOURCELONG:Mythic Adventures`, `SOURCESHORT:MA`,
    `SOURCEDATE:2013-08`, `BOOKTYPE:Supplement` — a real Paizo PF1 hardcover supplement
    (Pathfinder Roleplaying Game: Mythic Adventures, August 2013). `data/corpus/mythic_adventures/`
    does not exist (confirmed via `ls`), consistent with `content_kind_counts: null` in the
    manifest. Used the short form `"Mythic Adventures"` for `book_name` (matching `SOURCELONG`
    and the sibling entries' convention of dropping the "Pathfinder Roleplaying Game:" series
    prefix, e.g. `advanced_race_guide` → `"Advanced Race Guide"`).
  - **JSON manifest shape.** Followed the pilot (`data/stubs/advanced_race_guide.json`) and every
    landed sibling (`#0003`-`#0014`) exactly, no deviation: `{book_id: "mythic_adventures",
    book_name: "Mythic Adventures", planned_resolution_bundle: "SD-27+ (unscheduled)",
    content_kind_counts: null, registered_at: "2026-07-23T02:46:04Z"}`. Verified via
    `python3 -c "import json; ... assert set(d.keys())==expected_keys"` — valid JSON, correct key
    set and values.
  - **Registry entry.** Entry `#0015` follows `#0003`-`#0014`'s seven-field template exactly
    (Book/manifest path, What's missing, Justification, Audit-grep impact, Bundle-of-record,
    Remediation cycle, Status), same operator-verbatim citation (`README.md §3`, operator
    directive 2026-07-21 17:39:26), same `SD-27+ (unscheduled)` remediation-cycle value matching
    the manifest's own field. Updated the trailing reserved-entries footer note
    (`0016-000n reserved for the remaining 8...`, appending `mythic_adventures` done by 4.14 to
    the "minus" list), matching the pattern every prior 4.x cycle used to keep the footer
    accurate.
  - No cargo/JS test suite applies (markdown/JSON authoring criterion, consistent with every prior
    4.x cycle). Verification was JSON-schema shape assertion + structural-pattern comparison
    against `#0003`/`#0014`, plus the dual-audit grep gate (both `OK_*`).
- **Discovery forwards:** none.
- **Next-cycle plan:** future-state books remaining unclaimed after this cycle (per
  `content-unit-inventory.md §2.2`'s 21-book list, minus the 14 now landed: `advanced_race_guide,
  adventurers_guide, beginner_box, bestiary_2, bestiary_3, bestiary_4, bestiary_5, bestiary_6,
  bonus_bestiary, core_essentials, horror_adventures, monster_codex, mythic_adventures`):
  `occult_adventures, pathfinder_unchained, ultimate_campaign, ultimate_combat,
  ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness`. The next cycle
  should re-derive live state the same way (fetch/rebase, `ls data/stubs/`, grep the registry's
  highest heading) rather than trusting any assigned book/criterion-number label, since concurrent
  siblings land continuously.
