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

7. **A proof is only as wide as the cases it covers.**
   - State explicitly which real shapes your correctness proof does **not** cover. A proof that passes and is too narrow is more dangerous than no proof, because it ends scrutiny.
   - Earned the hard way: a parser reproduced all 64 hand-curated reference records exactly **and** mutation-proved its own test could fail, and was still fabricating a wrong value for **73.4%** of its output — the reference set never exercised the shapes it got wrong.
   - When a ground-truth set exists, ask what it does not contain before trusting agreement with it.

8. **A warning is not a control.**
   - If a failure has recurred, do not write a caution and move on. Build the mechanism that makes it impossible, or say plainly that you are choosing not to.
   - Earned: `wrong-base-worktree` fired **27 times** despite a warning in every dispatch prompt; the real fix (deleting the branches that could be selected) was one line. `disk-full` fired **120 times** and was treated as a per-run chore for thirty cycles rather than an unbuilt control.
   - Recurrence is data. `scripts/retro.py summary` clusters it; a key firing more than a handful of times is a missing mechanism, not bad luck.

9. **Every figure you write down carries the command that produced it.**
   - A number in a brief, receipt, or doc must be accompanied by the command a reader can run to re-derive it. If it cannot be, mark it an estimate **in the text**.
   - Earned: of 608 recorded corrections, the most frequently wrong artifacts are our own briefs, dispatch prompts, issue rows and package docs — not code and not people. Prose drifts silently because nothing tests it, and a wrong figure propagates into every downstream reader.
   - Corollary: never state a derived figure as settled before the work that derives it has returned.

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

## Blocker Discipline

**A blocker standing between the work and 100% of its stated Definition of Done gets attacked until
it is cleared, or escalated to the operator. It never gets deferred.** Two dispositions only:

1. **Clear it.** Do the work. A blocker bigger than one cycle is a sequencing problem, not an
   exemption — decompose it and run the cycles.
2. **Raise your hand.** Escalate, naming what blocks you, what you already tried, and the specific
   ruling, write scope, or precondition you need. Then stop and wait. "This is hard" is not an
   escalation; "I need write scope to `<path>` to fix `<defect>`" is.

Filing a `## Open blockers` entry **is** the written form of disposition 2 — a request for an
operator ruling. It **pauses** the work; it is not a disposition, not a closure path, and never a
licence to proceed past the blocked item. "Filed with a named owner", "forwarded to a successor
bundle", "deferred with reason", and "out of scope for this cycle" are ways of writing down that
the work is not done.

A cycle that correctly refuses to write outside its granted scope has **not** failed — it has hit a
blocker only the operator can clear. Prepare the exact change, escalate, wait.

Full doctrine: `docs/governance/blocker-closure-doctrine.md`. Distinguish a blocker from a planned
*capability deferral* (`docs/governance/deferral-revisit-doctrine.md`) with one test: **was this
scope in the Definition of Done when the work was scoped?** If yes, it is a blocker.

## Retrospective Logging

When you catch an error, hit an incident, defer work, or redo something, emit a one-line event to the retrospective log via `scripts/retro.py`. The log is append-only and survives the run — git does not.

- **Correction:** `scripts/retro.py correction --subject <who-was-wrong> --claimed <claimed-value> --actual <real-value> --verified-by <command-or-check>`. The `--verified-by` field is required; an unverified correction is just a competing assertion.
- **Incident, deferral, rework:** use the corresponding type (`incident`, `deferral`, `rework`) — run `python3 scripts/retro.py help <type>` for required fields.
- **Reference:** `docs/retro/schema.json` (the contract); `docs/governance/book-ingestion-playbook.md` (per-book cycle procedure).

## Concurrency and Measurement

Derived from the tranche/7 retrospective (`docs/retro/tranche-7-retrospective.md` §6.1). Every rule
below is backed by recorded incidents, not by preference. Shared-tree collisions were the single
largest incident class of that tranche — 10 of 34 — and nothing caught any of them prospectively.

