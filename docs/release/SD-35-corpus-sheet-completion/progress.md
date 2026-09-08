---
canonical: true
owner: god-emporer
bundle_id: SD-35
status: planning-ready — not launched (launch-readiness audit passed 2026-09-08 00:10Z)
date: 2026-09-07
---

# SD-35 Progress

Live cycle-by-cycle record. Cycles **prepend** their entry (newest first) and update
`kanban.md` in the same commit, via `workflow-instruction.md §5`'s retry protocol.

Every cycle entry carries, verbatim from its receipt: the `cycle_scope_gate.py --min 500` line,
the `--receipt` rows (closed / relabeled / rust_lines_changed / ratio / builds_recorded /
pcgen_live_files), and the refused-token remainder. **An entry without the scope-gate line is a
process defect** recorded by the epic wrap-up.

## Open blockers

*(none — an entry here pauses the bundle and is a request for an operator ruling;
`decisions.md §6`)*

## Status matrix

| Epic | Criteria | Complete | In progress | Not started |
|---|---:|---:|---:|---:|
| 1 — Tax cut | 6 | 6 | 0 | 0 |
| 2 — Sheet rule | 5 | 3 | 0 | 2 |
| 3 — Place and surface | 4 | 0 | 0 | 4 |
| 4 — Resolve and verify | 3 | 0 | 0 | 3 |
| 5 — Residues | 5 | 0 | 0 | 5 |
| 6 — PCGen exit | 4 | 0 | 0 | 4 |
| 7 — Closure | 3 | 0 | 0 | 3 |
| **Total** | **30** | **9** | **0** | **21** |

Corpus at the `tranche/15` cut (2026-09-07, `4c6c57eb9f`, identical to authoring at `5f6b18f4e3`):
`DONE=26123 of 49438`; non-DONE 23,315 of 49,438. Live-side PCGen residue at authoring: 78 files by coarse grep
(`content-unit-inventory.md §6`); the exact baseline is AT-35-E1-005's first run. Both
re-measured at the cut by the launch-readiness audit.

## Cycle log

