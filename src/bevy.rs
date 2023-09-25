use bevy::prelude::*;

use self::{camera::CameraPlugin, player::PlayerPlugin, setup::SetupPlugin, ui::UiPlugin};

mod camera;
mod player;
mod setup;
mod ui;
mod world_gen;

pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((SetupPlugin, PlayerPlugin, CameraPlugin, UiPlugin));
	}
}
