use std::any;

use crate::utils::*;

use super::MainPlayer;

#[derive(Debug, Clone)]
pub struct Thrust<S: ThrustStage> {
	/// Positive is forward obviously
	forward: <S as self::ThrustStage>::DimensionType,

	up: <S as self::ThrustStage>::DimensionType,

	right: <S as self::ThrustStage>::DimensionType,

	/// Left is positive
	turn_left: <S as self::ThrustStage>::DimensionType,

	/// Upwards is positive
	tilt_up: <S as self::ThrustStage>::DimensionType,

	/// Right is positive
	roll_left: <S as self::ThrustStage>::DimensionType,

	_stage: PhantomData<S>,
}

pub trait ThrustStage {
	type DimensionType: std::fmt::Debug + Clone;
}
macro_rules! thrust_stage {
	($(#[$($attrss:tt)*])* $(pub)? struct $name:ident; type = $type:ty) => {
		#[doc = concat!("Dimension type = ", stringify!($type), "\n")]
		$(#[$($attrss)*])*

		#[derive(Debug, Clone,)]
		pub struct $name;
		impl ThrustStage for $name {
			type DimensionType = $type;
		}
	};
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Signed<T>
where
	T: Default + Clone + Copy + Default,
{
	Positive(T),
	Negative(T),

	#[default]
	Zero,
}

impl<T: Default + Clone + Copy + Default> Signed<T> {
	fn is_zero(&self) -> bool {
		matches!(self, Signed::Zero)
	}

	fn is_positive(&self) -> bool {
		matches!(self, Signed::Positive(_))
	}

	/// Access the underlying `T`, panic-ing of [Signed::Zero]
	fn unwrap(self) -> T {
		match self {
			Signed::Positive(v) => v,
			Signed::Negative(v) => v,
			Signed::Zero => panic!(
				"Unwrapped a Signed<{:?}> which was Signed::Zero",
				any::type_name::<T>()
			),
		}
	}

	fn into_unit(self) -> f32 {
		match self {
			Signed::Positive(_) => 1.,
			Signed::Negative(_) => -1.,
			Signed::Zero => 0.,
		}
	}
}

impl Signed<Vec3> {
	fn factor_in(self) -> Vec3 {
		match self {
			Signed::Positive(v) => v,
			Signed::Negative(v) => -v,
			Signed::Zero => Vec3::ZERO,
		}
	}
}
impl From<f32> for Signed<f32> {
	fn from(value: f32) -> Self {
		if value > 0. {
			Signed::Positive(value)
		} else if value < 0. {
			Signed::Negative(value)
		} else {
			Signed::Zero
		}
	}
}

thrust_stage!(
	/// FLAGGED
	/// What keys were pressed Option of bool for each dimension
	#[derive(Default)]
	pub struct InputFlags; type = Option<bool>
);

thrust_stage!(
	/// type = [Vec3]
	///
	/// UN FLAGGED
	/// Vectors of length 1, used to give information about whereabouts and
	/// rotation of the player
	pub struct BaseNormalVectors; type = Vec3
);

thrust_stage!(
	/// FLAGGED
	/// Vectors of length 1, used as a helper stage. signed
	/// This MUST take into account the InputFlags, the 'flagged' part of the name
	/// means when the input flags are None, then the vector is zero [Vec3::ZERO]
	pub struct FlaggedNormalVectors; type = Signed<Vec3>
);

thrust_stage!(
	/// UN FLAGGED
	/// Vectors of maximum, used as a helper stage
	pub struct MaxVelocityMagnitudes; type = f32
);

thrust_stage!(
	/// type = [f32]
	///
	/// FLAGGED
	/// Shows how much of the maximum power can be used
	/// Used for animating the player and for relative readings
	#[derive(Default)]
	pub struct RelativeStrength; type = f32
);

thrust_stage!(
	/// UN FLAGGED
	/// Factors multiplied into the final thrust vectors
	pub struct ForceFactors; type = f32
);

thrust_stage!(
	/// Final result which is applied to physics engine
	/// Also used for absolute readings
	pub struct FinalVectors; type = Vec3
);

thrust_stage!(
	/// Just needs to be multiplied by the [ForceFactors] before it is a final thrust type
	pub struct AlmostFinalVectors; type = Vec3
);

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

// #[bevycheck::system]
pub fn gather_input_flags(keyboard_input: Res<Input<KeyCode>>) -> Thrust<InputFlags> {
	match keyboard_input.pressed(KeyCode::Space) {
		false => Thrust::<InputFlags> {
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
	}
}

pub fn get_base_normal_vectors(
	player_transform: Query<&Transform, With<MainPlayer>>,
) -> Thrust<BaseNormalVectors> {
	let player = match player_transform.get_single() {
		Ok(player) => player,
		Err(e) => panic!("No player found: {:?}", e),
	};

	let forward = player.forward();
	let up = player.up();

	// the meat of the system
	Thrust::<BaseNormalVectors> {
		forward,
		up,
		right: forward.cross(up),

		turn_left: up,
		tilt_up: forward.cross(up),
		roll_left: forward,
		_stage: PhantomData,
	}
}

// #[bevycheck::system]
/// Makes normal vectors which were not selected by user to be [Vec3::ZERO].
pub fn flag_normal_vectors(
	In((input_flags, base)): In<(Thrust<InputFlags>, Thrust<BaseNormalVectors>)>,
) -> Thrust<FlaggedNormalVectors> {
	#[extension(trait OptionExt)]
	impl Option<bool> {
		fn wrap_signed(self, wrapped: Vec3) -> Signed<Vec3> {
			match self {
				Some(true) => Signed::Positive(wrapped),
				Some(false) => Signed::Negative(wrapped),
				None => Signed::Zero,
			}
		}
	}

	impl std::ops::Mul<Thrust<BaseNormalVectors>> for Thrust<InputFlags> {
		type Output = Thrust<FlaggedNormalVectors>;

		fn mul(self, base: Thrust<BaseNormalVectors>) -> Self::Output {
			Thrust::<FlaggedNormalVectors> {
				forward: self.forward.wrap_signed(base.forward),
				right: self.right.wrap_signed(base.right),
				up: self.up.wrap_signed(base.up),

				roll_left: self.roll_left.wrap_signed(base.roll_left),
				turn_left: self.turn_left.wrap_signed(base.turn_left),
				tilt_up: self.tilt_up.wrap_signed(base.tilt_up),

				_stage: PhantomData,
			}
		}
	}

	input_flags * base
}

/// Takes into account the maximum power of each thruster and the current velocity
pub fn get_relative_strengths(
	In((aimed, max)): In<(Thrust<FlaggedNormalVectors>, Thrust<MaxVelocityMagnitudes>)>,
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
			let factor_slowing_down = 1.
				- aimed_vec
					.normalize()
					.dot(current.normalize())
					.add(1.)
					.div(2.);

			let percentage_of_max_allowed_velocity = (current.length() / max).clamp(0., 1.);

			if percentage_of_max_allowed_velocity > 0.9 {
				factor_slowing_down * aimed.into_unit()
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

pub const fn max_velocity_magnitudes() -> Thrust<MaxVelocityMagnitudes> {
	impl MainPlayer {
		const MAX_LINEAR_VELOCITY: f32 = 10.;
		const MAX_ANGULAR_VELOCITY: f32 = 0.2;
	}

	Thrust::<MaxVelocityMagnitudes> {
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

/// Combines the normal and relative thrusts into the final thrust vectors,
/// and saves the necessary information to various places including in the [MainPlayer] component
pub fn save_thrust_stages(
	In((relative_strength, normal_vectors, max)): In<(
		Thrust<RelativeStrength>,
		Thrust<BaseNormalVectors>,
		Thrust<ForceFactors>,
	)>,
	mut player_data: Query<&mut MainPlayer, With<MainPlayer>>,
) -> Thrust<FinalVectors> {
	// relative (F) * normals (U) = almost final (FLAGGED)
	impl std::ops::Mul<Thrust<RelativeStrength>> for Thrust<BaseNormalVectors> {
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

pub fn apply_thrust(
	In(thrust): In<Thrust<FinalVectors>>,
	mut player_physics: Query<&mut ExternalForce, With<MainPlayer>>,
	time: Res<Time>,
) -> Thrust<FinalVectors> {
	let mut player = player_physics.single_mut();
	let delta = time.delta_seconds_f64() as f32;

	impl MainPlayer {
		const MAX_TOTAL_ANGULAR_FORCE: f32 = 10_000_000.;
		const MAX_TOTAL_LINEAR_FORCE: f32 = 10_000_000.;
	}

	player.force = (thrust.forward + thrust.up + thrust.right)
		.clamp_length(0., MainPlayer::MAX_TOTAL_LINEAR_FORCE);
	player.force *= delta;

	player.torque = (thrust.turn_left + thrust.tilt_up + thrust.roll_left)
		.clamp_length(0., MainPlayer::MAX_TOTAL_ANGULAR_FORCE);
	player.torque *= delta;

	// info!("Thrust: (ang len = {})");

	thrust
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
		gather_input_flags(keyboard_input),
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

pub fn trigger_player_thruster_particles(
	player: Query<&MainPlayer>,
	mut particles: Query<(&mut EffectSpawner, &Thruster)>,
) {
	let MainPlayer {
		relative_thrust: thrust,
	} = player.single();

	impl ThrustFlags {
		/// If flags match, add relative strength, else add nothing
		fn degree_of_match(&self, actual: &Thrust<RelativeStrength>) -> f32 {
			let flags = self;
			let mut counter = 0.;

			let forward = Signed::from(actual.forward);
			if flags
				.forward_back
				.is_some_and(|f| f == forward.is_positive())
			{
				counter += forward.into_unit().abs();
			}

			let up = Signed::from(actual.up);
			if flags.up_down.is_some_and(|f| f == up.is_positive()) {
				counter += up.into_unit().abs();
			}

			let right = Signed::from(actual.right);
			if flags.right_left.is_some_and(|f| f == right.is_positive()) {
				counter += right.into_unit().abs();
			}

			let turn_left = Signed::from(actual.turn_left);
			if flags.turn_left.is_some_and(|f| f == turn_left.is_positive()) {
				counter += turn_left.into_unit().abs();
			}

			let tilt_up = Signed::from(actual.tilt_up);
			if flags.tilt_up.is_some_and(|f| f == tilt_up.is_positive()) {
				counter += tilt_up.into_unit().abs();
			}

			let roll_left = Signed::from(actual.roll_left);
			if flags.roll_left.is_some_and(|f| f == roll_left.is_positive()) {
				counter += roll_left.into_unit().abs();
			}

			counter
		}
	}

	for (mut spawner, Thruster { flags, .. }) in particles.iter_mut() {
		// todo: show gradient of particles, change acceleration / lifetime?

		let degree = flags.degree_of_match(thrust);
		debug!("Degree of match: {}", degree);
		if degree > 0. {
			spawner.set_active(true);
		} else {
			spawner.set_active(false);
		}
	}
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
