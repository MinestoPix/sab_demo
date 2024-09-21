use bevy::prelude::*;

pub fn get_style_button() -> Style {
    Style {
        width: Val::Px(140.0),
        height: Val::Px(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
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

pub fn get_style_link_button() -> Style {
    Style {
        width: Val::Px(170.0),
        height: Val::Px(50.0),
        justify_content: JustifyContent::SpaceAround,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(5.)),
        ..Default::default()
    }
}

pub fn get_style_link_button_text() -> TextStyle {
    TextStyle {
        font_size: 15.0,
        color: Color::linear_rgb(0.9, 0.9, 0.9),
        ..default()
    }
}

pub fn get_style_link_button_image() -> Style {
    Style {
        width: Val::Px(32.),
        ..default()
    }
}
