# Cycle bestiary_6_stub_manifest — Epic 4 / Criterion 4.8

- **Card ID:** (see kanban step, below)
- **Commit SHA:** (stamped below after commit)
- **Files touched:**
  - `data/stubs/bestiary_6.json` (new — book_stub manifest)
  - `docs/governance/wired-integration-stubs-registry.md` (added entry #0010; updated the
    reserved-entries footer note)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.6 — Book stub manifest for "bestiary_3" (dispatch
  brief text), replicating criterion 4.1's landed `book_stub` pattern exactly for the next
  future-state book.
- **Status:** complete (landed against a substitute book, `bestiary_6` — see Notes)
- **Notes:**
  - **Dispatch brief was stale twice over before this cycle's first write.** The brief named
    `bestiary_3` for "criterion 4.6." On the first re-fetch+rebase, live `tranche/5-4` already
    carried both `bestiary_3` (entry `#0007`, sibling's own label "criterion 4.4," commit
    `f174225`) and `bestiary_4` claiming the "criterion 4.6" label itself (entry `#0008`, commit
    `e73b0ac`). Re-derived the canonical 21-book list from `content-unit-inventory.md §2.2`
    (`advanced_race_guide, adventurers_guide, beginner_box, bestiary_2, bestiary_3, bestiary_4,
    bestiary_5, bestiary_6, ...`) and picked `bestiary_5` — the next unclaimed book — as criterion
    4.7, entry `#0009`. Implemented it fully (TDD RED/GREEN, registry entry, receipt) and committed
    locally (`e219f66`), **but on the pre-push `git fetch origin tranche/5-4 && git rebase`, found
    a second concurrent sibling had independently landed the exact same book** (`bestiary_5`,
    commit `69a3f86`, entry `#0009`, `registered_at` timestamp only 5 seconds ahead of mine —
    confirmed via `git show origin/tranche/5-4:data/stubs/bestiary_5.json`). This was a genuine
    add/add content collision on the merge conflict (not a mechanically-mergeable diff, since both
    sides wrote the identically-numbered registry entry for the identical book). Ran `git rebase
    --abort` then `git reset --hard origin/tranche/5-4` (safe — the local `e219f66` commit had
    never been pushed), re-derived the next unclaimed book a second time (`bestiary_6`, the book
    after `bestiary_5` in canonical list order, confirmed unclaimed both in the registry and in
    `data/stubs/`), and landed against it instead as criterion 4.8, entry `#0010`.
  - **Book identity verified against real source, not guessed.** Read
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_6/_bestiary_6.pcc`
    directly: `SOURCELONG:Bestiary 6`, `SOURCESHORT:B6`, `SOURCEDATE:2017-05`, `CAMPAIGN:Bestiary 6
    (Only Player Options Implemented)` — confirms a real Paizo PF1 sourcebook, "Bestiary 6" (May
    2017). `data/corpus/bestiary_6/` does not exist anywhere in this repo, confirming the "not yet
    ingested" claim is genuine.
  - **JSON manifest shape.** Followed the pilot's exact shape, no deviation: `{book_id:
    "bestiary_6", book_name: "Bestiary 6", planned_resolution_bundle: "SD-27+ (unscheduled)",
    content_kind_counts: null, registered_at: "2026-07-23T02:19:40Z"}`. TDD'd with a Python
    schema-assertion script: RED confirmed `FileNotFoundError` before the write; GREEN confirmed
    key set, `book_id`, `book_name == "Bestiary 6"`, `planned_resolution_bundle == "SD-27+
    (unscheduled)"`, `content_kind_counts is None`, and ISO-8601 `registered_at` all pass after the
    write.
  - **Registry entry.** Entry `#0010` follows the established seven-field `book_stub` template
    field-by-field, including the same operator-verbatim justification citation (`README.md §3`,
    2026-07-21 17:39:26) and the same `Remediation cycle` value (`SD-27+ (unscheduled)`). Entry
    number `#0010` determined at edit time by re-fetch+rebase then grepping the registry on disk
    for the highest existing `### NNNN` heading (`#0009`), not assumed in advance. Updated the
    reserved-range footer note to `0011-000n` / "13 remaining" / "criteria 4.9-4.22" and named all
    eight now-claimed books explicitly.
  - **No cargo test suite** (markdown/JSON authoring criterion, same as every prior 4.x book-stub
    cycle); verification was the Python JSON-schema-shape assertion above plus structural
    comparison against `#0003`/`#0009`'s landed templates, plus the `grep` checks in "Verification"
    below.
- **Discovery forwards:** none new beyond what 4.1's receipt already forwarded
  (`decisions.md §10`'s `"SD-27"` vs. the brief/pilot's `"SD-27+ (unscheduled)"` for
  `planned_resolution_bundle` — this cycle followed the pilot's already-landed value for
  consistency, unchanged). Worth flagging again for operator review (echoing 4.6's receipt): at
  this fan-out level, `git worktree list`-based collision avoidance is no longer sufficient by
  itself — this cycle hit a same-book collision with a sibling despite doing everything the
  protocol asks (fresh fetch+rebase before writing, re-derivation of the next unclaimed book from
  the live registry). The retry/pivot protocol correctly absorbed it with zero data loss (both
  attempts were caught before push), but it cost this cycle a full extra RED/GREEN/write cycle.
- **Next-cycle plan:** Remaining unclaimed future-state books per the canonical 21-book list:
  `bonus_bestiary, core_essentials, horror_adventures, monster_codex, mythic_adventures,
  occult_adventures, pathfinder_unchained, ultimate_campaign, ultimate_combat, ultimate_equipment,
  ultimate_intrigue, ultimate_magic, ultimate_wilderness` (13 books, entries `#0011` onward,
  criteria 4.9-4.22 nominal numbering). Each next cycle should re-verify the live registry +
  `data/stubs/` immediately before landing, and again immediately before push, per the discipline
  used here.

## Verification

```
$ python3 /tmp/.../verify_stub.py
GREEN: all assertions passed

$ grep -n "0010" docs/governance/wired-integration-stubs-registry.md
155:### 0010 — `book_stub`: `bestiary_6` not yet ingested

$ grep -n "bestiary_6" data/stubs/bestiary_6.json docs/governance/wired-integration-stubs-registry.md
data/stubs/bestiary_6.json:2:  "book_id": "bestiary_6",
docs/governance/wired-integration-stubs-registry.md:155:### 0010 — `book_stub`: `bestiary_6` not yet ingested
docs/governance/wired-integration-stubs-registry.md:157:- **Book / manifest path:** `bestiary_6` — `data/stubs/bestiary_6.json`
```
