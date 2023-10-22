use crate::utils::*;

// #[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
// pub enum GameStates {
// 	#[default]
// 	PlayField,
	
// 	// #[default]
// 	Designing,
// }

/// What context to execute logic for each frame
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
pub enum Controlling {
	/// Controlling a local player
	#[default]
	Local,

	/// Not controlling any character
	Global
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
pub enum ServerConnections {
	/// Hosting to outside world
	#[default]
	Hosting,

	/// Connecting to a server and displaying server state
	Client,

	/// Not interacting with any servers,
	/// or hosting (yet)
	Local,
}

impl ServerConnections {
	pub fn should_simulate(&self) -> bool {
		matches!(self, ServerConnections::Hosting | ServerConnections::Client)
	}
}