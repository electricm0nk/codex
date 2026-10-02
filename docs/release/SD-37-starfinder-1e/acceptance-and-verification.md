---
canonical: true
owner: operator
bundle_id: SD-37
date: 2026-10-02
---

# SD-37 Acceptance and Verification

Every gate below has a command that **can fail**, and states what its proof does **not** cover
(AGENTS.md rule 7; `decisions.md §16`). The per-card criteria are in `epic-breakdown.md`. This file
holds the bundle-level gates and maps them to the artifacts that prove them.

---

## G-1 PF byte-identical render (E1.4, E4a.4, re-checked at E7.2)

- **Command.** Build each tree with its own `CARGO_TARGET_DIR`. Render Aldric (Fighter 3) and
  Elowen (Wizard 5) through the same render entry point that the ui-smoke harness uses. Then run
  `sha256sum <render>` for each tree. Cross-tree renders swap the sheet-rule package at the baked
  path (`corpus_loader.rs:360`) or rebuild per tree (R-X).
- **Hash set.** `artifacts/epic_1/pf-render-hashes.txt` records 2 seeds × {before, after}, with the
  base SHA and the head SHA. E4a.4 appends its own pair. Pass = every after-hash equals its
  before-hash.
- **Plus** `structural_diff.py` `verdict=PASS`, with PF records unmoved at **49,450** (CUI F-15).
- **Does not cover:** PF builds other than the two seeds. The structural diff covers the package,
  not the rendered sheet of every build.

## G-2 SF seeds equal the hand-computed Core Rulebook values (E4.MC, E7.1)

For each SF seed (Soldier 3, Mystic 5, Technomancer 5, Envoy 3; `content-unit-inventory.md §4`),
the rendered sheet's **KAC, EAC, Stamina, Resolve, HP, Fort/Ref/Will, BAB and every skill total**
(plus spells known/per day for the Mystic and the Technomancer) must equal
`artifacts/epic_0/seed-hand-values.md`.

**Source of each hand value** (each one is transcribed by E0.4 from the Starfinder Reference
Document, with a URL and section per row; page numbers come from the PCGen `SOURCEPAGE` tokens and
are for cross-reference only):

| Value | SRD source table/section |
|---|---|
| BAB, base saves, HP/level, Stamina/level, skill ranks/level, key ability | the class's progression table and class header: Soldier (CRB p.110), Mystic (p.82), Technomancer (p.118), Envoy (p.60). Pages from `awk -F'\t' '/^CLASS:/{for(i=1;i<=NF;i++) if($i~/SOURCEPAGE/) print $1,$i}' $S/paizo/core/scr_classes.lst` |
| Race HP, ability adjustments | the race's entry (Races chapter) |
| Theme ability +1, theme class skill | the theme's entry (Themes) |
| EAC/KAC bonus, max Dex, armour check penalty | the armour table (Equipment chapter) |
| Resolve | the Resolve Points rule (Character creation / Classes) |
| Skill total rule, class-skill bonus | the Skills chapter's skill-check rule |
| Spells known / per day | the Mystic and Technomancer spells tables |
| Point buy | the ability-score generation rule (Character creation) |

