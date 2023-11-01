use ::bevy::{log::LogPlugin, prelude::*, window::WindowMode};
use bevy_replicon::prelude::*;
use bevy_replicon::ReplicationPlugins;
use creativity_game_bugged::*;
use serde::Serialize;
use std::net::*;
use std::time::*;

use creativity_game_bugged::utils::*;
fn main() {
	let mut app = App::new();

	app
		.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						fit_canvas_to_parent: true,
						prevent_default_event_handling: false,
						canvas: Some("#canvas".to_string()),
						title: "Creativity Game".to_string(),
						mode: WindowMode::BorderlessFullscreen,
						..default()
					}),
					..default()
				})
				.set(LogPlugin {
					level: bevy::log::Level::WARN,
					filter: "creativity_game_bugged=trace,bevy_ecs=info,bevy_replicon=debug".into(),
				})
				.build(),
		)
		// .add_plugins(MainPlugin)
		// .add_plugins((TestPlugin, bevy_editor_pls::EditorPlugin::default()))
		// .add_state::<ServerConnections>()
		// .add_state::<ScreenState>()
		// .init_resource::<SavedHostingInfo>()
		.add_plugins((
			// bevy_editor_pls::EditorPlugin::default(),
			// broken_ui::RenetPlugin,
			creativity_game_bugged::MainPlugin,
			// ReplicationPlugins,
			test_plugin::TestPlugin,
			// broken_ui::StartScreenPlugin,
		))
		.run();
}

mod test_plugin {
	// #region TestPlugin
	use super::*;
	pub struct TestPlugin;
	impl Plugin for TestPlugin {
		fn build(&self, app: &mut App) {
			app
				// .add_plugins(ReplicationPlugins)
				.replicate::<DummyComponent>()
				.add_systems(Startup, (spawn_ui, setup))
				.add_systems(
					Update,
					(
						cli_system.pipe(system_adapter::unwrap),
						test_for_replication
							.run_if(bevy::time::common_conditions::on_timer(
								Duration::from_millis(500),
							))
							.run_if(resource_exists::<RenetClient>()),
						add_btn,
					),
				);
		}
	}

	#[derive(Component)]
	struct HostBtn;

	// #[derive(Component)]
	// struct JoinBtn;

	#[derive(Component)]
	struct AddBtn;

	fn setup(mut commands: Commands) {
		// commands.spawn(Camera3dBundle::default());
	}

	fn spawn_ui(mut commands: Commands, ass: ResMut<AssetServer>) {
		// commands.spawn(TextBundle::from_section(
		// 	"Hello World",
		// 	TextStyle {
		// 		font_size: 30.0,
		// 		color: Color::WHITE,
		// 		..default()
		// 	},
		// ));

		commands
			.spawn(NodeBundle {
				style: Style {
					width: Val::Percent(100.),
					height: Val::Percent(100.),
					align_content: AlignContent::Center,
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					flex_direction: FlexDirection::Column,
					..default()
				},
				background_color: Color::GREEN.into(),
				..default()
			})
			.with_children(|parent| {
				parent
					.spawn(
						ButtonBundle {
							style: Style {
								width: Val::Px(100.0),
								height: Val::Px(100.0),
								..default()
							},
							background_color: Color::BLACK.into(),
							..default()
						}
						.insert(HostBtn),
					)
					.with_children(|parent| {
						parent.spawn(TextBundle::from_section(
							"Host",
							TextStyle {
								font: ass.load(creativity_game_bugged::utils::Font::Medium),
								font_size: 20.,
								color: Color::WHITE,
							},
						));
					});

				// parent
				// 	.spawn(
				// 		ButtonBundle {
				// 			style: Style {
				// 				width: Val::Px(100.0),
				// 				height: Val::Px(100.0),
				// 				..default()
				// 			},
				// 			background_color: Color::BLACK.into(),
				// 			..default()
				// 		}
				// 		.named("Join Btn")
				// 		.insert(JoinBtn),
				// 	)
				// 	.with_children(|parent| {
				// 		parent.spawn(
				// 			TextBundle::from_section(
				// 				"Join",
				// 				TextStyle {
				// 					font: ass.load(creativity_game_bugged::utils::Font::Medium),
				// 					font_size: 20.,
				// 					color: Color::WHITE,
				// 				},
				// 			)
				// 			.named("Join"),
				// 		);
				// 	});

				parent
					.spawn(
						ButtonBundle {
							style: Style {
								width: Val::Px(100.0),
								height: Val::Px(100.0),
								..default()
							},
							background_color: Color::BLACK.into(),
							..default()
						}
						.named("Add Btn")
						.insert(AddBtn),
					)
					.with_children(|parent| {
						parent.spawn(
							TextBundle::from_section(
								"Add",
								TextStyle {
									font: ass.load(creativity_game_bugged::utils::Font::Medium),
									font_size: 20.,
									color: Color::WHITE,
								},
							)
							.named("Add"),
						);
					});
			});
	}

	fn test_for_replication(
		replicated: Query<Entity, With<DummyComponent>>,
		is_client: Option<Res<RenetServer>>,
	) {
		if is_client.is_none() {
			info!("Number of dummy components: {}", replicated.iter().len());
		}
	}

	fn add_btn(mut commands: Commands, add_btn: Query<&Interaction, With<AddBtn>>) {
		if let Interaction::Pressed = add_btn.single() {
			commands.spawn((DummyComponent, Replication));
		}
	}

