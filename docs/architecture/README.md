# Architecture docs

> Scope: index and maintenance contract for the `docs/architecture/` living-documentation set,
> plus reading paths into it by intent.
> Last verified: **2026-09-20 against `tranche/16` (`b22ea9e113`, SD-36 Epic D)** — added
> `getting-started.md` and `glossary.md` (new this pass) to the index and folded the crate-wall
> (`crates/codex-ingest`) into the "Source dirs" column and the reading paths below. Prior pass
> 2026-07-22 against tranche/5-3 (SD-25 closure). **Path correction 2026-08-22**
> (SD-32 closure epilogue, `workflow-instruction.md §13` architecture-docs refresh): the
> provenance-note example and the update-and-feedback source-dirs row cited the old
> apps/desktop/src/sd16/ directory, renamed to `apps/desktop/src/feedback/` / `apps/desktop/src/update/`
> by `06d926e90` (2026-08-10) — fixed below; no other content in this doc re-verified.
> Maintenance: updated at SD closure — see §Maintenance contract below

## Purpose

This directory is a living architecture documentation set for humans and
agents entering this repository cold. Every doc describes **current-state
function** — what the code does today — not the history of how it got that
way. When code changes, the corresponding doc is edited in place to describe
the new reality; it is never appended to with a changelog entry.

## Provenance note (the one place history is acknowledged)

Modules, tests, and directories across this repo carry `sdNN`/`geNN`
prefixes (e.g. `tests/sd25_sorcerer_level_up_explanation_coverage.rs`,
`docs/retro/events/sd31-transcribe.jsonl`). A 2026-08-10 sweep (`feat(sd29): function-based naming
sweep`) removed `sdNN`/`geNN` prefixes from directory and source-file names, so the still-current
examples are test/log filenames, not directories — e.g. the former apps/desktop/src/sd16/
frontend directory is now `apps/desktop/src/feedback/` and `apps/desktop/src/update/`, un-prefixed.
These prefixes name the
originating spec-domain or grand-epic bundle that created the file — they
are **proper nouns**, not documentation of what the code currently does.
This doc set describes function, not the bundle that produced it; treat a
`sdNN`/`geNN` prefix the way you'd treat a person's name, not a status flag.
Release-bundle narratives, closure receipts, and tranche-level decisions live
under `docs/release/`, not here.

## Index

The "Source dirs" column names the real repo paths each doc covers. It is
not decorative — the maintenance procedure below uses it directly to map a
changed file to the doc(s) that need re-checking, so keep it accurate when a
doc's coverage shifts.

| Doc | Scope | Source dirs |
|---|---|---|
| [overview.md](./overview.md) | What Codex is, the product doctrine, the four planes, the workspace/crate dependency graph, the end-to-end data-flow diagram, the directory map | `src/`, `crates/codex-ingest/`, `apps/desktop/`, `.github/workflows/` |
| [getting-started.md](./getting-started.md) | Toolchain setup, build/run/test commands for every crate and the frontend, running the desktop app, `verify.sh`, a first-contribution walkthrough, the branch/PR model | (onboarding surface; no single source dir) |
| [glossary.md](./glossary.md) | Every project term (SD-nn, tranche, epic, STC package, corpus, wiring_class, crate wall, oracle, ui-smoke, ...) defined once | (reference surface; no single source dir) |
| [conventions.md](./conventions.md) | Cross-cutting idiom catalog: fail-honest, store shape, DI seams, boundary rule, TDD | `src/`, `apps/desktop/src/` |
| [status.md](./status.md) | What is real vs. stubbed/partial/deferred across the whole repo | `src/`, `apps/desktop/` |
| [corpus-ingest.md](./corpus-ingest.md) | PCGen `.pcc`/`.lst` parsing into canonical source-IR, the sheet-rule converter | `crates/codex-ingest/src/pcgen_import/` |
| [rules-engine.md](./rules-engine.md) | The headless PF1 compute spine and per-domain engines | `src/rules_core/` (excluding `rules_tables/`) |
| [rules-data-tables.md](./rules-data-tables.md) | Hand-transcribed per-book Paizo rule-data tables | `src/rules_core/rules_tables/` |
| [persistence.md](./persistence.md) | Saved-character and campaign local on-disk storage | `src/saved_character/`, `src/campaign/`, `apps/desktop/src-tauri/src/character_hub.rs`, `apps/desktop/src-tauri/src/campaign_drive.rs` |
| [homebrew-and-oracle.md](./homebrew-and-oracle.md) | Homebrew package authoring; oracle-parity fixture schema | `src/homebrew_authoring/`, `crates/codex-ingest/src/oracle_validation/` |
| [desktop-app.md](./desktop-app.md) | Tauri shell build, command inventory, boundary layer, frontend map, rule-system adapter seam | `apps/desktop/` |
| [update-and-feedback.md](./update-and-feedback.md) | Self-update chain and feedback/defect-report submission chain | `apps/desktop/src/feedback/`, `apps/desktop/src/update/`, `apps/desktop/src/testerWorkbench/feedback/`, `apps/desktop/src/testerWorkbench/update/`, `apps/desktop/src-tauri/src/update/`, `apps/desktop/src-tauri/src/browser_handoff.rs`, `schemas/update/` |
| [release-pipeline.md](./release-pipeline.md) | Publish workflow, manifest generation, branch-promotion gates | `.github/workflows/`, `scripts/release/`, `tools/release/`, `scripts/tranche/` |
| [testing.md](./testing.md) | Full verification command set, fixture grammar, corpus-gated tests | `tests/`, `apps/desktop/scripts/run-tests.mjs`, `apps/desktop/src/testSupport/` |
| README.md (this file) | Doc-set index, provenance note, maintenance contract | `docs/architecture/` |

