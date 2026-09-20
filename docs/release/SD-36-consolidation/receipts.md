---
canonical: true
bundle_id: SD-36
---

# SD-36 Receipts

Closure-gate receipt blocks and cycle receipts. Populated as the bundle runs.

---

## Architecture-truth-up receipt

(Populated at Epic D step 2)

```yaml
architecture_truth_up:
  bundle: SD-36
  integration_target: develop
  documents_touched: []
  exit_code: null
  timestamp: null
```

---

## Graphify-update receipt

(Populated at Epic D step 3)

```yaml
graphify_update:
  bundle: SD-36
  integration_target: develop
  exit_code: null
  timestamp: null
```

---

## Epic A evidence (criteria A5, A10 — 2026-09-19)

### A5 — `CARGO_MANIFEST_DIR` path rewrites

`git grep -n 'CARGO_MANIFEST_DIR' -- crates` finds exactly one live, non-comment
use: `crates/codex-ingest/src/lib.rs`'s own `pub fn repo_root()`, whose entire
job IS resolving `CARGO_MANIFEST_DIR` (this crate's own manifest dir, two
levels under the repo root, joined with `../..`) into the real repo root
for every other caller in the crate to use instead of a bare
`env!("CARGO_MANIFEST_DIR")` or a `"."`/relative-path fallback. The remaining
hits are doc comments explaining that resolver's own provenance (`bonus_stack_reader.rs`,
`corpus_trap_baseline.rs`, `parity_report.rs`, `pcgen_runner.rs`, two test
files) — not further definitions. This is the single-definition shape the
criterion intends: one canonical resolver, every caller routed through it.

`git grep -c 'crate::pcgen_import' -- crates/codex-ingest` shows in-crate
callers already spelled `crate::pcgen_import::…` (the crate's own internal
path), never `codex::pcgen_import` or `codex_ingest::pcgen_import` from
inside the crate itself — correct for code living inside the crate.

This cycle's own contribution (previously 18 uncommitted files, reviewed and
correctness-checked): 8 `crates/codex-ingest/src/bin/*.rs` tool bins and 6
test files that read `data/corpus` via a bare relative path or a
`CODEX_REPO_ROOT`/`"."` fallback (both silently wrong once the crate's
manifest dir stopped being the repo root) now resolve it via
`codex_ingest::repo_root()` / `crate::repo_root()`. Verified line-by-line;
no behavior change beyond the path source.

### A2 — residue gate `--check --closure` does NOT exit zero (open gap)

`grep 'lst_file' scripts/pcgen_residue_gate.py` and the identifier-scan part
of the acceptance command both hold: `lst_file files=0 hits=0` and
`codex_ingest files=0 hits=0` — Epic A's own code-wall patterns are clean.
But `python3 scripts/pcgen_residue_gate.py --check --closure` itself exits 1
(`verdict=FAIL`, `live_files=49885 live_hits=181711`), because the gate's
shipped-data class (ruling B17) folds in every file the Tauri bundle ships,
and `apps/desktop/src-tauri/tauri.conf.json` now bundles
`"../../../data/corpus/": "data/corpus/"` whole (this tranche's separate
desktop-corpus-root commit, `217f712bab`, explicitly out of this cycle's
write scope to undo). The recorded ratchet baseline
(`scripts/pcgen-residue-baseline.env`, `files=260 hits=12736`, 2026-09-07)
predates that bundling change by 10 days, so even plain `--check` (not just
`--closure`) now fails too (`verdict=FAIL_INCREASED`). Logged as
`scripts/retro.py correction` (`docs/retro/events/sd31-transcribe.jsonl`).
**Not fixed here** — resolving it means either the corpus ships redacted
(provenance fields stripped before bundling) or the residue gate's
shipped-data scan is deliberately re-scoped for a corpus that is meant to
ship, both of which are decisions past Epic A's "PCGen crate wall" scope.

### A10 — `codex-ingest` baselines (first recording)

