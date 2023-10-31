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
		.add_plugins(MainPlugin)
		// .add_plugins((TestPlugin, bevy_editor_pls::EditorPlugin::default()))
		.add_plugins(TestPlugin)
		.run();
}

struct TestPlugin;
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
			}
			// Cli::Client { port, ip } => {
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

// fn main() {
//     App::new()
//         .init_resource::<Cli>() // Parse CLI before creating window.
//         .add_plugins((DefaultPlugins, ReplicationPlugins, SimpleBoxPlugin))
//         .run();
// }

// struct SimpleBoxPlugin;

// impl Plugin for SimpleBoxPlugin {
//     fn build(&self, app: &mut App) {
//         app.replicate::<PlayerPosition>()
//             .replicate::<PlayerColor>()
//             .add_client_event::<MoveDirection>(SendPolicy::Ordered)
//             .add_systems(
//                 Startup,
//                 (
//                     Self::cli_system.pipe(system_adapter::unwrap),
//                     Self::init_system,
//                 ),
//             )
//             .add_systems(
//                 Update,
//                 (
//                     Self::movement_system.run_if(has_authority()), // Runs only on the server or a single player.
//                     Self::server_event_system.run_if(resource_exists::<RenetServer>()), // Runs only on the server.
//                     (Self::draw_boxes_system, Self::input_system),
//                 ),
//             );
//     }
// }

// impl SimpleBoxPlugin {

//     fn init_system(mut commands: Commands) {
//         commands.spawn(Camera2dBundle::default());
//     }

//     /// Logs server events and spawns a new player whenever a client connects.
//     fn server_event_system(mut commands: Commands, mut server_event: EventReader<ServerEvent>) {
//         for event in &mut server_event {
//             match event {
//                 ServerEvent::ClientConnected { client_id } => {
//                     info!("player: {client_id} Connected");
//                     // Generate pseudo random color from client id.
//                     let r = ((client_id % 23) as f32) / 23.0;
//                     let g = ((client_id % 27) as f32) / 27.0;
//                     let b = ((client_id % 39) as f32) / 39.0;
//                     commands.spawn(PlayerBundle::new(
//                         *client_id,
//                         Vec2::ZERO,
//                         Color::rgb(r, g, b),
//                     ));
//                 }
//                 ServerEvent::ClientDisconnected { client_id, reason } => {
//                     info!("client {client_id} disconnected: {reason}");
//                 }
//             }
//         }
//     }

//     fn draw_boxes_system(mut gizmos: Gizmos, players: Query<(&PlayerPosition, &PlayerColor)>) {
//         for (position, color) in &players {
//             gizmos.rect(
//                 Vec3::new(position.x, position.y, 0.0),
//                 Quat::IDENTITY,
//                 Vec2::ONE * 50.0,
//                 color.0,
//             );
//         }
//     }

//     /// Reads player inputs and sends [`MoveCommandEvents`]
//     fn input_system(mut move_events: EventWriter<MoveDirection>, input: Res<Input<KeyCode>>) {
//         let mut direction = Vec2::ZERO;
//         if input.pressed(KeyCode::Right) {
//             direction.x += 1.0;
//         }
//         if input.pressed(KeyCode::Left) {
//             direction.x -= 1.0;
//         }
//         if input.pressed(KeyCode::Up) {
//             direction.y += 1.0;
//         }
//         if input.pressed(KeyCode::Down) {
//             direction.y -= 1.0;
//         }
//         if direction != Vec2::ZERO {
//             move_events.send(MoveDirection(direction.normalize_or_zero()));
//         }
//     }

//     /// Mutates [`PlayerPosition`] based on [`MoveCommandEvents`].
//     ///
//     /// Fast-paced games usually you don't want to wait until server send a position back because of the latency.
//     /// But this example just demonstrates simple replication concept.
//     fn movement_system(
//         time: Res<Time>,
//         mut move_events: EventReader<FromClient<MoveDirection>>,
//         mut players: Query<(&Player, &mut PlayerPosition)>,
//     ) {
//         const MOVE_SPEED: f32 = 300.0;
//         for FromClient { client_id, event } in &mut move_events {
//             info!("received event {event:?} from client {client_id}");
//             for (player, mut position) in &mut players {
//                 if *client_id == player.0 {
//                     **position += event.0 * time.delta_seconds() * MOVE_SPEED;
//                 }
//             }
//         }
//     }
// }

// impl Default for Cli {
//     fn default() -> Self {
//         Self::parse()
//     }
// }

// #[derive(Bundle)]
// struct PlayerBundle {
//     player: Player,
//     position: PlayerPosition,
//     color: PlayerColor,
//     replication: Replication,
// }

// impl PlayerBundle {
//     fn new(id: u64, position: Vec2, color: Color) -> Self {
//         Self {
//             player: Player(id),
//             position: PlayerPosition(position),
//             color: PlayerColor(color),
//             replication: Replication,
//         }
//     }
// }

// /// Contains the client ID of the player.
// #[derive(Component, Serialize, Deserialize)]
// struct Player(u64);

// #[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
// struct PlayerPosition(Vec2);

// #[derive(Component, Deserialize, Serialize)]
// struct PlayerColor(Color);

// /// A movement event for the controlled box.
// #[derive(Debug, Default, Deserialize, Event, Serialize)]
// struct MoveDirection(Vec2);
