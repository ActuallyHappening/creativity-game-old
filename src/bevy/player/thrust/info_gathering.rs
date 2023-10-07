use super::*;

impl ThrustType {
	/// Returns (positive, negative) keybinds
	pub const fn keybinds(self) -> (KeyCode, KeyCode) {
		match self {
			ThrustType::Forward => (KeyCode::R, KeyCode::F),
			ThrustType::Right => (KeyCode::S, KeyCode::H),
			ThrustType::Up => (KeyCode::Y, KeyCode::W),

			ThrustType::TurnLeft => (KeyCode::D, KeyCode::G),
			ThrustType::TiltUp => (KeyCode::T, KeyCode::E),
			ThrustType::RollLeft => (KeyCode::X, KeyCode::B),
		}
	}

	fn gather_flag(self, inputs: &Input<KeyCode>) -> Option<bool> {
		let (positive, negative) = self.keybinds();
		match (inputs.pressed(positive), inputs.pressed(negative)) {
			(true, true) | (false, false) => None,
			(true, false) => Some(true),
			(false, true) => Some(false),
		}
	}

	fn gather_flags(inputs: &Input<KeyCode>) -> Thrust<GenericInputFlags> {
		let mut ret = Thrust::default();

		for ttype in ThrustType::iter() {
			let flag = ttype.gather_flag(inputs);
			ret.set_from_type(ttype, flag);
		}

		ret
	}
}

/// [Option::None] when braking
pub(super) fn gather_input_flags(
	keyboard_input: Res<Input<KeyCode>>,
) -> Option<Thrust<GenericInputFlags>> {
	if keyboard_input.pressed(KeyCode::ShiftLeft) {
		None
	} else {
		Some(ThrustType::gather_flags(&keyboard_input))
	}
}

pub fn get_base_normal_vectors(
	player_transform: Query<&Transform, With<MainPlayer>>,
) -> Thrust<BasePositionNormalVectors> {
	let player = player_transform.single();

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

pub(super) fn max_velocity_magnitudes() -> Thrust<MaxAllowableVelocityMagnitudes> {
	impl MainPlayer {
		const MAX_LINEAR_VELOCITY: f32 = 10.;
		const MAX_ANGULAR_VELOCITY: f32 = 0.2;
	}

	Thrust::<MaxAllowableVelocityMagnitudes> {
		forward: 20.,
		up: MainPlayer::MAX_LINEAR_VELOCITY,
		right: MainPlayer::MAX_LINEAR_VELOCITY,

		tilt_up: MainPlayer::MAX_ANGULAR_VELOCITY * 2.,
		turn_left: MainPlayer::MAX_ANGULAR_VELOCITY,
		roll_left: MainPlayer::MAX_ANGULAR_VELOCITY,
		_stage: PhantomData,
	}
}

pub(super) const fn force_factors() -> Thrust<ForceFactors> {
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

pub fn get_current_af_flags(
	player: Query<&MainPlayer>,
) -> Thrust<ArtificialFrictionFlags> {
	player.single().artificial_friction_flags.clone()
}
