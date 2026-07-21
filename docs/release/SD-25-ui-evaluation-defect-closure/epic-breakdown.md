# SD-25 — Epic Breakdown (8 epics / ~24 acceptance criteria)

> **Operating method:** see `./scope-draft.md` — `scripts/workflow-dispatch.sh` (Workflow orchestrator). Bundle fires on `tranche/5-3`, kanban board `codex-tranche-5`. Cycle dispatch model is deterministic-seeded-then-dynamic (per SD-24 doctrine inherited through the template).

## Execution lane split

E1 Identifier Cleanup is the governance base; it fires FIRST. E2 Operator Pre-Launch is the gating epic — it cannot dispatch until the Tier-1 launch-gate (SD-24 closure PR merged to develop) is satisfied. E3-E5 are structural work with parallel-eligible criteria (per `decisions.md §3`). E6 + E7 are dynamic-dominant (one cycle per defect / per-class-feature). E8 Closure Epilogue fires LAST.

## Epic 1 — Code-Side Identifier Cleanup (governance base; fires FIRST)

### Criterion 1.1 — Source-code identifier audit

- **Cycle artifact:** `./artifacts/epic_1/identifier-audit-cycle_receipt.md`
- **Cycle doc:** `./cycles/1_1.md`
- **RED:** `git grep -nE '\b(sd(16|19|22|23|24)_|SD(16|19|22|23|24)_|Sd(16|19|22|23|24)|t_[0-9a-f]{8,})\b' apps/desktop/ apps/desktop/src-tauri/ src/ scripts/` returns ≥1 hit.
- **GREEN:** the renames land; the same `git grep` returns 0 hits.
- **Concurrency:** single cycle; serial.

## Epic 2 — Operator Pre-Launch

### Criterion 2.1 — `codex-tranche-5` reachable (per template §1 item 1; `hermes kanban boards` not `list-boards`)
### Criterion 2.2 — `tranche/5-3` pushed to origin
### Criterion 2.3 — SD-24 closure PR merged to develop (Tier-1 launch gate)
### Criterion 2.4 — Working tree clean on `tranche/5-3`
### Criterion 2.5 — Doctrines and skills loaded (per template §1 item 6: skills are doctrine docs not hermes-skill-loaded; verified inline by grep in `loop-instruction.md §6`)

## Epic 3 — Character Hub as Hub of Hubs (Rule-System Adapter)

### Criterion 3.1 — `RuleSystemAdapter` trait definition

- **File:** `apps/desktop/src-tauri/src/rule_system_adapter.rs` (new)
- **Cycle doc:** `./cycles/3_1.md`
- **Concurrency:** `parallel: yes`
- **Methods (the trait surface):** `chassis_resolve`, `level_up`, `save_character`, `append_to_character`, `recompute`, `list_saved_characters`, `load_saved_character`.

### Criterion 3.2 — Pf1Adapter extraction from `character_hub.rs`

- **File:** new `apps/desktop/src-tauri/src/pf1_adapter.rs`. Existing logic moves here.
- **RED:** existing tests pass before the move.
- **GREEN:** after the move, existing tests still pass; the trait's Pf1 implementation lives at `pf1_adapter.rs`.
- **Concurrency:** `parallel: yes`.

### Criterion 3.3 — StubAdapter future-system stub

- **File:** new `apps/desktop/src-tauri/src/stub_adapter.rs`.
- **Behavior:** returns "Would render for system X; not yet implemented" results. Wired-integration doctrine forbids "Would …" strings in *shipping code* — this stub gets an entry in `governance/wired-integration-stubs-registry.md` with the operator-granted justification (the future-system rollout is operator-pinned).
- **Concurrency:** `parallel: yes`.

### Criterion 3.4 — Tauri command-surface routes through the hub-of-hubs

- **Files:** `apps/desktop/src-tauri/src/append_to_character.rs`, `recompute_character.rs`, `re_save_character.rs` (all accept `rule_system_id: String` argument; dispatch through trait).
- **Concurrency:** `parallel: no` (multi-file, depends on 3.1–3.3).

### Criterion 3.5 — UI panel adapter-aware

- **Files:** `apps/desktop/src/characterHub/CharacterHubPage.tsx`, `apps/desktop/src/characterHub/LoadCharacterScreen.tsx`, `apps/desktop/src/characterHub/characterHubRuntime.ts` (read active rule-system adapter; route interactions through it).
- **Concurrency:** `parallel: yes` (each file disjoint).

## Epic 4 — PCGen Runner Scaffolding

### Criterion 4.1 — `scripts/pcgen-run-character.sh` (Bash + Gradle + jq)

