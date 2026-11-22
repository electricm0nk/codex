---
canonical: true
owner: operator
bundle_id: SD-37
date: 2026-10-07
---

# SD-37 Release Notes

Starfinder 1e. Every figure in this file is listed under "Figures" as `R-n`, each with the command
that produced it, a second independent implementation, and the value both gave when this card ran.
Where prose below quotes a number from a receipt instead, it says "quoted" and names the receipt.

**Status at time of writing (2026-10-07, `tranche/17`, base `1335bb7248`).** Cards through E7.5 are
`complete` (R-3). Still to run after this file, in order: the architecture truth-up and claims critic
(E7.7), graphify against the final tree (E7.8), and the PR `tranche/17` to `develop` (E7.9), which the
operator merges. The repo's build value is the one in R-1; the published triple `0.17.<run>` resolves
at the first tester publish after the operator merges, and is not decided here.

Re-derive everything in one command (exit 0 only if every figure matches the notes, and the three
planted corruptions are rejected):

```
python3 docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.6_check.py --plant
```

## Summary

SD-37 adds Starfinder 1e as a second game system. The partition work (E1) made package roots, book
registries and the converter system-keyed, with the Pathfinder render byte-identical (R-6, R-7). The
schema (E2) gained additive Starfinder variants and published JSON schemas. The converter (E3) reads
PCGen's Starfinder formula system and converts the eight in-scope books (R-4, R-5). A generic
Starfinder chassis (E4) computes the sheet totals from the converted records, with no per-class or
per-race table, and is checked against SRD hand values for four seed characters (R-12). Printing (E5),
the desktop surfaces (E6), an oracle parity run (E7.1, R-13) and the move of the Pathfinder
`rules_tables` from compiled Rust source to a bundled data package (E4a, R-15) complete it.

## User-Visible Changes

Under the Starfinder 1e chip on the landing page of the desktop app:

- **Create.** A creation flow takes race, theme, class and point buy, with every list, pick and budget
  served by the engine (E6.2). Feats, spells known and gear can be chosen at creation, in the level-up
  dialog and on the sheet, from the engine's pools and catalogs (E6.5a).
- **Sheet.** A saved Starfinder character opens a Starfinder sheet (Stamina, HP, Resolve, EAC and KAC,
  saves, initiative, BAB, skills, spells, credits, bulk, melee and ranged attack, a Weapons table) and
  not the Pathfinder layout. The Pathfinder terms CMB, CMD, Touch and Flat-Footed do not appear in it.
  Every number on it is an engine explanation row, enforced by a test (E6.3).
- **Level up.** The sheet's Level up opens a dialog whose classes, ability increases at levels 5, 10, 15
  and 20, skill ranks, owed picks and changed totals all come from the engine (E6.5).
- **Catalogs.** Six Starfinder catalogs (races, themes, classes, feats, spells, equipment) read
  `data/starfinder-1e/sheet_rules` (E6.4).
- **Print.** The sheet prints the features, feats, spells, carried items and a Mechanic's drone block
  that the converted records grant, with formulas in spell prose written as words (E5).
- **Seeds.** Four Starfinder seeds, a Soldier 3, a Mystic 5, a Technomancer 5 and an Envoy 3, are built
  through the real creation flow by ui-smoke rows and open with sheet totals equal to the SRD hand values
  (R-12, R-14). Quoted from `artifacts/epic_6/E6.MC_cycle_receipt.md`: 6 of 6 seeds (the four above plus
  the two Pathfinder seeds) open in the real app under an isolated `XDG_DATA_HOME`.

Pathfinder users see no change in the two Pathfinder seed renders (R-7).

## Defects Fixed

Found by this bundle's own cards and fixed in it (each is a retro correction or a progress.md
finding; none is a defect in a shipped release):

- The tester-release workflow's version stamp line still read the previous tranche after the bump
  (E6.1, caught by `buildVersionTriple.test.ts`).
- Starfinder's human racial +2 pick had no converted record, so no data said a human chooses it (E6.2).
- Starfinder's global `Default` ability had no record, so `EffectiveLVL` read 0 on every non-drone
  character and per-weapon damage bonuses printed `+0` (E7.1; converter fix, structural diff PASS).
- The level-up dialog held its own constant for how many scores an increase raises, which is a
  hand-kept table; it now reads the engine's preview (E6.MC).
- Six table generators still wrote compiled source into the removed module and had drifted from the
  tables they were meant to produce; they now write the data package and reproduce the shipped tables
  (E4a.4a).
- The E7.5 receipt had been committed at the repository root instead of the package; moved in this
  card (correction event `sd37-e7-6`).

## Operational Notes

- **Bundle size.** The Starfinder data root is tracked files in R-19. PR #394 dropped WiX for a very
  large bundle and NSIS has not been measured with this root; the first tester publish after the merge
  is the first Windows build that carries it.
