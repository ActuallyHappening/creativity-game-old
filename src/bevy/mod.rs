use bevy::prelude::*;

use self::{setup::SetupPlugin, player::PlayerPlugin, world::WorldPlugin, camera::CameraPlugin};

mod player;
mod setup;
mod utils;
mod world;
mod camera;

pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((SetupPlugin, PlayerPlugin, WorldPlugin, CameraPlugin));
	}
}
