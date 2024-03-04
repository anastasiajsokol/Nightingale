use crate::common::state::GameState;
use bevy::prelude::*;

mod player;

use player::PlayerPlugin;

pub struct GamePlugin;

fn create_game(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
	commands.spawn(PbrBundle {
		mesh: meshes.add(Cuboid::default()),
		..default()
	});
}

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Game), create_game)
			.add_plugins(PlayerPlugin);
	}
}
