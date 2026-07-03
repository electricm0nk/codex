//! SD-16 update state: AppImage staged transaction, installed-state, and pending-update plumbing.
//!
//! The transaction module is the authoritative staging engine for the AppImage install path.
//! It performs a bounded, atomic-replace transaction and writes the pending-update marker that
//! the relaunch verifier consumes on the next launch.
//!
//! The Tauri command layer is intentionally not wired here — that is a separate slice. This
//! module is pure-Rust, filesystem-driven, and dependency-injectable so cargo tests can drive the
//! transaction end-to-end against fixture AppImages without a Tauri runtime.

// The transaction module's public surface is library-complete but is not yet referenced from
// any `#[tauri::command]` in main.rs (that wiring lands in a separate slice). Without this
// allowance the `cargo build` (non-test) profile flags every symbol as dead. Test builds do
// exercise the public surface and will surface real unused-symbol warnings naturally.
#![cfg_attr(not(test), allow(dead_code))]

pub mod transaction;
