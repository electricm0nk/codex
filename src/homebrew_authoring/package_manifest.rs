//! Manifest surface for the first bounded GE-08 authored package lifecycle.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackageValidationState {
    Draft,
    Valid,
    Invalid,
    Deferred,
}

impl PackageValidationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
            Self::Deferred => "deferred",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "draft" => Some(Self::Draft),
            "valid" => Some(Self::Valid),
            "invalid" => Some(Self::Invalid),
            "deferred" => Some(Self::Deferred),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofBinding {
    pub case_id: String,
    pub slot: String,
    pub remove: String,
    pub add: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageManifest {
    pub schema_version: u16,
    pub package_id: String,
    pub package_title: String,
    pub game_system_id: String,
    pub package_version: String,
    pub depends_on: Vec<String>,
    pub supported_object_kinds: Vec<String>,
    pub validation_state: PackageValidationState,
    pub provenance_policy: String,
    pub proof_binding: ProofBinding,
}
