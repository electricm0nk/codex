---
canonical: true
owner: god-emporer
status: approved (operator review 2026-07-15; operator directives 2026-07-17 expanded scope to APG + ACG; operator clarification 2026-07-18: "ACG, APG are the two advanced guides"; branch + board pinned 2026-07-18 to tranche/5 / codex-tranche-5; 8 epics / 30-criteria final shape; bundle marked planning-ready)
date: 2026-07-15
canonical_branch: tranche/5 (operator directive 2026-07-18)
kanban_board: codex-tranche-5 (operator directive 2026-07-18)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/decisions.md
mirror_of: /home/ubuntu/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md
---

# SD-22 — Acceptance and Verification

## Closure gates (mandatory)

SD-22 closes when every closure gate below is met AND a `tranche/5 → develop` promotion PR has been merged for the SD-22 work. Each gate is independently verifiable. **Per operator directive 2026-07-19, every cycle below must satisfy the red-green TDD mandate** (Step 4 RED phase, Step 5 GREEN phase, both persisted in the cycle artifact at `docs/release/SD-22/artifacts/<cycle_artifact_path>` per `corpus-source-inventory.md` §6 contract). A criterion whose cycle artifact is missing the RED phase is a Bucket-B / Bucket-C shortfall and Epic 9's evaluator treats it as a self-heal trigger.

### Per-criterion closure gate → artifact map

Every criterion below has an associated cycle artifact path (per `corpus-source-inventory.md`). Epic 9 evaluates the artifact path's existence + content as part of criterion-31's 30/30 eval.

| Criterion | Cycle artifact (under `docs/release/SD-22/artifacts/`) | Required sections in the artifact |
|---|---|---|
| 1 (Epic 1 audit run) | `epic_1/identifier_audit_red.log`, `epic_1/identifier_audit_green.log` | Red-phase grep output, Green-phase grep output |
| 2 (Epic 1 per-cycle tests) | `epic_1/per_rename_<old-id>_to_<new-id>_cycle_receipt.md` | Red-phase test output, Green-phase test output |
| 3 (Epic 2 board-pinned) | `epic_2/codex_tranche_5_pin_cycle_receipt.md` | Operator verification output |
| 4 (Epic 2 branch-pushed) | `epic_2/tranche_5_push_cycle_receipt.md` | Operator verification output |
| 5 (Epic 2 no claude in-flight) | `epic_2/no_claude_in_flight_cycle_receipt.md` | Operator verification output |
| 6 (Epic 3 apg mod.rs) | `apg/mod_rs_cycle_receipt.md` | Red/Green phase output |
| 7 (Epic 3 per-APG-class — 8 classes) | `apg/class_alchemist_cycle_receipt.md` × 8 (one per class) | Red/Green phase output + class-feature checklist |
| 8 (Epic 3 cross-book APG-CRB) | `apg/cross_book_apg_crb_invariants_cycle_receipt.md` | Red/Green + `RuleSetId::Apg::Some(…)` + `RuleSetId::Crb::None` assertions |
| 9 (Epic 3 spell/equipment resolution) | `apg/spell_list_cycle_receipt.md`, `apg/equipment_tables_cycle_receipt.md` | Red/Green phase |
| 10 (Epic 4 acg mod.rs) | `acg/mod_rs_cycle_receipt.md` | Red/Green phase output |
| 11 (Epic 4 per-ACG-class — 10 classes) | `acg/class_alchemist_cycle_receipt.md` × 10 | Red/Green + class-feature checklist |
| 12 (Epic 4 cross-book ACG) | `acg/cross_book_acg_invariants_cycle_receipt.md` | Red/Green + cross-book assertions |
| 13 (Epic 4 ACG spells) | `acg/spell_list_cycle_receipt.md`, `acg/equipment_tables_cycle_receipt.md` | Red/Green phase |
| 14 (Epic 5 beastiary1 mod.rs) | `beastiary1/mod_rs_cycle_receipt.md` | Red/Green phase |
| 15 (Epic 5 per-monster-block-subset) | `beastiary1/subset_<NN>_cycle_receipt.md` (one per subset) | Red/Green + monster-list checklist |
| 16 (Epic 5 cross-book Bestiary 1) | `beastiary1/cross_book_invariants_cycle_receipt.md` | Red/Green + cross-book assertions |
| 17 (Epic 5 DM-toolkit consumption) | `beastiary1/dm_toolkit_consumption_cycle_receipt.md` | Red/Green + encounter-difficulty result |
| 18 (Epic 6 encounters.rs) | `dm_toolkit/encounters_cycle_receipt.md` | Red/Green + the five deterministic test cases per `corpus-source-inventory.md` §4.1 |
| 19 (Epic 6 party_cr.rs) | `dm_toolkit/party_cr_cycle_receipt.md` | Red/Green + the canonical Paizo examples |
| 20 (Epic 6 DM-toolkit deterministic tests) | `dm_toolkit/deterministic_tests_cycle_receipt.md` | All five Paizo examples pass |
| 21 (Epic 6 happy-path integration) | `dm_toolkit/happy_path_integration_cycle_receipt.md` | PartySnapshot + MonsterRef → EncounterResult output |
| 22 (Epic 7 final criterion scan) | `epic_7/final_scan_cycle_receipt.md` | status matrix scan output |
| 23 (Epic 7 closure PR) | `epic_7/closure_pr_cycle_receipt.md` | `gh pr create` output + PR URL + commit SHAs |
| 24 (Epic 7 worktree cleanup) | `epic_7/worktree_branch_cleanup_cycle_receipt.md` | `git worktree list` post-cleanup, `git branch` listing |
| 25 (Epic 7 release notes) | `epic_7/release_notes.md` | Generated release-notes file |
| 26 (Epic 7 tranche version increment) | `epic_7/tranche_version_increment_cycle_receipt.md` | Version-file diff post-increment |
| 27 (Epic 8 three-version-fields) | `epic_8/three_version_fields_cycle_receipt.md` | Red/Green + grep output |
| 28 (Epic 8 build-label format) | `epic_8/build_label_format_cycle_receipt.md` | Red/Green + build-label rendering output |
| 29 (Epic 8 release-closure-checklist) | `docs/SD-22/release-closure-checklist.md` | The four-step closure-process checklist |
| 30 (Epic 8 per-cycle tests) | `epic_8/per_cycle_tests_cycle_receipt.md` | `cargo test --locked` + clippy output |
| 31 (Epic 9 closure readiness) | `closure-readiness-report.md` | Artifact-evidence survey output + cycle log + open-judgments log |

