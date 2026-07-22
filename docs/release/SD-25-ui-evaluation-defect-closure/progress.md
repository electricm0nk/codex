# SD-25 — Progress

> **Operating method:** see `./scope-draft.md` and `scripts/workflow-dispatch.sh`. This file is created on cycle 0 of Epic 2 with the deterministic seed. The orchestrator reads `## TODO` + `## DISCOVERED` and dispatches the highest-priority unclaimed item.

This file is the bundle's runtime state. The orchestrator's `progress.md` is the canonical cycle-log + status matrix; the kanban board is the durable receipt; the per-cycle `artifacts/<epic>/<cycle>_cycle_receipt.md` is the per-cycle truth.

## Status matrix (placeholder; populated by cycle 0 of Epic 2)

| Criterion | State | Cycle ID | Commit SHA | Notes |
|---|---|---|---|---|
| 1.1 Source-code identifier audit | complete | 1.1 | `62c4785d098c3c288093f5076130ec44efdce23f` | 404 residual hits, all documented exclusion (real tests/-file citations); see artifacts/epic_1/identifier-audit-cycle_receipt.md |
| 2.1 board reachable | complete | 2.1 | verification-only, no code commit | `hermes kanban boards` confirms `codex-tranche-5` reachable; see artifacts/epic_2/board-reachable-cycle_receipt.md |
| 2.2 branch pushed | complete | 2.2 | verification-only, no code commit | local HEAD matches `origin/tranche/5-3` tip; see artifacts/epic_2/branch-pushed-cycle_receipt.md |
| 2.3 SD-24 closure PR merged | complete | 2.3 | verification-only, no code commit | Tier-1 launch gate; PR #331 `state: MERGED`; see artifacts/epic_2/sd24-pr-merged-cycle_receipt.md |
| 2.4 working tree clean | complete | 2.4 | verification-only, no code commit | remediated (leftover 1.1-adjacent doc edits + Epic 2 receipts batch-committed); see artifacts/epic_2/tree-clean-cycle_receipt.md |
| 2.5 doctrines loaded | complete | 2.5 | verification-only, no code commit | all required doctrine/skill surfaces present and readable; see artifacts/epic_2/doctrines-loaded-cycle_receipt.md |
| 3.1 RuleSystemAdapter trait | complete | 3.1 | `490332d` | trait + inline dyn-dispatch test wired to real functions; see artifacts/epic_3/rule-system-adapter-trait-cycle_receipt.md |
| 3.2 Pf1Adapter extraction | complete | pf1-adapter-extraction | `dccfcb87673e2980b09fc9ba50f7bf68b7945f19` | Pf1Adapter extracted from character_hub.rs; register A5 (revision_id fold-in) + A2 (multiclass level-up dispatch) resolved; impl RuleSystemAdapter for Pf1Adapter completed after 3.1 landed mid-cycle; see artifacts/epic_3/pf1-adapter-extraction-cycle_receipt.md |
| 3.3 StubAdapter | complete | 3.3 | `c41aedc` | StubAdapter implements full RuleSystemAdapter surface; wired-integration-stubs-registry.md entry 0002 landed same commit; see artifacts/epic_3/stub-adapter-cycle_receipt.md |
| 3.4 Tauri command routing | complete | 3.4 | `49097b4` | appendToCharacter/recomputeCharacter/reSaveCharacter accept rule_system_id and dispatch through RuleSystemAdapter (Pf1Adapter for "pf1", StubAdapter otherwise); wired-integration-stubs-registry.md entry 0002 widened same commit; see artifacts/epic_3/command-routing-cycle_receipt.md |
| 3.5 UI panel adapter-aware | complete | 3.5 | `83e8197` | recompute_character wired end-to-end via new boundary/recomputeCharacter.ts + characterHubRuntime.ts's resolveRuleSystemId/buildRecomputeCharacterRequest (register A3); CharacterSheet.tsx's Open/Save/Clone no-op handlers closed (register A4: Open→Load screen nav, Clone→existing cloneCharacter boundary, Save→Recompute); Epic 3 fully closed; see artifacts/epic_3/ui-adapter-aware-cycle_receipt.md |
| 4.1 pcgen-run-character.sh | not-started | — | — | parallel: yes |
| 4.2 pcgen-normalize-output.py | not-started | — | — | parallel: yes |
| 4.3 pcgen_runner_smoke.rs | not-started | — | — | parallel: yes |
| 4.4 verification cycle | not-started | — | — | parallel: no |
| 5.1 corpus_ingest_diagnostic | not-started | — | — | serial |
| 6.1 UI-eval defect cycle shape | not-started | — | — | — |
| 6.2..6.N per-defect | dynamic-pending | — | — | spawned dynamically; not directly dispatchable until spawned |
| 7.1 residue intake | not-started | — | — | — |
| 7.2..7.M per-feature | dynamic-pending | — | — | spawned dynamically; not directly dispatchable until spawned |
| 7.N equipment/spell corpus intake | not-started | — | — | SD-24 carry-forward (register A8, A10–A17); 4 cycles (CRB-desc, APG-desc, APG-spell-text, Bestiary-1); parallel: yes, isolation: worktree |
| 7.O GE-07 pilot-shell-snapshot real implementation | not-started | — | — | SD-24 carry-forward (register A1); BLOCKED on open question Q5 (`risks-and-open-questions.md §4`) — dispatch the design-decision request only until answered |
| 7.P SD-24 documentation-staleness batch | not-started | — | — | SD-24 carry-forward (register §B: B1–B4, B6–B7, B9–B11, B14); docs-only, one batched cycle; Haiku |
| 8.1 Final criterion scan | not-started | — | — | fires LAST; Sonnet |
| 8.2 Architecture closure pipeline | not-started | — | — | fires LAST; Opus |
| 8.3 Release notes | not-started | — | — | fires LAST; Haiku |
| 8.4 Build version increment (→ 0.5.98) | not-started | — | — | fires LAST; Haiku |
| 8.5 PR + merge | not-started | — | — | fires LAST; Sonnet |

