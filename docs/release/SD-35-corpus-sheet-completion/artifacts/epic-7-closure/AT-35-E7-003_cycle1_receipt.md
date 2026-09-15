# Cycle AT-35-E7-003 cycle 1 — Epic 7 closure epilogue / AT-35-E7-003 (part 1: architecture docs, graphify, PR)

- **Commit SHA:** `cdf06fb00d`, `411e446088`, `59c2051a2e` (+ the graphify/PR commit below)
- **Scope gate:** `SCOPE_GATE: EXEMPT (closure-epilogue cycle — closes zero content units by design; workflow-instruction.md §6 step 1 floor exemption)`
- **Files touched:**
  - `docs/architecture/overview.md`, `rules-engine.md`, `rules-data-tables.md`, `corpus-ingest.md`, `desktop-app.md`, `status.md`, `testing.md`, `homebrew-and-oracle.md`
  - `docs/release/SD-35-corpus-sheet-completion/receipts.md` (new — the closure-pipeline ledger)
  - `docs/release/SD-35-corpus-sheet-completion/release-notes.md` (gate fix, see below)
  - `docs/retro/events/at-35-e7-003-arch.jsonl` (new)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS — the `SD-NN`/`AT-35-*` strings in these docs are citations of release-package sections, which `docs/` is the sanctioned home for, not identifiers leaking into code.
- **Wired-integration audit result:** OK_NO_TOKENS — docs-only cycle; no `src/`, `apps/`, `scripts/` or `data/` path touched.
- **Acceptance criterion** (verbatim, `epic-breakdown.md` AT-35-E7-003): "Full worktree/branch sweep with counts; architecture-docs refresh (`rules-engine.md`, `rules-data-tables.md`, `corpus-ingest.md`, `desktop-app.md`, `status.md`, `testing.md` — the boundary in `technical-design.md §0` becomes a stated architecture fact) and graphify per `../template/template.md §6`; PR to `develop`; release notes and version confirmation"

## What the refresh actually changed

Each doc is **current-state truth, re-verified against the live tree** — not a changelog entry.

| doc | new section | verified against |
|---|---|---|
| `overview.md` | §"The converter/live boundary" — `technical-design.md §0` becomes a stated architecture fact | `pcgen_residue_gate.py --check --closure` |
| `rules-engine.md` | §"The sheet rule — the sixth layer" | `src/rules_core/sheet_rule.rs` (`SheetLineValue` quoted verbatim, `render_sheet` at :1957), `sheet_rule_catalog.rs`, `corpus_loader.rs`, `pilot_compute/mod.rs:250` |
| `corpus-ingest.md` | §"The sheet-rule converter (`data/sheet_rules/`)" | `src/pcgen_import/sheet_rule/{closure,convert,ctx,formula,mod,prereq,prose,table}.rs`, `data/sheet_rules/_report.json`, `data/sheet_rules/GENERATED` |
| `rules-data-tables.md` | §"Two data stores, and which one a new rule goes in" | `src/rules_core/rules_tables/mod.rs`, `corpus_loader.rs`, `_report.json` |
| `desktop-app.md` | §"The 'Rules and features' section" | `apps/desktop/src-tauri/src/character_hub.rs:624-700`, `apps/desktop/src/characterHub/CharacterSheet.tsx:2041-2090`, `rulesAndFeaturesSection.test.ts` |
| `status.md` | §"Corpus coverage at SD-35 closure — 49,450 of 49,450", **superseding every corpus-coverage section below it** | `docs/work-inventory.json` totals, `_report.json`, the residue gate, the retrospective §5 |
| `testing.md` | §"SD-35: the tax cut, and the four gates every cycle runs" | `scripts/verify.sh` `ALL_STAGES`, `scripts/verify-baselines.env` tail values, `build-time.json` `paired_rerun` |

## Figures + their re-derive commands