See `scripts/verify-baselines.env`'s SD-36 Epic A section for the full
derivation. Headline: `BASELINE_INGEST_FULL_TESTS=1673`,
`BASELINE_INGEST_TEST_BINARIES=157`, `BASELINE_CLIPPY_WARNINGS_INGEST=0`,
measured from `cargo test --locked --no-fail-fast -p codex-ingest -j 6` and
`cargo clippy --locked --tests -j 6` (in `crates/codex-ingest`). The test run
is NOT clean: 6 suites each fail exactly 1 test, all six panicking inside
`pcgen-run-character.sh` with an identical `JAVA_HOME is set to an invalid
directory` error — `~/.sdkman/candidates/java/current` is an empty directory
on this box, not a symlink to a real JDK. Verified as a pre-existing host
condition, not an Epic A regression: `/usr/bin/java` works, the failure is
identical regardless of which crate hosts the file, and none of the six
files' content changed beyond the path move. Logged as
`scripts/retro.py incident` (`docs/retro/events/sd31-transcribe.jsonl`,
`recurrence_key=sdkman-java-current-broken`). Net effect: `verify.sh`'s
`ingest-full` stage will report FAIL (cargo exit 101) on this box until
either the box's `JAVA_HOME` is fixed, or these 6 tests gain the same
checkout-unavailable-skip pattern `sd27_feat_prerequisite_enforcement.rs`
already uses for `PCGEN_CORPUS_ROOT`. Neither fix is in this cycle's write
scope (`~/.sdkman` is outside the repo; the skip-pattern rewrite is new
production-test-logic scope beyond the A5/A9/A10 batch this cycle inherited).

---

## Epic C2 evidence (criterion C2.5 — 2026-09-20)

### C2.5 — Oracle tests kept, run once against the real PCGen corpus

Nothing under `#[ignore]` was deleted or edited by this cycle: `git grep -c
'^#\[ignore\]\|    #\[ignore\]' -- tests` shows 21 hits across 20 files
(`tests/sd22_*_resolves.rs`), and the same pattern over
`crates/codex-ingest/tests` shows 31 hits across 6 files (a 7th match,
`pcgen_runner_smoke.rs`, only mentions `` `#[ignore]` `` in a doc comment —
it has zero real `#[ignore]` attributes, confirmed by `grep -c '#\[ignore\]'
crates/codex-ingest/tests/pcgen_runner_smoke.rs` = 0).

