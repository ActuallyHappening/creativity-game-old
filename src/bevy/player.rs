use crate::core::PlayerInventory;

use super::camera::handle_camera_movement;
use crate::utils::*;
use std::ops::Deref;

mod thrust;
use lazy_static::lazy_static;
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
					// gather_player_movement.pipe(info),
					// gather_player_movement.pipe(vectorise_input_flags).pipe(info),
					// manual_get_final_thrust.pipe(apply_thrust).pipe(ignore),
					pipe(
						join3(
							pipe(
								join2(
									pipe(
										join2(gather_input_flags, get_base_normal_vectors),
										flag_normal_vectors,
									),//.pipe(info),
									max_velocity_magnitudes,
								),//.pipe(info),
								get_relative_strengths,
							),//.pipe(info),
							get_base_normal_vectors,
							force_factors,
						),//.pipe(info),
						save_thrust_stages,
					).pipe(apply_thrust).pipe(info),
				),
			);
	}
}

#[derive(Component, Default)]
pub struct MainPlayer {
	relative_thrust: Thrust<RelativeStrength>,
}

lazy_static! {
	static ref PLAYER_STRUCTURE: Structure = Structure::new([
		(PixelVariant::PlayerSteel, (0, 0, 0)),
		(PixelVariant::PlayerSteel, (0, 0, -1)),
		(PixelVariant::PlayerLargeEngineDecoration, (0, 0, 1)),
		(PixelVariant::PlayerSteel, (-1, 0, 0)),
		(PixelVariant::PlayerSteel, (1, 0, 0)),
		(PixelVariant::PlayerSteel, (-1, 0, 1)),
		(PixelVariant::PlayerSteel, (1, 0, 1)),
		(PixelVariant::PlayerSteel, (0, 1, 1)),
		(PixelVariant::PlayerSteel, (0, -1, 1)),
	])
	.with([
		(Thruster::new(Direction::Backward), (0, 0, 2)),
		(Thruster::new(Direction::Left), (-2, 0, 1)),
		(Thruster::new(Direction::Right), (2, 0, 1)),
		(Thruster::new(Direction::Up), (0, 2, 1)),
		(Thruster::new(Direction::Down), (0, -2, 1)),
	]);
}

fn initial_spawn_player(
	mut commands: Commands,
	mut mma: MMA,
	effects: ResMut<Assets<EffectAsset>>,
) {
	info!("Spawning player");
	commands
		.spawn(
			(
				PbrBundle {
					transform: Transform::from_xyz(0., PIXEL_SIZE * 7., 0.),
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
			// parent.spawn(PbrBundle {
			// 	material: mma.mats.add(Color::GREEN.into()),
			// 	transform: Transform::from_xyz(0., 0., -15.),
			// 	mesh: mma.meshs.add(shape::Box::new(PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE).into()),
			// 	..default()
			// });

			for part in PLAYER_STRUCTURE.spawn_bevy_bundles(&mut mma, effects) {
				part.spawn_to_parent(parent);
			}
		});
}

fn handle_player_mined_px(
	mut e: EventReader<PlayerMinedPixel>,
	mut inventory: ResMut<PlayerInventory>,
) {
	for px in e.iter().map(|p| p.deref()) {
		info!("Player mined pixel: {:?}", px);
		inventory[px.variant] += px
			.variant
			.get_variant_info()
			.collectable
			.as_ref()
			.unwrap()
			.amount_multiplier as u32;
	}
}
