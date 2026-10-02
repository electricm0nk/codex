---
canonical: true
owner: operator
bundle_id: SD-37
date: 2026-10-02
---

# SD-37 Content-Unit Inventory

This file holds the bundle's **figures of record** (§1). Every other file in this package cites
them by ID (`CUI F-n`) instead of restating a number without its command. It also holds the
per-content-unit tuple (§3) and the SF book roster with row counts (§2).

**Measurement context.** All figures were derived on 2026-10-02 in the worktree
`/home/ubuntu/workspace/worktrees/codex-sd37` (branch `tranche/17`, HEAD `20bf84a3b2`, which is
`origin/develop`). PCGen figures were taken from the pinned oracle checkout. `$PCGEN_REPO_DIR` and
`$PCGEN_CORPUS_ROOT` were unset at authoring time, so the default checkout was used.
`git -C <checkout> rev-parse HEAD` printed `7f818006e371188e5717fd18d74d18a420747fc6`, which equals
`PCGEN_ORACLE_SHA` in `scripts/pcgen-oracle-pin.env` (**on pin**). In the commands below, `$S`
means `$PCGEN_REPO_DIR/data/starfinder` and `$P` means `$PCGEN_REPO_DIR/data/pathfinder`.

**Two-implementation rule (AGENTS.md "Derive counts with awk, not grep -o").** Any figure that
moves a baseline was derived twice, with independent implementations (shell pipeline vs Python
`os.walk`/`json`), and both results are shown. `grep -o` was not used for any count.

---

## §1 Figures of record

