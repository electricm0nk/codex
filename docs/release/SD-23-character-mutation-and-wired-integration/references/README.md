# SD-23 References

External reference docs cited by the SD-23 package. Mirrors SD-21's `references/` directory. Each entry is a stable pointer — the doctrine may move within `governance/` but the canonical file remains at the path listed.

## Doctrine references

- **Wired Integration Doctrine** — `../../governance/no-stub-mvp-doctrine.md`
  - Authoritative doctrine for "no stubs in shipping code; stubs are the exception requiring explicit operator approval." Active 2026-07-20.
- **Identifier Discipline Doctrine** — `../../governance/identifier-discipline.md`
  - Sibling doctrine. Source-code identifiers describe what the artifact does, not which release or spec domain it came from.
- **Spec-Domain Lifecycle** — `../../governance/spec-domain-lifecycle.md`
  - Sibling doctrine. Closed bundles stay closed; bundles don't own code.

## Registry references

- **Wired Integration Stubs Registry** — `../../governance/wired-integration-stubs-registry.md`
  - Doctrine-of-record for any operator-granted stub exception. Entry #0001 is the browser-preview fallback in `characterHubRuntime.ts`.

## Skill references

- **`wired-integration-discipline`** — `~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/SKILL.md`
  - Procedural skill. The per-cycle four-check audit is defined here.
- **`identifier-discipline`** — `~/.hermes/profiles/god-emporer/skills/devops/identifier-discipline/SKILL.md`
  - Sibling skill. Source-code identifier audit + rename cycle.
- **`kanban-claude-code-execution-receipt`** — `~/.hermes/profiles/god-emporer/skills/devops/kanban-claude-code-execution-receipt/SKILL.md`
  - Sibling skill. Per-cycle receipt capture into kanban card comments stream.

## Predecessor bundle references

- **SD-21** — `../SD-21-campaign-manager-and-persistence/`
  - Predecessor bundle. Campaign Manager + Drive persistence. Some stubs originated here (Epic 2's OAuth/Drive API surface that this bundle simplifies per Decision §4).
- **SD-22** — `../SD-22-content-source-ingest-and-dm-toolkit/`
  - Active bundle on `tranche/5`. SD-23's launch gates on SD-22's closure PR landing on develop. SD-23's `artifacts/` directory and `content-unit-inventory.md` follow SD-22's pattern.

## Deferred research reference

- **Storage Tiers Convergence** — `../../research/storage-tiers-convergence-2026-07-20.md`
  - Deferred decision (Option B: introduce SQLite or similar). Operator ruling 2026-07-20: SD-23 takes Option A; structural convergence deferred to a future bundle.

## Repo-local references

- **`apps/desktop/src/characterHub/characterProgression.ts`** — LevelUpPlan integration surface; ClassSkillPoints + ClassFeatures tables.
- **`apps/desktop/src/campaign/campaignModel.ts`** — Campaign persistence; the seam Epic 4 simplifies.
- **`apps/desktop/src/characterHub/LoadCharacterScreen.tsx:268, 279`** — Delete/Import button no-ops that Epic 6 wires up.
- **`apps/desktop/src/boundary/writeCampaignDriveArtifacts.ts`** — Real Tauri command `write_campaign_drive_artifacts` already wired; Epic 4's rename target.
- **`src/saved_character/local_store.rs`** — `SavedCharacterStore` (file-based persistence; Epic 6 adds `delete` and `import` methods).
- **`src/rules_core/level_up.rs` + `src/rules_core/level_up/*.rs`** — Per-class `LevelUpPlan` Rust modules (SD-20 land; Epic 5's `level_up_character` consumes these).
- **`src/rules_core/character_input.rs`** — `CharacterInput.chosen.ChosenCharacterState` data model.
- **`apps/desktop/src-tauri/src/character_hub.rs`** — Existing Tauri commands; Epic 5 + Epic 6 extend this file.

## Recorded

Authored 2026-07-20 per SD-23 scope-drafting session. Mirrors SD-21's `references/` directory.
