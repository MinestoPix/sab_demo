use bevy::prelude::*;

use crate::{menus::GameStateButton, GameState};

use super::{ButtonColors, ChangeState};

pub(super) struct MapGenMenuPlugin;

impl Plugin for MapGenMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MapGeneration), setup_map_gen_menu)
            .add_systems(
                Update,
                (
                    handler_button_click.run_if(in_state(GameState::MapGeneration)),
                    // play_on_confirm.run_if(in_state(GameState::Menu)),
                ),
            )
            .add_systems(OnExit(GameState::MapGeneration), cleanup_map_gen_menu);
    }
}

#[derive(Component)]
struct MapGenMenu;

#[derive(Component)]
struct MapGenAction {
    action: fn(&mut Commands),
}

#[allow(dead_code)]
pub fn get_style_button() -> Style {
    Style {
        min_width: Val::Px(140.0),
        min_height: Val::Px(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(5.)),
        padding: UiRect::all(Val::Px(5.)),
        ..Default::default()
    }
}

pub fn get_style_button_text() -> TextStyle {
    TextStyle {
        font_size: 40.0,
        color: Color::linear_rgb(0.9, 0.9, 0.9),
        ..default()
    }
}

pub fn get_style_container() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Start,
        justify_content: JustifyContent::Start,
        ..default()
    }
}
fn setup_map_gen_menu(mut commands: Commands) {
    info!("Setting up map generation menu");
    commands
        .spawn((
            NodeBundle {
                style: get_style_container(),
                ..default()
            },
            MapGenMenu,
        ))
        .with_children(|child| {
            child
                .spawn(GameStateButton::new(GameState::Menu))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Back", get_style_button_text()));
                });
        });
}

fn handler_button_click(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&MapGenAction>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, map_gen_action) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(map_gen_action) = map_gen_action {
                    info!("Running action {:?}", map_gen_action.action);
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_map_gen_menu(mut commands: Commands, menu: Query<Entity, With<MapGenMenu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
