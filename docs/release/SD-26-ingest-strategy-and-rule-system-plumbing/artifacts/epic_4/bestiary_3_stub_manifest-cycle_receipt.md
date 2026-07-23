# Cycle bestiary_3_stub_manifest — Epic 4 / Criterion 4.4

- **Card ID:** (see kanban step, below)
- **Commit SHA:** f174225b1adb6a6574c06a91790cd81a12052c10
- **Files touched:**
  - `data/stubs/bestiary_3.json` (new — book_stub manifest)
  - `docs/governance/wired-integration-stubs-registry.md` (added entry #0007; updated the
    reserved-entries footer note)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.4 — Book stub manifest for "beginner_box" (dispatch
  brief text), replicating criterion 4.1's landed `book_stub` pattern exactly for the next
  future-state book.
- **Status:** complete (landed against a substitute book, second fallback — see Notes)
- **Notes:**
  - **Two successive book collisions found and resolved before the final write.** The brief
    named `beginner_box` explicitly.
    1. On the cycle's first `git fetch origin tranche/5-4 && git rebase origin/tranche/5-4`
       (step 1), the rebase pulled in commit `180a6b0` — a concurrent sibling's own criterion
       4.2, which had already landed `data/stubs/beginner_box.json` + registry entry `#0005`.
       Re-derived the canonical 21-book list from `content-unit-inventory.md §2.2` /
       `scope-draft.md §1.4`, confirmed `advanced_race_guide` (4.1), `adventurers_guide` (4.3),
       and `beginner_box` (4.2) were all claimed, and picked `bestiary_2` — the next unclaimed
       book in list order — as first fallback. Verified `bestiary_2` as a real book against
       `~/workspace/repos/pcgen/.../bestiary_2/bestiary_2.pcc` (`SOURCELONG:Bestiary 2`,
       `SOURCEDATE:2010-12`), wrote `data/stubs/bestiary_2.json` + registry entry `#0006`,
       committed locally (2 commits).
    2. Before pushing, re-ran `git fetch origin tranche/5-4` (step 6's pre-push fetch+rebase) and
       found `origin/tranche/5-4` had moved again: commit `9a3c1bd`, a **different** concurrent
       sibling's own criterion 4.5, had landed `data/stubs/bestiary_2.json` + registry entry
       `#0006` for the exact same book (`bestiary_2`) moments earlier (their `registered_at`
       timestamp `2026-07-23T00:58:51Z` vs. this cycle's local, unpushed
       `2026-07-23T00:56:17Z` — earlier wall-clock but later to actually land on `origin`).
       Rather than force a conflicting duplicate `#0006` onto the shared registry, discarded the
       two local `bestiary_2`-targeting commits entirely (`git reset --hard origin/tranche/5-4`
       — safe: neither commit had been pushed anywhere, confirmed via `git status` clean +
       `git log` showing both commits as strictly local before the reset) and re-derived the
       next unclaimed book from the now-current remote state: `advanced_race_guide`,
       `adventurers_guide`, `beginner_box`, and `bestiary_2` all claimed (`#0003`-`#0006`);
       `bestiary_3` next in list order and still unclaimed.
  - **Second-fallback book identity verified against real source, not guessed.** Read
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_3/bestiary_3.pcc`
    (and its `_bestiary_3_for_players.pcc` companion) directly: `SOURCELONG:Bestiary 3`,
    `SOURCESHORT:B3`, `SOURCEDATE:2012-01` — confirms a real Paizo PF1 sourcebook, "Bestiary 3"
    (2012), not a guessed or fabricated title. `data/corpus/bestiary_3/` does not exist anywhere
    in this repo, confirming the "not yet ingested" claim is genuine.
  - **Re-verified a third time immediately before the final shared-file edit.** After the reset,
    re-ran `git fetch origin tranche/5-4 && git rebase origin/tranche/5-4` once more (picked up
    one additional sibling progress.md commit, `d64f4f0`, no new registry entries), then
    re-grepped `^### ` headings (still topped out at `#0006`) and re-listed `data/stubs/` (still
    only the 4 already-claimed books, no `bestiary_3.json`) before writing entry `#0007` and
    `bestiary_3.json` — no third collision had landed by that point.
  - **JSON manifest shape.** Followed the pilot's exact shape, no deviation: `{book_id:
    "bestiary_3", book_name: "Bestiary 3", planned_resolution_bundle: "SD-27+ (unscheduled)",
    content_kind_counts: null, registered_at: "2026-07-23T01:04:12Z"}`. Validated via
    `python3 -c "import json; json.load(open(...))"` — parses cleanly.
  - **Registry entry.** Entry `#0007` follows the established seven-field `book_stub` template
    field-by-field, including the same operator-verbatim justification citation (`README.md §3`,
    2026-07-21 17:39:26) and the same `Remediation cycle` value (`SD-27+ (unscheduled)`).
    `Bundle-of-record` cites this cycle's own criterion number (4.4), per the convention already
    established by `#0005` (`beginner_box`, cites 4.2) and `#0006` (`bestiary_2`, cites 4.5) —
    each entry cites the criterion that actually landed it, not the book named in that
    criterion's original dispatch brief. Updated the reserved-range footer note to `0008-000n` /
    "16 remaining" / "criteria 4.6-4.22" and named all five now-claimed books explicitly.
  - **Audit gate run against the actual committed diff, not just the working tree.** `BASE_BRANCH
    ...HEAD` diffs commit refs only, so ran the gate once as a pre-change baseline (before any
    edits, `OK_*` trivially since nothing was staged), then again after this cycle's final commit
    (`f174225`) to actually validate the landed diff. Both `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`
    post-commit.
  - **No cargo test suite** (markdown/JSON authoring criterion, same as every prior 4.x book-stub
    cycle); verification was JSON-schema-shape validation (Python `json.load`) + structural
    comparison against `#0003`'s landed template, plus the `grep` checks in "Verification" below.
  - **Receipt filename.** `beginner_box_stub_manifest-cycle_receipt.md` was already claimed by
    the criterion-4.2 sibling and `bestiary_2_stub_manifest-cycle_receipt.md` was already claimed
    by the criterion-4.5 sibling (both confirmed present on disk after the final rebase). Wrote
    this receipt as `bestiary_3_stub_manifest-cycle_receipt.md` to match the book actually landed
    and avoid any collision.
