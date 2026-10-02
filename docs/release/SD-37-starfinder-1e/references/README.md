---
canonical: true
owner: operator
bundle_id: SD-37
artifact_type: reference-index
---

# SD-37 References

## Conduct

- `../../../../AGENTS.md`: Non-Negotiable Rules (TDD, no fake completion, scope, rules 7–9), Blocker
  Discipline, Concurrency and Measurement.
- `../../../../CLAUDE.md`: activation surface.

## Templates (authored from, not copied)

- `../../template/template.md`: the chassis.
- `../../../governance/workflow-instruction-template.md`: the per-cycle procedure. Its §11 closure
  order is defective; corrected in `../workflow-instruction.md §11` (FSR-C9).
- `../../../../.claude/skills/stc-authoring/SKILL.md`: the repo-local authoring/audit skill.
- `~/.claude/skills/stc-bundle-authoring/references/unattended-mode-doctrine.md`: the user-level
  unattended doctrine, reconciled in `../decisions.md §12`.

## Doctrine

- `../../../governance/blocker-closure-doctrine.md`
- `../../../governance/deferral-revisit-doctrine.md` (DEF-1)
- `../../../governance/no-stub-mvp-doctrine.md` §"Per-cycle audit"
- `../../../governance/wired-integration-stubs-registry.md` (entry 0002, retired for SF at E4.6)
- `../../../governance/license-matrix.md`, `../../../governance/ogl-pi-blacklist.md` (E0.2)
- `../../../governance/book-ingestion-playbook.md` (per-book cycle procedure)
- `../../../doctrine-external/identifier-discipline.md`

## Retrospectives (read before dispatching)

- `../../../retro/sd36-retrospective.md`: lessons 1–20, especially 14–20 (→ rules R1–R7), and
  "Changes for the next bundle".
- `../../../retro/sd31-retrospective.md`: the shape E7.4 follows.
- **SD-37's own retrospective:** E7.4 writes it under `docs/retro/` and adds its link here. (C0.2
  removed the pre-written link: it made E7.4's citation check pass before E7.4 ran.)

## Sibling bundles

- `../../SD-36-consolidation/`: decisions §11.1 (structural-diff protocol), FS-3/FS-2/FS-15/27/28,
  `artifacts/epic-f/scripts/structural_diff.py`.
- `../../SD-35-corpus-sheet-completion/`: C2.1 (adopted), §11 (converter kept), token mapping
  table `artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json` (273 rows, CUI F-14).
- `../../SD-34-book-completion/`: P1 rows (FSR-C1).

## Oracle

- `../../../../scripts/pcgen-oracle-pin.env` (SHA `7f818006e3…`), `../../../../scripts/fetch-pcgen-oracle.sh`.
  The checkout is resolved through `$PCGEN_REPO_DIR` / `$PCGEN_CORPUS_ROOT`, never by a literal
  path.
- Starfinder Reference Document (OGL text, online): the source of the seed hand values
  (`../decisions.md §18`). Its URLs are recorded per row in `../artifacts/epic_0/seed-hand-values.md`.

## Memory notes this package encodes

`next-bundle-changes-from-sd36-epic-f`, `paper-sheet-generator-not-video-game`,
`sheet-generator-print-the-rule-not-simulate`, `batch-big-not-small-per-dispatch`,
`one-pass-verify-dont-repeat-a-clean-gate`, `no-carve-outs-close-dont-flag`,
`closure-criteria-must-not-permit-open-cards`, `graphify-runs-against-final-repo-state`,
`workflow-agent-model-must-be-explicit`, `rules-tables-stay-rust-until-starfinder`,
`operator-away-autonomous-mode`, `sheet-rule-package-path-is-baked-at-compile-time`,
`two-book-scope-is-a-deliberate-proof`.
