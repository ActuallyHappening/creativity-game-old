use std::{f32::consts::PI, time::Duration};
use crate::utils::*;

use bevy::prelude::{shape::Icosphere, *};
use bevy_rapier3d::prelude::*;
use bevy_renet::renet::{ChannelConfig, ClientId, ConnectionConfig, SendType};
use serde::{Deserialize, Serialize};

mod client;
mod server;

pub struct RenetPlugin;
impl Plugin for RenetPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugins(bevy_renet::RenetServerPlugin)
			.add_plugins(bevy_renet::RenetClientPlugin)
			.add_systems(OnEnter(ServerConnections::Hosting), server::add_netcode_network);
	}
}

// #[cfg(feature = "transport")]
pub const PRIVATE_KEY: &[u8; bevy_renet::renet::transport::NETCODE_KEY_BYTES] =
	b"un example sehr tres secret key."; // 32-bytes
// #[cfg(feature = "transport")]
pub const PROTOCOL_ID: u64 = 7;

#[derive(Debug, Component)]
pub struct Player {
	pub id: ClientId,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
	pub up: bool,
	pub down: bool,
	pub left: bool,
	pub right: bool,
}

#[derive(Debug, Serialize, Deserialize, Component, Event)]
pub enum PlayerCommand {
	Shoot,
}

pub enum ClientChannel {
	Input,
	Command,
}

pub enum ServerChannel {
	ServerMessages,
	NetworkedEntities,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
	PlayerCreate {
		entity: Entity,
		id: ClientId,
		translation: [f32; 3],
	},
	PlayerRemove {
		id: ClientId,
	},
	SpawnProjectile {
		entity: Entity,
		translation: [f32; 3],
	},
	DespawnProjectile {
		entity: Entity,
	},
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NetworkedEntities {
	pub entities: Vec<Entity>,
	pub translations: Vec<[f32; 3]>,
}

impl From<ClientChannel> for u8 {
	fn from(channel_id: ClientChannel) -> Self {
		match channel_id {
			ClientChannel::Command => 0,
			ClientChannel::Input => 1,
		}
	}
}

impl ClientChannel {
	pub fn channels_config() -> Vec<ChannelConfig> {
		vec![
			ChannelConfig {
				channel_id: Self::Input.into(),
				max_memory_usage_bytes: 5 * 1024 * 1024,
				send_type: SendType::ReliableOrdered {
					resend_time: Duration::ZERO,
				},
			},
			ChannelConfig {
				channel_id: Self::Command.into(),
				max_memory_usage_bytes: 5 * 1024 * 1024,
				send_type: SendType::ReliableOrdered {
					resend_time: Duration::ZERO,
				},
			},
		]
	}
}

impl From<ServerChannel> for u8 {
	fn from(channel_id: ServerChannel) -> Self {
		match channel_id {
			ServerChannel::NetworkedEntities => 0,
			ServerChannel::ServerMessages => 1,
		}
	}
}

impl ServerChannel {
	pub fn channels_config() -> Vec<ChannelConfig> {
		vec![
			ChannelConfig {
				channel_id: Self::NetworkedEntities.into(),
				max_memory_usage_bytes: 10 * 1024 * 1024,
				send_type: SendType::Unreliable,
			},
			ChannelConfig {
				channel_id: Self::ServerMessages.into(),
				max_memory_usage_bytes: 10 * 1024 * 1024,
				send_type: SendType::ReliableOrdered {
					resend_time: Duration::from_millis(200),
				},
			},
		]
	}
}

pub fn connection_config() -> ConnectionConfig {
	ConnectionConfig {
		available_bytes_per_tick: 1024 * 1024,
		client_channels_config: ClientChannel::channels_config(),
		server_channels_config: ServerChannel::channels_config(),
	}
}

/// set up a simple 3D scene
pub fn setup_level(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// plane
	commands
		.spawn(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Box::new(40., 1., 40.))),
			material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
			transform: Transform::from_xyz(0.0, -1.0, 0.0),
			..Default::default()
		})
		.insert(Collider::cuboid(20., 0.5, 20.));
	// light
	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			shadows_enabled: true,
			..default()
		},
		transform: Transform {
			translation: Vec3::new(0.0, 2.0, 0.0),
			rotation: Quat::from_rotation_x(-PI / 4.),
			..default()
		},
		..default()
	});
}

#[derive(Debug, Component)]
pub struct Projectile {
	pub duration: Timer,
}

pub fn spawn_fireball(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	translation: Vec3,
	mut direction: Vec3,
) -> Entity {
	if !direction.is_normalized() {
		direction = Vec3::X;
	}
	commands
		.spawn(PbrBundle {
			mesh: meshes.add(
				Mesh::try_from(Icosphere {
					radius: 0.1,
					subdivisions: 5,
				})
				.unwrap(),
			),
			material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
			transform: Transform::from_translation(translation),
			..Default::default()
		})
		.insert(RigidBody::Dynamic)
		.insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y)
		// .insert(Collider::ball(0.1))
		.insert(Velocity::linear(direction * 1.))
		.insert(ActiveEvents::COLLISION_EVENTS)
		.insert(Projectile {
			duration: Timer::from_seconds(10.5, TimerMode::Once),
		})
		.id()
}
