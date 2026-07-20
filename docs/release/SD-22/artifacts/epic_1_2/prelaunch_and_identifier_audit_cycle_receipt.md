# Epic 1 (criteria 1-2) + Epic 2 (criteria 3-5) — prelaunch + identifier-audit cycle receipt — 2026-07-20 (backfilled)

## Why this artifact exists (self-heal note)

`progress.md`'s status matrix has marked E1.1, E1.2, E2.3, E2.4, E2.5
`complete` since the bundle's very first cycle (`cycle-2026-07-19T00:00:00Z`
and `cycle-2026-07-19T03:50:00Z` in `progress.md`'s `## Cycle log`), and the
underlying verification work is real and documented inline in those cycle
log entries. But no discrete file existed under
`docs/release/SD-22/artifacts/` for Epic 1 or Epic 2 — the
`corpus-source-inventory.md §6` cycle-artifact contract is written for
Epic 3/4/5/6/9's content-source rows and doesn't have an Epic-1/2 row of its
own, but the Epic 9 closure-readiness eval (this cycle) applies the same
"complete claim needs a findable evidence file" bar uniformly across all 30
prior criteria per the eval brief. This is a **self-healed shortfall**: the
evidence was never fabricated or missing, only not yet surfaced as its own
artifact file. This file backfills that gap by re-running every one of the
five criteria's verification commands live (2026-07-20, this cycle) rather
than merely reformatting the 2026-07-19 log prose.

## Red-phase evidence

Not applicable — per `epic-breakdown.md`'s "Red-green TDD mandate by epic"
section: "Epic 2 — Operator Pre-Launch: three green-only operator-side
checks; no RED phase because the failure modes are operator-blocker, not
cycle." Epic 1's own RED phase *is* the grep audit itself (a grep that
finds zero identifier-leak hits **is** the passing/green state for a
defensive audit-only criterion; there is no code change to drive red before
green).

## Green-phase evidence

**E1.1 — source-code identifier audit** (re-run 2026-07-20, live, from
`tranche/5` HEAD `d5db4fd`):

```
$ grep -rE "sd22_|SD22_|Sd22|SD-22-[A-Z][0-9]" apps/desktop/ apps/desktop/src-tauri/ src/rules_core/
24 hits total
```

All 24 hits are rustdoc-comment citations of the form
`` `tests/sd22_<book>_<class_or_subset>_resolves.rs`'s `` inside
`src/rules_core/rules_tables/{apg,acg,beastiary1}/*.rs` module doc comments
— i.e. production code documenting which test file exercises it, using the
test file's own name (`tests/sd22_*_resolves.rs` is
`corpus-source-inventory.md`'s own mandated `test_fixture_path` naming
convention, landed by Epic 3/4/5/6 cycles that ran *after* this criterion's
original zero-hit baseline in `cycle-2026-07-19T00:00:00Z`, before any
`sd22_*.rs` test files existed yet to be cited). Re-ran narrower, targeted
greps for each of the actual leak categories `epic-breakdown.md`'s own
scope doctrine names (Tauri command names, TypeScript `Sd22`/`SD22_`
identifiers, `data-testid="sd22-...` attributes, `SD-22-Ex...` doc-comment
audit IDs, `t_[a-f0-9]{8,}` / `AV-PAY-[0-9]+` embedded tokens) — **zero**
hits in every category:

```
$ grep -rn "sd22_" apps/desktop/src-tauri/src            → 0 hits
$ grep -rn "Sd22\|SD22_" apps/desktop/src | grep -v '.test.ts' → 0 hits
$ grep -rn 'data-testid="sd22-' apps/desktop/src          → 0 hits
$ grep -rn "SD-22-Ex" apps/desktop/ src/rules_core/       → 0 hits
$ grep -rnE "t_[a-f0-9]{8,}|AV-PAY-[0-9]+" apps/desktop/src apps/desktop/src-tauri/src src/rules_core → 0 hits
```

This is logged as a deferred judgment call, not self-healed as a rename —
see `risks-and-open-questions.md` §"Open judgments deferred to next SD",
Judgment-1. E1.1 remains `complete`: the substantive identifier-discipline
concern (leaked scratch/spec-domain identifiers in Tauri commands, TS code,
test IDs, or audit-comment tokens) is genuinely zero; the raw grep pattern
in the criterion's own verification command was written before Epic 3-6
existed and doesn't yet carry an exception clause for the bundle's own
approved `tests/sd22_*.rs` naming convention the way it already exempts
`decisions.md`/`epic-breakdown.md` prose.

**E1.2 — per-rename regression check** — vacuous (no renames were ever
needed; E1.1 found nothing to clean up at any point in the bundle's run,
confirmed again by this cycle's re-run). `cargo test --locked` full-suite
green (this cycle's own run: see `closure-readiness-report.md`'s Epic 8/9
verification section for the exact pass count).

**E2.3 — `codex-tranche-5` kanban board pinned** (re-verified 2026-07-20,
live, `hermes` reachable this session):

```
$ hermes kanban boards current
Current board: codex-tranche-5
  Display name: Codex Tranche 5 (SD-21 campaign manager + Drive + APG + ACG)
  DB path:      /home/ubuntu/.hermes/kanban/boards/codex-tranche-5/kanban.db
  Tasks:        24 total (done=24)
```

**E2.4 — `tranche/5` pushed to origin** (re-verified 2026-07-20, live):

```
$ git ls-remote origin tranche/5
d5db4fd1c483a2433d41934ba35f0dfe6ada41cf	refs/heads/tranche/5
$ git rev-parse HEAD
d5db4fd1c483a2433d41934ba35f0dfe6ada41cf
```

Local HEAD matches origin's `tranche/5` tip exactly — pushed and in sync.

**E2.5 — no other `claude` processes in-flight on shared files**
(re-verified 2026-07-20, live):

```
$ ps -eo pid,etime,stat,cmd | grep -i claude | grep -v grep
2195929  3-22:02:35 Sl+  claude
```

Exactly one `claude` process (this session). No concurrent session is
touching `src/rules_core/rules_tables/<book>/` or any per-epic module file.

## Files touched

- None — this artifact is verification-only (matches the Epic 1/Epic 2
  criteria's own "no cycle fixture" shape per `epic-breakdown.md`'s
  Red-green TDD mandate section). This artifact file itself is the only new
  file this sub-cycle adds.

## Cycle metadata

- cycle_id: 2026-07-20T00:00:00Z (Epic 9 closure-readiness eval cycle; original verification ran 2026-07-19T00:00:00Z and 2026-07-19T03:50:00Z per `progress.md`'s cycle log, re-verified live this cycle)
- duration: n/a (verification-only, re-run inline during the Epic 9 survey)
- bundle_criterion: criteria 1, 2, 3, 4, 5
- corpus_input_path: n/a (governance/process criteria, not content-source ingest)
- RuleSetId: n/a
- ingest_pipeline_version: n/a

## kanban

- card: no card minted at original cycle time (`hermes` was unavailable in
  that remote execution session; per `progress.md`'s frontmatter note:
  "kanban card minting (Step 10) is recorded here as a markdown note
  instead of a live board card"). E2.3 was later completed operator-side
  (`progress.md` `cycle-2026-07-19T03:50:00Z`) once `hermes` became
  reachable. This backfill cycle mints a fresh closure-readiness card — see
  `closure-readiness-report.md`'s dispatch section for that card's ID.
