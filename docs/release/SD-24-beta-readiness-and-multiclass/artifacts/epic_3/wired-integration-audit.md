# Wired-Integration Audit — Epic 3, Criteria 3.1-3.4

**Cycle:** `wired-integration-audit-cycle` (2026-07-21)
**Scope:** repo-wide, all four checks from `governance/no-stub-mvp-doctrine.md` §"Per-cycle audit" / `wired-integration-discipline`, run against the full working tree of `apps/desktop/`, `apps/desktop/src-tauri/`, `src/` (per `acceptance-and-verification.md` CG-02: "Wired-Integration Audit Cycle (3.1) covers the entire codebase" — broader than the per-cycle diff-scoped gate in `loop-instruction.md §2.3` step 4, which only checks `${BASE_BRANCH}...HEAD`).

## Method

Ran `git grep` (tracked files only — `apps/desktop/src-tauri/target/` is untracked/gitignored and correctly excluded) for each of the four checks, then manually reviewed every hit for a genuine forbidden pattern vs. a benign false positive (HTML/JSX `placeholder` text, prose using "placeholder" as an ordinary English word for a sentinel/default value, etc.).

## Results

| Check | Pattern | Repo-wide hits (raw) | Genuine findings |
|---|---|---|---|
| 1a — zero-tolerance tokens | `\b(STUB\|MOCK\|not yet implemented\|todo\|fixme\|hack)\b` | 0 | 0 |
| 1b — `placeholder` | `\bplaceholder\b` | 71 (excluding `target/`, test files) | **1** (7 lines across 3 files, one feature) |
| 2 — no-op `onClick` | `onClick=\{...\{\}\}\|onClick=\{undefined` | 0 | 0 |
| 3 — mock leaks | `mockResolvedValue\|mockReturnValue(\|vi\.mock(\|__mocks__` | 0 | 0 |
| 4 — `"Would ..."` strings | `"Would [^"]*"` | 0 | 0 |

Also ran a supplementary case-insensitive `\b(TODO|FIXME)\b` sweep (not part of the canonical pattern, which is case-sensitive) to check for uppercase debt the canonical check would miss: **0 hits**, repo-wide.

### The one genuine finding

`load_pilot_shell_snapshot` — a Tauri command (`apps/desktop/src-tauri/src/main.rs:53-68`) and its frontend caller (`apps/desktop/src/boundary/loadPilotShellSnapshot.ts`) — unconditionally returns hardcoded fixture data (`case_id: "ge07-e1-scaffold-placeholder"`, fixed `receipt_status: "Unknown/Unavailable"`, no real computation) regardless of any actual character/pilot state. This is fixture-only data crossing into a production Tauri command path (doctrine forbidden-pattern #5).

- **Files:** `apps/desktop/src-tauri/src/main.rs:56`, `apps/desktop/src/boundary/loadPilotShellSnapshot.ts:13,18,27,28`, plus two consuming references in `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts:611,616,618` and one in `apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts:122` that *honestly label* the same known placeholder (not a second stub).
- **Provenance:** the `ge07` tag predates the `SD-<N>` bundle-tag convention entirely (pre-dates even SD-11) — this is old scaffolding, not something introduced by SD-24 or any bundle after the 2026-07-20 "no more stubs" directive.
- **Blast radius:** consumed only by the SD-11 internal tester workbench (a developer-facing diagnostic surface), which already surfaces it honestly — `main.rs`'s own doc comment says "Frontend fallback must never masquerade as product truth," and `loadSd11TesterWorkbenchSurface.ts` strips the placeholder's marker diagnostics before display rather than presenting them as real data. Not a user-facing character-sheet affordance.
- **Why not remediated in this cycle:** real remediation requires an operator design decision ("a read-only Tauri command backed by the headless core" per the code's own note) — there is no existing input contract (the command takes no parameters) for what a real "pilot shell snapshot" would compute from, and wiring this to `rules_core` is out of this cycle's granted repo-wide-audit scope (which is read/audit + mechanical remediation, not new feature design). This is the exact "defer to risks-and-open-questions.md if the fix is non-trivial" self-heal path from `loop-instruction.md §4.1`.
- **Disposition:** recorded in `risks-and-open-questions.md` §5 Deferrals as a Wired Integration Cleanup candidate (not the Stubs Registry — per that registry's own rule, "exceptions are operator-granted, not self-asserted," and accidental-debt finds route to `risks-and-open-questions.md` instead). A standing regression test (`tests/sd24_wired_integration_audit.rs`) tolerates only this specific, named finding and fails the build on any new or additional forbidden-pattern hit.

### Remediation backlog (criteria 3.2-3.4)

- **3.2 (forbidden tokens):** the one finding above is a `placeholder` hit. No mechanical remediation performed this cycle (design decision needed, deferred per above); everything else zero-tolerance-clean.
- **3.3 (no-op handlers):** 0 findings repo-wide — nothing to remediate.
- **3.4 (mock leaks + "Would ..." strings):** 0 findings repo-wide — nothing to remediate.

## TDD evidence

RED: `tests/sd24_wired_integration_audit.rs`'s `placeholder_findings_are_ui_text_prose_or_the_one_documented_deferral` test, written with its documented-deferred-finding exclusion bucket temporarily disabled, failed listing the exact 7 real lines above plus 9 `pilot_compute.rs` anti-fabrication-explanation lines not yet bucketed (`cargo test --locked --test sd24_wired_integration_audit` → 1 failed, 4 passed).

GREEN: restored the documented-deferred-finding bucket (matching `scaffold-placeholder` / `loadPilotShellSnapshot` / `"Future slices should replace this placeholder"` / `placeholder/fallback`) and added a fourth bucket for `pilot_compute.rs`'s legitimate `packet placeholder` anti-fabrication explanation strings (runtime `format!` text, not doc comments, so the comment-prose bucket didn't already cover them) — `cargo test --locked --test sd24_wired_integration_audit` → 5 passed, 0 failed.

Full suite re-run after GREEN: `cargo test --locked --tests` (repo root) → 3955 passed, 0 failed. `cargo test --locked` (`apps/desktop/src-tauri/`) → 113 passed, 0 failed.

## Dual-audit gate (diff-scoped, per `loop-instruction.md §2.3` step 4)

Run against `${BASE_BRANCH}...HEAD` where `BASE_BRANCH = git merge-base HEAD origin/develop`:

- Identifier audit: `OK_NO_BUNDLE_TAGS`
- Wired-integration token audit: `OK_NO_TOKENS`

(The new standing test file lives at `tests/*.rs`, outside the diff-gate's three named path globs, same as `tests/sd24_identifier_discipline_audit.rs` from the Epic 1 cycle.)

## Next-cycle plan

Epic 3 (3.1-3.4) closes with the audit run, backlog enumerated, and the zero real-work-required checks (2/3/4) confirmed clean. The one genuine finding is deferred (not blocking) per the documented rationale above. Epic 4 (Per-Class Coverage Audit) is next in the deterministic seed.
