use bevy::window::{WindowMode, PresentMode};
use bevy::prelude::*;

pub struct ConfigurationPlugin;

impl Plugin for ConfigurationPlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
			.add_plugins(DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					mode: WindowMode::BorderlessFullscreen,
					present_mode: PresentMode::AutoVsync,
					title: "Nightingale".into(),
					..default()
				}),
				..default()
			}));
	}
}