### Gates (verbatim)

1. **Tranche-3 baseline green**. SD-18 chassis done; SD-19 corpus-aware seam + canonical Paizo-table store done. Confirmed by `cargo test --locked` green and the shared progress doc's `## SD-19 cycles` section closed.

2. **APG content-source ingest lands (Epic 3)**. `src/rules_core/rules_tables/apg/` populated with structured-data files for every APG class (Alchemist, Cavalier, Gunslinger, Inquisitor, Magus, Oracle, Summoner, Witch, plus any APG printing additions), every APG spell, and every APG equipment entry. The `RuleSetId::Apg` variant ships. The CRB-side `equipment_id_resolve` and `spell_id_resolve` resolvers accept `RuleSetId::Apg` and resolve APG records correctly. **Per-class artifacts:** 8 per-class `*_cycle_receipt.md` files under `docs/release/SD-22/artifacts/apg/` per the per-criterion artifact map above. **Cross-book invariants:** `apg/cross_book_apg_crb_invariants_cycle_receipt.md` asserts each APG key returns `Some` for `RuleSetId::Apg` and `None` for `RuleSetId::Crb`.

3. **ACG content-source ingest lands (Epic 4)**. Same shape as APG but for `src/rules_core/rules_tables/acg/`. Every ACG class (Alchemist, Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Swashbuckler, Warpriest, plus any ACG printing additions). The `RuleSetId::Acg` variant ships. **Per-class artifacts:** 10 per-class cycle-receipt files. **Cross-book:** `acg/cross_book_acg_invariants_cycle_receipt.md`.

