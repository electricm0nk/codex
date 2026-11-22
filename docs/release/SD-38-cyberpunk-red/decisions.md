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

# SD-38 Decisions

Bundle-specific ADRs for SD-38 (Cyberpunk RED). Each entry is marked **OPERATOR** (a ruling the
operator made, quoted verbatim from the planner's brief of 2026-10-13), **DEFAULT** (an
operator-away default taken by the planner, Fable 5.1, in that brief), **AUTHOR DEFAULT** (taken by
this package's author because the brief left the point open; flagged, `§21`) or **STANDING** (an
earlier operator ruling still in force). Every DEFAULT and AUTHOR DEFAULT says what enforces it
and what to change if the operator disagrees. All are listed in `README.md §6`.

Figures are cited as `CUI F-n` (`content-unit-inventory.md §2`), each with its command there.

---

## §1 — Package location: `docs/release/SD-38-cyberpunk-red/` (flagged deviation)

**DEFAULT (brief, "Package").** The package lives at `docs/release/SD-38-cyberpunk-red/`.

**Why.** The repo keys on `docs/release/SD-NN-<slug>/`: the chassis (`docs/release/template/template.md`,
`docs/governance/workflow-instruction-template.md`), the repo-local skill
`.claude/skills/stc-authoring/SKILL.md`, the release tooling (`tools/release/resolve_release_notes.py`
pattern `^docs/release/[^/]+/release-notes\.md$`, line 25) and every bundle script. `docs/stc/` does not
exist (`ls docs/stc` → "No such file or directory", re-run 2026-10-13). Same reasoning as SD-37
`decisions.md §1`.

**Flagged deviation.** The user-level overlay `stc-codex-overlay` names `docs/stc/stc-<##>-<description>/`.
This package does not follow it.

**Enforced by:** `ls docs/release/SD-38-cyberpunk-red/README.md`.

**Revisit if the operator disagrees:** a dedicated cycle moves the package and every reference
(`grep -rn 'SD-38-cyberpunk-red' docs scripts .claude tools`), or the overlay is amended. No cycle
does this without a ruling.

---

## §2 — Branch, board, version

**DEFAULT (brief, "Package").**

- **Branch:** `tranche-18`, cut from `origin/develop` **after PR #398 (tranche-17-ui → develop)
  merges**. On 2026-10-13 PR #398 is open (`gh pr view 398 --json state,title` →
  `{"state":"OPEN","title":"Tranche 17 UI"}`), and `origin/develop` is `0b8a5757bf` (PR #397's
  merge). Card C1 does not wait for #398: the branch is cut from #398's head (SD-k, revised).
- **Board:** local file `./kanban.md`, paired with `./progress.md` (Hermes retired 2026-08-01).
- **Target version:** `0.18.0`, stamped by **card C1's single commit** on the same 14 surfaces SD-37's
  C1 bumped (`git show --name-only --format= 9d03a76996 | awk 'NF'` → 14 files):
  `.github/workflows/publish-tester-release.yml` (stamp `VERSION="0.17.${GITHUB_RUN_NUMBER}"` →
  `0.18.`), `apps/desktop/package.json`, `apps/desktop/src-tauri/{Cargo.toml,Cargo.lock,tauri.conf.json}`,
  `apps/desktop/src/release/buildVersionTriple.test.ts`, `apps/desktop/src/releaseChecks/buildVersionTriple.test.ts`,
  `apps/desktop/src/testSupport/makeSurface.ts`, and six build-label fixtures
  (`operatorTriage/buildOperatorTriageDraft.test.ts`, `testerWorkbench/loadTesterWorkbenchSurface.test.ts`,
  `testerWorkbench/status/createWorkbenchStatus.test.ts`, `testerWorkbench/feedback/evidence/captureFeedbackEvidence.test.ts`,
  `testerWorkbench/feedback/enhancement/composeEnhancementRequest.test.ts`,
  `testerWorkbench/feedback/bug/composeBugReport.test.ts`). Root `Cargo.toml` stays `0.1.0`.
- **Re-derive at bump time (C1), two commands. A names 15 files: B's 14 plus
  `tools/release/test_resolve_release_notes.py`, which C1 does not edit (re-run 2026-10-13 by the
  auditor: `wc -l` → 15 and 14; `diff` → that one file only):**
  ```bash
  # A (brief's grep, extended by --include=*.lock so Cargo.lock is seen)
  grep -rln '0\.17\.' --include=*.json --include=*.toml --include=*.lock --include=*.ts --include=*.yml --include=*.py \
    apps .github tools docs/architecture docs/release/current-release.json | grep -v node_modules | grep -v target | sort
  # B (SD-37's bump commit)
  git show --name-only --format= 9d03a76996 | awk 'NF' | sort
  ```
  Measured 2026-10-13 on `tranche-17-ui` @ `bb6107aee0`: the brief's grep **without** `*.lock`
  names 14 files, and they are **not** the same 14: it includes
  `tools/release/test_resolve_release_notes.py` and misses `apps/desktop/src-tauri/Cargo.lock`
  (`grep -n '0\.17\.0' apps/desktop/src-tauri/Cargo.lock` → line 479).
  `test_resolve_release_notes.py` uses `0.17.0` only as a synthetic mismatching version against its own
  synthetic tranche-16 fixture (lines 31, 41, 69–72); it reads no repo version, so **C1 does not edit
  it**. C1's receipt shows A and B, and the difference (`test_resolve_release_notes.py` only) by name.
- **Release-notes pointer, same commit:** `docs/release/current-release.json` →
  `{"tranche": 18, "notes_path": "docs/release/SD-38-cyberpunk-red/release-notes.md"}` (today
  `{"tranche": 17, "notes_path": "docs/release/SD-37-starfinder-1e/release-notes.md"}`).
