---
canonical: true
purpose: What a fresh session needs to pick up this work. Written 2026-08-22 from the SD-31 session
  before it is retired, because that session holds context that is true but not written down anywhere.
read_first: true
---

# SD-32 handoff — read this before anything else

The SD-31 session ran for days across 31 waves and is being retired for stability. This document is
what it knew that the package docs do not say plainly.

## Where things stand

```
board        13,458 / 38,372 = 35.07%   (doneness_verdict replay over docs/work-inventory.json,
                                          EXCLUDED_BOOKS={'beginner_box'})
branch       tranche/11                 (tranche/12 to be cut after SD-31 closes)
not-done     24,914 — all 46 groups, uncovered = 0, see SD-31/artifacts/THE-BOX.md
```

**Never trust a board figure from a document.** Re-derive it. Several waves were planned against
stale or drifted numbers, and THE-BOX had to correct the orchestrator's own dispatch figures.

## The five things that will bite a fresh session

**1. Worktrees are cut from the wrong base.** Every wave since 15 has had lanes land on a
site-publish commit with no `docs/`, `data/`, `scripts/` or `schemas/` tree. Pin the base SHA in
every dispatch and tell lanes to verify and `git reset --hard` it if wrong. Delete spent
`site-publish/*` branches — they are what poisons the base.

**2. `find -newermt` lies on this box.** Agent-file mtimes run slightly ahead of system time, so it
reports zero for a file written seconds ago. Use a Python mtime comparison. Trusting it would
dispatch a second wave onto the same branch — two writers, the worst failure available here.

**3. Omitting `model` on an `agent()` call inherits the orchestrator's model.** It does not default
to Sonnet. One wave ran six Opus build lanes this way at 97% weekly quota. Set it explicitly every
time: Sonnet for build and integration, Opus only for adversarial verifiers.

**4. Never `git stash` in this repo.** The bare form stashes the whole shared checkout even from a
subdirectory. It has bitten this program three times.

**5. A ruling is not in force until it is committed.** A wave was dispatched telling lanes to read an
operator ruling that existed only in the orchestrator's working tree. The lane checked every ref,
found nothing, and correctly refused to reverse a pinned safety rule on a prompt's authority.

## The orchestrator failure mode, stated plainly

**Four times, a wave completed, a summary got written, and the turn ended without dispatching the
next one.** Work stopped entirely until the operator noticed. The summary *feels* like the
deliverable. It is not.

The fix that works is ordering: **dispatch first, report second**, so the report describes something
that already exists. A 3-hourly nudge cron exists as a backstop but dies with the session — recreate
it, and do not rely on it to make progress happen.

## Two theses that were refuted, so they are not retried

* **Bulk ingest** (wave 19). "not-ingested" does **not** mean the text is missing — it means the
  engine emits no explanation naming the record. The records already exist with real prose.
* **Generic roster without grant data** (wave 20). The emission loop is generic; the data it needs
  was not. Returned GAMED.

## What is genuinely built and trustworthy

* **The formula interpreter** — semantics derived per claim from PCGen's Java source, reproduces
  22 of 22 hand-modelled functions across 7,040 comparisons, zero disagreements. Reads 84% of corpus
  arithmetic and refuses the other 16% by name. Authorised by operator ruling §20, which overturned
  SD-27 §24.1's "no formula interpreter", **on the explicit condition that every interpreted value
  clears `derived_evaluator_fixture_check`**.
* **`scripts/coverage_ledger.py`** — proves inventory completeness mechanically. Fails closed on an
  empty predicate so a placeholder group cannot manufacture false 100% coverage.
* **The grant-fact parser** and its merged data, which refuses rather than defaults.

## The apparatus that makes the numbers believable

Four GAMED verdicts across waves 18–27, every one correct. Integration cycles have caught a
load-bearing defect in **every wave since 18** by re-deriving rather than trusting a lane or its
reviewer — a false root-cause claim, a half-unguarded fix, on-screen evidence filed as "up to 471"
when the real number was zero.

**Do not weaken this to go faster.** It is the reason 35.07% means something. The operator has said
plainly that a fast wrong number is worth less than a slow right one.

## Immediately actionable, already measured

* `scripts/derive_derived_evaluator_fixtures.py` was destroying 2,110 fixture entries per run and is
  now fixed — but roughly **30 Rust generators have never been checked for the same shape**. Some
  write `data/corpus`; revert cleanly between runs.
* **Ninja and Samurai** have complete, tested, dispatched chassis blocked by **one missing
  weapon-proficiency table row**. Two units, one row. Sweep S9 asks how many others are one row from
  working — nobody has answered.
* **612 units** move out of `unmeasurable` by wiring an existing matcher into one more call site.

## Live operator rulings

`SD-31/artifacts/OPERATOR-RULINGS-2026-08-19.md` and `-2026-08-21.md`. The load-bearing ones: §16
(delete Core Essentials residuals not in print — executed, label reached zero), §18 (option pools
show only valid choices), §19 (cross-book reprints settled by §10), §20 (the interpreter overturn).
