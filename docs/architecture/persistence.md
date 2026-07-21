# Persistence

> Scope: how saved characters and campaigns are typed, stored on disk, and reached from the desktop shell.
> Last verified: 2026-07-20 against ef9012bf5de8
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

This covers the two local-store boundaries under `src/`: `src/saved_character/`
(one saved character) and `src/campaign/` (one campaign). Both are headless —
no Tauri, no serde-free-for-all — and both follow the same shape: a typed
in-memory record, a concrete zero-field `*Store` struct with associated
save/load/list functions, and a documented on-disk layout.

## `saved_character/`: the envelope over `CharacterInput`

`src/saved_character/mod.rs` defines `SavedCharacterEnvelope` — identity,
revision, provenance, and schema metadata wrapped around the single
authoritative payload: `character_input: CharacterInput` (from
`crate::rules_core::character_input`). The envelope is not a new
representation of a character; it is bookkeeping (`character_id`,
`revision_id`, `revision_kind: SavedCharacterRevisionKind`, `saved_at`,
`schema_version`, `app_or_runtime_version`, `content_or_rules_provenance`,
`game_system`, `latest_authoritative_revision_ref`, `display_label`) plus that
one `CharacterInput` field. `CURRENT_SAVED_CHARACTER_SCHEMA_VERSION` is `2`;
schema_version 1 envelopes (no `game_system` line) still load, via
`local_store::derive_legacy_game_system`, which derives a short id like
`"pf1"` from the `content_or_rules_provenance` lineage prefix.

`SavedCharacterRevisionKind` currently has exactly one variant,
`Authoritative` — there is no autosave/recovery revision kind implemented yet,
only the type headroom for one.

### On-disk bundle layout

`src/saved_character/local_store.rs` defines `SavedCharacterStore`, a
concrete unit struct (`pub struct SavedCharacterStore;`) with associated
functions `save`, `load`, and `list_all`. Its own module doc comment states
the layout precisely: one directory (the bundle root) containing exactly two
files, named by the constants at the top of the file —

```rust
const ENVELOPE_FILE: &str = "envelope.txt";
const CHARACTER_INPUT_FILE: &str = "authoritative_character_input.txt";
```

- `envelope.txt` — `key = value` lines, one per envelope field (see
  `render_envelope`/`parse_envelope`).
- `authoritative_character_input.txt` — the `CharacterInput` rendered through
  the same hand-rolled `key=value` fixture grammar used elsewhere in
  `rules_core` (`render_character_input` writes it; `load_character_input_fixture`
  from `crate::rules_core::character_input` reads it back). The grammar itself
  — required-field diagnostics, repeatable keys, colon-segment choice
  encoding — is documented in [testing.md](./testing.md); this module only
  consumes it.

`SavedCharacterStore::save` refuses to write a record it cannot honestly read
back: `validate_character_input` rejects any field containing a newline (the
grammar is line-based) and enforces that `selected_choices` entries have the
exact colon-segment shape the loader expects (`choice_set_id` = exactly two
segments, `selection_id` = at least two) — see the doc comment directly above
`validate_character_input` in `local_store.rs`. `SavedCharacterStore::save`
also validates the envelope fields are single-line before writing
`envelope.txt`.

`SavedCharacterStore::list_all(characters_root)` walks every subdirectory of
`characters_root`, sorted by file name, and calls `load` on each
independently. A `NotFound` root returns an empty `SavedCharacterListing`
(not an error — no characters yet is not a failure). One unreadable
subdirectory is collected into `SavedCharacterListing::unreadable_entries`
(as a `SavedCharacterListingError { entry_name, message }`) without failing
the rest of the listing.

## `campaign/`: `CampaignSnapshot` and the campaign store

`src/campaign/mod.rs`'s own doc comment states the design intent directly:
campaigns are not rules-computation, so this module lives as a sibling of
`saved_character` rather than under `rules_core`. `CampaignSnapshot`'s fields
are documented as mirroring `apps/desktop/src/campaign/campaignModel.ts`'s
`Campaign` + `CampaignAssets` TypeScript types 1:1 (`id`, `name`,
`rule_set_id`, `rule_set_label`, `description`, `members: Vec<CampaignMember>`,
`party_character_ids: Vec<String>`, `created_at`, `updated_at`,
`assets: CampaignAssets`), plus a `schema_version` field added from day one
(`CURRENT_CAMPAIGN_SCHEMA_VERSION = 1`) — there is no legacy campaign format
to be back-compatible with, unlike `saved_character`. `CampaignAssets` holds
four `Vec<CampaignAsset>` lists — `resources`, `adventure_log`, `maps`,
`wiki` — each `CampaignAsset { title, body }`. This is the one deliberate
exception to the "1:1" framing: `campaignModel.ts`'s asset shape is
`MarkdownAsset { id, title, body, updatedAt }`, and `CampaignAsset` drops the
UI-local `id`/`updatedAt` bookkeeping fields, per `src/campaign/mod.rs`'s own
`CampaignAsset` doc comment. All types derive
`Serialize`/`Deserialize` with `#[serde(rename_all = "camelCase")]`, and a
unit test (`serializes_camel_case_field_names_matching_campaign_model_ts`)
asserts the exact camelCase wire shape (`ruleSetId`, `partyCharacterIds`,
etc.).

### On-disk layout and `CampaignStore`

