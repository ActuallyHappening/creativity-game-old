use bevy::{log::LogPlugin, prelude::*};
use creativity_game::*;

fn main() {
	let mut app = App::new();

	app
		.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						fit_canvas_to_parent: true,
						prevent_default_event_handling: false,
						canvas: Some("#canvas".to_string()),
						title: "Creativity Game".to_string(),
						..default()
					}),
					..default()
				})
				.set(LogPlugin {
					level: bevy::log::Level::WARN,
					filter: "creativity_game=debug,bevy_ecs=info".into(),
				})
				.build(),
		)
		.add_plugins(MainPlugin)
		.run();
}
