---
title: SD-21 — Technical Design
status: approved (operator review 2026-07-16; operator directives 2026-07-17: branch flip tranche/5 → tranche/4-1, board flip codex-tranche-5 → codex-tranche-4-1, APG+ACG+advanced guides moved to SD-22, Identifier Cleanup renumbered as Epic 1, 7-epic / 30-criteria final shape; Q1–Q5 PINNED, override flags A–D defaulted, resolver cross-book fallback shape added per Q5; bundle marked approved with operator directives 2026-07-16/17)
date: 2026-07-15
canonical_branch: tranche/4-1
kanban_board: codex-tranche-4-1
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md
---

# SD-21 — Technical Design

This file is the load-bearing engineering design surface for SD-21. `decisions.md` records *why* SD-21 is shaped the way it is; this file records *what* the shape is at the code level. A future implementer should be able to work from this document with no further clarification needed.

## 1. Campaign-shape boundary contract (Epic 2)

The engine exposes a `CampaignSnapshot` type whose shape is the contract between the engine and any backing-store adapter (Drive, Dropbox, local file system, future backends).

### 1.1 `CampaignSnapshot` shape

```rust
// src/rules_core/campaign.rs (NEW module)
pub struct CampaignSnapshot {
    pub metadata: CampaignMetadata,
    pub party: Party,
    pub resources: PartyResources,
    pub adventure_log: Vec<AdventureLogEntry>,
    pub maps: Vec<MapRef>,
    pub wiki: Vec<WikiPage>,
    pub nonce: u64,  // monotonic; bumped on every save; conflict detection
}

pub struct CampaignMetadata {
    pub campaign_id: CampaignId,        // UUID v7 or similar sortable
    pub name: String,
    pub rule_set: RuleSetId,            // SD-19 §9 shape: Crb now, Apg/Acg future
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dm_member_id: MemberId,          // the DM is the first member
    pub description: String,            // markdown body, shown in campaign list
}

pub struct Party {
    pub members: Vec<PartyMember>,
    pub max_size: u8,                    // typical 4-6 for PF1
}

pub struct PartyMember {
    pub member_id: MemberId,
    pub character_id: Option<CharacterId>,  // None until linked to a character sheet
    pub role: PartyMemberRole,              // PlayerCharacter | GameMaster | NonPlayerCompanion
    pub display_name: String,
    pub snapshot_at_link: CharacterSummary,  // the snapshot the campaign holds for this member
}

pub struct CharacterSummary {
    pub character_id: CharacterId,
    pub name: String,
    pub race_name: String,                // looked up from SD-19's table store
    pub class_summary: String,            // e.g. "Fighter 3 / Wizard 1"
    pub level: u8,
    pub hp_current: u8,
    pub hp_max: u8,
    pub ac: i8,
    pub bab: i8,
    pub fort_save: i8,
    pub ref_save: i8,
    pub will_save: i8,
    pub key_ability_mods: BTreeMap<Ability, i8>,
    pub at_link_date: DateTime<Utc>,      // when the campaign captured this snapshot
}

pub struct PartyResources {
    pub gold_gp: u32,
    pub platinum_pp: u32,
    pub shared_inventory: Vec<InventoryItem>,
    pub xp_total: u32,
    pub xp_to_next_level: u32,
}

pub struct InventoryItem {
    pub item_id: EquipmentKey,           // SD-19 lookup
    pub quantity: u8,
    pub notes: String,                    // "held by Alia" etc.
}

pub struct AdventureLogEntry {
    pub entry_id: EntryId,
    pub session_number: u32,
    pub date: NaiveDate,
    pub author_member_id: MemberId,
    pub body_markdown: String,
    pub tags: Vec<String>,
}

pub struct MapRef {
    pub map_id: MapId,
    pub name: String,
    pub source_file: String,             // path under <campaign>/maps/
    pub description: String,
}

pub struct WikiPage {
    pub page_id: WikiPageId,
    pub title: String,
    pub slug: String,                     // filename under <campaign>/wiki/
    pub body_markdown: String,
    pub last_edited_by: MemberId,
    pub last_edited_at: DateTime<Utc>,
}
```

