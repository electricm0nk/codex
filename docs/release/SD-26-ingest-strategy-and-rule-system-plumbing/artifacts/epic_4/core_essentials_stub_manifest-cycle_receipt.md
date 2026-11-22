# Cycle core_essentials_stub_manifest — Epic 4 / Criterion 4.10

- **Card ID:** (see kanban step, below)
- **Commit SHA:** cf7b84fc284f4b0351d9a624f6c3adf8b2b62d83
- **Files touched:**
  - `data/stubs/core_essentials.json` (new — book_stub manifest)
  - `docs/governance/wired-integration-stubs-registry.md` (added entry #0012; updated the
    reserved-entries footer note)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.9 — Book stub manifest for "bestiary_6" (dispatch
  brief text), replicating criterion 4.1's landed `book_stub` pattern exactly for the next
  future-state book.
- **Status:** complete (landed against a substitute book, `core_essentials` — see Notes)
- **Notes:**
  - **Dispatch brief was stale before this cycle's first write.** The brief named `bestiary_6`
    for "criterion 4.9." On the first re-fetch+rebase (step 1), live `tranche/5-4` already carried
    `bestiary_6` fully registered (entry `#0010`, criterion 4.8, commit `b7ab111`, per
    `progress.md` row 26). Re-derived the canonical 21-book list from
    `content-unit-inventory.md §2.2` (`advanced_race_guide, adventurers_guide, beginner_box,
    bestiary_2, bestiary_3, bestiary_4, bestiary_5, bestiary_6, bonus_bestiary, core_essentials,
    ...`) and picked `bonus_bestiary` — the next book after `bestiary_6` in list order — as the
    first candidate. Wrote `data/stubs/bonus_bestiary.json` locally, then on the pre-registry-edit
    re-fetch+rebase (step 7's discipline, applied early) the rebase itself failed with "untracked
    working tree files would be overwritten by checkout" because a concurrent sibling had already
    landed the identical `bonus_bestiary.json` on `origin/tranche/5-4` (entry `#0011`, criterion
    4.9, commit `bdaf39b`, `registered_at: 2026-07-23T02:24:12Z` — earlier than this cycle's own
    unwritten timestamp). Deleted the local untracked file, re-ran the rebase cleanly (picked up
    the sibling's `bonus_bestiary` commits), re-derived the next unclaimed book a second time
    (`core_essentials`, the book after `bonus_bestiary` in canonical list order, confirmed
    unclaimed both in the registry — highest entry `#0011` — and in `data/stubs/`), and landed
    against it instead as criterion 4.10, entry `#0012`. No data loss: the discarded
    `bonus_bestiary.json` was never committed or pushed.
  - **Book identity verified against real source, not guessed.** Read
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/_core_essentials.pcc`
    directly: `CAMPAIGN:Core Essentials`, `PUBNAMELONG:Paizo Inc.`, `TYPE:Paizo Publishing.Pathfinder
    RPG`, `STATUS:RELEASE` — confirms a real Paizo PF1 free rules-reference sourcebook, "Core
    Essentials" (most metadata lines in this particular `.pcc` are `#`-commented out in the
    upstream PCGen data, but the active `CAMPAIGN:`/`PUBNAMELONG:`/`TYPE:` directives plus the
    commented `#SOURCELONG:Core Essentials` / `#SOURCEDATE:2009-08` still unambiguously identify
    the real book and its August 2009 date). `data/corpus/core_essentials/` does not exist
    anywhere in this repo, confirming the "not yet ingested" claim is genuine.
  - **JSON manifest shape.** Followed the pilot's exact shape, no deviation: `{book_id:
    "core_essentials", book_name: "Core Essentials", planned_resolution_bundle: "SD-27+
    (unscheduled)", content_kind_counts: null, registered_at: "2026-07-23T02:28:25Z"}`. TDD'd with
    a Python schema-assertion script: RED confirmed `FileNotFoundError` before the write; GREEN
    confirmed key set, `book_id`, `book_name == "Core Essentials"`, `planned_resolution_bundle ==
    "SD-27+ (unscheduled)"`, `content_kind_counts is None`, and ISO-8601 `registered_at` all pass
    after the write.
  - **Registry entry.** Entry `#0012` follows the established seven-field `book_stub` template
    field-by-field, including the same operator-verbatim justification citation (`README.md §3`,
    2026-07-21 17:39:26) and the same `Remediation cycle` value (`SD-27+ (unscheduled)`). Entry
    number `#0012` determined at edit time by re-fetch+rebase then grepping the registry on disk
    for the highest existing `### NNNN` heading (`#0011`), not assumed in advance. Updated the
    reserved-range footer note to `0013-000n` / "11 remaining" / "criteria 4.11-4.22" and named all
    ten now-claimed books explicitly.
  - **No cargo test suite** (markdown/JSON authoring criterion, same as every prior 4.x book-stub
    cycle); verification was the Python JSON-schema-shape assertion above plus structural
    comparison against `#0003`/`#0010`'s landed templates, plus the `grep` checks in "Verification"
    below.
  - **Push:** single-attempt success (no non-fast-forward rejection at push time — the collision
    this cycle hit was caught earlier, at the rebase step before the registry edit, not at push).
- **Discovery forwards:** none new beyond what 4.1's/4.8's/4.9's receipts already forwarded
  (`decisions.md §10`'s `"SD-27"` vs. the brief/pilot's `"SD-27+ (unscheduled)"` for
  `planned_resolution_bundle` — this cycle followed the pilot's already-landed value for
  consistency, unchanged). Echoing prior receipts' flag for operator review: at this fan-out level,
  a fresh fetch+rebase immediately before writing a shared-name file is necessary but sometimes
  still not sufficient — this cycle's `bonus_bestiary` pick collided with a sibling that had
  pushed in the narrow window between this cycle's own step-1 rebase and its write. The rebase
  step's hard failure (untracked-file-would-be-overwritten) is itself a reliable, cheap collision
  detector and was caught before any commit, so the protocol absorbed it with zero data loss.
- **Next-cycle plan:** Remaining unclaimed future-state books per the canonical 21-book list:
  `horror_adventures, monster_codex, mythic_adventures, occult_adventures, pathfinder_unchained,
  ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic,
  ultimate_wilderness` (11 books, entries `#0013` onward, criteria 4.11-4.22 nominal numbering).
  Each next cycle should re-verify the live registry + `data/stubs/` immediately before landing,
  and again immediately before push, per the discipline used here.

## Verification

```
$ python3 /tmp/.../verify_core_essentials_stub.py
GREEN: all assertions passed

$ grep -n "0012" docs/governance/wired-integration-stubs-registry.md
175:### 0012 — `book_stub`: `core_essentials` not yet ingested

$ grep -n "core_essentials" data/stubs/core_essentials.json docs/governance/wired-integration-stubs-registry.md
data/stubs/core_essentials.json:2:  "book_id": "core_essentials",
docs/governance/wired-integration-stubs-registry.md:175:### 0012 — `book_stub`: `core_essentials` not yet ingested
docs/governance/wired-integration-stubs-registry.md:177:- **Book / manifest path:** `core_essentials` — `data/stubs/core_essentials.json`
```
