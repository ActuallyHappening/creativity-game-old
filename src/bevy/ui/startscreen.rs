use crate::utils::*;

pub struct StartScreenPlugin;
impl Plugin for StartScreenPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(ScreenState::StartScreen), (setup_ui)).add_systems(OnExit(ScreenState::StartScreen), (cleanup_ui));
	}
}

#[derive(Component)]
struct StartScreenUi;

fn setup_ui(mut commands: Commands, ass: ResMut<AssetServer>) {
	info!("Start screen");
	commands.spawn(NodeBundle {
		style: Style {
			width: Val::Percent(100.),
			height: Val::Percent(100.),
			justify_content: JustifyContent::Center,
			..default()
		},
		..default()
	}).insert(StartScreenUi).with_children(|parent| {
		parent.spawn(ButtonBundle {
			style: Style {
				width: Val::Px(200.),
				height: Val::Px(60.),
				..default()
			},
			..default()
		}).with_children(|parent| {
			parent.spawn(TextBundle::from_section(
				"Start (Hosted) Game",
				TextStyle {
					font: ass.load(Font::Medium),
					font_size: 40.,
					color: Color::WHITE,
				}
			));
		});
	});
}

fn cleanup_ui(mut commands: Commands, ) {

}