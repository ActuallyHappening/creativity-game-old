use bevy::{log::LogPlugin, prelude::*, window::WindowMode};
use bevy_replicon::prelude::*;
use bevy_replicon::ReplicationPlugins;
use creativity_game::*;
use serde::Serialize;
use std::net::*;
use std::time::*;

use creativity_game::utils::*;
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
					filter: "creativity_game=trace,bevy_ecs=info,bevy_replicon=debug".into(),
				})
				.build(),
		)
		// .add_plugins(MainPlugin)
		// .add_plugins((TestPlugin, bevy_editor_pls::EditorPlugin::default()))
		.add_state::<ServerConnections>()
		.add_state::<ScreenState>()
		.init_resource::<SavedHostingInfo>()
		.add_plugins((
			bevy_editor_pls::EditorPlugin::default(),
			broken_ui::RenetPlugin,
			// ReplicationPlugins,
			test_plugin::TestPlugin,
			broken_ui::StartScreenPlugin,
		))
		.run();
}

mod broken_ui {
	use creativity_game::utils::*;

	pub use renet::RenetPlugin;
	mod renet {
		use creativity_game::utils::*;
		pub struct RenetPlugin;
		impl Plugin for RenetPlugin {
			fn build(&self, app: &mut App) {
				app
					.add_plugins(ReplicationPlugins)
					.add_plugins((client::ClientPlugin, server::ServerPlugin));
			}
		}

		mod server {

			use creativity_game::utils::*;

			#[derive(Component, Serialize, Deserialize)]
			struct DummyComponent;