- **Discovery forwards:** none new beyond what 4.1's receipt already forwarded
  (`decisions.md §10`'s `"SD-27"` vs. the brief/pilot's `"SD-27+ (unscheduled)"` for
  `planned_resolution_bundle` — this cycle followed the pilot's already-landed value for
  consistency, unchanged).
  - Worth flagging separately (process observation, not a doctrine gap): three concurrent E4
    cycles independently attempting `beginner_box` → `bestiary_2` → landing on three different
    books in the same short window (this cycle's own two collisions plus the criterion-4.5
    sibling's own) confirms the per-cycle "re-verify immediately before the shared-file edit,
    retry on rejection" protocol is working as designed under real concurrent load, but also that
    dispatch briefs naming a specific book for a specific criterion number will very likely keep
    colliding at this concurrency level for the remaining books — each future cycle should expect
    to fall back at least once and budget for it.
- **Next-cycle plan:** Remaining unclaimed future-state books per the canonical 21-book list:
  `bestiary_4, bestiary_5, bestiary_6, bonus_bestiary, core_essentials, horror_adventures,
  monster_codex, mythic_adventures, occult_adventures, pathfinder_unchained, ultimate_campaign,
  ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness`
  (16 books, entries `#0008` onward, criteria 4.6-4.22 nominal numbering). Each next cycle should
  re-verify the live registry + `data/stubs/` immediately before landing (same discipline used
  here), and re-verify a second time immediately before push, since criterion numbers in dispatch
  briefs and actual book assignments have now diverged repeatedly under concurrent dispatch.

## Verification

```
$ python3 -c "import json; json.load(open('data/stubs/bestiary_3.json'))" && echo VALID_JSON
VALID_JSON

$ grep -n "0007" docs/governance/wired-integration-stubs-registry.md
125:### 0007 — `book_stub`: `bestiary_3` not yet ingested

$ grep -n "bestiary_3" data/stubs/bestiary_3.json docs/governance/wired-integration-stubs-registry.md
data/stubs/bestiary_3.json:2:  "book_id": "bestiary_3",
docs/governance/wired-integration-stubs-registry.md:125:### 0007 — `book_stub`: `bestiary_3` not yet ingested
docs/governance/wired-integration-stubs-registry.md:127:- **Book / manifest path:** `bestiary_3` — `data/stubs/bestiary_3.json`
```
