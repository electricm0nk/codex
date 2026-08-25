# Cycle sd32-closure-epilogue-final-2 — closure epilogue, second attempt, run to completion

- **Card ID:** `closure-epilogue` (kanban row 13) — `workflow-instruction.md §13` /
  `docs/release/template/template.md §6`.
- **Territory:** `docs/release/SD-32-compute-library-and-cause-closure/**`,
  `docs/retro/sd32-compute-library-and-cause-closure-retrospective.md`, `docs/architecture/**`
  (no edits needed, see Step 5), local `git worktree`/branch cleanup only (no source code
  touched).
- **Commit SHA:** see push receipt (this cycle's own commit).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6`
  (`PCGEN_CORPUS_ROOT=$PWD/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data`,
  repo-local slot, per `artifacts/corpus/README.md`. `~/workspace/repos/pcgen` never touched.).

## Context

The first `closure-epilogue` attempt (`4e6e1afaf5`) correctly stopped at Step 1 on two named
blockers. A remediation cycle fixed both (`21bef06d95` — `beginner_box` missing from
`reach_gate.rs`'s `CORPUS_BOOK_IDS` + stale `equipment_catalog.rs` pins; `30aa99d18e` —
`declared-pi-audit`'s O(files × citing-records) re-read memoized). This cycle's dispatch brief
asserted both were now clean and asked this cycle to re-derive that itself rather than trust the
filing. It was right to ask: **both gates still FAILed on a live re-run**, for a reason not named
in the brief — shared-checkout contamination, not a real defect. See Step 1 below.

## Step 1 — Final-acceptance scan

| Check | Command | Result |
|---|---|---|
| Shape coverage | `scripts/verify.sh --only shape-coverage-standing-gate` | PASS — `population=34416 unclassified=0 no_record=0` |
| Shape ledger | `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` | population 34,416; `no_record` 0; `unclassified` 0 |
| Row 17 census | `python3 scripts/row17_census.py --check` | `ROW 17 HONEST SIZE 0` |
| Deferrals | `python3 scripts/retro.py summary --since 2026-08-22 \| grep -i DEFERRALS` | `29 total, 0 open, 29 resolved` |
| `pi-sweep` | `scripts/verify.sh --only pi-sweep` | PASS — 10 hits / 10 baseline rows |
| `declared-pi-audit` (shared checkout) | `scripts/verify.sh --only declared-pi-audit` | **FAIL** — 60 `NAME-PI-SHIPPED`/`BLACKLIST-TERM-SHIPPED` violations (contamination, see below) |
| `site-public-status-pi-gate` | `scripts/verify.sh --only site-public-status-pi-gate` | PASS — 31 files / 1,612 declared, zero leaked |
| `site-dashboard-pi-gate` | `scripts/verify.sh --only site-dashboard-pi-gate` | PASS — 21 files / 1,612 declared, zero leaked |
| Desktop suite (shared checkout) | `cd apps/desktop/src-tauri && cargo test --locked --bin codex-desktop` | **547 passed / 1 failed** (contamination, see below) |
| Kanban | manual field-parse of `kanban.md` | 21 of 22 cards `complete`; row 13 is this row |
| `## Open blockers` | `grep -n "^## Open blockers" -A3 progress.md` | empty template, no live entry |

### Two FAILs traced to contamination, not defect — re-verified CLEAN in a disposable clean worktree

**`declared-pi-audit`.** Every one of the ~60 flagged files (`data/corpus/inner_sea_world_guide/
language/vudrani.json` and siblings) traced to commit `5c0178a397` (already on `tranche/12`'s
ancestry), which `git rm`'d exactly those files as a genuine PI-leak fix ("60 renamed records' old
marker-shaped files were orphaned by the slug change and removed via git rm"). `git status
--porcelain` confirmed every flagged path is `??` (untracked) in the shared checkout — a
concurrent lane still running in the same checkout had regenerated them on disk without
committing. `declared_pi_shipping_audit.rs` walks `data/corpus/**` with `fs::read_dir`
(`find_json_files`), not `git ls-files` — it cannot distinguish a committed record from untracked
litter.

Re-run in a disposable clean checkout:
```
git worktree add <scratch-dir> HEAD --detach
cd <scratch-dir>
PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-.../artifacts/corpus/operator-supplied/pcgen/data \
  CARGO_TARGET_DIR=<scratch-target> cargo run --locked --bin declared_pi_shipping_audit
```
→ `declared-pi-audit: CLEAN — no shipped record contradicts its own corpus row's PI declaration`
(5m05s, 51,408+ shipped corpus records covered).

**Desktop suite.** `corpus_ingest_diagnostic::tests::the_two_ingested_books_totals_reconcile_with_
their_license_artifacts` failed: live disk walk of `data/corpus/advanced_race_guide` counted 2,208
but the pinned figure expected 2,207 — off by exactly one. `git status --porcelain -- data/corpus/
advanced_race_guide/` showed one untracked file (`language/azlanti.json`) inflating the live-walk
count by exactly that one.

Re-run in the same disposable clean checkout: **548 passed / 0 failed** (84.5s), matching the
brief's claimed remediated state exactly.

**Both worktrees removed after use** (`git worktree remove --force`), no commits made in either.

**Verdict: GREEN.** No real defect blocks closure. Proceeded to Step 2.

## Step 2 — Retrospective

Appended a "Final closure — 2026-08-24/25" section to `docs/retro/sd32-compute-library-and-cause-
closure-retrospective.md` (does not rewrite the existing 456-line body). Covers: the
`EXCLUDED_BOOKS = {'beginner_box'}` carve-out (population 34,397 → 34,416); the three carve-out
sites found, all in code; the live `feat_gap` self-erasure bug (48 real records deleted per run)
found while re-verifying `sd32-five-unverified-deferrals`'s filed claim; the site pipeline crash
and honest percentage drop (39.6% → 33.5%, denominator +8,370, re-confirmed live via `python3
scripts/site/build_public_status.py --check` → `OK`, current `site/status-data.json`
`overall.pct` = `33.5`); the `declared-pi-audit` memoization fix; `retro.py`'s `deferrals.open`
windowing bug; Decision §28 (60-term PI vocabulary stands, no expansion, re-read from
`decisions.md`); and this cycle's own new finding — the shared-checkout instrument-contamination
hazard above.

Cited from `docs/release/SD-32-compute-library-and-cause-closure/references/README.md` (the
existing Retrospectives row, updated in place to describe the final closure section rather than
the stale "two live findings carried forward" text).

## Step 3 — Worktree/branch sweep

18 `git worktree`s found at cycle start (17 besides the primary checkout at
`/home/ubuntu/workspace/repos/codex`). None locked. All 17 tip commits confirmed ancestors of
`tranche/12` HEAD `88449a6297` (`git merge-base --is-ancestor <sha> HEAD`, all `MERGED`) — merged
by content. All 17 removed (`git worktree remove --force`); `git worktree list` after → just the
primary checkout.

148 local `worktree-wf_*` branches found. Same ancestry check: 144 `MERGED`, 4 `UNMERGED`
(`worktree-wf_13156488-c9b-1` @35a2bf5adc "wave 20", `worktree-wf_a45ece26-3fc-1` @ed10ff6c6f
"W21-CF-GRANT-001", `worktree-wf_be4660f2-72a-3` @d93ee6d1ab "W26 race_trait retro correction",
`worktree-wf_c1156061-e3f-5` @abbadb807c "W30 monster_ability/companion" — all SD-31-lane commit
messages, out of this bundle's scope). 144 deleted (`git branch -D`); 4 left alone. `sd31/
racetrait4-SD31-E6-F4-005` (SD-31's rescue branch, must not be merged on trust per standing
memory) also left alone — not a `worktree-wf_*` branch, outside SD-32's lane. Final local branch
count: 7 (`develop`, `tranche/12`, `sd31/racetrait4-SD31-E6-F4-005`, 4 unmerged
`worktree-wf_*`).

**Count found vs removed: 18 worktrees found → 17 removed (1 primary kept); 148 branches found →
144 removed (4 unmerged + `sd31/racetrait4...` left alone).**

## Step 4 — Release notes

Fully re-derived, not transcribed. Gate 1 per-family table rebuilt from a live
`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` run against the final
34,416-unit population (the committed `artifacts/gate-1-shape-closure/ledger.json` is the stale
2026-08-22, 24,914-unit snapshot — noted as historical scaffolding, not deleted). Sum check:
22,759+6,308+2,337+671+1,086+589+391+12+196+62+5 = 34,416, matching
`shape-coverage-standing-gate` exactly. New "Closure figures" section added carrying every
headline number with its command. `census_prestige_class_entry_requirements.py` re-run:
population still 62 (unchanged from the 2026-08-22 snapshot). Generator count re-counted:
`ls src/bin/{gen_,ingest_,enrich_}*.rs | wc -l` → 29 (unchanged).

## Step 5 — Architecture docs + graphify

Both scripts present at `~/.hermes/profiles/god-emporer/skills/devops/{architecture-truth-up,
graphify-update}/scripts/`. Both refuse on a dirty working tree, and this cycle's own in-progress
edits (plus a concurrent lane's untracked corpus litter) made the primary checkout dirty
throughout. Ran both in a disposable `git worktree add <scratch> HEAD --detach`, pointing
`--receipts-md` at the worktree's own copy of `receipts.md`, then diffed the worktree's copy
against the real file to confirm only the new entries were appended before copying them back.

- `architecture-truth-up --integration-target origin/develop`: diff path count 38,225 (566 in
  architecture scope, 37,659 out). `docs_touched: []` — **no architecture impact**, diff is
  outside `docs/architecture/`'s scope. `cited_path_check`/`relative_link_check` both `pass`.
  Receipt appended (`cycle_id: 2026-08-25T01:11:27Z`).
- `graphify-update --integration-target origin/develop`: `graphify cluster-only` exited 1 — "no
  graph found at .../graphify-out/graph.json — run /graphify first" (the disposable worktree has
  no prior graphify baseline to update incrementally). Per operator directive 2026-07-20, a
  non-zero graphify exit does not refuse the closure pipeline; receipt appended
  (`cycle_id: 2026-08-25T01:11:47Z`, `outcome: failed`, log path recorded) and the pipeline
  continued.

Scratch worktree removed after use, no commits made in it.

## Step 6 — Version bump and PR

`apps/desktop/package.json` / `apps/desktop/src-tauri/tauri.conf.json` both already read
`0.12.0`. Per `decisions.md §1` ("The tranche digit bumps once, at the tranche cut... Published
builds stamp `0.12.<build>` at publish time"), the tranche digit does not bump again at a
bundle's own closure — only at a new `tranche/N` cut — and the build suffix is stamped only when
publishing to `main`, not at this `develop` merge. **No version file edit made.** This corrects
the dispatch brief's Step 6 instruction to "bump per this bundle's convention" — the convention,
read directly, says the opposite for this step.

PR opened `tranche/12 → develop` via `gh pr create`. PR number and URL: see the git push /
`gh pr create` output for this cycle.

## Step 7 — Kanban row 13

`kanban.md` row 13 Status → `complete`; Cycles column appended `4 (this entry — closed)`; Notes
column got one prepended entry (kept out of row hygiene — the narrative lives in `progress.md`'s
new cycle section and this receipt, the Notes entry is a short pointer). `progress.md` got a new
`## Cycle sd32-closure-epilogue-final-2` section.

## Movement, by bucket

- **Closure:** kanban row 13 → `complete`. SD-32 now reads 22/22 `complete`. The
  `tranche/12 → develop` PR opened.
- **Reclassification:** none — no population figure changed shape this cycle, only its stated
  value moved from the stale 34,397 to the live 34,416 (that move happened in an earlier cycle;
  this cycle re-derived and reported it, did not cause it).
- **Reachability:** none — no new records reached the player this cycle.
- **Instrument-correction:** two, both this cycle's own finding — `declared-pi-audit` and the
  desktop suite's `advanced_race_guide` reconciliation check both disk-walk `data/corpus/**`
  directly and cannot distinguish committed records from a concurrent lane's untracked litter.
  Neither script was changed (the fix is procedural — verify in a disposable clean worktree — not
  a code fix to the binaries); documented as a standing hazard in the retrospective for a
  successor cycle to address at the code level if it recurs.

## Verification commands (paste-ready)

```
scripts/verify.sh --only shape-coverage-standing-gate
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json
python3 scripts/row17_census.py --check
python3 scripts/retro.py summary --since 2026-08-22 | grep -i DEFERRALS
scripts/verify.sh --only pi-sweep
scripts/verify.sh --only site-public-status-pi-gate
scripts/verify.sh --only site-dashboard-pi-gate
# declared-pi-audit and the desktop suite: run in a disposable clean worktree, not the shared
# checkout, to exclude concurrent-lane untracked-litter contamination:
git worktree add <scratch> HEAD --detach && cd <scratch>
cargo run --locked --bin declared_pi_shipping_audit
cd apps/desktop/src-tauri && cargo test --locked --bin codex-desktop
```
