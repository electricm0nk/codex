# SD-25 — Technical Design

> **Operating method:** see `./scope-draft.md`. Architectural surface for SD-25's four-load bundle.

## 1. Architectural posture

SD-25 carries four loads that share a single architectural posture: **structural refactor + scaffolding + dynamic discovery** for character_hub (a single-source rule system) to become character-hub-of-hubs (a hub of N rule systems), backed by an external oracle (PCGen).

## 2. Character-Hub-as-Hub-of-Hubs (Epic 3)

### 2.1 Trait surface

The `RuleSystemAdapter` trait at `apps/desktop/src-tauri/src/rule_system_adapter.rs`:

```rust
pub trait RuleSystemAdapter {
    fn adapter_id(&self) -> &str;  // e.g., "pf1", "dnd5e", "pf2", "stub"
    fn chassis_resolve(&self, chassis_query: ChassisQuery) -> Result<ChassisRow, AdapterError>;
    fn level_up(&self, character_id: CharacterId) -> Result<CharacterSnapshot, AdapterError>;
    fn save_character(&self, character: CharacterSnapshot) -> Result<SavedRef, AdapterError>;
    fn append_to_character(&self, character_id: CharacterId, items: Vec<ItemRef>) -> Result<CharacterSnapshot, AdapterError>;
    fn recompute(&self, character_id: CharacterId) -> Result<CharacterSnapshot, AdapterError>;
    fn list_saved_characters(&self) -> Result<Vec<SavedRef>, AdapterError>;
    fn load_saved_character(&self, ref: SavedRef) -> Result<CharacterSnapshot, AdapterError>;
}
```

### 2.2 Pf1Adapter extraction (Criterion 3.2)

The existing logic from `apps/desktop/src-tauri/src/character_hub.rs` (single Rust backend file — verified) moves into `apps/desktop/src-tauri/src/pf1_adapter.rs`. The trait's `adapter_id()` returns `"pf1"`. Existing tests pass against the extracted module without test infrastructure refactor (per `decisions.md §4 Q3`).

### 2.3 StubAdapter future-system stub (Criterion 3.3)

Returns "would have rendered for system X; not yet implemented" results. **This stub gets an entry in `governance/wired-integration-stubs-registry.md`** per the wired-integration doctrine — operator-granted exception with the future-system rollout as the justification.

### 2.4 Tauri command routing (Criterion 3.4)

The Tauri commands `append_to_character`, `recompute_character`, `re_save_character` (one each under `apps/desktop/src-tauri/src/`) accept a `rule_system_id: String` argument and dispatch through the trait. The character_hub runtime no longer owns the PF1 implementation; it picks an adapter by id.

### 2.5 UI panel (Criterion 3.5)

`apps/desktop/src/characterHub/{CharacterHubPage.tsx,LoadCharacterScreen.tsx,characterHubRuntime.ts}` reads the active rule-system adapter and routes interactions through it. The UI's existing PF1-only shape persists; the adapter is the routing layer.

### 2.6 Concurrency shape

3.1, 3.2, 3.3, 3.5 = `parallel: yes` (different files; orchestrator invokes with `isolation: 'worktree'`). 3.4 = `parallel: no` (multi-file Tauri commands, depends on 3.1–3.3).

## 3. PCGen Runner (Epic 4)

### 3.1 Architecture

The runner is a three-script pipeline:

```
character_input_ref → pcgen-run-character.sh → raw XML output
                                       ↓
                      pcgen-normalize-output.py → normalized JSON
                                       ↓
              pcgen_runner_smoke.rs (Rust) → ground against golden fixture
```

### 3.2 `scripts/pcgen-run-character.sh`

Bash + Gradle + jq. Takes a `character_input_ref` (path to a deterministic input file like `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`), invokes `/home/ubuntu/workspace/repos/pcgen/gradlew` against `code/testsuite/base-xml.ftl` (the legacy-route field from `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`), and emits the raw XML output to `tests/fixtures/oracle_validation/pcgen_outputs/<case-id>.raw.xml`.

### 3.3 `scripts/pcgen-normalize-output.py`

Python. Reads the raw XML, walks the typed shape from `src/oracle_validation/selected_parity_dimensions.rs` (which already projects to 8 selected parity dimensions), and writes normalized JSON to `tests/fixtures/oracle_validation/pcgen_outputs/<case-id>.json` with a SHA-256 frontmatter mirroring the legacy PCGen output's SHA at `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt:legacy_raw_output_sha256`.

### 3.4 `tests/oracle_validation/pcgen_runner_smoke.rs`

Rust. Smoke test that calls 3.2 and 3.3 against the pilot case and verifies the normalized JSON's SHA matches what's expected.

### 3.5 Verification (Criterion 4.4)

Single verification cycle that runs all three against the pilot case. Confirms the orchestrator can produce one PCGen oracle output + the runner scaffolding works end-to-end.

### 3.6 What's deferred to SD-26

- **Library build** — SD-26 fans 3.1 + 3.2 across ~30 deterministic inputs to produce the durable library at `tests/fixtures/oracle_validation/pcgen_outputs/*.json`.
- **Comparator** — SD-26 builds the comparator + normalization engine that consumes the library and asserts Codex equivalence.
- **Parity-report writer** — SD-26.

## 4. Corpus Ingest Diagnostic Sketch (Epic 5)

### 4.1 Shape

A single Tauri command:

```rust
#[tauri::command]
async fn corpus_ingest_diagnostic() -> Result<Vec<BookIngestStatus>, AdapterError>;
```

Returns `Vec<BookIngestStatus>` with book_id, status enum (`stubbed` | `partial` | `complete` | `failed`), last_ingested_at, content_kind_counts.

### 4.2 Status table

For SD-25 (sketch-only), the table returns hardcoded counts based on `cargo test` results + `ls` of `src/rules_core/rules_tables/<book>/`. Future-state books return `stubbed` with `planned_resolution_bundle: "SD-26"` so the operator can see which books are operator-granted stubs.

### 4.3 UI panel

`apps/desktop/src/characterHub/CorpusIngestDiagnosticPanel.tsx` renders the response into a status table. Sketch only.

### 4.4 SD-26 hand-off

SD-26 reads the JSON cache + book-stub-manifest and replaces the hardcoded counts with live data. The Sketch ships the API + UI; SD-26 supplies the data.

## 5. UI-Evaluation Discovered Defects (Epic 6)

Per-defect cycle shape: TDD, dual-audit, receipt, push. Cycle picker reads `## DISCOVERED` for accumulated findings.

## 6. Per-class residue (Epic 7)

Per-feature cycle shape mirroring Epic 6, sourced from SD-22 + SD-24 coverage audits.

## 7. Closure (Epic 8)

Standard sub-pipeline: architecture-truth-up + graphify-update + PR + merge. Subagent tiering per the per-criterion override map at `decisions.md §3`.

## 8. Cross-reference

- `./scope-draft.md §1` — Epic decomposition
- `./content-unit-inventory.md` — per-content-unit routing
- `./loop-instruction.md §6` — per-cycle procedure (dual-audit gate)
- `./decisions.md §3` — per-epic concurrency + tiering
- `../SD-24-beta-readiness-and-multiclass/technical-design.md` — closed predecessor (Tauri command-surface pattern + character_hub loadout context)
- `src/oracle_validation/` — Oracle-harness schema surface (Epic 4 reads)
- `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` — pilot case for Epic 4 verification
- `/home/ubuntu/workspace/repos/pcgen/gradlew` + `code/testsuite/base-xml.ftl` — PCGen Gradle headless route
