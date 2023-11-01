// #![feature(const_trait_impl)]

pub mod utils;
// pub use utils::init_debug_tools;

mod bevy;
pub use bevy::renet::RenetPlugin;
use std::sync::{Arc, Mutex};

pub use bevy::MainPlugin;
mod core;
pub use utils::SpawnChildStructure;
pub use core::WorldObjectType;

use crate::utils::*;

// lazy_static::lazy_static!(
// 	pub static ref ADD_SERVER: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
// );

pub fn add_server(mut commands: Commands, network_channels: Res<NetworkChannels>) {
	// if *ADD_SERVER.lock().unwrap() {
	// 	*ADD_SERVER.lock().unwrap() = false;

		info!(
			"Setting up server resources: server: {:?}; client: {:?}",
			network_channels.get_server_configs(),
			network_channels.get_client_configs()
		);

		use std::net::*;
		use std::time::*;

		let server_channels_config = network_channels.get_server_configs();
		let client_channels_config = network_channels.get_client_configs();

		let server = RenetServer::new(ConnectionConfig {
			server_channels_config,
			client_channels_config,
			..Default::default()
		});

		let current_time = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap();
		let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5069);

		let socket = match UdpSocket::bind(public_addr) {
			Ok(p) => p,
			Err(e) => {
				panic!(
					"Couldn't bind to port {} because of error: {:?}",
					public_addr, e
				);
			}
		};

		let server_config = ServerConfig {
			max_clients: 10,
			protocol_id: 0,
			public_addr,
			authentication: ServerAuthentication::Unsecure,
		};
		let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

		commands.insert_resource(server);
		commands.insert_resource(transport);

		commands.spawn(TextBundle::from_section(
			"Server",
			TextStyle {
				font_size: 30.0,
				color: Color::WHITE,
				..default()
			},
		));

		// commands.spawn((
		// 	(DummyComponent, Replication),
		// 	Name::new("Dummy example main.rs"),
		// ));
		// commands.spawn(PlayerBundle::new(SERVER_ID, Vec2::ZERO, Color::GREEN));
	// }
}
