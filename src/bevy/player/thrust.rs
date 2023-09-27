use std::marker::PhantomData;

use crate::utils::*;

#[derive(Debug, Default)]
pub struct Thrust<S: ThrustStage> {
	// todo: document parity
	side_left: <S as self::ThrustStage>::DimensionType,

	/// Upwards is positive
	tilt_up: <S as self::ThrustStage>::DimensionType,

	roll_right: <S as self::ThrustStage>::DimensionType,

	/// Positive is forward obviously
	forward: <S as self::ThrustStage>::DimensionType,

	_stage: PhantomData<S>,
}

trait ThrustStage {
	type DimensionType: std::fmt::Debug + Default;
}
macro_rules! thrust_stage {
	($(#[$($attrss:tt)*])* $(pub)? struct $name:ident; type = $type:ty) => {
		$(#[$($attrss)*])*

		#[derive(Debug, Default)]
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
	pub struct MaxPowerVectors; type = Vec3
);

thrust_stage!(
	/// Shows how much of the maximum power can be used
	/// Used for animating the player and for relative readings
	pub struct RelativeStrength; type = f32
);

thrust_stage!(
	/// Final result which is applied to physics engine
	/// Used for absolute readings
	pub struct FinalVectors; type = Vec3
);

impl std::ops::Mul<Thrust<RelativeStrength>> for Thrust<NormalVectors> {
	type Output = Thrust<FinalVectors>;

	fn mul(self, rhs: Thrust<RelativeStrength>) -> Self::Output {
		Thrust::<FinalVectors> {
			side_left: self.side_left * rhs.side_left,
			tilt_up: self.tilt_up * rhs.tilt_up,
			roll_right: self.roll_right * rhs.roll_right,
			forward: self.forward * rhs.forward,
			_stage: PhantomData,
		}
	}
}

fn gather_player_movement(keyboard_input: Res<Input<KeyCode>>) -> Thrust<InputFlags> {
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
		roll_right: match (
			keyboard_input.pressed(KeyCode::Q),
			keyboard_input.pressed(KeyCode::E),
		) {
			(true, true) | (false, false) => None,
			(true, false) => Some(true),
			(false, true) => Some(false),
		},
		_stage: PhantomData,
	}
}

// fn enact_player_movement(
// 	mut player: Query<(&mut ExternalForce, &Transform, &Velocity), With<MainPlayer>>,
// 	time: Res<Time>,

// ) {
// 	let (mut player, transform, velocity) = player.single_mut();

// 	// gather inputs

// 	let mut movement_force = Vec3::ZERO;
// 	let mut torque = Vec3::ZERO;
// 	let forward = transform.forward().normalize();
// 	let up = transform.up().normalize();
// 	if keyboard_input.pressed(KeyCode::W) {
// 		// forward
// 		movement_force += forward;
// 	}
// 	if keyboard_input.pressed(KeyCode::S) {
// 		// backwards
// 		movement_force -= forward / 2.;
// 	}
// 	if keyboard_input.pressed(KeyCode::A) {
// 		// turn left
// 		torque += up;
// 	}
// 	if keyboard_input.pressed(KeyCode::D) {
// 		// turn right
// 		torque -= up;
// 	}
// 	if keyboard_input.pressed(KeyCode::Space) {
// 		// tilt up
// 		torque += forward.cross(up).normalize();
// 	}
// 	if keyboard_input.pressed(KeyCode::ShiftLeft) {
// 		// tilt down
// 		torque -= forward.cross(up).normalize();
// 	}
// 	if keyboard_input.pressed(KeyCode::Q) {
// 		// roll left
// 		torque -= forward;
// 	}
// 	if keyboard_input.pressed(KeyCode::E) {
// 		// roll right
// 		torque += forward;
// 	}

// 	// enact inputs

// 	if movement_force == Vec3::ZERO {
// 		player.force = Vec3::ZERO;
// 	} else {
// 		player.force =
// 			movement_force.normalize() * MainPlayer::MOVE_FACTOR * time.delta_seconds_f64() as f32;

// 		// limit velocity
// 		let current_velocity = velocity.linvel;
// 		if current_velocity.length() > MainPlayer::MAX_LINEAR_VELOCITY {
// 			let forward_factor = player
// 				.force
// 				.normalize()
// 				.dot(current_velocity.normalize())
// 				.add(1.)
// 				.div(2.);

// 			// #[cfg(feature = "debugging")]
// 			// info!(
// 			// 	"len = {} Forward factor: {}",
// 			// 	current_velocity.length(),
// 			// 	forward_factor
// 			// );

// 			player.force *= 1. - forward_factor;
// 		}
// 	}
// 	if torque == Vec3::ZERO {
// 		player.torque = Vec3::ZERO;
// 	} else {
// 		player.torque = torque.normalize() * MainPlayer::TURN_FACTOR * time.delta_seconds_f64() as f32;

// 		// TODO: fix bug with zero velocity stopping angvel after a while
// 		// limit angular velocity
// 		let current_angular_velocity = velocity.angvel;

// 		if current_angular_velocity.length() > MainPlayer::MAX_ANGULAR_VELOCITY {
// 			let turn_factor = player
// 				.torque
// 				.normalize()
// 				.dot(current_angular_velocity.normalize())
// 				.add(1.)
// 				.div(2.);

// 			// #[cfg(feature = "debugging")]
// 			// info!(
// 			// 	"len = {} angle_factor factor: {}",
// 			// 	current_angular_velocity.length(),
// 			// 	turn_factor
// 			// );

// 			player.torque *= 1. - turn_factor;
// 		}
// 	}
// }
