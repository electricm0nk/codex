# SD-24 — Progress

> **Operating method:** see `./scope-draft.md` and `./loop-instruction.md`. This file is created on cycle 0 of Epic 2 with the deterministic seed of 35 criteria. Cycle picker reads `## TODO` + `## DISCOVERED` per the deterministic-seeded-then-dynamic model.

This file is the bundle's runtime state. The loop's `progress.md` is the canonical cycle-log + status matrix; the kanban board is the durable receipt; the per-cycle `artifacts/<epic>/<cycle>_cycle_receipt.md` is the per-cycle truth.

`## TODO` — the deterministic seed + dynamic entries, in dispatch-priority order. The picker reads this first.

`## DONE` — completed criteria, with commit SHA and cycle-id. Append-only.

`## DISCOVERED` — cycle-found items outside the deterministic list. Each entry has `<ISO-8601> | <epic-of-origin> | <criterion-of-origin> | <priority-bump-tag> | <description> | <suggested-epic-and-criterion>`. Priority-bump items go to the front of `## TODO` on operator call.

`## Status matrix` — per-criterion state in epic+criterion-number order (1.1, 1.2, 2.1, ..., 8.4). Update on every cycle's Step 9.

`## Cycle log` — append-only cycle entries (per `./loop-instruction.md §3` schema).

`## Open blockers` — non-self-healable items requiring operator intervention.

## Status matrix (placeholder — populated by cycle 0 of Epic 2)

