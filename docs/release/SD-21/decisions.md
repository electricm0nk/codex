---
title: SD-21 — Decision Record ("Why we did that")
status: approved (operator review 2026-07-16; changes noted: launches on tranche/5 branch, kanban board codex-tranche-5, Q1–Q5 PINNED, override flags A–D defaulted, §15 tranche/5 branch decision added; bundle marked approved with operator directives 2026-07-16)
date: 2026-07-15
companion_to: /home/ubuntu/workspace/SD-21-campaign-manager-and-persistence-scope-draft.md
---

# SD-21 — Decision Record

This file captures the deliberate choices made in the SD-21 planning conversation on 2026-07-15. Each item has the decision, the displaced alternative(s) with reasons they lost, and the reason the winner won. A future session asking "why is SD-21 shaped this way" should find the answer here without re-litigating it.

SD-21 exists because the operator named the gaming pod as the actual product unit ("we are designing a system by which a user can build and manage their characters in whatever games they are playing... typically that there will be many characters and game systems involved since those don't live forever... these games are not played solo. They are group activities. People play with other people in a party. The parties are refereed by a game master. This whole group makes up an active game. Our focus should expand to supporting the entire gaming pod."), then specified the campaign epic's natural seam: "i think this gets split. the campaign manager can be done as a part of SD-21, possibly in parallel with SD-20." SD-21 is the product-surface epic that turns per-character rules-engine completeness into "the gaming pod is usable in practice."

## 1. SD-21 is a top-level spec-domain bundle, not an epic inside SD-20

**Decision:** SD-21 lives at `programs/codex/requirements/SD-21-campaign-manager-and-persistence/` as the next program-level spec-domain bundle after SD-20. Its content is a single product-surface deliverable (the gaming pod: campaigns with parties + DM, persisted to a shared backing store with markdown sync, including the APG and ACG content catalogues).

**Displaced alternatives:**

- *Campaign manager as Epic 1 inside SD-20.* Rejected by the operator on 2026-07-15 ("i think this gets split. the campaign manager can be done as a part of SD-21").
- *One bundle per epics* (campaign-manager, APG, ACG, Drive adapter as four separate spec-domain bundles). Rejected for the same reason SD-18, SD-19, SD-20 are single bundles — the gaming pod is one product-surface deliverable.

**Reason:** Per operator directive 2026-07-15 ("i think this gets split. the campaign manager can be done as a part of SD-21, possibly in parallel with SD-20") and the SD-N convention ("SD is spec domain, it's top level and done numerically"): the campaign + Drive + APG + ACG scope is one product-surface deliverable (the gaming pod, usable in practice). Per the campaign-manager epic being parallelizable with SD-20, SD-21 carries its own lifecycle and doesn't gate SD-20.

## 2. SD-21 reads from SD-19's table store directly, not from SD-20's epic outputs

