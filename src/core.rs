pub use crate::utils::*;

mod events;
mod pixels;
mod player;
mod states;

pub use events::*;
pub use pixels::*;
pub use player::*;
pub use states::*;

pub struct CorePlugin;
impl bevy::prelude::Plugin for CorePlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app
			.add_state::<ServerConnections>()
			.add_state::<ScreenState>()
			.init_resource::<SavedHostingInfo>()
			// todo: find out why this causes the bug?
			.replicate::<SpawnChildStructure>();
	}
}
