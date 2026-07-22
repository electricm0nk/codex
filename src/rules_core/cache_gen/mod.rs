//! JSON cache generation (SD-26 Epic 3). Per-book generator modules live
//! here (`apg`, and siblings added by sibling per-book cycles). Each
//! generator DUMPS the current runtime state of its book's already-landed
//! `rules_tables` module to `data/corpus/<book>/**/*.json` -- it never
//! re-derives field values by re-parsing raw PCGen LST (`decisions.md
//! §11.3`). The only reason any of this code touches the real LST corpus
//! at all is to recover a real, checkable line-number *citation* for a
//! value that is already known (from the compiled Rust module) to be
//! correct -- never to compute the value itself.

pub mod apg;
