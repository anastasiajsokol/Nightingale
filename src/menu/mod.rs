use crate::common::state::GameState;
use bevy::prelude::*;

#[derive(Component)]
struct MenuRoot;

fn construct_menu(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default()).insert(MenuRoot);

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
		.insert(MenuRoot)
		.with_children(|parent| {
			parent
				.spawn(ButtonBundle {
					style: Style {
						width: Val::Px(300.0),
						height: Val::Px(70.0),
						border: UiRect::all(Val::Px(1.0)),
						// horizontally center child text
						justify_content: JustifyContent::Center,
						// vertically center child text
						align_items: AlignItems::Center,
						..default()
					},
					border_color: BorderColor(Color::BLACK),
					background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.3)),
					..default()
				})
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Play Game",
						TextStyle {
							font_size: 40.0,
							color: Color::rgb(0.8, 0.8, 0.8),
							..default()
						},
					));
				});
		});
}

fn button_system(
	query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
	mut state: ResMut<NextState<GameState>>,
) {
	for interaction in &query {
		match *interaction {
			Interaction::Pressed => {
				state.set(GameState::Game);
			}
			_ => {}
		}
	}
}

fn destruct_menu(query: Query<Entity, With<MenuRoot>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn_recursive();
	}
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Menu), construct_menu)
			.add_systems(OnExit(GameState::Menu), destruct_menu)
			.add_systems(Update, button_system.run_if(in_state(GameState::Menu)));
	}
}