**Decision:** SD-21's spell/class/equipment/resolution lookups go through SD-19's `src/rules_core/rules_tables/crb/`, `apg/`, and `acg/` directories (populated by SD-19's foundation slice per SD-19 §9 source-book subdirectories). SD-21 does NOT depend on SD-20's per-epic compute outputs.

**Displaced alternative:** SD-21 waits for SD-20 to close, then reads SD-20's per-character `PilotReceipt` for every party member of a campaign.

**Reason:** SD-21 can run in parallel with SD-20 (per operator directive). For a party-level `CampaignSnapshot`, the campaign manager needs spell names, class names, equipment names, and basic stats — all readable from SD-19's table store + per-character chassis. Per-character spell effects, feat effects, damage totals, skill allocations arrive after SD-20 closes; the `CampaignSnapshot` shape accommodates "partial character summaries" during the parallel window and "full character receipts" after SD-20 closes. The shape accommodates both states.

## 3. SD-21's product-surface deliverable is campaign-manager-first; APG + ACG ingestion is equal-weight but separate

**Decision:** SD-21 ships three epics: campaign manager + Drive persistence (epic 1), APG ingestion (epic 2), ACG ingestion (epic 3). APG and ACG could ship as a combined "non-CRB ingestion" epic, but the operator named them separately and they have different table-store shapes (APG adds classes like Alchemist that have unique features; ACG adds classes like Summoner; both add spells and equipment).

**Displaced alternative:** A single "non-CRB ingestion" epic that lands APG + ACG together.

**Reason:** The operator named APG and ACG separately ("with SD-21, i want to include ingesting the Advanced Players Guide and the Advanced Class Guide"). They have distinct content shapes that benefit from independent per-class cycles.

## 4. Drive persistence is the operator's choice of backend; the engine is backend-agnostic

**Decision:** The engine exposes a backend-agnostic persistence boundary (`load_campaign(campaign_id) -> CampaignSnapshot`, `save_campaign(campaign_id, snapshot) -> Result`). Drive is one backing-store implementation; other implementations (Dropbox, Syncthing, local file system) become additional backing-store adapters. Per the operator's earlier framing: "i supposed this could be google drive, dropbox, syncthing - it doesnt matter too much."

**Displaced alternative:** Drive is hardcoded into the engine's persistence layer.

**Reason:** Per operator framing: "a directory structure would be built for each campaign and members of the party could be granted permission to each campaign." The engine doesn't know which backing store is in use. The Drive adapter handles OAuth, folder selection, conflict resolution, and markdown sync; the engine just calls `save_campaign(...)` and gets back Result.

## 5. Campaign manager's data is canonical on the *campaign*, not on individual characters

**Decision:** A campaign owns its `CampaignSnapshot`. Characters that join a campaign are referenced by their character_id (UUID or similar); the campaign stores per-member summaries (party-stat-block-at-a-glance, level/class/race snapshot, current HP). A character may be a member of multiple campaigns. Updates to the character (via SD-20's character-sheet flow) propagate to each campaign the character is a member of on next save, via per-campaign reconciliation logic.

**Displaced alternative:** Each character is a member of exactly one campaign; updating a character updates the campaign automatically.

**Reason:** A user might play the same character in multiple campaign settings (a Pathfinder PF1 character might appear in a one-shot, a chronicle, and a homebrew campaign). Canonical-on-character with fan-out-to-campaigns lets the same character serve multiple campaigns without data duplication. The fan-out reconciliation is a per-campaign snapshot job — the engine doesn't auto-sync, it queues for the next campaign-save.

## 6. Markdown sync is the interop boundary for non-engine consumers

**Decision:** The Drive adapter (and any other backend adapter) serializes `CampaignSnapshot` to a directory of markdown files: one `campaign.md` for the campaign metadata, one `member/<character_id>.md` per party member (with their snapshot stats as frontmatter + a short description as body), one `resources.md` for the party-resource pool (gold, items, XP), one `adventure_log/<entry_id>.md` per DM-authored log entry, one `maps/<map_name>.md` per map, one `wiki/<page_name>.md` per wiki page.

**Displaced alternative:** Single monolithic JSON file per campaign, machine-friendly but human-unfriendly.

**Reason:** Per operator framing: "things like party treasure, story recaps, maps, and other information provided by the game master all need a home." Each of those needs human authoring (DMs edit campaign notes between sessions; party treasures get marked up by a player on paper; maps get linked from a hand-drawn diagram). Markdown is the format that survives hand-authoring, version control, and Obsidian-style indexing. The interop boundary with Obsidian (which the operator already uses, per persistent memory: "much of this is done with obsidian portal today") is natural — Obsidian reads markdown natively.

## 7. Conflict resolution is last-write-wins with explicit conflict logs

**Decision:** When two devices edit the same campaign file and Drive sync surfaces a conflict, the engine saves both copies to a conflict log (`campaign_conflicts/<timestamp>/<file>.local.md` and `<file>.remote.md`) and loads the local version as the active state. The DM resolves the conflict manually by reading both, picking the right one, and deleting the other.

**Displaced alternative:**

- *Three-way merge* (common ancestor + two sides). Markdown merge is lossy; three-way merge on prose produces incoherent text.
- *Operational-transform or CRDT* style real-time merging. Overkill for the use case; character sheets are session-bound, not collaborative in real time.
- *Hard-locking per file.* Prevents the conflict from surfacing but blocks casual two-device edits.

**Reason:** Per the operator's likely use case (single DM + 2-4 players, each editing between sessions on different devices, syncing occasionally), the conflict rate is low and the conflicts that do happen are bounded to whole files. Last-write-wins-with-explicit-conflict-logs is the right tradeoff: surfaces the conflict immediately (no silent loss) but doesn't force a complex merge strategy on the user.

## 8. Campaign-vs-character data ownership: a character is canonical on its own state; a campaign is canonical on party composition + shared resources

**Decision:** The character-sheet flow (SD-20's closure path) is canonical for everything about a single character (stats, feats, spells, equipment). The campaign manager is canonical for party composition (who's in the party), shared resources (party gold, shared inventory, XP), and DM-authored content (adventure log, maps, wiki, NPC notes). A campaign references characters by character_id but does not own their state.

**Displaced alternative:** Campaign manager owns character state, single-character sheet is a derived view.

**Reason:** This is the natural shape of tabletop RPG data — players own their characters, the DM owns the table state, and the campaign is the bridge. Inverting this (campaign owns character state) means a character can't leave a campaign without losing state, which is wrong for the "same character in multiple campaigns" use case from decision §5.

## 9. SD-22 (tranche-5+ content-source ingest + DM toolkit) follows downstream of SD-20 and SD-21 (operator directive 2026-07-17, scope expanded 2026-07-17)

**Decision (original 2026-07-15):** per operator directive 2026-07-15 ("With SD-22, we should include at least beastiary 1 rules"), SD-22 was named for the DM toolkit + encounter builder + Bestiary 1 ingestion work; that bundle is downstream of SD-20 (per-character rules engine) and SD-21 (campaign manager with party-CR math). SD-21's epic 1 (campaign manager) consumes the party-CR math that SD-22 will provide.

**Decision (scope expansion 2026-07-17):** per operator directive 2026-07-17 — first "those [advanced guides] need to move to SD-22" and then "APG and ACG [also] have moved to SD-22" — SD-22's scope expanded to include the **full content-source ingest lane**: APG (Advanced Player's Guide classes, spells, equipment, races), ACG (Advanced Class Guide classes, spells, equipment), the two Ultimate-line advanced guides (Ultimate Combat, Ultimate Magic) currently referenced from `src/rules_core/pilot_compute.rs` as `advanced_guide` paths, Bestiary 1 monster data, plus the DM toolkit (encounter builder, party-CR math, diagnostic surfaces). SD-22 owns *every* content-source ingest lane going forward; SD-21 *no longer* owns any content-source ingest work. SD-21 reads from SD-19's `rules_tables/crb/` only.

**Displaced alternative (advanced guides + APG + ACG stay in SD-21):** SD-21 attempts to carry four content-source lanes (APG + ACG + two advanced guides + Bestiary 1) alongside its per-character-rules-engine work. Per the operator's directive and the lifecycle-routing rule, content that lands in `tranche/3` chassis substrate and shapes the *next* release belongs to whichever bundle is currently shaping that release — but bundle-scope fragmentation across 11+ epics makes the bundle too large to land cleanly in a single release. Concentrating content-source ingest in SD-22 keeps SD-21 focused on the per-character-rules-engine-completeness lane (CRB-only data) with multiclass + the three governance epics (Identifier Cleanup, Closure Epilogue, Build Version) plus the Update UI bug fix.

**Reason (operator-pinned 2026-07-17):** the operator's stated framing was direct ("those need to move to SD-22" for the advanced guides; "APG and ACG are also moving" for the core books). The two reasons the move makes doctrinal sense: (1) content-source ingest is a coherent lane on its own — APG + ACG + advanced guides + Bestiary 1 are all "load book content into the rules-core corpus" work, and SD-22's bundle can grow that as its primary surface; (2) SD-22 will need its own tranche + kanban board anyway since its scope is now substantial (DM toolkit + party-CR + content-source ingest for multiple book lines + the Future apparatus). Carving content-source ingest out of SD-21 lets SD-21 ship a focused per-character-rules-engine-completeness release without pulling SD-22's content work forward.

**Operational consequence for SD-21's epic-breakdown.** SD-21's epic decomposition now has **7 epics: Identifier Cleanup (Epic 1), Campaign Manager + Drive (Epic 2), Update UI bug (Epic 3), Closure Epilogue (Epic 4), Build Version Numbering (Epic 5), Single-class Coverage Completion (Epic 6), Multiclass Stacking (Epic 7)**. The four APG/ACG criteria that previously sat in SD-21's Epic 2 + Epic 3 sections are deleted from `epic-breakdown.md` entirely — they now live in SD-22's bundle. SD-21's Epic 2 reads from SD-19's `rules_tables/crb/` only. Epic 3's `release-notes fetch path` work in `apps/desktop/src/sd16/update/fetch.ts` is unaffected — that's a desktop-shell Update UI artifact, not content-source ingest work.

**Operational consequence for SD-22.** SD-22's bundle directory `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/` is created as a planning-only source STC (same shape as SD-16's `programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/`). No implementation work begins until SD-22's bundle is operator-reviewed and a launch-branch decision is recorded (alongside SD-21's branch/board directive at §11). Bestiary 1 rules + party-CR math + at least one advanced-guide book line are the minimum acceptance criteria when SD-22 fires.