### 2026-09-08 — AT-35-E2-003 cycle 1 — `sheet-complete-status` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (status-vocabulary cycle — closes zero units by design)` — `decisions.md §2`. `pcgen_residue_gate.py --check` at start (`9c8a3abe3d`): `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=442 ratio=n/a builds_recorded=2 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 9c8a3abe3d --before /tmp/wi-before-AT-35-E2-003.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-003` at the tree of `a81c2a005c`).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `a81c2a005c` — not risen (the rung reads the converter's package and the evaluator's output only).
- **Refused tokens:** none (no converter run; `sheet_rule_convert -- --check` → `records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081 verdict=PASS`).
- **What landed:** `sheet-complete` in `v06_work_inventory`'s `status_vocabulary` with `technical-design.md §3`'s meaning; the rung `apply_sheet_complete_rung` (run last of the status passes) lifts `engine-does-not-hold`/`ingested-magnitude` units whose id has a `SheetRule` in `data/sheet_rules/`, is absent from `_refused.json`, whose kind has an on-screen test (list pinned to the frontend test's `KINDS` by reading the file), and which `rules_core::sheet_rule::evaluate` renders for the probe character (the deterministic Human Fighter 1 holding the rule outright) — evidence `sheet_rule_rendered:<number|dice|words>`; `sheet-complete` joins `DONE_RUNG_STAMP_STATUSES` (a regen on a tree missing the package fails loudly). Consumers, found by grep: `completion_atlas.py` (DONE; DONE-evidence requires the rendered-form marker), `pf1e_dashboard_producer.py` (`done` for every wiring class), `test_cycle_scope_gate.py`, `companion_chassis.rs` `HELD_STATUSES`. **The inventory is not regenerated here** — AT-35-E2-005 does that once; the rung over the live package renders all **47,628** top-level rules (number=8017 dice=1461 words=38150, 2.28 s) and the projection from the committed inventory is **21,911** units moving (17,640 `engine-does-not-hold` + 4,271 `ingested-magnitude`).
- **Verification (one pass, `decisions.md §3`):** `cargo test --locked --no-run -j 6` exit 0 (2 min 53 s); `--lib` → 3214 passed / 0 failed / 14 ignored; `--no-fail-fast -j 6` → **411 binaries, 411 ok, 8,715 passed, 0 failed, 67 ignored**; clippy 0 warnings on `--lib --bin v06_work_inventory`; python RED (`132 run, 5 failures + 1 error`) → GREEN (`143 run, OK`); atlas `unclassified=0 overlap=0 done_evidence_violations=0`; shape-engine-boundary / missing-engine-tables / denominator-gate / pi-sweep green; literal scan 0; `token_coverage.py` absent until AT-35-E2-004; desktop and frontend at epic cadence (no `apps/` touch).
- **Discoveries (2 `correction` events, `docs/retro/events/at-35-e2-003.jsonl`):** the census evidence is not a literal equality — before **6**, after **7**: the five status-branching files are in both sets, `formula_interpreter_corpus_wide.rs` (before-only) is an F1 pin-history narrative, the two after-only files are the RED→GREEN tests (`…-b92fcf`); the producer's grid test enumerated 9 of 11 vocabulary words (the two oracle words never added — now 10 of 12; `…-41c58b`). Also: 4,681 of 47,628 top-level rules are `print: false` (R1) and are stamped with their value's form.
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-003_cycle1_receipt.md`. Code `a81c2a005c` (after `f731af3759`, a fold of a live `sd31-transcribe` retro append). **Epic 2: 3 of 5 complete — AT-35-E2-004 next.**

### 2026-09-08 — AT-35-E2-002 cycle 1 — `live-evaluator-and-sheet-section` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (live-evaluator + sheet-section cycle — closes zero units by design)` — `decisions.md §2`. `pcgen_residue_gate.py --check` at start (`643cc89bba`): `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=1818 ratio=n/a builds_recorded=3 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 643cc89bba --before /tmp/wi-before-AT-35-E2-002.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-002` at `909bb0837c`).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `909bb0837c` — not risen (the evaluator, loader, DTO and section read `SheetRule`/`VarTable` JSON and the chassis output only).
- **Refused tokens:** none (no converter run; `sheet_rule_convert -- --check` → `records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081 verdict=PASS`).
- **What landed:** `src/rules_core/sheet_rule.rs` gains the evaluator (`technical-design.md §2`): exact-rational `Rat` with ONE truncation at the `SheetValue` boundary; `Expr` leaves over `CharacterFacts` (built from `CharacterInput` + `PilotBaseChassisComputation`); the two-valued `Applies` gate plus `Situational`; the `Var` contribution fold by bonus type (`STACKING_TYPES`, `Stack`, `Replace`); slot filling with family order and `pick_last` / `suppress_when_all_zero`; dice folding and the damage-die ladder; the held-set fixpoint over the seed (race, classes, feats, traits, equipment, spells, skills, the chassis' grounded `class_feature.*` records joined by `<class>_<feature>` slug, the race resolver's applied-trait keys) with `Rule`/`Class`/`Race`/`Deity`/`Choice` grants gated by `when` + `applies`, `FactDeclare`, `CountsAs`, `Waives`/`Revokes`; `render_sheet` sorted by kind then label. `corpus_loader::load_sheet_rules` reads `data/sheet_rules/` (47,628 rule files + 5,081 `_vars/` → 66,147 rules in 2.66 s debug, parallel). `PilotBaseChassisComputation.sheet_lines` + `with_sheet_rules`. Desktop: `SheetLineDto`, the package loaded once per process, `LoadSavedCharacterResponse.sheet_lines` / `sheet_rules_unavailable_reason` on both response constructors; `CharacterSheet.tsx` renders one generic **Rules and features** section grouped by kind in the Actions tab (a `words` line renders no number); `buildClassFeatureSurface(..., sheetLines)` keeps only records with no rule in the `Not computed` lane.
- **Verification (one pass, `decisions.md §3`):** `cargo test --locked --no-run -j 6` exit 0; `--lib` → 3214 passed / 0 failed / 14 ignored; `--no-fail-fast -j 6` → **411 binaries, 411 ok, 8,710 passed, 0 failed, 67 ignored**; `cargo test --locked --lib sheet_rule` → 28 passed (the three value forms on real records — Ill Omen `DC 13` at Cha 14 / `Resolved(15)` on the design shape at spell level 3; Longsword `1d8` / `1d8+2` / `1d8+4` (Str 18) / `1d10` (one step); Magical Knack `Words` → `Wizard`; the per-kind gate over all 66,147 rules of 19 kinds; fixpoint; var fold; the fixture fighter's 45 lines with Acrobatic `+4` at 10 ranks); frontend `101/101` files (`rulesAndFeaturesSection.test.ts`: 19 per-kind DOM tests + 5; `classFeaturesModel.test.ts` +1), `tsc --noEmit` clean; desktop crate `574 passed / 0 failed` incl. the reach-gate IPC test on a Human Fighter 3 created through `create_character_at_root`; clippy 0 warnings on the lib and the desktop tests (3 `should_implement_trait` fixed in-cycle); atlas / shape-engine-boundary / missing-engine-tables / denominator-gate / pi-sweep green; literal scan 0; `token_coverage.py` absent until AT-35-E2-004.
- **Discoveries (3 `correction` events, `docs/retro/events/at-35-e2-002.jsonl`):** the chassis output carries fewer leaf facts than `technical-design.md §2` claims (size from `race_tables::race_size` for the 7 CRB races, walk speed from the `race.<slug>.trait_bundle.speed` record, the rest 0 and named in `CharacterFacts::from_character`; `…-796340`); the package carries no `Granter::Class`/`Race` rows (`python3` census over `data/sheet_rules/{core_rulebook,advanced_players_guide,bestiary,ultimate_psionics}/*/*.json` `granted_by`: `Rule` 8,103, `ClassSpellList` 5,149, `Deity` 2,126) and 1,187 of 1,738 CRB `class_feature` rules have no `granted_by` (`…-decb80`) — placement is bridged from the engine until AT-35-E3-001; the §6 wired-integration grep matches 3 rulebook-prose lines in the generated package, none in code (`…-7cbeb2`). Also: templates are not universal (the first fixpoint held every ungranted `applies: Always` template — fixed, pinned); `fighter_bonus_feats` prints 23 lines (22 `#bonusN` siblings) — AT-35-E3-001's placement shape.
- **Gate self-heal:** `verify.sh --only figure-provenance` was red on one pre-existing AT-35-E2-001 receipt line (`~23 s` with no command) — command added in this cycle's docs commit; `violations=0` after.
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-002_cycle1_receipt.md`. Code `909bb0837c` (after `097e7c1aa5`, a fold of a live `sd31-transcribe` retro append). **Epic 2: 2 of 5 complete — AT-35-E2-003 next.**

### 2026-09-08 — AT-35-E2-001 cycle 1 — `sheet-rule-converter` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (converter-building cycle — closes zero units by design; AT-35-E2-005 is the pass that moves the population)` — `decisions.md §2`. `pcgen_residue_gate.py --check` at start: `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=6463 ratio=n/a builds_recorded=3 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 928272a444 --before /tmp/wi-before-AT-35-E2-001.json --after docs/work-inventory.json`; `builds_recorded` 3: a converter-building cycle iterates build → run → `--check` until the package is clean).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `72ad0be010` — not risen.
- **Refused tokens:** 1,810 of 49,438 records (659 non-DONE of 23,315) in 82 refusal strings / ~16 shapes — `no_corpus_record=829, FORMULA:var(COUNT)=211, unmapped:STARTSKILLPTS=162, unmapped:SLOTS=95, FORMULA:malformed=87, SPELLS (PI-redacted)=78, BONUS:[redacted PI]=62, unmapped:ALTTYPE=49, unmapped:SPELLSTAT=49, unmapped:MODTOSKILLS=41, DEFINE (PI-redacted)=40, unmapped:MEMORIZE=38, …` (full list: `data/sheet_rules/_refused.json`; deferral event `1788840316824-at-35-e2-001-de4597`).
- **What landed:** `src/rules_core/sheet_rule.rs` (schema v2, no source-format reading), `src/pcgen_import/sheet_rule/` (the converter: 249-row table transcribed and proven both ways; corrected corpus-wide `.MOD` index with `_pfs/` skipped and KEY matching — B9/R3; class continuation rows + level lines — B5; the no-`raw_tokens` row read — B7; the pinned DEFINE/BONUS:VAR index — B2; own formula parser; PRE → `Applies`; prose slots; PI term screen on every text), `src/bin/sheet_rule_convert.rs` (regenerate / `--check` / `--one`), `data/sheet_rules/` (`records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081`, ~23 s per pass; literal grep 0), `tests/sheet_rule_convert_gate.rs` (26 tests: Ill Omen's DC = `Sum([Const(10), Const(1), AbilityMod(Cha)])`, the longsword = `Dice{"1d8", None}`, Magical Knack = `Text` + `offers` + `ChoiceName`; 19 per-kind gates; literal scan; freshness; determinism), `verify.sh` stage `sheet-rules-check` (46 stages).
- **Verification:** `cargo test --locked --no-run` exit 0; `cargo test --locked --no-fail-fast -j 6` → 411 test binaries, 410 ok, 1 FAILED — the lib binary at the pre-fix tree (3203 passed, 3 failed: `sheet_rule::{ctx,formula,table}::tests` expectation drift, fixed in the same cycle); re-run at HEAD `cargo test --locked --lib -j 6` → 3206 passed, 0 failed, 14 ignored; 8,699 tests passed across the run; converter gate binary 26/26; clippy 0 warnings on the touched targets; `sheet_rule_convert --check` `verdict=PASS`; `completion_atlas.py --check` `unclassified=0` (atlas `derived_at` re-stamp reverted); `shape_engine_boundary.py`, `missing_engine_tables.py` `violations=0`; `denominator_gate.py` `files_checked=29 violations=0`; `verify.sh --only pi-sweep` PASS.
- **Discoveries:** 24 token heads outside the mapping table once the closure is read from the pinned tree (STARTSKILLPTS 162, SLOTS 95, ALTTYPE 49, SPELLSTAT 49, MODTOSKILLS 41, MEMORIZE 38, …; correction `1788840316691-at-35-e2-001-4e50ce`); the epic's `SpellLevel` is the table's `Const(<spell level>)` (`…-8ed3da`); no corpus record carries a `+N` die literal (`…-569089`); refusal shapes > 10 by construction of the converter-building cycle — the §8 signal for AT-35-E4-001's scoping, not a blocker on this criterion.
- **Receipt:** `artifacts/epic-2-sheet-rule/AT-35-E2-001_cycle1_receipt.md`. Code `72ad0be010`. **Epic 2: 1 of 5 complete — AT-35-E2-002 next.**

### 2026-09-08 — AT-35-E1-004 cycle 1 — `ratio-row-and-gate-scope` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (gate-retargeting cycle — closes zero units by design, decisions.md §2)` — `cycle_scope_gate.py` present; exemption by design.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since d1b5738658 --before /tmp/wi-before-AT-35-E1-004.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E1-004` at `2bf452b038`).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `2bf452b038` — not risen.
- **Refused tokens:** none (no units scoped).
- **What moved:** `denominator_gate.py`'s `DEFAULT_GLOBS` and `PROVENANCE_DEFAULT_GLOBS` each gained SD-35's root `*.md` + `artifacts/**/*.md`; SD-33/SD-34 entries untouched (frozen pre-widening lists in the test). `verify.sh --only denominator-gate` → `files_checked=213 violations=0` (186 before + 27 of 27 SD-35 `.md`); `--only figure-provenance` → `files_checked=143 figures_examined=139 violations=0`. Tests 46 → 54 (RED 6 errors first). No stage added (45 / 39).
- **Discoveries:** the criterion's "never advanced to SD-34" was stale (AT-34-E1-006 had); SD-33 cannot enter the provenance default (44 violations of 137 figures in 78 files, out of write scope) — both emitted as `correction` events in `docs/retro/events/at-35-e1-004.jsonl`; the token-mapping denominator red E1-003/E1-005 saw was already cleared by E1-002 (`815139fadd`).
- **Receipt:** `artifacts/epic-1-tax-cut/AT-35-E1-004_cycle1_receipt.md`. Code `2bf452b038`. **Epic 1 is 6 of 6 complete — wrap-up (§10) next.**

### 2026-09-08 — AT-35-E1-003 cycle 1 — `test-families-table-driven` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (build-time tax cut — closes zero corpus units by design, decisions.md §2)` — `cycle_scope_gate.py` was absent at cycle start (`53296d80f0`) and arrived via AT-35-E1-001 on the pre-push rebase.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=1906 ratio=n/a builds_recorded=0 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 53296d80f0 --before /tmp/wi-before-AT-35-E1-003.json --after docs/work-inventory.json` at `03072aea0c`; `builds_recorded=0` reads the deleted target dirs — the transcripts show 4 cold measurement compiles + 1 verify session, receipt row for the breakdown).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` at `03072aea0c` — not risen.
- **Refused tokens:** none (no converter run).
- **What moved:** 184 integration-test binaries → 2 (`tests/sd18_widening/`, `tests/sd13_progression/`, one module per original file behind a class-keyed roster table); `cargo test -- --list` name-by-name diff 8,723 = 8,723 `IDENTICAL`; `BASELINE_ROOT_TEST_BINARIES` 589 → 408, test floors unchanged (8,656 / 3,186). Cold `cargo test --locked --no-run -j 6`, paired quiet-box: 3:08.97 → 2:25.89 (−43.08 s of 188.97 s); full suite after: 8,656 passed / 0 failed / 67 ignored across 408 targets in 37.6 min; clippy on both targets 0 warnings.
- **Discoveries:** the package's `denominator_gate.py` artifacts glob was already red at cycle start (11 violations, all in `epic-2-sheet-rule/token-mapping/`, for AT-35-E1-004); the criterion's "~80k of 187k" is 74,932 of 180,260; 244 `src/`+`docs/architecture/` citations of the old test paths deferred to AT-35-E7-003 (retro events in `docs/retro/events/at-35-e1-003.jsonl`).
- **Receipt:** `artifacts/epic-1-tax-cut/AT-35-E1-003_cycle1_receipt.md`; `build-time.json`, `test-list-diff.txt` beside it. Code `03072aea0c`.

### 2026-09-07 — AT-35-E1-002 cycle 1 — `content-anchored-citations` — **complete**

- **Scope gate:** `SCOPE_GATE: EXEMPT (instrument-hardening cycle — closes zero units by design, decisions.md §2)` — `cycle_scope_gate.py` was absent at cycle start (`53296d80f0`) and arrived via AT-35-E1-001 on the pre-push rebase.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=260` (`cycle_scope_gate.py --receipt --since 53296d80f0 --before /tmp/wi-before-AT-35-E1-002.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E1-002`, at the rebased HEAD with AT-35-E1-005's gate present; coarse-grep stand-in read 78 at start and end).
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` (`python3 scripts/pcgen_residue_gate.py --check` at the rebased HEAD) — equal to E1-005's first recording, not risen.
- **Refused tokens:** none
- **Commits:** `815139fadd` (code, artifacts, retro events; rebased onto AT-35-E1-001/E1-006/E1-005), plus this entry's commit (receipt, progress, kanban). Receipt: `artifacts/epic-1-tax-cut/AT-35-E1-002_cycle1_receipt.md`.
- **What landed:** 16 `file:line` pins → 16 content anchors across the three citation instruments, one shared resolver (`completion_atlas.resolve_content_anchor`); `--by-kind` / `--by-evidence`; `verify.sh` +3 stages (`shape-engine-boundary-selftest`, `shape-engine-boundary`, `missing-engine-tables`) — 42 → **45** after the rebases onto E1-001's `cycle-scope-gate-selftest` and E1-005's `pcgen-residue-gate` (`scripts/verify.sh --list | tail -n +2 | wc -l`); RED→GREEN transcript on the live engine source in `artifacts/epic-1-tax-cut/citation-anchor-proofs.md` (move 50 lines: all resolved lines +50, all green; change one condition each: all three fail closed and the two stages FAIL).
- **Discoveries (2 `correction` events):** `site-dashboard-check` already wrapped in `timeout` since AT-34-E6-001 wave 27 — nothing to add, D1.2 row updated; §6 step 3's denominator gate was red at cycle start on 11 pre-launch token-mapping lines (`artifacts/**` glob; the launch audit scanned the package root only — the same finding E1-001, E1-005 and E1-006 recorded and left) — **fixed here**, no figure changed, `denominator-gate` stage now `files_checked=186 violations=0`.

### 2026-09-07 — AT-35-E1-005 cycle 1 — the PCGen residue gate exists, baseline recorded — `complete`

- **Scope gate:** `SCOPE_GATE: EXEMPT (gate-building cycle — this cycle CREATES pcgen_residue_gate.py; closes zero units by design, decisions.md §2)`. `scripts/cycle_scope_gate.py` was absent in the cycle's tree at `53296d80f0` (AT-35-E1-001 landed on `origin/tranche/15` while this cycle ran; the receipt rows below were re-derived with it after the rebase).
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=260` — see the receipt for the mechanical `cycle_scope_gate.py --receipt` line run after the rebase, and the hand commands it agrees with.
- **PCGen residue (first recording, `53296d80f0`):** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` (`python3 scripts/pcgen_residue_gate.py --check`). `identifier_files=68 identifier_hits=514` of those 260 files are the five-identifier readers the authoring-time "78 files" figure was counting (that grep scanned `apps/desktop/src-tauri/src` only and included `cache_gen/`); the rest is token-syntax literals, 8,078 `PRE[A-Z]+:` + 2,403 `BONUS:` hits of 12,736, almost all in generated `src/rules_core/rules_tables/**`. Correction event `1788831658230-at-35-e1-005-0d124e`.
- **RED→GREEN on the real tree:** planted `src/rules_core/zz_planted_residue_probe.rs` (`r.raw_tokens.len()`) → `live_files=261 live_hits=12737 baseline_files=260 baseline_hits=12736 verdict=FAIL_INCREASED` exit 1; removed → `verdict=PASS` exit 0. `--check --closure` at the baseline → `live_files=260 live_hits=12736 verdict=FAIL` exit 1. `--rebaseline` at the baseline → `rebaseline=REFUSED ... verdict=FAIL_NOT_REDUCED` exit 1.
- **verify.sh:** stage `pcgen-residue-gate` in both sets after `figure-provenance` — `scripts/verify.sh --list` → 42 stages (36 quick) with AT-35-E1-001's `cycle-scope-gate-selftest` landed first; `--only pcgen-residue-gate` → `PASS`; `PCGEN_RESIDUE_GATE_CLOSURE=1 ... --only pcgen-residue-gate` → `FAIL` (closure mode, for AT-35-E6-004). `scripts/tests/test_pcgen_residue_gate.py` → `Ran 15 tests OK` (RED first: `ModuleNotFoundError`).
- **Build:** `cargo test --locked --no-run -j 6` exit 0; `--lib` 3186 passed / 0 failed (= floor); `--no-fail-fast`, desktop, frontend, clippy not run — no Rust touched (`git diff --stat 53296d80f0 -- '*.rs'` empty). Fast gates green: atlas, shape-engine-boundary, missing-engine-tables, pi-sweep; `data/sheet_rules/` grep → 0 (directory not yet created). **`denominator_gate.py --check` over `*.md` + `artifacts/**/*.md` is RED on inherited prose:** `files_checked=22 violations=11`, all 11 in `artifacts/epic-2-sheet-rule/token-mapping/` (committed `a232e27b03`, pre-cycle, outside Epic 1's touch set); this cycle's own three files → `violations=0`. Incident event emitted (the same finding AT-35-E1-001 and AT-35-E1-006 recorded); owner: the token-mapping synthesis / AT-35-E1-004.
- **Refused tokens:** none. Receipt: `artifacts/epic-1-tax-cut/AT-35-E1-005_cycle1_receipt.md` (names the code and docs SHAs); transcript: `artifacts/epic-1-tax-cut/pcgen-residue-first-run.txt`.

### Cycle — AT-35-E1-006 cycle 1 — SD-34's unrun closure folded: retrospective written and cited, 17 open rows and 29 open deferrals dispositioned — complete (2026-09-08)

**Status: complete.** Docs only; zero units moved by design. Work commit `9cc73dca76`, receipt
commit `800c363e42`; receipt `artifacts/epic-1-tax-cut/AT-35-E1-006_cycle1_receipt.md`.

- **Scope gate:** `SCOPE_GATE: EXEMPT (docs-only fold of SD-34's closure epilogue — closes zero units by design, decisions.md §2 and §12)` — `cycle_scope_gate.py` was absent at cycle start (`53296d80f0`) and arrived via AT-35-E1-001 on the pre-push rebase.
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=unavailable` (`cycle_scope_gate.py --receipt --since 53296d80f0 --target-dir /tmp/cargo-sd35-AT-35-E1-006`; `pcgen_residue_gate.py` absent — AT-35-E1-005 builds it; coarse-grep stand-in 78 files, `content-unit-inventory.md §6`).
- **Refused tokens:** none.
- **Evidence:** `test -f docs/retro/sd34-book-completion-retrospective.md` → present; `grep -c sd34-book-completion-retrospective` → 1 in each `references/README.md`; row map 1,590 of 1,590 (core_rulebook 1,529 of 6,701 + ultimate_campaign 61 of 265, `completion_atlas.py --book <book> --check`) sum-checked; SD-34 `progress.md` `status: closed-by-fold`; 29 of 29 deferrals dispositioned (8 resolved with a SHA, 1 superseded by register C2.5, 20 mapped) — `retro.py summary --since 2026-08-27` now reads `deferrals.open=20`, all SD-35-owned.
- **Gates:** `completion_atlas.py --check` 0; `shape_engine_boundary.py --check` 0; `missing_engine_tables.py --check` 0; `verify.sh --only pi-sweep` PASS; **`denominator_gate.py --check` red — 11 violations in 4 pre-existing `artifacts/epic-2-sheet-rule/token-mapping/*.md` files this cycle did not touch** (incident `1788831974625-at-35-e1-006-ae4135`; owner: the token-mapping synthesis / AT-35-E2-001).
- **Carried one-liner** (`SD-34 forward-scope-register.md` C1.8, `358a71516f`): `monk_ki_pool` "size"-suffix — unit is bucket C at the cut, owned by AT-35-E3-003; not applied here (docs only).
- **Operator attention:** the two fable-review P1s (R11-01, R14-02) are unfixed and outside SD-35 scope per `forward-scope-register.md` C2.5 — dispositioned `superseded` (register), not resolved.

### 2026-09-07 — AT-35-E1-001 cycle 1 — `batch-floor-gate` — complete

- **Scope gate:** `SCOPE_GATE: EXEMPT (gate-building cycle — this cycle CREATES cycle_scope_gate.py; it closes zero units by design, decisions.md §2)`
- **Receipt rows:** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=unavailable` (`pcgen_residue_gate.py` not yet in the tree — AT-35-E1-005)
- **Refused tokens:** none
- **Landed:** `scripts/cycle_scope_gate.py` (floor + `--receipt`), `scripts/tests/test_cycle_scope_gate.py` (51 cases), `verify.sh` stage `cycle-scope-gate-selftest` (stage count 40 → 41; `scripts/verify.sh --list`). Live at HEAD: `--bucket B --kind class_feature` → `scoped=7866 remaining_non_done=23315 verdict=PASS`; `--bucket A --kind companion` → `scoped=28 ... FAIL_UNDER_FLOOR` exit 1 (`python3 scripts/cycle_scope_gate.py --min 500 ...`).
- **Found, not fixed (outside file-touch set):** `denominator_gate.py --check` on the package is `violations=11` of `files_checked=21`, all in 4 pre-launch `artifacts/epic-2-sheet-rule/token-mapping/*.md` files — retro incident `denominator-gate-red-on-package-prose`; owner AT-35-E1-004.
- **Receipt:** `artifacts/epic-1-tax-cut/AT-35-E1-001_cycle1_receipt.md` — code at `1d821cdc8d`.
