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

SD-22 closes when every closure gate below is met AND a `tranche/5 → develop` promotion PR has been merged for the SD-22 work. Each gate is independently verifiable.

1. **Tranche-3 baseline green**. SD-18 chassis done; SD-19 corpus-aware seam + canonical Paizo-table store done. Confirmed by `cargo test --locked` green and the shared progress doc's `## SD-19 cycles` section closed.

2. **APG content-source ingest lands (Epic 3)**. `src/rules_core/rules_tables/apg/` populated with structured-data files for every APG class (Alchemist, Cavalier, Gunslinger, Inquisitor, Magus, Oracle, Summoner, Witch, plus any APG printing additions), every APG spell, and every APG equipment entry. The `RuleSetId::Apg` variant ships. The CRB-side `equipment_id_resolve` and `spell_id_resolve` resolvers accept `RuleSetId::Apg` and resolve APG records correctly.

3. **ACG content-source ingest lands (Epic 4)**. Same shape as APG but for `src/rules_core/rules_tables/acg/`. Every ACG class (Alchemist, Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Swashbuckler, Warpriest, plus any ACG printing additions). The `RuleSetId::Acg` variant ships.

4. **Bestiary 1 content-source ingest lands (Epic 5)**. `src/rules_core/rules_tables/beastiary1/` populated with structured-data files for the 300+ Bestiary 1 monsters distributed across CR bands. The `RuleSetId::Bestiary1` variant ships.

5. **Cross-book resolution works (Epics 3+4+5 + the Q5 SD-21 doctrine)**. APG-only items return `Some` for `RuleSetId::Apg` queries and `None` for `RuleSetId::Crb` queries; ACG-only items return `Some` for `RuleSetId::Acg` and `None` for `RuleSetId::Crb`; Bestiary 1 monsters return `Some` for `RuleSetId::Bestiary1` queries and `None` for `RuleSetId::Crb`. The cross-book priority order (per SD-21 §12 doctrine) is **APG → CRB → ACG → Bestiary1** for the resolver; for content reads from `CharacterSnapshot`, each book is queried independently per the `RuleSetId` parameter.

6. **DM Toolkit lands (Epic 6)**. `src/rules_core/encounters.rs` computes encounter difficulty (Easy / Medium / Hard / Deadly) per PF1's encounter-building rules; `src/rules_core/party_cr.rs` computes party challenge rating per PF1's "Determining Party Strength" rules. Both modules ship with deterministic tests against canonical Paizo examples.

