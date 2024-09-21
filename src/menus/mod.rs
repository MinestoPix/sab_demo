use bevy::prelude::*;
use style::main_menu::get_style_button;

use crate::GameState;

mod main_menu;
mod map_gen_menu;
mod style;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera) // TODO: Move this up
            .add_plugins(main_menu::MainMenuPlugin)
            .add_plugins(map_gen_menu::MapGenMenuPlugin);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Bundle)]
struct GameStateButton {
    button_bundle: ButtonBundle,
    button_colors: ButtonColors,
    change_state: ChangeState,
}

impl Default for GameStateButton {
    fn default() -> Self {
        GameStateButton {
            button_bundle: ButtonBundle {
                style: get_style_button(),
                background_color: ButtonColors::default().normal.into(),
                ..Default::default()
            },
            button_colors: ButtonColors::default(),
            change_state: ChangeState(GameState::Playing),
        }
    }
}

impl GameStateButton {
    fn new(state: GameState) -> Self {
        GameStateButton {
            change_state: ChangeState(state),
            ..GameStateButton::default()
        }
    }
}
