# SD-27 — Technical Requirements

## 1. Pre-loop prerequisites

The bundle's Epic 2.1+ cannot dispatch until:

1. **SD-26 closure PR has landed on develop.** — **SATISFIED as of 2026-07-27** (corrected same day — an earlier note here said PR #339; that was backwards). SD-26 merged via **PR #338** — `62e7b617` is a confirmed ancestor of `origin/develop`, and the SD-26 package + `src/bin/sd26_gen_core_rulebook_cache.rs` are both present there. PR #339 is a later, unrelated CG-03 bugfix. Caveat: SD-26's own `progress.md` on develop still shows its terminal `6.5` row as "awaiting operator merge" — stale paper-trail, not evidence against the merge. SD-27's cycle 2.1 still reads the canonical Shape B schema authority surface to verify the per-book cycle's cache matches the established schema; if that authority does not exist, the cycle blocks.

2. **The bundle label has been resolved.** Cycle 2.0 is the operator's lever-pull; cycle 2.1+ blocks on 2.0's completion.

3. **The dispatcher's tier model has been authorized.** Per Cycle 2.1+'s per-cycle tier, the operator authorizes Sonnet (default) or a free/discounted model. The orchestrator logs the tier per cycle.

4. **The 2 in-scope books are confirmed available.** Advanced Race Guide and Pathfinder Unchained both have a source LST corpus on disk (23 and 11 `.lst` files respectively, verified 2026-07-27) and registry entries `#0003` / `#0017`. `preflight` asserts all of this. The Tier-1 / Tier-2 fan-out across the other 17 books is SD-28+'s concern, not a SD-27 prerequisite.

5. **The bundle is repo-resident at `docs/release/SD-27-future-state-book-content-ingestion/`.** Promotion via the `release-package-promotion` skill has already happened; this is the canonical and only copy. There is no `programs/` tree in this repo (`decisions.md §6`).

6. **The reporting surface is reachable.** `python3 scripts/sd27-workflow.py preflight` exits 0. Per `loop-instruction.md §8`, an unreachable dashboard is a hard stop, not a warning.

## 2. Normative requirements

### 2.1 Per-cycle procedure

Every cycle follows the canonical six-section shape, recorded in `artifacts/epic_<n>/<cycle>-cycle_receipt.md`
(the `-cycle_receipt.md` suffix, per `artifacts/README.md` and every concrete filename in this bundle):

1. **Cycle header** — `Cycle ID`, `Criterion`, `Owner`, `Status`, `Route class`, `Started at`, `Completed at`.
2. **Inputs** — exact file paths consulted, exact prior cycle outputs.
3. **Outputs** — exact files created/modified, exact lines added, exact commits.
4. **Operations** — RED → GREEN → REFACTOR walkthrough, dual-audit gate result.
5. **Verification** — exact commands run, exact pass/fail counts, exact receipts.
6. **Notes** — judgment calls, deferred items, audit-exclusion requests.

The per-book cycles mirror SD-26's E4 receipt shape verbatim. The dispatch instruction pack is the SD-26 receipt with `{book_id}` substituted.

### 2.2 Dual-audit gate

Every cycle must pass the dual-audit gate before marking complete:

- **Identifier-discipline audit:** `bash scripts/identifier-discipline-audit.sh`, which prints `OK_NO_BUNDLE_TAGS` on a clean diff. The audit checks for `sd<N>_*` / `SD<N>_*` / `Sd<N>*` patterns and `t_<hex>` kanban tokens in shipping source. Per skill `identifier-discipline` (machine-local; the runnable gate is vendored into `scripts/`). **The audit is diff-scoped** — set `BASE_BRANCH` to the cycle's own base, not the long-lived integration branch, or it reports the whole branch's history. Tests and `docs/release/**` are excluded by construction, so the bundle's own `SD-27-` slug is never flagged.

- **Wired-integration four-check audit:** `bash scripts/wired-integration-audit.sh`, which prints `OK_NO_TOKENS` / `OK_NO_NOOP_HANDLERS` / `OK_NO_MOCK_LEAKS` / `OK_NO_WOULD_STRINGS` and then `AUDIT PASSED`. Checks forbidden tokens (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`), empty onClick handlers, mock-library leaks outside tests, and "Would …" strings. Per `docs/governance/no-stub-mvp-doctrine.md`. SD-26's own passing result is recorded at `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_6/final-criterion-scan-cycle_receipt.md:9-10` (that file is 55 lines; an earlier `:596-611` citation here was impossible).

### 2.3 PCGen parity baseline

The per-book parity baseline is the per-book `<id>_parity-cycle_receipt.md` recording the comparator's match/mismatch table. The cycle's assertion is "match rate at the time of cycle close" — the inherited CG-03 baseline is documented in the receipt, not chased.

### 2.4 File-touch partition enforcement

Every cycle must verify it touched only the partition-permitted file paths (`scope-draft.md §4`; `decisions.md §8` is the authority where they differ — §8 of scope-draft is *Hard-stop conditions*, not the partition):

- **Audit command:** `git diff --name-only <branch-base>..HEAD` against the partition's allowed list — the full pipeline, including cycle 2.0's exception, is in `loop-instruction.md §6`.
- **Audit failure mode:** cycle returns to the operator with the breach file list.
- **Recovery:** the cycle reverts the breach commits and re-runs.

### 2.5 Per-book cycle's tier model

The per-book cycle (E2.1-2.2, E3.1-3.2) can run on a free or discounted model per `decisions.md §11`. The tier is recorded in the cycle's `Route class` field. The dual-audit gate (§2.2) is the load-bearing enforcement; the model cannot skip it, and `complete` refuses a receipt that does not exist on disk.

### 2.6 Cycle sequencing

SD-27 dispatches **2 books**, so there is no Tier-1/Tier-2 stagger inside this bundle — that shape belongs to SD-28+, where the remaining 17 books fan out. Within SD-27:

- The 2 books (ARG, PU) are file-disjoint and run in parallel, serializing only on the shared stubs registry and `data/stubs/`.
- The 4 stages within a book are serial: `license → pre_build → verify → parity`.

Ordering is enforced mechanically by the `depends_on` chain in the reporting manifest, not by the picker's goodwill: `python3 scripts/sd27-workflow.py claim` refuses an out-of-order claim. See `loop-instruction.md §8`.

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
- `docs/governance/loop-instruction-template.md` — the canonical loop-instruction template.
- `docs/governance/no-stub-mvp-doctrine.md` — the wired-integration parent doctrine.
- `scripts/wired-integration-audit.sh` / `scripts/identifier-discipline-audit.sh` — the runnable dual-audit gate.
- `scripts/sd27-workflow.py` — dispatch-state driver + reporting writer.
- Machine-local skills (outside this repo, under `$HERMES_HOME/profiles/god-emporer/skills/`): `wired-integration-discipline`, `identifier-discipline`, `release-package-promotion`, `dual-canonical-doctrine`.
- `../SD-26-ingest-strategy-and-rule-system-plumbing/loop-instruction.md` — predecessor's loop-instruction template.
- `../SD-26-ingest-strategy-and-rule-system-plumbing/technical-requirements.md` — predecessor's normative requirements.
