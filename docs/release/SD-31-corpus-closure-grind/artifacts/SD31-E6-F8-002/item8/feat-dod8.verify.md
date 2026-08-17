# DoD-8 — SD31-E6-F8-002 (feat 5-book gap lane)

`verify-on-screen.sh` has no `feat` family (families: `equipment`/`spell`/`race_trait`/`monster`/
`companion` only — confirmed by reading the script's own `case "$FAMILY" in` arm), so this record
family was driven directly with `driver.sh`, per this cycle's own dispatch instruction.

- **Agent:** `RUN_DESKTOP_AGENT=sd31-feat-companion`
- **Date/time:** 2026-08-16, ~21:35-21:45 EDT (after the full `verify.sh` gate finished — SKILL.md
  bars running `driver.sh launch` concurrently with `verify.sh`)
- **HEAD:** worktree tip at the time of capture (see `progress.md`'s `SD31-E6-F8-002` receipt for the
  exact commit)
- **Window:** 1920x1200, `DISPLAY=:73`

## Sequence

1. `driver.sh launch` — app up, hub screenshot confirmed live.
2. Hub → "New Character" → Dwarf Fighter 1, named "DoD8 Test", created.
3. Hub → "Load Character" → selected "DoD8 Test" → Load → full character sheet.
4. Character sheet → **Feats** tab. The tab's own live copy reads:
   > "Add feats from the real feat catalog: **1903 feats across 17 books (CRB, APG, ACG, ARG, PU,
   > Uca, Ui, Uw, Uc, Um, Upsi, Ce, Ha, Isr, Oa, Iswg, MonsterCodex)**."
   This is the exact total (`325` gap rows, `5` new books) this cycle's own generator run produced —
   confirmed live, on the player-facing surface, not only in the compiled table or a unit test.
5. **Feats → Add Feat → search "Bleeding Stare"** (`feat-bleeding-stare.png`). Result:
   > **Bleeding Stare** — Oa · General · "Your stare causes your foe to bleed out of its eyes. When
   > you trigger your painful stare, the target takes an amount of bleed damage equal to 1/3 your
   > mesmerist level. Bleed damage from multiple uses of Bleeding Stare doesn't stack."
   > Unavailable — requires 1 of: Mesmerist 5; your classes are fighter 1
   Real corpus prose rendered verbatim (byte-matches `occult_adventures/oa_feats.lst`'s own `DESC:`),
   correctly attributed to book `Oa`, AND the record's own `PREABILITY:...Mesmerist...`-derived
   prerequisite correctly gates it unavailable for a level-1 Fighter with a stated reason — proving
   both the description path and the prerequisite-evaluator path this cycle's new gap rows feed.
6. **Feats → Add Feat → search "Angelbane"** (`feat-angelbane-strike.png`). Result:
   > **Angelbane Strike** — MonsterCodex · General · "You channel the power of your Abyssal patron
   > through your weapon to punish the righteous. ..."
   > Unavailable — requires the Channel Smite feat (you have 0 of the 1 needed)
   A second book (`monster_codex`), same shape: real prose, correct book attribution, correct
   prerequisite gate.

Both records are drawn from `SD31-E6-F8-002`'s own newly-added gap rows (`occult_adventures`/
`monster_codex`, neither book had a feat table before this cycle). Neither leaked raw PCGen syntax
(no stray `%N`, `|`, or `&entity;`), matching `feat_descriptions_are_rendered_and_otherwise_byte_
identical`'s own certification.

## Verdict: PASS

Both the catalog total (1903, live) and two individual new records (one per new book, from two of the
five) render real, correct, non-fabricated content on the player-visible character sheet.
