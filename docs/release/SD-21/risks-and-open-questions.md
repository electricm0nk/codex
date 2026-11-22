---
title: SD-21 — Risks and Open Questions
status: approved (operator review 2026-07-16; operator directives 2026-07-17: branch flip tranche/5 → tranche/4-1, board flip codex-tranche-5 → codex-tranche-4-1, APG+ACG+advanced guides moved to SD-22, Identifier Cleanup renumbered as Epic 1, 7-epic / 30-criteria final shape; Q1 status quo single dm_member_id, Q2 status quo Drive permission model, Q3 status quo manual export + re-import migration, Q4 status quo auto-upgrade on SD-20 close (banner not blocker), Q5 cross-book fallback at resolver layer with APG→CRB→ACG priority and provenance field; override flags A–D defaulted; bundle marked approved with operator directives 2026-07-16/17)
date: 2026-07-15
canonical_branch: tranche/4-1
kanban_board: codex-tranche-4-1
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md
---

# SD-21 — Risks and Open Questions

This file enumerates the risks, blockers, and open questions specific to SD-21. Structured to mirror SD-18, SD-19, and SD-20's risks docs.

## Self-healable conditions (resolve inline, exit GREEN)

| Condition | Detection | Self-heal |
|---|---|---|
| Working tree dirty at cycle start | `git status --porcelain \| wc -l` returns non-zero | Run `git stash` (if unfinished) or `git checkout -- .` (stray noise); retry |
| A Drive OAuth token refresh fails because Google revoked the refresh token | Engine surfaces "re-authorize required" diagnostic | GUI prompts user to re-authorize via the Drive authorize button |
| A markdown file on disk fails to parse (e.g. corrupt YAML frontmatter) | Per-file parse error on campaign load | Surface the file path and parse error in the load result; don't fail the whole load — the user can repair the file manually and re-load |
| RuleSetId variants from a future book arrive in SD-22's content-source ingest work (e.g. `RuleSetId::Um` for Ultimate-line books) | Compile error or runtime match error in SD-22's per-class/per-book code | Document that SD-22 owns all `RuleSetId::Apg`, `RuleSetId::Acg`, `RuleSetId::Um`, `RuleSetId::Bestiary1`, etc. SD-21 reads `RuleSetId::Crb` only; SD-22 owns the cross-book ladder; the operator fields feature requests for new ruleset variants under SD-22 not SD-21. |
| Two cycles both try to add new content to the same APG/ACG table cell | Merge conflict on the structured-data file | Resolve inline if mechanical (ordering, an extra row); escalate to operator if semantic (which variant of a class feature is authoritative) |
| A cycle's RED test fails because the rule_set-wide canonical mapping is missing | `tests/sd21_apg_class_tables_resolve` fails for a class that hasn't been ingested yet | Route to Open Blockers; operator decides whether the cycle is the right time to ingest that class |
| Markdown file on disk has a stale `nonce` (from a Drive sync edge case) | `CampaignSnapshot.nonce != saved_nonce` on load | Engine surfaces "stale nonce, please save again"; doesn't trigger conflict log unless the *content* also differs |

## Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Detection | Why not self-heal |
|---|---|---|
| The campaign-shape boundary contract (`CampaignSnapshot`) doesn't match the GUI's vibe-coded expectations from PR #316 | Engine produces a snapshot the GUI rejects (specific field naming or shape mismatch) | Boundary contract drift — the contract needs to be amended, the GUI needs to be patched, or both; cycle can't fix this alone |
| The Google Cloud Console project for codex doesn't have OAuth credentials configured (no client ID, secret, or redirect URI registered) | Engine surfaces "OAuth credentials not configured" on first campaign-create attempt | Operator-side fix (Google Cloud Console); bundle can't proceed |
| Two `claude` processes both touch `src/rules_core/persistence/` | `ps -eo pid,etime,stat,cmd \| grep claude` shows multiple on the same file set | Structural: one-lane-at-a-time rule |
| A campaign on disk has `campaign.md` with frontmatter YAML that parses but typed values fail (e.g. `level: "four"` instead of `4`) | Per-field type error during load | Engine surfaces the file + the field with the bad value; cycle can't fix the user's data |
| Cycles for APG/ACG ingestion touch the same spell's effect-text in both books' structured-data files (e.g. a spell named identically in both APG and ACG would lose its book attribution) | Cycle's per-book spell-test fails or the cross-book resolution test fails | Need explicit `book_name` prefix in `RuleSetId::Apg + spell_key` vs `RuleSetId::Acg + spell_key` to disambiguate; escalate |
| Cargo test regresses on a row other than the one the cycle touched | Full suite regresses after a cycle's change | Sibling-preservation is a hard rule |
| Progress doc and live matrix disagree on a row's `evidence_tier` (not just stale snapshot) | Cycle's expected vs. actual differ | Manual operator reconciliation required |

