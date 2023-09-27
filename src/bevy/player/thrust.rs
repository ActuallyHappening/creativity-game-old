use crate::utils::*;

use super::MainPlayer;

#[derive(Debug, Default, Clone)]
pub struct Thrust<S: ThrustStage> {
	/// Left is positive
	side_left: <S as self::ThrustStage>::DimensionType,

	/// Upwards is positive
	tilt_up: <S as self::ThrustStage>::DimensionType,

	/// Right is positive
	roll_left: <S as self::ThrustStage>::DimensionType,

	/// Positive is forward obviously
	forward: <S as self::ThrustStage>::DimensionType,

	_stage: PhantomData<S>,
}

pub trait ThrustStage {
	type DimensionType: std::fmt::Debug + Default + Clone;
}
macro_rules! thrust_stage {
	($(#[$($attrss:tt)*])* $(pub)? struct $name:ident; type = $type:ty) => {
		$(#[$($attrss)*])*

		#[derive(Debug, Default, Clone)]
		pub struct $name;
		impl ThrustStage for $name {
			type DimensionType = $type;
		}
	};
}

thrust_stage!(
	/// What keys were pressed Option of bool for each dimension
	pub struct InputFlags; type = Option<bool>
);

thrust_stage!(
	/// Vectors of length 1, used as a helper stage
	pub struct NormalVectors; type = Vec3
);

thrust_stage!(
	/// Vectors of maximum, used as a helper stage
	pub struct MaxVelocityMagnitudes; type = f32
);

thrust_stage!(
	/// Shows how much of the maximum power can be used
	/// Used for animating the player and for relative readings
	pub struct RelativeStrength; type = f32
);

thrust_stage!(
	/// Factors multiplied into the final thrust vectors
	pub struct ForceFactors; type = f32
);

thrust_stage!(
	/// Final result which is applied to physics engine
	/// Used for absolute readings
	pub struct FinalVectors; type = Vec3
);

thrust_stage!(
	/// Just needs to be multiplied by the [ForceFactors] before it is a final thrust type
	pub struct AlmostFinalVectors; type = Vec3
);

// #[bevycheck::system]
pub fn gather_player_movement(keyboard_input: Res<Input<KeyCode>>) -> Thrust<InputFlags> {
	Thrust::<InputFlags> {
		forward: match (
			keyboard_input.pressed(KeyCode::W),
			keyboard_input.pressed(KeyCode::S),
		) {
			(true, true) | (false, false) => None,
			(true, false) => Some(true),
			(false, true) => Some(false),
		},
		side_left: match (
			keyboard_input.pressed(KeyCode::A),
			keyboard_input.pressed(KeyCode::D),
		) {
			(true, true) | (false, false) => None,
			(true, false) => Some(true),
			(false, true) => Some(false),
		},
		tilt_up: match (
			keyboard_input.pressed(KeyCode::Space),
			keyboard_input.pressed(KeyCode::ShiftLeft),
		) {
			(true, true) | (false, false) => None,
			(true, false) => Some(true),
			(false, true) => Some(false),
		},
		roll_left: match (
			keyboard_input.pressed(KeyCode::E),
			keyboard_input.pressed(KeyCode::Q),
		) {
			(true, true) | (false, false) => None,
			(true, false) => Some(true),
			(false, true) => Some(false),
		},
		_stage: PhantomData,
	}
}

// #[bevycheck::system]
pub fn vectorise_input_flags(
	In(input_flags): In<Thrust<InputFlags>>,
	player: Query<&Transform, With<MainPlayer>>,
) -> Thrust<NormalVectors> {
	let player = match player.get_single() {
		Ok(player) => player,
		Err(e) => panic!("No player found: {:?}", e),
	};

	#[extension(trait OptionExt)]
	impl Option<bool> {
		fn into_f32(self) -> f32 {
			match self {
				Some(true) => 1.,
				Some(false) => -1.,
				None => 0.,
			}
		}
	}

	let forward = player.forward();
	let up = player.up();

	Thrust::<NormalVectors> {
		forward: forward * input_flags.forward.into_f32(),
		side_left: up * input_flags.tilt_up.into_f32(),
		tilt_up: forward.cross(up) * input_flags.side_left.into_f32(),
		roll_left: forward * input_flags.roll_left.into_f32(),
		_stage: PhantomData,
	}
}

/// Takes into account the maximum power of each thruster and the current velocity
pub fn get_relative_strengths(
	In(max_thrust): In<Thrust<MaxVelocityMagnitudes>>,
	player: Query<(&Velocity, &Transform), With<MainPlayer>>,
) -> Thrust<RelativeStrength> {
	Thrust::<RelativeStrength> {
		side_left: 1.,
		tilt_up: 1.,
		roll_left: 1.,
		forward: 1.,
		_stage: PhantomData,
	}
}

pub const fn get_max_velocity_vectors() -> Thrust<MaxVelocityMagnitudes> {
	impl MainPlayer {
		const MAX_LINEAR_VELOCITY: f32 = 10.;
		const MAX_ANGULAR_VELOCITY: f32 = 0.3;
	}

	Thrust::<MaxVelocityMagnitudes> {
		side_left: MainPlayer::MAX_ANGULAR_VELOCITY,
		tilt_up: MainPlayer::MAX_ANGULAR_VELOCITY,
		roll_left: MainPlayer::MAX_ANGULAR_VELOCITY,
		forward: MainPlayer::MAX_LINEAR_VELOCITY,
		_stage: PhantomData,
	}
}

pub const fn get_force_factors() -> Thrust<ForceFactors> {
	impl MainPlayer {
		const MOVE_FACTOR: f32 = 5_000_000.;
		const TURN_FACTOR: f32 = 25_000_000.;
	}
	Thrust::<ForceFactors> {
		side_left: MainPlayer::TURN_FACTOR,
		tilt_up: MainPlayer::TURN_FACTOR,
		roll_left: MainPlayer::TURN_FACTOR,
		forward: MainPlayer::MOVE_FACTOR,
		_stage: PhantomData,
	}
}

/// Combines the normal and relative thrusts into the final thrust vectors,
/// and saves the necessary information to various places including in the [MainPlayer] component
pub fn save_thrust_stages(
	In((normal_vectors, relative_strength, max)): In<(
		Thrust<NormalVectors>,
		Thrust<RelativeStrength>,
		Thrust<ForceFactors>,
	)>,
	mut player: Query<&mut MainPlayer, With<MainPlayer>>,
) -> Thrust<FinalVectors> {
	// relative * normals
	impl std::ops::Mul<Thrust<RelativeStrength>> for Thrust<NormalVectors> {
		type Output = Thrust<AlmostFinalVectors>;

		fn mul(self, rhs: Thrust<RelativeStrength>) -> Self::Output {
			Thrust::<AlmostFinalVectors> {
				side_left: self.side_left * rhs.side_left,
				tilt_up: self.tilt_up * rhs.tilt_up,
				roll_left: self.roll_left * rhs.roll_left,
				forward: self.forward * rhs.forward,
				_stage: PhantomData,
			}
		}
	}

	// Almost final * force factor
	impl std::ops::Mul<Thrust<ForceFactors>> for Thrust<AlmostFinalVectors> {
		type Output = Thrust<FinalVectors>;

		fn mul(self, rhs: Thrust<ForceFactors>) -> Self::Output {
			Thrust::<FinalVectors> {
				side_left: self.side_left * rhs.side_left,
				tilt_up: self.tilt_up * rhs.tilt_up,
				roll_left: self.roll_left * rhs.roll_left,
				forward: self.forward * rhs.forward,
				_stage: PhantomData,
			}
		}
	}

	let final_vectors = normal_vectors * relative_strength.clone() * max;

	player.single_mut().thrust = relative_strength;

	final_vectors
}

pub fn apply_thrust(
	In(thrust): In<Thrust<FinalVectors>>,
	mut player: Query<&mut ExternalForce, With<MainPlayer>>,
	time: Res<Time>,
) -> Thrust<FinalVectors> {
	let mut player = player.single_mut();
	let delta = time.delta_seconds_f64() as f32;

	player.force = thrust.forward;

	player.torque = thrust.side_left + thrust.tilt_up + thrust.roll_left;
	player.torque *= delta;

	thrust
}

pub fn manual_get_final_thrust(
	keyboard_input: Res<Input<KeyCode>>,
	player_pos: Query<&Transform, With<MainPlayer>>,
	player_current: Query<(&Velocity, &Transform), With<MainPlayer>>,
	player_save: Query<&mut MainPlayer, With<MainPlayer>>,
) -> Thrust<FinalVectors> {
	let normals = vectorise_input_flags(In(gather_player_movement(keyboard_input)), player_pos);
	let relative = get_relative_strengths(In(get_max_velocity_vectors()), player_current);

	save_thrust_stages(In((normals, relative, get_force_factors())), player_save)
}

fn enact_player_movement(
	mut player: Query<(&mut ExternalForce, &Transform, &Velocity), With<MainPlayer>>,
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	let (mut player, transform, velocity) = player.single_mut();

	// gather inputs

	let mut movement_force = Vec3::ZERO;
	let mut torque = Vec3::ZERO;
	let forward = transform.forward().normalize();
	let up = transform.up().normalize();
	if keyboard_input.pressed(KeyCode::W) {
		// forward
		movement_force += forward;
	}
	if keyboard_input.pressed(KeyCode::S) {
		// backwards
		movement_force -= forward / 2.;
	}
	if keyboard_input.pressed(KeyCode::A) {
		// turn left
		torque += up;
	}
	if keyboard_input.pressed(KeyCode::D) {
		// turn right
		torque -= up;
	}
	if keyboard_input.pressed(KeyCode::Space) {
		// tilt up
		torque += forward.cross(up).normalize();
	}
	if keyboard_input.pressed(KeyCode::ShiftLeft) {
		// tilt down
		torque -= forward.cross(up).normalize();
	}
	if keyboard_input.pressed(KeyCode::Q) {
		// roll left
		torque -= forward;
	}
	if keyboard_input.pressed(KeyCode::E) {
		// roll right
		torque += forward;
	}

	// enact inputs

	if movement_force == Vec3::ZERO {
		player.force = Vec3::ZERO;
	} else {
		player.force =
			movement_force.normalize() * MainPlayer::MOVE_FACTOR * time.delta_seconds_f64() as f32;

		// limit velocity
		let current_velocity = velocity.linvel;
		if current_velocity.length() > MainPlayer::MAX_LINEAR_VELOCITY {
			let forward_factor = player
				.force
				.normalize()
				.dot(current_velocity.normalize())
				.add(1.)
				.div(2.);

			// #[cfg(feature = "debugging")]
			// info!(
			// 	"len = {} Forward factor: {}",
			// 	current_velocity.length(),
			// 	forward_factor
			// );

			player.force *= 1. - forward_factor;
		}
	}
	if torque == Vec3::ZERO {
		player.torque = Vec3::ZERO;
	} else {
		player.torque = torque.normalize() * MainPlayer::TURN_FACTOR * time.delta_seconds_f64() as f32;

		// TODO: fix bug with zero velocity stopping angvel after a while
		// limit angular velocity
		let current_angular_velocity = velocity.angvel;

		if current_angular_velocity.length() > MainPlayer::MAX_ANGULAR_VELOCITY {
			let turn_factor = player
				.torque
				.normalize()
				.dot(current_angular_velocity.normalize())
				.add(1.)
				.div(2.);

			// #[cfg(feature = "debugging")]
			// info!(
			// 	"len = {} angle_factor factor: {}",
			// 	current_angular_velocity.length(),
			// 	turn_factor
			// );

			player.torque *= 1. - turn_factor;
		}
	}
}
