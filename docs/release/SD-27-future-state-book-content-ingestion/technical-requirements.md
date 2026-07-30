# SD-27 — Technical Requirements

## 1. Pre-loop prerequisites

The bundle's Epic 2.1+ cannot dispatch until:

1. **SD-26 closure PR has landed on develop.** SD-26's `tranche/5-4 → develop` PR (#338) is the predecessor's gate. SD-27's cycle 2.1 reads the canonical Shape B schema authority surface to verify the per-book cycle's cache matches the established schema. If the schema authority does not exist, the cycle blocks.

2. **The bundle label has been resolved.** Cycle 2.0 is the operator's lever-pull; cycle 2.1+ blocks on 2.0's completion.

3. **The dispatcher's tier model has been authorized.** Per Cycle 2.1+'s per-cycle tier, the operator authorizes Sonnet (default) or a free/discounted model. Theorchestrator logs the tier per cycle.

4. **The per-book tier (Tier-1 / Tier-2) has been scheduled.** Tier-1 (13 books) launches first; Tier-2 (6 books) launches after Tier-1 reaches the parity-baseline phase. The orchestrator's cycle picker reads the per-book tier from this scope-draft.

5. **The workspace-side bundle author at `programs/codex/requirements/SD-27-future-state-book-content-ingestion/` has been promoted to `docs/release/SD-27-future-state-book-content-ingestion/`** via the `release-package-promotion` skill. This is the canonical (repo-resident) home.

## 2. Normative requirements

### 2.1 Per-cycle procedure

Every cycle follows the canonical six-section shape, recorded in `artifacts/epic_<n>/<cycle>_receipt.md`:

1. **Cycle header** — `Cycle ID`, `Criterion`, `Owner`, `Status`, `Route class`, `Started at`, `Completed at`.
2. **Inputs** — exact file paths consulted, exact prior cycle outputs.
3. **Outputs** — exact files created/modified, exact lines added, exact commits.
4. **Operations** — RED → GREEN → REFACTOR walkthrough, dual-audit gate result.
5. **Verification** — exact commands run, exact pass/fail counts, exact receipts.
6. **Notes** — judgment calls, deferred items, audit-exclusion requests.

The per-book cycle mirror SD-26's E4 receipt shape verbatim. The dispatch instruction pack is the SD-26 receipt with `{book_id}` substituted.

### 2.2 Dual-audit gate

Every cycle must pass the dual-audit gate before marking complete:

- **Identifier-discipline audit:** `bash scripts/identifier-discipline-audit.sh` (or equivalent). The audit checks for `sd<N>_*` / `SD<N>_*` / `Sd<N>*` patterns in source files, `t_<hex>` kanban tokens, `SD-N-Ex...` audit-IDs, "Tranche N chassis lane" string literals. Per `docs/governance/identifier-discipline.md`. Per-cycle exclusion: `docs/release/SD-27-future-state-book-content-ingestion/` docs themselves may carry the `SD-27-` slug (the bundle's own naming convention).

- **Wired-integration four-check audit:** grep for forbidden tokens (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`), empty onClick handlers, mock-library leaks outside tests, "Would …" strings. Per `docs/governance/no-stub-mvp-doctrine.md`. SD-26's audit references are at `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_6/final-criterion-scan-cycle_receipt.md:596-611`.

### 2.3 PCGen parity baseline

The per-book parity baseline is the per-book `<id>_parity-cycle_receipt.md` recording the comparator's match/mismatch table. The cycle's assertion is "match rate at the time of cycle close" — the inherited CG-03 baseline is documented in the receipt, not chased.

### 2.4 File-touch partition enforcement

Every cycle must verify it touched only the partition-permitted file paths (§8 of `scope-draft.md`):

- **Audit command:** `git diff --name-only <branch-base>..HEAD` against the partition's allowed list.
- **Audit failure mode:** cycle returns to the operator with the breach file list.
- **Recovery:** the cycle reverts the breach commits and re-runs.

### 2.5 Per-book cycle's tier model

The per-book cycle (E2.1-2.2, E3.1-3.2) can run on a free or discounted model per `decisions.md §11`. The tier is recorded in the cycle's `Route class` field. The dual-audit gate (§2.2) is the load-bearing enforcement; the model cannot skip it.

### 2.6 Tier-1 / Tier-2 sequencing

The orchestrator's cycle picker reads the per-book tier from `scope-draft.md §3` and dispatches in this order:

- **Tier-1 (13 books):** 2.1-2.13 + 3.1-3.13 in parallel-after-2.0.
- **Tier-2 (6 books):** 2.14-2.19 + 3.14-3.19 in parallel-after-Tier-1 reaches parity-baseline phase.

The orchestrator's cycle picker does not block on Tier-1's full completion before starting Tier-2; Tier-2 starts as soon as Tier-1's parity-baseline cycles (3.1-3.13) are dispatched. This is a templated staggered-fanout, not a hard serialization.

## 3. Out-of-scope (carve-out)

Per `scope-draft.md §6` + `forward-scope-register.md` §"Class excluding":

- v0.6's class/race breadth (8 of 11 CRB classes).
- Equipment-attachment schema (v0.6 actively scoping).
- Feat-effects engine beyond 4-feat scope (v0.6 already closed items 17).
- Class-skill recognition beyond Fighter/Wizard/Rogue (v0.6's lane).
- Starting wealth for non-CRB classes (v0.6's lane).
- Companion / animal / familiar stat-block engine (not in scope).
- Parameterized feats (Skill Focus, Teamwork feats) (not in scope).
- Temporary HP / favored-class-bonus HP (not in scope at the per-book-content level).
- UI affordances (arcane-school selector, unequip button) (v0.6 backlog).
- Multiclass durability-level ordering (Shape B JSON cache does not encode level-ordering ambiguity).

## 4. Cross-reference

- `./scope-draft.md` — the committed scope.
- `./decisions.md` — decision record.
- `./technical-design.md` — architectural surface.
- `./epic-breakdown.md` — per-cycle stories.
- `./loop-instruction.md` — per-cycle procedure.
- `loop-instruction-template.md` (governance) — the canonical loop-instruction template.
- `wired-integration-discipline.md` (governance) — the four-check audit.
- `identifier-discipline.md` (governance) — the identifier-discipline audit.
- `release-package-promotion.md` (governance) — workspace-to-repo promotion.
- `dual-canonical-doctrine.md` (governance) — workspace-citation + repo-local canonical pattern.
- `../SD-26-ingest-strategy-and-rule-system-plumbing/loop-instruction.md` — predecessor's loop-instruction template.
- `../SD-26-ingest-strategy-and-rule-system-plumbing/technical-requirements.md` — predecessor's normative requirements.