| figure | value | command |
|---|---|---|
| inventory completion | 49,450 of 49,450 = 100% | `python3 scripts/completion_atlas.py --check` |
| corpus completion (headline) | 48,864 of 48,864 real `data/corpus` rules records = 100% | `python3 -c "import json;d=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population-census-final.json'));print(d)"` |
| live PCGen residue | `live_files=0 live_hits=0 verdict=PASS` | `python3 scripts/pcgen_residue_gate.py --check --closure` |
| converter coverage | 49,450 of 49,450 converted, 0 refused, 70,317 rules, 5,294 var tables, 423 degraded | `python3 -c "import json;print(json.load(open('data/sheet_rules/_report.json')))"` |
| `verify.sh` stages | 49 of 49 (`--quick` runs 42 of 49) | `python3 -c "import re;s=open('scripts/verify.sh').read();print(len(re.search(r'ALL_STAGES=\((.*?)\)',s,re.S).group(1).split()))"` |
| registered Tauri commands | 69 | `python3 -c "import re;s=re.sub(r'//[^\n]*','',open('apps/desktop/src-tauri/src/main.rs').read());print(len(re.search(r'generate_handler!\[(.*?)\]',s,re.S).group(1).split(',')))"` |
| test-binary baseline | 589 → 419 (deliberate; the AT-35-E1-003 fold) | `grep -E '^BASELINE_ROOT_TEST_BINARIES=' scripts/verify-baselines.env \| tail -1` |
| build time before/after | 188.97 s → 145.89 s cold, paired (−43.08 s, −22.8% of 188.97 s) | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-1-tax-cut/build-time.json'))['paired_rerun'])"` |
| worktrees still present | 16 dirs, 13 G, 15 `worktree-wf_*` branches | `ls .claude/worktrees \| wc -l` ; `du -sh .claude/worktrees` |

## Corrections this cycle made (each logged as a retro event)

1. **`release-notes.md` shipped over a red gate at `bfd0832799`.**
   `denominator_gate.py --check` returned **violations=27** — every percentage in the
   lines-per-unit and `Words`-share tables was bare -- a percent sign with no "of N" behind
   it -- which is the exact
   failure the gate exists to prevent: the right number against an unnamed denominator. Plus
   three cites to a `retrospective.md` that does not exist, and two re-derive commands naming
   `lines_per_unit_census.py` / `words_share_census.py`, neither of which was ever written —
   a green `--check-provenance` over an unrunnable command, which is `AGENTS.md` rule 9's
   named failure. Fixed: `violations=27 → 0`, cites repointed at
   `docs/retro/sd35-corpus-sheet-completion-retrospective.md §2`/`§3`, which carry the real
   heredoc commands.

2. **Ten stale cited paths in `docs/architecture/`**, all found by
   `architecture_truth_up.py`'s `cited_path_check` (EXIT=7), none of which any doc-level
   review had caught. Six were SD-35's own Epic 6 relocation (`cache_gen/`, `wiring_class.rs`,
   `formula_interpreter{,_corpus_wide}.rs` moving from `src/rules_core/` to
   `src/pcgen_import/`); two were the AT-35-E1-003 test fold; one was a deleted probe bin.
   **One had never been true:** `corpus-ingest.md §Stage 5` described a
   `src/pcgen_import/source_content_payload.rs` module and a `pub use` re-export that have
   never existed, and stated the dependency direction backwards. The enum is
   `pub enum SourceContentPayload<'a>` at `src/rules_core/source_content.rs:79`.

## Verification run

```
python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md'
    files_checked=14  violations=27  ->  files_checked=14  violations=0
python3 scripts/denominator_gate.py --check-provenance
    files_checked=295  figures_examined=627  violations=0
python3 scripts/pcgen_residue_gate.py --check --closure
    live_files=0 live_hits=0 verdict=PASS
python3 scripts/completion_atlas.py --check
    done_evidence_violations=0  missing_clearing_mechanisms=0  citation_failures=0
bash scripts/verify.sh --only pi-sweep
    RESULT: PASS  (11 hits over src/rules_core/rules_tables, 11 baseline rows)
architecture_truth_up.py --integration-target develop --bundle SD-35
    EXIT=7 (10 findings) -> EXIT=7 (3) -> EXIT=0 "cited-path check + relative-link check both pass"
```

