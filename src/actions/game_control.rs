use bevy::prelude::{ButtonInput, KeyCode, Res};

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Exit,
}

impl GameControl {
    pub fn pressed(&self, keyboard_input: &Res<ButtonInput<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight)
            }
            GameControl::Confirm => {
                keyboard_input.just_pressed(KeyCode::Enter) || keyboard_input.just_pressed(KeyCode::KeyE)
            },
            GameControl::Exit => {
                keyboard_input.just_pressed(KeyCode::KeyQ) || keyboard_input.just_pressed(KeyCode::Escape)
            },
        }
    }
}

pub fn get_movement(control: GameControl, input: &Res<ButtonInput<KeyCode>>) -> f32 {
    if control.pressed(input) {
        1.0
    } else {
        0.0
    }
}

pub fn get_exit(input: &Res<ButtonInput<KeyCode>>) -> bool {
    GameControl::Exit.pressed(input)
}

pub fn get_confirm(input: &Res<ButtonInput<KeyCode>>) -> bool {
    GameControl::Confirm.pressed(input)
}
