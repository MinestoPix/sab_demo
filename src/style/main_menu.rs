use bevy::prelude::*;

pub fn get_button_style() -> Style {
    Style {
        width: Val::Px(140.0),
        height: Val::Px(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    }
}

pub fn get_button_text_style() -> TextStyle {
    TextStyle {
        font_size: 40.0,
        color: Color::linear_rgb(0.9, 0.9, 0.9),
        ..default()
    }
}