- **Cycle doc:** `./cycles/4_1.md`
- **Concurrency:** `parallel: yes`

### Criterion 4.2 — `scripts/pcgen-normalize-output.py` (Python)

- **Concurrency:** `parallel: yes`

### Criterion 4.3 — `tests/oracle_validation/pcgen_runner_smoke.rs` (Rust smoke test)

- **Concurrency:** `parallel: yes`

### Criterion 4.4 — Verification cycle: run all three against the pilot case

- **Inputs:** `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` + `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`
- **Concurrency:** `parallel: no` (multi-artifact verification).

## Epic 5 — Corpus Ingest Diagnostic Sketch (one cycle)

### Criterion 5.1 — `corpus_ingest_diagnostic` Tauri command + UI panel route

- **File:** new `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`; new `apps/desktop/src/characterHub/CorpusIngestDiagnosticPanel.tsx`.
- **Returns:** `Vec<BookIngestStatus>` with book_id, status, last_ingested_at, content_kind_counts.
- **Sketch only:** SD-26 fans out the full status table + flags + ETA once the JSON cache lands.

## Epic 6 — UI-Evaluation Discovered Backend Defects (dynamic-dominant)

### Criterion 6.1 — Discovered-defect cycle shape

The cycle shape that processes `## DISCOVERED` entries. Per-defect: one defect → one cycle → one cycle receipt.

### Criterion 6.2..6.N — Per-defect cycles (dynamic)

Criteria spawn dynamically as the operator's UI-eval session surfaces defects. Each criterion: one defect → one cycle → one cycle receipt at `./artifacts/epic_6/<defect-id>_cycle_receipt.md`. The closure gate is "every UI-discovered defect has either a `complete` cycle receipt or an entry in `## Open blockers`."

## Epic 7 — Deferred Per-Class Work & SD-22/SD-24 Coverage Backlog

### Criterion 7.1 — Per-class residue intake

Reads SD-24 Epic 4's `per-class-coverage-matrix.md`. Emits per-feature `## DISCOVERED` entries.

### Criterion 7.2..7.M — Per-feature cycles (dynamic)

One class-feature → one cycle → one receipt at `./artifacts/epic_7/<feature-id>_cycle_receipt.md`.

## Epic 8 — Closure Epilogue (fires LAST; subagent tiering per-criterion)

### Criterion 8.1 — Final criterion scan (criteria 1–N)

- Subagent: Sonnet.
- Behavior: scans every prior criterion for `complete` or `## Open blockers`.

### Criterion 8.2 — Architecture closure pipeline (truth-up + graphify + PR + merge)

- Subagent: Opus (template §2's adversarial verification / judge-panel).
- Steps: `architecture-truth-up` script, `graphify-update` script, PR + merge-conflict-resolution.

### Criterion 8.3 — Release notes generated at `./release-notes.md`

- Subagent: Haiku (housekeeping).
- Sections per template: Summary, User-Visible Changes, Defects Fixed, Operational Notes, Verification Evidence, Known Issues, Update Eligibility.

### Criterion 8.4 — Build version increment

- Subagent: Haiku.
- Files: `apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`. First concrete value: `0.5.98`.

### Criterion 8.5 — `tranche/5-3 → develop` PR + merge

- Subagent: Sonnet.
- Behavior: PR via `gh pr create`; merge-conflict-resolution per template §6 step 4.

---

## Quick reference — 8 epics / ~24 criteria

| Epic | Declarative criteria | Dynamic criteria | Concurrency |
|---|---|---|---|
| E1 Identifier Cleanup | 1 | 0 | serial |
| E2 Operator Pre-Launch | 5 | 0 | serial |
| E3 Hub-of-Hubs | 5 | 0 | 4 parallel + 1 serial |
| E4 PCGen Runner | 4 | 0 | 3 parallel + 1 serial |
| E5 Corpus Ingest Diagnostic | 1 | 0 | serial |
| E6 UI-Eval Defects | 1 cycle-shape + dynamic | ~5–10 defects | serial |
| E7 Per-class residue | 1 intake + dynamic | ~3–5 per-class features | serial |
| E8 Closure Epilogue | 5 | 0 | serial; sub-step tiering (Haiku/Sonnet/Opus) |
| **Total** | **23** | **~8–15 dynamic** | per-`parallel` row gets `isolation: worktree` |

Dynamic criteria grow as the operator's UI-eval session + per-class residue intake produce findings. The orchestrator script handles dynamic entries via `## DISCOVERED` priority-bump mechanism; the closure gate covers both declarative and dynamic criteria.
