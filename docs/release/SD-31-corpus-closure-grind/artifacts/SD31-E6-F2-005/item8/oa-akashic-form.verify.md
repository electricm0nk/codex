# item-8 on-screen verification — PASS (captured manually)

- verdict: **PASS**
- family: `spell` · record: `Akashic Form` (`occult_adventures:spell:akashic_form`)
- expected on screen: `Akashic Form` — present
- expected on screen: `Akashic Record` — present (in the rendered description)
- agent: `sd31-e6-f2-005` · date: 2026-08-16T12:30:00Z
- HEAD: `b8c36417d`
- harness: manual (`apps/desktop/.claude/skills/run-desktop/driver.sh`), not
  `verify-on-screen.sh`'s automated path — its own first run this cycle
  landed on a stale "Create a character" screen left over from an earlier
  agent session sharing this box (its own FAILED artifact,
  `oa-akashic-form.FAILED.verify.md`, is kept alongside this file as
  evidence rather than discarded), not a defect in this cycle's own change.
  Clicked "Back" to return to the hub, then drove the Spell Catalog
  navigation directly.

## What this proves

`occult_adventures:spell:akashic_form` is one of the 40 `occult_adventures`
`wiring_class=static`+`status=ingested-magnitude` units this cycle's
`cache_gen::spell_lane_dump` + `enrich_spell_raw_tokens` widening moved to
`literal-verified` (`done`) — the `corpus_literal_sweep`-provable byte match
between `data/corpus/occult_adventures/spell/akashic_form.json`'s
`raw_tokens` and the pinned oracle's `oa_spells.lst:5`. Decision 7 condition
3 requires the description to be present AND rendered, proven on-screen —
not inferred from the green `corpus_literal_sweep`/gate result. The Spell
Catalog screen's own header text confirms the catalog now chains 8 books
("Core Rulebook, Advanced Player's Guide, Advanced Class Guide, Advanced
Race Guide, Ultimate Intrigue, Ultimate Magic, Occult Adventures and
Ultimate Combat") and its own filter chips show `OA (144)`, `UM (269)`,
`UC (146)` — the exact per-book base-declaration counts this cycle's
generator wrote, live evidence the catalog and the newly-written corpus
cache agree on the same population.

## Screenshot

`oa-akashic-form.png` — search box filtered to "Akashic Form", 1 matching
spell, rendering:

> **Akashic Form** OA · Necromancy · Level 9
> Store a copy of your body in the Akashic Record, and restore yourself to
> that form upon your death.

byte-matching `data/corpus/occult_adventures/spell/akashic_form.json`'s own
`data.description` field, which itself byte-matches the pinned oracle's
`oa_spells.lst:5` `DESC:` token via `corpus_literal_sweep` (0 findings,
24,519 examined at this cycle's tip).