	fn cli_system(
		mut commands: Commands,
		host_btn: Query<&Interaction, With<HostBtn>>,
		// join_btn: Query<&Interaction, With<JoinBtn>>,
		// cli: Res<Cli>,
		network_channels: Res<NetworkChannels>,
		mut setup_already: Local<bool>,
	) -> Result<(), Box<dyn std::error::Error>> {
		let mut cli = None;
		if let Interaction::Pressed = host_btn.single() {
			cli = Some(Cli::Server { port: PORT })
		}
		// if let Ok(Interaction::Pressed) = join_btn.get_single() {
		// 	cli = Some(Cli::Client {
		// 		ip: Ipv4Addr::LOCALHOST.into(),
		// 		port: PORT,
		// 	})
		// }

		if let Some(cli) = cli {
			if *setup_already {
				return Ok(());
			} else {
				*setup_already = true;
			}
			info!("Setting up cli: {:?}", cli);
			match cli {
				Cli::SinglePlayer => {
					// commands.spawn(PlayerBundle::new(SERVER_ID, Vec2::ZERO, Color::GREEN));
				}
				Cli::Server { .. } => {
					// *creativity_game_bugged::ADD_SERVER.lock().unwrap() = true;
					creativity_game_bugged::add_server(commands, network_channels);
					// creativity_game_bugged::add_server(&mut commands, network_channels);
					// let server_channels_config = network_channels.server_channels();
					// let client_channels_config = network_channels.client_channels();

					// let server = RenetServer::new(ConnectionConfig {
					// 	server_channels_config,
					// 	client_channels_config,
					// 	..Default::default()
					// });

					// let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
					// let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5069);

					// let socket = match UdpSocket::bind(public_addr) {
					// 	Ok(p) => p,
					// 	Err(e) => {
					// 		panic!(
					// 			"Couldn't bind to port {} because of error: {:?}",
					// 			public_addr, e
					// 		);
					// 	}
					// };

					// let server_config = ServerConfig {
					// 	max_clients: 10,
					// 	protocol_id: PROTOCOL_ID,
					// 	public_addr,
					// 	authentication: ServerAuthentication::Unsecure,
					// };
					// let transport = NetcodeServerTransport::new(current_time, server_config, socket)?;

					// commands.insert_resource(server);
					// commands.insert_resource(transport);

					// commands.spawn(TextBundle::from_section(
					// 	"Server",
					// 	TextStyle {
					// 		font_size: 30.0,
					// 		color: Color::WHITE,
					// 		..default()
					// 	},
					// ));

					// commands.spawn(((DummyComponent, Replication), Name::new("Dummy example main.rs")));
					// // commands.spawn(PlayerBundle::new(SERVER_ID, Vec2::ZERO, Color::GREEN));
				} // Cli::Client { port, ip } => {
				  // 	let server_channels_config = network_channels.get_client_configs();
				  // 	let client_channels_config = network_channels.get_server_configs();

				  // 	let client = RenetClient::new(ConnectionConfig {
				  // 		server_channels_config,
				  // 		client_channels_config,
				  // 		..Default::default()
				  // 	});

				  // 	let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
				  // 	let client_id = current_time.as_millis() as u64;
				  // 	let server_addr = SocketAddr::new(ip, port);
				  // 	let socket =
				  // 		UdpSocket::bind((ip, 0)).expect(&format!("Couldn't bind to UdpSocked {:?}", (ip, 0)));
				  // 	let authentication = ClientAuthentication::Unsecure {
				  // 		client_id,
				  // 		protocol_id: PROTOCOL_ID,
				  // 		server_addr,
				  // 		user_data: None,
				  // 	};
				  // 	let transport = NetcodeClientTransport::new(current_time, authentication, socket)
				  // 		.expect("Couldn't create netcode client transform");

				  // 	commands.insert_resource(client);
				  // 	commands.insert_resource(transport);

				  // 	commands.spawn(TextBundle::from_section(
				  // 		format!("Client: {client_id:?}"),
				  // 		TextStyle {
				  // 			font_size: 30.0,
				  // 			color: Color::WHITE,
				  // 			..default()
				  // 		},
				  // 	));
				  // }
			}
		}

		Ok(())
	}

	#[derive(Serialize, serde::Deserialize, Component)]
	struct DummyComponent;

	// //! A simple demo to showcase how player could send inputs to move the square and server replicates position back.
	// //! Also demonstrates the single-player and how sever also could be a player.

	use std::{
		error::Error,
		net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
		time::SystemTime,
	};

	use bevy::prelude::*;
	use clap::Parser;
	// use serde::{Deserialize, Serialize};

	use bevy_replicon::{
		prelude::*,
		renet::{
			transport::{
				ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication,
				ServerConfig,
			},
			ConnectionConfig, ServerEvent,
		},
	};

	const PORT: u16 = 5069;
	const PROTOCOL_ID: u64 = 0;

	#[derive(Parser, PartialEq, Resource, Debug)]
	enum Cli {
		SinglePlayer,
		Server {
			#[arg(short, long, default_value_t = PORT)]
			port: u16,
		},
		// Client {
		// 	#[arg(short, long, default_value_t = Ipv4Addr::LOCALHOST.into())]
		// 	ip: IpAddr,

		// 	#[arg(short, long, default_value_t = PORT)]
		// 	port: u16,
		// },
	}

	// #endregion
}
