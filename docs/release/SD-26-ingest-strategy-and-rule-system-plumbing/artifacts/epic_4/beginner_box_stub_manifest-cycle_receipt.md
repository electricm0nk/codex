# Cycle beginner_box_stub_manifest — Epic 4 / Criterion 4.2

- **Card ID:** (see kanban step, below)
- **Commit SHA:** (filled in after push — see `progress.md` for the landed SHA)
- **Files touched:**
  - `docs/governance/wired-integration-stubs-registry.md` (added registry entry #0005, `book_stub`: `beginner_box`)
  - `data/stubs/beginner_box.json` (new — criterion 4.2 per-book stub manifest)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.2 — Book stub manifest for the next unclaimed
  future-state book in `content-unit-inventory.md §2.2`'s 21-book list, following criterion 4.1's
  landed `book_stub` kind + pilot pattern.
- **Status:** complete
- **Notes:**
  - **This cycle went through two book choices before landing; documenting both for the
    record.**
    1. **First choice, `advanced_race_guide` (the dispatch brief's literal instruction) —
       rejected.** The brief named `advanced_race_guide` for criterion 4.2. Before writing
       anything, per this repo's `CLAUDE.md` ("read only the repo files... explicitly required by
       the brief"), the three sources the brief itself pointed at were read:
       `artifacts/epic_4/research_book_stub_kind-cycle_receipt.md`, the registry, and
       `data/stubs/advanced_race_guide.json`. All three already existed on `tranche/5-4` —
       landed by criterion 4.1 as its own pilot (registry entry #0003, commit `d6a7c61`). 4.1's
       receipt's own "Next-cycle plan" states explicitly: "4.2 should pick the next book in the
       list, `adventurers_guide`, not re-do `advanced_race_guide`." `progress.md`'s row for
       `4.2..4.22` independently confirmed the same. Concluded the brief's book name was stale
       (most likely drafted before criterion 4.1 landed and picked its own pilot book) and
       switched to `adventurers_guide`.
    2. **Second choice, `adventurers_guide` — also superseded, by a concurrent sibling cycle.**
       Implemented `data/stubs/adventurers_guide.json` + registry entry #0004 for
       `adventurers_guide` and attempted the standard fetch-rebase-push cycle. The rebase hit a
       three-way add/add conflict: a sibling cycle had independently reached the identical
       conclusion (their receipt states the same reasoning almost verbatim) and had already
       landed `adventurers_guide` as registry entry #0004 (commit `6f820ee`, labeled "criterion
       4.3" in their own numbering). Rather than force a duplicate/conflicting entry for a book
       already registered, aborted the rebase, dropped the local `adventurers_guide` commit
       entirely (`git reset --hard`, confirmed no other uncommitted work existed first), and
       re-fetched the now-current `origin/tranche/5-4` to determine the actual next unclaimed
       book.
    3. **Landed choice, `beginner_box`.** Per `content-unit-inventory.md §2.2`'s alphabetical
       21-book list, `beginner_box` is the next entry after `advanced_race_guide` (4.1) and
       `adventurers_guide` (4.3), and — confirmed by re-grepping the registry and `data/stubs/`
       immediately before writing — had no landed manifest or registry entry yet from any
       sibling. Implemented this cycle's actual output against `beginner_box`.
  - **Real book identity confirmed against PCGen source.** Checked
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/beginner_box/` directly:
    `_beginner_box.pcc`-style campaign file confirms `SOURCELONG:Beginner Box`, a Paizo
    Pathfinder RPG supplement (`BOOKTYPE:Supplement`), `SOURCEDATE:2011-10`, `DESC:` describing
    it as the introductory boxed set ("Take your first step into an exciting world of fantasy
    adventure..."), with real LST content present (`bbox_equip_arms_armor.lst`,
    `bbox_equip_magic_items.lst`). `book_name` in the manifest (`"Beginner Box"`) is taken
    directly from `SOURCELONG`, not guessed.
  - **JSON manifest shape.** Replicated criterion 4.1's exact pilot shape, no deviation:
    `{book_id, book_name, planned_resolution_bundle, content_kind_counts: null, registered_at}`.
    Verified by direct JSON-schema assertion (see Verification below).
  - **Registry entry.** Added #0005 (the next free number confirmed by re-fetching
    `origin/tranche/5-4` and grepping the live registry file immediately before editing — highest
    existing heading was `#0004` at that point). Followed #0003/#0004's seven-field `book_stub`
    template structurally field-for-field. Updated the reserved-entries footer note to
    `0006-000n`.
- **Discovery forwards:** none new (the `decisions.md §10` `planned_resolution_bundle` wording
  discrepancy was already forwarded by criterion 4.1's receipt; this cycle's book-choice
  discrepancy is fully resolved, not open, so not re-filed as a `## DISCOVERED` entry — noted here
  and in the cycle summary for the orchestrator so future dispatch briefs for 4.4+ can re-verify
  book availability at cycle-start rather than trusting a pre-assigned book name).
- **Next-cycle plan:** Criteria 4.4-4.22 continue down `content-unit-inventory.md §2.2`'s 21-book
  list (`bestiary_2` is next after `beginner_box`), each re-verifying at cycle-start (not
  pre-assuming) which books are still unclaimed by grepping the live registry + `data/stubs/`
  immediately before writing, then following this cycle's and #0003/#0004's shared template
  exactly.

## Verification

```
$ python3 -c "
import json
d = json.load(open('data/stubs/beginner_box.json'))
assert set(d.keys()) == {'book_id','book_name','planned_resolution_bundle','content_kind_counts','registered_at'}
assert d['book_id'] == 'beginner_box'
assert d['content_kind_counts'] is None
assert d['planned_resolution_bundle'] == 'SD-27+ (unscheduled)'
print('JSON_SHAPE_MATCHES_PILOT: OK')
"
JSON_SHAPE_MATCHES_PILOT: OK

$ grep -n '^### 0005' docs/governance/wired-integration-stubs-registry.md
95(+offset): ### 0005 — `book_stub`: `beginner_box` not yet ingested
```
