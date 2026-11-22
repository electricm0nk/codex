# SD-40 references

## Doctrine (repo-local)

- `../../../governance/blocker-closure-doctrine.md` — clear or escalate, never defer.
- `../../../governance/deferral-revisit-doctrine.md` — planned capability deferrals (DEF-1…DEF-4).
- `../../../governance/no-stub-mvp-doctrine.md`, `../../../governance/wired-integration-stubs-registry.md`.
- `../../../doctrine-external/identifier-discipline.md`.
- `../../../../AGENTS.md`, `../../../../CLAUDE.md`.

## Templates and skills

- `../../template/template.md`; `../../../governance/workflow-instruction-template.md` (its §11 order is defective; `decisions.md §15` has the corrected order).
- `../../../../.claude/skills/stc-authoring/SKILL.md`; Hermes skills `stc-bundle-authoring`, `stc-codex-overlay`, `stc-lifecycle-routing`, `workflow-authoring`.

## Sibling bundles

- `../../SD-39-ui-print-corrections/` — predecessor: the print pipeline (E8.1–E8.8), rules R1–R20
  (`workflow-instruction.md §12`), the E9.4 receipt whose operator words open this bundle, `decisions.md §25`'s print corrections.
- `../../tranche-17-ui/` — `sheetLayout.ts` registry (the shared vocabulary's UI side).
- `../../SD-38-cyberpunk-red/`, `../../SD-37-starfinder-1e/` — the SF/CPR sheets whose print output SD-40 keeps unchanged.

## The picture to aim at (pinned oracle, never shipped)

- `$PCGEN_REPO_DIR/outputsheets/d20/fantasy/pdf/` — `csheet_fantasy_std_*.xslt`, `csheet_fantasy_simple_*.xslt`,
  `csheet_fantasy_spellbook*`, `fantasy_master_common_blocks.xslt` (the std page order); resolved per
  `scripts/pcgen-oracle-pin.env` and `scripts/fetch-pcgen-oracle.sh`.
- `scripts/oracle_harness/charbuild_remainder_run_one.sh` — the headless BatchExporter invocation E0.1 copies.
- `artifacts/epic_0/E0.1/` — the rendered reference set (after E0.1).

## Retrospectives

- `../../../retro/sd39-retrospective.md` — the five "Changes for SD-40" → R21–R24, FSR-C1.
- `../../../retro/sd38-retrospective.md`, `../../../retro/sd37-retrospective.md`.
- `../../../retro/sd40-retrospective.md` — SD-40's own, written and cited here at E5.3.

## Operator rulings

- `../artifacts/cycle_0/operator-brief-2026-11-07.md` — the seven rulings and the operator's words, verbatim where quoted.
- Standing rulings 2026-09-07 (paper-sheet rule; no PCGen in live code) — `decisions.md §17, §18`.
