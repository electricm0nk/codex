# SD-25 — Epic Breakdown (8 epics / ~24 acceptance criteria)

> **Operating method:** see `./scope-draft.md` — `scripts/workflow-dispatch.sh` (Workflow orchestrator). Bundle fires on `tranche/5-3`, kanban board `codex-tranche-5`. Cycle dispatch model is deterministic-seeded-then-dynamic (per SD-24 doctrine inherited through the template).

## Execution lane split

E1 Identifier Cleanup is the governance base; it fires FIRST. E2 Operator Pre-Launch is the gating epic — it cannot dispatch until the Tier-1 launch-gate (SD-24 closure PR merged to develop) is satisfied. E3-E5 are structural work with parallel-eligible criteria (per `decisions.md §3`). E6 + E7 are dynamic-dominant (one cycle per defect / per-class-feature). E8 Closure Epilogue fires LAST.

## Epic 1 — Code-Side Identifier Cleanup (governance base; fires FIRST)

### Criterion 1.1 — Source-code identifier audit

- **Cycle artifact:** `./artifacts/epic_1/identifier-audit-cycle_receipt.md`
- **Cycle doc:** `./cycles/1_1.md`
- **RED:** `git grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' apps/desktop/ apps/desktop/src-tauri/ src/ scripts/` returns ≥1 hit. (Trailing `\b` deliberately dropped and pattern generalized to `sd[0-9]+` — the literal `sd(16|19|22|23|24)_...\b` form SD-24 shipped was live-proven broken: `\b` never matches between `_` and a following word char, so it silently missed real hits. See `sd24-carry-forward-register.md` item A7.)
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
- **SD-24 carry-forward (`sd24-carry-forward-register.md` A2):** the `level_up` method must not silently inherit `level_up::compute_level_up_grants`'s top-level-dispatcher gap — it currently returns an honestly-empty `LevelUpPlan::default()` for any multiclass mix (including Fighter+Wizard) because it never routes to the per-class functions for a mix. Widen the signature to accept explicit per-class from/to sub-levels, or add a dedicated multiclass entry point, as part of this extraction.
- **SD-24 carry-forward (register A5):** `mutate_saved_character_at_root` and everything routed through it preserve whatever `revision_id` was already on disk instead of advancing it — only the new `reSaveCharacter` command advances the counter. Confirm with the operator whether to fold revision-advancing into `mutate_saved_character_at_root` itself as part of this extraction (a behavior change, not a pure move) or leave it for a later cycle.

### Criterion 3.3 — StubAdapter future-system stub

- **File:** new `apps/desktop/src-tauri/src/stub_adapter.rs`.
- **Behavior:** returns "Would render for system X; not yet implemented" results. Wired-integration doctrine forbids "Would …" strings in *shipping code* — this stub gets an entry in `governance/wired-integration-stubs-registry.md` with the operator-granted justification (the future-system rollout is operator-pinned).
- **Concurrency:** `parallel: yes`.

### Criterion 3.4 — Tauri command-surface routes through the hub-of-hubs

- **Files:** `apps/desktop/src-tauri/src/append_to_character.rs`, `recompute_character.rs`, `re_save_character.rs` (all accept `rule_system_id: String` argument; dispatch through trait).
- **Concurrency:** `parallel: no` (multi-file, depends on 3.1–3.3).
- **SD-24 carry-forward (register A3):** these three commands (SD-24 Epic 7) are registered and tested but have **zero frontend callers** today — no `boundary/*.ts` wrapper, no `invoke()` call site anywhere in `apps/desktop/src`. Routing them through the trait is necessary but not sufficient — criterion 3.5 must give at least one a real UI call site.

### Criterion 3.5 — UI panel adapter-aware

- **Files:** `apps/desktop/src/characterHub/CharacterHubPage.tsx`, `apps/desktop/src/characterHub/LoadCharacterScreen.tsx`, `apps/desktop/src/characterHub/characterHubRuntime.ts` (read active rule-system adapter; route interactions through it).
- **Concurrency:** `parallel: yes` (each file disjoint).
- **SD-24 carry-forward (register A3):** close the zero-frontend-caller gap flagged in 3.4 — wire at least one real UI affordance to `append_to_character`/`recompute_character`/`re_save_character`, matching SD-24 criterion 7.4's own Add-Weapon/Add-Armor/Add-Spell precedent.
- **SD-24 carry-forward (register A4):** `CharacterSheet.tsx`'s top-menu "Open"/"Save"/"Clone" items are genuine no-op handlers (`onSelect: () => {}}`) — real accidental stub debt that the dual-audit gate's forbidden-token grep doesn't catch (bare `() => {}` isn't a matched pattern). Wire them to real behavior or remove them if genuinely out of scope for this release, while this file is already open for adapter-aware routing.

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

