use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::prelude::*;

mod fps;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((WorldInspectorPlugin::new(), fps::FPSPlugin));
	}
}
