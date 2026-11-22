# SD-42 references

## Doctrine (repo-local)

- `../../../governance/blocker-closure-doctrine.md` — clear or escalate, never defer.
- `../../../governance/deferral-revisit-doctrine.md` — planned capability deferrals (DEF-1…DEF-5).
- `../../../governance/no-stub-mvp-doctrine.md`, `../../../governance/wired-integration-stubs-registry.md`.
- `../../../doctrine-external/identifier-discipline.md`.
- `../../../../AGENTS.md` (rule 9: every figure with its command; the twin rule), `../../../../CLAUDE.md`.

## Templates and skills

- `../../template/template.md`; `../../../governance/workflow-instruction-template.md` (its §11 order is defective; `decisions.md §23` has the corrected order).
- `../../../../.claude/skills/stc-authoring/SKILL.md`; Hermes skills `stc-bundle-authoring`, `stc-codex-overlay`, `stc-lifecycle-routing`, `workflow-authoring`, `run-desktop` (screenshots, E2.MC / E4.MC).

## Sibling bundles

- `../../SD-41-gear-and-equipment-sets/` — live predecessor on `tranche-21`: stacks in pieces (§28, §30), `CopyFacts` (§24), Buy Custom (§24), the campaign write-through mirror (§29), container tables (§38; the intake's item 3), rules R1–R25 (`workflow-instruction.md §12`).
- `../../SD-40-pf-sheet-design/` — the Notes and Biography tabs (E3.6) the import's Notes block lands in.
- `../../SD-39-ui-print-corrections/` — R13: the app never parses a `.pcg`; the sidecar.
- `../../SD-37-starfinder-1e/` — the SF engine, `sf_oracle_parity.rs` (the explained-ledger pattern E1.7 reuses), the SF tables under `data/starfinder-1e/`.
- `../../SD-33-computed-value-verification/` — `scripts/oracle_harness/` (the PCGen export templates E1.7 reuses).

## The code this bundle reads first

- `crates/codex-ingest/src/pcg_import/{map,catalog,wire,parse,fixtures_tests}.rs`; `crates/codex-ingest/src/bin/pcg_import.rs`;
  `apps/desktop/src-tauri/src/character_hub.rs` (`import_pcg`, `import_pcg_json`, `import_character_from_json`, `after_write`, `purchase_custom_equipment_at_root`);
  `src/saved_character/{equipment_sets,campaign_mirror,local_store}.rs`; `src/campaign/{mod,local_store}.rs`;
  `apps/desktop/src-tauri/src/{campaign_drive,encounter_rating,sf_adapter,rule_system_adapter}.rs`;
  `apps/desktop/src/campaign/**`, `apps/desktop/src/settings/{GoogleDrivePanel.tsx,googleDrive.ts}`,
  `apps/desktop/src/characterHub/{CampaignMembershipDialog.tsx,campaignMembershipModel.ts,CharacterHubPage.tsx,gearScreenModel.ts}`,
  `apps/desktop/src/dmToolkit/{encounterBuilderModel.ts,EncounterBuilderScreen.tsx}`, `apps/desktop/src/boundary/{importPcg,rateEncounter,characterPortrait}.ts`.
- `docs/architecture/pcg-import.md` — the mapping table the converter prints (re-printed at E4.MC).
- `artifacts/epic_0/census.json` — the figures and their keys.

## The oracle (pinned, never shipped)

- `$PCGEN_REPO_DIR/code/testsuite/PCGfiles/{pf_Cleric,pf_Paladin,pf_Rogue,pf_goldielocks,sf_mechanic,sf_mechanic_drone,sf_soldier}.pcg` — the 7 sample files (Q3, Q7);
  `$PCGEN_REPO_DIR/system/gameModes/Pathfinder/equipmentslots.lst` — the slot names E1.2's table covers; resolved per
  `scripts/pcgen-oracle-pin.env` and `scripts/fetch-pcgen-oracle.sh` (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`).
- The operator's file: `$CODEX_SD42_PCG`, local only (`decisions.md §10`).
- No PCGen source is cited; `decisions.md §22`.

## Third-party packages (all MIT; `decisions.md §18`)

- CodeMirror 6: https://codemirror.net/ (`@codemirror/*`, `@lezer/*`); `marked`: https://marked.js.org/; Obsidian (the compatibility target, not a dependency): https://obsidian.md.

## Retrospectives

- `../../../retro/sd40-retrospective.md`, `../../../retro/sd39-retrospective.md` — the rules R21–R25 came from these.
- `../../../retro/sd41-retrospective.md` — written at SD-41's E5.2 (after this package); the lead reads its changes before dispatching E4.1 (`workflow-instruction.md §12.2`).
- `../../../retro/sd42-retrospective.md` — SD-42's own, written and cited here at E5.2.

## Operator rulings

- `../artifacts/cycle_0/operator-brief-2026-11-16.md` — the intake (2026-11-15) and rulings Q2–Q25 (2026-11-16), verbatim.
- Standing rulings 2026-09-07 (paper-sheet rule; no PCGen in live code) — `decisions.md §22, §24`.
