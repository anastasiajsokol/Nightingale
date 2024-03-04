/*
 *  Perform your job delivering supplies from port to port in Nightingale
*/
use bevy::prelude::*;

mod common;
mod game;
mod menu;
mod splash;

use common::{configuration::ConfigurationPlugin, debug::DebugPlugin};
use splash::SplashPlugin;
use game::GamePlugin;
use menu::MenuPlugin;

fn main() {
	App::new()
		.add_plugins((ConfigurationPlugin, DebugPlugin))
		.add_plugins((SplashPlugin, MenuPlugin, GamePlugin))
		.run();
}
