# SD-24 — Epic Breakdown (8 epics / 35 acceptance criteria)

> **Operating method:** see `./scope-draft.md` — `/loop 1m /batch /goal ./loop-instruction.md`. Bundle fires on `tranche/5-2`, kanban board `codex-tranche-5`. Cycle dispatch model is deterministic-seeded-then-dynamic (per `./decisions.md §2`).

## Execution lane split

Epic 1 (Identifier Cleanup) is the governance base; it fires FIRST. Epic 2 (Operator Pre-Launch) is the gating epic — it cannot dispatch until the Tier-1 launch-gate (SD-23 closure PR merged to develop) is satisfied. Epics 3-7 fire after Epic 2; Epic 8 (Closure Epilogue) fires LAST.

Epic 4 (Per-class audit) must complete before Epic 5 (Multiclass) starts. Epic 5 is gated on Epic 4's coverage-matrix output. Epic 6 (Equipment content completion) is gated on Epic 4's findings (which classes are fully wired, which spill into Epic 6's cycle backlog). Epic 7 (Tauri command-surface + unwired user workflows) is gated on Epic 5's multiclass dispatch + Epic 6's equipment remediation (the cycle that wires a picker uses the equipment-corpus data).

Epic 8 fires after Epic 9's evaluation; this SD adopts SD-23's "Epic 9 = Closure Readiness" pattern but inlines it into Epic 8's criterion 29 evaluation step rather than as a separate epic. The closure-pipeline sub-steps (architecture-truth-up, graphify-update, merge-conflict-resolution) run during Epic 8's criterion 30 cycle.

## Epic 1 — Code-Side Identifier Cleanup (governance base; fires FIRST)

**Epic purpose:** under the identifier-discipline doctrine (`/home/ubuntu/workspace/repos/codex/governance/identifier-discipline.md` and skill `identifier-discipline`), audit and remove bundle-tag identifier leaks and apply PascalCase / camelCase discipline.

**Cycle eligibility:** Epic 1 cycles are the only ones eligible at loop launch. Subsequent cycles cannot dispatch until Epic 1's criteria are `complete`.

### Criterion 1.1 — Source-code identifier audit (in scope: `apps/desktop/`, `apps/desktop/src-tauri/`, `src/`)
- **Cycle artifact:** `./artifacts/epic_1/identifier-audit-cycle_receipt.md`
- **Files touched:** none (read-only audit cycle)
- **RED:** `git grep -nE '\b(sd(16|19|22|23|24)_|SD(16|19|22|23|24)_|Sd(16|19|22|23|24)|t_[0-9a-f]{8,})\b' apps/desktop/ apps/desktop/src-tauri/ src/` returns ≥1 hit.
- **GREEN:** the renames land; the same `git grep` returns 0 hits (or only test-fixture / audit-trail exemptions).

### Criterion 1.2 — Per-cycle tests pass after every rename
- **Cycle artifact:** per-rename cycle, named after the rename.
- **Files touched:** the renamed file + its call sites + its tests.
- **RED:** rename introduces a missing import or wrong-name reference.
- **GREEN:** `cargo test --locked --tests 2>&1 | tail -20` returns 0 failures; the rename is consistent across the surface.
- **Note:** defensive cleanup; scope scales with what's actually dirty. Post-SD-23 cleanup, scope is light.

## Epic 2 — Operator Pre-Launch (board-exists / branch-pushed / SD-23-merged / clean-tree)

**Epic purpose:** verify launch infrastructure is ready before any non-read-only cycle dispatches.

### Criterion 2.1 — `codex-tranche-5` kanban board reachable
- **Verification:** `hermes kanban list-boards` shows `codex-tranche-5`. If absent, exit FAIL; operator creates board.

### Criterion 2.2 — `tranche/5-2` branch pushed to origin
- **Verification:** `git ls-remote origin tranche/5-2` resolves to a SHA.

### Criterion 2.3 — SD-23 closure PR merged to develop
- **Verification:** `git log origin/develop --oneline | head -5` shows the SD-23 closure commit as HEAD of develop.
- **Tier-1 launch-gate:** this is the SD-24 launch-gate dependency. The loop refuses to dispatch cycle 3 of Epic 2 (the first non-gating-cycle) until this criterion is `complete`.

### Criterion 2.4 — Working tree clean on `tranche/5-2`
- **Verification:** `git status --porcelain` returns empty.

### Criterion 2.5 — Doctrines and skills loaded
- **Verification:** skill list includes `wired-integration-discipline`, `identifier-discipline`, `kanban-claude-code-execution-receipt`. Loop verifies at every cycle.

## Epic 3 — Wired-Integration Audit + Remediation (operator's "loose ends" sweep)

**Epic purpose:** per the wired-integration-discipline skill, run the four-check audit read-only across the codebase, generate a remediation backlog, and remediate each finding. All new Stubs Registry entries land at `/home/ubuntu/workspace/repos/codex/governance/wired-integration-stubs-registry.md` only when operator-granted.

### Criterion 3.1 — Wired-Integration Audit (read-only sweep)
- **Cycle artifact:** `./artifacts/epic_3/wired-integration-audit.md`
- **RED:** audit script finds ≥1 forbidden pattern (token / noop handler / mock leak / "Would …" string).
- **GREEN:** audit output captured; per-finding remediation backlog enumerated; cycle's `## DISCOVERED` entries tagged with `epic-3-originated`.

### Criterion 3.2 — Remediation cycle: forbidden tokens
- **Cycle artifact:** per-file remediation cycle.
- **Files touched:** one file per cycle under the file-touch partition.

### Criterion 3.3 — Remediation cycle: empty event handlers / noop handlers
- **Cycle artifact:** per-file remediation cycle.

### Criterion 3.4 — Remediation cycle: mock leaks + "Would …" strings
- **Cycle artifact:** per-file remediation cycle.

## Epic 4 — Per-Class Coverage Audit + Plan

**Epic purpose:** per operator skepticism on SD-22 throughput (§1 of decisions.md), audit per-class feature-table coverage for every class in CRB / APG / ACG. Output the coverage matrix; produce the remediation plan.

### Criterion 4.1 — Per-class audit: CRB classes (Fighter, Wizard, Cleric, Rogue, Sorcerer, Barbarian, Bard, Druid, Monk, Paladin, Ranger)
- **Cycle artifact:** `./artifacts/epic_4/per-class-coverage-matrix.md`
- **Files touched:** per-class `class_*.rs` files in `src/rules_core/rules_tables/crb/` for read-only audit.

### Criterion 4.2 — Per-class audit: APG classes (Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch)
- **Cycle artifact:** extension of `./artifacts/epic_4/per-class-coverage-matrix.md`
- **Files touched:** per-class `class_*.rs` files in `src/rules_core/rules_tables/apg/`.

### Criterion 4.3 — Per-class audit: ACG classes (Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest)
- **Cycle artifact:** extension of `./artifacts/epic_4/per-class-coverage-matrix.md`.
- **Files touched:** per-class `class_*.rs` files in `src/rules_core/rules_tables/acg/`. — (corrected 2026-07-22 per SD-25 criterion 7.P: roster was "Alchemist-side" (APG-only class, not ACG) → removed; "Slayer" was missing → added; real 10-class ACG roster: Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest per `rules_tables::acg::mod.rs`) |

### Criterion 4.4 — Remediation plan enumeration
- **Cycle artifact:** `./artifacts/epic_4/remediation-plan.md`
- For each class where `class_features_wired < class_features_expected`, list the missing features with the cycle-id that fixes them.

### Criterion 4.5 — APG/ACG-class multiclass scope decision
- **Decision:** per Epic 4's coverage report, APG/ACG-class multiclass is deferred to a follow-on bundle (per operator directive 2026-07-21 §1).

## Epic 5 — Multiclass Stacking Real and Full (Fighter + Wizard only)

**Epic purpose:** per operator directive 2026-07-21 (§4 of decisions.md), ship multiclass real-and-full scoped to Fighter + Wizard only. Level 1 → 10 advancement. APG/ACG-class multiclass is deferred (per §1 of decisions.md).

### Criterion 5.1 — Fighter + Wizard multiclass dispatch
- **Files touched:** `src/rules_core/pilot_compute.rs`, `src/rules_core/rules_tables/crb/class_fighter.rs`, `class_wizard.rs`.
- **RED:** advancement from level 1 → 10 fails at level 5 (the multiclass split-class transition level) for the Fighter-side or Wizard-side.
- **GREEN:** advancement from level 1 → 10 succeeds; BAB stacking follows canonical PF1 best-progression; saves use best-fractional-progression.

### Criterion 5.2 — Deterministic test surface: 30 character-advancement cycles
- **Cycle artifact:** `./artifacts/epic_5/multiclass-fixture.md` with per-cycle input/output.
- **Files touched:** test fixture file `tests/sd24_multiclass_deterministic.rs`.

### Criterion 5.3 — Integration test consumes ingested content
- **Cycle artifact:** `./artifacts/epic_5/integration-test-cycle_receipt.md`.

### Criterion 5.4 — Multiclass dispatch passes the four-check audit
- **Cycle artifact:** dual-audit gate output captured.

### Criterion 5.5 — APG/ACG-class multiclass deferral (deferred-work-item)
- **Cycle artifact:** `./artifacts/epic_5/apg-acg-multiclass-deferred.md`. Documents the deferral with reference to Epic 4's coverage report.

## Epic 6 — Equipment/Armor/Spells Content Completion (strict 100% field coverage)

**Epic purpose:** per operator directive 2026-07-21 (§5 of decisions.md), ship strict 100% field coverage for cost (where applicable), weight (where applicable), full description on every equipment item, armor piece, weapon, spell in the beta-tier corpus (full PF1 core rules + APG + ACG + Bestiary 1).

### Criterion 6.1 — Equipment coverage audit (read-only)
- **Cycle artifact:** `./artifacts/epic_6/equipment-coverage-matrix.md`
- **Files touched:** per-row enumeration only; no production change.

### Criterion 6.2 — Equipment content completion: cost field
- **Cycle artifact:** per-file remediation log entry in `./artifacts/epic_6/content-completion-log.md`.

### Criterion 6.3 — Equipment content completion: weight field
- **Cycle artifact:** per-file log entry.

### Criterion 6.4 — Equipment content completion: full description
- **Cycle artifact:** per-spell / per-item log entry.

### Criterion 6.5 — Spell content completion: full text per SRD/PRD
- **Cycle artifact:** per-spell log entry; full text transcriptions cross-checked against canonical sources.

## Epic 7 — Unwired User-Facing Workflows + Tauri Command-Surface Repair

**Epic purpose:** two intertwined work streams:
1. Tauri command-surface extension for iterative character mutation (load/append/recompute/re-save).
2. Unwired Add Weapon / Add Armor / Add Spell onClick handlers wired to real corpus data.

### Criterion 7.1 — `appendToCharacter` Tauri command
- **Files touched:** new `apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs`, plus the IPC adapter and call sites.
- **RED:** adding equipment to a saved character returns a `success: true` that lies (per the duracon 2026-07-18 18:20:41 sentinel).
- **GREEN:** appending equipment actually appends it; the saved character round-trips through load+append+recompute+re-save.

### Criterion 7.2 — `recomputeCharacter` Tauri command
- **Files touched:** new `apps/desktop/src-tauri/src/characterHub/recomputeCharacter.rs`.
- **RED:** recomputing after a level-up does not refresh derived stats.
- **GREEN:** recomputing refreshes BAB / saves / skill points / caster level / etc. against the new level.

### Criterion 7.3 — `reSaveCharacter` Tauri command
- **Files touched:** `apps/desktop/src-tauri/src/characterHub/reSaveCharacter.rs`.
- **RED:** re-saving an existing character at the same path fails (write conflict).
- **GREEN:** re-saving preserves revision_id (`{id}.rev.N` counter increments; duracon 2026-07-18 18:20:41 hardcoded the hardcoded `.rev.1` is replaced).

### Criterion 7.4 — Add Weapon / Add Armor / Add Spell onClick wired to real corpus data
- **Files touched:** `apps/desktop/src/characterHub/**/*.tsx` for picker modal flows + onClick handlers.
- **RED:** clicking "Add Weapon" does nothing (`onClick={() => {}}` per the wired-integration audit pattern).
- **GREEN:** the picker modal opens, queries real corpus data, and the chosen item appends to the character's loadout via the new Tauri command surface.

### Criterion 7.5 — `character_hub.rs::compose_character_input` loadout hardcoding removed
- **Files touched:** `apps/desktop/src/characterHub/characterHubRuntime.ts` (frontend TS, not src-tauri). `compose_character_input` is a Rust fn at `character_hub.rs:211`, not in that TS file. — (corrected 2026-07-22 per SD-25 criterion 7.P: path was `src-tauri/.../characterHubRuntime.ts` (doesn't exist); real path is `src/.../characterHubRuntime.ts` frontend file; `compose_character_input` Rust function lives in `character_hub.rs` not in TS)
- **RED:** loadout is a hardcoded array of `null`/`[empty]` defaults (per duracon 2026-07-18 18:20:41).
- **GREEN:** loadout reads from the saved character's equipment list (the new Tauri command surface).

## Epic 8 — Closure Epilogue (final scan + PR + worktree cleanup + release notes + version increment; fires LAST)

**Epic purpose:** the standard part-of-handoff-doctrine for every spec-domain closure. Epic 8 fires LAST. Its cycle scans every prior criterion (1-35) for `complete` or `## Open blockers` status; opens the `tranche/5-2 → develop` closure PR; cleans up worktrees and stale branches; generates release notes; runs the architecture-truth-up + graphify-update closure-pipeline sub-steps.

### Criterion 8.1 — Final criterion scan (criteria 1-35 evaluation)
- **Cycle artifact:** evaluation cycle output written to `./artifacts/epic_8/final-criterion-scan-cycle_receipt.md`.
- **Behavior:** for each criterion 1-35, verify `## Status matrix` says `complete`. If any criterion is `in-progress` or `returned-to-backlog`, self-heal cycles run until every criterion is `complete` or has a `## Open blockers` entry.

### Criterion 8.2 — Architecture closure pipeline (truth-up + graphify + PR + merge)
- **Files touched:** `../../architecture/` (read-only survey), `./progress.md` (state), `./receipts.md` (cycle artifacts).
- **Steps (in order):**
  1. Run `architecture-truth-up` script at `~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py` with `--integration-target <target> --receipts-md ./receipts.md --bundle SD-24`. The script edits touched docs in place, refreshes `Last verified:` headers, and appends a YAML receipt to `receipts.md`.
  2. Run `graphify-update` script with the same flags. Captures stdout/stderr/exit-code; appends a `graphify:update` receipt to `receipts.md`. Non-zero exit does NOT refuse the closure pipeline; the failure receipt is the audit trail.
  3. Open the `tranche/5-2 → develop` PR via `gh pr create`.
  4. Run `merge-conflict-resolution` script in pre-flight mode (rebase target) and post-pr mode (GitHub API mergeable state). Resolve any conflicts; re-run until clean.
  5. Stop the loop.

### Criterion 8.3 — Release notes generated at `./release-notes.md`
- **Cycle artifact:** `./release-notes.md` per the template's REQUIRED_NOTES_SECTIONS: Summary, User-Visible Changes, Defects Fixed, Operational Notes, Verification Evidence, Known Issues, Update Eligibility.
- **Files touched:** `./release-notes.md`.

### Criterion 8.4 — Build version increment lands at `0.5.<next_build>`
- **Files touched:** `Cargo.toml`, `apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`.
- **Note:** per the `<major>.<tranche-base>.<build>` scheme, the tranche position is not incremented (tranche/5-2 still carries tranche-base=5); only build counter increments.

---

## Quick reference — 35 acceptance criteria

| Epic | Criterion count | Range |
|------|-----------------|-------|
| Epic 1 (Identifier Cleanup) | 2 | 1.1–1.2 |
| Epic 2 (Operator Pre-Launch) | 5 | 2.1–2.5 |
| Epic 3 (Wired-Integration Audit + Remediation) | 4 | 3.1–3.4 |
| Epic 4 (Per-Class Coverage Audit) | 5 | 4.1–4.5 |
| Epic 5 (Multiclass F+W to 10) | 5 | 5.1–5.5 |
| Epic 6 (Equipment 100%) | 5 | 6.1–6.5 |
| Epic 7 (Unwired Workflows + Tauri Surface) | 5 | 7.1–7.5 |
| Epic 8 (Closure Epilogue) | 4 | 8.1–8.4 |
| **Total** | **35** | — |

Plus an open-ended self-heal-cycles budget for Epic 3's remediation backlog and Epic 4's class-coverage remediation; self-heal-cycles are dispatched under the deterministic-then-dynamic picker.

Note: the `scope-draft.md`, `decisions.md`, and `loop-instruction.md` previously cited "30 criteria" — that was a planning-round miscount. The actual seed is **35 criteria** across 8 epics; the dispatcher walks criterion numbers in epic+number order (1.1, 1.2, 2.1, ..., 8.4).