The `CampaignSnapshot` is JSON-serializable; the engine produces it, the Drive adapter persists it, the GUI consumes it. Same dovetail pattern as SD-20's per-character boundary contract.

### 1.2 Per-character summary scope during parallel-window

When SD-21 launches before SD-20 closes, the `CharacterSummary` carries chassis-only fields (race, class, level, HP, AC, BAB, saves, ability mods — all readable from SD-19's table store). When SD-20 closes and the GUI captures per-character receipts, `CharacterSummary` grows to include spellbook coverage, feat effects, skill totals, equipment effects (all readable from SD-20's per-character `PilotReceipt`). The contract accommodates both states via `Option<T>` fields or a versioned snapshot format.

**Default**: `CharacterSummary` is the chassis-only shape during the parallel window, and gains the SD-20-derived fields once SD-20 closes. The contract docs include both shapes.

### 1.3 Engine-side persistence boundary (backend-agnostic)

```rust
// src/rules_core/persistence/mod.rs (NEW module)
pub trait CampaignBackend: Send + Sync {
    fn load_campaign(&self, campaign_id: &CampaignId) -> Result<CampaignSnapshot, PersistenceError>;
    fn save_campaign(&self, snapshot: &CampaignSnapshot) -> Result<(), PersistenceError>;
    fn list_campaigns(&self) -> Result<Vec<CampaignMetadata>, PersistenceError>;
    fn create_campaign(&self, metadata: &CampaignMetadata, dm_member_id: MemberId) -> Result<CampaignId, PersistenceError>;
    fn delete_campaign(&self, campaign_id: &CampaignId) -> Result<(), PersistenceError>;
    fn snapshot_known_format(&self) -> SnapshotFormatVersion;  // for forward-compat detection
}
```

The `CampaignBackend` trait is the boundary. The engine calls into a trait object. The Drive adapter and any other backend (local file system, Dropbox) implement the same trait. Production code picks one at startup; tests can swap freely.

## 2. Drive adapter (Epic 2)

### 2.1 OAuth flow

The Drive adapter uses Google OAuth 2.0 authorization-code flow (not service-account, not impersonation). The flow:

1. **First-run authorization.** The GUI's campaign-manager landing page detects no OAuth token in the configuration and presents an "Authorize Google Drive" button. Clicking opens a system browser to `https://accounts.google.com/o/oauth2/v2/auth` with `redirect_uri`, `client_id`, `scope=https://www.googleapis.com/auth/drive.file` (file-scoped; the bare minimum scope), `access_type=offline`, and `prompt=consent`. Google's consent screen appears. After consent, Google redirects to the `redirect_uri` with `code`.
2. **Token exchange.** The Tauri backend receives the `code` via a deep-link callback and exchanges it for access + refresh tokens at `https://oauth2.googleapis.com/token`. Tokens are stored in OS-keyring-backed storage (`keyring` crate on Linux, macOS keychain on macOS, Windows credential store on Windows).
3. **Token refresh.** Access tokens expire in ~1 hour. The Drive adapter refreshes via the refresh token ~5 minutes before expiry. Refresh tokens are long-lived; if one revokes, the user re-authorizes.
4. **Folder selection.** After first-run authorization, the GUI prompts the user to pick a Drive folder (or create one) as the campaign root. The adapter caches `campaign_root_folder_id` per profile.

### 2.2 Campaign-to-Drive mapping

Each campaign lives in its own folder under the configured `campaign_root_folder_id`. Folder structure:

```
campaign_root/
├── _codex_campaign_index.json          # list of CampaignMetadata for fast enumeration
├── campaigns/
│   └── <campaign_id>/
│       ├── campaign.md                 # CampaignMetadata + party summary as frontmatter
│       ├── party.md                    # Party members with their CharacterSummary
│       ├── resources.md                # PartyResources (gold, inventory, XP)
│       ├── adventure_log/
│       │   ├── 001.md                 # session 1
│       │   ├── 002.md
│       │   └── ...
│       ├── maps/
│       │   └── <map_name>.md
│       ├── wiki/
│       │   └── <page_slug>.md
│       ├── members/
│       │   └── <character_id>.md      # per-character frontmatter + description body
│       └── conflicts/
│           └── <timestamp>/
│               ├── campaign.md.local
│               ├── campaign.md.remote
│               └── CONFLICT_REPORT.md   # human-readable summary
```

Each markdown file has YAML frontmatter for structured fields (campaign_id, name, rule_set, created_at, etc.) and a markdown body for human-authored content. Machines parse frontmatter; humans edit body.

### 2.3 Save / load flow

- **Save**: serialize `CampaignSnapshot` to JSON, write each top-level field to its corresponding markdown file. Idempotent re-saves are byte-identical for unchanged fields.
- **Load**: list files in the campaign folder, parse frontmatter from each, reconstitute `CampaignSnapshot`. Conflict detection: if `nonce` on disk differs from the in-memory version (i.e. another device wrote since our last save), the engine surfaces a conflict before overwriting.
- **Conflict resolution** (per `decisions.md` §7): when conflict surfaces, both copies go to `conflicts/<timestamp>/`, local version becomes active. DM resolves manually.

### 2.4 Tauri command surface

The Drive adapter integrates with the desktop app via Tauri commands. New commands:

```rust
// apps/desktop/src-tauri/src/campaign_drive.rs (NEW module)
#[tauri::command]
async fn drive_authorize(state: tauri::State<'_, AppState>) -> Result<DriveAuthHandle, String>;

#[tauri::command]
async fn drive_pick_folder(state: tauri::State<'_, AppState>) -> Result<DriveFolder, String>;

#[tauri::command]
async fn drive_list_campaigns(state: tauri::State<'_, AppState>) -> Result<Vec<CampaignMetadata>, String>;

#[tauri::command]
async fn drive_load_campaign(state: tauri::State<'_, AppState>, id: CampaignId) -> Result<CampaignSnapshot, String>;

#[tauri::command]
async fn drive_save_campaign(state: tauri::State<'_, AppState>, snapshot: CampaignSnapshot) -> Result<(), String>;

#[tauri::command]
async fn drive_delete_campaign(state: tauri::State<'_, AppState>, id: CampaignId) -> Result<(), String>;
```

