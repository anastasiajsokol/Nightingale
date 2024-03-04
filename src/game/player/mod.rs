use crate::common::state::GameState;
use bevy::prelude::*;

mod flycamera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Game), flycamera::spawn_camera)
			.add_systems(
				Update,
				(
					flycamera::update_camera_position,
					flycamera::update_player_rotation,
				)
					.run_if(in_state(GameState::Game)),
			);
	}
}