4. **Bestiary 1 content-source ingest lands (Epic 5)**. `src/rules_core/rules_tables/beastiary1/` populated with structured-data files for the 300+ Bestiary 1 monsters distributed across CR bands. The `RuleSetId::Bestiary1` variant ships. **Per-subset artifacts:** one `subset_<NN>_cycle_receipt.md` per monster-block subset (default and operator-pinned subset count). **Cross-book:** `beastiary1/cross_book_invariants_cycle_receipt.md`.

5. **Cross-book resolution works (Epics 3+4+5 + the Q5 SD-21 doctrine)**. APG-only items return `Some` for `RuleSetId::Apg` queries and `None` for `RuleSetId::Crb` queries; ACG-only items return `Some` for `RuleSetId::Acg` and `None` for `RuleSetId::Crb`; Bestiary 1 monsters return `Some` for `RuleSetId::Bestiary1` queries and `None` for `RuleSetId::Crb`. The cross-book priority order (per SD-21 §12 doctrine) is **APG → CRB → ACG → Bestiary1** for the resolver; for content reads from `CharacterSnapshot`, each book is queried independently per the `RuleSetId` parameter. **Verification:** `apg/cross_book_apg_crb_invariants_cycle_receipt.md`, `acg/cross_book_acg_invariants_cycle_receipt.md`, `beastiary1/cross_book_invariants_cycle_receipt.md` all assert cross-book invariants with RED phase green and GREEN phase assertion-clean.

6. **DM Toolkit lands (Epic 6)**. `src/rules_core/encounters.rs` computes encounter difficulty (Easy / Medium / Hard / Deadly) per PF1's encounter-building rules; `src/rules_core/party_cr.rs` computes party challenge rating per PF1's "Determining Party Strength" rules. Both modules ship with deterministic tests against canonical Paizo examples. **Per-fixture artifacts:** `dm_toolkit/encounters_cycle_receipt.md` and `dm_toolkit/party_cr_cycle_receipt.md` carry all five canonical test cases per `corpus-source-inventory.md` §4.1.

