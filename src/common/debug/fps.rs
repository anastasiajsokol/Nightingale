use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Component)]
struct FpsRoot;

#[derive(Component)]
struct FpsText;

fn setup_fps_counter(mut commands: Commands) {
	let root = commands
		.spawn((
			FpsRoot,
			NodeBundle {
				background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
				z_index: ZIndex::Global(i32::MAX),
				style: Style {
					position_type: PositionType::Absolute,
					right: Val::Percent(1.),
					top: Val::Percent(1.),
					bottom: Val::Auto,
					left: Val::Auto,
					padding: UiRect::all(Val::Px(4.0)),
					..Default::default()
				},
				..Default::default()
			},
		))
		.id();
	let text_fps = commands
		.spawn((
			FpsText,
			TextBundle {
				text: Text::from_sections([
					TextSection {
						value: "FPS: ".into(),
						style: TextStyle {
							font_size: 16.0,
							color: Color::WHITE,
							..default()
						},
					},
					TextSection {
						value: " N/A".into(),
						style: TextStyle {
							font_size: 16.0,
							color: Color::WHITE,
							..default()
						},
					},
				]),
				..Default::default()
			},
		))
		.id();
	commands.entity(root).push_children(&[text_fps]);
}

fn fps_text_update_system(
	diagnostics: Res<DiagnosticsStore>,
	mut query: Query<&mut Text, With<FpsText>>,
) {
	for mut text in &mut query {
		if let Some(value) = diagnostics
			.get(&FrameTimeDiagnosticsPlugin::FPS)
			.and_then(|fps| fps.smoothed())
		{
			text.sections[1].value = format!("{value:>4.0}");

			text.sections[1].style.color = if value >= 120.0 {
				Color::rgb(0.0, 1.0, 0.0)
			} else if value >= 60.0 {
				Color::rgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
			} else if value >= 30.0 {
				Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
			} else {
				Color::rgb(1.0, 0.0, 0.0)
			}
		} else {
			text.sections[1].value = " N/A".into();
			text.sections[1].style.color = Color::WHITE;
		}
	}
}

pub struct FPSPlugin;

impl Plugin for FPSPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(FrameTimeDiagnosticsPlugin::default())
			.add_systems(Startup, setup_fps_counter)
			.add_systems(Update, fps_text_update_system);
	}
}
