use crate::utils::*;

use super::MainPlayer;

mod helpers;
use helpers::*;

mod stages;
pub use stages::*;

mod info_gathering;
pub use info_gathering::*;
mod info_processors;
pub use info_processors::*;
mod info_enactors;
pub use info_enactors::*;

#[derive(Debug, Clone)]
pub struct Thrust<S: ThrustStage> {
	/// Positive is forward obviously
	pub forward: <S as self::ThrustStage>::DimensionType,

	pub up: <S as self::ThrustStage>::DimensionType,

	pub right: <S as self::ThrustStage>::DimensionType,

	/// Left is positive
	pub turn_left: <S as self::ThrustStage>::DimensionType,

	/// Upwards is positive
	pub tilt_up: <S as self::ThrustStage>::DimensionType,

	/// Right is positive
	pub roll_left: <S as self::ThrustStage>::DimensionType,

	_stage: PhantomData<S>,
}

pub trait ThrustStage {
	type DimensionType: std::fmt::Debug + Clone;
}


impl<D, T> Default for Thrust<D>
where
	D: ThrustStage<DimensionType = T> + std::default::Default,
	D::DimensionType: Default,
{
	fn default() -> Self {
		Thrust::<D> {
			forward: T::default(),
			up: T::default(),
			right: T::default(),

			turn_left: T::default(),
			tilt_up: T::default(),
			roll_left: T::default(),
			_stage: PhantomData,
		}
	}
}


/// Combines the normal and relative thrusts into the final thrust vectors,
/// and saves the necessary information to various places including in the [MainPlayer] component
pub fn save_thrust_stages(
	In((relative_strength, normal_vectors, max)): In<(
		Thrust<RelativeStrength>,
		Thrust<BasePositionNormalVectors>,
		Thrust<ForceFactors>,
	)>,
	mut player_data: Query<&mut MainPlayer, With<MainPlayer>>,
) -> Thrust<FinalVectors> {
	// relative (F) * normals (U) = almost final (FLAGGED)
	impl std::ops::Mul<Thrust<RelativeStrength>> for Thrust<BasePositionNormalVectors> {
		type Output = Thrust<AlmostFinalVectors>;

		fn mul(self, rhs: Thrust<RelativeStrength>) -> Self::Output {
			Thrust::<AlmostFinalVectors> {
				forward: self.forward * rhs.forward,
				up: self.up * rhs.up,
				right: self.right * rhs.right,

				turn_left: self.turn_left * rhs.turn_left,
				tilt_up: self.tilt_up * rhs.tilt_up,
				roll_left: self.roll_left * rhs.roll_left,
				_stage: PhantomData,
			}
		}
	}

	// Almost final * force factor
	impl std::ops::Mul<Thrust<ForceFactors>> for Thrust<AlmostFinalVectors> {
		type Output = Thrust<FinalVectors>;

		fn mul(self, rhs: Thrust<ForceFactors>) -> Self::Output {
			Thrust::<FinalVectors> {
				forward: self.forward * rhs.forward,
				up: self.up * rhs.up,
				right: self.right * rhs.right,

				turn_left: self.turn_left * rhs.turn_left,
				tilt_up: self.tilt_up * rhs.tilt_up,
				roll_left: self.roll_left * rhs.roll_left,
				_stage: PhantomData,
			}
		}
	}

	let final_vectors = normal_vectors * relative_strength.clone() * max;

	player_data.single_mut().relative_thrust = relative_strength;

	final_vectors
}

pub fn manually_threading_player_movement(
	keyboard_input: Res<Input<KeyCode>>,
	player_transform: Query<&Transform, With<MainPlayer>>,
	player_velocity: Query<&Velocity, With<MainPlayer>>,
	player_data: Query<&mut MainPlayer, With<MainPlayer>>,
	time: Res<Time>,
	player_physics: Query<&mut ExternalForce, With<MainPlayer>>,
) {
	let base_normal = get_base_normal_vectors(player_transform);
	let flagged_inputs = flag_normal_vectors(In((
		gather_input_flags(keyboard_input).expect("Unimplemented braking system"),
		base_normal.clone(),
	)));
	let relative_strengths = get_relative_strengths(
		In((flagged_inputs, max_velocity_magnitudes())),
		player_velocity,
	);
	let final_vectors = save_thrust_stages(
		In((relative_strengths, base_normal, force_factors())),
		player_data,
	);

	apply_thrust(In(final_vectors), player_physics, time);
}


// #[bevycheck::system]
// pub fn manual_get_final_thrust(
// 	keyboard_input: Res<Input<KeyCode>>,
// 	player_pos: Query<&Transform, With<MainPlayer>>,
// 	player_current: Query<(&Velocity, &Transform), With<MainPlayer>>,
// 	player_save: Query<&mut MainPlayer, With<MainPlayer>>,
// ) -> Thrust<FinalVectors> {
// 	let normals = flag_normal_vectors(In(gather_player_movement(keyboard_input)), player_pos);
// 	let relative = get_relative_strengths(
// 		In((get_max_velocity_magnitudes(), normals.clone())),
// 		player_current,
// 	);

// 	save_thrust_stages(In((normals, relative, get_force_factors())), player_save)
// }
