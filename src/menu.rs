use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::style::main_menu::{
    get_style_button, get_style_button_text, get_style_link_button_image, get_style_link_button,
    get_style_link_button_text,
};
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

#[derive(Bundle)]
struct LinkButton {
    button_bundle: ButtonBundle,
    button_colors: ButtonColors,
    open_link: OpenLink,
}

impl Default for GameButton {
    fn default() -> Self {
        GameButton {
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

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

impl Default for LinkButton {
    fn default() -> Self {
        LinkButton {
            button_bundle: ButtonBundle {
                style: get_style_link_button(),
                background_color: Color::NONE.into(),
                ..Default::default()
            },
            button_colors: ButtonColors {
                normal: Color::NONE,
                ..Default::default()
            },
            open_link: OpenLink("https://bevyengine.org"),
        }
    }
}

impl LinkButton {
    fn link(link: &'static str) -> Self {
        LinkButton {
            open_link: OpenLink(link),
            ..LinkButton::default()
        }
    }
}

fn spawn_link_children_txt_img(
    children: &mut ChildBuilder<'_>,
    link_button: LinkButton,
    link_text: &str,
    img_handle: Handle<Image>,
) {
    children.spawn(link_button).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            link_text,
            get_style_link_button_text(),
        ));
        parent.spawn(ImageBundle {
            image: img_handle.into(),
            style: get_style_link_button_image(),
            ..default()
        });
    });
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
                    parent.spawn(TextBundle::from_section("Play", get_style_button_text()));
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
            spawn_link_children_txt_img(
                children,
                LinkButton::default(),
                "Made with Bevy",
                textures.bevy.clone(),
            );
            spawn_link_children_txt_img(
                children,
                LinkButton::link("https://github.com/NiklasEi/bevy_game_template"),
                "Bevy Engine",
                textures.github.clone(),
            );
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
