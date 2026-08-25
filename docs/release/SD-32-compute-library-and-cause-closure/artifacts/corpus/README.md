---
canonical: true
owner: god-emporer
purpose: The repo-local home of every PCGen file SD-32 reads. Operator directive 2026-08-22 — "any and all references to pcgen files are made local to the repo in the artifacts folder." SD-22 precedent (docs/release/SD-22/artifacts/corpus/).
date: 2026-08-22
---

# SD-32 corpus slot — the PCGen oracle, repo-local

This directory is the **only** location this bundle knows for PCGen data. No SD-32 document,
dispatch prompt, cycle receipt, or verification command references `~/workspace/repos/pcgen` or any
path outside the repo.

## Layout

| Path | What | Ships to origin? |
|---|---|---|
| `README.md` (this file) | The slot's contract | yes |
| `operator-supplied/README.md` | How the slot is populated and checked | yes |
| `operator-supplied/pcgen/` | The PCGen oracle checkout at the pinned SHA (`scripts/pcgen-oracle-pin.env`, sparse cone `data/pathfinder system/gameModes/Pathfinder`, ~86 MB) | **no** — git-ignored by the repo rule `docs/release/SD-*/artifacts/corpus/operator-supplied/**` (Paizo/PCGen EULA; the oracle is a public repo that is never vendored) |

Bundle-produced evidence (census diffs, shape ledgers, per-engine fixture and expected-value files)
does **not** live here — it lives in the per-gate directories (`../gate-0-census-closure/`,
`../gate-1-shape-closure/`, `../gate-2-engines/`, `../gate-3-closure-invariant/`). This slot holds
oracle input only.

## How every cycle resolves it

Exported before any corpus command (`workflow-instruction.md §2.1`):

```bash
export PCGEN_REPO_DIR="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen"
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
scripts/verify.sh --only preflight-oracle >/dev/null || scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"
```

`scripts/fetch-pcgen-oracle.sh --dest` clones the sparse cone at `PCGEN_ORACLE_SHA` into the slot
and prints the two `export` lines; `scripts/verify.sh --only preflight-oracle` checks the slot is at
the pin (`--check`). In a fresh worktree the slot is empty (ignored files are not checked out) and
the fallback fetch repopulates it.

## Why here and not `~/workspace/repos/pcgen`

Same three reasons SD-22 recorded: (1) **self-sufficiency** — a coding harness reads only the repo;
(2) **per-bundle ownership** — SD-32's oracle belongs to SD-32; (3) **clear gitignore frontier** —
the licence boundary is enforced by exclusion, not by operator self-discipline.

## Verified 2026-08-22 (launch-readiness remediation)

```
scripts/fetch-pcgen-oracle.sh --dest <slot>/pcgen   → pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6
PCGEN_REPO_DIR=<slot>/pcgen PCGEN_CORPUS_ROOT=<slot>/pcgen/data scripts/verify.sh --only preflight-oracle → PASS
git status --porcelain | grep -c operator-supplied   → 0
```
