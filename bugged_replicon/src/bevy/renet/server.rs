#![allow(unused_imports)]

use std::{
	collections::HashMap,
	f32::consts::PI,
	net::{Ipv4Addr, SocketAddr},
	sync::{Arc, Mutex},
	time::SystemTime,
};

use super::PROTOCOL_ID;

use crate::{
	utils::*,
};
use bevy::{
	diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
	prelude::*,
};
use bevy_egui::{EguiContexts, EguiPlugin};
// use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
// use bevy_renet::{
// 	renet::{ClientId, RenetServer, ServerEvent},
// 	RenetServerPlugin,
// };
// use renet_visualizer::RenetServerVisualizer;

pub struct ServerPlugin;
impl Plugin for ServerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(ServerConnections::Hosting), (add_server,))
			.add_systems(Update, server_event_system.run_if(has_authority()));
	}
}

fn add_server(
	commands: Commands,
	network_channels: Res<NetworkChannels>,

	mut setup_already: Local<bool>,
) {
	crate::add_server(commands, network_channels);
}

/// Logs server events and spawns a new player whenever a client connects.
fn server_event_system(mut server_event: EventReader<ServerEvent>, mut commands: Commands) {
	for event in &mut server_event {
		match event {
			ServerEvent::ClientConnected { client_id } => {
				info!("player: {client_id} Connected");
			}
			ServerEvent::ClientDisconnected { client_id, reason } => {
				info!("client {client_id} disconnected: {reason}");
			}
		}
	}
}