**SD-24 carry-forward (`sd24-carry-forward-register.md` A6):** explicitly include the 9 CRB classes (Cleric, Rogue, Sorcerer, Barbarian, Bard, Druid, Monk, Paladin, Ranger) SD-24 never audited for the `class_spell.*`-vs-`class_feature.*`-prefix bug found and fixed for Wizard — each already has a landed `level_up/<class>.rs` module; the same bug class (a later `pilot_compute.rs` grounding landing after the module and never added to its explanation-id filter) could recur in any of them. LOW priority, one file at a time per the file-touch partition.

### Criterion 7.2..7.M — Per-feature cycles (dynamic)

One class-feature → one cycle → one receipt at `./artifacts/epic_7/<feature-id>_cycle_receipt.md`.

### Criterion 7.O — GE-07 pilot-shell-snapshot real implementation (SD-24 carry-forward, added 2026-07-21)

**Origin (`sd24-carry-forward-register.md` A1):** `load_pilot_shell_snapshot` Tauri command + `apps/desktop/src/boundary/loadPilotShellSnapshot.ts` unconditionally return hardcoded GE-07-era fixture data (`case_id: "ge07-e1-scaffold-placeholder"`) regardless of real character state — fixture-only data in a production command path, a genuine no-stub-doctrine violation. Legacy scaffolding predating the bundle-tag convention, consumed only by the SD-11 internal tester workbench, which already labels it honestly. SD-24 deferred this to `risks-and-open-questions.md §5` rather than remediating in-cycle, since real remediation needs a design decision this criterion resolves first.

**This criterion is blocked on an operator design decision** (see `risks-and-open-questions.md §4`, new open question): what a headless-core-backed pilot shell snapshot actually computes, and from what input contract. Do not dispatch the implementation cycle until that's answered — dispatch only the design-decision request itself first.

### Criterion 7.N — Equipment/spell corpus intake (SD-24 carry-forward, added 2026-07-21)

Reads SD-24's `progress.md ## Open blockers` and `## TODO` remainder directly (not just the per-class-coverage-matrix). SD-24 closed with three real, corpus-data-limited gaps inside its own declared scope (PF1 core rules + APG + ACG + Bestiary 1) — these are backlog intake, not new book-scope (the "equipment corpus extension beyond PF1 core rules + APG + ACG + Bestiary 1" deferral in `risks-and-open-questions.md §5` is unaffected and stays deferred):

1. **6.4 description field, CRB** — 1156/2977 equipment rows (mostly equipmods + slot-type markers) have no `DESC:` token in the ingested PCGen LST corpus. Ceiling: 61.2% (1821/2977).
2. **6.4 description field, APG** — 0/338 equipment rows have a `DESC:` token at all; the LST corpus carries zero prose for APG equipment.
3. **6.5 full spell text, APG** — 36/297 spells have no matching `.MOD` full-text record or no `SCHOOL:`/`CLASSES:` token on the base record.
4. **Bestiary 1 equipment + spells** — never ingested in SD-24 (no `beastiary1/equipment_tables.rs` module exists at all; ~7 real equipment records unclaimed per SD-24's `equipment-coverage-matrix.md`). This one is a plain scope gap (SD-24's own orchestrator omitted Bestiary 1 from its equipment fan-out), not a corpus-data ceiling — do not expect the same "unreachable" shape as 1-3 above; it should mechanically reach whatever ceiling the LST corpus supports, the same way CRB/APG/ACG did.

**Recommended resolution (operator directive 2026-07-21):** where the ingested PCGen LST corpus (`~/workspace/repos/pcgen/data/...`) genuinely lacks the data — no `DESC:` token, no `.MOD` full-text record — run a second-source web content pass against **d20pfsrd.com** and/or **aonprd.com** (Archives of Nethys) instead of accepting the ceiling or fabricating text. Both are standard PF1 SRD/OGL reference sources.

