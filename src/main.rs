use bevy::prelude::*;
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
						..default()
					}),
					..default()
				})
				.build(),
		)
		.add_plugins(MainPlugin)
		.run();
}
