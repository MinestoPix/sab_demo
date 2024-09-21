use bevy::prelude::*;

mod main_menu;
mod map_gen_menu;
mod style;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera) // TODO: Move this up
            .add_plugins(main_menu::MainMenuPlugin);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