Both sets were run once, for real, against the pinned PCGen checkout's data
directory (`PCGEN_CORPUS_ROOT=$HOME/workspace/repos/pcgen/data`, the same
default `tests/support/paths.rs`'s `pcgen_data_root()` resolves to):

```
PCGEN_CORPUS_ROOT=$HOME/workspace/repos/pcgen/data cargo test --locked \
  --no-fail-fast --test sd22_acg_class_hunter_resolves \
  --test sd22_apg_class_inquisitor_resolves --test sd22_acg_class_warpriest_resolves \
  --test sd22_apg_class_oracle_resolves --test sd22_acg_class_skald_resolves \
  --test sd22_acg_class_shaman_resolves --test sd22_acg_class_arcanist_resolves \
  --test sd22_apg_class_summoner_resolves --test sd22_acg_class_bloodrager_resolves \
  --test sd22_acg_class_brawler_resolves --test sd22_acg_class_swashbuckler_resolves \
  --test sd22_apg_class_alchemist_resolves --test sd22_acg_class_investigator_resolves \
  --test sd22_acg_spell_list_resolves --test sd22_apg_spell_list_resolves \
  --test sd22_apg_equipment_resolves --test sd22_apg_class_witch_resolves \
  --test sd22_acg_class_slayer_resolves --test sd22_apg_class_cavalier_resolves \
  --test sd22_acg_equipment_resolves -- --ignored --test-threads=2
```

Result: **21 passed, 0 failed** (root `tests/`, one file —
`sd22_acg_class_warpriest_resolves.rs` — carries 2 `#[ignore]` tests, all
others 1 each). Same shape for `crates/codex-ingest`:

```
PCGEN_CORPUS_ROOT=$HOME/workspace/repos/pcgen/data cargo test --locked \
  --no-fail-fast -p codex-ingest --test sd17_a_include_graph \
  --test sd17_b_spellcasting_class --test sd31_e2_ground_truth_agreement \
  --test sd27_feat_prerequisite_enforcement --test pcgen_runner_smoke \
  --test sd17_b_monster_stat_block --test sd17_b1_martial_class -- \
  --ignored --test-threads=2
```

Result: **31 passed, 0 failed** (`sd17_a_include_graph` 1,
`sd17_b1_martial_class` 5, `sd17_b_monster_stat_block` 7,
`sd17_b_spellcasting_class` 14, `sd27_feat_prerequisite_enforcement` 3,
`sd31_e2_ground_truth_agreement` 1; `pcgen_runner_smoke` has none, all its
tests are unconditional). Total: **52 oracle/grounding tests kept, run once
against the real PCGen corpus this cycle, 52/52 green** — the JAVA_HOME
hazard recorded in A10 above did not recur (`java -version` on this box now
resolves through `~/.sdkman/candidates/java/current` to Temurin 25).

---

## Cycle receipts

| Cycle | Epic | Status | Baseline command | Output |
|---|---|---|---|---|
| — | — | — | — | — |

---

## Closure block (Epic D, steps 2–5)

(Populated at closure. Records: architecture-truth-up, graphify, merge-conflict resolution, if any.)

| Step | Status | Notes |
|---|---|---|
| Acceptance criteria 100% | awaiting | all epics → complete |
| Retrospective written | awaiting | Epic D step 1 (workflow-instruction §11) |
| Worktree sweep | awaiting | Epic D step 1 (workflow-instruction §11) |
| Architecture docs updated | done | D1 (2026-09-20 full-set rewrite) |
| Graphify run | done (graphify exit=1, receipt filed) | Step 3 receipt above; operator to decide retry-vs-proceed per script's own non-refusal policy |
| PR open and merged | pending | Step 5/6 — PR to be opened; operator merges |

---


- cycle_id: 2026-09-20T10:27:50Z
  row_or_kind: graphify:update
  bundle: SD-36
  branch: 45ef7e232755b457d3c6f0afbbbf8c812eaaa875
  integration_target: develop
  branch_tip: 45ef7e23
  graphify_exit_code: 1
  outcome: failed
  wall_clock_seconds: 1365.9
  log_path: graphify-out/.truth-up-run-2026-09-20T10:27:50Z.log
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: graphify exited 1; operator to decide retry-vs-proceed (see log)

---

## Epic D6 — worktree/branch inventory (read-only, 2026-09-20)

`git worktree list`:

```
/home/ubuntu/workspace/repos/codex                45ef7e2327 [tranche/16]
/home/ubuntu/workspace/worktrees/codex-ci-oracle  29cbe1fa2a [fix/ci-fetch-pcgen-oracle]
```

`git branch -a`:

```
  develop
+ fix/ci-fetch-pcgen-oracle
  fix/pcgen-pinned-tree-ci-guard
  sd36/package
* tranche/16
  remotes/origin/HEAD -> origin/develop
  remotes/origin/develop
  remotes/origin/fix/ci-fetch-pcgen-oracle
  remotes/origin/fix/pcgen-pinned-tree-ci-guard
  remotes/origin/main
  remotes/origin/sd36/package
  remotes/origin/test
  remotes/origin/tranche/16
  remotes/origin/update-index
```

`tranche/15` is already gone (no local or remote ref) — confirmed closed per prior
SD-35 record. POST-MERGE cleanup for the operator (nothing deleted here):

- Worktree `~/workspace/worktrees/codex-ci-oracle` (branch `fix/ci-fetch-pcgen-oracle`) —
  stale lane; sweep after tranche/16 merges if the branch is confirmed superseded.
- Local branch `fix/ci-fetch-pcgen-oracle` — mirrors the worktree above.
- Local branch `fix/pcgen-pinned-tree-ci-guard` — no active worktree; verify merged-by-content
  (not commit count, per standing convention) before deleting.
- Local branch `sd36/package` — SD-36 packaging scratch branch; verify superseded by
  `tranche/16` before deleting.
- `test` and `update-index` remote branches are infra (self-healing release gate,
  updater feed) — never delete per standing convention.

Nothing above was deleted; this is inventory only, for the operator to action after merge.
