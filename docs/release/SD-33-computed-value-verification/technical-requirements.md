---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
---

# SD-33 Technical Requirements

## 1. Pre-launch prerequisites (all must hold before Epic 1 cycle 1)

| # | Requirement | State at authoring | Check |
|---|---|---|---|
| 1 | SD-32's closure PR merged to `develop` | ⛔ **OPEN** — `origin/develop` HEAD is `1bb523773d` (PR #374, `tranche/11`) | `git log origin/develop --oneline \| head -3` |
| 2 | SD-32's instrument debt closed **inside SD-32** | ⛔ **OPEN** — three named items | `../../retro/sd32-compute-library-and-cause-closure-retrospective.md` findings 1 and 3 |
| 3 | `tranche/13` cut from `develop` and pushed | ⛔ **OPEN** | `git ls-remote --heads origin tranche/13` → 1 line |
| 4 | Working tree clean on `tranche/13` | pending #3 | `git status --porcelain \| wc -l` → 0 |
| 5 | Oracle pin readable | ✅ met | `grep -E "^[A-Z_]+=" scripts/pcgen-oracle-pin.env` |
| 6 | Repo-local oracle slot populated | ✅ met | slot at `SD-32-.../artifacts/corpus/operator-supplied/pcgen` |
| 7 | JDK present for Epic 2's spike | ✅ met — OpenJDK 25 (Temurin) | `java -version` |
| 8 | Per-epic artifact directories exist | ✅ met | `artifacts/README.md` |

**Requirement 2 is not importable.** SD-32's open items close in SD-32 (`decisions.md §6`). Moving them here would be the laundering `../../governance/blocker-closure-doctrine.md` removes.

## 2. Normative requirements

### R1 — The box partitions the **full** inventory

`THE-BOX.md` covers all 49,438 units, not the not-done subset. **This is a deliberate widening from SD-32's instrument**, whose `not_done_population()` drops 15,022 already-`done` units and 19 `EXCLUDED_BOOKS` units. Those 15,041 are exactly the units no cycle re-examines; a box that inherits the same exclusion inherits the same blind spot.

### R2 — Verification outcomes are three-valued

`agree` | `disagree` | `unverifiable`. **`unverifiable` is never silently promoted to `agree`,** and never folded into `done`. An error in the harness surfaces as `unverifiable`, never as agreement.

### R3 — No hardcoded exclusion list may gate a closure figure

Any book/kind exclusion consumed by a closure-figure computation is declared in a **committed, human-readable registry** with a reason admissible under `decisions.md §6` — source data absent, or licensing forbids shipping. Cost, awkwardness, novel shape, and "no consumer reaches it" are **not** admissible.

**Corollary:** "not applicable to the modelled campaign set" is a **reachability** statement, not an **ingest or verification** statement. Reachability zero with real verified records is a correct outcome.

### R4 — Every figure carries its command and its denominator

Enforced by `scripts/verify.sh --only denominator-gate` (AT-33-E1-004) and by §7's receipt schema. A figure derived from the pinned corpus also carries `PCGEN_ORACLE_SHA`.

### R5 — Product Identity discipline is unchanged and still binding

- Never write a blacklist term or PI item name into a receipt, test name, test constant, kanban row, or commit message — **coordinates only**.
- A name-PI unit's neutral name derives **only** from `(kind, book, source_file, source_line)` — never from the PI name, not even hashed.
- **A `BONUS:`/`DEFINE:` value is a game rule, not Product Identity.** Never redact one.
- Known false positive: `Nex` inside `next`.

### R6 — Corpus write discipline is unchanged

- **Never hand-edit `data/corpus/**`** — guarded generator path only.
- **Never `--allow-stamp-loss`.** Stamp preservation is proved **by ID, not by count**.
- `git status --porcelain` before **every** git write. Never `git add -A`. **Never `git stash` in this repo.**

### R7 — Test-count changes require a sweep, not just a build

A record-count change compiles clean and still leaves other files' hardcoded assertions red. Grep both the old and the new count across `tests/`, `src/`, and `apps/` before committing.

## 3. Explicitly out of scope

- **Building a second PCGen-format reader** (Starfinder et al.) — `SD-32-.../forward-scope-register.md` C2.1.
- **Prose-sourced systems** — C3.1; a research question, not an engineering one.
- **Re-opening SD-32's closed `no_record` population.** SD-33 consumes it as given. If Epic 1's box finds a discrepancy, that is a **finding with a count**, escalated — not a silent re-ingest.
