# SD-43 references

Paths are relative to this file (`docs/release/SD-43-market-inventory-and-abilities/references/`).

## The source of this package

- `../../SD-43-intake/intake.md` — the operator's intake, items 0–23 with 22a, 22b and 22c (22c committed `c9994e01bd`: the Class Features scope is every class- or archetype-granted choice; folded into the back half's acceptance and inventory), opened 2026-11-18, last
  edited 2026-11-22; every ruling quoted in `../decisions.md §3`. Untouched by this package.
- `../../SD-43-intake/signed-update-manifest-brief.md` — **not SD-43 work.** The brief places itself outside SD-43
  (operator correction 2026-11-21: *"NOT inside SD-43, NOT a new SD, and NOT a new tranche"*) and is already
  merged as PR #419 (`git merge-base --is-ancestor 2225c0111c HEAD && echo in` → `in`). SD-43 has no card for it;
  it reuses only its CI-secret pattern (`UPDATE_SIGNING_KEY`) for the report key (`../decisions.md §23`, `§25`) and
  re-checks at closure that the publish still signs (`../acceptance-and-verification.md` G-R).

## Doctrine (repo-local)

- `../../../governance/blocker-closure-doctrine.md` — clear or escalate, never defer; a filed blocker pauses.
- `../../../governance/deferral-revisit-doctrine.md` — condition, checker, accepted cost (`../forward-scope-register.md`).
- `../../../governance/no-stub-mvp-doctrine.md` §"Per-cycle audit" — the four-check audit every code card runs;
  `../../../governance/wired-integration-stubs-registry.md` — operator-granted exceptions (none for SD-43).
- `../../../doctrine-external/identifier-discipline.md` — the identifier audit.
- `../../../../AGENTS.md` (rules 7, 8, 9; Concurrency and Measurement; Blocker Discipline), `../../../../CLAUDE.md`.
- `../../SD-27-future-state-book-content-ingestion/decisions.md §29.1` — the twin rule (a magnitude is wired only
  when it moves on the surface the player reads).

## Templates and skills

- `../../template/template.md`; `../../../governance/workflow-instruction-template.md` (the procedure
  `../workflow-instruction.md` is authored from, not forked).
- `../../../../.claude/skills/stc-authoring/SKILL.md` (closure obligations, release-artifact closure, dual-audit
  gate, merge and watch); skills `workflow-authoring` (the script), `run-desktop` (screenshots), `graphify`.
- Closure scripts: `~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py`,
  `~/.hermes/profiles/god-emporer/skills/devops/graphify-update/scripts/update_graphify.py` (both present on
  2026-11-22: `ls` of each directory).

## Sibling bundles

- `../../SD-42-pcgen-import-and-campaign-folder/` — predecessor, merged (PR #413): the gear screen's container
  tables and per-copy names (`decisions.md §27.3–§27.5` there), the campaign folder (`§11`), the open-campaign
  banner (`§26.2`); its back half is the shape reference for this one.
- `../../SD-41-gear-and-equipment-sets/` — stacks, `CopyFacts`, equipment sets, the Market's price modes
  (`decisions.md §38` there: container tables).
- `../../SD-40-pf-sheet-design/` — the PF sheet's tabs, the Biography tab the rest text lands under.

## Retrospectives

- `../../../retro/sd42-retrospective.md` — **the predecessor's retrospective** (SD-42, written at its E5.2). Its
  four "Changes for the next bundle" are built into `../workflow-instruction.md §12.2` (script committed and RED
  recorded before dispatch; `pgrep -x cargo` as a command; receipt-row greps on return; `pcgen-residue-gate` on
  desktop test-module changes).
- `../../../retro/sd41-retrospective.md`, `../../../retro/sd40-retrospective.md` — the source of the carried rules
  in `../workflow-instruction.md §12.1`.
- `../../../retro/sd31-retrospective.md` — the shape the closure retrospective follows.
- **SD-43's own retrospective — not yet written.** E7.2 writes it under `../../../retro/` from
  `scripts/retro.py summary --since 2026-11-22 --json` (file name in `../decisions.md §26`) and adds its link to
  this list in the same commit. This entry deliberately does not spell the file name, so G-CL's
  `grep -c` for it in this file reads 0 until E7.2 cites it.

## The oracle (pinned, never shipped)

- `../../../../scripts/pcgen-oracle-pin.env` names `PCGEN_ORACLE_SHA`; resolve the checkout only through
  `$PCGEN_REPO_DIR` (repo root) and `$PCGEN_CORPUS_ROOT` (data), bootstrapped by
  `../../../../scripts/fetch-pcgen-oracle.sh` (`source <(bash scripts/fetch-pcgen-oracle.sh --print-env)`).
  Never a literal checkout path. E0.1 reads PCGen's UI strings for the seven sub-tabs and its `ABILITYCATEGORY`
  lines there; nothing from it enters live code (`../decisions.md §28`).

## Release tooling

- `../../../../tools/release/{resolve_release_notes,assert_version_stamp,verify_release_consistency}.py`,
  `../../../../scripts/tranche/validate-tranche-notes.py`, `../../current-release.json`,
  `../../../../.github/workflows/publish-tester-release.yml` (stamp at line 129).
- `../../../../apps/desktop/src-tauri/tauri.conf.json` `bundle.resources`;
  `../../../../apps/desktop/src-tauri/tests/packaged_resources.rs`.

## Event log

- `../../../../scripts/retro.py`, `../../../retro/schema.json`; events under `../../../retro/events/`.
