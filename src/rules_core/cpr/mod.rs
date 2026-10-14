//! Cyberpunk RED (`GameSystem::CyberpunkRed`, id `cyberpunk-red`).
//!
//! - [`input`]: `CprCharacterInput`, the player's choices, and its text grammar.
//! - [`store`]: `CprSavedCharacter` save/load/list/delete. A CPR character is its own
//!   directory under the same characters root as Pathfinder and Starfinder characters,
//!   holding `envelope.txt` (the saved-character envelope grammar, `game_system =
//!   cyberpunk-red`) and `cpr_character_input.txt`.
//! - [`sheet`]: the `cpr.` explanation rows the character sheet renders. Every number on a
//!   CPR sheet is one of these rows; no frontend code re-derives one.
//!
//! The engine modules each lane adds go under that lane's anchor below, one `pub mod` line
//! per module, so two lanes never edit the same line.

pub mod input;
pub mod sheet;
pub mod store;

// lane A modules
pub mod stats;

// lane B modules
pub mod cyberware;
pub mod roles;

// lane C modules
pub mod netrunning;