## Reading paths by intent

Start at [overview.md](./overview.md) if you have never seen this repo before, then
[getting-started.md](./getting-started.md) to get a toolchain up and a test passing. Once oriented,
jump straight to the doc(s) for what you're actually doing:

- **Fix a wrong number on a sheet** → [rules-engine.md](./rules-engine.md) (the compute spine and
  the fail-honest pattern) → [rules-data-tables.md](./rules-data-tables.md) if the wrong number
  comes from a hand-transcribed table rather than a compute function → [testing.md](./testing.md)
  §"The fixture grammar" to write the failing fixture first (TDD is mandatory, `AGENTS.md` rule 1).
- **Add a book** (new Paizo sourcebook) → [corpus-ingest.md](./corpus-ingest.md) (how a `.pcc`/`.lst`
  book becomes `data/corpus/<book>/**/*.json` and, where sheet-rule-converted,
  `data/sheet_rules/<book>/**`) → [rules-data-tables.md](./rules-data-tables.md) §"`RuleSetId` and
  per-book resolution" if the book needs a new hand-transcribed chassis →
  `docs/governance/book-ingestion-playbook.md` for the per-book cycle procedure.
- **Add a UI screen or Tauri command** → [desktop-app.md](./desktop-app.md) (command inventory, the
  `build*Surface`/`*Runtime` DI pattern, the boundary-wrapper rule) →
  [conventions.md](./conventions.md) §"Command / pure-fn split" and §"Boundary wrapper rule" for the
  two idioms every new command/screen pair follows.
- **Add or fix a test/gate** → [testing.md](./testing.md) (the full command set, fixture grammar,
  corpus-gated-test patterns, `verify.sh` stage list) → [getting-started.md](./getting-started.md)
  §"First contribution, red to green" for a worked example → [glossary.md](./glossary.md) if a gate
  name (`denominator-gate`, `crate-wall`, `pcgen-residue-gate`, ...) is unfamiliar.
- **Cut a release** → [release-pipeline.md](./release-pipeline.md) (publish pipeline, branch
  promotion, version stamp) → [getting-started.md](./getting-started.md) §"Branch model, commits,
  and PRs" for the `tranche/N` → `develop` → `test` → `main` flow and who merges what.
- **Investigating CI** → [release-pipeline.md](./release-pipeline.md).
- **What works today** → [status.md](./status.md).
- **How saved characters/campaigns are stored** → [persistence.md](./persistence.md).
- **Self-update or feedback-submission behavior** → [update-and-feedback.md](./update-and-feedback.md).
- **"How do we normally do X in this repo"** → [conventions.md](./conventions.md).
- **An unfamiliar term** → [glossary.md](./glossary.md).

## §Maintenance contract

**Rules.** These docs describe current state only. An edit **replaces** an outdated statement — it never appends a history note, a changelog line, or "as of SD-NN" phrasing. Obsolete statements are REMOVED, not annotated as deprecated. Every factual claim about code cites a backticked, repo-relative path.

**Update-on-PR procedure.** Before **any** PR that opens against the integration target (`develop` for closure; intra-tranche for hotfix or partial-promotion PRs):