## 10. Cross-bundle dependency: SD-21 reads from SD-19's table store + chassis; auto-upgrade on SD-20 close (operator directive 2026-07-16)

**Decision:** SD-21's `CampaignSnapshot.CharacterSummary` reads from SD-19's table store + per-character chassis during the parallel window (SD-20 in flight). When SD-20 closes, the `CharacterSummary` auto-upgrades to full-detail `PilotReceipt` fields. The auto-upgrade is silent at the engine level and surfaces as a banner in the GUI, not a save-blocker. This is tightly coupled to SD-20's Q4 (PINNED: SD-21 owns `CampaignSnapshot`; SD-20 doesn't grow a campaign-shape view) and SD-21's Q4 (PINNED: auto-upgrade as banner not blocker, per operator directive 2026-07-16).

**Displaced alternative:** SD-21 refuses to save a campaign containing a character whose latest sheet is post-SD-20, forcing the user to re-link the character.

**Reason:** Per operator directive 2026-07-16: status quo (auto-upgrade). Auto-upgrade is the right behavior because the alternative forces operator action at exactly the moment when SD-21 should be invisible — the DM doesn't care about the SD-20 transition; they care that the campaign reflects their characters. The `CharacterSummary` shape has a `schema_version: u8` field (or equivalent) so the GUI can surface "this snapshot was taken before SD-20 epic 1 closed" as a banner when the field is missing. Recorded as `risks-and-open-questions.md` Q4.

Mirrored as `risks-and-open-questions.md` Q4 (PINNED: status quo, auto-upgrade as banner not blocker).

## 11. SD-21 launches on `tranche/4-1` branch with `codex-tranche-4-1` board (operator directive 2026-07-17, originally 2026-07-16 as `tranche/5` / `codex-tranche-5`)

**Decision (original 2026-07-16):** SD-21 commits originally were directed to a new integration branch `tranche/5`, separate from `tranche/3` (chassis lane, SD-18+SD-19) and `tranche/4` (per-character-rules-engine lane, SD-20). The `codex-tranche-5` board was created as the kanban surface for SD-21 cycles.

**Decision (branch flip 2026-07-17):** per operator directive 2026-07-17 ("SD-21 is going to be tranche/4-1"), SD-21's integration branch is now **`tranche/4-1`** (not `tranche/5`). The `codex-tranche-4-1` kanban board replaces `codex-tranche-5`. The dash-release branch (`tranche/4-1`) is a follow-on from `tranche/4` (which originally carried SD-20's per-character-rules-engine work); SD-21 inherits the dash-release cadence alongside SD-20's lineage rather than carving a fifth trunk branch. **Note:** the previously created `codex-tranche-5` board is repurposed or retired per the operator's call — current state of that board is *not* changed by this decision; the SD-21 loop's Step 10 mint command hard-codes `--board codex-tranche-4-1` going forward, so cycles land on the new board regardless of the operator's default-board setting.

**Reason:** the operator's stated framing was direct: "SD-21 is going to be tranche/4-1." Operationally, a dash-release from `tranche/4` keeps the per-character-rules-engine and per-character-product-surface (`tranche/5` → `tranche/4-1`) lifecycles visibly adjacent on the merge graph, which is easier to audit than a separate trunk branch. SD-22 retains its own future tranche to be assigned at SD-22 launch.

**Operational consequence for the loop.** The loop-instruction's Step 3 (working-tree check), Step 6 (commit and push), Step 10 (kanban card mint), and §Cross-reference are all rewritten to point at `tranche/4-1` instead of `tranche/5`. The kanban card mint command is `hermes kanban --board codex-tranche-4-1 create ...` (the `--board` flag is hard-coded). The promotion PR at SD-21 closure is `tranche/4-1 → develop`, not `tranche/5 → develop` (per the closure-flow doctrine at `governance/spec-domain-lifecycle.md`, the loop also opens a documentation PR against `programs/codex/requirements/SD-21-campaign-manager-and-persistence/README.md` flipping closure-state frontmatter).

**Operational consequence for SD-20 and SD-22.** SD-20 stayed on `tranche/4` (its original lane); SD-21's `tranche/4-1` dash-branch is *separate* from SD-20's `tranche/4`. SD-22 will get its own future tranche + board at SD-22 launch, not pinned to `tranche/4-2` (operator may choose when SD-22 fires).

