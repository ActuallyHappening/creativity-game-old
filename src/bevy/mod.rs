use bevy::prelude::*;

use self::{setup::SetupPlugin, player::PlayerPlugin, world::WorldPlugin};

mod player;
mod setup;
pub use setup::MainCamera;
mod utils;
mod world;

pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((SetupPlugin, PlayerPlugin, WorldPlugin));
	}
}
