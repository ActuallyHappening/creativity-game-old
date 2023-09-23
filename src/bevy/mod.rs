use bevy::prelude::*;

use self::setup::SetupPlugin;

mod player;
mod setup;

pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(SetupPlugin);
	}
}
