use super::*;

/// [Option::None] when braking
pub fn gather_input_flags(keyboard_input: Res<Input<KeyCode>>) -> Option<Thrust<NonBrakingInputFlags>> {
	if keyboard_input.pressed(KeyCode::ShiftLeft) {
		None
	} else {
		Some(match keyboard_input.pressed(KeyCode::Space) {
			false => Thrust::<NonBrakingInputFlags> {
				forward: match (
					keyboard_input.pressed(KeyCode::W),
					keyboard_input.pressed(KeyCode::S),
				) {
					(true, true) | (false, false) => None,
					(true, false) => Some(true),
					(false, true) => Some(false),
				},
				up: match (
					keyboard_input.pressed(KeyCode::Q),
					keyboard_input.pressed(KeyCode::E),
				) {
					(true, true) | (false, false) => None,
					(true, false) => Some(true),
					(false, true) => Some(false),
				},
				right: match (
					keyboard_input.pressed(KeyCode::D),
					keyboard_input.pressed(KeyCode::A),
				) {
					(true, true) | (false, false) => None,
					(true, false) => Some(true),
					(false, true) => Some(false),
				},
				..default()
			},
			true => Thrust {
				turn_left: match (
					keyboard_input.pressed(KeyCode::A),
					keyboard_input.pressed(KeyCode::D),
				) {
					(true, true) | (false, false) => None,
					(true, false) => Some(true),
					(false, true) => Some(false),
				},
				tilt_up: match (
					keyboard_input.pressed(KeyCode::S),
					keyboard_input.pressed(KeyCode::W),
				) {
					(true, true) | (false, false) => None,
					(true, false) => Some(true),
					(false, true) => Some(false),
				},
				roll_left: match (
					keyboard_input.pressed(KeyCode::Q),
					keyboard_input.pressed(KeyCode::E),
				) {
					(true, true) | (false, false) => None,
					(true, false) => Some(true),
					(false, true) => Some(false),
				},
				..default()
			},
		})
	}
}

pub fn get_base_normal_vectors(
	player_transform: Query<&Transform, With<MainPlayer>>,
) -> Thrust<BasePositionNormalVectors> {
	let player = match player_transform.get_single() {
		Ok(player) => player,
		Err(e) => panic!("No player found: {:?}", e),
	};

	let forward = player.forward();
	let up = player.up();

	// the meat of the system
	Thrust::<BasePositionNormalVectors> {
		forward,
		up,
		right: forward.cross(up),

		turn_left: up,
		tilt_up: forward.cross(up),
		roll_left: -forward,
		_stage: PhantomData,
	}
}

pub const fn max_velocity_magnitudes() -> Thrust<MaxAllowableVelocityMagnitudes> {
	impl MainPlayer {
		const MAX_LINEAR_VELOCITY: f32 = 10.;
		const MAX_ANGULAR_VELOCITY: f32 = 0.2;
	}

	Thrust::<MaxAllowableVelocityMagnitudes> {
		forward: MainPlayer::MAX_LINEAR_VELOCITY,
		up: MainPlayer::MAX_LINEAR_VELOCITY,
		right: MainPlayer::MAX_LINEAR_VELOCITY,

		turn_left: MainPlayer::MAX_ANGULAR_VELOCITY,
		tilt_up: MainPlayer::MAX_ANGULAR_VELOCITY,
		roll_left: MainPlayer::MAX_ANGULAR_VELOCITY,
		_stage: PhantomData,
	}
}

pub const fn force_factors() -> Thrust<ForceFactors> {
	impl MainPlayer {
		const MOVE_FACTOR: f32 = 5_000_000.;
		const TURN_FACTOR: f32 = 5_000_000.;
	}

	Thrust::<ForceFactors> {
		forward: MainPlayer::MOVE_FACTOR,
		up: MainPlayer::MOVE_FACTOR,
		right: MainPlayer::MOVE_FACTOR,

		turn_left: MainPlayer::TURN_FACTOR,
		tilt_up: MainPlayer::TURN_FACTOR,
		roll_left: MainPlayer::TURN_FACTOR,
		_stage: PhantomData,
	}
}

/// Should be ran after [PlayerMovement] system set
// pub fn get_current_final_vectors(player: Query<&MainPlayer>) -> Thrust<FinalVectors> {
// 	player.single().final_vectors.clone()
// }

pub fn get_current_relative_strengths(player: Query<&MainPlayer>) -> Thrust<RelativeStrength> {
	player.single().relative_strength.clone()
}

pub fn get_current_braking_info(player: Query<&MainPlayer>) -> BrakingInfo {
	player.single().inputs.clone()
}