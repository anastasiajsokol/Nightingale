/*
 *  Perform your job delivering supplies from port to port in Nightingale
*/
use bevy::prelude::*;

mod common;
mod splash;

use common::{configuration::ConfigurationPlugin, debug::DebugPlugin};
use splash::SplashPlugin;

fn main() {
	App::new()
		.add_plugins((ConfigurationPlugin, DebugPlugin))
		.add_plugins(SplashPlugin)
		.run();
}
