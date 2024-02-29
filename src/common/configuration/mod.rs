use bevy::window::{WindowMode, PresentMode};
use super::state::GameState;
use bevy::prelude::*;

pub struct ConfigurationPlugin;

impl Plugin for ConfigurationPlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app.insert_resource(ClearColor(Color::hsl(267.0, 1.0, 0.05)))
			.add_plugins(DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					mode: WindowMode::BorderlessFullscreen,
					present_mode: PresentMode::AutoVsync,
					title: "Nightingale".into(),
					..default()
				}),
				..default()
			}))
			.init_state::<GameState>();
	}
}
