//! The per-system layout files (operator ruling R2): which blocks a system's sheet prints, in which
//! order, with which labels. E8.1 holds Pathfinder's; E8.3 adds Starfinder's and Cyberpunk RED's.

pub mod pf1e;

/// A value the sheet shows with its sign, as the screen's tiles write it: `+3`, `0` stays `+0`, `-1`.
/// Formatting only; the number is the engine's.
pub(crate) fn signed(value: i16) -> String {
    if value >= 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

/// What a cell prints when the engine holds no value for it: a dash, never a guessed number.
pub(crate) const NO_VALUE: &str = "—";
