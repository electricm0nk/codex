# SD-24 — Risks and Open Questions

> **Operating method:** see `./scope-draft.md`. Self-heal runs against this document's split — self-healable items resolve inline; non-self-healable items land in `## Open blockers` on `./progress.md` and trigger operator intervention.

## 1. Self-healable conditions

| Condition | Self-heal |
|---|---|
| Working tree dirty from a prior failed cycle | `git checkout -- <file>` or `git reset --hard HEAD~1` |
| Identifier audit finds a single `sd<N>_*` / `Sd<N>` leak | rename inline; re-audit; commit as part of the same cycle |
| Wired-integration audit finds a single forbidden token (1-3 occurrences in a single file) | remove the token; re-audit; commit as part of the same cycle |
| Cycle's tests fail for an unrelated reason (broken test setup, missing fixture) | fix the test setup; do not carry the cycle forward |
| Cycle finds a stub not in the operator's count of "designed stubs" (2-3 per the operator 2026-07-20 11:30:56) | record in Stubs Registry as accidental debt; remediate in the same cycle or a follow-on |
| Build counter out of sync with develop | re-read develop's `Cargo.toml`; update `decisions.md §3` |
| Cycle's criterion-row in `## Status matrix` disagrees with `## DONE` | reconcile by re-running the cycle artifact generation |
| Cycle's `## DISCOVERED` entry is a duplicate of an existing one | merge duplicates; mark de-dup |
| Cycle finds a missing unit test that should exist | write the test in the same cycle; commit together |
| Cycle finds a stub the operator designed (max 2-3 in operator's count) | Stubs Registry entry creation is operator-granted; cycle does NOT auto-create |

## 2. Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Action |
|---|---|
| Working tree diverged from `tranche/5-2` in a way that needs manual rebase | `## Open blockers`; exit FAIL |
| Two live `claude` processes on conflicting files | First wins; second writes `## Open blockers`; exit FAIL |
| SD-23 closure PR not merged to develop (Tier-1 launch gate) | Loop refuses to dispatch; cycle exits FAIL |
| `## DISCOVERED` queue exceeds 10 entries | Operator override required; cycle pauses; cycle exits FAIL |
| Epic 4 finds APG/ACG classes are *not* fully wired | Multiclass Epic 5 scope is restricted to Fighter + Wizard only; defer APG/ACG-class multiclass; document in `risks-and-open-questions.md` `## Deferrals` |
| Epic 6 finds the strict-field-coverage threshold cannot be met within SD-24's cycle budget | `## Open blockers`; operator decides on threshold relaxation or deferral |
| Cycle's RED → GREEN transition is not preserved in the artifact | Cycle re-run with RED → GREEN captured; do not mark `complete` |
| A cycle lands a PR-URL comment into the kanban card before the commit lands at origin (respawn-guard footgun per duracon 2026-07-04 12:41:37) | Operator override to bypass the respawn-guard rule; cycle continues |
| Cycle finds `success: true` returns from operations that did not actually do the work | Cycle is rejected; falls back to wired-integration audit; cannot mark `complete` |
| Cycle finds inline mock libraries in shipping modules (not `__tests__` / `*.test.*`) | Cycle is rejected; cannot mark `complete` |
| Cycle finds "Would …" / "Will simulate …" return strings in shipping code | Cycle is rejected; cannot mark `complete` |

## 3. Override flags

Operator-pinned flags that suppress criterion-row dispatch until cleared. Format: `FLAG-<letter>: <description>` with optional `RESET <reason>` comment.

| Flag | Status | Notes |
|---|---|---|
| FLAG-A: STRICT-STOP-AT-5AM | unset | Default-and-flag: unset = grace-tail at 5am (current cycle completes; no new cycles spawn; absolute ceiling 5:30am). Set = strict stop at 5am regardless of cycle-in-flight. Operator sets this to control hard-stop shape. |
| FLAG-B: APG-MULTICLASS-DEFER | unset | Default-and-flag: unset = cycle does not dispatch any APG/ACG-class multiclass criterion (cycle writes to `## Open blockers` if it accidentally lands). Set = APG/ACG-class multiclass is dispatch-eligible. |
| FLAG-C: EQUIPMENT-FIELD-COVERAGE-RELAXED | unset | Default-and-flag: unset = strict 100% field coverage (per `decisions.md §5`). Set = relaxed (operator specifies threshold). Operator sets this only after Epic 6's audit reveals the strict threshold cannot be met. |
| FLAG-D: SKIP-WIRED-INTEGRATION-AUDIT | unset | Default-and-flag: unset = audit runs on every code-bearing cycle. Set = audit suppressed (operator override only; not recommended). |
| FLAG-E: KANBAN-DISPATCH-BYPASS | unset | Default-and-flag: unset = kanban does not dispatch (per `decisions.md §7`). Set = dispatcher falls back to kanban ready-queue (NOT recommended; reverting to SD-23 retrospective's failed pattern). |

## 4. Open questions (deferred to operator)

| Q | Question | Deferred from | Default behavior |
|---|---|---|---|
| Q1 | APG/ACG-class multiclass delivery vehicle: SD-24 Epic 5 deferred; is the follow-on bundle a SD-25 immediately after closure, or an operator-pinned later bundle? | Operator 2026-07-21 §1 | Default: deferral to SD-25 (immediately following SD-24 closure). Operator may pin differently. |
| Q2 | Equipment corpus extension: if the field-coverage audit reveals items beyond the PF1 core rules + APG + ACG + Bestiary 1 corpus (e.g. ARG / Ultimate-line books), are they in scope? | Operator 2026-07-21 §5 | Default: out of scope (only the operator-pinned corpus is in scope). Operator sets FLAG-C to expand scope. |
| Q3 | Tauri command-surface extension: are there additional Tauri commands beyond `appendToCharacter` / `recomputeCharacter` / `reSaveCharacter` needed for beta-readiness (e.g. `deleteCharacter`, `importCharacter`)? | Operator 2026-07-20 11:38:02 Option A | Default: in-scope for SD-24 per the Option A ruling for storage-tier convergence. Operator may suppress FLAG-B path. |
| Q4 | Loader/preview fallback: the operator's Option A ruling has a "browser preview fallback" caveat (per the Stubs Registry entry #0001 at `apps/desktop/src/characterHub/characterHubRuntime.ts:17-18`); is this entry still required post-SD-24? | Operator 2026-07-20 12:37:07 | Default: entry persists until operator-granted removal. SD-24 Epic 7 may incidentally clean it up; otherwise deferred. |
| Q5 | Build-counter inheritance from develop at SD-24 launch: is the next-build value captured in `decisions.md §3`? | Operator 2026-07-21 | Default: cycle 0 of Epic 2 captures this. Operator may pin a specific value. |
| Q6 | Hard-stop at 5am: grace-tail or strict? | Operator 2026-07-21 12:48:50 | Default: grace-tail (current cycle completes; absolute ceiling 5:30am). Set FLAG-A=SET for strict. |

## 5. Deferrals (operator-pinned non-self-healable items deferred to follow-on bundles)

- **APG/ACG-class multiclass (formalized by criterion 4.5, 2026-07-21).** Epic 4's coverage audit (criteria 4.2 and 4.3, `./artifacts/epic_4/per-class-coverage-matrix.md`) confirmed all 16 real APG/ACG classes (Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch, Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest) are chassis-only: BAB/saves fully wired and verified (20/20 levels each), but 0 named class features wired against corpus-derived expected counts, 0 live `pilot_compute.rs`/`compute_class_chassis` integration (proven empirically via the honest `class_chassis.unsupported` diagnostic, not fabricated data), and 0 `level_up::<class>` modules for any of the 16. This is the exact precondition `§2`'s "Epic 4 finds APG/ACG classes are *not* fully wired" row anticipated. Per operator directive 2026-07-21, SD-24 Epic 5's multiclass is Fighter+Wizard only (unaffected by this finding — Fighter+Wizard's own coverage, criterion 4.1, is complete and gap-free); APG/ACG-class multiclass is deferred to SD-25 (default per Q1) or an operator-pinned later bundle. Formal decision record + evidence: `./artifacts/epic_4/apg-acg-multiclass-deferred.md`. Remediation-plan status for the 16 classes' named-feature gaps (no SD-24 cycle-id assigned to any of them): `./artifacts/epic_4/remediation-plan.md §5`. `FLAG-B: APG-MULTICLASS-DEFER` remains unset (§3), so the cycle picker continues refusing to dispatch any APG/ACG-class multiclass criterion within SD-24.
- **Equipment corpus extension.** Beyond PF1 core rules + APG + ACG + Bestiary 1. Defer to operator-pinned later bundle.
- **Storage-tier structural convergence.** Per duracon 2026-07-20 11:38:02 Option A, SD-24 covers the file-based fixes (per-`rev.N` counter, append/recompute/re-save). The structural convergence to SQLite is deferred.
- **Inline mocks / "Would" strings beyond the bundle's cycle scope.** If Epic 3's audit finds inline mocks in modules outside SD-24's file-touch partition, the module is recorded for a follow-on Wired Integration Cleanup epic (outside SD-24).
- **Identifier-discipline directory renames.** `apps/desktop/src/sd<N>/` → `apps/desktop/src/<descriptive>/`. Per `identifier-discipline SKILL.md §Operator-recorded open calls`, directory renames are follow-on bundle work, not in-bundle.
- **GE-07 pilot-shell-snapshot scaffold (Wired Integration Cleanup candidate, found by Epic 3's codebase-wide audit, 2026-07-21).** `apps/desktop/src-tauri/src/main.rs`'s `load_pilot_shell_snapshot` Tauri command and its frontend caller `apps/desktop/src/boundary/loadPilotShellSnapshot.ts` unconditionally return hardcoded fixture data (`case_id: "ge07-e1-scaffold-placeholder"`) regardless of real character/pilot state — fixture-only data in a production command path (doctrine forbidden-pattern #5). Legacy scaffolding predating the `SD-<N>` bundle-tag convention; consumed only by the SD-11 internal tester workbench, which already labels it honestly to the viewer rather than presenting it as real diagnostic data. Not remediated in SD-24 Epic 3: real remediation needs an operator design decision on what a "headless-core-backed" pilot shell snapshot computes and from what input (the command currently takes no parameters), which is a feature-design call outside an audit-and-mechanical-remediation cycle's granted scope. Full audit writeup: `./artifacts/epic_3/wired-integration-audit.md`. This is NOT a Stubs Registry entry (registry entries require operator-verbatim justification, which this accidental find does not have) — it is a Wired Integration Cleanup candidate per the registry's own routing rule, deferred to a follow-on bundle or operator-pinned cycle. A standing regression test (`tests/sd24_wired_integration_audit.rs`) tolerates only this named finding and fails on any new/additional instance.

## 6. Latent risks (monitored but not-blocking)

- **SD-23 closure PR lag.** If SD-23's closure PR is the bundle's Tier-1 launch-gate dependency and SD-23's cycle 14 / Criterion 28 (per duracon 2026-07-21 09:24:59) is mid-cycle at 5am, SD-24's launch is blocked. Operator-direct: do not launch SD-24 until SD-23 closure is in develop.
- **PCGen corpus LST data freshness.** Epic 6's equipment coverage audit depends on the actual LST data. If the LST records are stale or incomplete, the audit may report missing coverage that can't be remediated without an upstream corpus re-ingest.
- **Oracle parity drift.** Epic 5's multiclass dispatch adds new code paths; the Oracle parity comparison (per SD-20's per-character rules-engine comparison with PF1 Oracle expectations) may regress. Cycle's GREEN phase must include oracle parity regression-check.
- **Graphify 2M-token clip.** Per the graphify-update sub-step in Epic 8: graphify token-budget can clip single-file chunks. SD-24's closure-pipeline graceful-degradation handles this.
- **5am cycle in-flight.** A 30-40 minute cycle that starts at 4:50am risks overshooting the 5:30am absolute ceiling. The cycle's Step 11 includes a "5am approaching?" gate; cycle 0 of Epic 2 captures this flag check.

## 7. Doctrine-distinction note (test-fixture path conventions)

The cycle picker references test fixtures by path using the `sd<N>_<descriptor>.rs` convention (e.g. `tests/sd24_multiclass_deterministic.rs`, `tests/sd24_characterhub_append.rs`, `tests/sd24_equipment_masterwork_backpack.rs`). This pattern appears in:

- `./content-unit-inventory.md` (per-content-unit routing tables)
- `./acceptance-and-verification.md` (per-criterion verification commands)
- `./epic-breakdown.md` (per-cycle stories)

Per SD-22's `corpus-source-inventory.md` pattern (which carries `tests/sd22_<book>_<class_or_subset>_resolves.rs` verbatim) and SD-23's loop-instruction Step 3 (`tests/sd22_<criterion>.rs` as the dispatch-referencing pattern), this is a **content-unit identifier** at the cycle dispatcher surface, not an identifier-discipline leak. The identifier-discipline doctrine forbids `sd<N>_*` in *source-code identifiers inside shipping modules* (functions, methods, constants, properties, Tauri commands, CSS classes, test-IDs rendering in UI); test-fixture file paths naming the cycle's content unit are part of the cycle's dispatch routing, mirroring SD-22's prior precedent.

The Epic 8 closure evaluator (criterion 8.1's final scan) is expected to recognize this distinction and not flag the test-fixture path references as identifier-discipline violations.

## 8. Cross-reference

- `./scope-draft.md §6 Hard-stop conditions` — bundle-level hard stops
- `./decisions.md §4` — multiclass scope (Fighter+Wizard only)
- `./decisions.md §5` — equipment-corpus strict 100% field coverage
- `./decisions.md §6` — publish mode (move-not-copy)
- `./loop-instruction.md §4` — self-heal posture
- `./loop-instruction.md §2.5` — discovery forwarding rules
