use crate::core::PlayerInventory;

use super::camera::handle_camera_movement;
use crate::utils::*;
use std::ops::Deref;

mod thrust;
use thrust::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(
				Startup,
				(initial_spawn_player, |mut commands: Commands| {
					commands.insert_resource(PlayerInventory::default());
				}),
			)
			.add_systems(
				Update,
				(
					handle_camera_movement,
					handle_player_mined_px,
					// gather_player_movement.pipe(vectorise_input_flags).pipe(info)
					manual_get_final_thrust.pipe(apply_thrust).pipe(info),
				),
			);
	}
}

const PLAYER_HEIGHT: f32 = 25.;

#[derive(Component, Default)]
pub struct MainPlayer {
	thrust: Thrust<RelativeStrength>,
}

fn initial_spawn_player(
	mut commands: Commands,
	MMA {
		mut meshs,
		mut mats,
		..
	}: MMA,
) {
	info!("Spawning player");
	commands
		.spawn(
			(
				PbrBundle {
					material: mats.add(Color::SILVER.into()),
					transform: Transform::from_xyz(0., PLAYER_HEIGHT, 0.),
					mesh: meshs
						.add(shape::Box::new(2. * PIXEL_SIZE, 2. * PIXEL_SIZE, 2. * PIXEL_SIZE).into()),
					..default()
				},
				MainPlayer::default(),
			)
				.named("Main Player")
				.physics_dynamic()
				.physics_collider_ball(10.)
				.physics_zero_force()
				.physics_zero_velocity()
				.physics_zero_damping(),
		)
		.with_children(|parent| {
			parent.spawn(PbrBundle {
				material: mats.add(Color::GREEN.into()),
				transform: Transform::from_xyz(0., 0., -15.),
				mesh: meshs.add(shape::Box::new(PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE).into()),
				..default()
			});
		});
}

fn handle_player_mined_px(
	mut e: EventReader<PlayerMinedPixel>,
	mut inventory: ResMut<PlayerInventory>,
) {
	for px in e.iter().map(|p| p.deref()) {
		info!("Player mined pixel: {:?}", px);
		inventory[px.variant] += px.collectable.as_ref().unwrap().amount_multiplier as u32;
	}
}
