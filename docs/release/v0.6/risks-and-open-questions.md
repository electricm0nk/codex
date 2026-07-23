# v0.6 Alpha Release Swarm — Risks and Open Questions

## Operator override: wired-integration ceremony waiver (2026-07-23)

Per `docs/governance/no-stub-mvp-doctrine.md` (`scope: universal`), the four-check audit criterion is "not optional and not waivable except by explicit operator override recorded in `risks-and-open-questions.md`." This file records that override for the v0.6 alpha release swarm, approved by the operator (Todd Hintzmann) via the launch-readiness revision of 2026-07-23:

- **Waived:** the per-cycle receipt *ceremony only* — no cycle receipts under `programs/codex/requirements/`, no closure-epilogue doctrine, no per-bundle kanban board. The swarm's audit evidence lands in `docs/release/v0.6/SWARM_REPORT.md` instead (raw grep output pasted per §7.1 of `release-swarm.md`).
- **NOT waived:** the four-check audit itself (forbidden tokens, no-op handlers, mock-library leaks, "Would …" strings). It runs against the swarm's combined diff (`git diff develop...tranche/6`) before the closure PR opens, executed by the QA engineer and re-verified by the lead. There is no in-repo runner script; the four greps are run by hand per the doctrine's "Per-cycle audit" section.

## Risks

1. **Alpha-bar distance.** Current state (`docs/architecture/status.md`, 2026-07-23): only single-class Fighter 1–3 reaches a `Computed` receipt; 7 of 10 CharacterSheet tabs stubbed; Feat picker and Money panel do not exist; level-up/skill/bio persistence are no-ops. The §1 bar (6-level multiclass, four books, PCGen parity, zero stubs) requires multiple task waves; the go/no-go checkpoints in §6 are the containment mechanism.
2. **RESOLVED — 0.6 version bump.** Landed as `0c614d9`: `buildVersionTriple.test.ts:44-47` anchor updated `0.5.` → `0.6.` — and its previously-unnoticed sibling `apps/desktop/src/sd21/buildVersionTriple.test.ts` (backend caught it proactively, same documented reason, flagged for visibility rather than silently expanding scope). Triggered one downstream pre-existing test (`buildLabelFixtureFreshness.test.ts`, expects literal "Codex 0.6.0-test" in 3 frontend-owned fixtures) — frontend has this in flight.
3. **Three writers, one checkout.** No worktrees by design; commit serialization (§2) mitigates git-index and cargo target-dir races but depends on teammate discipline. First symptom to watch: a "committed" change absent from `git log` on `tranche/6`.
4. **Unsigned Windows installers.** SmartScreen interposition is expected and accepted for alpha (bar item 1 qualifier). Code-signing remains out of scope.
5. **Observer lane may be dark.** As of 2026-07-23 no listener and no tick loop are running (§8.3.1 status check). If the operator does not bring it up pre-launch, the only visibility is `SWARM_STATUS.md` via `cat`.

## Open questions

1. **Tranche-6 CI.** No tranche-specific workflow exists past `tranche-3-ci.yml` (none was authored for tranche/5 either). Does `tranche/6` need one, or do `publish-tester-release.yml` + the promotion gates suffice? Owner: operator, pre-launch checklist step 5.
2. **Book-stub label reconciliation.** SD-26's `decisions.md §10` pins the literal `"SD-27"` as the stub resolution bundle while all 21 landed stubs carry `"SD-27+ (unscheduled)"`. Not v0.6-blocking, but the swarm's content work touches `data/stubs/` — the operator should say which label wins before anyone "fixes" it in passing.
3. **Alpha-bar staging.** The bar is kept whole (not narrowed) in the revised doc; if checkpoint burn-rate makes the full bar untenable, the fallback is a staged bar (e.g. single-class alpha across CRB first). Operator decision at a checkpoint, not a teammate improvisation.
4. **RESOLVED — "durability" definition (§1 item 4).** QA flagged that the alpha bar's calculation list ("... AC, durability, carry capacity ...") is ambiguous between character survivability and item hardness/breakage; the codebase has no item-hardness/breakage mechanic anywhere. Ruling (lead, 2026-07-23): **durability = the character's displayed survivability stat — max/current HP, temporary HP, nonlethal damage tracking, and unconscious/dying/death thresholds** — distinct from "level-up hit points" (§1 item 4's separate entry), which is specifically the hit-die + Con-mod increment calculated during the level-up flow. This keeps the bar scoped to character-sheet mechanics already in the codebase's model and introduces no new game system. QA may proceed writing red-green tests against this definition.