| Criterion | State | Cycle ID | Commit SHA | Notes |
|---|---|---|---|---|
| 1.1 Source-code identifier audit | complete | identifier-audit-cycle | 3c3882a | Corrected-pattern audit found + remediated 6 sd16_/sd19_ modules, a resource dir, 13 SD19_* consts; see ## DISCOVERED |
| 1.2 Per-cycle tests pass | complete | identifier-audit-cycle | 3c3882a | cargo test --locked --tests (root) + cargo test --locked (src-tauri) both 0 failures |
| 2.1 board reachable | complete | operator-pre-launch-cycle | d6cddac | `hermes kanban boards` shows codex-tranche-5 |
| 2.2 branch pushed | complete | operator-pre-launch-cycle | d6cddac | ls-remote resolves, matches local HEAD |
| 2.3 SD-23 closure PR merged | complete | operator-pre-launch-cycle | d6cddac | Tier-1 launch gate CONFIRMED: 09e43c3 (PR #329) is HEAD of origin/develop |
| 2.4 working tree clean | complete | operator-pre-launch-cycle | d6cddac | git status --porcelain empty |
| 2.5 doctrines loaded | complete | operator-pre-launch-cycle | d6cddac | gap re-confirmed non-blocking per ./missing_skills.md; dual-audit grep gate is the operative check and is clean |
| 3.1 Wired-Integration Audit | complete | wired-integration-audit-cycle | 8756340 | Repo-wide four-check audit; 1 genuine finding (GE-07 pilot-shell-snapshot scaffold), deferred; see ## DISCOVERED |
| 3.2 forbidden tokens remediation | complete | wired-integration-audit-cycle | 8756340 | Zero-tolerance tokens (STUB/MOCK/todo/fixme/hack) clean repo-wide; the one `placeholder` finding deferred per risks-and-open-questions.md §5 |
| 3.3 noop handlers remediation | complete | wired-integration-audit-cycle | 8756340 | 0 findings repo-wide; nothing to remediate |
| 3.4 mock leaks + "Would …" remediation | complete | wired-integration-audit-cycle | 8756340 | 0 findings repo-wide; nothing to remediate |
| 4.1 per-class audit CRB | not-started | — | — | — |
| 4.2 per-class audit APG | not-started | — | — | — |
| 4.3 per-class audit ACG | not-started | — | — | — |
| 4.4 remediation plan | not-started | — | — | — |
| 4.5 APG/ACG multiclass deferral | not-started | — | — | — |
| 5.1 F+W multiclass dispatch | not-started | — | — | gated on 4.x |
| 5.2 30 character-advancement cycles | not-started | — | — | gated on 5.1 |
| 5.3 integration test | not-started | — | — | gated on 5.2 |
| 5.4 multiclass four-check audit | not-started | — | — | gated on 5.3 |
| 5.5 APG/ACG multiclass deferred | not-started | — | — | — |
| 6.1 Equipment coverage audit | not-started | — | — | — |
| 6.2 Equipment content completion: cost | not-started | — | — | gated on 6.1 |
| 6.3 Equipment content completion: weight | not-started | — | — | gated on 6.1 |
| 6.4 Equipment content completion: description | not-started | — | — | gated on 6.1 |
| 6.5 Spell content completion: full text | not-started | — | — | gated on 6.1 |
| 7.1 appendToCharacter Tauri command | not-started | — | — | gated on 5/6 |
| 7.2 recomputeCharacter Tauri command | not-started | — | — | — |
| 7.3 reSaveCharacter Tauri command | not-started | — | — | — |
| 7.4 Add Weapon/Armor/Spell onClick | not-started | — | — | — |
| 7.5 loadout hardcoding removed | not-started | — | — | — |
| 8.1 Final criterion scan | not-started | — | — | fires LAST |
| 8.2 Architecture closure pipeline | not-started | — | — | fires LAST |
| 8.3 Release notes | not-started | — | — | fires LAST |
| 8.4 Build version increment | not-started | — | — | fires LAST |

## TODO (deterministic seed; populated by cycle 0 of Epic 2)

- 4.1, 4.2, 4.3, 4.4, 4.5 (Epic 4, per-class audit)
- 5.1, 5.2, 5.3, 5.4, 5.5 (Epic 5, multiclass F+W)
- 6.1, 6.2, 6.3, 6.4, 6.5 (Epic 6, equipment 100%)
- 7.1, 7.2, 7.3, 7.4, 7.5 (Epic 7, unwired workflows + Tauri surface)
- 8.1, 8.2, 8.3, 8.4 (Epic 8, closure epilogue)

## DONE

- 1.1 Source-code identifier audit — cycle `identifier-audit-cycle`, commit `3c3882a`, receipt `./artifacts/epic_1/identifier-audit-cycle_receipt.md`
- 1.2 Per-cycle tests pass — cycle `identifier-audit-cycle`, commit `3c3882a`, receipt `./artifacts/epic_1/identifier-audit-cycle_receipt.md`
- 2.1 board reachable — cycle `operator-pre-launch-cycle`, receipt `./artifacts/epic_2/operator-pre-launch-cycle_receipt.md`
- 2.2 branch pushed — cycle `operator-pre-launch-cycle`, receipt `./artifacts/epic_2/operator-pre-launch-cycle_receipt.md`
- 2.3 SD-23 closure PR merged (Tier-1 launch gate CONFIRMED) — cycle `operator-pre-launch-cycle`, receipt `./artifacts/epic_2/operator-pre-launch-cycle_receipt.md`
- 2.4 working tree clean — cycle `operator-pre-launch-cycle`, receipt `./artifacts/epic_2/operator-pre-launch-cycle_receipt.md`
- 2.5 doctrines loaded (known non-blocking skills gap re-confirmed, see `./missing_skills.md`) — cycle `operator-pre-launch-cycle`, receipt `./artifacts/epic_2/operator-pre-launch-cycle_receipt.md`
- 3.1 Wired-Integration Audit (read-only sweep, codebase-wide) — cycle `wired-integration-audit-cycle`, receipt `./artifacts/epic_3/wired-integration-audit.md`
- 3.2 forbidden tokens remediation (zero-tolerance clean; 1 finding deferred) — cycle `wired-integration-audit-cycle`, receipt `./artifacts/epic_3/wired-integration-audit.md`
- 3.3 noop handlers remediation (0 findings) — cycle `wired-integration-audit-cycle`, receipt `./artifacts/epic_3/wired-integration-audit.md`
- 3.4 mock leaks + "Would …" remediation (0 findings) — cycle `wired-integration-audit-cycle`, receipt `./artifacts/epic_3/wired-integration-audit.md`

## DISCOVERED

- `2026-07-21T00:00:00Z` | epic-1 | criterion-1.1 | governance-pattern-bug | The canonical identifier-discipline grep in `epic-breakdown.md`/`loop-instruction.md §2.3 step 4` (`` \b(sd(16|19|22|23|24)_|SD(16|19|22|23|24)_|Sd(16|19|22|23|24)|t_[0-9a-f]{8,})\b `` ) has a trailing-`\b` boundary bug: `\b` never matches between an underscore and a following word character, so the pattern silently fails to catch any bundle-tag identifier that is followed by more identifier characters (e.g. `sd19_class_catalog`) and only matches a bare standalone token like `sd24_` on its own. Live proof: the literal command returned 0 hits against the pre-remediation repo while 6 production Rust modules, a bundled Tauri resource directory, and 13 constants were genuinely bundle-tag-prefixed. A corrected pattern (drop the trailing `\b`) is now load-bearing in `tests/sd24_identifier_discipline_audit.rs`. | suggested: harden `governance/identifier-discipline.md`'s canonical pattern (or `epic-breakdown.md`/`loop-instruction.md`'s embedded command) in a follow-up governance-doc cycle — out of this cycle's granted write scope.
- `2026-07-21T00:00:00Z` | epic-3 | criterion-3.1 | epic-3-originated | Codebase-wide four-check wired-integration audit found one genuine, pre-existing finding: `apps/desktop/src-tauri/src/main.rs`'s `load_pilot_shell_snapshot` Tauri command + `apps/desktop/src/boundary/loadPilotShellSnapshot.ts` unconditionally return hardcoded fixture data (`case_id: "ge07-e1-scaffold-placeholder"`) regardless of real character state — fixture-only data in a production command path. Legacy GE-07 scaffolding (predates the `SD-<N>` bundle-tag convention), consumed only by the SD-11 internal tester workbench which already labels it honestly. Recorded in `risks-and-open-questions.md §5 Deferrals` (not the Stubs Registry — accidental finds without operator-verbatim justification route to risks-and-open-questions.md per the registry's own rule) rather than remediated in-cycle, since real remediation needs an operator design decision (what a "headless-core-backed" pilot shell snapshot computes, from what input contract) outside an audit-and-mechanical-remediation cycle's granted scope. A standing regression test (`tests/sd24_wired_integration_audit.rs`) tolerates only this named finding. | suggested: epic-3-or-later Wired Integration Cleanup follow-on, or an operator-pinned design cycle for the GE-07 pilot-shell-snapshot feature.

