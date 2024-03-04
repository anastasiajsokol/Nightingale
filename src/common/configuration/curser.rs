use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::prelude::*;

fn initial_curser_grab(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
	if let Ok(mut window) = primary_window.get_single_mut() {
		window.cursor.grab_mode = CursorGrabMode::Locked;
		window.cursor.visible = false;
	}
}

fn toggle_curser_grab(
	mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
	keys: Res<ButtonInput<KeyCode>>,
) {
	if keys.just_pressed(KeyCode::Escape) {
		if let Ok(mut window) = primary_window.get_single_mut() {
			if window.cursor.grab_mode == CursorGrabMode::None {
				window.cursor.grab_mode = CursorGrabMode::Locked;
				window.cursor.visible = false;
			} else {
				window.cursor.grab_mode = CursorGrabMode::None;
				window.cursor.visible = true;
			}
		}
	}
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, initial_curser_grab)
			.add_systems(Update, toggle_curser_grab);
	}
}