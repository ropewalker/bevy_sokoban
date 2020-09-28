use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn gameplay_state_system(
    mut gameplay: ResMut<Gameplay>,
    mut box_spot_query: Query<(&BoxSpot, &Position)>,
    mut box_query: Query<(&Box, Changed<Position>)>,
) {
    let occupied_positions_with_colors = box_query
        .iter()
        .iter()
        .map(|t| (*t.1, t.0.colour))
        .collect::<HashMap<_, _>>();

    for (box_spot, position) in &mut box_spot_query.iter() {
        if let Some(&colour) = occupied_positions_with_colors.get(position) {
            if colour != box_spot.colour {
                gameplay.state = GameplayState::Playing;
                return;
            }
        } else {
            gameplay.state = GameplayState::Playing;
            return;
        }
    }

    gameplay.state = GameplayState::Won;
}
