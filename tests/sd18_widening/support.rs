//! Shared per-test setup helper for the sd18_widening family (SD-36 Epic C2
//! table-driven rewrite, instruction step 3 — "move the repeated setup ...
//! into shared helper fns", leaving each bespoke test's own assert lines
//! untouched and moved verbatim).
//!
//! Nearly every test in this family opened with the same two lines:
//! `let input = load(FIXTURE); let computation =
//! compute_pilot_base_chassis(&input);` — this collapses that into one.

use crate::common::load;
use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};

/// Load `fixture` and run it through `compute_pilot_base_chassis`.
pub fn compute(fixture: &str) -> PilotBaseChassisComputation {
    let input = load(fixture);
    compute_pilot_base_chassis(&input)
}
