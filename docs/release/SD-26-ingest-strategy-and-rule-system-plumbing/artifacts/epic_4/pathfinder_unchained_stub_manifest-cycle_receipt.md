# Cycle pathfinder_unchained_stub_manifest — Epic 4 / Criterion 4.16

- **Card ID:** `t_6b750489` (board `codex-tranche-5`, assignee `operator`; a duplicate `t_a65d91f3` created by a CLI-output parsing mistake was archived, not completed)
- **Commit SHA:** `a61305f` (manifest + registry entry #0017), `224fdaa` (progress.md)
- **Files touched:**
  - `data/stubs/pathfinder_unchained.json` (new — per-book stub manifest for `pathfinder_unchained`)
  - `docs/governance/wired-integration-stubs-registry.md` (added registry entry #0017 for `pathfinder_unchained`; updated the reserved-entries footer note to reflect 6 remaining books/#0018-000n)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.16 — Book stub manifest for `pathfinder_unchained`. Replicate criterion 4.1's landed `book_stub` pattern exactly (not redesigned): write `data/stubs/pathfinder_unchained.json` in the pilot's exact shape and add the next-numbered registry entry following #0003's template.
- **Status:** complete
- **Notes:**
  - **Replicated, did not redesign.** Used `content-unit-inventory.md §2.1`'s shape exactly:
    `{book_id, book_name, planned_resolution_bundle, content_kind_counts: null,
    registered_at: <ISO-8601>}`. Verified by direct key-order/type comparison against the pilot
    `data/stubs/advanced_race_guide.json` (`python3 -c "import json; ..."` — see Verification
    below).
  - **Book identity verified against real source, not guessed.** Checked
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/pathfinder_unchained/_pathfinder_unchained.pcc`:
    `CAMPAIGN:Pathfinder Unchained`, `SOURCELONG:Pathfinder Unchained`, `SOURCESHORT:PU`,
    `PUBNAMELONG:Paizo Inc.`, `SOURCEDATE:2015-04` — a real Paizo PF1 hardcover supplement
    (April 2015, "an indispensable companion to the Pathfinder Roleplaying Game: Core Rulebook").
    The directory also has real LST content files confirming coverage: `pu_abilities_class.lst`,
    `pu_abilities_race.lst`, `pu_abilitycategories.lst`, `pu_companionmods.lst`,
    `pu_datacontrols.lst`, `pu_equip.lst`, `pu_equipmods.lst`, `pu_feats.lst`, `pu_skills.lst`,
    `pu_spells.lst`, `pu_templates.lst`, plus a `support/` subdirectory with cross-book ability
    tie-ins (`ability_um.lst`, `ability_arg.lst`, `ability_apg.lst`, `ability_acg.lst`,
    `ability_uc.lst`). `book_name` set to `"Pathfinder Unchained"` (the `SOURCELONG`/`CAMPAIGN`
    value, matching the convention every prior `book_stub` entry used — e.g. `"Horror
    Adventures"`, `"Mythic Adventures"`, `"Monster Codex"`). None of this content is ingested
    into `data/corpus/pathfinder_unchained/` (directory does not exist), confirming the stub is
    honest.
  - **Registry-number collision on push, resolved during rebase.** At the initial rebase (before
    implementation) the highest registry heading was `#0015` (`mythic_adventures`), and
    `pathfinder_unchained` was unclaimed, so this cycle wrote and committed entry `#0016`. On the
    push-time re-fetch+rebase, a concurrent sibling cycle had landed `occult_adventures` as `#0016`
    (criterion 4.15, commit `f31d833`) first, producing a rebase content conflict on the shared
    registry file (both branches inserted a `### 0016` heading after `#0015`). Resolved by keeping
    the sibling's `occult_adventures` as `#0016` unchanged and renumbering this cycle's entry to
    `#0017`, then correcting the reserved-entries footer note to "6 remaining books"/`#0018-000n`
    (folding `occult_adventures` into the "done by" list alongside `pathfinder_unchained`). No
    content was lost or overwritten from either side.
  - **Entry number (#0017, final)** determined by the rebase-time conflict resolution above —
    the first assumption (`#0016`) was invalidated by a genuine concurrent write, not assumed in
    advance per the brief's explicit instruction; re-verified against the registry on disk after
    resolving the conflict, immediately before `rebase --continue` and push.
  - **`planned_resolution_bundle` value.** Used `"SD-27+ (unscheduled)"`, matching the pilot's
    (#0003's) corrected value and every subsequent `book_stub` entry (#0004-#0015), not
    `decisions.md §10`'s originally-pinned `"SD-27"` literal — consistent with the discrepancy
    resolution already recorded by criterion 4.1's receipt.
- **Verification:**
  - `python3 -c "import json; ..."` — confirmed `data/stubs/pathfinder_unchained.json` is valid
    JSON with the exact same key set and key order as the pilot `advanced_race_guide.json`
    (`book_id`, `book_name`, `planned_resolution_bundle`, `content_kind_counts`, `registered_at`),
    `content_kind_counts` is JSON `null` (not `0` or a string), and `book_id` /
    `planned_resolution_bundle` hold the expected literal values.
  - `grep -n "pathfinder_unchained" docs/governance/wired-integration-stubs-registry.md` —
    confirms the new #0017 entry landed with the `book_stub` structural pattern (all seven fields:
    Book / manifest path, What's missing, Justification, Audit-grep impact, Bundle-of-record,
    Remediation cycle, Status) matching #0003-#0016's template field-by-field.
  - `cargo test --locked --test sd26_identifier_discipline_audit` — 1/1 pass
    (`no_bundle_tag_identifier_leaks_in_scripts_and_data`).
  - Dual-audit gate re-run against the new diff: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`.
- **Discovery forwards:** none new this cycle (the `decisions.md §10` / `"SD-27"` vs.
  `"SD-27+ (unscheduled)"` discrepancy was already forwarded by criterion 4.1's receipt).
- **Next-cycle plan:** 7 future-state books remain unclaimed as of this cycle
  (`occult_adventures`, `ultimate_campaign`, `ultimate_combat`, `ultimate_equipment`,
  `ultimate_intrigue`, `ultimate_magic`, `ultimate_wilderness`, plus criteria 4.12/4.15 labels
  that may already be claimed by the time the next cycle starts — re-derive the actual unclaimed
  set from `data/stubs/` + the registry on disk at cycle start, per the established pivot
  protocol from prior cycles' receipts).