- **Command.** `cargo test --locked -j 8 --lib sf_seed -- --test-threads=8` (the test file is
  E4.1's to create). Each fixture row carries `source: <SRD URL>`, and a guard test fails if any
  row lacks one.
- **Planted mutation.** Dropping Con from Stamina, or swapping ALTHP↔CURRENTMAX, must turn at least
  one seed red (E3.3, E4.MC).
- **Does not cover:**
  - 7 of the 11 classes (Mechanic, Operative, Solarian, Biohacker, Vanguard, Witchwarper, Drone);
  - levels other than 3 and 5;
  - races and themes other than the four picked;
  - archetypes;
  - feats that modify these totals unless a seed takes one;
  - augmentations and weapon fusions;
  - any armour other than the seed loadouts;
  - starships.

  E7.1's parity roster widens class coverage to all 11 at level 1 (oracle-only, no hand values).

## G-3 Widest-scope `cargo test`, both workspaces (E7.2)

- **Root workspace:** `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (includes
  `crates/codex-ingest`).
- **Desktop workspace:** `cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`.
  This is a separate workspace that the root sweep never touches.
- **Frontend:** `npm test`, `npm run typecheck` in `apps/desktop`.
- **Full gate:** `bash scripts/verify.sh -j 8`, one run. Every stage must PASS, and the stage count
  equals 51 (CUI F-16) plus the stages added by E0.1/E2.2/E3/E4 (each named in its receipt). Every
  `test result: FAILED` line is attributed to its `Running` line, and there must be none.
- **Does not cover:** the Windows/macOS packaged builds. CI's publish workflow covers those.

## G-4 SF oracle parity, with the sparse-path fix proven in a fresh clone (E0.1, E7.1)

- **Fresh clone** (E0.1): in a directory outside any existing clone, run
  `scripts/fetch-pcgen-oracle.sh --dest <fresh> --quiet` →
  `test -f <fresh>/data/starfinder/paizo/core/_starfinder_core_rulebook.pcc` and
  `test -d <fresh>/system/gameModes/Starfinder`. Then delete `<fresh>/data/starfinder` and re-run
  the completeness probe; it must exit non-zero.
- **Parity** (E7.1): PCGen SF game-mode runs for the 4 SF seeds and for each of the 11 classes at
  level 1. 0 unexplained mismatches. Every explained mismatch cites the SRD (where PCGen is wrong)
  or a fix commit (where Codex was wrong).
- **"What the oracle does not contain"** list in the E7.1 receipt, at minimum: starship rules;
  the Resolve formula; post-COM/Near Space books (**estimate**); behaviour of `STATUS:BETA` data
  that PCGen itself gets wrong.
- **Does not cover:** anything on that list.

## G-5 Licence and PI (E0.2, E3.MC)

- `grep -c 'starfinder' docs/governance/license-matrix.md` ≥ 10 (one row per SF book dir).
- SF registry test: exactly the 8 in-scope books; SSRGG and LPJ absent.
- PI sweep over `data/starfinder-1e/**` with the SF term set: 0 unredacted hits.
- `operator_sign_off` stays `false` until the operator signs. That is **not** a closure blocker
  for SD-37's code, but README §6 lists it.

## G-6 Closure (E7.3–E7.9)

- E7.3: `awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 !~ /complete/' kanban.md` prints only the E7.3–E7.9
  closure-chain rows still in flight. Any other row means **stop**.
- DEF-1 revisit check run (`decisions.md §17`) and its output pasted.
- Retrospective cited: `grep -c 'sd37-retrospective' references/README.md` ≥ 1.
- Graphify receipt: clean tree, HEAD = `origin/tranche/17`, indexed SHA recorded.
- PR open. The operator merges.

## Per-criterion artifact map

| Gate | Artifact |
|---|---|
| G-1 | `artifacts/epic_1/pf-render-hashes.txt`, `artifacts/epic_1/E1.4_cycle_receipt.md`, `artifacts/epic_4a/E4a.4_cycle_receipt.md` |
| G-2 | `artifacts/epic_0/seed-hand-values.md`, `artifacts/epic_4/E4.MC_cycle_receipt.md` |
| G-3 | `artifacts/epic_7/verify-e7.2.log`, `artifacts/epic_7/E7.2_cycle_receipt.md` |
| G-4 | `artifacts/epic_0/E0.1_cycle_receipt.md`, `artifacts/epic_7/E7.1_cycle_receipt.md` |
| G-5 | `artifacts/epic_0/E0.2_cycle_receipt.md`, `artifacts/epic_3/E3.MC_cycle_receipt.md` |
| G-6 | `artifacts/epic_7/E7.3_cycle_receipt.md` … `E7.9_cycle_receipt.md`, `receipts.md` |
