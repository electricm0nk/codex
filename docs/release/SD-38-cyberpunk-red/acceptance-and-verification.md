---
canonical: true
owner: operator
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Acceptance and Verification

Bundle-level gates. Each has a command that **can fail** and says what its proof does **not** cover
(AGENTS.md rule 7). Per-card criteria are in `epic-breakdown.md`. Each gate names the Definition of
Done item (`scope-draft.md`, DoD 1–7) it proves.

---

## G-1 PF and SF byte-identical (DoD 6) — E0.6, re-checked at E2.MC, E4.MC and E7.4

Base = the C1 commit, built in its own worktree with its own `CARGO_TARGET_DIR` (the sheet-rule path
is baked at compile time, R-X). Head = the card's tree. The commands are SD-37's seed render commands
(`apps/desktop/src-tauri/src/pf_seed_render_hash.rs:9–17`, SD-37 E1.4 receipt; the SF seed-lines test in
`apps/desktop/src-tauri/src/sf_sheet_print.rs:429–449`, SD-37 E7.9a receipt):

```bash
# PF: Aldric (Fighter 3) and Elowen (Wizard 5), the LoadSavedCharacterResponse JSON the sheet receives
cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=<dir> cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture
diff <base>/sha256.txt <head>/sha256.txt                       # pass = no output
# SF: the four SD-37 seeds' printed lines
cd apps/desktop/src-tauri && SF_SEED_LINES_OUT=<dir> cargo test --locked -j 8 --bins sf_seed_lines_print_every_resolved_term_and_no_pcgen_residue -- --test-threads=8
diff -r <base> <head>                                          # pass = no output
# SF totals and both converters
cargo test --locked -j 8 --lib sf_seed -- --test-threads=8
cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check
cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
```