			pub struct ServerPlugin;
			impl Plugin for ServerPlugin {
				fn build(&self, app: &mut App) {
					app
			.add_systems(OnEnter(ServerConnections::Hosting), (add_server_flag, spawn_initial_world))
			.add_systems(Update, (server_event_system.run_if(has_authority()), crate::add_server))
			.replicate::<DummyComponent>()
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

			fn add_server_flag() {
				*crate::ADD_SERVER.lock().unwrap() = true;
				info!("Setting up server flag from faulty place");
			}

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
							// commands.spawn(AuthorityPlayerBundle::new(
							// 	ControllablePlayer {
							// 		network_id: *client_id,
							// 	},
							// 	PLAYER_STRUCTURE.clone(),
							// 	Transform::from_xyz(0., 100., 0.),
							// ));
						}
						ServerEvent::ClientDisconnected { client_id, reason } => {
							info!("client {client_id} disconnected: {reason}");
						}
					}
				}
			}
		}

		mod client {
			use creativity_game::utils::*;
			pub struct ClientPlugin;
			impl Plugin for ClientPlugin {
				fn build(&self, app: &mut App) {
					app.add_systems(OnEnter(ServerConnections::Client), add_client);
					// app
					// 	.add_plugins((
					// 		bevy_renet::RenetClientPlugin,
					// 		bevy_renet::transport::NetcodeClientPlugin,
					// 	))
					// 	.configure_set(
					// 		Update,
					// 		Connected.run_if(bevy_renet::transport::client_connected()),
					// 	)
					// 	.add_systems(
					// 		Update,
					// 		(panic_on_error_system, update_visulizer_system)
					// 			.run_if(in_state(ServerConnections::Client)),
					// 	)
					// 	.add_systems(OnEnter(ServerConnections::Client), add_netcode_client_network)
					// 	.add_systems(OnExit(ServerConnections::Client), remove_netcode_network);
				}
			}

			fn add_client(
				mut commands: Commands,
				network_channels: Res<NetworkChannels>,
				config: Res<SavedHostingInfo>,

				mut setup_already: Local<bool>,
			) {
				if *setup_already {
					warn!("Client already setup");
					return;
				} else {
					*setup_already = true;
				}

				let server_channels_config = network_channels.get_client_configs();
				let client_channels_config = network_channels.get_server_configs();

				let client = RenetClient::new(ConnectionConfig {
					server_channels_config,
					client_channels_config,
					..Default::default()
				});

				let current_time = SystemTime::now()
					.duration_since(SystemTime::UNIX_EPOCH)
					.unwrap();
				let client_id = current_time.as_millis() as u64;
				let server_addr = SocketAddr::new(config.join_ip, config.join_port);
				let socket = UdpSocket::bind((config.join_ip, 0)).expect("Couldn't bind to socket");
				let authentication = transport::ClientAuthentication::Unsecure {
					client_id,
					protocol_id: 0,
					server_addr,
					user_data: None,
				};
				let transport =
					transport::NetcodeClientTransport::new(current_time, authentication, socket)
						.expect("Couldn't join to server");

				commands.insert_resource(client);
				commands.insert_resource(transport);

				// commands.spawn(TextBundle::from_section(
				// 	format!("Client: {client_id:?}"),
				// 	TextStyle {
				// 		font_size: 30.0,
				// 		color: Color::WHITE,
				// 		..default()
				// 	},
				// ));

				info!("Acting as client");
			}
		}
	}

	pub use start_screen_plugin::StartScreenPlugin;
	mod start_screen_plugin {
		use crate::utils::*;
		pub struct StartScreenPlugin;
		impl Plugin for StartScreenPlugin {
			fn build(&self, app: &mut App) {
				app
					.add_state::<StartScreens>()
					.add_systems(OnExit(ScreenState::StartScreen), cleanup_ui)
					.add_systems(OnEnter(StartScreens::Default), setup_default_ui)
					.add_systems(OnExit(StartScreens::Default), cleanup_ui)
					.add_systems(
						Update,
						(
							handle_default_ui.run_if(in_state(StartScreens::Default)),
							handle_host_controls_ui.run_if(in_state(StartScreens::HostControls)),
							handle_client_controls_ui.run_if(in_state(StartScreens::ClientControls)),
						)
							.run_if(in_state(ScreenState::StartScreen)),
					)
					.add_systems(OnEnter(StartScreens::HostControls), setup_host_controls_ui)
					.add_systems(OnExit(StartScreens::HostControls), cleanup_ui)
					.add_systems(
						OnEnter(StartScreens::ClientControls),
						setup_client_controls_ui,
					)
					.add_systems(OnExit(StartScreens::ClientControls), cleanup_ui);
			}
		}

		#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
		pub enum StartScreens {
			#[default]
			Default,

			HostControls,
			ClientControls,
		}

		#[derive(Component)]
		struct StartScreenUi;

		#[derive(Component)]
		struct DefaultBtn(ServerConnections);

		fn setup_default_ui(mut commands: Commands, ass: ResMut<AssetServer>) {
			info!("Start screen");
			commands
				.spawn(NodeBundle {
					style: Style {
						width: Val::Percent(100.),
						height: Val::Percent(100.),
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						flex_direction: FlexDirection::Column,
						..default()
					},
					..default()
				})
				.insert(StartScreenUi)
				.with_children(|parent| {
					let btn = |name: &'static str, btn_type: ServerConnections, parent: &mut ChildBuilder| {
						parent
							.spawn(ButtonBundle {
								style: Style {
									width: Val::Px(400.),
									height: Val::Px(50.),
									justify_content: JustifyContent::Center,
									align_items: AlignItems::Center,
									..default()
								},
								background_color: Color::BLACK.into(),
								..default()
							})
							.insert(DefaultBtn(btn_type))
							.with_children(|parent| {
								parent.spawn(TextBundle::from_section(
									name,
									TextStyle {
										font: ass.load(Font::Medium),
										font_size: 30.,
										color: Color::WHITE,
									},
								));
							});
					};
					btn("Start (Hosted) Game", ServerConnections::Hosting, parent);
					btn("Start Private Game", ServerConnections::Local, parent);
					btn("Join hosted Game", ServerConnections::Client, parent);
				});
		}

		fn handle_default_ui(
			btns: Query<(&DefaultBtn, &Interaction), Changed<Interaction>>,
			mut next_screen: ResMut<NextState<StartScreens>>,
			mut start_game: ResMut<NextState<ServerConnections>>,
		) {
			for (btn, interaction) in btns.iter() {
				if interaction == &Interaction::Pressed {
					match btn.0 {
						ServerConnections::Local => start_game.0 = Some(ServerConnections::Local),
						ServerConnections::Client => next_screen.0 = Some(StartScreens::ClientControls),
						ServerConnections::Hosting => next_screen.0 = Some(StartScreens::HostControls),
						_ => {}
					}
				}
			}
		}

		#[derive(Component)]
		struct HostControlsBtn;

		fn setup_host_controls_ui(mut commands: Commands, ass: ResMut<AssetServer>) {
			info!("- Host Controls UI");
			commands
				.spawn(NodeBundle {
					style: Style {
						width: Val::Percent(100.),
						height: Val::Percent(100.),
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						flex_direction: FlexDirection::Column,
						..default()
					},
					..default()
				})
				.insert(StartScreenUi)
				.with_children(|parent| {
					parent
						.spawn(ButtonBundle {
							style: Style {
								width: Val::Px(400.),
								height: Val::Px(50.),
								justify_content: JustifyContent::Center,
								align_items: AlignItems::Center,
								..default()
							},
							background_color: Color::BLACK.into(),
							..default()
						})
						.insert(HostControlsBtn)
						.with_children(|parent| {
							parent.spawn(TextBundle::from_section(
								"Host Public Game",
								TextStyle {
									font: ass.load(Font::Medium),
									font_size: 30.,
									color: Color::WHITE,
								},
							));
						});
				});
		}

		fn handle_host_controls_ui(
			btn: Query<&Interaction, (With<HostControlsBtn>, Changed<Interaction>)>,
			mut start_game: ResMut<NextState<ServerConnections>>,
			mut in_game: ResMut<NextState<ScreenState>>,
		) {
			if let Some(Interaction::Pressed) = btn.iter().next() {
				start_game.0 = Some(ServerConnections::Hosting);
				in_game.0 = Some(ScreenState::InGame);
			}
		}

		#[derive(Component)]
		struct ClientControlsBtn;

		fn setup_client_controls_ui(mut commands: Commands, ass: ResMut<AssetServer>) {
			info!("- Client Controls UI");
			commands
				.spawn(NodeBundle {
					style: Style {
						width: Val::Percent(100.),
						height: Val::Percent(100.),
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						flex_direction: FlexDirection::Column,
						..default()
					},
					..default()
				})
				.insert(StartScreenUi)
				.with_children(|parent| {
					parent
						.spawn(ButtonBundle {
							style: Style {
								width: Val::Px(400.),
								height: Val::Px(50.),
								justify_content: JustifyContent::Center,
								align_items: AlignItems::Center,
								..default()
							},
							background_color: Color::BLACK.into(),
							..default()
						})
						.insert(ClientControlsBtn)
						.with_children(|parent| {
							parent.spawn(TextBundle::from_section(
								"Join Machine-Local Game",
								TextStyle {
									font: ass.load(Font::Medium),
									font_size: 30.,
									color: Color::WHITE,
								},
							));
						});
				});
		}

		fn handle_client_controls_ui(
			btn: Query<&Interaction, (With<ClientControlsBtn>, Changed<Interaction>)>,
			mut start_game: ResMut<NextState<ServerConnections>>,
			mut in_game: ResMut<NextState<ScreenState>>,
		) {
			if let Some(Interaction::Pressed) = btn.iter().next() {
				start_game.0 = Some(ServerConnections::Client);
				in_game.0 = Some(ScreenState::InGame);
			}
		}

		fn cleanup_ui(mut commands: Commands, ui: Query<Entity, With<StartScreenUi>>) {
			commands.entity(ui.single()).despawn_recursive();
		}
	}
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
		commands.spawn(Camera3dBundle::default());
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
								font: ass.load(creativity_game::utils::Font::Medium),
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
				// 					font: ass.load(creativity_game::utils::Font::Medium),
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
									font: ass.load(creativity_game::utils::Font::Medium),
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
					*creativity_game::ADD_SERVER.lock().unwrap() = true;
					// creativity_game::add_server(&mut commands, network_channels);
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
