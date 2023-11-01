pub use crate::utils::*;

mod pixels;

pub struct CorePlugin;
impl bevy::prelude::Plugin for CorePlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app
			.add_state::<ServerConnections>()
			.add_state::<ScreenState>()
			// todo: find out why this causes the bug?
			.replicate::<crate::core::pixels::SpawnChildStructure>();
	}
}

use std::net::Ipv4Addr;

/// For UI purposes
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
pub enum ScreenState {
	#[default]
	StartScreen,

	InGame,
}

// #[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
// pub enum GameStates {
// 	#[default]
// 	PlayField,
	
// 	// #[default]
// 	Designing,
// }

// /// What context to execute logic for each frame
// #[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
// pub enum Controlling {
// 	/// Controlling a local player
// 	#[default]
// 	Local,

// 	/// Not controlling any character
// 	Global
// }

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
pub enum ServerConnections {
	/// Hosting to outside world
	// #[default]
	Hosting,

	/// Connecting to a server and displaying server state
	Client,

	/// Not interacting with any servers,
	/// or hosting (yet)
	Local,

	#[default]
	NotPlaying,
}

impl ServerConnections {
	pub fn should_simulate(&self) -> bool {
		matches!(self, ServerConnections::Hosting | ServerConnections::Client)
	}
}