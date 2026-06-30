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
    /// Reference to the latest authoritative revision in the lineage.
    pub latest_authoritative_revision_ref: String,
    /// Human-facing summary handle for this saved character.
    pub display_label: String,
    /// The authoritative user-authored character choices.
    pub character_input: CharacterInput,
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
