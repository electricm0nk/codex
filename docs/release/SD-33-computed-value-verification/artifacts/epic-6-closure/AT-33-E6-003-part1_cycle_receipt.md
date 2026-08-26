# Cycle AT-33-E6-003 (part 1) — epic-6-closure / architecture docs, graphify, PR

- **Commit SHAs:** `93ccd564ab` (5 architecture docs), `58f0ba34a4` (architecture-truth-up receipt
  + correction), `a47bd3bb2e` (two doc fixups — unambiguous `decisions.md` cite, drop an ungrounded
  figure), `<this-receipt-commit-sha>` (this receipt + progress.md/kanban.md pointers; `graphify-out/`
  is gitignored, no commit for the graph itself).
- **Worktree:** clean `git worktree add -b sd33-r10-archdocs-pr` off `origin/tranche/13` =
  `d069d41806` (`AT-33-E6-002`'s own landing SHA), outside the shared checkout at
  `/home/ubuntu/workspace/repos/codex`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS` (one false-positive hit on "placeholder" —
  prose describing PCGen's own unsubstituted `OUTPUTNAME:` `[NAME]` token, not a code stub —
  reworded to avoid the collision anyway; re-scanned clean)
- **Acceptance criterion (`AT-33-E6-003` part 1, `epic-breakdown.md`):** architecture docs
  refreshed for what SD-33 actually changed; graphify run per template §6; PR opened
  `tranche/13 -> develop`, leading with defects found and fixed, citing the retrospective and
  receipts, stating headline figures with denominators, noting the 31 inherited failing suites so
  a reviewer is not surprised by a non-green workspace run.

## Environment finding — the shared checkout was not touched

`/home/ubuntu/workspace/repos/codex` was found **stale relative to `origin/tranche/13`** (local
`HEAD` = `06e858b0e6`, origin = `d069d41806`, 12 commits behind) with 159 `git status --porcelain`
entries this agent did not create (137 `data/corpus/**` modifications, 7 Epic-5-artifact
deletions, 15 untracked `.workflow.js` files) — the same recurring hazard `AT-33-E6-001-build-green`
and `AT-33-E6-002` each independently found and worked around. Per `AGENTS.md` "One writer per
tree", nothing was written there and nothing was discarded. All work below happened in a fresh
`git worktree add -b sd33-r10-archdocs-pr <path> origin/tranche/13` (checked out clean at
`d069d41806`), and every push used the standard fetch-rebase-push retry loop (`workflow-instruction.md §5`)
from that worktree.

## 1. Architecture docs — refreshed

`docs/architecture/` updated in place for current-state truth, no stub-graduation history
appended:

- **`homebrew-and-oracle.md`** — new §"The SD-33 corpus-wide oracle harness (`scripts/oracle_harness/`)":
  `compare.py`/`oracle_export.py`/`run.py` (the `(ours, oracle, verdict)` comparison surface feeding
  `box_ledger.py`), `campaign_key.py` (the `KEY:`-not-display-name campaign-load fix),
  `derive_spell_casting_ability_mapping.py` (real `SPELLSTAT:` derivation from the pinned oracle),
  `charbuild_remainder_generate.py` (the corrected L20-per-class/L1-per-race `.pcg` fixture
  template), and the per-type AC isolator (`src/bin/e5_ac_isolator.rs`, replacing the
  whole-character `AC.TOTAL` diff with PCGen's own `BONUS.COMBAT.AC.TOTAL.!BASE.!Ability.!Size`
  read).
- **`rules-engine.md`** — new §3c (`formula_interpreter_corpus_wide.rs`'s fresh-census fix, closing
  the frozen-file staleness that produced a false "6,854 of 11,652 never run (41%)" figure) and a
  new paragraph in the
  `equipment_effects.rs` catalog entry: the `WeaponEnhancementBonus` `tohit_bonus`/`damage_bonus`
  split (`heavy_hammer`), the `arms_armor.rs`/`general.rs` EQMOD-referenced-modifier resolvers, the
  `equipment_id_resolve` OUTPUTNAME fix (`corpus_loader.rs`'s synthetic `KEY` token), and the two
  new `EQMWEAPON|DAMAGESIZE`/`EQM|WEIGHTDIV` resolvers, with real corpus citations for each.
- **`corpus-ingest.md`** — new §"`raw_tokens` enrichment and the corpus-literal sweep's own closure
  builder": `enrich_equipment_raw_tokens.rs`'s book-agnostic `Value`-only enrichment (and the
  data-loss failure mode its typed-struct predecessor had), and `corpus_literal_sweep.rs`'s two
  real defects (unsorted book-wide `.COPY=` resolution; the `DESC`-exempt blacklist gap) the
  enrichment made visible and that were fixed in the sweep, not the data.
- **`testing.md`** — new §"SD-33 additions": `box_ledger.py`/`THE-BOX.md`'s structural-partition
  proof, the `denominator-gate`/`corpus-sweep` `verify.sh` stages, and the L20/L1 pilot-build `.pcg`
  fixture generator.
- **`status.md`** — new §"SD-33: `unknown` reaches zero, and `docs/work-inventory.json` grows to
  49,438 units": the 4,224→0 `unknown` movement (3,052 `not-ingested` + 854 `ingested-magnitude` +
  318 `unmeasurable`, exact, zero unaccounted), and the doneness-mapping widening
  (`(ambiguous, literal-/fixture-verified) -> held`) that reclassification required, plus the
  `AT-33-E6-001`-scan-caught lib-suite regression it briefly introduced and this bundle closed. The
  stale tranche/9-era `unknown` **3,547** headline is marked superseded in place with a pointer,
  not silently left to contradict the new section.

Each doc's `Last verified` header updated to `2026-08-25 against tranche/13`, scoped to exactly the
new section(s) — every other section's prior verification date is preserved untouched, per the
existing convention (`status.md`, `rules-engine.md` both already carry multiple such scoped
re-verification stamps).

### Verification one-liners (run by hand — see "Instrument finding" below for why)

```
$ grep -rhoE '`(src|apps|tests|scripts|tools|schemas|docs|\.github)/[^`]*`' docs/architecture/*.md \
    | tr -d '`' | sed 's/[:#].*$//' | grep -vE '[* <]' | sort -u \
    | while read p; do [ -e "$p" ] || echo "MISSING: $p"; done
(no output — every cited path exists)

$ grep -rhoE '\]\(\./[^)]+\.md' docs/architecture/*.md | sed 's/](\.\///' | sort -u \
    | while read l; do [ -f "docs/architecture/$l" ] || echo "BROKEN LINK: $l"; done
(no output — every relative link resolves)
```

Both PASS.

## Instrument finding — `architecture_truth_up.py`'s own doc-mapping regex is broken

`~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py::parse_source_dirs_index`
requires a literal doubled pipe (`||`) at both ends of a table row; `docs/architecture/README.md`'s
real index table uses a single `|` per normal Markdown convention. The regex therefore matches zero
rows on every invocation, `map_paths_to_docs` always returns `{}`, and the script reports
`docs touched: none — no architecture impact` regardless of what the diff actually changed —
confirmed by running it for real against this cycle's own 5-doc, 288-line diff (`93ccd564ab`) and
getting exactly that false-negative result. `branch_tip_before`/`branch_tip_after` are also wrong
whenever `--branch` is passed explicitly (8-char-truncates the literal branch-name string instead
of a resolved SHA). Filed per `AGENTS.md` rule 8 ("a warning is not a control") as a correction
entry in `receipts.md` immediately after the auto-appended receipt, and as a retro `incident`
(`docs/retro/events/sd33-r10-archdocs-pr.jsonl`, `--recurrence-key archtruthup-docs-touched-regex`)
— not fixed in-repo, since the script lives outside this bundle's write scope. The doc edits
themselves, and the hand-run verification one-liners above, are the real evidence for this
sub-step; the script's receipt is retained (mandatory even on what it believes is an empty diff)
with the correction standing next to it.

## 2. Graphify — run

Two attempts, disclosed honestly:

1. **`graphify update .`** (mechanical re-extraction, no LLM, my own pre-step — not itself required
   by template §6, added for freshness) against a copy of the shared checkout's existing
   `graphify-out/` (432 MB `graph.json`, dated 2026-08-22): reached `AST extraction:
   54792/54792 files (100%)` after ~10 minutes, then ran CPU-bound (single core, 100%, memory
   climbing to 2.7 GB, `graph.json` on disk still untouched — writes atomically at the end) for a
   further ~7 minutes with no further log output. Killed at ~17 minutes total, a judgment call
   against turn-budget risk, not a failure signal — `graph.json` was confirmed byte-unchanged
   (`ls -la` timestamp still the copy time) so nothing was corrupted by killing it.
2. **The official pipeline script** (`update_graphify.py --force`, since the tree carried
   in-progress uncommitted `kanban.md`/`progress.md` at invocation time) — the literal template §6
   sub-step 3 requirement — invoking `graphify cluster-only <repo_root> --budget 500000 --exclude
   node_modules,target,dist,build,.git,out,dist-ssr,.next,coverage` against that same (2026-08-22,
   now current-code, since `cluster-only` re-derives structure from the existing graph plus the
   live tree) graph. **Ran to completion: `graphify exit=0, elapsed=1463.5s (~24.4 min),
   outcome=success`.** Receipt auto-appended to `receipts.md` (`graphify:update` block, cycle_id
   `2026-08-26T01:32:02Z`). `graphify-out/GRAPH_REPORT.md` confirms a real result, not a stub:
   **648,328 nodes · 656,154 edges · 30,247 communities**, **3,620 of 656,154 edges (1%) INFERRED**
   (the remainder `EXTRACTED`/`AMBIGUOUS`, per `GRAPH_REPORT.md`'s own summary line), "Built from
   commit: `a47bd3bb`" — this cycle's own latest architecture-doc commit at
   run time, so the graph reflects current code, not the stale 2026-08-22 base. `graphify-out/` is
   gitignored (`.gitignore:10-12`) — the graph itself is never committed; the receipt is the durable
   evidence.

## 3. PR — opened

`tranche/13 -> develop`. Leads with the twelve defects found and fixed (table with unit counts and
commits), the throughput arc (32 → 6,940 → 7,939 → 8,255 → 8,263 → 8,291 → 8,330 of 8,330
examined; disagreements 26 → 4 → 1 → 0), headline figures each with its denominator, the
nine-failed-scans/tenth-passed history with what each attempt closed, and the 31-of-599 inherited
failing suites (49 of 8,026 tests, 0 of 31 carrying a commit since the cut) so a reviewer checking
out the branch and running the full workspace suite is not surprised by a non-green result. PR URL
and number recorded in this cycle's final report and in `progress.md`.

## 4. Merge conflicts — none

`git fetch origin tranche/13 && git rebase origin/tranche/13` was clean (fast-forward or no-op) at
every push this cycle; no lane wrote to `docs/architecture/**`, `receipts.md`, or `graphify-out/`
concurrently. **Not merging** — the operator merges `tranche/13 -> develop` per
`workflow-instruction.md §11.4`/this dispatch's own instruction.

## Movement, four buckets

Closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this cycle writes
package/architecture docs and opens the closure PR; no `docs/work-inventory.json` unit, status, or
gate changed.

## Notes

- Row 21 (`sweep-archdocs-graphify-pr`) is **not** marked `complete` by this cycle — release-notes
  (`AT-33-E6-003` part 2, `workflow-instruction.md §11.5`) owns that transition, per this cycle's
  own dispatch instruction. `kanban.md` row 21's Notes column gets a pointer to this receipt only,
  status left `not-started`.
- DO NOT MERGE was honored literally — no `gh pr merge` call anywhere in this cycle.

## Next-cycle plan

Release notes and version bump (`AT-33-E6-003` part 2 / `workflow-instruction.md §11.5`), then row
21 -> `complete`.
