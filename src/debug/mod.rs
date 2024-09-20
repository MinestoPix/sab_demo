use bevy::prelude::*;

use crate::{actions::Actions, GameState};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (back_to_menu.run_if(in_state(GameState::Playing)),
            exit_app.run_if(in_state(GameState::Menu))
        ));
    }
}

fn back_to_menu(mut state: ResMut<NextState<GameState>>, actions: Res<Actions>) {
    if actions.exit {
        state.set(GameState::Menu)
    }
}

fn exit_app(mut exit_events: EventWriter<AppExit>, actions: Res<Actions>) {
    if actions.exit {
        exit_events.send(AppExit::Success);
    }
}