`src/campaign/local_store.rs` defines `CampaignStore` (also a concrete unit
struct) with `save`, `load`, `list_all`, `delete`, `save_under_root`,
`load_with_nonce`, `save_with_conflict_detection`, and
`save_under_root_with_conflict_detection`. Its module doc comment gives the
layout under a campaign's own directory:

```text
<campaign_dir>/
  .config/<sanitized name>.json   # CampaignSnapshot minus `assets`
  resources/<sanitized title>.md
  adventure-log/<sanitized title>.md
  maps/<sanitized title>.md
  wiki/<sanitized title>.md
```

The JSON config carries every `CampaignSnapshot` field except `assets`
(`config_only.assets = CampaignAssets::default()` before serializing); each
markdown asset is written verbatim as its own `.md` file, named from
`sanitize_filename(&asset.title)`. `CampaignStore::load` re-reads the `.md`
files fresh on every call — an edit made outside the app (e.g. in Obsidian)
between save and load is honored, per the module doc comment and the
`load_honors_an_external_obsidian_style_edit_to_an_asset_markdown_file` test.
Empty asset groups never get a subdirectory (`write_asset_group` early-returns
on an empty slice).

`CampaignStore::list_all(campaigns_root)` mirrors
`SavedCharacterStore::list_all` exactly: a missing root returns an empty
`CampaignListing` rather than an error, and each subdirectory load failure is
isolated into `CampaignListing::unreadable_entries` without failing the rest
of the listing.

### Conflict detection lives in `campaign/local_store.rs`, not `campaign_drive.rs`

Nonce-based conflict detection is implemented in this module, not in the
desktop-side `campaign_drive.rs`. A revision nonce is written to a sidecar
file, `NONCE_FILE = "nonce"` under `.config/`, deliberately kept out of the
JSON config and out of `CampaignSnapshot`'s own fields (it has no
`campaignModel.ts` counterpart). `save_with_conflict_detection(snapshot,
campaign_dir, expected_nonce)` compares `expected_nonce` against the nonce
currently on disk; on a mismatch it calls
`move_existing_state_to_conflicts`, which moves the existing `.config` +
four asset directories into `<campaign_dir>/conflicts/<unix-nanos
timestamp>/` before the new snapshot is written as the active state. Local
always wins; both copies are preserved for manual review (the doc comment
above `save_with_conflict_detection` cites `decisions.md` §7 for this
policy). `expected_nonce: None` (a brand-new campaign) never triggers a
conflict.

## Reaching these stores from the desktop

`apps/desktop/src-tauri/src/character_hub.rs` wraps `SavedCharacterStore`
behind Tauri commands (`create_character`, `clone_character`,
`list_saved_characters`, `load_saved_character`, plus the portrait commands
below); `apps/desktop/src-tauri/src/campaign_drive.rs` wraps `CampaignStore`
behind `write_campaign_drive_artifacts`, `drive_list_campaigns`,
`drive_load_campaign`, `drive_save_campaign`, and `drive_delete_campaign`.
`campaign_drive.rs`'s own module doc comment describes itself as "the thin
Tauri-command adapter over the headless `codex::campaign` crate ... it
deserializes the frontend's already-JSON campaign payloads into a typed
`CampaignSnapshot` and delegates all real file I/O to
`codex::campaign::local_store::CampaignStore`". The full command inventory
and request/response DTO shapes are catalogued in
[desktop-app.md](./desktop-app.md); this file only names the entry points.

Characters root on disk: `character_hub.rs`'s
`characters_root_from_app_data_dir` joins the OS app-data directory with a
fixed `CHARACTERS_ROOT_DIR_NAME = "characters"` subdirectory
(`resolve_characters_root` resolves the real `tauri::AppHandle` app-data
path; `resolve_character_root` joins one more path segment, `character_id`).
Campaigns root: there is no fixed app-data subdirectory — every
`campaign_drive.rs` command takes a `drive_folder_path` (a user-configured
local directory; the name reflects a not-yet-implemented Google Drive sync
feature — see the module doc comment's note that "Google OAuth / Drive
API integration does not exist ... the 'Drive folder' is really just a local
path").

### Portrait storage

`character_hub.rs` stores a character's portrait as `portrait.png`
(`const PORTRAIT_FILE_NAME: &str = "portrait.png"`) written directly into
that character's own bundle directory — the same directory as `envelope.txt`
and `authoritative_character_input.txt` — via `save_character_portrait`,
`load_character_portrait`, and `delete_character_portrait`. `save_character_portrait`
requires the character to already exist (`root.exists()` check) — a portrait
is never the first write to a character directory, per the doc comment above
it. Portraits are capped at `MAX_PORTRAIT_BYTES = 3 * 1024 * 1024` bytes as a
defensive backstop (the frontend crops/resizes before sending bytes).
`load_character_portrait` returns the bytes re-encoded as a
`data:image/png;base64,...` URL, or `None` if no portrait file exists.

## Design rule: no `*Backend` trait, no trait-object indirection

Both stores are concrete zero-field structs with associated functions, not
trait objects behind a `dyn *Backend` interface — the standing rule for any
persistence backend in this codebase. Full statement of the rule, its source
citation, and the "when to introduce a trait seam" guidance: see
[conventions.md](./conventions.md) §"Concrete zero-field `*Store` structs,
no `*Backend` trait."
