//! Inner Sea Magic (ISM). SD-32 Gate 0 book-onboarding precondition
//! (`gate-0-book-onboarding-precondition`, AT-32-G0-003) -- first compiled
//! rule set of any kind for this book (its `class_feature` corpus JSON
//! already exists on disk from an earlier cache-gen lane, but no
//! `RuleSetId` existed to unlock the book-level gate for it -- see
//! `RuleSetId::InnerSeaMagic`'s own doc comment). First slice: the base
//! spell declarations in `ism_spells.lst`. See `spell_list`'s own module
//! doc comment and `src/bin/ingest_inner_sea_setting_spells.rs` for the
//! ingest path.

pub mod spell_list;
