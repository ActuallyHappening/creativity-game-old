use bevy::prelude::*;

use self::{camera::CameraPlugin, player::PlayerPlugin, setup::SetupPlugin, ui::UiPlugin};

mod camera;
mod player;
mod setup;
mod ui;

pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((SetupPlugin, PlayerPlugin, CameraPlugin, UiPlugin));
	}
}
