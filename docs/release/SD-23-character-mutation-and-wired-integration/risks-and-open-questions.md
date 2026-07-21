# SD-23 Risks and Open Questions — Character Mutation and Wired Integration

Append-only log. Each entry: number, type (risk / open-question / deferred), summary, mitigation or owner, recorded-by date.

---

## Risks

### R1 — Storage tier referential integrity (latent)

- **Type:** Latent architectural risk, not yet surfaced as a bug.
- **Summary:** SD-23 ships `delete_character` and `import_character` on the file-based `SavedCharacterStore`. Campaigns continue to live in browser localStorage with `partyCharacterIds: string[]` referencing characters by id. Deleting a character silently orphans the id in any campaign referencing it — no cascade, no warning, no foreign key.
- **Mitigation:** SD-23 documents the risk; campaigns keep their in-memory filter join (`CampaignSheet.tsx:248-260`). Future bundle (deferred) addresses the structural convergence: either collapse tiers to a single embedded store or add an explicit referential-integrity layer over the file store.
- **Owner:** Deferred to a future bundle (see `programs/codex/research/storage-tiers-convergence-2026-07-20.md`).
- **Recorded:** 2026-07-20.

### R2 — Stat-field promotion deferred (latent)

- **Type:** Latent feature gap.
- **Summary:** Picker adds equipment/spells as identity-only entries (school/level/description for spells; category/name/cost for equipment). Added items do not affect computed combat stats because the corpus → generated-table refactor that promotes richer mechanical fields (damage dice, AC bonus, range/duration/save) is out of scope for SD-23.
- **Mitigation:** The doctrine's "fully wired" bar is met at the affordance level (real call, real persistence, real refresh). The "stats actually affect combat" bar is a follow-on. Documented in epic-breakdown.md Epic 5.
- **Owner:** Future bundle (post-SD23).
- **Recorded:** 2026-07-20.

### R3 — Tranche/5 still mid-execution at SD-23 launch

- **Type:** Sequencing risk.
- **Summary:** SD-22 closure PR is a precondition for SD-23 cycle launch (pre-launch checklist step 1). If SD-22 closure PR takes longer than expected, SD-23 scope-draft is finalized but cycles don't fire.
- **Mitigation:** Scope-drafting proceeds now (no board writes); cycles gate on the SD-22 closure PR merge to develop. The loop-instruction pre-launch checklist enforces the gate.
- **Owner:** `loop-instruction.md` pre-launch checklist step 1.
- **Recorded:** 2026-07-20.

### R4 — Codebase-wide stub audit may surface substantial accidental debt

- **Type:** Scope discovery risk.
- **Summary:** The Wired Integration Cleanup epic (Epic 3) runs the four-check audit across the active diff against develop. If the audit surfaces more than the 2-3 operator-designed stubs, the cleanup cycles expand. Operator-confirmed cap is "stubs are the exception and must be explicitly approved by the operator" — anything beyond the operator-designed count is accidental debt to remediate.
- **Mitigation:** Per-cycle audit captures findings in the kanban card's comments stream and in `progress.md`. If the audit surfaces a stub not in the Stubs Registry, the cycle remediates (doctrine-compliant fix) before marking `complete`. Operator may grant permanent-exception entries via the Stubs Registry if remediation is technically infeasible.
- **Owner:** Epic 3 cycles; operator override via Stubs Registry.
- **Recorded:** 2026-07-20.

### R5 — SD-23 file-touch contention with SD-22 closure

- **Type:** Concurrency risk during the SD-22 closure window.
- **Summary:** If SD-22 closure PR is mid-merge while SD-23 cycles begin, file-touch partition in `/batch` may collide on shared files (e.g., `apps/desktop/src-tauri/src/main.rs` for Tauri command registration).
- **Mitigation:** Pre-launch checklist step 1 requires SD-22 closure PR is MERGED to develop, not just opened. No overlap window.
- **Owner:** `loop-instruction.md` pre-launch checklist step 1.
- **Recorded:** 2026-07-20.

## Open questions

### OQ1 — Should `CampaignMember.members` array become optional?

- **Type:** Open question.
- **Summary:** Decision §5 deletes `CampaignMember.invited`, leaving `{email}` as the member shape. Whether to keep `members` as a required array (with zero members being a valid campaign) or make it optional needs a UI sanity check. The current Create Campaign screen accepts member emails as input.
- **Resolution:** Decision §5 is sufficient — `members: []` is a valid empty campaign. The Create Campaign UI keeps the email input but the data model doesn't carry `invited` anymore. Resolve by code review during Epic 4 cycle.
- **Recorded:** 2026-07-20.

### OQ2 — Does the `level_up_character` operation need to handle multiclass party composition?

- **Type:** Open question.
- **Summary:** SD-22 / SD-21 work delivered single-class Wizard (SD-21 Epic 6 / SD-22's class coverage) and multiclass stacking (SD-21 Epic 7). Does `level_up_character(character_id, class_id)` need to validate that the class can be added at the requested level (e.g., class features that gate multiclass), or does it just increment without validation?
- **Resolution:** The operation takes the level without validation per the level-up persistence brief 2026-07-20 ("Accept currently no-ops by design"). Future bundle may add validation if multiclass rules require it.
- **Recorded:** 2026-07-20.

## Deferred (out of scope for SD-23)

### D1 — Storage-tiers convergence (Option B: database)

- **Type:** Deferred to a future bundle.
- **Summary:** Structural fix introducing SQLite (or similar) to collapse the three current storage tiers (per-character files, campaign localStorage, Drive markdown mirrors) into a single queryable, relationally-consistent store. Currently 5 cycles of investigation documented at `programs/codex/research/storage-tiers-convergence-2026-07-20.md`.
- **Owner:** Future bundle (research is the seed).
- **Recorded:** 2026-07-20.

### D2 — Stat-field promotion (corpus → generated-table refactor)

- **Type:** Deferred to a future bundle.
- **Summary:** Promote richer mechanical fields (weapon damage/crit, armor AC bonus, spell range/duration/save) from the already-parsed token data into the generated tables/types. Per SD-19 decisions.md, this was deliberately ruled out of scope for SD-19's "reachability" acceptance bar. SD-23's picker/level-up work surfaces this again because added items don't affect computed combat stats.
- **Owner:** Future bundle. Identified criteria: parser → generated-table refactor; combat-stats recompute; spell slot tracking.
- **Recorded:** 2026-07-20.

### D3 — Auto-granting spells/feats at level-up

- **Type:** Deferred to a future bundle.
- **Summary:** The `level_up_character` operation takes the level but does not choose specific known spells or bonus feats. The level-up brief 2026-07-20 explicitly deferred this. Future bundle will need UI for choosing bonus feats and known spells at level-up, plus `level_up_character` extension to accept those choices.
- **Owner:** Future bundle.
- **Recorded:** 2026-07-20.

### D4 — Browser-preview fallback is permanent exception

- **Type:** Permanent exception (operator-granted).
- **Summary:** `characterHubRuntime.ts:17-18` browser-preview fallback (`return buildPreviewListSurface()` when `!hasTauriRuntime()`) is a permanent exception per Stubs Registry entry #0001. No remediation cycle.
- **Owner:** Doctrine (registry entry).
- **Recorded:** 2026-07-20.