- **The notes file resolves from C1 on (resolves former risk R-7).** The resolver fails when the
  pointer's tranche differs from `apps/desktop/package.json` (`resolve_release_notes.py:45–51`) and when
  the notes body holds `Populated at closure` or `Pre-population placeholder` (line 26). C1 sets
  `0.18.0`, so the pointer must say 18 in the same commit; leaving it on SD-37's notes fails the tranche
  check (simulated 2026-10-13: exit 2, "is for tranche 17 but apps/desktop/package.json is 0.18.0").
  So `release-notes.md` ships an **interim body** with neither resolver marker. Its first paragraph
  opens with the package-local sentinel `SD-38 INTERIM NOTES`. The CI step
  `release-tooling-tests.yml:49` (`resolve_release_notes.py --repo-root .`) stays green for the whole
  run (simulated with `package.json` 0.18.0 + pointer 18: exit 0, prints the SD-38 path).
  **What keeps the interim body from shipping:** E7.8 fails while `grep -c 'SD-38 INTERIM NOTES'` on the
  notes is not 0; E7.11 runs the same grep before `gh pr create`; nothing publishes from `tranche-18`
  (`publish-tester-release.yml` runs on `develop`/`main` pushes only), and the operator merges.
  **Residual:** the publish workflow does not know the sentinel; a merge that skipped E7.8 would ship the
  interim body. Adding the sentinel to `PLACEHOLDER_MARKERS` is a `tools/release/` change outside SD-38's
  cards. The lead accepted this residual on 2026-10-13 (`risks-and-open-questions.md` R-7).
- **Version rule.** The tranche digit moves only at a new bare `tranche-N` cut. Closure moves only the
  build position (`0.18.${GITHUB_RUN_NUMBER}`, stamped at publish after the operator merges).

**Enforced by:** C1's receipt (`git show --name-only --format= <bump-sha> | awk 'NF' | wc -l` → 15:
the 14 surfaces + `current-release.json`); `npx tsx src/release/buildVersionTriple.test.ts && npx tsx src/releaseChecks/buildVersionTriple.test.ts`
in `apps/desktop` (`npm test` takes no filter); `python3 tools/release/resolve_release_notes.py --repo-root .`
exit 0; `git merge-base --is-ancestor <#398 merge sha> tranche-18`.

**Revisit if the operator disagrees:** if the operator wants SD-38 cut before #398 merges, C1 cuts from
current `origin/develop` and the closure PR resolves the #398 overlap (`CharacterHubPage.tsx`,
`character_hub.rs`, `sheetLayout.ts`). Change C1 only.

---

## §3 — Operator rulings on record (verbatim)

**OPERATOR (2026-10-13), quoted from the brief:**

- **Licence:** *"license - do the whole thing for now. We can back stuff out or reword it later if
  need be. For now, i'm the only person even using the app."* → ship the whole book text (`§4`).
- **Scope:** *"Scope: full book."* → everything a player or GM uses at the table (`§5`).
- **Mode:** travelling for a few weeks; autonomous; report after each epic; implementation lanes run
  as `Workflow` scripts. The unattended-mode protocol applies (`§16`).
- **Automation:** *"none of the automation that we built will apply"* — confirmed: no PCGen data
  exists for Cyberpunk RED (`§23`).

**Enforced by:** `§4`, `§5`, `§16`, `§23`.

---

## §4 — Licence: whole text ships; flagged deviation from the licence-matrix posture

**OPERATOR ruling (§3), implemented as follows (DEFAULT shape from the brief).**

- Cyberpunk RED is **not** OGL. R. Talsorian's Homebrew Content Policy applies. The repo's
  licence posture (`docs/governance/license-matrix.md`, "Per-book OGL/attribution/PI-screening
  evidence") is built for OGL books. SD-38 records a **flagged deviation**: the whole book text
  (names, tables, rule text, descriptions) ships in a private single-user build.
- **Per-book licence file** `data/cyberpunk-red/corpus/LICENSE.json`, in the shape of
  `data/starfinder-1e/corpus/core/LICENSE.json` (keys `book`, `license_declaration`,
  `operator_sign_off`, `records_processed`, `screening_method_note`), with:
  `license_declaration: {open_game_content: false, policy: "R. Talsorian Homebrew Content Policy",
  operator_ruling: "ship whole text, private single-user build, 2026-10-13"}` and
  `operator_sign_off.signed_off: true`.
- **Licence-matrix row.** A new section "Cyberpunk RED (SD-38)" in `docs/governance/license-matrix.md`
  with one row in the Starfinder table's column set (PCC/source, code, publisher, OGL text,
  COPYRIGHT lines, ISOGL, INFOTEXT, licence, PI posture, data rows, include/exclude,
  `operator_sign_off`; header at `docs/governance/license-matrix.md:234`), adapted: source = the PDF,
  licence = "R. Talsorian Homebrew Content Policy; not OGL", PI posture = "not screened: operator
  ruling 2026-10-13", include, `operator_sign_off` true, and the word **DEVIATION** in the row.
- **No PI screening** is applied to Cyberpunk RED records: PI is an OGL concept. `pi-sweep` scans
  `src/rules_core/rules_catalog` + `data/rules_tables` only (`scripts/verify.sh` stage header line
  2249), so it does not read `data/cyberpunk-red/`. E0.2 proves that with the stage run.

**Enforced by:** E0.2 (row present, LICENSE.json fields, `pi-sweep` and `pcgen-residue-gate` PASS
with the data dir present); G-6.

**Revisit if the operator disagrees** ("we can back stuff out or reword it later"): a later bundle
rewrites `text` fields, keeping names/numbers, and flips `signed_off`. Every record carries
`source_page`, so a rewrite can be scoped page by page. The queue entry is `forward-scope-register.md`
FSR-C1; its revisit command (`signed_off` prints `False`) is checked at E7.5.

---

## §5 — Scope: the full corebook v1.22 table content; fiction is not content

**OPERATOR ruling (§3) with the brief's DEFAULT split.**

- **Content units:** everything a player or GM uses at the table in corebook v1.22, inventoried in
  `content-unit-inventory.md` (CUI §3 kind table).
- **Not content units:** the setting/fiction chapters *Never Fade Away, View from the Edge, Tales from
  the Street, The Fall of the Towers, Welcome to the Dark Future, Time of the Red, Welcome to Night
  City, Everyday Life, Screamsheets, Black Dog*.
- **GM tables are content units, print-only** (DEFAULT): NET Architecture generation, Mooks & Grunts
  stat blocks, Encounters, Beat Charts, the Night Market generator and the Playstyle IP award table
  ship as catalog/reference pages. No engine reads them.
- **Mooks & Grunts, Encounters and Beat Charts have no inventory** (none of the seven inventories
  covers them). Card E1.8 inventories them before transcribing (CUI §2.8).

**Enforced by:** CUI §3 (`print_only` flag in MANIFEST); E6.4; E1.8.

**Revisit if the operator disagrees:** add or drop a kind in CUI §3 and MANIFEST; the count test moves
with it.

---

## §6 — Paper-sheet rule (STANDING)

