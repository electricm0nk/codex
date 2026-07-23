# Cycle occult_adventures_stub_manifest — Epic 4 / Criterion 4.15

- **Card ID:** t_b2690598 (board `codex-tranche-5`, assignee `operator`)
- **Commit SHA:** f31d833 (manifest + registry entry)
- **Files touched:**
  - `data/stubs/occult_adventures.json` (new — per-book stub manifest)
  - `docs/governance/wired-integration-stubs-registry.md` (added entry #0016 for `occult_adventures`; updated the trailing reserved-entries footer note)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.15 — "Book stub manifest for `occult_adventures`." One cycle per future-state book; per cycle writes `data/stubs/<book>.json` + Stubs Registry entry (`epic-breakdown.md` §Criteria 4.2..4.22).
- **Status:** complete
- **Notes:**
  - **Live-state re-derivation before claiming a number.** After `git fetch origin tranche/5-4 &&
    git rebase origin/tranche/5-4`, confirmed via `grep` on the registry that 15 books were already
    landed (`advanced_race_guide` through `mythic_adventures`, highest heading `#0015` /
    criterion 4.14) and `occult_adventures` itself was unclaimed. Re-fetched/rebased a second
    time immediately before editing the shared registry file and re-confirmed `#0015` was still
    the highest heading before writing entry `#0016` — no collision on this cycle's single push
    attempt.
  - **Book identity verified against real source, not guessed.** Read
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/occult_adventures/_occult_adventures.pcc`,
    confirming `CAMPAIGN:Occult Adventures`, `SOURCELONG:Occult Adventures`, `SOURCESHORT:OA`,
    `SOURCEDATE:2015-07`, `BOOKTYPE:Supplement` — a real Paizo PF1 hardcover supplement
    (Pathfinder Roleplaying Game: Occult Adventures, July 2015; introduces the kineticist,
    medium, mesmerist, occultist, psychic, and spiritualist base classes plus the psychic magic
    system). `data/corpus/occult_adventures/` does not exist in this repo, consistent with
    `content_kind_counts: null` in the manifest. Used the short form `"Occult Adventures"` for
    `book_name` (matching `SOURCELONG` and the sibling entries' convention of dropping the
    "Pathfinder Roleplaying Game:" series prefix, e.g. `advanced_race_guide` →
    `"Advanced Race Guide"`).
  - **JSON manifest shape.** Followed the pilot (`data/stubs/advanced_race_guide.json`) and every
    landed sibling (`#0003`-`#0015`) exactly, no deviation: `{book_id: "occult_adventures",
    book_name: "Occult Adventures", planned_resolution_bundle: "SD-27+ (unscheduled)",
    content_kind_counts: null, registered_at: "2026-07-23T02:52:17Z"}`. Verified via
    `python3 -c "import json; ... assert set(d.keys())==expected_keys"` — valid JSON, correct key
    set and values.
  - **Registry entry.** Entry `#0016` follows `#0003`-`#0015`'s seven-field template exactly
    (Book/manifest path, What's missing, Justification, Audit-grep impact, Bundle-of-record,
    Remediation cycle, Status), same operator-verbatim citation (`README.md §3`, operator
    directive 2026-07-21 17:39:26), same `SD-27+ (unscheduled)` remediation-cycle value matching
    the manifest's own field. Updated the trailing reserved-entries footer note
    (`0017-000n reserved for the remaining 7...`, appending `occult_adventures` done by 4.15 to
    the "minus" list), matching the pattern every prior 4.x cycle used to keep the footer accurate.
  - No cargo/JS test suite applies (markdown/JSON authoring criterion, consistent with every prior
    4.x cycle). Verification was JSON-schema shape assertion + structural-pattern comparison
    against `#0003`/`#0015`, plus the dual-audit grep gate (both `OK_*`), plus a direct `grep` for
    `occult_adventures` confirming both the manifest and registry entry landed.
- **Discovery forwards:** none.
- **Next-cycle plan:** future-state books remaining unclaimed after this cycle (per
  `content-unit-inventory.md §2.2`'s 21-book list, minus the 15 now landed: `advanced_race_guide,
  adventurers_guide, beginner_box, bestiary_2, bestiary_3, bestiary_4, bestiary_5, bestiary_6,
  bonus_bestiary, core_essentials, horror_adventures, monster_codex, mythic_adventures,
  occult_adventures`): `pathfinder_unchained, ultimate_campaign, ultimate_combat,
  ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness`. The next cycle
  should re-derive live state the same way (fetch/rebase, `ls data/stubs/`, grep the registry's
  highest heading) rather than trusting any assigned book/criterion-number label, since concurrent
  siblings land continuously.