- Match by identity first: cross-check name + level/school (spells) or name + category/cost (equipment) against the LST record before writing anything — a same-named item across books, or a 3.5e/PF2e cousin, is a false match.
- Cite the source URL in the cycle receipt for every web-sourced field, the same way corpus-sourced fields cite their LST token.
- Respect each site's terms of use (reasonable request pacing; no bulk-scraping the whole site — fetch only the specific pages needed for the identified gap records).
- If a record can't be confidently identity-matched on either site, leave the field `None` and keep the corpus ceiling rather than guess — this is the same no-fabrication rule SD-24's cycles already applied to the LST-only approach.
- This does not relax criterion 6.4/6.5's wording — it's an additional legitimate source, alongside the three operator options SD-24's Open Blockers already named (accept ceiling / license a second source / reword the criterion). A successful web-sourced pass can retroactively close SD-24's two Open Blockers.

**Additional carry-forward items folded in (register A8, A10, A11):**
- **A8 — shared codegen path.** SD-24's Epic 6 cycles each wrote their own one-off ad-hoc ingestion script instead of reusing the existing, tested `src/pcgen_import/lst_parser/equipment.rs`/`spell.rs` tokenizer (SD-17 Slice B-5). Before writing a 5th ad-hoc script for these cycles, consider a shared `pcgen_import`-backed codegen path (parse → semantic-map → emit Rust literals). Not a hard requirement — flagged so the choice is deliberate, not accidental.
- **A10 — Bestiary-1 description sourcing.** Check for a `DESC:`- or `SPROP:`-equivalent convention in Bestiary-1's own 7-record equipment corpus first (ACG reached a 98.1% ceiling via `SPROP:` with zero `DESC:` tokens) before falling back to the d20pfsrd/aonprd web pass.
- **A11 — `SOURCELONG:`-header miscount.** CRB, APG, and ACG each independently hit the same measurement error (a `SOURCELONG:` header line double-counted as a record). Apply the exclusion from the start for Bestiary-1's counts rather than as a later correction.

**Concurrency:** CRB-description, APG-description, APG-spell-text, and Bestiary-1 ingestion are 4 disjoint file-touch cycles — `parallel: yes`, same `isolation: 'worktree'` pattern as SD-24's own Epic 6.

**Full carry-forward register:** `./sd24-carry-forward-register.md` — all 41 of SD-24's `## DISCOVERED` entries plus its 4 `## TODO` remainders, each with a disposition (real follow-on / documentation-only / already-fixed / process lesson) and, where applicable, an SD-25 epic/criterion assignment. This section and Epic 3's 3.2/3.4/3.5 carry only the items with real dispatchable work; the register has full custody of everything else, including 14 documentation-staleness corrections (batchable in one cycle) and 3 process/tooling lessons for `scripts/workflow-dispatch.sh`'s own authoring.

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

## Quick reference — 8 epics / ~25 criteria

| Epic | Declarative criteria | Dynamic criteria | Concurrency |
|---|---|---|---|
| E1 Identifier Cleanup | 1 | 0 | serial |
| E2 Operator Pre-Launch | 5 | 0 | serial |
| E3 Hub-of-Hubs | 5 | 0 | 4 parallel + 1 serial |
| E4 PCGen Runner | 4 | 0 | 3 parallel + 1 serial |
| E5 Corpus Ingest Diagnostic | 1 | 0 | serial |
| E6 UI-Eval Defects | 1 cycle-shape + dynamic | ~5–10 defects | serial |
| E7 Per-class residue + equipment/spell corpus intake + GE-07 snapshot | 3 intake + dynamic | ~3–5 per-class features | 4 of the corpus-intake cycles parallel; rest serial |
| E8 Closure Epilogue | 5 | 0 | serial; sub-step tiering (Haiku/Sonnet/Opus) |
| **Total** | **25** | **~8–15 dynamic** | per-`parallel` row gets `isolation: worktree` |

Dynamic criteria grow as the operator's UI-eval session + per-class residue intake produce findings. The orchestrator script handles dynamic entries via `## DISCOVERED` priority-bump mechanism; the closure gate covers both declarative and dynamic criteria. Full carry-forward custody from SD-24's 41 `## DISCOVERED` entries + 4 `## TODO` remainders: `./sd24-carry-forward-register.md`.
