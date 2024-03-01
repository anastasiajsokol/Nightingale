use crate::common::state::GameState;
use bevy::prelude::*;

#[derive(Component)]
struct MenuRoot;

fn construct_menu(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default()).insert(MenuRoot);
}

fn destruct_menu(query: Query<Entity, With<MenuRoot>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn_recursive();
	}
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Menu), construct_menu)
			.add_systems(OnExit(GameState::Menu), destruct_menu);
	}
}
