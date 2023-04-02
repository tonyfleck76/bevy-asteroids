use bevy::prelude::*;

use crate::{global::state::AppState, main_menu::constants::*};

type InteractionQualifiers = (Changed<Interaction>, With<Button>);

pub fn input(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), InteractionQualifiers>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