- **ui-smoke isolation.** The ui-smoke harness runs against an isolated app-data root and refuses the
  real one; the real store's entry count and sha256 were equal before and after (R-14).
- **Package paths.** `data/rules_tables` is the bundled data package; the Rust module
  `src/rules_core/rules_tables` is gone and the package carries no `.lst` citation text (R-15).
- **Licence.** Every Starfinder licence-matrix row has `operator_sign_off` false (R-16). Only the
  operator signs. Product-identity screening redacts a subset of corpus records (R-17, R-18).

## Verification Evidence

The last full run is `bash scripts/verify.sh --show-actuals -j 8` in E7.2, on `2443fb2ac4`: every stage
passed (R-8), with test counts in R-9, no `test result: FAILED` line in any of the four logs (R-10) and
the frontend suite in R-11. Commits after it touched only documents, the baseline floors file and the
E7.5 receipt (`git diff --name-only 2443fb2ac4..1335bb7248 -- . ':!docs' ':!artifacts'` prints
`scripts/verify-baselines.env` only). Logs: `artifacts/epic_7/E7.2_logs/`. Per-card receipts are listed
in `kanban.md` and `receipts.md`. CI evidence (`pr-tests`) belongs to E7.9 and does not exist yet.

Oracle: `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` for every corpus-derived figure.

## Known Issues

- **Drone totals.** A Mechanic's drone prints its terms only. The oracle's drone totals (EAC/KAC,
  initiative, saves, attacks, two skills) have no engine total, and PCGen's drone skill totals look
  wrong against the SRD. Whether drone totals are sheet totals under `decisions.md §5` is an operator
  ruling (progress.md, 2026-10-06, E7.1). Drone modifications past level 1 are not covered (E5.4).
