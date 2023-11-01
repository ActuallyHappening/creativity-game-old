#![allow(unused_imports)]

use std::{
	collections::HashMap,
	f32::consts::PI,
	net::{Ipv4Addr, SocketAddr},
	sync::{Arc, Mutex},
	time::SystemTime,
};

use crate::{
	bevy::player::{
		authoritative_spawn_initial_player, AuthorityPlayerBundle, ControllablePlayer, PLAYER_STRUCTURE,
	},
	utils::*,
};
use bevy::{
	diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
	prelude::*,
};
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
			.add_systems(OnEnter(ServerConnections::Hosting), (add_server, spawn_initial_world, authoritative_spawn_initial_player))
			.add_systems(Update, server_event_system.run_if(has_authority()))
			// .add_systems(
			// 	Update,
			// 	(
			// 		server_update_system,
			// 		#[cfg(feature = "debugging")]
			// 		update_server_visualizer.run_if(in_state(ServerConnections::Hosting)),
			// 	),
			// );
			;
	}
}

// fn add_server_flag() {
// *crate::ADD_SERVER.lock().unwrap() = true;
// info!("Setting up server flag from faulty place");
// }

fn add_server(
	commands: Commands,
	network_channels: Res<NetworkChannels>,
	config: Res<SavedHostingInfo>,

	mut setup_already: Local<bool>,
) {
	crate::add_server(commands, network_channels);

	// let server_channels_config = network_channels.server_channels();
	// let client_channels_config = network_channels.client_channels();

	// let server = RenetServer::new(renet::ConnectionConfig {
	// 	server_channels_config,
	// 	client_channels_config,
	// 	..Default::default()
	// });

	// let current_time = SystemTime::now()
	// 	.duration_since(SystemTime::UNIX_EPOCH)
	// 	.unwrap();
	// // let public_addr = SocketAddr::new(config.host_ip, config.host_port);
	// let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5069);

	// let socket = UdpSocket::bind(public_addr).expect("Couldn't bind to socket");
	// let server_config = renet::transport::ServerConfig {
	// 	max_clients: 10,
	// 	protocol_id: PROTOCOL_ID,
	// 	public_addr,
	// 	authentication: transport::ServerAuthentication::Unsecure,
	// };
	// let transport = transport::NetcodeServerTransport::new(current_time, server_config, socket)
	// 	.expect("Failed to start server");

	// commands.insert_resource(server);
	// commands.insert_resource(transport);

	// info!("Acting as a server");

	// commands.spawn((DummyComponent, Replication, Name::new("TEST")));

	// // commands.spawn(PlayerBundle::new(SERVER_ID, Vec2::ZERO, Color::GREEN));
}

fn disconnect_server() {}

/// Logs server events and spawns a new player whenever a client connects.
fn server_event_system(mut server_event: EventReader<ServerEvent>, mut commands: Commands) {
	for event in &mut server_event {
		match event {
			ServerEvent::ClientConnected { client_id } => {
				info!("player: {client_id} Connected");
				// Generate pseudo random color from client id.
				// let r = ((client_id % 23) as f32) / 23.0;
				// let g = ((client_id % 27) as f32) / 27.0;
				// let b = ((client_id % 39) as f32) / 39.0;
				// commands.spawn(PlayerBundle::new(
				// 	*client_id,
				// 	Vec2::ZERO,
				// 	Color::rgb(r, g, b),
				// ));
				// spawn_player.send(SpawnPlayer {
				// 	pos: Transform::from_xyz(0., 0., 0.),
				// 	id: *client_id,
				// });
				commands.spawn(AuthorityPlayerBundle::new(
					ControllablePlayer {
						network_id: *client_id,
					},
					PLAYER_STRUCTURE.clone(),
					Transform::from_xyz(0., 100., 0.),
				));
			}
			ServerEvent::ClientDisconnected { client_id, reason } => {
				info!("client {client_id} disconnected: {reason}");
			}
		}
	}
}
