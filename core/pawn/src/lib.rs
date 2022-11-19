//! Pawns are controllable entities by either players or AI.

/// Manage pawn actions.
mod actions;
/// Manage examine events.
mod examine_events;
/// The networking module of this crate.
pub mod networking;
/// Pawn resources.
pub mod pawn;
/// The Bevy plugin of this crate.
pub mod plugin;