**Sequencing note.** This flip is operator-driven because the operator noticed the gap when Epic 5/Epic 6 scope grew large and SD-21 became a "release package" rather than a "tranche convergence." The flip does not invalidate any prior cycles — none of SD-21's cycles have landed on `tranche/5` (the bundle has been pre-launch throughout the session).

## 12. Resolver cross-book fallback at the engine layer (operator directive 2026-07-16, Q5)

**Decision:** `equipment_id_resolve(item_id, rule_set, corpus)` (and `spell_id_resolve` symmetrically) tries the requested `rule_set` first, then falls back to other rule sets in priority order **APG → CRB → ACG**. Returns the first match with a `resolved_from_rule_set: RuleSetId` provenance field indicating which book the record came from. The resolver contract shape changes: `Some(&EquipmentRecord, Option<TableCellRef>, RuleSetId)` (adding the provenance field).

**Displaced alternative (prior default, superseded):** Each `RuleSetId` is queried independently; no fallback. The cross-book resolution is a higher-level feature that can land later.

**Reason:** Per operator directive 2026-07-16 (Q5): "cross-book fallback at resolver layer." The resolver-layer fallback is the right call because (a) one resolver call covers the user's intent ("does this spell exist somewhere?"), (b) the GUI doesn't need a "not in this book" diagnostic for cross-book lookups, (c) matches what a Pathfinder player actually expects. The risk of surfacing an unintended book (e.g. user wants CRB-only but resolver returns APG reprint) is mitigated by the explicit `resolved_from_rule_set` field so the GUI can surface which book it came from. The priority order **APG → CRB → ACG** reflects the operator's likely intent: APG first (newer content is the default expectation), CRB as the base fallback, ACG last (supplemental). Recorded as `risks-and-open-questions.md` Q5.

**What this means for SD-19 (which landed the original resolver).** SD-19's `equipment_id_resolve` and `spell_id_resolve` shipped without the cross-book fallback. SD-21's epic 1 (campaign manager) or a small SD-19-follow-on extends the resolvers to add the fallback + provenance field. Test coverage must add: APG-key resolves to APG when present, falls back to CRB; CRB-key never falls through to APG (CRB is queried directly when `rule_set = Crb`); APG-key never falls through to ACG (priority order is strict); `resolved_from_rule_set` is set correctly in every match path.

**Displaced alternative (also displaced):** Per-book scoping with a separate "find anywhere" resolver variant. Pros: no risk of cross-book surprise. Cons: two resolver APIs, GUI has to pick which to call, doesn't match the "find this thing" user mental model.

Mirrored as `risks-and-open-questions.md` Q5 (PINNED: cross-book fallback with APG→CRB→ACG priority and provenance field).

## 13. Status matrix in progress doc (operator improvement 2026-07-17; SD-21 only)

**Decision:** SD-21's progress doc `./progress.md` maintains a `## Status matrix` block near the top of the file, with one row per planned loop across the 15 acceptance criteria in `./scope-draft.md` §1.1–§1.8. The matrix is the operator's at-a-glance quick reference; the per-cycle `## Cycle log` continues to hold the full per-cycle evidence.

**Columns:** `Loop | Brief description | Started | Duration | Status | Receipt`
- `Loop`: criterion identifier (e.g. `campaign_manager:character_create`, `drive:snapshot_save`, `apg:barbarian_class_grant`)
- `Brief description`: one-sentence per-criterion prose copied from the scope draft
- `Started`: ISO timestamp of the cycle that first claimed the criterion (or `—` for not yet started)
- `Duration`: elapsed seconds for this cycle (e.g. `~2700s`), or `—`
- `Status`: `pending` (open/unclaimed) | `running` (in-flight stream) | `complete` (cycle landed with green tests / merged) | `blocked` (real blocker per `## Open blockers`)
- `Receipt`: commit SHA on `tranche/5` and hermes kanban card id (e.g. `f99a264 / t_ba4b156a`), or `—` for non-complete rows

**Lifecycle:** on cycle 1 the loop initializes the matrix skeleton from the scope draft's §1.1–§1.8 acceptance criteria (every criterion as a `pending` row with `Started`/`Duration`/`Receipt` set to `—`). On every subsequent cycle the loop updates the row matching the cycle's criterion: `Started` populated the first time the row's claimed, `Duration` populated per cycle, `Status` updated to `complete` on green tests / merge or `blocked` per §Open blockers. Editing is in place; the cycle log keeps full evidence.

**Vocabulary:** the matrix uses the existing SD-21 vocabulary (`pending`/`running`/`complete`/`blocked`) rather than introducing a new status enum. The gap the matrix closes is *quick-reference visibility*, not vocabulary fragmentation — the operator's previous complaint was that the existing vocabulary was not surfaced anywhere visible. The matrix surfaces it.

**Scope:** SD-21 only. SD-20's existing progress file is not retrofitted (operator directive 2026-07-17); the operator will read SD-20's matrix-less prose-bullet status summary for the remainder of that run. If SD-22 inherits this matrix pattern, that becomes a separate `decisions.md` §N at operator direction.

## 14. Q1–Q5 closure summary (operator directive 2026-07-16)

Q1 (multi-DM campaigns): PINNED status quo, single `dm_member_id` — solo-DM is the dominant case; multi-DM is an edge case.
Q2 (cross-account sharing): PINNED status quo, Drive permission model — engine doesn't abstract cross-account sharing.
Q3 (account migration): PINNED status quo, manual export + re-import — engine doesn't auto-migrate; operator can land as future epic.
Q4 (snapshot granularity on SD-20 close): PINNED status quo, auto-upgrade as banner not blocker — DM doesn't see save-blocker; GUI surfaces the transition.
Q5 (cross-book resolution at resolver layer): PINNED cross-book fallback with APG→CRB→ACG priority and `resolved_from_rule_set: RuleSetId` provenance field — recorded as `decisions.md` §12.

