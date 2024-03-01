use crate::common::state::GameState;
use std::time::Duration;
use bevy::prelude::*;

pub struct SplashPlugin;

#[derive(Component)]
struct SplashRoot;

#[derive(Resource)]
struct SplashTimer {
	timer: Timer,
}

fn construct_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(Camera2dBundle::default()).insert(SplashRoot);

	commands
		.spawn(NodeBundle {
			style: Style {
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				..default()
			},
			..default()
		})
		.insert(SplashRoot)
		.with_children(|parent| {
			parent.spawn(TextBundle::from_section(
				"Nightingale",
				TextStyle {
					font_size: 100.0,
					color: Color::BLACK,
					..default()
				},
			));
		});

	let position = Transform::from_translation(Vec3::new(0.0, 17.0, 0.0))
		.with_rotation(Quat::from_rotation_z(-0.5))
		.with_scale(Vec3::new(1.2, 1.2, 1.2));

	commands
		.spawn(SpriteBundle {
			texture: asset_server.load("logo.png"),
			transform: position,
			..default()
		})
		.insert(SplashRoot);
}

fn update_time(
	mut state: ResMut<NextState<GameState>>,
	mut splash: ResMut<SplashTimer>,
	time: Res<Time>,
) {
	splash.timer.tick(time.delta());

	if splash.timer.finished() {
		state.set(GameState::Menu);
	}
}

fn destruct_splash(query: Query<Entity, With<SplashRoot>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn_recursive();
	}

	commands.remove_resource::<SplashTimer>();
}

impl Plugin for SplashPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Splash), construct_splash)
			.add_systems(OnExit(GameState::Splash), destruct_splash)
			.add_systems(Update, update_time.run_if(in_state(GameState::Splash)))
			.insert_resource(SplashTimer {
				timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
			});
	}
}