**STANDING (2026-09-07), binding.** Campaign Codex is a paper character-sheet generator. Only values
that feed a number on the sheet are computed. Everything else is printed. Dice stay literal on the sheet
(it prints "1d10", never rolls it); creation-time table rolls are the one exception (DEF-2). A term that resolves for this character is added into one
number ("Facedown 9 + 1d10", not "COOL + Rep + 1d10").

**Enforced by:** each card's criterion; reviewers reject compute machinery for a print-only rule.
The test question: does this value land on the sheet as a number?

**Revisit:** standing ruling.

---

## §7 — Epic spine and concurrency

**DEFAULT (brief, "Epics").**

```
C0 → C1 → E0 (serial) → E1.0 → { E1.1 … E1.8 } (parallel, cargo-free) → E1.V
E1 lane cards gate the E2 lanes:  lane A (E2.1–E2.4) ∥ lane B (E2.5–E2.8) ∥ lane C (E2.9)
lane C ends → E6 (Sonnet) in the freed cargo slot;  lane A ends + E6.1 → E3.1
all lanes + E1.V → E2.10 → E2.MC;  E6.MC → E3.2 → E3.3 … E3.MC
E2.MC + E3.2 → E4 (beside E3.3–E3.MC, disjoint files);  E3.MC + E4.MC → E5;  E5.MC → E7 (serial)
```

At most **3 cargo lanes** at once (`§16`). The full parallel/serial map leads `epic-breakdown.md`.
Shared files are serialised by dependency, not by fence prose (writer order in `epic-breakdown.md §0`):
`apps/desktop/src-tauri/src/main.rs` E0.4 → E6.1 → E3.1 → E4.5 → E4.6 → E5.1;
`CharacterHubPage.tsx` E6.2 → E3.2 → E4.1 → E5.1; `LandingScreen.tsx` E0.5 → E6.2 → E3.2;
`tauri.conf.json` E0.3 only; `packaged_resources.rs` E0.3 → E7.2; `spec.json` one UI card at a time.
**Fence exit (SD-37 lesson, written in advance):** once a card's epic merge check is `complete`, a later
serial card may edit that epic's files under its own criterion and re-runs that epic's gate (PF/SF
byte-identical for engine files; the epic's acceptance command for the file's own behaviour).

**Enforced by:** `epic-breakdown.md §0`; `kanban.md` `Depends on`; the Workflow script reads
`Depends on` and refuses a card whose dependency is not `complete` (`§19` rule R8).

**Revisit if the operator disagrees:** reorder in `epic-breakdown.md §0`; the script reads the board.

---

## §8 — `GameSystem::CyberpunkRed`; no sheet-rule package (design decided)

**DEFAULT (brief, "Architecture"), with the open point decided here.**

- `GameSystem::CyberpunkRed`, `id() = "cyberpunk-red"`, `corpus_relative() = "data/cyberpunk-red/corpus"`.
  Extend `GameSystem::ALL` (`src/rules_core/game_system.rs:48`, today `[GameSystem; 2]`), `index()`
  (line 67), `PerSystem<T>` (line 143: add `cyberpunk_red: T`) and every exhaustive `match`
  (today: `game_system.rs:52, 68, 76, 84, 151, 170`; `src/rules_core/sheet_rule.rs:360`;
  `src/rules_core/money.rs:334`; `crates/codex-ingest/src/pcgen_import/sheet_rule/prose.rs:151`).
  Every `PerSystem { … }` literal gains the field (24 sites:
  `grep -rn 'PerSystem {\|pathfinder_only(' --include=*.rs src apps/desktop/src-tauri/src crates tests | awk 'END{print NR}'`
  → 24 on 2026-10-13, 4 inside `game_system.rs`: same grep on that file, `awk 'END{print NR}'` → 4,
  re-run by the auditor).
- **`sheet_rules_relative()` → `Option<&'static str>`**, `None` for `CyberpunkRed` (the brief's
  preferred "returns an Option" form). Why, measured: the alternative (a path to an empty
  `data/cyberpunk-red/sheet_rules/`) needs a tracked file under it, because the
  `tauri-resources-tracked` stage fails any `bundle.resources` path with 0 git-tracked files
  (`scripts/verify.sh:1520–1529`) and requires every system root to be bundled (lines 1535–1554):
  that would ship an empty directory claiming to be a package. The Option keeps PF/SF paths and bytes
  identical. Callers to update (all read PF or SF only, so `.expect` on a known system is exact):
  `src/rules_core/corpus_loader.rs:376`, `src/rules_core/sheet_rule.rs:4047`,
  `apps/desktop/src-tauri/src/character_hub.rs:774`, `sf_catalog.rs:183,202,416`,
  `sf_adapter.rs:112,1650,1653,1675`, `crates/codex-ingest/src/bin/sheet_rule_convert.rs:92`.
  `PackageRoots.sheet_rules` becomes `Option<PathBuf>`; `live_sheet_rules_for(CyberpunkRed)` returns
  `None` (a missing package is "not verified", never a verdict, `corpus_loader.rs:353–354`).
- **The `tauri-resources-tracked` stage reads `Some("…")` arms** (its regex is `=>\s*"([^"]+)"`,
  `scripts/verify.sh:1539`): E0.1 updates the regex to accept `=> Some("…")` and skip `=> None`, and
  proves both with a planted arm.
