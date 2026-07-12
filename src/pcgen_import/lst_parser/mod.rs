//! PCGen LST parsers, partitioned by object kind.

pub mod class;
pub mod metadata;

// Preserve the slice APIs while avoiding collisions between their diagnostic
// types. The metadata API retains the umbrella names because it landed first;
// the class parser's diagnostic types remain available through `class`.
pub use class::{
    ClassEntry, ClassFeatureBlock, ClassLevelLine, ClassParseResult, ClassToken,
    MARTIAL_CLASS_NAMES, parse_class_entries, parse_class_file,
};
pub use metadata::{
    LstDiagnostic, LstDiagnosticKind, LstMetadataDocument, LstRecord, MetadataKind,
    parse_lst_metadata, parse_lst_metadata_text,
};
