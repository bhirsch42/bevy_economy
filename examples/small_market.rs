extern crate bevy_economy;

use bevy::{prelude::*, window::close_on_esc};
use bevy_economy::EconomyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EconomyPlugin)
        .add_system(close_on_esc)
        .run();
}