- Converter-side matches (`codex-ingest`) give `CyberpunkRed` a named refusal ("no PCGen data for
  cyberpunk-red"), never a fallback to Pathfinder.
- Saved-character envelope `game_system = "cyberpunk-red"` (`src/saved_character/mod.rs:34`).

**Enforced by:** E0.1 (tests in `game_system.rs`, `tauri-resources-tracked` PASS + planted-arm
plant); E0.6 (PF/SF byte-identical).

**Revisit if the operator disagrees:** the empty-directory alternative is one line in
`game_system.rs` plus a tracked `.gitkeep`; it would need a registry entry in
`wired-integration-stubs-registry.md`.

---

## §9 — Tauri adapter and desktop modules

**DEFAULT (brief, "Architecture").**

- `apps/desktop/src-tauri/src/rule_system_adapter.rs` `resolve_rule_system_adapter` (lines 148–157)
  gains `"cyberpunk-red" => CyberpunkAdapter`; `load_saved_character_via_envelope` (lines 175–182)
  routes a `game_system == "cyberpunk-red"` envelope to it. `StubAdapter` stops answering
  `"cyberpunk-red"`; stub registry entry 0002 stays accepted for the other placeholder rule sets
  (`solarus-arcanum`, `traveller`, `world-of-darkness`) and gains a "retired for Cyberpunk RED" update
  line (as SD-37 E4.6 did for Starfinder, `docs/governance/wired-integration-stubs-registry.md:75`).
- New modules mirror the SF set (`main.rs:42–49` declares `sf_adapter`, `sf_catalog`, `sf_choices`,
  `sf_creation`, `sf_drone_print`, `sf_level_up`, `sf_oracle_parity`, `sf_sheet_print`; SF commands at
  `main.rs:250–257` inside `invoke_handler` at line 240): `cpr_adapter.rs`, `cpr_catalog.rs`,
  `cpr_creation.rs`, `cpr_sheet_print.rs`, `cpr_improve.rs` (IP spend, role switch, therapy, install),
  each declared in `main.rs` and each command registered in `invoke_handler`.

**Enforced by:** E0.4 (`rule_system_adapter` test: `"cyberpunk-red"` resolves to `CyberpunkAdapter`,
RED on base where `StubAdapter` serves it); every card that adds a command lists `main.rs` in FILES
YOU OWN; `grep -c 'cpr_' apps/desktop/src-tauri/src/main.rs` rises with each.

**Revisit if the operator disagrees:** module names are internal; rename in one card.

---

## §10 — Engine: `src/rules_core/cpr/`, every number an `explanations` row

**DEFAULT (brief, "Engine").** New directory `src/rules_core/cpr/`:

| File | Computes (sheet numbers only) |
|---|---|
| `stats.rs` | 10 STATs; HP = 10 + 5 × ceil((BODY+WILL)/2); Seriously Wounded = ceil(HP/2); Death Save = BODY; Humanity max = EMP × 10; current EMP = floor(Humanity/10) (A-4: not below 0); MOVE; Luck pool = LUCK |
| `skills.rs` | 66 skills; base = STAT + level; ×2 cost flag; Basic skills at 2; 86-point creation budget; cap 6 at creation |
| `armor.rs` | SP per location (head, body), highest source wins; armour penalty to REF/DEX/MOVE applied once, worst penalty, MOVE floor 0 |
| `cyberware.rs` | Humanity Loss (preset at creation; entered after, A-3); slots per container, enforced (D-11); Skin Weave SP 7, Subdermal Armor SP 11, Grafted Muscle BODY +2 (cap 10), Linear Frame Sigma BODY 12 / Beta 14, Kerenzikov +2 Initiative; therapy cap (D-5) |
| `roles.rs` | 10 Roles; ability rank 1–10, start 4; rank-derived values (Combat Awareness points, Team Members, Believability, Backup tier, NET Actions, Moto bonus, Maker/Medicine pools) |
| `netrunning.rs` | deck slots (5/7/9 + Bodyweight Suit hardware-only +1 + Cyberarm deck +1, A-7); Program 1 slot, Black ICE 2, Hardware 1 or 2; NET Actions by Interface rank |
| `improvement.rs` | IP costs: skill 20 × new level, (x2) 40 ×, Role rank 60 × new rank; role switch when the current Role is rank ≥ 4 (A-1 price) |
| `reputation.rs` | level 0–10; Facedown printed as "<COOL + Rep> + 1d10" with the resolved number (cowardice = negative Rep) |
| `critical.rs` | wound-state thresholds; Death Save; stabilization DVs 10/13/15 printed with the computed thresholds; Base Death Save Penalty from recorded criticals |

Every computed value is an `explanations` row whose id starts `cpr.` (for example
`cpr.derived.hit_points`). The sheet renders the engine row; **no TSX re-derives a number** (SD-27
§29.1 twin rule, cited in `AGENTS.md`).

**Enforced by:** E2 cards; E4.1's test (every number on the CPR sheet equals one `cpr.` row; a +1000
shift of the rows shifts the sheet); NR-2.

**Revisit if the operator disagrees:** module boundaries are internal; numbers are fixed by the book.

---

## §11 — Data layout and transcription method

**DEFAULT (brief, "Data" and "Transcription method").**

- `data/cyberpunk-red/corpus/<kind>.json`: one file per kind, a JSON array of records. Each record
  carries `source_page` (`pNNN`), `name`, `kind`, the inventory's fields, and `text` (verbatim rule
  text). Kinds and expected counts: CUI §3. Plus `LICENSE.json` (`§4`) and `MANIFEST.json`.
- **Transcription:** Sonnet lanes transcribe from `$P/pNNN.txt` (hand-transcribing the CUI §4 pages
  from the page image), file-disjoint per kind. A count check per kind pins the MANIFEST count; every
  record has `source_page`.
- **The only oracle is the page image.** E1.V (Opus) checks a 10% random sample per kind against the
  page PNG (`pdftoppm -f N -l N -r 150 -png "$PDF" out`), with the seed of the random draw recorded.
- **Book-internal parity fixtures:** the HP table p080 (126 cells, CUI F-23); the 10 Streetrat
  skill templates each summing to 86 (F-25); the 10 Role cyberware packages' Humanity-Loss sums
  (F-66); the sample character sheet p121.
- **Never inside the git tree:** the PDF, `cpr-corebook-v122.txt`, `pages/`, page PNGs. Only the
  transcribed JSON and the receipts' sampled crops' **text** ship. A card that needs to cite an image
  writes its page number and crop box, not the image.

**Enforced by:** E1.0 (MANIFEST + count checks, Python and Rust); E1.V; `git ls-files | grep -ciE '\.(pdf|png)$'`
does not rise across E1 (receipt shows before/after).

**Revisit if the operator disagrees:** one-file-per-record is a mechanical split of each array.

---

## §12 — Desktop surfaces

**DEFAULT (brief, "Desktop").**

- Landing: `RULE_SETS` entry `id: 'cyberpunk'` (`apps/desktop/src/characterHub/LandingScreen.tsx:62`,
  `available: false` at line 65; `RuleSetId` union at line 14) → `id: 'cyberpunk-red'`, label "Cyberpunk Red"
  unchanged. This is the only id change. `resolveRuleSystemId` (`characterHubRuntime.ts:33–35`) passes
  it through unchanged.
- `characterCreationGate` (`characterHubRuntime.ts:79–88`) returns `form: 'cyberpunk'` for
  `'cyberpunk-red'` (the `form` union at line 69 gains `'cyberpunk'`).