- **One writer per tree.** Two agents must never hold uncommitted work in the same working tree.
  Before your first write, run `git status --porcelain`; **if it lists a file you did not modify, stop
  and report** rather than proceeding. Concurrent agents get `git worktree add` *and* their own
  `CARGO_TARGET_DIR`.
- **Never `git stash` in a shared tree.** It is tree-wide and takes everyone's work. To read a HEAD
  baseline, use `git show HEAD:<file>` into a temp path, or a separate worktree.
- **`CARGO_TARGET_DIR` is one directory per agent *per source tree*, never per agent.** Sharing one
  between a worktree and the working tree makes cargo serve the wrong tree's artifacts — it produces a
  plausible wrong number rather than an error, and the one recorded instance was caught by luck.
- **Delete your `CARGO_TARGET_DIR` when you finish, and check disk before a full sweep.** A full sweep
  needs ~24 G. Never place one under `/tmp`. `ld terminated with signal 7 [Bus error]` and "couldn't
  create a temp dir" are disk exhaustion wearing a compiler bug's clothes.
- **A verification stage red for more than one run is a blocker, not a background condition.** Before
  excusing a failure as environmental, attribute **every** `test result: FAILED` line back to its
  `Running` line and name each suite. "The N known environmental failures" is a bucket, not an
  attribution — one such bucket concealed two never-executed parity gates for 36 hours.
- **Derive counts with `awk`, not `grep -o`.** Some harnesses shim `grep` to ugrep, whose `-o`
  silently drops matches on large files while `-c` and `-n` stay correct. Any number that moves a
  baseline needs two independent implementations agreeing.
- **Verify at the widest build scope the repo has.** `cargo build --lib` green is not a completed
  phase: `cargo test` builds bin targets, and one broken bin meant **0 of 502 suites ran** while the
  phase reported COMPLETE.
- **The PCGen oracle is pinned, never cited by literal local path.** `scripts/pcgen-oracle-pin.env`
  names the pinned commit; resolve the checkout via `$PCGEN_CORPUS_ROOT` (data) /
  `$PCGEN_REPO_DIR` (repo root), bootstrapped by `scripts/fetch-pcgen-oracle.sh` — never write
  `~/workspace/repos/pcgen` literally into new docs or scripts. `scripts/verify.sh` fails its
  `preflight-oracle` stage when the checkout is absent or off-pin.
- **A magnitude is not wired until it moves on the twin the player reads** (`docs/release/SD-27-future-state-book-content-ingestion/decisions.md §29.1`).
  Any surface that re-derives a rules number instead of rendering an engine `explanations` row is a
  candidate twin (`§29.2`).

### If you are dispatching work to other agents

- **A number in a brief ships with the command that produced it, or it does not ship** — not the value,
  the invocation. Dispatching briefs were the largest single source of corrected claims in tranche/7:
  41 of 115, and only 6 were caught before implementation began.
- **A ratio ships with its predicate.** "N of M carry X" is meaningless without the definition of X;
  one property on one unchanged tree read 23 → 32 → 46 → 49 → 51 → 52 in a single session, every step
  correctly verified.
- **`FILES YOU OWN` must be closed under the change it mandates.** Ask of each named fix: what else
  must change for this to reach a user? Command registration, DTO producers, second call sites.
- **Carry every environment rule, or none.** A guard that exists but is not named in the dispatch is a
  guard that does not exist.
- **Partition on observed concurrency, not on a stated premise.** Verify with `git status` and
  `git worktree list` before ruling that a tree has one writer.
- **Challenge the category, not just the count.** When a correction makes a number more precise without
  changing the frame, that is the moment to test the frame — two correct corrections once reinforced a
  false category for 25 more hours.
- **Re-read the brief against the repo before dispatching on it**, deriving each stated figure by the
  command that would produce it. This was the single missing occasion in tranche/7's shape.

## Practical Default

Be conservative, exact, and auditable.

This repo rewards disciplined progress, not theatrical progress.
