use bevy::render::camera::Projection::Perspective;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component)]
pub struct FlyCamera;

pub fn spawn_camera(mut commands: Commands) {
	commands
		.spawn(Camera3dBundle {
			transform: Transform::from_xyz(1.0, 4.5, 5.0)
				.looking_at(Vec3::new(0.0, 1.5, 0.0), Vec3::Y),
			projection: Perspective(PerspectiveProjection {
				far: 100000.0,
				..default()
			}),
			..default()
		})
		.insert(FlyCamera);
}

pub fn update_camera_position(
	mut query: Query<&mut Transform, With<FlyCamera>>,
	keys: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
) {
	let speed = if keys.pressed(KeyCode::Space) {
		5000.0 * time.delta_seconds()
	} else {
		50.0 * time.delta_seconds()
	};

	for mut transform in query.iter_mut() {
		let (phi, _, _) = transform.rotation.to_euler(EulerRot::YXZ);

		let forward = Vec3::X * phi.sin() + Vec3::Z * phi.cos();
		let side = Vec3::new(forward.z, 0.0, -forward.x);

		if keys.pressed(KeyCode::KeyE) {
			transform.translation += Vec3::Y * speed;
		}

		if keys.pressed(KeyCode::KeyQ) {
			transform.translation -= Vec3::Y * speed;
		}

		if keys.pressed(KeyCode::KeyW) {
			transform.translation -= forward * speed;
		}

		if keys.pressed(KeyCode::KeyS) {
			transform.translation += forward * speed;
		}

		if keys.pressed(KeyCode::KeyA) {
			transform.translation -= side * speed;
		}

		if keys.pressed(KeyCode::KeyD) {
			transform.translation += side * speed;
		}
	}
}

pub fn update_player_rotation(
	mut query: Query<&mut Transform, With<FlyCamera>>,
	primary_window: Query<&Window, With<PrimaryWindow>>,
	mut motion: EventReader<MouseMotion>,
) {
	if let Ok(window) = primary_window.get_single() {
		// only apply view changes if the cursor is grabbed
		if window.cursor.grab_mode == CursorGrabMode::None {
			return;
		}

		// this should be configurable
		let sensitivity = 0.000001;

		// there really should only be one event at max, but just in case
		for event in motion.read() {
			// update every camera - this should be changed in the future to only update player views
			for mut transform in query.iter_mut() {
				// get current rotation
				let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

				// scale by the size of the window (ie how important each pixel is)
				let window_scale = window.height().min(window.width());

				yaw -= event.delta.x * window_scale * sensitivity;
				pitch -= event.delta.y * window_scale * sensitivity;

				// limit how far up or down the player can look
				pitch = pitch.clamp(-0.45 * std::f32::consts::PI, 0.45 * std::f32::consts::PI);

				// apply new angle
				transform.rotation =
					Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
			}
		}
	}
}
