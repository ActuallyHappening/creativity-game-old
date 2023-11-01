use bevy::prelude::*;

use crate::core::CorePlugin;

use self::{camera::CameraPlugin, setup::SetupPlugin, ui::UiPlugins};

mod camera;
pub mod renet;
mod setup;
mod ui;

pub use renet::ClientID;

pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			self::renet::RenetPlugin,
			CorePlugin,
			SetupPlugin,
			CameraPlugin,
			UiPlugins.build(),
		));
	}
}
