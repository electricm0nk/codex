# SD-41 references

## Doctrine (repo-local)

- `../../../governance/blocker-closure-doctrine.md` — clear or escalate, never defer.
- `../../../governance/deferral-revisit-doctrine.md` — planned capability deferrals (DEF-1…DEF-4).
- `../../../governance/no-stub-mvp-doctrine.md`, `../../../governance/wired-integration-stubs-registry.md`.
- `../../../doctrine-external/identifier-discipline.md`.
- `../../../../AGENTS.md` (the twin rule under "Concurrency and Measurement"), `../../../../CLAUDE.md`.

## Templates and skills

- `../../template/template.md`; `../../../governance/workflow-instruction-template.md` (its §11 order is defective; `decisions.md §18` has the corrected order).
- `../../../../.claude/skills/stc-authoring/SKILL.md`; Hermes skills `stc-bundle-authoring`, `stc-codex-overlay`, `stc-lifecycle-routing`, `workflow-authoring`, `run-desktop` (screenshots, E3.MC / E4.MC).

## Sibling bundles

- `../../SD-40-pf-sheet-design/` — live predecessor on `tranche-20`: the print pipeline and looks, the Inventory section (E3.2) E4.1 re-lays, rules R1–R24 (`workflow-instruction.md §12`), the side-lane precedent on `character_hub.rs` (`epic-breakdown.md §0`), `decisions.md §21`'s fresh-tree corpus-bundle note.
- `../../SD-39-ui-print-corrections/` — the print pipeline's origin (E8); the Gear tab's Equipment panel (E7.5, "a copy left behind weighs nothing").
- `../../tranche-17-ui/` — `sheetLayout.ts` registry (the Gear tab's ids).
- `../../SD-27-future-state-book-content-ingestion/decisions.md §29.1` — the twin rule's origin.

## The code this bundle reads first

- `src/rules_core/{character_input,body_slots,equipment_effects,encumbrance,damage_total,equipment_record}.rs`;
  `src/saved_character/{mod,local_store}.rs`; `apps/desktop/src-tauri/src/{character_hub,character_custom}.rs`;
  `apps/desktop/src-tauri/src/print/blocks/gear.rs`; `apps/desktop/src/characterHub/{EquipmentPanel,CharacterSheet,MarketDialog,CustomDialog}.tsx`;
  `apps/desktop/src/boundary/{listHeldEquipment,setEquipmentActiveState}.ts`; `data/equipment_types.json`.
- `artifacts/epic_0/census.txt` — the figures and their commands.

## The functional reference (pinned oracle, never shipped)

- `$PCGEN_REPO_DIR/docs/tabpages/inventory/inventoryequipping.html` and `inventorygear.html` — PCGen's
  Equipping and Purchase sub-tabs, described in our words in `technical-design.md §1`; resolved per
  `scripts/pcgen-oracle-pin.env` and `scripts/fetch-pcgen-oracle.sh` (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`).
- No PCGen source is cited; `decisions.md §17`.

## Retrospectives

- `../../../retro/sd39-retrospective.md` — the five "Changes for SD-40" → R21–R24.
- `../../../retro/sd40-retrospective.md` — written at SD-40's E5.3 (after this package); the lead reads its
  changes before dispatching E2.1 (`workflow-instruction.md §12.2`).
- `../../../retro/sd41-retrospective.md` — SD-41's own, written and cited here at E5.2.

## Operator rulings

- `../artifacts/cycle_0/operator-brief-2026-11-12.md` — the request and the nine answers, verbatim.
- Standing rulings 2026-09-07 (paper-sheet rule; no PCGen in live code) — `decisions.md §16, §17`.