No remaining open architectural questions for SD-21. Future-class concerns (multi-DM, cross-account, account migration) are verified by epic-1's acceptance criteria + the campaign-shape boundary contract + the GUI's banner-on-stale-snapshot behavior — not by additional bundle doctrine.

## 15. Epic 3 (Update UI bug remediation) lives in SD-21 by lifecycle routing (operator directive 2026-07-17)

**Decision:** an Update-tab bug ("Update: Release Notes & Eligibility — release notes never render; eligibility always 'Unknown'") surfaced against code that originally shipped under the now-closed `SD-16-feedback-loop-and-self-update-hardening/` bundle. Per the spec-domain lifecycle doctrine (`governance/spec-domain-lifecycle.md`), the work belongs to the bundle currently shaping the next release package — SD-21 — not to the originating bundle. Epic 3 is SD-21's bundle of bug fixes for the next release package; it cites affected code by file:line (file paths under `apps/desktop/src/sd16/update/` and `apps/desktop/src-tauri/`) and does not reorganize SD-16's doctrine to absorb the fix.

**Reason:** under the lifecycle routing rule, "spec domains" are release carriers. They close when their tranche merges to develop. Feathering bug fixes back into a closed spec domain would mean future sessions reading `SD-16-feedback-loop-and-self-update-hardening/` see ongoing work that, conceptually, has nothing to do with that release. Bundle the fix into the next release instead, and a future session reads SD-21 + Epic 3 without having to walk back through closed history.

**Operational consequence for the loop.** The SD-21 loop's `## SD-21 cycles` matrix records Epic 3's 4 acceptance criteria (criteria 12–15 in `epic-breakdown.md`) under the same `pending` / `running` / `complete` / `blocked` vocabulary as the other epics. Loop cycles against Epic 3 land on `tranche/4-1` alongside Epic 2 cycles, with file-touch partition enforcing the one-cycle-at-a-time-per-file discipline from `governance/agents/CLAUDE.md`. Line numbers in Epic 3 acceptance criteria were verified against the live `apps/desktop/src/sd16/update/` files at 2026-07-17 before commit; a future cycle that re-verifies the line numbers against any post-2026-07-17 movement of the file should update the Epic 3 acceptance criteria before relying on them.

