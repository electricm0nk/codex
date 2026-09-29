# F7 merge-readiness receipt (quota wrap-up mode)

Branch `sd36/epic-f7-sheet-visible` at `9c2b361a6a` vs `tranche/16` at `809b95f769`. Nothing in the
code or the package was edited. Evidence is beside this file (`mr-*`, `census-mr-f7.json`) and under
`../../ui-smoke/f7/{elowen,mr-created,mr-full}/`.

**Verdict: merge-ready. There are 0 blockers.**

## Blocker checks

| Check | Command / evidence | Result |
|---|---|---|
| Printed ability score = engine score | real app, isolated root. Scratch copy of the ui-smoke harness (not committed) with 12 rows: Aldric, Elowen, and 6 created characters with raw scores Str 16, Dex 15, Con 13, Int 10, Wis 11, Cha 9. Results in `ui-smoke/f7/mr-created/results-run{1,2}.json`, spec `spec-run2.json`, screenshots beside them | **8 of 8 match** the CRB racial table plus the engine's Human +2 (48 of 48 scores). Aldric 19/13/14/14/12/8. Elowen 8/14/**13**/18/12/10. Dwarf Fighter 1: 16/15/15/10/13/7. Elf Magus 1: 16/17/11/12/11/9. Halfling Monk 1: 14/17/13/10/11/11. Gnome Cleric 1: 14/15/15/10/11/11. Half-orc Samurai 1: 16/15/13/10/11/9 (see polish P1). Human Wizard 1: 18/15/13/10/11/9 |
| Formula token or broken condition sentence in printed prose | Package grep over `data/sheet_rules` for `CASTERLEVEL`, `min(`, `max(`, `floor(`, `))d`, `If requires`, `no record in the corpus`. `python3 scripts/pcgen_residue_gate.py --check --closure`. The same needles scanned in the full bodyText of 8 app characters (12 rows) and in 6 `--sheet-dump` renders | Package: 0 files, except `CASTERLEVEL` in `_tokens.json`, the provenance token ledger, which is not prose and is the same on tranche/16. Residue gate: `live_hits=0 verdict=PASS`, 70,049 shipped files. App sheets: **0 hits**. Elowen's Fireball prints "1d6 points of fire damage per caster level (maximum 10d6)" (`elowen-spells.png`). Dump LINE rows: 0. EXPL `detail` rows: 47 hold `floor(`/`max(`/`min(`, and tranche/16 has the same 47 (P2) |
| A seed with unallocated skill points | real app | Elowen: "fully allocated", Spellcraft **+12** (5 ranks), `elowen-skills.png`. Aldric: fully allocated (ui-smoke full run) |
| Non-weapon in the Weapons tab | real app, forbid list: Flurry of Blows, Spells (Ray), Spells (Touch), Splash Weapon, Unarmed Strike, Grapple | 0 hits on 8 characters. Elowen: "Also proficient with: Club, Crossbow (Heavy), Crossbow (Light), Dagger, Quarterstaff" (`elowen-weapons.png`) |
| Lowercased slug in a printed requirement | `census-mr-f7.json`, regex `[a-z]+_[a-z_]+ (ranks\|as a class skill)` and lowercase skill-term scan | 0 |
| Printed lines vs tranche/16, 6 builds | `class_census --sheet-dump <b> --with-sheet-rules`, release builds of both trees (the tranche/16 build is a read-only `git archive`, with its own target dir). Builds: fighter:3, wizard:5, monk:5, magus:4+samurai:2, cleric:5, fighter:1. Diff: `mr-render6-t16-vs-head.diff` | 6 of 6 exit 0 on both. **36 changed lines (18 pairs), all one mechanism**: the condition `requires climb/intimidate/swim as a class skill` now prints the skill label `Climb/Intimidate/Swim` (F7c §(b), `level_up_option_filter::skill_label`, which is shared by the catalog condition sentences). 0 EXPL deltas. 0 uncited |
| Census regression | `class_census --json` (release, HEAD), `census-mr-f7.json`, compared leaf by leaf with `census-f7b.json` and `stage-f6/merge-readiness/census-mr.json` | ids 137, computed 63, blocked 0. Prestige mix 68 of 74. Mix panel 185 of 185. 62 differing leaves, all `prestige[*].alone_blocking_diagnostics[1].message`, and all equal after case/punctuation folding (the F7c skill-label rewrite). 0 status changes |
| ui-smoke | `RUN_DESKTOP_AGENT=f7-mr node scripts/ui-smoke/run.mjs` (full spec, 86 rows), `mr-full/ui-smoke-full.log`, `results.json` | **82 of 86 green, 0 red**, 1 blocked, 3 manual (native file dialogs). The blocked row, `campaign-manager-list`, could not reach landing (the same command-channel stall F6 recorded); its `--only` rerun is green (`ui-smoke-rerun-campaign-manager-list.log`). 40 created, 40 deleted, 0 leftover. Isolated roots removed |
| Real store untouched | `~/.local/share/io.electricm0nk.codex` before and after all 4 app runs (`mr-full/real-{before,after}.txt`) | 1 character dir, 15,380 entries, sha256 `38446d8e…dbdda2`, unchanged. App stopped |
| Protected paths | `git diff --name-only tranche/16...HEAD -- data/corpus site`; structural diff re-run against the tranche/16 archive (`mr-structural-diff.txt`); `check_frozen_status.py --check` | 0 files; `verdict=PASS`, F7b 2,317 of 2,317 pinned deltas, records 49,450 -> 49,450; site frozen at 49,450 OK |
| Suites (committed F7 logs) | `f7-root-suite.log`, `f7-desktop-suite.log` (639 passed, 0 failed), `f7-desktop-npm.log` (132 of 132 files), `f7-root-clippy.log` EXIT 0 | green. Not re-run here |