- Components: `CyberpunkCreateForm.tsx` (Streetrat template picker, Edgerunner roller, Complete Package
  allocator; Lifepath roll-or-pick; skills allocator; starting kit; cyberware pack with Humanity
  preview; cash), `CyberpunkCharacterSheet.tsx`, `CyberpunkImproveDialog.tsx`; catalogs under
  `apps/desktop/src/cyberpunkCatalog/`; boundaries `apps/desktop/src/boundary/cpr*.ts`.
- `CharacterHubPage.tsx` gains `cyberpunk` branches beside the Starfinder ones (lines 11–13, 24,
  153–155, 223–226, 306–307).
- Layout follows the tranche-17 brief-2 registry (`apps/desktop/src/characterHub/sheetLayout.ts`:
  `LayoutBox` line 14, `SHEET_LAYOUT` line 43, `CREATE_LAYOUT` line 91): a CPR registry file
  (`cyberpunkSheetLayout.ts`) in the same shape. Moving a box is an edit to its entry.
- ui-smoke rows in `apps/desktop/scripts/ui-smoke/spec.json` (104 rows today, shape
  `{id, screen, steps[], marker, expect[], notes}`), then `npm run ui-smoke:doc` regenerates
  `docs/testing/ui-smoke-inventory.md` (the script refuses nothing; a stale doc is the risk it exists to
  avoid).
- **No Encumbrance box** on the CPR sheet (D-12).

**Enforced by:** E0.5, E3.2, E4.1–E4.7; R2 (no hand-kept tables).

**Revisit if the operator disagrees:** layout entries move boxes without code changes.

---

## §13 — Packaging

**DEFAULT (brief, "Packaging").** Every runtime read of `data/cyberpunk-red/` is a `bundle.resources`
entry in `apps/desktop/src-tauri/tauri.conf.json` (resources object at line 34; today 14 entries, none
for Cyberpunk RED) **and** an assertion in `apps/desktop/src-tauri/tests/packaged_resources.rs` (the
`for relative in [...]` list at lines 73–85 and the per-system load asserts at lines 113–119). Production
code reads through `support::paths::repo_root()` / `GameSystem::package_roots`, never
`env!("CARGO_MANIFEST_DIR")`.

**Enforced by:** E0.3 (entry + assertion + `tauri-resources-tracked`); E7.2 (CPR corpus loads under
the packaged root; clean-checkout build); G-7.

**Revisit:** none expected; this is a correctness control (the 2026-10 class-roster defect).

---

## §14 — Seeds

**DEFAULT (brief, "Seeds").** Three CPR starter characters — a **Streetrat Solo**, a **Complete
Package Netrunner**, an **Edgerunner Medtech** — restorable through the existing restore-starters
mechanism (`character_hub.rs:2471` `restore_starter_characters_at`, `:2214` `starter_seeds()`, command
registered at `main.rs:278`). Today `StarterSeed` (`character_hub.rs:2103`) holds a Pathfinder
`CreateCharacterRequest`; E7.1 generalises it to a system-aware request without changing the PF seeds'
bytes. The merge-check roster is CUI §6.

**Enforced by:** E7.1 (3 restore; ui-smoke opens each; PF starter bytes unchanged).

**Revisit if the operator disagrees:** swap a seed in CUI §6; E7.1 re-writes its build.

---

## §15 — Model tiering

**DEFAULT (brief, "Model tiering"), per `~/.claude/CLAUDE.md` (2026-09-29).**

| Work | Model |
|---|---|
| Engine, UI, merge checks, the E1.V verify lane, adversarial review | **Opus 5.5** (`model: 'opus'`) |
| Transcription lanes and well-specified mechanical work; every long-run wait | **Sonnet 5.5** (`model: 'sonnet'`) |
| Housekeeping (version bump, sweep) | **Haiku 4.5** (`model: 'haiku'`) |

- Merge checks are never downgraded. Long waits (> 20 min) never on Haiku.
- **Every `agent()` sets `model`.** Check: `awk '/agent\(/ && !/model:/' <script>` prints nothing.
- **Planning disclosure.** The brief was written by Fable 5.1 (planning tier). This package was
  transcribed from it by an Opus 5.5 sub-agent. No separate review card is in the board; the lead may
  run one before launch.

**Enforced by:** `epic-breakdown.md §0` Tier column; `kanban.md` Tier column; R3.

**Revisit if the operator disagrees:** edit the Tier column; the script reads it.

---

## §16 — Unattended-mode protocol

**OPERATOR mode ruling (§3)** plus the standing directive of 2026-08-01 (verbatim): *"include
instructions to all 3 that indicate they will be running in unnattended mode since i will be out of
town while this runs. They may not stop to ask questions - it might be days before i notice."* The
protocol appears in three places: the `workflow-instruction.md` OPERATING METHOD callout, this
decision, and `progress.md` Cycle 0.

1. **Clear first.** A blocker on the Definition of Done is decomposed and attacked.
2. **If only an operator ruling can clear it,** mark **only that card** `blocked-escalated`; write the
   exact ruling, write scope or precondition under `progress.md ## Open blockers` with the command and
   its output. Continue **every independent card**.
3. **Never open the final PR while any card is open, partial or blocked-escalated.**
4. A choice inside scope with a safe default (`§16.1`): take it, log it in `progress.md ## Decisions
   taken on safe defaults` with the alternative not taken, continue.
5. **No `AskUserQuestion`, no `clarify`.**
6. **Report after each epic** (operator mode ruling): the epic's merge-check card writes a one-screen
   "Epic report" block into `progress.md` (cards done, figures with commands, seed deltas, open
   blockers). That block is the report.

### §16.1 Safe defaults