## Cycle log

- **cycle_id:** identifier-audit-cycle (2026-07-21T00:00:00Z)
  **criterion:** 1.1 + 1.2 (Epic 1)
  **summary:** Ran the repo-wide identifier-discipline scan per criterion 1.1's own verbatim command; discovered the command's trailing-\b regex has a boundary bug that produces a false-clean 0-hit result. Ran a corrected (boundary-fixed) pattern instead, which found real bundle-tag leaks: 6 production Rust modules (`sd16_browser_handoff`, `sd19_class_catalog`, `sd19_corpus`, `sd19_equipment_catalog`, `sd19_race_catalog`, `sd19_spell_catalog`), a bundled Tauri resource directory (`resources/sd19_corpus_fixtures/`), and 13 `SD19_*_TEST` constants in `support_state_matrix.rs`. Remediated all of them (renamed files/modules/consts/resource dir, updated every call site, doc-comment citation, and `tauri.conf.json` path). Added `tests/sd24_identifier_discipline_audit.rs` as a standing regression test using the corrected pattern. RED→GREEN demonstrated live via `git stash`: stashed the remediation, confirmed the new test fails listing the real pre-remediation leaks, restored the remediation, confirmed the test (and the full `cargo test --locked --tests` root suite, and `cargo test --locked` in `apps/desktop/src-tauri`) all pass with 0 failures. Both dual-audit gates (`identifier-discipline`, `wired-integration-discipline`) are clean on the final diff.
  **commit:** `3c3882a`
  **receipt:** `./artifacts/epic_1/identifier-audit-cycle_receipt.md`
  **discovered:** 1 entry (governance-pattern-bug, see `## DISCOVERED`)
  **next:** Epic 2 (Operator Pre-Launch, criteria 2.1–2.5) is next in the deterministic seed.