## Polish (not merge-blocking; each one is outside the F7 work list or already on tranche/16)

- **P1. The Half-Orc's (and by the same code path the Half-Elf's) free +2 is not applied.** The
  Half-orc Samurai was created with raw scores and no +2 allocation made in the form. It prints the
  raw scores. The engine's +2 is `apply_human_ability_bonus` (human-only, F7a §1). CRB p.24 gives
  Half-Orcs +2 to one ability score. The printed score matches the engine, so this is an engine or
  create-path defect, not an F7 print defect. This check did not test whether the form requires
  the player to place that +2.
- **P2. Engine explanation text carries formula notation and ids.** 47 EXPL `detail` strings on
  the 6 dumps use `floor(`/`max(`/`min(` or a source variable (`MonkLVL`), and tranche/16 has the
  same 47. The Spells tab prints the Wizard spells-per-day detail verbatim, including the internal
  id `` `class_chassis.wizard.specialist_bonus_slot` `` (`elowen-spells.png`). This is engine-authored
  derivation text, not PCGen prose, and it is not a new delta.
- **P3. The armor check penalty reaches Climb and Swim but not the Dex-based skills.** Elowen
  prints Climb and Swim at −2 (1 rank, Str −1, −2), but Acrobatics, Stealth, Escape Artist and
  Sleight of Hand print the bare Dex +2. Aldric prints Climb +7 (ACP −1 after Armor Training) but
  Acrobatics +1. Whether tranche/16 does the same was not checked. It is not on the F7 list.
- **P4.** Characters created at level 1 were used for Monk, Cleric, Samurai and Wizard, because the
  Level control is a `<select>` that the scratch rows could not drive by id. Ability scores do not
  depend on level, so the ability check stands. Level 5 was covered by the seeds and the dumps.
- **P5.** The scratch spec's first row (`create-character-render`) was red in both scratch runs.
  This is a warm-up artifact: the first row after launch still showed "Loading". The same row is
  green in the full spec run.
