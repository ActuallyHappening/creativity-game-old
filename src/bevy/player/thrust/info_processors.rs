use std::ops::Mul;

use super::*;

/// Takes into account the maximum power of each thruster and the current velocity
pub fn get_relative_strengths(
	In((aimed, max)): In<(
		Thrust<FlaggedPositionNormalVectors>,
		Thrust<MaxAllowableVelocityMagnitudes>,
	)>,
	player_velocity: Query<&Velocity, With<MainPlayer>>,
) -> Thrust<RelativeStrength> {
	let Velocity { linvel, angvel } = player_velocity.single();

	fn factor_allowed_forwards(aimed: Signed<Vec3>, max: f32, current: &Vec3) -> f32 {
		if aimed.is_zero() {
			0.
		} else if current.length() == 0. {
			aimed.into_unit()
		} else {
			let aimed_vec: Vec3 = aimed.factor_in();
			// 0 when speeding up, 1 when slowing down
			let factor_slowing_down = 1. - aimed_vec.factor_towards(current);

			// 1 when at max velocity, 0 when at 0 velocity
			let percentage_of_max_allowed_velocity = (current.length() / max).clamp(0., 1.);

			const CUTOFF: f32 = 0.6;
			if percentage_of_max_allowed_velocity > CUTOFF {
				// gradually slow down
				// 0 when at max velocity, 1 when at cutoff velocity
				let max_limit_factor = 1. - percentage_of_max_allowed_velocity.map_num(CUTOFF, 1., 0., 1.);
				assert!((0. .. 1.).contains(&max_limit_factor));

				(factor_slowing_down + max_limit_factor).clamp_max(1.) * aimed.into_unit()
			} else {
				aimed.into_unit()
			}
		}
	}

	Thrust::<RelativeStrength> {
		forward: factor_allowed_forwards(aimed.forward, max.forward, linvel),
		up: factor_allowed_forwards(aimed.up, max.up, linvel),
		right: factor_allowed_forwards(aimed.right, max.right, linvel),

		tilt_up: factor_allowed_forwards(aimed.tilt_up, max.tilt_up, angvel),
		roll_left: factor_allowed_forwards(aimed.roll_left, max.roll_left, angvel),
		turn_left: factor_allowed_forwards(aimed.turn_left, max.turn_left, angvel),
		_stage: PhantomData,
	}
}

pub fn calculate_relative_velocity_magnitudes(
	In(base): In<Thrust<BasePositionNormalVectors>>,
	velocity: Query<&Velocity, With<MainPlayer>>,
) -> Thrust<RelativeVelocityMagnitudes> {
	let max = max_velocity_magnitudes();
	let velocity = velocity.single();

	Thrust::<RelativeVelocityMagnitudes> {
		forward: velocity
			.linvel
			.vector_project(&base.forward)
			.signed_length()
			/ max.forward,
		up: velocity.linvel.vector_project(&base.up).signed_length() / max.up,
		right: velocity.linvel.vector_project(&base.right).signed_length() / max.right,

		turn_left: velocity
			.angvel
			.vector_project(&base.turn_left)
			.signed_length()
			/ max.turn_left,
		tilt_up: velocity
			.angvel
			.vector_project(&base.tilt_up)
			.signed_length()
			/ max.tilt_up,
		roll_left: velocity
			.angvel
			.vector_project(&base.roll_left)
			.signed_length()
			/ max.roll_left,

		_stage: PhantomData,
	}
}
