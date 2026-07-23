# Cycle bestiary_2_stub_manifest — Epic 4 / Criterion 4.5

- **Card ID:** (see kanban step, below)
- **Commit SHA:** (filled in after push — see `progress.md` for the landed SHA)
- **Files touched:**
  - `docs/governance/wired-integration-stubs-registry.md` (added registry entry #0006, `book_stub`: `bestiary_2`; updated the reserved-entries footer note)
  - `data/stubs/bestiary_2.json` (new — criterion 4.5 per-book stub manifest)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Epic 4, Criterion 4.5 — Book stub manifest for `bestiary_2`, following
  criterion 4.1's landed `book_stub` kind + pilot pattern (registry entry #0003,
  `advanced_race_guide`).
- **Status:** complete
- **Notes:**
  - **Book availability re-verified at cycle-start.** Before writing anything, re-fetched
    `origin/tranche/5-4` and grepped the live registry + `data/stubs/`: entries #0003
    (`advanced_race_guide`, 4.1), #0004 (`adventurers_guide`, 4.3), and #0005 (`beginner_box`,
    4.2) were already landed; `bestiary_2` had no manifest or registry entry from any sibling
    cycle. Confirms this brief's assigned criterion/book pairing (4.5 / `bestiary_2`) is still
    live and matches criterion 4.2's receipt's own "Next-cycle plan" ("`bestiary_2` is next after
    `beginner_box`").
  - **Real book identity confirmed against PCGen source, not guessed.** Checked
    `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_2/bestiary_2.pcc`
    directly: `CAMPAIGN:Bestiary 2`, `SOURCELONG:Bestiary 2`, `PUBNAMELONG:Paizo Inc.`,
    `BOOKTYPE:Supplement`, `SOURCEDATE:2010-12`, `DESC:` confirming it as the second Pathfinder
    RPG bestiary ("Go beyond goblins with an army of fantasy's most fearsome foes! ... hundreds
    of different creatures ..."). Real LST content is present in the directory (e.g.
    `b2_races.lst`, `b2_templates.lst`, `b2_abilities_race.lst`, `b2_equip_general.lst`).
    `book_name` in the manifest (`"Bestiary 2"`) is taken directly from `SOURCELONG`, not
    invented.
  - **JSON manifest shape.** Replicated criterion 4.1's pilot shape exactly, no deviation:
    `{book_id, book_name, planned_resolution_bundle, content_kind_counts: null, registered_at}`.
    Verified by direct JSON-schema assertion (see Verification below).
  - **Registry entry.** Added #0006 — the next free number, confirmed by re-fetching
    `origin/tranche/5-4` and grepping the live registry file immediately before editing (highest
    existing heading was `#0005` at that point, not a number assumed in advance). Followed
    #0003/#0004/#0005's seven-field `book_stub` template structurally field-for-field, including
    the same operator-verbatim justification citation and `SD-27+ (unscheduled)` remediation
    cycle value. Updated the reserved-entries footer note from `0006-000n` /
    "remaining 18 ... minus `beginner_box`" to `0007-000n` / "remaining 17 ... minus
    `beginner_box` ... and `bestiary_2`".
- **Discovery forwards:** none new (the `decisions.md §10` `planned_resolution_bundle` wording
  discrepancy was already forwarded by criterion 4.1's receipt; no new discrepancy found this
  cycle).
- **Next-cycle plan:** Per `content-unit-inventory.md §2.2`'s 21-book list, `bestiary_3` is next
  alphabetically after `bestiary_2` (criterion 4.4, `adventurers_guide`'s sibling, is presumably
  already claimed per 4.3's landed entry — the next cycle should re-verify live registry state
  at cycle-start rather than trust a pre-assigned number, same posture as this and the prior two
  cycles). Follow this cycle's and #0003/#0004/#0005's shared template exactly.

## Verification

```
$ python3 -c "
import json
d = json.load(open('data/stubs/bestiary_2.json'))
assert set(d.keys()) == {'book_id','book_name','planned_resolution_bundle','content_kind_counts','registered_at'}
assert d['book_id'] == 'bestiary_2'
assert d['content_kind_counts'] is None
assert d['planned_resolution_bundle'] == 'SD-27+ (unscheduled)'
print('JSON_SHAPE_MATCHES_PILOT: OK')
"
JSON_SHAPE_MATCHES_PILOT: OK

$ grep -n '^### 0006' docs/governance/wired-integration-stubs-registry.md
### 0006 — `book_stub`: `bestiary_2` not yet ingested
```
