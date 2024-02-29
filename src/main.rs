/*
 *  Perform your job delivering supplies from port to port in Nightingale
*/
use bevy::prelude::*;

mod common;

use common::{configuration::ConfigurationPlugin, debug::DebugPlugin};

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera3dBundle::default());
}

fn main() {
	App::new()
        .add_systems(Startup, spawn_camera)
		.add_plugins(ConfigurationPlugin)
		.add_plugins(DebugPlugin)
        .run();
}
