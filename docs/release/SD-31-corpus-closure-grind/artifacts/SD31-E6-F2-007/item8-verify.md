# DoD item 8 — on-screen verification, `SD31-E6-F2-007`

Driven directly via `apps/desktop/.claude/skills/run-desktop/driver.sh`
(`RUN_DESKTOP_AGENT=sd31spellfeat`), serialized strictly after both full-gate runs finished (per
the skill's own "do not run concurrently with `scripts/verify.sh`" rule).

1. `01-hub.png` — app launches cleanly at the fully-fixed tip.
2. `02-new-char.png` / `03-created.png` — created a real Dwarf Fighter 1 ("Mythic Tes") through
   the real character-creation flow (not a fixture).
3. `06-load.png` — the character shows up in the real Load Character list alongside prior
   sessions' test characters, proving the save round-trips through disk.
4. `07-sheet.png` — the full character sheet loads.
5. `08-feats.png` — **the Feats tab's own header text reads "Add feats from the real feat catalog:
   2261 feats across 18 books (CRB, APG, ACG, ARG, PU, Uca, Ui, Uw, Uc, Um, Upsi, Ce, Ha, Isr, Oa,
   Iswg, MonsterCodex, Mythic)."** — the exact 2261-record, 18-book figure this cycle derived and
   pinned in every test file, generated live from the real compiled catalog, not hand-typed. The
   same screen already shows two REAL Mythic feats attached from a prior session's test data,
   **Power Attack (Mythic)** and **Weapon Focus (Mythic)**, each rendering its real corpus
   `DESC:`/`BENEFIT:` text (e.g. "*Your attacks are truly devastating. When you use Power Attack,
   you gain a +3 bonus on melee damage rolls instead of +2...*") — proof the Mythic book's records
   were already reaching a player before this cycle's own new addition was even opened.
6. `09-addfeat.png` / `10-search.png` — opened the real "Add Feat" picker and searched
   `Accursed Hex`. **`Accursed Hex (Mythic)` — the exact record traced end to end in this cycle's
   own OPEN-ISSUES.md row 167 and `mythic_adventures decisions.md` doc comment — renders live**:
   *"Mythic · Mythic · Your hexes flare with persistent potency. When you use Accursed Hex to
   target a creature with one of your hexes a second time, that creature must roll its saving
   throw twice and take the lower result."* Byte-for-byte the same prose the raw `ma_feats.lst`
   row carries (`DESC:` + `BENEFIT:` joined), confirmed against the pinned oracle before this
   receipt was written. The picker also correctly marks the record's OWN mythic-upgrade
   prerequisite as unmet for a level-1 Fighter with no Accursed Hex base feat (a sibling row,
   struck through, "Unavailable — requires the Accursed Hex feat") — the prerequisite gate this
   cycle's own `feat_prereqs.rs` fix exercises, live.

Confirms: the app builds and launches cleanly at the fully-merged, fully-fixed tip; the new
Mythic Adventures feat records are reachable through the SAME player surface every other book's
feats already use (no new screen needed); the rendered text is the real corpus prose, not a
placeholder; and the prerequisite gate correctly denies the mythic-upgrade form to a character
who does not hold its base feat.