- **The base hashes are measured at E0.6, not copied** from SD-37 (tranche-17-ui changed the sheet
  since SD-37's `1d830682…`/`8d1a711c…`; R13).
- **Does not cover:** PF/SF builds other than these six seeds; the structural package diff (the
  converters' `--check` stands in for it, because SD-38 changes no PF/SF data).

## G-2 Creation by all three methods (DoD 1) — E3.1–E3.MC

- A character is created through the real form by **Streetrat**, **Edgerunner** and **Complete
  Package**, with Lifepath, skills (86 points, (x2), cap 6, Basic ≥ 2), Role Ability at rank 4 with its
  sub-choices, starting kit, cyberware, cash.
- **Command:** E3.MC's three ui-smoke rows under an isolated `XDG_DATA_HOME` (R5), then for each saved
  character: every number on its loaded sheet equals `compute_sheet` for its saved input
  (`cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins cpr_creation -- --test-threads=8`
  plus E3.MC's comparison script).
- **Book-internal parity:** 100 of 100 Streetrat template rows round-trip (CUI F-22); 10 of 10 Role
  skill lists cost 86 (F-25); 10 of 10 Role cyberware packs give F-66.
- **Does not cover:** every Role × method combination (three builds plus the per-Role parity tables);
  Lifepath answers beyond the Cultural Origin language (printed, no number).

## G-3 The sheet (DoD 2) — E4.1–E4.MC

- STATs, HP/Seriously Wounded/Death Save/Humanity/EMP, MOVE, skills by category with bases, weapons with
  damage/ROF/hands/conceal and the range-DV mini-table (D-1), armour SP per location with penalty,
  cyberware with HL and slots, Role Ability with rank-derived values, Netrunner deck/programs/NET
  Actions, reputation, IP, wound states, criticals checklist, Lifepath text. **No Encumbrance box**
  (D-12).
- **Command:** `cd apps/desktop && npm run typecheck && npm test` (the row-mapping test: every rendered
  digit is one `cpr.` row; a +1000 shift of the rows shifts the sheet); E4.7's ui-smoke rows.
- **Book-internal parity:** HP table p080, 126 of 126 cells (F-23, E2.1); p121 sample sheet (E2.10,
  branch recorded from the image).
- **Does not cover:** in-play state (ablation, wound simulation, dice).

## G-4 Improvement after creation (DoD 3) — E5.1–E5.MC

- IP spend on skills and Role rank (F-33 costs), Role switch at rank ≥ 4 (A-1), cyberware install with
  Humanity Loss (A-3, D-11), therapy (D-5 cap), reputation change — all saved through the store and
  re-read.
- **Command:** `cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins cpr_improve -- --test-threads=8`;
  E5.MC's ui-smoke rows; real store count + sha256 equal before/after (R5).
- **Does not cover:** IP awards (GM, printed).

## G-5 Catalogs and GM reference (DoD 4) — E6.1–E6.MC

- Every MANIFEST kind is browsable; `print_only` kinds render as read-only GM pages; counts equal
  MANIFEST.
- **Command:** `cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins cpr_catalog -- --test-threads=8`;
  frontend tests; ui-smoke rows `browse-cyberpunk-catalog`, `browse-cyberpunk-gm-reference`.
- **Does not cover:** generators (DEF-1, DEF-5).

## G-6 Content: MANIFEST counts, `source_page`, 10% image sample, licence (DoD 5) — E1.0–E1.V, E0.2

- **MANIFEST count gate (two implementations):**
  ```bash
  python3 scripts/cpr_manifest_check.py --all                                  # exit 0
  cargo test --locked -j 8 --test cpr_corpus_manifest -- --test-threads=8      # PASS
  ```
  Both fail on: a kind file missing or extra, a count ≠ `expected`, a `null` count, a record without a
  `source_page` matching `^p[0-9]{3}$` inside `p001`–`p459`.
- **Sum check:** MANIFEST `expected` sum = CUI §3's column sum (E1.0 receipt, two implementations).
- **10% image sample:** E1.V draws ceil(0.1 × n) records per kind (seed recorded) and checks each field
  against `pdftoppm -f N -l N -r 150 -png`; 0 `disagree` rows left
  (`awk -F'\t' 'NR>1 && $5=="disagree"' artifacts/epic_1/E1.V_verdicts.tsv | awk 'END{print NR}'` → 0).
- **Nothing from the source in git:** `git ls-files | grep -ciE '\.(pdf|png)$'` equal at C1 and at E1.V.
- **Licence:** E0.2's LICENSE.json assertion and the DEVIATION row (`decisions.md §4`).
- **Does not cover:** the 90% of records not sampled (each lane's own read only); the GM-table counts
  until E1.8/E1.V pin them.

## G-7 Packaged resources (DoD 7) — E0.3, E7.2

- `data/cyberpunk-red/corpus/` is a `bundle.resources` entry at its own path and
  `apps/desktop/src-tauri/tests/packaged_resources.rs` asserts it and loads a CPR seed's sheet under the
  packaged root.
- **Command:**
  ```bash
  cd apps/desktop/src-tauri && cargo test --locked -j 8 --test packaged_resources -- --test-threads=8
  bash scripts/verify.sh --only tauri-resources-tracked
  cargo test --locked -j 8 --test no_baked_manifest_dir_in_production -- --test-threads=8   # repo root
  ```
- **Does not cover:** Windows/macOS bundles.

## G-8 Seeds and ui-smoke (DoD 7) — E7.1, E4.7

- Three CPR starters (Streetrat Solo, Complete Package Netrunner, Edgerunner Medtech) are restored by
  Restore starter characters and open; the PF starters' bytes are unchanged; every ui-smoke row is green
  under an isolated `XDG_DATA_HOME`; `docs/testing/ui-smoke-inventory.md` regenerated
  (`npm run ui-smoke:doc`, then `git status --porcelain docs/testing/ui-smoke-inventory.md` empty).
- **Does not cover:** characters other than the seeds and the smoke builds.

## G-9 Widest verify, both workspaces + frontend (DoD 7) — E7.4

```bash
pgrep -f '[c]argo'; df -h /                                             # before the sweep
cargo test --locked -j 8 --no-fail-fast -- --test-threads=8             # repo root
cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8
cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc
bash scripts/verify.sh -j 8                                             # every stage PASS
python3 -m pytest tools/release -q
```

Every `test result: FAILED` attributed to its `Running` line (there must be none). Stage count: 53 today
(`bash scripts/verify.sh --list | tail -n +2 | wc -l` → 53 on 2026-10-13) plus any stage a card adds
(named in its receipt). `scripts/verify-baselines.env` re-derived from that one logged run. G-1 re-run.
**Does not cover:** CI (no workflow runs on `tranche-18` pushes; the PR's `pr-tests`, E7.11, is the CI
evidence, SD-j).

## G-10 Fresh-install acceptance — E7.3

Install the .deb **and** the AppImage built from `tranche-18` on a clean profile. Open Settings →
Update → Check: nothing reads "unknown"; the Installed panel shows the stamped version, source commit
and artifact hash; release notes load; Install is enabled when a newer release exists. Open a CPR seed.
Record what was observed per artifact in the receipt — "the suite passed" is not this evidence. If the
box cannot run it: SD-m (`blocked-escalated`, operator action named).
**Does not cover:** Windows/macOS installers.

## G-11 Closure — E7.5–E7.11

- E7.5: the fenced scan (`workflow-instruction.md §11` step 1) prints nothing; 68 rows; DEF-1…DEF-5
  revisit conditions checked. Any output → **stop**.
- Retrospective written and cited: `test -s docs/retro/sd38-retrospective.md` and
  `grep -c 'retro/sd38-retrospective.md' docs/release/SD-38-cyberpunk-red/references/README.md` ≥ 1.
- Release notes: `grep -c 'SD-38 INTERIM NOTES' docs/release/SD-38-cyberpunk-red/release-notes.md` → 0
  (the interim body is gone, `decisions.md §2`); `python3 tools/release/resolve_release_notes.py --repo-root .`
  prints this package's notes path (it already passes from C1 on, so it proves nothing about E7.8 by
  itself); `python3 scripts/tranche/validate-tranche-notes.py --release-notes-path docs/release/SD-38-cyberpunk-red/release-notes.md` PASS.
- Graphify receipt: clean tree, HEAD = `origin/tranche-18`, indexed SHA recorded.
- PR open; `pr-tests` green. The operator merges.

## Per-gate artifact map

| Gate | Artifact |
|---|---|
| G-1 | `artifacts/epic_0/E0.6_cycle_receipt.md` (+ `pf-render/{base,head}/sha256.txt`, `sf-lines/{base,head}/`); E2.MC, E4.MC, E7.4 receipts |
| G-2 | `artifacts/epic_3/E3.MC_cycle_receipt.md` |
| G-3 | `artifacts/epic_4/E4.MC_cycle_receipt.md`; `artifacts/epic_2/E2.1_cycle_receipt.md`, `E2.10_cycle_receipt.md` |
| G-4 | `artifacts/epic_5/E5.MC_cycle_receipt.md` |
| G-5 | `artifacts/epic_6/E6.MC_cycle_receipt.md` |
| G-6 | `artifacts/epic_1/E1.0_cycle_receipt.md`, `E1.V_cycle_receipt.md`, `E1.V_verdicts.tsv`; `artifacts/epic_0/E0.2_cycle_receipt.md` |
| G-7 | `artifacts/epic_0/E0.3_cycle_receipt.md`, `artifacts/epic_7/E7.2_cycle_receipt.md` |
| G-8 | `artifacts/epic_7/E7.1_cycle_receipt.md`, `artifacts/epic_4/E4.7_cycle_receipt.md` |
| G-9 | `artifacts/epic_7/E7.4_cycle_receipt.md` + logs |
| G-10 | `artifacts/epic_7/E7.3_cycle_receipt.md` |
| G-11 | `artifacts/epic_7/E7.5_cycle_receipt.md` … `E7.11_cycle_receipt.md`, `receipts.md` |
