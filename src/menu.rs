use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::style::main_menu::{get_button_style, get_button_text_style};
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(
                Update,
                (
                    click_play_button.run_if(in_state(GameState::Menu)),
                    play_on_confirm.run_if(in_state(GameState::Menu)),
                ),
            )
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

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

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

#[derive(Bundle)]
struct GameButton {
    button_bundle: ButtonBundle,
    button_colors: ButtonColors,
    change_state: ChangeState,
}

impl Default for GameButton {
    fn default() -> Self {
        GameButton {
            button_bundle: ButtonBundle {
                style: get_button_style(),
                background_color: ButtonColors::default().normal.into(),
                ..Default::default()
            },
            button_colors: ButtonColors::default(),
            change_state: ChangeState(GameState::Playing),
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    info!("menu");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn(GameButton::default())
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Play", get_button_text_style()));
                });
        });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..Default::default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Made with Bevy",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::linear_rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.bevy.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::linear_rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/NiklasEi/bevy_game_template"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Open source",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::linear_rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.github.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
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

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn play_on_confirm(mut state: ResMut<NextState<GameState>>, actions: Res<Actions>) {
    if actions.confirm {
        state.set(GameState::Playing);
    }
}