**Build scope verified:** N/A — no `src/`, `apps/`, `scripts/` or `data/` path was touched, so
nothing this cycle wrote can move a compile or a test. The last full-workspace result stands
(`verify.sh` full, 49 of 49 PASS, recorded in `docs/retro/events/sd31-transcribe.jsonl` at
`7753c29915`).

- **PCGen residue:** `live_files=0 live_hits=0 verdict=PASS` — unchanged, and at the closure floor.
- **Oracle parity:** N/A — no `Number` mapping added; no live path touched.
- **Movement, four buckets:** closure 0 / relabel 0 / reachability 0 / instrument-correction 2
  (the release-notes denominator + provenance fix, and the ten cited-path corrections). A
  closure-epilogue cycle moves no content units by design.
- **Refused tokens:** none — this cycle converts nothing.
- **Sweep population:** N/A — `data/corpus/**` untouched.

## Graphify — which SHA it indexed

Run per `../template/template.md §6` step 3, over the **final** repo state, with
`git status --porcelain` printing nothing and HEAD equal to `origin/tranche/15`:

```
INDEXED SHA: 7a0af6f2e48cd7db9be17eee4701ca231dddf7a7
[graphify-update] working tree: clean
[graphify-update] invocation: graphify cluster-only /home/ubuntu/workspace/repos/codex \
                  --budget 500000 --exclude node_modules,target,dist,build,.git,out,dist-ssr,.next,coverage
[graphify-update] graphify exit=0, elapsed=1322.4s, outcome=success
[graphify-update] log written: graphify-out/.truth-up-run-2026-09-15T21:17:57Z.log
EXIT=0
```

`graphify:update` receipt appended to `receipts.md`. `graphify-out/` is gitignored
(`.gitignore:10-12`), so the index run does not move the tree and the indexed SHA stays the
one recorded here.

**One thing this run does NOT cover, stated rather than buried.** A prior full re-extraction
(`graphify . --update`) was attempted first and **exited early**: `error: no LLM API key found
(1672 doc/paper/image file(s) need semantic extraction)`. The mandated closure step is
`cluster-only`, which re-clusters the existing graph and is what succeeded above — so the
community structure is current as of this SHA, but the **semantic extraction of the 1,672
doc/image files is as of the last keyed run (2026-08-22)**. Code extraction needs no key. If the
operator wants the doc half re-extracted, it needs one of `GEMINI_API_KEY` / `ANTHROPIC_API_KEY`
/ `OPENAI_API_KEY` in the environment and a re-run of `graphify . --update`.

## Open blocker raised (disposition 2 — raise your hand and wait)

**The worktree sweep cannot be completed by any dispatched agent.** 16 worktrees under
`.claude/worktrees/` holding **13 G**, plus their 15 `worktree-wf_*` branches, are still present
at HEAD. `git worktree remove --force` was refused by the Claude Code auto-mode permission
classifier — the **sixth** consecutive refusal (AT-35-E2-REGATE, AT-35-E2-REGATE2,
AT-35-E2-REGATE3, AT-35-E3-WRAPUP-RERUN, AT-35-E6-WRAPUP-FIX2, and this cycle). It is refused by
the harness, not by git: AT-35-E6-WRAPUP-FIX2 already proved removal **lossless** for four of
them (9 of 9 retro shards byte-identical, every report a strict superset, no unmerged branch, no
tracked modifications).

`AGENTS.md` rule 8: a warning copied forward six times is a missing mechanism, not bad luck. The
exact thing needed, named:

```
git worktree remove --force .claude/worktrees/wf_291be5c8-5f3-*
git branch -D worktree-wf_291be5c8-5f3-*
```

— run by the operator, **or** a Bash permission rule for `git worktree remove` added to
settings so a dispatched agent can do it. Logged as
`deferral 1789506077090-at-35-e7-003-arch-67aba6` and named in the PR body.

- **Status:** complete (for this cycle's own scope: architecture docs, truth-up gate, graphify,
  PR), with the worktree sweep escalated to the operator as above.
- **Next-cycle scope:** criterion at zero for the architecture-docs / graphify / PR half.
  Part 2 (release notes + version confirmation at `0.15.0`) is the release-notes agent's;
  its gate violations are already fixed here.
