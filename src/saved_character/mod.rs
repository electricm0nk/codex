//! Saved-character envelope and local-store boundary for SD14-E1-F1.
//!
//! This module carries a typed saved-character envelope over the existing
//! CharacterInput authoritative payload, along with a deterministic local-store
//! boundary for save/load operations.

pub mod local_store;

use crate::rules_core::character_input::CharacterInput;

/// The saved-character envelope carrying identity, revision, provenance, and
/// the authoritative CharacterInput payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedCharacterEnvelope {
    /// Stable identity that survives rename, reopen, and revision creation.
    pub character_id: String,
    /// Identity of the exact saved revision.
    pub revision_id: String,
    /// Whether this revision is authoritative, autosave, recovery, or other governed state.
    pub revision_kind: SavedCharacterRevisionKind,
    /// Save timestamp for this revision.
    pub saved_at: String,
    /// Persisted-artifact schema version.
    pub schema_version: u16,
    /// Version or build identity that produced this revision.
    pub app_or_runtime_version: String,
    /// Package/content/rules lineage needed to classify compatibility honestly.
    pub content_or_rules_provenance: String,
    /// Explicit game-system identifier (e.g. "pf1"), distinct from
    /// `content_or_rules_provenance`'s content-package lineage. Added at
    /// schema_version 2; envelopes saved before then have no on-disk
    /// `game_system` line and get an honest back-compat default on load
    /// (see `local_store::derive_legacy_game_system`).
    pub game_system: String,
    /// Reference to the latest authoritative revision in the lineage.
    pub latest_authoritative_revision_ref: String,
    /// Human-facing summary handle for this saved character.
    pub display_label: String,
    /// The authoritative user-authored character choices.
    pub character_input: CharacterInput,
}

/// Persisted-artifact schema version written by new saves as of the addition
/// of the `game_system` field. Older schema_version=1 envelopes remain
/// readable via a back-compat default in `local_store::parse_envelope`.
pub const CURRENT_SAVED_CHARACTER_SCHEMA_VERSION: u16 = 2;

/// Summary of one saved character, as returned by
/// `local_store::SavedCharacterStore::list_all`. Carries the fields a
/// character-list UI needs without loading the full `CharacterInput` payload
/// twice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedCharacterSummary {
    pub character_id: String,
    pub display_label: String,
    pub game_system: String,
    pub schema_version: u16,
    pub saved_at: String,
    pub race_id: String,
    /// Joined `class_id:level` pairs, comma-separated (Phase 1's golden path
    /// is always single-class, but this stays correct for future multiclass).
    pub class_summary: String,
}

/// The result of listing every saved character under a characters-root
/// directory. One unreadable entry never fails the whole listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedCharacterListing {
    pub characters: Vec<SavedCharacterSummary>,
    pub unreadable_entries: Vec<SavedCharacterListingError>,
}

/// One saved-character subdirectory that could not be loaded as part of a
/// listing, named so a caller can report which entry is corrupt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedCharacterListingError {
    pub entry_name: String,
    pub message: String,
}

/// Classification of a saved revision's role in the character lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedCharacterRevisionKind {
    /// The latest accepted user-authored save, valid for normal reopening.
    Authoritative,
}

impl SavedCharacterRevisionKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
        }
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "authoritative" => Some(Self::Authoritative),
            _ => None,
        }
    }
}

/// Error returned when a saved-character save or load operation cannot complete honestly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedCharacterStoreError {
    pub message: String,
}

impl std::fmt::Display for SavedCharacterStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for SavedCharacterStoreError {}