- **cycle_id:** operator-pre-launch-cycle (2026-07-21)
  **criterion:** 2.1 + 2.2 + 2.3 + 2.4 + 2.5 (Epic 2, Operator Pre-Launch)
  **summary:** Re-verified all 5 items of `loop-instruction.md §1`'s pre-launch checklist plus `acceptance-and-verification.md`'s Epic-2 verification commands, live, right now (this is a re-confirmation gate mid-bundle, not the original first-launch check). Results: (2.1) `hermes kanban boards` lists `codex-tranche-5` (done=43) — PASS. (2.2) `git ls-remote origin tranche/5-2` resolves to `e433a63...`, matching local HEAD after a no-op fetch+rebase — PASS. (2.3, **Tier-1 launch gate**) `git fetch origin develop` then `git log origin/develop --oneline | head -5` shows `09e43c3` (SD-23 closure, PR #329) as the literal HEAD of `origin/develop` — **CONFIRMED, bundle is NOT blocked** — PASS. (2.4) `git status --porcelain` empty — PASS. (2.5) re-checked `hermes skills list`: `identifier-discipline` / `wired-integration-discipline` / `kanban-claude-code-execution-receipt` are still not installed as invocable skills — this reconfirms the gap already tracked non-blocking in `./missing_skills.md` (2026-07-21); the operative gate (the embedded dual-audit grep in `loop-instruction.md §2.3` step 4) was re-run against the diff since `merge-base HEAD origin/develop` and is clean (`OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`) — PASS with known caveat. Also cross-checked pre-launch item 7 (build counter): `develop`'s `package.json`/`tauri.conf.json` both still `0.5.97`, no drift from `decisions.md §3`'s captured value. No production/test files touched (verification-only cycle); no TDD RED/GREEN applicable.
  **commit:** `d6cddac`
  **receipt:** `./artifacts/epic_2/operator-pre-launch-cycle_receipt.md`
  **discovered:** 0 new entries (a minor stale-CLI-form nit in `acceptance-and-verification.md` line 22 — `hermes skills --profile god-emporer --list` errors; correct form is `hermes skills list` — noted in the receipt's Notes but not filed as a fresh `## DISCOVERED` entry since it doesn't change any criterion's disposition)
  **next:** Epic 3 (Wired-Integration Audit + Remediation, criterion 3.1 — read-only sweep) is next in the deterministic seed.

- **cycle_id:** wired-integration-audit-cycle (2026-07-21)
  **criterion:** 3.1 + 3.2 + 3.3 + 3.4 (Epic 3, Wired-Integration Audit + Remediation)
  **summary:** Ran the full four-check wired-integration audit codebase-wide (per `acceptance-and-verification.md` CG-02's "covers the entire codebase" requirement, broader than the per-cycle diff-scoped gate) against `apps/desktop/`, `apps/desktop/src-tauri/`, `src/` via `git grep` (tracked files only, so `src-tauri/target/` build artifacts are naturally excluded). Checks 2 (no-op onClick handlers), 3 (mock leaks), and 4 ("Would ..." strings) are 0 hits, repo-wide -- nothing to remediate for 3.3/3.4. Check 1's zero-tolerance alternatives (STUB/MOCK/not yet implemented/todo/fixme/hack) are also 0 hits repo-wide (confirmed with a supplementary case-insensitive TODO/FIXME sweep too). Check 1's noisy `placeholder` alternative produced 71 raw hits; manual per-hit review classified 70 as benign (HTML/JSX input-placeholder-text affordances, CSS `::placeholder`, or ordinary engineering prose using "placeholder" as a sentinel-value concept) and 1 genuine finding: `load_pilot_shell_snapshot` (Tauri command in `main.rs` + `loadPilotShellSnapshot.ts`) unconditionally returns hardcoded GE-07-era fixture data regardless of real state -- fixture-only data in a production command path. This is legacy scaffolding predating the SD-N convention, consumed only by the SD-11 internal tester workbench (which already labels it honestly), and its real fix needs an operator design decision outside this audit-and-mechanical-remediation cycle's scope -- deferred to `risks-and-open-questions.md §5 Deferrals` (not the Stubs Registry, since that requires operator-verbatim justification this accidental find doesn't have) per the self-heal path in `loop-instruction.md §4.1`. Wrote a standing regression test `tests/sd24_wired_integration_audit.rs` (5 tests: zero-tolerance tokens, placeholder-bucketed, no-op handlers, mock leaks, would-strings) that tolerates only the one named/documented finding and fails on anything new. RED->GREEN demonstrated live: disabled the documented-deferred-finding exclusion bucket, ran the test, it failed listing the real finding's 7 lines plus 9 previously-unbucketed `pilot_compute.rs` anti-fabrication-explanation lines (a legitimate design pattern using "packet placeholder" as an anti-fabrication assurance, not a stub); re-enabled the bucket and added a 4th bucket for the `pilot_compute.rs` pattern, ran again -- 5 passed, 0 failed. Full suite re-run: `cargo test --locked --tests` (root) 3955 passed/0 failed; `cargo test --locked` (`apps/desktop/src-tauri/`) 113 passed/0 failed. Both dual-audit gates (`identifier-discipline`, `wired-integration-discipline`) on the diff since `merge-base HEAD origin/develop` are clean (`OK_NO_BUNDLE_TAGS`/`OK_NO_TOKENS`).
  **commit:** `8756340`
  **receipt:** `./artifacts/epic_3/wired-integration-audit.md`
  **discovered:** 1 entry (epic-3-originated, GE-07 pilot-shell-snapshot scaffold deferral, see `## DISCOVERED`)
  **next:** Epic 4 (Per-Class Coverage Audit + Plan, criterion 4.1) is next in the deterministic seed.

## Open blockers

(empty — populated by non-self-healable failures)

---

*Per `./loop-instruction.md §2.3 step 9`: the cycle picker updates this file in place on every cycle. Do not rewrite from scratch; each cycle appends to its own section.*
