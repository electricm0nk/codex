# AGENTS.md

## Purpose

This file is the durable repo-root conduct surface for a coding harness operating inside this repository.

It does **not** define product scope by itself. It defines how the harness must behave once it wakes up in the repo.

Use the execution handoff or other explicitly cited implementation brief for task scope.

## Operating Model

- Treat this repository as the implementation surface unless the handoff explicitly grants write authority elsewhere.
- Treat the execution handoff as the bounded code-writing brief.
- Treat the source STC, grand epic, or other upstream planning artifacts as requirements inputs, not as permission to improvise implementation scope.
- Prefer the smallest compliant change.
- If required context, scope, or write authority is unclear, stop and report the missing truth instead of guessing.

## Required Minimum Handoff

Do **not** begin implementation work until the run is anchored by an execution handoff or equally explicit brief that names at least:

- exact objective
- exact target repo or workdir
- exact allowed write scope
- exact required reads
- explicit non-goals
- verification commands or acceptance evidence

If those fields are missing, stop. Planning-ready is not execution-ready.

## Non-Negotiable Rules

1. **TDD is mandatory.**
   - Write or update a failing test before changing production code.
   - Confirm the test fails for the intended reason.
   - Implement the smallest change needed to pass.
   - Run the relevant test set after each meaningful change.
   - Refactor only after green.

2. **No fake completion.**
   - Do not claim success because code looks plausible or merely compiles.
   - Report the real verification commands run and their actual results.
   - If you could not verify, say so plainly.

3. **Do not expand scope.**
   - No unrelated cleanup, renames, formatting sweeps, speculative refactors, or architecture detours.
   - If broader changes appear necessary, stop and explain why.

4. **Do not write outside the granted surface.**
   - Stay inside the repo, workdir, and file scope named by the handoff.
   - Do not patch external governance, release, infra, or publication surfaces unless the brief explicitly authorizes that write path.

5. **Fix the source, not the symptom.**
   - Do not rely on downstream CI, human review, or later cleanup to catch something you already know is wrong.
   - Correct the source artifact, test, config, doc, or implementation directly.

6. **No stubs in shipping code. Wired Integration doctrine applies.**
   - Code paths that ship must actually do what they claim to do. No empty event handlers on user-facing affordances, no "would have done" return strings, no fixture-only data in production paths, no `success: true` from operations that did not actually do the work.
   - Full doctrine: `./docs/governance/no-stub-mvp-doctrine.md`. Companion skill: `wired-integration-discipline`.
   - Stubs are the exception, not the rule. Operator-granted exceptions live in `./docs/governance/wired-integration-stubs-registry.md`.
   - Every code-bearing cycle runs the four-check audit defined in the skill before marking `complete`. Audit output is captured in the cycle receipt at `programs/codex/requirements/SD-N-<slug>/artifacts/<epic>/<cycle>_cycle_receipt.md`.
   - The doctrine applies to every SD-N bundle launching on or after 2026-07-20. Earlier bundles' stubs are remediated in their next bundle's Wired Integration Cleanup epic.

## Read Discipline

Keep context lean. Read additional material only when the task or handoff requires it.

- Read the execution handoff before reading broad repo documentation.
- Read only the files, docs, tests, contracts, or runbooks explicitly cited by the handoff or clearly required by the change.
- Before architecture-sensitive changes, read the cited architecture docs, ADRs, or interface contracts.
- Before CI/CD, release, deployment, migration, or operational changes, read the cited runbooks, pipeline docs, environment rules, and verification procedures.
- Before compatibility or migration work, read the cited oracle, reference corpus, legacy behavior notes, or acceptance comparisons.
- Do not load large documentation trees when a cited index or narrow leaf doc will answer the question.

## Role Boundaries

- The handoff defines **what** to do.
- This file defines **how** to behave while doing it.
- Repo-local code and tests define implementation truth.
- Upstream planning artifacts define intent and constraints, not permission to improvise beyond the bounded run.

When asked to implement, implement. When asked to plan, plan. Do not blur the lanes.

## Delivery Format

When finishing a task, provide a concise factual report:

- files changed
- tests added or updated
- commands run
- verification results
- blockers or unresolved questions

## Hard Stop Conditions

Stop and report the blocker instead of guessing when any of the following is true:

- required handoff fields are missing
- the correct write scope is ambiguous
- the task would violate TDD or required review gates
- the requested side effects exceed the granted authority surface
- verification failed
- required authoritative inputs are missing

## Retrospective Logging

When you catch an error, hit an incident, defer work, or redo something, emit a one-line event to the retrospective log via `scripts/retro.py`. The log is append-only and survives the run — git does not.

- **Correction:** `scripts/retro.py correction --subject <who-was-wrong> --claimed <claimed-value> --actual <real-value> --verified-by <command-or-check>`. The `--verified-by` field is required; an unverified correction is just a competing assertion.
- **Incident, deferral, rework:** use the corresponding type (`incident`, `deferral`, `rework`) — run `python3 scripts/retro.py help <type>` for required fields.
- **Reference:** `docs/retro/schema.json` (the contract); `docs/governance/book-ingestion-playbook.md` (per-book cycle procedure).

## Practical Default

Be conservative, exact, and auditable.

This repo rewards disciplined progress, not theatrical progress.
