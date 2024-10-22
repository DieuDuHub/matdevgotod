#![warn(clippy::all, clippy::pedantic)]

mod player;
mod tile_map_rules;
mod player_advanced;
use godot::{init::ExtensionLibrary, prelude::gdextension};

struct MatDev;

#[gdextension]
unsafe impl ExtensionLibrary for MatDev {}
