use crate::utils::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
pub enum GameStates {
	#[default]
	PlayField,
	
	// #[default]
	Designing,
}

/// What context to execute logic for each frame
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
pub enum ConnectionState {
	/// Playing solo
	LocalOnly,

	/// Playing locally and also hosting current local session as a server
	#[default]
	LocalServer,

	/// Acting as a dedicated server
	/// TODO: Add headless option
	ServerOnly,

	/// Client for server
	Client,
}

impl ConnectionState {
	/// Whether to 
	pub fn is_server(self) -> bool {
		matches!(self, Self::LocalServer | Self::ServerOnly)
	}

	pub fn is_local(self) -> bool {
		matches!(self, Self::LocalOnly | Self::LocalServer)
	}
}