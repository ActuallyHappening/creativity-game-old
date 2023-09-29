//! Thrust is processed in various ways.
//!
//! Braking:
//! - Tries to stop player in space across all dimensions
//! - Ignores all other player inputs while braked

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
mod braking;
pub use braking::*;

pub mod types;

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
#[allow(clippy::type_complexity)]
pub fn save_thrust_stages(
	In((relative_strength, normal_vectors, max, braking_info)): In<(
		Thrust<RelativeStrength>,
		Thrust<BasePositionNormalVectors>,
		Thrust<ForceFactors>,
		BrakingInfo,
	)>,
	mut player_data: Query<&mut MainPlayer, With<MainPlayer>>,
) -> Thrust<FinalVectors> {
	let final_vectors = normal_vectors * relative_strength.clone() * max;

	*player_data.single_mut() = MainPlayer {
		relative_strength,
		inputs: braking_info,
	};

	final_vectors
}

pub fn manually_threading_player_movement(
	In(current_velocity): In<Thrust<RelativeVelocityMagnitudes>>,
	keyboard_input: Res<Input<KeyCode>>,
	player_transform: Query<&Transform, With<MainPlayer>>,
	player_velocity: Query<&Velocity, With<MainPlayer>>,
	player_data: Query<&mut MainPlayer, With<MainPlayer>>,
	time: Res<Time>,
	player_physics: Query<&mut ExternalForce, With<MainPlayer>>,
) {
	let base_normal = get_base_normal_vectors(player_transform);

	let BrakingInfo(is_braking, input_flags) = match gather_input_flags(keyboard_input) {
		None => {
			// breaking, must do opposite of current velocity to counteract / brake
			const CUTOFF: f32 = 0.02;
			let mut flagged_inputs = Thrust::<NonBrakingInputFlags>::default();

			for thrust_type in ThrustType::iter() {
				let current = current_velocity.get_from_type(thrust_type);
				if current > &CUTOFF {
					flagged_inputs.set_from_type(thrust_type, Some(false));
				} else if current < &-CUTOFF {
					flagged_inputs.set_from_type(thrust_type, Some(true));
				} else {
					// cheating-ly stop velocity because it is small enough
					flagged_inputs.set_from_type(thrust_type, None);
				}
			}

			if current_velocity.forward > CUTOFF {
				flagged_inputs.forward = Some(false)
			} else if current_velocity.forward < -CUTOFF {
				flagged_inputs.forward = Some(true)
			} else {
				// cheating-ly stop velocity because it is small enough
				flagged_inputs.forward = None;
			}

			BrakingInfo(true, flagged_inputs)
		}
		Some(input_flags) => {
			// not breaking, can use what player provides
			BrakingInfo(false, input_flags)
		}
	};

	const BRAKING_FORCE_PENALTY: f32 = 0.15;
	let force_factors = force_factors() * if is_braking {
		BRAKING_FORCE_PENALTY
	} else {
		1.0
	};

	let flagged_inputs = input_flags.clone() * base_normal.clone();
	let relative_strengths = get_relative_strengths(
		In((flagged_inputs, max_velocity_magnitudes())),
		player_velocity,
	);
	let final_vectors = save_thrust_stages(
		In((relative_strengths, base_normal, force_factors, BrakingInfo(is_braking, input_flags))),
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