7. **DM-toolkit consumes ingested content (Epic 6 happy path)**. A campaign-shaped fixture (PartySnapshot with mixed-class party members, each from an Epic 3 / Epic 4 ingested class) + a monster-block fixture (MonsterRef from Epic 5's Bestiary 1) → `Encounter::new(...) -> EncounterResult` produces a valid encounter whose difficulty rating matches the canonical Paizo encounter-table result.

8. **MD interop with DM-toolkit output round-trips**. An encounter's `PartySnapshot` + `MonsterRef` shapes, when serialized to a JSON fixture and re-parsed, round-trip byte-identically to the original shape for unchanged fields. Same pattern as SD-21's MD interop gate; no new schema work.

9. **`tranche/5 → develop` promotion PR opened**. Operator opens the promotion PR per the existing cadence. The PR includes the SD-22 commits alongside any in-flight SD-N work, with audit-trail comments per codex-tranche-2-5 respawn-guard pattern.

10. **Epic 1 (Code-Side Identifier Cleanup) fires FIRST**: at `tranche/5` after Epic 1's final cycle, `grep -rE "sd22_|SD22_|Sd22|sd[0-9]+_|SD-[0-9]+-[A-Z][0-9]|Tranche [0-9]+ chassis lane" apps/desktop/src/ apps/desktop/src-tauri/src/ src/rules_core/` returns zero hits in identifier positions. Defensive cleanup; SD-22 doesn't ship new Tauri commands but inherits old identifiers from earlier sessions.

11. **Epic 3 acceptance criteria land (APG content-source ingest)**. The four Epic 3 criteria (6, 7, 8, 9) all move to `complete` on the SD-22 progress matrix before tranche closure. Epic 3 carries APG content-source ingest under SD-22's bundle per the lifecycle routing rule; its closing commits are on `tranche/5` and ship together with the rest of the SD-22 work.

12. **Epic 4 acceptance criteria land (ACG content-source ingest)**. The four Epic 4 criteria (10, 11, 12, 13) all move to `complete`. Same shape as Epic 3 for ACG.

13. **Epic 5 acceptance criteria land (Bestiary 1 content-source ingest)**. The four Epic 5 criteria (14, 15, 16, 17) all move to `complete`.

14. **Epic 6 acceptance criteria land (DM Toolkit)**. The four Epic 6 criteria (18, 19, 20, 21) all move to `complete`. The DM toolkit ships with deterministic tests covering the canonical Paizo examples and a happy-path integration test against ingested content.

15. **Epic 8 (Build Version Numbering) fires before Epic 7**: the three version fields (`apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`) read `"0.5.<current_build>"`; `createSd11WorkbenchStatus.ts` reads `BUILD_PREFIX = 'Codex'` and the template `${BUILD_PREFIX} ${buildVersion}` (rendering the `<major>.<tranche>.<build>` triple from the version files); the test fixtures update to assert/fixture `Codex 0.5.<build>` shape; `docs/SD-22/release-closure-checklist.md` exists with the four-step closure-process using the per-position increment rules. Verification: `grep -E "\"version\"|^version" apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json apps/desktop/src-tauri/Cargo.toml` returns `"0.5.<build>"` for all three; `grep "codex@\|@0\.0" apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` returns zero hits; `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` runs cleanly and refreshes `Cargo.lock`'s embedded version. **Epic 8 lands BEFORE Epic 7** (the version commit is in Epic 7's closure PR's commit history).

16. **Epic 7 (Closure Epilogue) fires LAST**: every criterion (1-30) is `Status: complete` OR `## Open blockers`; the closure PR (`tranche/5 → develop`) is opened via `gh pr create`; worktrees and stale branches are cleaned up; release notes are generated; the *tranche-position* version increment lands (`<major>.<tranche>.<build>` triple, only the *tranche* position increments on tranche promotion; criterion 26's mechanic — the per-CI-build *build* increment is operator-pinned at cycle launch and the per-main-publish *major* increment is a future bundle's epic). Verification: at the moment the loop terminates, `hermes kanban list --board codex-tranche-5 --status done` shows all Epic 7 cycle-cards closed, and the `tranche/5 → develop` PR has been opened against the bundle's actual receipt history.

17. **Epic 9 (Closure Readiness) gates Epic 7**: criterion-31 is `Status: complete`; the `docs/release/SD-22/closure-readiness-report.md` artifact exists and records a clean 30/30 evaluation against `docs/release/SD-22/artifacts/` evidence; any self-heal cycles fired by Epic 9 are recorded in the cycle log with their input-shortfall and output-state; any operator-judgment calls are recorded in `risks-and-open-questions.md` §"Open judgments deferred to next SD"; Epic 7's kanban card has been transitioned from `pending` to `ready` (i.e. Epic 9 has dispatched it). Verification: at the moment Epic 9 is `complete`, Epic 7 becomes eligible at the dispatcher; Epic 7's `hermes kanban list --board codex-tranche-5 --status ready` shows Epic 7's card in `ready` state with the dispatch audit-trail comment chain.

## Verification at closure

The closure posture is reviewable entirely from these surfaces:

- `~/workspace/SD-18-core-rules-breadth-progress.md` — shared progress doc; SD-22 appends under its own `## SD-22 cycles` section.
- `./release-notes.md` — generated release notes.
- `./docs/SD-22/release-closure-checklist.md` — per-position bump-process checklist.
- `git log --oneline tranche/5 -N` — the SD-22 commit history.
- `codex-tranche-5` board — SD-22 cards populated, every epic-card `status=done`, with audit comments per codex-tranche-2-5 respawn-guard pattern.

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

- `decisions.md` — the 3-item decision record (§1 scope, §2 tranche/5 + codex-tranche-5, §3 deferred shape decisions).
- `epic-breakdown.md` — 30 acceptance criteria grouped into 8 epics (Epic 1 Code-Side Identifier Cleanup; Epic 2 Operator Pre-Launch; Epic 3 APG content-source ingest; Epic 4 ACG content-source ingest; Epic 5 Bestiary 1 content-source ingest; Epic 6 DM Toolkit; Epic 7 Closure Epilogue; Epic 8 Build Version Numbering).
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags.
- `technical-design.md` — content-source ingest patterns + DM-toolkit architecture.
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
- `../SD-21/` — sibling bundle; SD-21's Epic 2 (Campaign Manager + Drive) consumes the party-CR math that SD-22's Epic 6 produces.
- `../SD-19/` — Tranche-3 corpus-source ingest pattern (source-book sibling-directory convention).
- `../SD-20/` — per-character rules-engine surface.
