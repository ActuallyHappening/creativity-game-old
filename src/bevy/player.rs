use crate::core::PlayerInventory;

use self::weapons::{handle_firing, should_fire_this_frame, toggle_fire, update_bullets};

use super::camera::handle_camera_movement;
use crate::utils::*;
use lazy_static::lazy_static;

mod thrust;
use thrust::*;
pub use thrust::{
	calculate_relative_velocity_magnitudes, get_base_normal_vectors, get_current_relative_strengths,
	types, RelativeStrength, RelativeVelocityMagnitudes, Thrust, ThrustReactions,
	ThrustReactionsStage,
};

mod weapons;
pub use weapons::WeaponFlags;

pub struct PlayerPlugin;

/// After player thrusts and movement have been handled
#[derive(Hash, Debug, PartialEq, Eq, Clone, SystemSet)]
pub struct PlayerMove;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<PlayerInventory>()
			.add_systems(Startup, (initial_spawn_player,))
			.add_systems(Update, (update_bullets,))
			.add_systems(
				Update,
				(
					handle_camera_movement,
					should_fire_this_frame.pipe(toggle_fire).pipe(handle_firing),
					join2(
						sequence(
							get_base_normal_vectors,
							calculate_relative_velocity_magnitudes,
						),
						get_current_af_flags,
					)
					.pipe(manually_threading_player_movement)
					.in_set(PlayerMove),
					trigger_player_thruster_particles.after(PlayerMove),
				),
			);
	}
}

/// Denotes the main, controllable player
#[derive(Component, Default)]
pub struct MainPlayer {
	/// Current relative strength, used for UI
	pub relative_strength: Thrust<RelativeStrength>,
	/// Current inputs including braking info, used for UI
	pub thrust_responses: Thrust<ThrustReactionsStage>,
	/// Optional artificial friction flags, starts all enabled
	pub artificial_friction_flags: Thrust<ArtificialFrictionFlags>,
}

lazy_static! {
	static ref PLAYER_STRUCTURE: Structure = Structure::new([
		(PixelVariant::PlayerSteel, (0, 0, 0)), // center
		(PixelVariant::PlayerSteel, (0, 0, -1)), // front 1
		(PixelVariant::PlayerSteel, (0, 0, -2)), // front 2
		(PixelVariant::PlayerLargeEngineDecoration, (0, 0, 1)), // back 1
		(PixelVariant::PlayerSteel, (-1, 0, 0)), // left 1
		(PixelVariant::PlayerSteel, (-2, 0, 0)), // left 2
		(PixelVariant::PlayerSteel, (-2, 0, -1)), // left 2, front 1
		(PixelVariant::PlayerSteel, (-1, 0, 1)), // surrounding engine left
		(PixelVariant::PlayerSteel, (0, 1, 1)), // surrounding engine above
	]).with([
		(Thruster::new(Direction::Up, ThrusterFlags::builder().up_down(false).tilt_up(true).roll_right(true).build().unwrap()), (-1, 1, 1)),
		(Thruster::new(Direction::Left, ThrusterFlags::builder().right_left(true).turn_right(false).roll_right(false).build().unwrap()), (-1, 1, 1)),
		(Thruster::new(Direction::Backward, ThrusterFlags::builder().forward_back(true).build().unwrap()), (-1, 0, 2)),
		(Thruster::new(Direction::Forward, ThrusterFlags::builder().forward_back(false).build().unwrap()), (-1, 0, -1)),
	]).with([
		(Weapon::new(Direction::Forward), (-2, 0, -2)), // left 2, front 2
	])
	.reflect_horizontally()
	.reflect_vertically();
}

fn initial_spawn_player(
	mut commands: Commands,
	mut mma: MMA,
	effects: ResMut<Assets<EffectAsset>>,
) {
	info!("Spawning player");

	let (collider, player_parts) = PLAYER_STRUCTURE.compute_bevy_bundles(&mut mma, Some(effects));

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
				.insert(collider)
				// .physics_collider_ball(10.)
				.physics_zero_force()
				.physics_zero_velocity()
				.physics_zero_damping()
				.physics_never_sleep(),
		)
		.with_children(|parent| {
			for part in player_parts {
				part.default_spawn_to_parent(parent);
			}
		});
}
