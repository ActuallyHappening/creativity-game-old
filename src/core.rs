mod events;
mod pixels;
mod player;
mod states;

pub use events::*;
pub use pixels::*;
pub use player::*;

pub struct CorePlugin;
impl bevy::prelude::Plugin for CorePlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app.add_event::<PlayerMinedPixel>();
	}
}