| # | Decision point | Safe default |
|---|---|---|
| SD-a | A page's text and image disagree | The image wins; record both in `transcription_note` |
| SD-b | Compute vs print | Print unless the value lands on the sheet as a number (`§6`) |
| SD-c | A count differs from its CUI figure | Re-derive with two implementations. If both agree on the new value, the lane records it in its receipt and emits a `retro.py correction` (`--verified-by` = both commands). Lanes never edit `MANIFEST.json` (one writer): E1.V applies every recorded correction in one commit. Never edit a count to match the data without the two implementations |
| SD-d | A rule the book does not state | A named refusal on the sheet, never a guess; if it blocks a DoD item, `blocked-escalated` with the page cites |
| SD-e | A PF or SF render byte-diff | A defect. Fix it. **Never re-baseline PF or SF** |
| SD-f | A worktree, branch or file another session may own | Do not touch it; log it |
| SD-g | Dice in a rule | Print the dice; compute only the resolved additive term (`§6`) |
| SD-h | A `(kind, name)` collision inside Cyberpunk RED (e.g. Bodyweight Suit in armour and gear) | Keep both records with distinct `kind`; the catalog shows both |
| SD-i | A usage-limit stop condition (`§16.3`) | Stop dispatching, write a resume receipt |
| SD-j | No CI on `tranche-18` pushes | CI evidence = the closure PR's `pr-tests`, awaited by E7.11 |
| SD-k | PR #398 not merged when C1 is ready | **Revised 2026-10-13 (lead):** the lead cut `tranche-18` from `tranche-17-ui`'s tip `bb6107aee0`, which is PR #398's head; `git merge-base --is-ancestor origin/develop bb6107aee0` holds, so `tranche-18` already contains everything `develop` has plus #398's content. C1 does not wait for the merge. E7.11 checks `git merge-base --is-ancestor bb6107aee0 origin/develop` before opening the PR; if #398 landed as a squash (prior PRs #394–#397 were merge commits), E7.11 rebases `tranche-18` onto `origin/develop` first and records it |
| SD-l | Rebase conflict | **`kanban.md`: never keep both sides** (two lanes editing adjacent rows conflict, and keeping both duplicates both rows: tested 2026-10-13 with `git merge-file --union`). Take the upstream file (`git checkout --ours -- docs/release/SD-38-cyberpunk-red/kanban.md`; in a rebase "ours" is `origin/tranche-18`), re-apply **only your own card's row**, then `awk -F'\|' '/^[\|] (C\|E)[0-9]/{gsub(/ /,"",$2);print $2}' kanban.md \| sort \| uniq -d` prints nothing and the 68-row check holds. **`progress.md`:** keep both sides in the append-only sections (Cycle log, Epic reports, Decisions taken on safe defaults, Open blockers); in the Summary table take upstream and re-count from `kanban.md`. Any other file: abort, `blocked-escalated` with the paths — except SD-p |
| SD-m | The fresh-install check cannot run on this box (no display / no install rights) | E7.3 `blocked-escalated` with the exact operator action; the bundle waits at E7.5 |
| SD-n | A page-image check is impossible (`pdftoppm` missing) | `blocked-escalated` on that lane's card only |
| SD-o | A GM table with no inventory (E1.8) has an unclear row count | Two implementations; if they disagree, count by reading the image and record all three |
| SD-p | A rebase conflict confined to the lane-anchor lines of `src/rules_core/cpr/mod.rs` (A-10) | Keep both sides (each lane adds `pub mod` lines under its own anchor) |

### §16.2 Watchdog: one long Workflow run

The bundle runs as one long `Workflow` run with a recorded run id; recovery starts a **new** run with
`START=<card>` (SD-37 lesson: never resume a script with parallel lanes). No cron.

### §16.3 Quota stop rule

Stop dispatching new lanes when (a) any `agent()` result carries
`/usage limit|rate limit|quota|limit reached/i` (binding), (b) reported subagent tokens since the weekly
reset reach **10 M** (an **estimate** carried from SD-37 `decisions.md §12.3`; only if the runtime
reports usage), or (c) the harness shows a weekly reading ≥ 85%. Lanes in flight finish. Write a resume
receipt in `progress.md`. **Never downgrade a model.** Resumption is not automatic.

### §16.4 Crash-resume recipe

1. Keep every dirty worktree. Do not reset, stash or clean. 2. Find the run id in `progress.md`; read
`journal.jsonl`. 3. Start a new run with `START=` the first unrun card; the shared prefix must be
byte-identical to `artifacts/cycle_0/sd38-workflow.js`. 4. Log a `retro.py incident`.

### §16.5 Environment guards (every dispatch prefix)

Carried verbatim into `workflow-instruction.md §2.1` and `§6`: one writer per tree; own
`CARGO_TARGET_DIR=/home/ubuntu/cargo-target/<lane>`; `df -h /` first; ≤ 3 cargo lanes; never
`git stash`; never `git add -A`; `pgrep -f '[c]argo'` before a full sweep; both cargo workspaces;
`--test-threads=8`; `nohup` + not-run skeleton past 10 minutes; `npm run typecheck && npm test && npm run ui-smoke:doc`
in `apps/desktop`; four-check + identifier audit; TDD with the failing reason quoted; receipts with every
figure's command; `retro.py` events with `RETRO_ACTOR`.

**Enforced by:** `workflow-instruction.md` callout and §2.1/§6; `progress.md` Cycle 0.

**Revisit if the operator disagrees:** lower the quota threshold; switch to a cron (count crontab
jobs before and after).

---

## §17 — Closure order (STANDING, operator-pinned 2026-09-14)

All cards `complete` → retrospective written and cited → full worktree/branch sweep → **release
notes** → **architecture truth-up** → **graphify LAST**, over a tree where unfiltered
`git status --porcelain` prints nothing and HEAD = `origin/tranche-18` → **PR as the final action**.
The operator merges. The template's §11 order (release notes after graphify) is the known defect
FSR-C9 of SD-37; this bundle's `workflow-instruction.md §11` uses the corrected order.

**Enforced by:** E7.5–E7.11 dependencies; E7.10's receipt records the indexed SHA and the empty status.

---

## §18 — Per-cycle discipline

Every code-bearing card: TDD (RED confirmed for the intended reason, quoted in the receipt); the
identifier audit and the four-check wired-integration audit (`docs/governance/no-stub-mvp-doctrine.md
§"Per-cycle audit"`), on added lines only, after the local commit and before the push; a receipt at
`artifacts/epic_<n>/<card>_cycle_receipt.md` with every figure's command; the three-status vocabulary
`complete | partial | blocked-escalated`; `retro.py` events when they happen; per-seed deltas.

**Enforced by:** `workflow-instruction.md §6–§7`.

---

## §19 — SD-37 lessons as rules