7. **DM-toolkit consumes ingested content (Epic 6 happy path)**. A campaign-shaped fixture (PartySnapshot with mixed-class party members, each from an Epic 3 / Epic 4 ingested class) + a monster-block fixture (MonsterRef from Epic 5's Bestiary 1) → `Encounter::new(...) -> EncounterResult` produces a valid encounter whose difficulty rating matches the canonical Paizo encounter-table result. **Verification:** `dm_toolkit/happy_path_integration_cycle_receipt.md` carries the integration test's RED → GREEN transition with the actual `EncounterResult` output.

8. **MD interop with DM-toolkit output round-trips**. An encounter's `PartySnapshot` + `MonsterRef` shapes, when serialized to a JSON fixture and re-parsed, round-trip byte-identically to the original shape for unchanged fields. Same pattern as SD-21's MD interop gate; no new schema work. **Verification:** `dm_toolkit/json_roundtrip_cycle_receipt.md` exists with the round-trip pair.

9. **`tranche/5 → develop` promotion PR opened**. Operator opens the promotion PR per the existing cadence. The PR includes the SD-22 commits alongside any in-flight SD-N work, with audit-trail comments per codex-tranche-2-5 respawn-guard pattern. **Verification:** `epic_7/closure_pr_cycle_receipt.md` carries the `gh pr create` output and the PR URL.

10. **Epic 1 (Code-Side Identifier Cleanup) fires FIRST**: at `tranche/5` after Epic 1's final cycle, `grep -rE "sd22_|SD22_|Sd22|sd[0-9]+_|SD-[0-9]+-[A-Z][0-9]|Tranche [0-9]+ chassis lane" apps/desktop/src/ apps/desktop/src-tauri/src/ src/rules_core/` returns zero hits in identifier positions. Defensive cleanup; SD-22 doesn't ship new Tauri commands but inherits old identifiers from earlier sessions. **Verification:** `epic_1/identifier_audit_red.log` (showing the dirty-identifier hits before rename) and `epic_1/identifier_audit_green.log` (showing zero hits post-rename) both exist.

11. **Epic 3 acceptance criteria land (APG content-source ingest)**. The four Epic 3 criteria (6, 7, 8, 9) all move to `complete` on the SD-22 progress matrix before tranche closure. Epic 3 carries APG content-source ingest under SD-22's bundle per the lifecycle routing rule; its closing commits are on `tranche/5` and ship together with the rest of the SD-22 work. **Verification:** per-class artifacts (10) + cross-book-invariant artifact (1).

12. **Epic 4 acceptance criteria land (ACG content-source ingest)**. The four Epic 4 criteria (10, 11, 12, 13) all move to `complete`. Same shape as Epic 3 for ACG. **Verification:** 12 Epic-4 artifacts.

13. **Epic 5 acceptance criteria land (Bestiary 1 content-source ingest)**. The four Epic 5 criteria (14, 15, 16, 17) all move to `complete`. **Verification:** per-subset artifacts (default 8-12) + cross-book + DM-toolkit-consumption artifacts.

14. **Epic 6 acceptance criteria land (DM Toolkit)**. The four Epic 6 criteria (18, 19, 20, 21) all move to `complete`. The DM toolkit ships with deterministic tests covering the canonical Paizo examples and a happy-path integration test against ingested content. **Verification:** 4 Epic-6 artifacts (encounters, party_cr, deterministic tests, happy-path integration).

15. **Epic 8 (Build Version Numbering) fires before Epic 7**: the three version fields (`apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`) read `"0.5.<current_build>"`; `createSd11WorkbenchStatus.ts` reads `BUILD_PREFIX = 'Codex'` and the template `${BUILD_PREFIX} ${buildVersion}` (rendering the `<major>.<tranche>.<build>` triple from the version files); the test fixtures update to assert/fixture `Codex 0.5.<build>` shape; `docs/SD-22/release-closure-checklist.md` exists with the four-step closure-process using the per-position increment rules. **Verification:** `grep -E "\"version\"|^version" apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json apps/desktop/src-tauri/Cargo.toml` returns `"0.5.<build>"` for all three; `grep "codex@\|@0\.0" apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` returns zero hits; `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` runs cleanly and refreshes `Cargo.lock`'s embedded version. **Epic 8 lands BEFORE Epic 7** (the version commit is in Epic 7's closure PR's commit history). **Verification:** `epic_8/three_version_fields_cycle_receipt.md`, `epic_8/build_label_format_cycle_receipt.md`, `epic_8/per_cycle_tests_cycle_receipt.md`.

16. **Epic 7 (Closure Epilogue) fires LAST**: every criterion (1-30) is `Status: complete` OR `## Open blockers`; the closure PR (`tranche/5 → develop`) is opened via `gh pr create`; worktrees and stale branches are cleaned up; release notes are generated; the *tranche-position* version increment lands (`<major>.<tranche>.<build>` triple, only the *tranche* position increments on tranche promotion; criterion 26's mechanic — the per-CI-build *build* increment is operator-pinned at cycle launch and the per-main-publish *major* increment is a future bundle's epic). Verification: at the moment the loop terminates, `hermes kanban list --board codex-tranche-5 --status done` shows all Epic 7 cycle-cards closed, and the `tranche/5 → develop` PR has been opened against the bundle's actual receipt history. **Verification:** 5 Epic-7 artifacts (final_scan, closure_pr, worktree_branch_cleanup, release_notes, tranche_version_increment).

17. **Epic 9 (Closure Readiness) gates Epic 7**: criterion-31 is `Status: complete`; the `docs/release/SD-22/closure-readiness-report.md` artifact exists and records a clean 30/30 evaluation against `docs/release/SD-22/artifacts/` evidence; any self-heal cycles fired by Epic 9 are recorded in the cycle log with their input-shortfall and output-state; any operator-judgment calls are recorded in `risks-and-open-questions.md` §"Open judgments deferred to next SD"; Epic 7's kanban card has been transitioned from `pending` to `ready` (i.e. Epic 9 has dispatched it). Verification: at the moment Epic 9 is `complete`, Epic 7 becomes eligible at the dispatcher; Epic 7's `hermes kanban list --board codex-tranche-5 --status ready` shows Epic 7's card in `ready` state with the dispatch audit-trail comment chain.

## Verification at closure

The closure posture is reviewable entirely from these surfaces:

- `~/workspace/SD-18-core-rules-breadth-progress.md` — shared progress doc; SD-22 appends under its own `## SD-22 cycles` section.
- `~/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/release-notes.md` — generated release notes.
- `~/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/docs/SD-22/release-closure-checklist.md` — per-position bump-process checklist.
- `git log --oneline tranche/5 -N` — the SD-22 commit history.
- `codex-tranche-5` board — SD-22 cards populated, every epic-card `status=done`, with audit comments per codex-tranche-2-5 respawn-guard pattern.
- `docs/release/SD-22/artifacts/` — per-cycle receipt artifacts (the **load-bearing surfaces for Epic 9's 30/30 evaluation**).
- `docs/release/SD-22/corpus-source-inventory.md` — the per-criterion row that named each cycle's expectation.

Operator's first action on return from a multi-day run: read the `## SD-22 cycles` section of the shared progress doc; if empty, gates 1-16 above are the entire verification.

## What does *not* gate closure

- Loop's cycle log size (10 cycles or 100; criterion is the criterion, not volume).
- Number of self-heals.
- Whether some epic-cards land as documentation-only versus full code-bearing (per the eligibility check).
- Whether the DM-toolkit GUI screens are merged before SD-22's loop cycles finish — the engine-side DM toolkit is independent of the GUI-side's merge status.
- Tier-5 / ultimate-line book ingest (Ultimate Combat / Ultimate Magic / etc.) — NOT in scope per operator clarification 2026-07-18.
- DM-toolkit GUI screens — out of scope for SD-22; if a GUI-bundle is needed, that's `SD-23`.
- Identifier-cleanup directory-tree rename (`apps/desktop/src/sd16/` → `apps/desktop/src/update/`) — recorded as out-of-scope for Epic 1; future bundle's epic.

## Cross-reference

- `corpus-source-inventory.md` — load-bearing content inventory; per-criterion Rust-module / test-fixture / cycle-artifact / `RuleSetId` four-tuple; the per-criterion artifact map's source-of-truth.
- `decisions.md` — the 4-item decision record (§1 scope, §2 tranche/5 + codex-tranche-5, §3 deferred shape decisions, §4 Epic 9 — Closure Readiness added 2026-07-19).
- `epic-breakdown.md` — 31 acceptance criteria grouped into 9 epics (Epic 1 Code-Side Identifier Cleanup; Epic 2 Operator Pre-Launch; Epic 3 APG content-source ingest; Epic 4 ACG content-source ingest; Epic 5 Bestiary 1 content-source ingest; Epic 6 DM Toolkit; Epic 7 Closure Epilogue; Epic 8 Build Version Numbering; Epic 9 — Closure Readiness, added 2026-07-19).
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags.
- `technical-design.md` — content-source ingest patterns + DM-toolkit architecture.
- `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md` — canonical handoff.
- `artifacts/README.md` — evidence-ledger index; per-receipt contract documentation.
- `artifacts/corpus/` — on-disk source-shape stubs for Epic 3/4/5/6 ingest cycles (APG/ACG/Bestiary 1 spell/equipment tables); see `artifacts/corpus/README.md` for the schema-of-record and the operator-supplied swap procedure.
- `ingest.md` — canonical process doctrine for content-source ingest (per operator directive 2026-07-19); per-cycle RED → GREEN → cycle-artifact → commit pipeline that every Epic 3/4/5/6 cycle reads before the GREEN phase.
- `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-loop-instruction.md` — loop body; carries the per-epic Step 4 (RED) / Step 5 (GREEN) shape per the operator-pinned 2026-07-19 red-green TDD mandate.
- `~/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/` — sibling bundle; SD-21's Epic 2 (Campaign Manager + Drive) consumes the party-CR math that SD-22's Epic 6 produces.
- `~/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/` — Tranche-3 corpus-source ingest pattern (source-book sibling-directory convention).
- `~/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/` — per-character rules-engine surface.