The GUI `apps/desktop/src/campaign/` (PR #316's vibe-coded screens) consumes these Tauri commands via `apps/desktop/src/boundary/campaignDrive.ts` (NEW boundary module).

## 3. APG + ACG ingestion (epics 2 and 3)

### 3.1 Ingestion pattern

Per SD-19 §9, APG populates `src/rules_core/rules_tables/apg/` and ACG populates `src/rules_core/rules_tables/acg/`. Each ingestion epic:

1. Reads the relevant PRD/PRD-fork sourcebook content (APG and ACG are well-defined books; content is public).
2. For each class table, spell list, equipment list, race table: produces structured-data files in the corresponding `apg/` or `acg/` directory, mirroring the CRB shape from `rules_tables/crb/class_tables.rs` (or whatever the foundation slice's CRB class-tables module is named).
3. Adds the new book to the `RuleSetId` enum: `pub enum RuleSetId { Crb, Apg, Acg, /* future: Um, ... */ }`.
4. Runs the existing `equipment_id_resolve` and `spell_id_resolve` seams against the new data; both take `RuleSetId` per SD-19 §3.
5. Tests: `cargo test --locked --test sd21_apg_class_tables_resolve` (and analogous for ACG); these are the per-cycle acceptance criteria for the ingestion loop.

### 3.2 Per-class cycle shape

The ingestion loop pattern: one cycle lands one class's table entries from a single source book. The cycle implements:

- The class's level-by-level feature table at `src/rules_core/rules_tables/apg/class_<class>.rs` (or `acg/`).
- Per-class spell list entries at `<book>/spell_list.rs`.
- Any class-specific equipment (e.g. Alchemist's bombs) at `<book>/equipment_tables.rs`.
- A `cargo test --locked --test sd21_apg_<class>_class_table_resolves` test that asserts every level row resolves via the `RuleSetId::Apg` parameter.

### 3.3 Resolver cross-book fallback (Q5 PINNED 2026-07-16)

Per `decisions.md` §12, `equipment_id_resolve` and `spell_id_resolve` (landed by SD-19, used by SD-22's APG/ACG/advanced-guides content-source ingest work) gain cross-book fallback at the resolver layer. The priority order is **APG → CRB → ACG**: a query with `rule_set = Apg` first checks APG, then falls back to CRB, then to ACG. A query with `rule_set = Crb` checks CRB first, then APG, then ACG. A query with `rule_set = Acg` checks ACG first, then APG, then CRB.

**Resolver contract shape change.** SD-19's resolver returns `Option<(&'a EquipmentRecord, Option<TableCellRef>)>`. The cross-book version returns `Option<(&'a EquipmentRecord, Option<TableCellRef>, RuleSetId)>` — the third tuple element is the `resolved_from_rule_set: RuleSetId` provenance field indicating which book the record came from. Same shape for `spell_id_resolve`.

```rust
// src/rules_core/equipment_resolver.rs (REVISED per Q5)
pub fn equipment_id_resolve<'a>(
    item_id: &str,
    rule_set: RuleSetId,
    corpus: &'a SourcePackageContent,
) -> Option<(&'a EquipmentRecord, Option<TableCellRef>, RuleSetId)> {
    // Priority order: requested rule_set first, then APG → CRB → ACG (or rotated per the request)
    let priority = match rule_set {
        RuleSetId::Apg => &[RuleSetId::Apg, RuleSetId::Crb, RuleSetId::Acg],
        RuleSetId::Crb => &[RuleSetId::Crb, RuleSetId::Apg, RuleSetId::Acg],
        RuleSetId::Acg => &[RuleSetId::Acg, RuleSetId::Apg, RuleSetId::Crb],
    };
    for rs in priority {
        if let Some((record, table_ref)) = lookup_in_book(item_id, *rs, corpus) {
            return Some((record, table_ref, *rs));
        }
    }
    None
}
```

**Test coverage that must land with the resolver change.**

- `equipment_id_resolve("potion-of-healing", RuleSetId::Apg, corpus)` returns `Some((record, Some(table_ref), RuleSetId::Apg))` when APG has the record; falls back to CRB with `resolved_from_rule_set: RuleSetId::Crb` when APG doesn't.
- `equipment_id_resolve("potion-of-healing", RuleSetId::Crb, corpus)` returns `Some((record, Some(table_ref), RuleSetId::Crb))` directly (CRB is queried first, no fallback).
- `equipment_id_resolve("apg-only-spell", RuleSetId::Crb, corpus)` returns `Some((record, Some(table_ref), RuleSetId::Apg))` — CRB query fails, fallback hits APG.
- `equipment_id_resolve("nonexistent", RuleSetId::Apg, corpus)` returns `None` — all three books queried, no match.
- Priority order is strict: APG-key never falls through to ACG without first checking CRB; CRB-key never falls through to APG without first hitting CRB (which it does directly, so the fallback only fires when CRB misses).
- `resolved_from_rule_set` is always set to the actual matching book, never the requested `rule_set` parameter.

**What this means for SD-19's existing tests.** SD-19's resolver tests assert the prior 2-tuple return shape. The cross-book version breaks that contract. The migration is: update the resolver signature, update all call sites, regenerate the tests. SD-22's Epic 1 (APG content-source ingest) is the natural place to land this — campaign manager is the first consumer that benefits from cross-book resolution (a DM's campaign references spells across books). Alternatively, a small SD-19-follow-on slices the resolver change in isolation. Operator call; the technical-design captures the shape either way.

**Displaced alternative (prior default, superseded):** Each `RuleSetId` is queried independently; no fallback. The cross-book resolution is a higher-level feature that can land later. This is what SD-19 shipped; Q5's PINNED supersedes it.

**Displaced alternative (also displaced):** Per-book scoping with a separate "find anywhere" resolver variant. Two resolver APIs, GUI has to pick which to call. Rejected because it doesn't match the "find this thing" user mental model.

## 4. Cross-cutting authority surface

| Epic | Authoritative for | Forbidden to fabricate |
|---|---|---|
| Epic 1 — Code-Side Identifier Cleanup | Identifier renames in source (Rust Tauri commands, TS functions/constants, `data-testid`, inline doc-comments) per the identifier-discipline doctrine (`governance/identifier-discipline.md`) | Any feature work. Epic 1 only renames identifiers; doesn't change behavior, doesn't add tests beyond what the rename requires. |
| Epic 2 — Campaign manager + Drive persistence | `CampaignSnapshot` shape, `CampaignBackend` trait, Drive adapter, markdown file layout | Anything outside `CampaignSnapshot`. Per-character detailed stats (Epic 2 reads `CharacterSummary` from SD-19's `rules_tables/crb/` table store, not from SD-20 receipt). |
| Epic 3 — Update UI bug remediation | Affected code at `apps/desktop/src/sd16/update/controllerAdapter.ts`, `fetch.ts`, `CheckPanel.tsx`, `Ui.tsx` plus their Tauri Rust counterparts | Any change to the spec-domain lifecycle doctrine itself. Re-opening closed spec domains. APG/ACG ingestion (those live in SD-22). |
| Epic 4 — Closure Epilogue | Final criterion scan + closure PR (`tranche/4-1 → develop`) + worktree cleanup + release-notes generation + tranche-position version increment | The specific `<major>.<tranche-base>.<build>` value (that's Epic 5's surface). Worktree cleanup outside the SD-21 lane. Auto-merging the closure PR (`decisions.md` §6 no-branches convention). The per-CI-build *build*-counter increment and the per-main-publish *major*-position increment (those are operator-pinned at cycle launch and a future bundle's epic respectively; Epic 4's job is the *tranche*-position increment on tranche promotion). |
| Epic 5 — Build Version Numbering | Three version fields (package.json, tauri.conf.json, Cargo.toml) set to `0.4.<current_build>`, build-label format (`Codex 0.4.<build>` instead of `codex@0.0.0`), `docs/SD-21/release-closure-checklist.md` | Per-CI-build *build*-counter automation (operator-pinned at cycle launch; future bundle's epic). Major-publish automation (future bundle's epic). Build-label parsing anywhere in the codebase (the format is presentation-only). |
| Epic 6 — Single-class coverage completion | Rules-engine core at `src/rules_core/pilot_compute.rs:4568` — `compute_pilot_base_chassis` dispatch + `compute_wizard_chassis` extension to level 1-20. Per-class foundation module shapes | Multiclass dispatch (that's Epic 7's surface). PCGen re-ingestion. GUI changes. Charter module rework that fixes per-class function bugs (those get filed as separate bugs). |
| Epic 7 — Multiclass stacking | Length-2+ `class_levels` dispatch via `compute_multiclass_base_chassis`; BAB stacking; PF1 best-fractional-progression save stacking (via `decideEligibility.class_save_bonus`); per-class feature integration | Multiclass spell-stacking edge cases. Triple-class (length-3+). Multiclass skill-point allocation if it surfaces as a bug. |

APG / ACG / advanced-guide / Bestiary 1 content-source ingest is **SD-22's surface** (`programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/`). SD-21's Epic 2 reads `rules_tables/crb/` only; SD-22 owns every other `RuleSetId::*` content.

## 5. File-touch partition (defense for the loop)

Per-epic module placement:

- **Epic 1 — Identifier Cleanup**: `apps/desktop/src-tauri/src/sd*_*.rs` (Rust Tauri command name updates + their JS invoke-string callers in `apps/desktop/src/sd16/` + their test-assertion strings in `*.test.ts`); `apps/desktop/src/` TS function and constant renames; `apps/desktop/src/` `data-testid` and inline doc-comment cleanup. One-cycle-at-a-time per file.
- **Epic 2 — Campaign Manager + Drive persistence**: `src/rules_core/campaign.rs` (NEW; the `CampaignSnapshot` types), `src/rules_core/persistence/mod.rs` (NEW; backend-agnostic trait), `src/rules_core/persistence/drive.rs` (NEW; Drive adapter), `apps/desktop/src-tauri/src/campaign_drive.rs` (NEW; Tauri commands), `apps/desktop/src/boundary/campaignDrive.ts` (NEW; GUI boundary).
- **Epic 3 — Update UI bug remediation**: `apps/desktop/src/sd16/update/fetch.ts` (release-notes fetch path), `apps/desktop/src/sd16/update/controllerAdapter.ts` (probe + `computeDecision` rewiring), `apps/desktop/src/sd16/update/CheckPanel.tsx` (render branch). Tauri Rust companion commands (`is_install_eligible`, `perform_install`) added under `apps/desktop/src-tauri/src/`. Per Epic 1's renames, identifiers in this code already carry the new descriptive names.
- **Epic 4 — Closure Epilogue**: `docs/release/SD-21/release-notes.md` (relocated 2026-07-20; NEW, generated by the loop's release-notes generator) + the closure PR itself (the loop runs `gh pr create` + worktree cleanup + branch sweep). No code surface.
- **Epic 5 — Build Version Numbering**: `apps/desktop/package.json` (version field set to `0.4.<current_build>`), `apps/desktop/src-tauri/tauri.conf.json` (version field set to `0.4.<current_build>`), `apps/desktop/src-tauri/Cargo.toml` (version field set to `0.4.<current_build>`), `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts:61` (`BUILD_PREFIX = 'Codex'`), `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts:72-74` (template `${BUILD_PREFIX} ${buildVersion}` rendering `<major>.<tranche>.<build>` as space-separated display), three test-fixture files. New `docs/SD-21/release-closure-checklist.md` (per-position increment rules: build per-CI-build, tranche per-tranche-promotion, major per-main-publish).
- **Epic 6 — Single-class coverage completion**: `src/rules_core/pilot_compute.rs:4568` (Epic 6's refactor of `compute_pilot_base_chassis`), `src/rules_core/pilot_compute.rs:13967` (`compute_wizard_chassis` extension to level 1-20). One-cycle-at-a-time per sub-feature.
- **Epic 7 — Multiclass stacking**: `src/rules_core/pilot_compute.rs` (length-2+ dispatch via `compute_multiclass_base_chassis`), the per-class feature integration surface in `src/rules_core/`. Per-cycle tests at `tests/sd21_multiclass_<X>_<Y>_chassis_computes.rs`.

The chassis and corpus-aware seam files (`pilot_compute.rs`, `pilot_compute_corpus.rs`, `support_state_matrix.rs`) stay untouched. The engine's `CampaignSnapshot` types live in a new module and don't touch the chassis surface.

## 6. Cross-reference

- `acceptance-and-verification.md` — 13 closure gates.
- `decisions.md` — the 21-item decision record.
- `epic-breakdown.md` — 30 acceptance criteria grouped into 7 epics.
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags.
- `technical-requirements.md` — pre-loop prerequisites.
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
- `../SD-19/` — corpus-aware seam + canonical Paizo-table store (CRB).
- `../SD-22/` — sibling bundle (advanced guides + APG + ACG + Bestiary 1 + DM toolkit; owns the cross-book ladder).
- `../SD-20/` — sibling bundle (parallel; per-character tabletop-readiness).
