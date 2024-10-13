#![warn(clippy::all, clippy::pedantic)]

mod player;
mod tileMapRules;
use godot::{init::ExtensionLibrary, prelude::gdextension};

struct MatDev;

#[gdextension]
unsafe impl ExtensionLibrary for MatDev {}
