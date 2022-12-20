extern crate bevy_economy;

use bevy::{prelude::*, window::close_on_esc};
use bevy_economy::EconomyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EconomyPlugin)
        .add_startup_system(seed_world)
        .add_system(close_on_esc)
        .run();
}

fn seed_world() -> Result<()> {
    use fake::faker::name::raw::*;
    use fake::locales::*;

    let agents = (0..10).into_iter().map(|_| NewAgent::new(Name(EN).fake()));
    Ok(())
}