**Out of scope (recorded explicitly to prevent scope creep):** any change to the spec-domain lifecycle doctrine itself, re-opening any closed spec domain, drive persistence or campaign-manager bug fixes, APG/ACG ingestion bug fixes (those now live in SD-22), identifier-cleanup work (that is Epic 1's surface).

## 16. Identifier discipline is a governance base requirement; Epic 1 fires it (operator directive 2026-07-17)

**Decision:** under the identifier-discipline doctrine (`governance/identifier-discipline.md`), source-code identifiers (functions, methods, constants, properties, Tauri command names, CSS classes, `data-testid` attributes, file-path patterns, inline doc-comments tying back to the originating release) describe what the artifact does, not which release or spec domain it came from. The operator's named convention is **PascalCase** for functions/methods/constants/properties/Tauri commands (`MyPreferredMethodIsPascalCase`); lowercase `camelCase` for variables. Bundle-of-record identifiers (`SD-N-...`, `Tranche N chassis lane`, `AV-PAY-N`, `t_<hex>`) stay in `programs/.../requirements/SD-N-.../` and the doctrine, never in source.

**Reason:** when source-code identifier names carry a bundle tag (`sd16_browser_handoff`, `SD16_UI_*`, `data-testid="sd16-restore-previous-button"`), new readers pattern-match on the tag and assume the code belongs to a closed/semi-closed spec domain. The whole point of `governance/spec-domain-lifecycle.md` is that closed bundles stay closed and new work goes to the active bundle. If the source keeps saying `sd16_`, that governance leaks into the active bundle's working surface, and any newcomer reading the code forms a wrong mental model. This is especially load-bearing because the operator plans to onboard additional contributors after the next release; tribal-coded identifiers become a known friction the moment a second person reads the code.

**Operational consequence for SD-21.** Epic 1 (Code-Side Identifier Cleanup) covers criteria 1-4 in `epic-breakdown.md`: Rust Tauri command renames, TypeScript function and constant renames, inline doc-comment and `data-testid` cleanups, per-cycle test follow-ups. Epic 1 fires first (before Epic 3) because both touch `apps/desktop/src/sd16/update/controllerAdapter.ts` and Epic 1's removals mean Epic 3's bug-fix cycles don't have to fight the `sd16_*` style. The cycle-ordering section of `epic-breakdown.md` makes Epic 1 the first epic listed; the loop's Step 1 priority order picks it up before Epic 3.

**Out of scope for Epic 1:** directory tree renames (`apps/desktop/src/sd16/` → `apps/desktop/src/update/`, `apps/desktop/src-tauri/src/sd{16,19}_*.rs` → descriptive file names). Those are a follow-on epic because the rename churns every relative import, every release-channel JSON, and every electron-vite config — release-package-scale work that needs its own acceptance criteria and its own cycle.

**Bundle-side artifacts updated to enforce the rule.** `governance/agents/CLAUDE.md` and `governance/agents/AGENTS.md` (and their `repos/codex/` mirrors) gained a Non-Negotiable Rule #6 ("Source-code identifiers must not bundle-tag") with cross-reference to this doctrine file and the procedural skill at `~/.hermes/profiles/god-emporer/skills/devops/identifier-discipline/SKILL.md`. The rule is now part of the durable conduct surface, not just a per-loop reminder.

## 17. Closure Epilogue is a standard part of every spec-domain handoff (operator directive 2026-07-17)

**Decision:** every spec-domain bundle (active or future) includes a **Closure Epilogue** epic — a final-cycle pass that (a) scans every acceptance criterion for `complete` or `## Open blockers` status, (b) opens the closure PR for the working integration branch → develop, (c) cleans up worktrees and stale branches, (d) generates release notes, and (e) increments the version number per the bundle's versioning convention. This is part of the standard handoff format going forward; the operator directive 2026-07-17 ("This should be a standard part of our handoff document going forward") is the foundation. SD-21's Epic 4 (criteria 16-21) is the first worked example.

**Reason:** four concurrent failure modes surface without a closure epilogue. (1) A criterion that was `pending` or `running` slips to a final-merge state, shipping partial functionality while claiming completeness — exactly the failure pattern `governance/agents/AGENTS.md` rule #2 ("No fake completion") warns against. (2) Worktree accumulation across cycles leaves ten+ `wt_*` directories on the operator's disk and orphaned branches that consume minutes of operator time before each new cycle. (3) Without release notes, the post-merge state is opaque to anyone who didn't participate in the loop's cycle log. (4) Without a version bump, the displayed build label keeps showing the previous release forever (the same defect Epic 5 addresses for this SD's specific version increment). A single bundled epic at the end of every SD-N closes all four failure modes.

**Operational consequence for SD-21.** Epic 4 is listed LAST in the cycle-ordering section of `epic-breakdown.md`. The loop's Step 11 (per-cycle progress-doc update) already mints a kanban post-mortem card; Epic 4's cycle is the one where the post-mortem card's description aggregates the full matrix and the closure PR is opened. The release-notes generator is a one-shot shell command (the loop produces them and commits them under `docs/release/SD-21/release-notes.md`, relocated 2026-07-20 from `programs/codex/requirements/SD-21-campaign-manager-and-persistence/`); it has no live consumers in this SD's run, but the convention generates them as a durable artifact going forward.

**Out of scope (recorded explicitly to prevent scope creep):** worktree and branch cleanup for branches outside the SD-21 lane (Tranche-3 chassis lane, sibling SD-20/SD-22 work); auto-merging the closure PR (`decisions.md` §6 no-branches convention keeps that operator-driven). Future SD-N's Closure Epilogue epic is templated from this SD-21 Epic 4's criteria 19-24. The Epic 4 *tranche*-position increment is part of the closure package; the per-CI-build *build*-counter increment is operator-pinned at cycle launch (not Epic 4's job) and the per-main-publish *major*-position increment is a future bundle's epic.

## 18. Build version scheme is `<major>.<tranche-base>.<build>` with three-position rules (operator directive 2026-07-17)

**Decision:** the displayed build version follows a three-position **`<major>.<tranche-base>.<build>`** scheme (replacing the prior `0.0.X` patch-only scheme that the operator has confirmed was a bad call). The three positions follow distinct increment rules:

- **`major`** (first number) is `0` until the first publish to `main`. Increments by `1` per merge to `main` (the publish surface). A future repo that ships to main for the first time might land as `1.x.y`; the operator's stated example: *"our first publish to main might be `1.6.134` if we play our cards right"* — that example confirms the *major* increments on main-publish, *tranche-base* advances by base-tranche digit, *build* is the high-cardinality ordinal.
- **`tranche-base`** (second number) is the **base** of the active working tranche, not an increment counter. `tranche/4-1` carries `4`; `tranche/4-2` (a future dash from Tranche 4) also carries `4`; `tranche/5` carries `5`. Increments *slowly*, only on tranche promotion off the prior base. The operator's stated example: *"with tranche 4-1, we care about the 4. The next build would be `0.4.93`"*.
- **`build`** (third number) is a **monotonic counter across all builds across all branches — never resets**. Increments by `1` on every merge to a working branch (typically `tranche/<N>`). The operator's stated example: *"That last number will just keep incrementing toward infinity"*. The build counter accumulates from `0` → `92` → `93` → `100` → ... → `∞` over the lifetime of the project. The operator's stated example: *"we do more PRs before going to tranche 5 it might be `0.4.100`"*.

The three positions advance at different rates: build is the most frequent (per-merge), tranche is mid-frequency (per-tranche-promotion), major is rare (per-main-publish). This makes the displayed build version operationally meaningful — viewers can read it and know (a) which tranche built the artifact, (b) how many builds ago that happened, and (c) whether the artifact was pre- or post-first-main-publish.

**Why this replaces `0.0.X`:** the prior scheme (per-tranche-patch-increment) conflated the *tranche* position with the *build* position, so viewers couldn't tell whether `0.0.93` meant "Tranche 4, build 93" or "Tranche 93 of some 0.0 series". The new scheme fixes that by making tranche the second position (the visible base-tranche digit) and build the third (the high-cardinality ordinal).

**Concrete values.** The three version files (`apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`) read `"0.0.0"` on disk today (Vite/Tauri scaffolding defaults). After Epic 5 lands, the version reads `"0.4.<current_build>"` — e.g. `"0.4.93"` if the build counter is at 92 today. The current-build anchor is *checkable via git log + the prior-session receipt comment chain*; if not retrievable from there, the operator pins a value at SD-21 cycle launch. `Cargo.lock`'s embedded version updates automatically on the next `cargo check`, no manual edit needed.

**Per-position increment responsibility:**
- **Per-CI-build / per-merge:** increment **build** (`0.4.92` → `0.4.93`). Operator-pinned at SD-21 cycle launch; Epic 5 (Build Version Numbering) doesn't own this — it's a per-build operator action outside Epic 5's scope. Future bundle's epic for automation (out of scope for SD-21).
- **Per-tranche-promotion:** increment **tranche**, **reset build to 0** (`0.4.<last_build>` → `0.5.0`). Epic 4 (Closure Epilogue) owns this; criterion 23 in `epic-breakdown.md` documents the mechanics.
- **Per-main-publish:** increment **major**, **reset both tranche and build to 0** (`0.4.<last_build>` → `1.0.0`). Out of scope for SD-21; first-main-publish is a future bundle's epic.

**The build label format change** is presentation-only. `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts:61` changes `BUILD_PREFIX = 'codex'` to `BUILD_PREFIX = 'Codex'`; `createSd11WorkbenchStatus.ts:72-74` changes the template from `${BUILD_PREFIX}@${buildVersion}` to `${BUILD_PREFIX} ${buildVersion}` (drop the `@`, add a space). Every consumer of `buildLabel` (verified at `apps/desktop/src/sd11/`, `apps/desktop/src/boundary/loadSd11UpdateAction.ts`, `apps/desktop/src/boundary/loadSd12ReleaseTruth.ts`, and `apps/desktop/src/sd15/`) treats the value as an opaque display string; no parsing, no `split on @`, no pattern-matching against release tags. Verified low-risk by the prior bug handoff.

**Out of scope (recorded explicitly to prevent scope creep):** major-publish automation (future bundle's epic — not in SD-21); automated build-counter increment on every CI commit (future bundle's epic — Epic 5 establishes the *value* and the *format*, not the bump-counter scripting); build-label parsing anywhere in the codebase (the format is presentation-only).

**Operational consequence for SD-21.** Epic 5 (criteria 25-27 in `epic-breakdown.md`) lands **before Epic 4** so the version commit is in the closure PR's history. **Net for this SD release: Epic 5 lands first (sets the version to `0.4.<current_build>` — e.g. `0.4.93` — major stays `0` until first main-publish, tranche stays `4` because `tranche/4-1` is a dash release off Tranche 4, build is the next monotonic counter value after the last committed build on `tranche/4-1`), then Epic 4 runs the closure PR with the version already committed.** *The concrete bump to `0.4.<current_build>` is Epic 5 (Build Version Numbering); Epic 4 (Closure Epilogue) owns the per-tranche-promotion bump shape (the next time SD-22 launches on `tranche/5`, Epic 4's equivalent bumps to `0.5.0`).* Future SD-N's closure bumps are defined by their own operator-pinned per-bump value (e.g. SD-22's release might land as `0.5.<next_build>`).

