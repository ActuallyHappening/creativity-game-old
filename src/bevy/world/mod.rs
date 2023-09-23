use bevy::prelude::*;

use self::resources::WorldResourcesPlugin;

mod resources;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(WorldResourcesPlugin);
	}
}
