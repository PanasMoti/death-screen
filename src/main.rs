#![windows_subsystem = "windows"]
mod setup;
use bevy::prelude::*;
use setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .run();
}