## TODO (deterministic seed)

- 4.1–4.4, 5.1, 6.1, 7.1, 7.N (×4 corpus-intake cycles), 7.O (design-decision request first; register A1), 7.P (SD-24 doc batch; register §B), 8.1–8.5

## DONE

- 1.1 Source-code identifier audit — commit `62c4785d098c3c288093f5076130ec44efdce23f` — receipt `artifacts/epic_1/identifier-audit-cycle_receipt.md`
- 2.1 Board reachable (`codex-tranche-5`) — verification-only, no code commit — receipt `artifacts/epic_2/board-reachable-cycle_receipt.md`
- 2.2 Branch pushed (`tranche/5-3` matches origin) — verification-only, no code commit — receipt `artifacts/epic_2/branch-pushed-cycle_receipt.md`
- 2.3 SD-24 closure PR merged (Tier-1 gate; PR #331 `MERGED`) — verification-only, no code commit — receipt `artifacts/epic_2/sd24-pr-merged-cycle_receipt.md`
- 2.4 Working tree clean — verification-only, no code commit — receipt `artifacts/epic_2/tree-clean-cycle_receipt.md`
- 2.5 Doctrines loaded — verification-only, no code commit — receipt `artifacts/epic_2/doctrines-loaded-cycle_receipt.md`
- 3.1 RuleSystemAdapter trait — commit `490332d` — receipt `artifacts/epic_3/rule-system-adapter-trait-cycle_receipt.md`
- 3.2 Pf1Adapter extraction — commit `dccfcb87673e2980b09fc9ba50f7bf68b7945f19` (extraction+A5+A2: `4fe7703`; RuleSystemAdapter impl: `dccfcb8`) — receipt `artifacts/epic_3/pf1-adapter-extraction-cycle_receipt.md`
- 3.3 StubAdapter — commit `c41aedc` — receipt `artifacts/epic_3/stub-adapter-cycle_receipt.md`
- 3.4 Tauri command routing — commit `49097b4` — receipt `artifacts/epic_3/command-routing-cycle_receipt.md`
- 3.5 UI panel adapter-aware — commit `83e8197` — receipt `artifacts/epic_3/ui-adapter-aware-cycle_receipt.md`

## DISCOVERED

- Criterion 3.4 wired real (non-test) call sites to `Pf1Adapter` via each command file's own `resolve_rule_system_adapter` (`Box::new(Pf1Adapter)` for `"pf1"`), confirmed by `cargo build -p codex-desktop` dropping from 7 dead-code warnings to 5 pre-cycle vs. post-cycle. The two `#[allow(dead_code)]` annotations on `Pf1Adapter`'s struct/inherent-impl in `pf1_adapter.rs` are now cosmetically stale (the struct is genuinely used; the attribute is merely inert, not causing a warning either way) but were **not** removed — `pf1_adapter.rs` is outside criterion 3.4's file-touch grant (`cycles/3_4.md` names only the three command files + conditionally `main.rs`). Small, bounded, cosmetic-only; a future cycle that already has `pf1_adapter.rs` in its own grant (or an explicit housekeeping cycle) can drop the two annotations. Not blocking.
- 3.4's own carry-forward note (register A3, from `cycles/3_4.md`) is reconfirmed still true post-cycle: `grep -rn` across `apps/desktop/src/` for any of the three command names still returns nothing — zero frontend callers exist yet. Criterion 3.5 owns closing that gap. **Resolved by 3.5** (see below).
- Criterion 3.5: `revisionId` never crosses the wire to the frontend (`CharacterSummaryDto` / `LoadSavedCharacterResponse` in `character_hub.rs` never expose it, even though every mutate-op advances it server-side). This blocks any honest UI caller of `re_save_character` (which needs `expectedRevisionId` for its write-conflict guard) — register A3 was closed via `recompute_character` instead, which needs no such value. A follow-on cycle touching `character_hub.rs`'s response DTOs should add `revisionId` so `re_save_character` can eventually get a real frontend caller too. See `artifacts/epic_3/ui-adapter-aware-cycle_receipt.md`'s own `## DISCOVERED` section for the full note.

## Cycle log

(empty)

## Open blockers

(empty)

---

*Per `loop-instruction.md §6 step 7`: the orchestrator updates this file in place on every cycle via the concurrent-write protocol (`§5`).*
