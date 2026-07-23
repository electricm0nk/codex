# Cycle bestiary_4_stub_manifest — Epic 4 / Criterion 4.6

- **Card ID:** (see kanban step, below)
- **Commit SHA:** (filled in after push — see `progress.md` for the landed SHA)
- **Files touched:**
  - `data/stubs/bestiary_4.json` (new — book_stub manifest)
  - `docs/governance/wired-integration-stubs-registry.md` (added entry #0008; updated the
    reserved-entries footer note)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.6 — Book stub manifest for "bestiary_3" (dispatch
  brief text), replicating criterion 4.1's landed `book_stub` pattern exactly for the next
  future-state book.
- **Status:** complete (landed against a substitute book, `bestiary_4` — see Notes)
- **Notes:**
  - **Collision found before any write, and resolved by pivoting to the next unclaimed book.**
    The brief named `bestiary_3` explicitly and gave step-by-step instructions to replicate the
    #0003 pilot for it. Before starting, `git worktree list` showed a sibling worktree
    (`wf_0ebaeb25-4cc-4`) with a local commit `f174225` already producing
    `data/stubs/bestiary_3.json` + registry entry `#0007` for `bestiary_3` (their own criterion
    label "4.4"), not yet pushed to `origin/tranche/5-4` at that point. Rather than race a
    guaranteed duplicate registry entry for the exact same `book_id` onto the shared registry
    (a substantive collision, not just a mergeable diff), re-derived the canonical 21-book list
    from `content-unit-inventory.md §2.2` and picked `bestiary_4` — the next unclaimed book in
    list order (`advanced_race_guide, adventurers_guide, beginner_box, bestiary_2, bestiary_3,
    bestiary_4, ...`) — before doing any work, following the exact precedent already established
    by the 4.2 and 4.4 cycles' own receipts (both document the identical "re-verify, pivot to
    next unclaimed book" pattern).
  - **Confirmed by re-fetch immediately before the shared-file edit.** After a first
    `git fetch origin tranche/5-4 && git rebase origin/tranche/5-4` (step 1) that only picked up
    unrelated commits, ran a second `git fetch origin tranche/5-4 && git rebase
    origin/tranche/5-4` immediately before writing the registry entry (step 7's discipline,
    applied proactively) and found `origin/tranche/5-4` now *did* carry `#0007` /
    `bestiary_3.json` (the sibling had pushed in the interim) — confirming the pre-emptive pivot
    to `bestiary_4` was correct and avoiding a guaranteed conflict/duplicate-book situation
    entirely rather than discovering it at push time.
  - **Book identity verified against real source, not guessed.** Read
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_4/_bestiary_4.pcc`
    directly: `SOURCELONG:Bestiary 4`, `SOURCESHORT:B4`, `SOURCEDATE:2013-10`, `CAMPAIGN:Bestiary
    4` — confirms a real Paizo PF1 sourcebook, "Bestiary 4" (2013), with real LST content files
    (`b4_races.lst`, `b4_templates.lst`, `b4_feats.lst`, `b4_equip_magic_items.lst`, etc.).
    `data/corpus/bestiary_4/` does not exist anywhere in this repo, confirming the "not yet
    ingested" claim is genuine.
  - **JSON manifest shape.** Followed the pilot's exact shape, no deviation: `{book_id:
    "bestiary_4", book_name: "Bestiary 4", planned_resolution_bundle: "SD-27+ (unscheduled)",
    content_kind_counts: null, registered_at: "2026-07-23T01:07:36Z"}`. TDD'd with a small Python
    schema-assertion script: RED confirmed the file didn't exist / validator failed with
    `FileNotFoundError` before the write; GREEN confirmed key set, `book_id`, non-empty
    `book_name`, `planned_resolution_bundle == "SD-27+ (unscheduled)"`, `content_kind_counts is
    None`, and ISO-8601 `registered_at` all pass after the write.
  - **Registry entry.** Entry `#0008` follows the established seven-field `book_stub` template
    field-by-field, including the same operator-verbatim justification citation (`README.md §3`,
    2026-07-21 17:39:26) and the same `Remediation cycle` value (`SD-27+ (unscheduled)`).
    `Bundle-of-record` cites this cycle's own criterion number (4.6), per the convention already
    established by `#0005`-`#0007` (each entry cites the criterion that actually landed it, not
    the book named in that criterion's original dispatch brief). Updated the reserved-range
    footer note to `0009-000n` / "15 remaining" / "criteria 4.7-4.22" and named all six now-claimed
    books explicitly.
  - **No cargo test suite** (markdown/JSON authoring criterion, same as every prior 4.x book-stub
    cycle); verification was the Python JSON-schema-shape assertion above plus structural
    comparison against `#0003`/`#0007`'s landed templates, plus the `grep` checks in
    "Verification" below.
  - **Receipt filename.** `bestiary_3_stub_manifest-cycle_receipt.md` was already claimed by the
    criterion-4.4 sibling (confirmed present on disk after the final rebase, with a commit SHA
    already stamped). Wrote this receipt as `bestiary_4_stub_manifest-cycle_receipt.md` to match
    the book actually landed and avoid any collision, per the same convention that receipt itself
    used.
- **Discovery forwards:** none new beyond what 4.1's receipt already forwarded
  (`decisions.md §10`'s `"SD-27"` vs. the brief/pilot's `"SD-27+ (unscheduled)"` for
  `planned_resolution_bundle` — this cycle followed the pilot's already-landed value for
  consistency, unchanged).
  - Worth flagging again (process observation, consistent with 4.4's own receipt): dispatch
    briefs naming a specific book for a specific criterion number keep colliding with concurrent
    siblings at this fan-out level (17-20 parallel per-book cycles against one shared registry).
    This cycle avoided a collision only because the sibling's work was visible locally
    (`git worktree list` across the shared host) before either side pushed; a cycle running on a
    host without that visibility would have had to discover the collision via rebase conflict
    instead. Not a doctrine gap — the retry/pivot protocol already covers it — but worth the next
    operator review noting that pre-flight `git worktree list` is a cheap collision-avoidance step
    worth keeping in the standard procedure for this fan-out shape.
- **Next-cycle plan:** Remaining unclaimed future-state books per the canonical 21-book list:
  `bestiary_5, bestiary_6, bonus_bestiary, core_essentials, horror_adventures, monster_codex,
  mythic_adventures, occult_adventures, pathfinder_unchained, ultimate_campaign, ultimate_combat,
  ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness` (15 books, entries
  `#0009` onward, criteria 4.7-4.22 nominal numbering). Each next cycle should re-verify the live
  registry + `data/stubs/` (and sibling worktrees, where visible) immediately before landing, and
  again immediately before push, per the discipline used here.

## Verification

```
$ python3 /tmp/.../validate_stub.py data/stubs/bestiary_4.json bestiary_4
GREEN: schema OK for data/stubs/bestiary_4.json

$ grep -n "0008" docs/governance/wired-integration-stubs-registry.md
135:### 0008 — `book_stub`: `bestiary_4` not yet ingested

$ grep -n "bestiary_4" data/stubs/bestiary_4.json docs/governance/wired-integration-stubs-registry.md
data/stubs/bestiary_4.json:2:  "book_id": "bestiary_4",
docs/governance/wired-integration-stubs-registry.md:135:### 0008 — `book_stub`: `bestiary_4` not yet ingested
docs/governance/wired-integration-stubs-registry.md:137:- **Book / manifest path:** `bestiary_4` — `data/stubs/bestiary_4.json`
```
