use bevy::prelude::*;

use self::{setup::SetupPlugin, player::PlayerPlugin};

mod player;
mod setup;
pub use setup::MainCamera;
mod utils;
use utils::*;

pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((SetupPlugin, PlayerPlugin));
	}
}
