use crate::utils::*;

pub struct StartScreenPlugin;
impl Plugin for StartScreenPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(ScreenState::StartScreen), (setup_ui))
			.add_systems(OnExit(ScreenState::StartScreen), (cleanup_ui));
	}
}

#[derive(Component)]
struct StartScreenUi;

fn setup_ui(mut commands: Commands, ass: ResMut<AssetServer>) {
	info!("Start screen");
	commands
		.spawn(NodeBundle {
			style: Style {
				width: Val::Percent(100.),
				height: Val::Percent(100.),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				flex_direction: FlexDirection::Row,
				..default()
			},
			..default()
		})
		.insert(StartScreenUi)
		.with_children(|parent| {
			let btn = |name: &'static str, parent: &mut ChildBuilder| {
				parent
					.spawn(ButtonBundle {
						style: Style {
							width: Val::Px(400.),
							height: Val::Px(50.),
							justify_content: JustifyContent::Center,
							align_items: AlignItems::Center,
							..default()
						},
						background_color: Color::BLACK.into(),
						..default()
					})
					.with_children(|parent| {
						parent.spawn(TextBundle::from_section(
							name,
							TextStyle {
								font: ass.load(Font::Medium),
								font_size: 30.,
								color: Color::WHITE,
							},
						));
					});
			};
			btn("Start (Hosted) Game", parent);
			btn("Start Private Game", parent);
			btn("Join hosted Game", parent);
		});
}

fn cleanup_ui(mut commands: Commands, ui: Query<Entity, With<StartScreenUi>>) {
	commands.entity(ui.single()).despawn_recursive();
}