## 19. Multiclass + broader single-class support lands in SD-21 Epic 6 + 7 (operator directive 2026-07-17)

**Decision:** the bug handoff identifying the `compute_fighter_chassis` structural gate (per `governance/spec-domain-lifecycle.md`, the bug's "originating release" is Tranche-3 chassis substrate by SD-18; closed; bug handoff states: *only single-class Fighter can ever reach a Computed result, every other single class and any multiclass combination is permanently blocked*) routes to SD-21's next release via lifecycle-routing — the bug's home is whichever bundle is currently shaping the next release package. SD-21 (now on `tranche/4-1`) owns the fix.

**Epic decomposition** (per bug handoff's recommended two-phase split, plus an option B for multiclass completion):
- **Epic 6 — Single-class coverage completion.** Bring at least one — the bug handoff recommends "most naturally Wizard," given the existing partial groundwork at `src/rules_core/pilot_compute.rs:13967` — to full single-class Computed support. Epics 6a/6b/6c/etc. as operator-pinned: Wizard first, then Cleric/Sorcerer/etc. as work proceeds. **One sub-feature per cycle** (BAB progression, saves, spell slots, Arcane School, Scribe Scroll, etc. — six or more cycles for the Wizard extension alone).
- **Epic 7 — Multiclass stacking.** Once Epic 6 proves a single second class can be independently Computed, the resolver-layer logic is extended to sum each class's own BAB progression (full/3-4/half per class), apply PF1's best-fractional-progression rule for saves (the *correct* PF1 rule, not a naive sum), and reconcile per-class feature integration. Each single-class function currently assumes it owns the entire character (skill points, spellcasting, class features keyed off total vs. class level). Multiclass requires reconciling two+ classes' feature grants without clobbering each other. **The save-stacking formula is pinned to `src/rules_core/pilot_compute.rs`'s `decideEligibility` table as the canonical source of truth** rather than re-derived in Epic 7 — `compute_<class>_chassis` calls into `decideEligibility.class_save_bonus(class_j, level_j)`, never locally.

**Reasoning:**
- *Why SD-21, not SD-18 + SD-19 retrofit.* Per lifecycle doctrine, the bug's home is the bundle currently shaping the next release. Closing the bug by opening SD-18 or SD-19 again would violate the doctrine-of-record.
- *Why two epics, not one.* Bug handoff's two-phase recommendation is structurally important — Epic 6 must prove a second class can be computed correctly *before* Epic 7 stacks them. Conflating them risks Epic 7's stacking bugs being misattributed to Epic 6's class shape. Two epics means two failure-isolation boundaries.
- *Why "Wizard first."* The partial implementation at `pilot_compute.rs:13967` (capped at level 11) means most of the cleanup is already on disk. Wizard's lead position reduces total cycle work; the remaining classes (Cleric, Sorcerer, etc.) get Epic 6b/6c/etc. as operator-pinned.
- *Why best-fractional-progress is non-negotiable.* A naive sum of saves would give multiclass characters higher saves than any single class — a rules-engine correctness violation that PF1 players will catch quickly. The bug handoff's emphasis on best-fractional progression is a real PF1 rule, not a stylistic preference.

**Operational consequence for SD-21's loop-instruction.**
- Epic 6 cycles modify Tranche-3 substrate (`src/rules_core/pilot_compute.rs`) — outside SD-21's normal file-touch partition, but the bundle's authority over "next-release rules-engine work" supersedes the lane partitioning for this single epic.
- Epic 7 cycles also modify `pilot_compute.rs` (BAB/save stacking) + the per-class-feature integration surface in `src/rules_core/`. These touchpoints mirror Tranche-3 substrate authority.
- File-touch partition extends to include pilot_compute.rs + per-class module files as Epic 6/7 surfaces. Loop-instruction's §Concurrency rules gains new rows.

**Out of scope (recorded explicitly to prevent scope creep):**
- *Charter module rework.* Bugs deeper than "the compute path is gated wrong." If Epic 6 reveals that the per-class functions themselves are buggy (e.g. Wizard's spell DC formula isn't PF1-correct), that's a separate bug, filed separately, not absorbed into Epic 6.
- *PCGen re-ingestion.* The Tranche-3 corpus-source ingestion (`src/pcgen_import/`) is independent of the rules-core compute path. Epic 6/7 doesn't touch PCGen parsing.
- *Multiclass spell-stacking edge cases (Sorcerer/Wizard prepared casting across both classes).* Edge case requiring investigation; deferred to a future Epic 7b or Epic 7c.
- *Character creation GUI redesign.* The Campaign Manager GUI is outside the bundle per `decisions.md` §6; multiclass support is an engine-side fix, not a UI fix.

## 20. SD-21 is sized for one tranche — 7 epics, 30 criteria, single release (operator directive 2026-07-17)

**Decision:** SD-21's final epic decomposition is **7 epics with 30 acceptance criteria** (Identifier Cleanup as Epic 1; Campaign Manager + Drive as Epic 2; Update UI bug as Epic 3; Closure Epilogue as Epic 4; Build Version Numbering as Epic 5; Single-class Coverage Completion as Epic 6; Multiclass Stacking as Epic 7). The bundle ships as a single release on `tranche/4-1` with the bundle's `codex-tranche-4-1` board. Epic 6 and Epic 7 are interleaved with Epics 2-3 in the cycle order rather than served as a separate bundle; the bundle's per-tranche scope is now compact enough that one loop can drive all seven epics to closure without fragmenting across multiple launch branches.

**Reasoning:**
- *Why SD-21 fits in one tranche.* With APG + ACG + advanced guides moved to SD-22 (per operator directive 2026-07-17), SD-21's content-source lane collapses; the bundle's remaining scope is per-character-rules-engine completeness (multiclass support) + governance epics (Identifier Cleanup, Closure Epilogue, Build Version) + the Update UI bug remediation + Campaign Manager + Drive persistence. None of these has the multi-class content-volume that APG/ACG/Ultimate-line content would have introduced. One tranche fits.
- *Why Identifier Cleanup is Epic 1, not Epic 5.* Per operator directive 2026-07-17 ("make the code cleanup epic the new epic 1"), the identifier-cleanup governance base requirement lands first. It anchors the bundle's identifier shape before any feature work begins; new readers land on a clean identifier surface before encountering feature code; future contributors the operator is onboarding after the next release see clean names from day one.
- *Why Closure Epilogue is Epic 4 (LAST in 7-epic layout), not Epic 6.* The closure epilogue fires LAST by definition (the cycle-ordering block at the end of `epic-breakdown.md` makes this explicit); in a 7-epic layout its slot is position 4. The label, the criterion numbers, and the cycle-ordering block all carry "Epic 4 = LAST" without confusion.

**Out of scope.** A "bundle-size budget" doctrine (max N epics per bundle, max M criteria) — would require a separate governance file at `governance/spec-domain-size-budget.md`. Not written this turn; the operator can decide whether to pin a size budget at the closure-flow review.

## 21. Operator-deferred shape decisions now closed (operator directive 2026-07-17)

**Decision:** multiple operator-deferred shape decisions have closed or been explicitly deferred as scope-of-record:
- **Closure-state frontmatter field vocabulary (governance/spec-domain-lifecycle.md open call #1):** DEFERRED. The bundle's `status: approved` + `status: shipped` transition is currently via operator review; no `closed_by_tranche: <N>` field pinning yet. The closure-flow PR will add the field shape the first time it's needed (likely SD-21's closure flow itself, since SD-21 is the first worked example with 7 epics closing).
- **Backfill shape for SD-20 and earlier (operator directive 2026-07-17):** DEFERRED. Operator's standing directive "record status of all SD-20 and earlier as completed/closed" remains open. Shape discussion deferred to a follow-on edit. Skill proposed: `spec-domain-bundle-authoring` §"Closed-bundle backfill" covers the recipe once shape is pinned.
- **Audit-trail size for closure PRs (governance/spec-domain-lifecycle.md open call #3):** RESOLVED via Epic 4 criterion 21's closure-test-suite run. The audit-trail is the cycle log + closure PR description; no separate ledger file is needed (lightweight is enough per standing memory).
- **Rehome rule for code ownership (governance/spec-domain-lifecycle.md open call #4):** RESOLVED via Epic 1 of this bundle. The receiving bundle cites code by file:line; the bundle does not own code. The file is whatever it is.
- **Pinned closure-flow field for SD-22:** DEFERRED. SD-22's launch-branch (likely `tranche/4-2` or operator's choice) is operator-pinned when SD-22 fires.

## Cross-reference

- `README.md` — bundle overview, posture summary, navigation.
- `acceptance-and-verification.md` — closure gates including campaign-manager integration.
- `epic-breakdown.md` — 32 acceptance criteria grouped into 7 epics (3 epics for the SD-21-shaped slices + Epic 4 for the post-closure-routed bug remediation + Epic 5 for the code-side identifier cleanup + Epic 6 closure epilogue + Epic 7 build versioning).
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split, override flags A–D (all defaulted), 5 architectural questions Q1–Q5 (all PINNED 2026-07-16), update-UI bug discovery cross-link.
- `technical-design.md` — campaign-shape boundary contract shape, Drive adapter boundary contract, markdown file format.
- `technical-requirements.md` — pre-loop prerequisites for SD-21 (SD-19 ships; SD-20 may be in flight but does not gate SD-21).
- `governance/spec-domain-lifecycle.md` — sibling lifecycle doctrine; governs Epic 4's lifecycle routing.
- `governance/identifier-discipline.md` — sibling identifier-discipline doctrine; governs Epic 5's identifier-cleanup criteria.
- `docs/SD-21/release-closure-checklist.md` (to be created by Epic 7 criterion 32) — closure-time checklist, part of the standard handoff per the operator directive 2026-07-17.
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
- `../SD-18/` — chassis grounding.
- `../SD-19/` — corpus-aware seam + canonical Paizo-table store (CRB).
- `../SD-20/` — sibling bundle (parallel; per-character tabletop-readiness).
- `~/workspace/programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/` — corpus-side parsing consumed by SD-21 ingestion.
