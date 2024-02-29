use bevy::render::camera::ScalingMode;
use crate::common::state::GameState;
use bevy::prelude::*;

pub struct SplashPlugin;

fn create_splash(mut commands: Commands) {
	commands.spawn(Camera3dBundle {
		projection: OrthographicProjection {
			scaling_mode: ScalingMode::FixedVertical(6.0),
			..default()
		}
		.into(),
		..default()
	});
}

impl Plugin for SplashPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Splash), create_splash);
	}
}
