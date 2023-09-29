use bevy::prelude::*;

use crate::core::CorePlugin;

use self::{camera::CameraPlugin, player::PlayerPlugin, setup::SetupPlugin, ui::UiPlugin};

mod camera;
mod player;
mod setup;
mod ui;
mod world_gen;

pub use player::types;

pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			CorePlugin,
			SetupPlugin,
			PlayerPlugin,
			CameraPlugin,
			UiPlugin,
		));
	}
}