1. Run `git diff <target>...HEAD --stat -- src crates apps schemas scripts tools .github docs/release docs/architecture` to see everything that changed since the integration target. Every PR is preceded by a truth-up cycle, regardless of whether the diff has architecture impact — empty updates still write a receipt as the audit trail.
2. Map each changed path to a doc using the index table's "Source dirs" column above.
3. Update every doc whose source dirs were touched — edit the affected statement in place; remove obsolete statements rather than appending "as of SD-NN" phrasing.
4. Always re-check [status.md](./status.md) — stub graduations (a stub becoming real, tested behavior) and regressions (real behavior reverting to stub) are the most common change this doc set needs to reflect, and they are easy to miss if you only diff the doc that first named the stub.
5. Refresh the `Last verified` line of every doc you actually re-verified against the new commit.

**Epic Closure pipeline.** Truth-up is sub-step 2 of 5 in the bundle's Epic Closure. The pipeline is sequential; no sub-step is skipped or conditional:

1. All acceptance criteria done? If not, self-heal and run more loops. Repeat until done.
2. Architecture docs updated? If not, run the truth-up script (`~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py`). It edits in place, removes obsolete content, refreshes headers, runs the verification one-liners, appends a YAML receipt to `<bundle>/receipts.md`. Repeat until verification one-liners pass and the receipt is in `receipts.md`.
3. Graphify run? If not, run the graphify-update script (`~/.hermes/profiles/god-emporer/skills/devops/graphify-update/scripts/update_graphify.py`). The script invokes graphify against the codex repo, captures stdout/stderr/exit-code, appends a `graphify:update` receipt — **success OR failure, do not refuse on graphify non-zero exit**. Operator decides retry-vs-proceed.
4. PR open? If not, open it. (PR creation is a bash-level command in the workflow-instruction, not a separate skill.)
5. Merge conflicts resolved? If any, fix them via the merge-conflict-resolution skill (`~/.hermes/profiles/god-emporer/skills/devops/merge-conflict-resolution/scripts/resolve_merge_conflicts.py`). Otherwise proceed.
6. Stop the loop.

Skills owned by this pipeline: `architecture-truth-up` (sub-step 2), `graphify-update` (sub-step 3), `merge-conflict-resolution` (sub-step 5). The truth-up skill is the load-bearing reference for this sub-step; the other two skills carry their own doctrine.

**Three always-ask questions:**

- Did any stub graduate to real behavior, or regress to a stub? → update [status.md](./status.md).
- Did any Tauri command or `boundary/*.ts` wrapper get added, removed, or re-routed? → update [desktop-app.md](./desktop-app.md)'s command inventory.
- Did versioning, CI, or branch-promotion behavior change? → update [release-pipeline.md](./release-pipeline.md).

**Verification one-liners** (run from the repo root):

```bash
# cited-path existence check (excludes glob/placeholder patterns like `*`/`<xyz>`,
# which are illustrative, not literal paths)
grep -rhoE '`(src|crates|apps|tests|scripts|tools|schemas|docs|data|\.github)/[^`]*`' docs/architecture/*.md | tr -d '`' | sed 's/[:#].*$//' | grep -vE '[* <]' | sort -u | while read -r p; do [ -e "$p" ] || echo "MISSING: $p"; done
# relative-link check
grep -rhoE '\]\(\./[^)]+\.md' docs/architecture/*.md | sed 's/](\.\///' | sort -u | while read -r f; do [ -f "docs/architecture/$f" ] || echo "BROKEN LINK: $f"; done
```

The relative-link check prints nothing when the doc set is internally
consistent. All citations in [release-pipeline.md](./release-pipeline.md)
and [desktop-app.md](./desktop-app.md) are written in full repo-relative
form, so the cited-path check has nothing to flag there. An abbreviated
citation — e.g. `<src-dir>/testSupport/makeSurface.ts` instead of the real
`apps/desktop/src/testSupport/makeSurface.ts` — is written with a
placeholder segment (`<...>`) precisely so the cited-path check's own
exclusion pattern skips it, rather than as a bare path the checker would
have to be told by hand to ignore. Any hit from the cited-path check is a
real citation that needs to be written in full repo-relative form.

**Out-of-repo mirror note.** The authoritative SD-lifecycle doctrine —
governing what a "closure" is and when this contract applies — lives in the
operator's workspace at `~/workspace/governance/spec-domain-lifecycle.md`,
outside this repo and not editable from here. If the closure obligation
described above changes, the operator must mirror that change into their own
workspace doc by hand. `docs/doctrine-external/` in this repo holds stub
mirrors of external doctrine only — those stub files must not receive new
content from within this repo.