| ID | Figure | Value | Predicate / denominator | Implementation A | Implementation B |
|---|---|---|---|---|---|
| F-1 | SF `.lst` files | **131** | every regular file named `*.lst` under `$S` | `find $S -type f -name '*.lst' \| wc -l` → 131 | Python `os.walk($S)`, `splitext=='.lst'` → 131 |
| F-2 | SF `.pcc` files | **12** | `*.pcc` under `$S` (includes `paizo/master_loader.pcc` and the society `_.pcc`) | `find $S -type f -name '*.pcc' \| wc -l` → 12 | Python `os.walk` → 12 |
| F-3 | SF `.lst` lines | **16,594** newline-terminated; **16,603** records | A counts `\n` bytes; B counts records, including the final unterminated line of 9 files | `find $S -name '*.lst' -print0 \| xargs -0 cat \| wc -l` → 16594; Python `b.count(b'\n')` → 16594 | per-book awk `{l++}` summed → 16603; Python line iteration → 16603 |
| F-4 | SF `.lst`+`.pcc` lines | **17,400** | newline count over both kinds | `find $S \( -name '*.lst' -o -name '*.pcc' \) -print0 \| xargs -0 cat \| wc -l` → 17400 | Python `b'\n'` count → 17400 (lst 16594 + pcc 806) |
| F-5 | PF `.lst` lines (scale only) | **228,220** | newline count, `*.lst` under `$P` (2,939 files) | `find $P -name '*.lst' -print0 \| xargs -0 cat \| wc -l` → 228220 | Python → 228220 |
| F-6 | SF LST data rows | **12,947** total; **12,733** in scope (8 Paizo books); **214** excluded | a line whose first byte is not `#`, space, tab, CR or LF, and which does not start `SOURCELONG/SOURCESHORT/SOURCEWEB/SOURCEDATE`. Rows are **not** unique records: `.MOD` rows and multi-row class level blocks are included | per-book `awk '/^[^#[:space:]]/ && !/^(SOURCELONG\|SOURCESHORT\|SOURCEWEB\|SOURCEDATE)/{r++}'` (§2 table) | Python, same predicate (§2 table); both agree per book |
| F-7 | SF distinct `CLASS:` names | **11** | distinct first fields starting `CLASS:` in every `*classes*.lst` | `find $S -name '*classes*.lst' -print0 \| xargs -0 awk -F'\t' '/^CLASS:/{print $1}' \| sort -u \| wc -l` → 11 | Python set over the same files → 11 |
| F-8 | CRB Theme rows | **67** | non-comment rows of `paizo/core/scr_abilities.lst` that carry a tab field exactly `CATEGORY:Theme` | `awk -F'\t' '/^[^#]/ && /CATEGORY:Theme(\t\|$)/' …/scr_abilities.lst \| awk 'END{print NR}'` → 67 | Python, exact tab-field match → 67 |
| F-9 | formula-system tokens `MODIFY:`/`MODIFYOTHER:` | **SF 1,954 / PF 35** | tab fields that **start** with `MODIFY:` or `MODIFYOTHER:` on non-comment lines of `*.lst` | `find <tree> -name '*.lst' -print0 \| xargs -0 awk -F'\t' '!/^#/{for(i=1;i<=NF;i++) if($i ~ /^MODIFY(OTHER)?:/) c++} END{print c+0}'` (PF output is split over 3 xargs batches: 35+0+0) | Python regex `(?:^\|\t)MODIFY(?:OTHER)?:` → 1954 / 35 |
| F-10 | `src/rules_core/rules_tables` size | **180,883** lines in **250** `.rs` files | newline count over every `*.rs` | `find src/rules_core/rules_tables -name '*.rs' -print0 \| xargs -0 cat \| wc -l`; `… \| wc -l` for files | Python `os.walk` → 250 files, 180883 lines |
| F-11 | `.lst` citation lines in `rules_tables` | **12,529** | lines containing the literal `.lst` | `grep -rcF '.lst' src/rules_core/rules_tables \| awk -F: '{s+=$2} END{print s}'` → 12529 | Python per-line `b'.lst' in line` → 12529 |
| F-12 | files importing `rules_tables::` (outside `rules_tables/`) | **252** = src 72 + crates 92 + apps/desktop/src-tauri 16 + tests 72 | a `*.rs` file containing `rules_tables::`, excluding files under `src/rules_core/rules_tables/` | `for r in src crates apps/desktop/src-tauri tests; do grep -rlE 'rules_tables::' $r --include='*.rs' \| awk '!/src\/rules_core\/rules_tables\//' \| awk 'END{print NR}'; done` | Python `os.walk` per root (skipping `target`, `node_modules`) → 72/92/16/72 |
| F-13 | hardcoded book-list consts | **19** | lines matching `const [A-Z_]*BOOKS[A-Z_]*\s*:` in `src`, `apps/desktop/src-tauri/src`, `crates/codex-ingest/src` | `grep -rnE 'const [A-Z_]*BOOKS[A-Z_]*\s*:' src apps/desktop/src-tauri/src crates/codex-ingest/src --include='*.rs' \| awk 'END{print NR}'` → 19 | Python regex per line → 19 |
| F-14 | PF token-mapping rows | **273** (0 for `MODIFY`/`MODIFYOTHER`/`CHANNEL`, per the engine-reuse report; not re-derived here) | `rows` array of `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json` | `jq '.rows\|length' <file>` → 273 | `python3 -c "import json;print(len(json.load(open('<file>'))['rows']))"` → 273 |
| F-15 | PF work-inventory units | **49,450** | `units` array of `docs/work-inventory.json` | `jq '.units\|length' docs/work-inventory.json` → 49450 | Python `len(json.load(...)['units'])` → 49450 |
| F-16 | `verify.sh` stages | **51** | entries of `ALL_STAGES` | `bash scripts/verify.sh --list \| tail -n +2 \| wc -l` → 51 | `sed -n 110p scripts/verify.sh \| sed 's/.*(\(.*\))/\1/' \| awk '{print NF}'` → 51 |
| F-17 | version-bump surfaces | **14** | files that carry the tranche version | `grep -rlE '0\.16\.0' . --exclude-dir={node_modules,target,.git,docs,.worktrees,graphify-out,data}` → 12, plus `grep -rlF "'0.16.'" apps --exclude-dir=node_modules` → 2 | `git show --name-only --format= 9a650cfd41 \| awk 'NF' \| wc -l` → 14 (the SD-36 bump commit); the two lists name the same 14 files |
| F-18 | Starfinder game-mode files | **10** `.lst` (12 directory entries: plus `base.xml.ftl` and `bio/`) | `$PCGEN_REPO_DIR/system/gameModes/Starfinder` | `find …/gameModes/Starfinder -maxdepth 1 -name '*.lst' \| wc -l` → 10 | `ls …/gameModes/Starfinder \| wc -l` → 12 entries, read by eye: 10 `.lst` |
| F-19 | worktrees on this clone | **11** (10 before this package's worktree) | `git worktree list` lines | `git worktree list \| awk 'END{print NR}'` → 11 | read by eye: main checkout, 2 under `.worktrees/`, `codex-ci-oracle`, 6 `codex-epic-f*`, `codex-sd37` |
| F-20 | merged remote `sd36/*` branches | **7** | `origin/sd36/*` remote-tracking refs | `git branch -r \| grep -c 'origin/sd36/'` → 7 (`-c` counts lines; it is not `-o`) | listed by name: `epic-f1`, `epic-f1c`, `epic-f2-f3`, `epic-f4-f5`, `epic-f6-desktop-polish`, `epic-f7-sheet-visible`, `package` |

**Figures quoted from the research reports and not re-derived here** (marked as quotes; do not
promote them to baselines without re-deriving):
- `pilot_compute` = 106,306 lines, of which 82,457 are in 28 per-PF-class files (engine-reuse report §0).
- `data/sheet_rules` = 56,008 JSON files, 260 MB; `data/corpus` = 51,555 JSON files, 237 MB (engine-reuse report §2).
- 61 PI blacklist terms (engine-reuse report §1(B)).
- Epic F ≈ 10.7 M subagent tokens, weekly quota at 87% before its last batch (SD-36 retrospective, Epic F closure addendum).

**Stale figures found during authoring** (logged in `progress.md` Cycle 0 for a `retro.py
correction` at C0.1, because this package's write scope does not include `docs/retro/`):
- `rules_tables` "181,797 lines" (SD-36 FS-3, scope-draft, memory) → **180,883** (F-10).
- "8,592 `.lst` citations" (SD-36 D6/FS-3): predicate never stated; F-11 (12,529) uses a stated
  predicate. The two are not comparable; the old figure is retired, not corrected.
- "12 files" under `system/gameModes/Starfinder` described as `.lst` (source-inventory report §0) →
  10 `.lst` + 2 non-`.lst` entries (F-18).
- Engine-reuse report's `MODIFY`/`MODIFYOTHER` "2,021 vs 41" and the source-inventory report's
  "`MODIFY:` 1,773 vs 22" are three different predicates; F-9 states its own.

---

## §2 Starfinder book roster (pinned oracle)

Row predicate = F-6. Files = `*.lst` files (Python count; the awk pass reports fewer for four books
because it cannot see empty files: armory 10, COM 17, near_space 15, AA2 13 — the difference is 7
empty `.lst` files). Lines = records (F-3 B). Licence column from the source-inventory report §1
(`grep -nE '^(COPYRIGHT|INFOTEXT|#EXTRAFILE|SOURCE|BOOKTYPE|STATUS)' <pcc>`), to be re-verified by
E0.2.

| Book dir | Code | `.lst` files | Lines | Rows | SD-37 scope | Licence posture (quoted; E0.2 verifies) |
|---|---|---|---|---|---|---|
| `paizo/core` | SCR | 30 | 7,375 | 5,974 | **in — proof book** | Paizo Community Use Policy INFOTEXT + OGL 1.0a COPYRIGHT; `OGL.txt` present |
| `paizo/armory` | SA | 11 | 3,542 | 2,977 | in — go-wide | same |
| `paizo/character_operations_manual` | SCOM | 21 | 2,047 | 1,507 | in — go-wide | same |
| `paizo/pact_worlds` | SPW | 15 | 814 | 527 | in — go-wide | same + Tome of Horrors OGL line |
| `paizo/near_space` | SNS | 16 | 589 | 396 | in — go-wide | same |
| `paizo/alien_archive` | SAA | 12 | 766 | 488 | in — go-wide (races/abilities/spells/equipment as present) | same |
| `paizo/alien_archive_2` | SAA2 | 14 | 719 | 519 | in — go-wide | same |
| `paizo/alien_archive_3` | SAA3 | 5 | 453 | 345 | in — go-wide | same |
| `paizo/starfinder_society_rules` | SSRGG | 5 | 238 | 177 | **excluded** (`decisions.md §6`) | "All Rights Reserved" + Paizo trademark line |
| `lpj_design/infinite_space` | LPJ9304 | 2 | 60 | 37 | **excluded** (`decisions.md §6`) | third-party; licence not verified |
| **Total** | | **131** | **16,603** | **12,947** | in-scope rows **12,733** | |

Sum check: in-scope rows 5,974 + 2,977 + 1,507 + 527 + 396 + 488 + 519 + 345 = 12,733; plus
excluded 177 + 37 = 214; 12,733 + 214 = 12,947 = F-6 total.

**The SF unit denominator is not this table.** Rows include `.MOD` rows and class level blocks.
The real denominator — the Starfinder work inventory, one unit per sheet-reachable record — is an
**E0.3 deliverable**, derived mechanically and summed by a command that fails closed
(`workflow-instruction.md §12`, "Sum the piles"). Until E0.3 lands, every SF "N of M" figure in
this bundle is provisional and must say so.

---

## §3 Per-content-unit tuple

One row per content family the bundle must carry onto the sheet. "Compute" means the value feeds a
sheet total (`decisions.md §5`); "print" means the rule's text is printed with every resolvable
term resolved for this character. Module names marked *(proposed)* do not exist yet; the owning
card names the real file in its receipt.

| Content family | Oracle source (CRB file) | Compute or print | Engine module | Test fixture | Owning card | Desktop surface |
|---|---|---|---|---|---|---|
| Classes (11 names, F-7) | `scr_classes.lst` (+ SCOM classes) | compute BAB, saves, HP, Stamina, Resolve, skill ranks/level, key ability; print class features | generic SF chassis reader *(proposed `src/rules_core/pilot_compute/sf_chassis.rs`)* | SRD-transcribed seed values (`artifacts/epic_0/seed-hand-values.md`, E0.4) | E4.1 | creation + sheet header |
| Races | `scr_races.lst` | compute race HP, ability adjustments, size; print racial traits | SF chassis reader | same | E4.1, E5.1 | race picker |
| Themes (67 CRB rows, F-8) | `scr_abilities.lst` `CATEGORY:Theme` | compute +1 ability, theme class-skill; print theme knowledge and 6/12/18 benefits | SF chassis reader + print path | same | E4.3, E5.1 | theme picker |
| EAC / KAC | `scr_equip.lst` armour rows (`TYPE=EAC_Armor`/`KAC_Armor`), `scr__variables.lst` `AC_EAC`/`AC_KAC` | compute | SF defence *(proposed `sf_defense.rs`)* | seed values | E4.2 | sheet defence block |
| Stamina / HP / Resolve | `scr_classes.lst` `BONUS:HP\|CURRENTMAX` + `HD:1`, `BONUS:HP\|ALTHP`; Resolve has no direct row | compute | SF chassis reader | seed values; E3.3 oracle row per overloaded field | E3.3, E4.1 | sheet header |
| Saves, BAB | `scr_classes.lst` `BONUS:SAVE`/`BONUS:COMBAT\|BASEAB` | compute | reuse `Expr::BaseSave/BaseAttack` | seed values | E4.1 | sheet |
| Skills | `scr_skills.lst` | compute totals (ranks + ability + class-skill bonus + armour check penalty where it applies) | SF chassis reader | seed values (Envoy 3 is the skill-heavy seed) | E4.2 | skills panel |
| Point buy, ability increases | game mode `pointbuymethods_system.lst` | compute | desktop `abilityScoreMethods.ts` SF table + engine | seed values | E4.3, E6.2 | creation flow |
| Feats | `scr_feats.lst` | print; compute only where a feat feeds a total | print path | converter output check | E5.2 | feats panel |
| Spells 0–6 | `scr_spells.lst` (`CLASSES:<Class>=<n>`) | compute spells known/per day and DCs; print spell text | SF spellcasting *(proposed)* | Mystic 5 and Technomancer 5 seeds | E4.4, E5.2 | spells panel |
| Equipment (item level, bulk, credits, upgrade slots, fusions) | `scr_equip*.lst`, `scr_equipmods.lst` | compute bulk/encumbrance and credits totals and armour/weapon numbers that feed EAC/KAC/attacks; print the rest | `src/rules_core/encumbrance.rs`, `src/rules_core/money.rs` generalised per system | seed loadouts | E4.5, E5.3 | equipment tab |
| Augmentations | `TYPE:Cybernetic…` rows | print; compute only numeric bonuses that feed totals; one per body system | print path | converter check | E5.3 | equipment tab |
| Drone (Mechanic) | `CLASS:Drone`, `scr_companionmods.lst` | print | print path | converter check | E5.4 | companion panel |
| Starship | none in the pinned oracle | **not in this bundle** — planned deferral FSR row DEF-1 | — | — | — | — |

---

## §4 Seed characters (merge-check roster)

The fixed roster every merge check renders and every status message reports deltas for
(`decisions.md §9`, `workflow-instruction.md` rule R7). Hand values are an E0.4 deliverable,
transcribed from the Starfinder Reference Document, never from the `.lst` the converter reads.

| Seed | System | Build (operator may replace) | Exercises |
|---|---|---|---|
| Aldric | PF1e | Fighter 3 (existing seed) | byte-identical PF render (E1.4, E4a.4) |
| Elowen | PF1e | Wizard 5 (existing seed) | byte-identical PF render incl. spells |
| SF-Soldier-3 | SF1e | Human Soldier 3 (Str key ability) | full BAB, good Fort/Will, heavy armour EAC/KAC |
| SF-Mystic-5 | SF1e | Lashunta Mystic 5 (Wis key ability) | spells 0–2, Resolve from Wis |
| SF-Technomancer-5 | SF1e | Android Technomancer 5 (Int key ability) | spells 0–2, poor Fort/Ref |
| SF-Envoy-3 | SF1e | Ysoki Envoy 3 (Cha key ability) | 8 + Int skill ranks per level, skill totals |

Display names for the SF seeds are codex-neutral labels; race and theme picks are E0.4's to fix and
log (a pick the oracle cannot express is replaced by another CRB option and logged).