The SD-37 retrospective's "Changes for the next bundle" (`docs/retro/sd37-retrospective.md:273`)
become rules **R8–R13** in `workflow-instruction.md §12`, beside SD-36's R1–R7 where they still apply:
the script reads `Depends on` from the board; never resume a parallel script (new run with `START`);
fences have a written exit (a completed epic's files open to later serial cards, gate re-run);
clippy and the wired-integration audit test run in every code card; a generated artifact has a
re-run-and-diff check; every figure copied from an earlier doc is re-derived before use.

**Enforced by:** `workflow-instruction.md §12`.

---

## §20 — DEFAULTs for the inventories' UNCLEAR items (brief, verbatim content)

Each item below is the brief's DEFAULT. CUI §5 maps every UNCLEAR to one of these or to `§21`.

### §20.1 D-1 — Range DV rows vs weapon types

**DEFAULT.** Map Medium / Heavy / Very Heavy Pistol → **Pistol** row; Heavy SMG / SMG → **SMG**;
Shotgun → **Shotgun (Slug)**; Assault Rifle; Sniper Rifle; Bows & Crossbows; Grenade Launcher; Rocket
Launcher (8 rows, CUI F-71). **Verify against the PDF image of p174 before encoding.**
**Enforced by:** E1.6 (image check recorded in the receipt; each `range_dv` row lists its weapon types);
E4.3 (each weapon on the sheet shows its row). **Revisit if the operator disagrees:** edit the
`weapon_types` field of the `range_dv` records; nothing else changes.

### §20.2 D-2 — Netrunner brain damage

**DEFAULT.** Brain damage subtracts from HP; armour is ignored (from the "direct" wording, p202). The
sheet prints the rule text. **Enforced by:** E2.9 (printed rule line); E1.7 `transcription_note`.
**Revisit if the operator disagrees:** change the printed line in `cpr_sheet_print.rs`; no number depends
on it.

### §20.3 D-3 — Air Pistol concealability

**DEFAULT.** The chapter table (p096) wins; record both (p096 "Very Heavy Pistol", p348 "Exotic Medium
Pistol") in `transcription_note`. **Enforced by:** E1.4. **Revisit:** flip the `base_weapon` field.

### §20.4 D-4 — Luxury vs Super Luxury DV both 29

**DEFAULT.** Check the PDF image (p141 repair sidebar, p150 Maker table); if the image agrees, keep
29/29. **Enforced by:** E1.1 and E1.3 receipts (image crop box cited). **Revisit:** edit the two records.

### §20.5 D-5 — Therapy max-Humanity penalty

**DEFAULT: COMPUTE.** Max Humanity = 10 × base EMP − 2 per installed cyberware piece − 4 per borgware
piece; 0-HL items excluded; the free Standard Hand / Standard Foot excluded. Confirm the p231 text
against the image first (CUI §4). **Enforced by:** E2.5 (fixtures for each clause); E5.4 (therapy cannot
raise Humanity above the cap). **Revisit if the operator disagrees:** turn the cap into a printed rule;
E2.5 and E5.4 change.

### §20.6 D-6 — "10 Humanity Loss (−2 EMP)" strings

**DEFAULT.** Compute EMP from Humanity; print the loss, never the "(−2 EMP)" fragment (p118 Media and
Lawman print −2 for HL 10, which the p081 rule makes −1 from any starting multiple of 10).
**Enforced by:** E2.5 / E3.7 (EMP after the Media and Lawman packs is computed; the string is absent
from records' printed fields: `grep -c '(-2 EMP)' data/cyberpunk-red/corpus/starting_kit.json` → 0).
**Revisit:** none expected (arithmetic).

### §20.7 D-7 — Starting Maker / Medicine specialty points at Rank 4

**DEFAULT.** The book is unclear. The player distributes the rank points at creation (allocator),
and the sheet prints the book's table. **Enforced by:** E3.5. **Revisit if the operator disagrees:** a
fixed starting split replaces the allocator in E3.5.

### §20.8 D-8 — Lawman Backup 16/15/14

**DEFAULT.** Keep as printed (Ranks 8/9/10). **Enforced by:** E1.1 (record equals the page);
E2.6 fixture. **Revisit:** edit three numbers.

### §20.9 D-9 — Exec Corp Kind "1d6" with 10 rows

**DEFAULT.** Treat as 1d10; flag in `transcription_note`. **Enforced by:** E1.2. **Revisit:** the
`dice` field.

### §20.10 D-10 — Price-scale oddities

**DEFAULT.** Transcribe the number as printed ("50,0000eb" Luxury Penthouse; High Fashion Jewelry
"50,000eb (Luxury)"; "10b (Cheap)" Road Flare) and flag it in the record's `transcription_note`.
**Enforced by:** E1.4. **Revisit:** correct the number in a later rewording pass (`§4`).

### §20.11 D-11 — Cyberware slot enforcement

**DEFAULT: ENFORCE** at creation and install (reject an over-slot install). The chassis computes
totals; slot counts are totals. (The cyberware inventory recommended no enforcement; the brief
overrides it because a wrong install is a sheet-total defect.) **Enforced by:** E2.5 (refusal test per
container: Fashionware 7, Internal 7, External 7, Neural Link 5, Cybereye 3 per eye, Cyberaudio Suite 3,
Cyberarm 4, Cyberleg 3); E3.7; E5.3. **Revisit if the operator disagrees:** turn the refusal into a
printed warning.

### §20.12 D-12 — Encumbrance

**DEFAULT.** None in this game (`grep -il encumbr $P/p*.txt` → no files, gear §15). No Encumbrance box
on the CPR sheet. **Enforced by:** E4.1 (a test that the CPR sheet layout has no `encumbrance` box).
**Revisit:** none.

---

## §21 — AUTHOR DEFAULTS (not in the brief; flagged for operator review)

The brief's DEFAULT list does not cover these points. Each is needed to close a card. Each is the
smallest reading the book supports, and each is listed in `README.md §6` for the operator.

| # | Point | Author default | Enforced by | Revisit if the operator disagrees |
|---|---|---|---|---|
| A-1 | Multiclass: IP price of Rank 1 in a new Role (p144 says only "buying Rank 1 … with Improvement Points") | **60 IP** — the p411/p412 table's Rank 1 price; the dialog prints the p144 rule text | E5.2 | change one constant in `improvement.rs` |
| A-2 | Starting Moto vehicle picks, Exec team, Cryosystem points at Rank 4 | Same reading as D-7: the per-rank picks the book describes are made at creation, one per rank (Exec team count is computed from the rank rule, 1 at rank 3–4) | E3.5 | a fixed starting pick list replaces the allocator |
| A-3 | Humanity Loss after creation is a dice roll (p112); in-play dice are not in the DoD (DEF-2) | The install dialog accepts the rolled HL, entered by the player or drawn by the same seedable table roller creation uses (`technical-design.md §5`); the engine rejects a value outside the dice range (2d6 → 2–12, 4d6 → 4–24, 1d6 → 1–6, 1d6/2 round up → 1–3) | E5.3 | entry only, no roller |
| A-4 | EMP for negative Humanity | `EMP = max(0, floor(HUM/10))` (p233 says EMP "Equal to 0 and You Have Negative Humanity") | E2.1 | drop the clamp |
| A-5 | Option slots of extra Cybereyes (MultiOptic Mount) and extra Cyberarms (Artificial Shoulder Mount) | each added foundation carries its own printed slot count (3 per eye, 4 per arm; p361 "Each Cybereye has 3 Option Slots") | E2.5 | set the extra foundations' slots to 0 |
| A-6 | Attachment slot cost "not stated" for 5 of 8 attachments | 1 slot (the three that cost more say "Requires 2 Attachment Slots"); `transcription_note` on each | E1.4 | edit 5 records |
| A-7 | Bodyweight Suit hardware slot and Cyberarm deck slot on one deck; an over-capacity deck | both slots apply (p351, p365; nothing prohibits it); a loadout over the deck's slot total is refused, the D-11 reading applied to deck slots (p197 "the same limited slots") | E2.9, E6.3 | make the slots exclusive; turn the refusal into a warning |
| A-8 | Linear Frame Sigma (gear) needs 1 or 2 Interface Plugs (p355 states both) | record both in `transcription_note`; enforce neither | E1.4 | enforce one |
| A-9 | Starting cash on the sheet | computed: the method's budget (500eb Streetrat/Edgerunner; 2,550eb Complete Package; the 800eb fashion budget is lost if unspent, p106) minus purchases at creation; after creation it is a write-in | E3.6 | print the budget only |
| A-10 | Two engine lanes add modules to `src/rules_core/cpr/mod.rs` | E0.4 writes three anchor comments (`// lane A modules`, `// lane B modules`, `// lane C modules`), separated by blank lines; each lane adds its `pub mod` lines under its own anchor; SD-p resolves a conflict there by keeping both sides | E0.4, E2 | serialise lanes B and C after A |
| A-11 | Where a CPR character's input is saved | **REVISED after run 4 (E0.4 declined, premise refuted):** `SavedCharacterEnvelope` has no serde derive; `local_store.rs` hand-renders/parses it (`render_envelope` :329, `parse_envelope` :493 rejects unknown keys :539) and `save`/`load` require a PF `CharacterInput` (race, class levels, six scores); 39 struct-literal sites in 13 files (`grep -rn 'SavedCharacterEnvelope {' --include=*.rs src apps/desktop/src-tauri crates tests \| awk -F: '{f[$1]++;n++} END{print n, length(f)}'` → `40 14`, one is the definition). Ruling = the sibling-file design: a CPR character is its own directory under the SAME characters root (`character_hub.rs:1109 characters_root_from_app_data_dir`, so `resolve_character_root` keeps working for load) holding `envelope.txt` in local_store's key=value grammar with `game_system = cyberpunk-red` and `cpr_character_input.txt` (NO `authoritative_character_input.txt`); `src/rules_core/cpr/store.rs` (new) owns `CprSavedCharacter { character_id, revision_id, saved_at, schema_version, app_or_runtime_version, display_label, input: CprCharacterInput }` with save/load/list/delete; `SavedCharacterEnvelope` and its 39 literals are untouched. Two generic touches in `src/saved_character/local_store.rs`: (a) `list_all` neither lists nor reports as unreadable a directory whose `envelope.txt` names a game system the store does not own (`pub fn owns_payload(game_system: &str) -> bool`, false only for `cyberpunk-red`); (b) `pub fn peek_game_system(root) -> Result<String>` parses `envelope.txt` alone, so `load_saved_character_via_envelope` routes a CPR root to `CyberpunkAdapter` before any PF payload read | E0.4, E0.5, E0.6 | the envelope field (rejected: it needs made-up PF data on the shipping path) |

---

## §22 — Planned capability deferrals (not in the Definition of Done)

From the brief's "Not in DoD" list. The test (`docs/governance/deferral-revisit-doctrine.md`): was
this scope in the Definition of Done when the work was scoped? It was not, for each row.

| ID | Capability | Revisit condition (checked at E7.5) |
|---|---|---|
| DEF-1 | NET Architecture **generator** (tables ship print-only) | the operator asks for it (the command in `forward-scope-register.md §4`) |
| DEF-2 | In-play dice engine (the sheet prints dice). Creation-time table rolls — Streetrat row, Edgerunner per-STAT rolls, Lifepath "roll or pick" — are **in** scope (brief, "Desktop") through one seedable roller | the operator asks for in-play rolls |
| DEF-3 | NET-run simulation | the operator asks for it |
| DEF-4 | Vehicle-combat simulation | the operator asks for it |
| DEF-5 | NPC / Mook generator (stat blocks ship as print-only catalog pages) | the operator asks for it |

Rows are in `forward-scope-register.md §4`. Only the operator moves scope between classes.

---

## §23 — No PCGen; the oracle pin is irrelevant to SD-38

**OPERATOR (§3).** No PCGen data exists for Cyberpunk RED. The source is the PDF text only.
`crates/codex-ingest`'s PCGen converter is **not** used for SD-38 content. The PCGen oracle pin
(`scripts/pcgen-oracle-pin.env`) does not govern any SD-38 figure; SD-38 receipts quote no
`PCGEN_ORACLE_SHA` for CPR figures. The pin still governs the **PF and SF** gates SD-38 re-runs (E0.6,
E7.4: `sheet_rule_convert --check`, the residue gate), because those stay in the widest verify.

**Enforced by:** no CPR card lists `crates/codex-ingest/**` in FILES YOU OWN except E0.1's
named-refusal match arms.

## §24 — E7.10 closed on the graphify node-count guard outcome (lead ruling, 2026-10-15)

`update_graphify.py` exited 1 on the dedup-collapse guard (new 648,327 nodes vs existing 648,328;
`graph.json` not written; no force flag; receipt `artifacts/epic_7/E7.10_cycle_receipt.md`; indexed SHA
4d1af843d1 = `origin/tranche-18`; tree clean). That is exactly the card's stated outcome ("on the node-count
guard (exit 1) file the receipt and stop — never force") and the documented non-blocking policy
(operator-confirmed 2026-07-20), and it is how SD-37 closed its graphify card
(`docs/release/SD-37-starfinder-1e/decisions.md §23`, 2026-10-08). Ruling: E7.10 is `complete` on that
outcome; the semantic graph is unchanged and indexes no SD-38 content; a full semantic re-extraction is an
**operator decision** (cost), carried in `forward-scope-register.md` FSR-C3. E7.11 proceeds.