- **Attack-bonus reads in printed prose.** `sf_defense::held` leaves `CharacterFacts::base_attack` at 0,
  so a printed line computed from BAB reads 0 (the Soldier's Deadly Aim line prints "an additional 0").
  A retro deferral is filed; the sheet's attack totals use `sf_attack` and are unaffected (E7.1).
- **Multiclass.** Taking a level in a second class is blocked by the engine's
  `sf_chassis.multiclass_key_ability`; E6.5 names it as not covered.
- **Feat counts** are printed, not enforced (E6.5a).
- **Degraded records.** The converter marks some records degraded, each named by token type in
  `_report.json` (count in R-5, down from 34 at E3.5 as E4 and E5 fixes landed:
  `git show d08635eed8:data/starfinder-1e/sheet_rules/_report.json | jq .degraded_records` prints 34).
- **Excluded content.** Three PCCs are excluded, not converted: the core book's `_society` directory
  (no licence declared), the Starfinder Society Roleplaying Guild Guide (its PCC reads "All Rights
  Reserved") and the Louis Porter Jr. Design Infinite Space theme PCC. The matrix has 3 exclude rows
  of the total in R-16; reasons are in `docs/governance/license-matrix.md` and `decisions.md §6`.
- **Starship** is a planned capability deferral (E8), not built; its revisit condition ran clean at E7.3
  (`decisions.md §17`, FSR DEF-1).
- **Operator decisions waiting** (not cards): FSR-C4 within-system `(kind, slug)` collisions, a condition
  met at E7.3 and recorded in the register for the operator (E7.3 receipt).
- **A flaky gate.** `sd17_b5_equipment::parse_runs_in_linear_time_on_a_synthetic_large_file` asserts a
  2 s wall-clock bound and read 4.46 s at load average 18 (quoted, progress.md 2026-10-06, E4a.2);
  not part of any recorded failure in R-10.
- **An unwired test.** `scripts/tests/test_transcribe_monster_tables.py`'s docstring says it is a
  `verify.sh` stage; `bash scripts/verify.sh --list` has no such stage (progress.md, 2026-10-07,
  E4a.4a, not changed).

## Does not cover

- Windows and macOS packaged builds, and CI (`pr-tests`, E7.9).
- A build from a clean checkout. The shared target directory was warm for every card, so a
  stale-artifact class of error is not excluded by R-8 to R-11, though cargo ran `--locked`.
- Starfinder seed renders as hashes. No Starfinder render-hash harness exists; the Starfinder seeds are
  checked by tests (R-12), not by hash. Only the two Pathfinder seeds have a hash pair (R-7).
- Anything the oracle does not export (R-13's third value is the list kept equal to the computation by a
  test), and parity beyond the roster: the four seeds, each of the ten player classes at level 1 and a
  Mechanic 1 with its drone. Higher-level builds for the other six classes are not oracle-checked.
- Books outside the eight converted ones, and any starship content.
- A re-run of the E7.2 test counts on this tree: R-8 to R-11 are re-derived from E7.2's recorded logs,
  not re-run, because the build directory was removed at E7.5 and a rebuild is out of this card's scope.
  The identity of those logs with the current tree rests on the diff statement under "Verification
  Evidence".
- R-3 and R-2 are pinned to `1335bb7248`; they move with every later commit, and this card's own row is
  not in R-3.
- R-20 is the recorded summary written at E7.4 (2026-10-07T18:01:14Z); the retro log keeps growing and a
  re-run of `scripts/retro.py summary` reads higher.
- Judgments in the prose (what a capability "is") are checked against receipts, not by a command; only
  `R-n` figures carry commands.

## Figures

Each line: value in bold, implementation A (shell, run from the repository root), implementation B
(Python). "Gives" is the output of that implementation when E7.6 ran, 2026-10-07.

- **R-1** build version in the two desktop manifests: **0.17.0,0.17.0**. A: `grep -h -m1 '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | awk -F'"' '{print $4}' | paste -sd,` gives `0.17.0,0.17.0`. B (Python, independent): `json.load` of both files, key `version` gives `0.17.0,0.17.0`.
- **R-2** commits on tranche/17 past the cut 20bf84a3b2, to 1335bb7248: **117**. A: `git log --oneline 20bf84a3b2..1335bb7248 | awk 'END{print NR}'` gives `117`. B (Python, independent): `git rev-list --count` over the same range gives `117`.
- **R-3** kanban rows, then rows `complete`, at 1335bb7248 (before this card's own row moved): **57 53**. A: `git show 1335bb7248:docs/release/SD-37-starfinder-1e/kanban.md | awk -F'|' '$2 ~ /^ (C|E)[0-9]/ { n++; if ($5 ~ /^ complete *$/) c++ } END { print n, c }'` gives `57 53`. B (Python, independent): regex over the same file, status = column 4 after the id gives `57 53`.
- **R-4** Starfinder work units, in-scope LST rows, excluded rows (CRB + 7 books): **8582 12718 229**. A: `jq -r '[(.units|length), .totals.rows_in_scope, .totals.rows_excluded]|map(tostring)|join(" ")' docs/work-inventory.starfinder-1e.json` gives `8582 12718 229`. B (Python, independent): `len(units)` and `totals` via `json.load` gives `8582 12718 229`.
- **R-5** converter report: records, converted, refused, degraded records: **8582 8582 0 30**. A: `jq -r '[.records,.converted,.refused,.degraded_records]|map(tostring)|join(" ")' data/starfinder-1e/sheet_rules/_report.json` gives `8582 8582 0 30`. B (Python, independent): `json.load` of the same report gives `8582 8582 0 30`.
- **R-6** Pathfinder work-inventory units (the PF side, unmoved): **49450**. A: `jq '.units|length' docs/work-inventory.json` gives `49450`. B (Python, independent): `len(json.load(...)['units'])` gives `49450`.
- **R-7** Pathfinder seed render hashes (first 8 hex), last run in E7.2: **aldric 1d830682 elowen 8d1a711c**. A: `awk '/^pf-seed-render (aldric|elowen) /{printf "%s%s %s", (n++?" ":""), $2, substr($3,8,8)} END{print ""}' docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.2_logs/pf_hash.log` gives `aldric 1d830682 elowen 8d1a711c`. B (Python, independent): regex over the same log gives `aldric 1d830682 elowen 8d1a711c`.
- **R-8** `verify.sh` stages passed in the full run (E7.2): **53**. A: `awk '/^    PASS/{n++} END{print n}' docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.2_logs/verify-full.out` gives `53`. B (Python, independent): regex `^    PASS ` over the same log gives `53`.
- **R-9** tests passed: root lib, root full, ingest full, desktop: **2838 6514 1832 700**. A: `for n in root-lib root-full ingest-full desktop; do awk '/^test result:/{for(i=1;i<=NF;i++) if($i=="passed;") s+=$(i-1)} END{printf "%s ", s+0}' docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.2_logs/$n.log; done | awk '{$1=$1; print}'` gives `2838 6514 1832 700`. B (Python, independent): regex `test result: \w+\. (\d+) passed;` summed per log gives `2838 6514 1832 700`.
- **R-10** `test result: FAILED` lines in the same four logs: **0 0 0 0**. A: `for n in root-lib root-full ingest-full desktop; do awk '/^test result: FAILED/{c++} END{printf "%s ", c+0}' docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.2_logs/$n.log; done | awk '{$1=$1; print}'` gives `0 0 0 0`. B (Python, independent): regex `^test result: FAILED` per log gives `0 0 0 0`.
- **R-11** frontend test files passed: **137/137**. A: `awk '/PASS +frontend-test/{gsub(/[()]/,"",$3); print $3}' docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.2_logs/verify-full.out` gives `137/137`. B (Python, independent): regex over the same log gives `137/137`.
- **R-12** Starfinder seed hand values (4 seeds), each with an SRD URL: **160**. A: `cd docs/release/SD-37-starfinder-1e/artifacts && awk -F'|' '/^\|/ && $2 ~ /^ SF-/ && $5 ~ /https:/' epic_0/seed-hand-values.md epic_4/E4.2-initiative-hand-values.md epic_4/E4.4-spell-dc-hand-values.md epic_4/E4.5-loadout-hand-values.md | awk 'END{print NR}'` gives `160`. B (Python, independent): line split on `|`, same predicate gives `160`.
- **R-13** oracle parity: PCGen observation files, explained differences, engine fields the oracle exports no value for: **15 36 1413**. A: `cd scripts/oracle_harness/sf_parity && echo $(ls *.oracle.txt | awk 'END{print NR}') $(awk '!/^#/ && NF' explained.tsv | awk 'END{print NR}') $(awk '!/^#/ && NF' not_in_oracle.tsv | awk 'END{print NR}')` gives `15 36 1413`. B (Python, independent): `os.listdir` and non-comment, non-blank line counts gives `15 36 1413`.
- **R-14** ui-smoke rows in `spec.json`, of them Starfinder rows, then the real store's entry count before and after, then whether its sha256 is equal: **103 14 15379 15379 true**. A: `jq -r --slurpfile s docs/release/SD-37-starfinder-1e/artifacts/epic_6/E6.6_logs/smoke_final/real_store.json '[(.rows|length), ([.rows[]|select(.id|test("starfinder"))]|length), $s[0].before.entries, $s[0].after.entries, ($s[0].before.sha256==$s[0].after.sha256)]|map(tostring)|join(" ")' apps/desktop/scripts/ui-smoke/spec.json` gives `103 14 15379 15379 true`. B (Python, independent): `json.load` of both files gives `103 14 15379 15379 true`.
- **R-15** `data/rules_tables` JSON files, files carrying the text `.lst`, and the Rust module `src/rules_core/rules_tables`: **281 0 absent**. A: `echo $(find data/rules_tables -name '*.json' | awk 'END{print NR}') $(grep -rlF '.lst' data/rules_tables | awk 'END{print NR}') $(test -d src/rules_core/rules_tables && echo present || echo absent)` gives `281 0 absent`. B (Python, independent): `os.walk`; per-line byte search for `.lst`; `os.path.isdir` gives `281 0 absent`.
- **R-16** Starfinder licence-matrix rows, `include`, `exclude`, `operator_sign_off` false: **11 8 3 11**. A: `awk -F'|' '/^\| `starfinder\//{n++; if($(NF-2) ~ /include/) i++; if($(NF-2) ~ /exclude/) x++; if($(NF-1) ~ /^ *false *$/) f++} END{print n, i, x, f}' docs/governance/license-matrix.md` gives `11 8 3 11`. B (Python, independent): split on `|`, same columns gives `11 8 3 11`.
- **R-17** Starfinder product-identity screen terms (`SF_PI_TERMS`): **41**. A: `awk '/^pub const SF_PI_TERMS/{s=1;next} s && /^\];/{exit} s && !/^ *\/\//{n+=gsub(/"[^"]+"/,"&")} END{print n}' src/rules_core/pi_screening.rs` gives `41`. B (Python, independent): regex `"([^"]+)"` over the array body minus comment lines gives `41`.
- **R-18** Starfinder corpus records (files carrying a `license` field), of them `PI-REDACTED`: **8582 289**. A: `echo $(grep -rlE '"license": "(OGL|PI-REDACTED)"' data/starfinder-1e/corpus | awk 'END{print NR}') $(grep -rl '"license": "PI-REDACTED"' data/starfinder-1e/corpus | awk 'END{print NR}')` gives `8582 289`. B (Python, independent): `os.walk` + `json.loads`, top-level `license` key gives `8582 289`.
- **R-19** tracked files under `data/starfinder-1e`: **17717**. A: `git ls-files data/starfinder-1e | awk 'END{print NR}'` gives `17717`. B (Python, independent): `git ls-files` read in Python, non-empty lines gives `17717`.
- **R-20** retro events since 2026-10-02 (recorded summary), corrections, incidents: **160 37 15**. A: `jq -r '[(.events.by_type|add), .events.by_type.correction, .events.by_type.incident]|map(tostring)|join(" ")' docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.4_logs/retro-summary.json` gives `160 37 15`. B (Python, independent): `json.load`, `sum(by_type.values())` gives `160 37 15`.

## Update Eligibility

Set at publish by the release manifest (`tools/release/check_release_manifest.py`; the repo keeps the
version placeholder until then). Not decided in this document.