## Override flags (durable; patched when operator accepts a default)

### Flag A — `CampaignSnapshot` field set

**Default chosen**: the `CharacterSummary` shape exposes chassis-only fields during the parallel window (race, class summary, level, HP, AC, BAB, saves, ability mods). When SD-20 closes, the field set grows to include spellbook coverage, feat effects, skill totals, equipment effects.

**Override alternatives:**

- *Always full character detailed fields* (wait for SD-20 to close before SD-21's epic-1 ships). Loses the parallelizable-with-SD-20 benefit.
- *Always chassis-only fields* (never grow the summary). Means DM stays unsatisfied with stale character data after SD-20 lands.

**Override cost**: ~30 minutes; affects the artifacts doc at `docs/SD-21/campaign-boundary-contract.md` and the `technical-design.md` §1.2.

### Flag B — Drive OAuth scope

**Default chosen**: `https://www.googleapis.com/auth/drive.file` (file-scoped; the bare minimum scope that lets the user pick a folder and write/read files within it but doesn't grant access to the user's whole Drive).

**Override alternatives:**

- *Full Drive scope* (`https://www.googleapis.com/auth/drive`). Allows broader visibility but requires explicit user trust.
- *No OAuth, local file system only*. Loses collaboration.

**Override cost**: ~5 minutes (Google Cloud Console scope change + operator refreshes the OAuth flow).

### Flag C — Markdown file granularity

**Default chosen**: one markdown file per top-level field of `CampaignSnapshot` (campaign.md, party.md, resources.md, plus per-row files for `adventure_log/`, `maps/`, `wiki/`, `members/`). Matches the operator's likely Obsidian-folder-view workflow.

**Override alternatives:**

- *Single monolithic `campaign.md`* with all sections. Easier to version-control, harder to edit by hand.
- *Per-row-per-section* (one file per adventure-log entry, per map, per wiki page, per member; no campaign-level files). Most granular, lets Obsidian index per-row, but harder to keep atomic on save.

**Override cost**: ~15 minutes; affects the `technical-design.md` §2.2 layout and the markdown file-format artifact doc.

### Flag D — `CampaignSnapshot` ownership: backend-agnostic vs. Drive-coupled

**Default chosen**: `CampaignBackend` trait with the Drive adapter as one impl. Engine never imports Drive types directly (per `decisions.md` §4).

**Override alternatives:**

- *Drive-coupled engine*: engine imports `google_drive3` (or equivalent) and the engine's persistence layer calls Drive directly. Faster to ship, locks the engine to Drive.

**Override cost**: significant; this would mean re-doing Epic 2's campaign persistence module to remove the abstraction. Operator-only call.

## Architectural questions (Q1–Q5 PINNED 2026-07-16)

These are SD-21-shaped design calls. Q1–Q4 were defaulted; Q5 was explicitly directed to cross-book fallback at the resolver layer per operator directive 2026-07-16. All five are now PINNED; future sessions recover the bundle shape from these pinned answers without re-litigating them.

### Q1 — Multi-DM campaigns (PINNED: status quo, single `dm_member_id`)

PF1 traditionally has one DM per campaign. Should `CampaignMetadata.dm_member_id` be a list (allowing co-DMs) or a single value? My current default: single value. If the operator runs joint or rotating-DM campaigns, change to a list.

**Pinned to status quo (single `dm_member_id`)** per operator directive 2026-07-16. Solo-DM is the dominant case; multi-DM is an edge case worth handling later if at all. Recorded as `decisions.md` §15.

**Override cost**: ~10 min; affects the `CampaignMetadata` struct shape and the Drive adapter's per-DM folder binding.

### Q2 — Campaign sharing across DM accounts (PINNED: status quo, Drive permission model)

If a DM creates a campaign and then another DM (different Google account) needs to participate, the Drive folder permissions model supports this (OAuth scope `drive.file` lets the user pick a folder that's not owned by the same account). But the engine's `dm_member_id` is a single identifier — the second DM has a different account. My current default: each DM has their own Drive folder; cross-DM campaign sharing uses Drive's permission model rather than the engine. If the operator wants the engine to abstract cross-account sharing, that's an additional epic.

**Pinned to status quo (Drive permission model handles cross-DM sharing; engine doesn't abstract it)** per operator directive 2026-07-16. Drive's permission model is the right abstraction for cross-account sharing; duplicating that in the engine is wasted work. Recorded as `decisions.md` §16.

**Override cost**: significant if you want engine-abstracted cross-account sharing; this would mean a new epic with OAuth-federation work.

### Q3 — Campaign migration between Drive accounts (PINNED: status quo, manual export + re-import)

If a DM moves from one Google account to another (job change, account loss), can the campaign migrate? The Drive adapter stores the `campaign_root_folder_id` per profile; migration means re-pointing the folder. The engine doesn't currently know how to do this; manual export + re-import is the fallback. Open until operator needs.

**Pinned to status quo (manual export + re-import is the migration fallback; no engine-abstracted migration epic)** per operator directive 2026-07-16. Migration is an edge case; the manual fallback is acceptable; if a real migration need arises later, it's an additional epic. Recorded as `decisions.md` §17.

**Override cost**: ~30-60 min if you want engine-abstracted migration now (adds a `migrate_campaign(old_account → new_account)` flow that reads the old campaign's markdown files, copies them into the new account's folder via Drive's API, and rewrites `campaign_root_folder_id` in the campaign snapshot).

### Q4 — Per-character saved-snapshot granularity on SD-20 close (PINNED: status quo, auto-upgrade as banner not blocker)

When SD-21's Epic 2 ships before SD-20 closes, the `CharacterSummary` is chassis-only. After SD-20 closes, the snapshot becomes full-detail. There's a window where a user is partway through editing a character and SD-21's campaign holds a stale chassis-only snapshot. The behavior: SD-21's `save_campaign` always reads `CharacterSummary` from the character's current state (whatever detail level that state has), so the campaign snapshot auto-upgrades when SD-20 closes. The DM doesn't need to do anything explicit. Open Q4 is whether that auto-upgrade is the right behavior or whether SD-21 should refuse to save a campaign containing a character whose latest sheet is post-SD-20 (forcing the user to re-link the character). My current default: auto-upgrade is fine.

**Pinned to status quo (auto-upgrade is fine; surface the SD-20 transition as a banner in the GUI, not a save-blocker)** per operator directive 2026-07-16. Auto-upgrade is the right behavior because the alternative forces operator action at exactly the moment when SD-21 should be invisible — the DM doesn't care about the SD-20 transition; they care that the campaign reflects their characters. If the upgrade creates a snapshot-versioning problem later (e.g. "this campaign snapshot was taken before SD-20 epic 1 closed, so the spellbook coverage field is missing"), surface it as a banner, not a blocker. This Q is tightly coupled to SD-20's Q4 (SD-21 owns `CampaignSnapshot`); both PINNEDs agree the auto-upgrade is the right behavior. Recorded as `decisions.md` §18.

**Override cost**: ~15-30 min if you want refuse-on-mismatch (adds an eligibility check + diagnostic + GUI banner).

### Q5 — Cross-book resolution at the resolver layer (PINNED: APG → CRB → ACG fallback with provenance)

PF1 spells are mostly identical across books (a spell first published in CRB remains the same when reprinted in APG), but new APG spells don't exist in CRB. The cross-book resolution question: a query `equipment_id_resolve(item_id, RuleSetId::Apg, corpus)` — should it check APG first then fall back to CRB? My current default: each `RuleSetId` is queried independently; there's no fallback. The cross-book resolution is a higher-level feature that can land later.

**Pinned to cross-book fallback at the resolver layer with APG → CRB → ACG priority and provenance field** per operator directive 2026-07-16. The resolver `equipment_id_resolve(item_id, rule_set, corpus)` (and `spell_id_resolve` symmetrically) tries the requested `rule_set` first, then falls back to other rule sets in priority order. Returns the first match with a `resolved_from_rule_set: RuleSetId` provenance field indicating which book the record came from. Pros: one resolver call covers the user's intent ("does this spell exist somewhere?"); the GUI doesn't need a "not in this book" diagnostic for cross-book lookups; matches what a Pathfinder player actually expects. Cons: ~30-60 min of resolver-layer work (priority order, provenance tagging, test coverage for cross-book matches); changes the resolver contract shape (adds a `resolved_from_rule_set: RuleSetId` field to the result); risk of surfacing an unintended book (e.g. user wants CRB-only but resolver returns APG reprint) — mitigated by the explicit `resolved_from_rule_set` field so the GUI can surface which book it came from. Recorded as `decisions.md` §19; resolver-layer shape captured in `technical-design.md` (new section to be added).

**Override cost**: ~30-60 min if you want to refine the priority order or the provenance field shape.

## Cross-reference

- **Update UI bug remediation epic (Epic 3; operator directive 2026-07-17)** — this release ships a fix for an Update-tab bug surfaced in this session ("Update: Release Notes & Eligibility — release notes never render; eligibility always 'Unknown'"). The work is owned by SD-21 Epic 3 (Update UI Bug Remediation). Affected code is cited by file:line in `epic-breakdown.md` §Epic 3 — this entry exists so that a future session reading SD-21's risks trace can find the upstream discovery without having to re-derive the affected files. Under the spec-domain lifecycle doctrine (see `governance/spec-domain-lifecycle.md`), a bug against code that originally shipped in a closed spec domain is owned by the bundle currently shaping the next release package; the receiving bundle cites the affected code by file:line rather than re-opening the originating bundle's doctrine.
- `acceptance-and-verification.md` — closure gates.
- `decisions.md` — the 21-item decision record (SD-21 §1–§21: 9 original decisions plus §10 cross-bundle auto-upgrade on SD-20 close, §11 SD-21 launch branch flip `tranche/5 → tranche/4-1` per operator directive 2026-07-17, §12 resolver cross-book fallback APG→CRB→ACG, §13 Status matrix in progress doc, §14 Q1–Q5 closure summary, §15 Epic 3 lifecycle routing under the spec-domain lifecycle doctrine, §16 identifier discipline + Epic 1 routing under the identifier-discipline doctrine, §17 closure epilogue as standard handoff per the operator directive 2026-07-17, §18 build version numbering `<major>.<tranche-base>.<build>` three-position scheme per the operator directive 2026-07-17, §19 multiclass + broader single-class support (Epic 6 + 7), §20 SD-21 bundle-sized-for-one-tranche posture, §21 operator-deferred shape decisions now closed).
- `epic-breakdown.md` — 30 acceptance criteria grouped into 7 epics (Epic 1 Code-Side Identifier Cleanup; Epic 2 Campaign manager + Drive persistence; Epic 3 Update UI bug remediation; Epic 4 Closure Epilogue; Epic 5 Build Version Numbering; Epic 6 Single-class coverage completion; Epic 7 Multiclass stacking).
- `technical-design.md` — campaign-shape boundary contract, Drive adapter, markdown file format.
- `technical-requirements.md` — pre-loop prerequisites.
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.

