use super::*;

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

thrust_stage!(
	/// type = [Option] < bool >
	///
	/// FLAGGED
	///
	/// What keys were pressed, without braking required.
	#[derive(Default)]
	pub struct NonBrakingInputFlags; type = Option<bool>
);

thrust_stage!(
	/// type = [Option] < bool >
	///
	/// FLAGGED - requires more information to flag
	///
	/// What "keys should be pressed" to brake the player
	#[derive(Default)]
	pub struct BreakingReactionFlags; type = Option<bool>
);

thrust_stage!(
	/// type = [Vec3]
	///
	/// UN FLAGGED
	/// Vectors of length 1, used to give information about whereabouts and
	/// rotation of the player
	pub struct BasePositionNormalVectors; type = Vec3
);

thrust_stage!(
	/// FLAGGED
	/// Vectors of length 1, used as a helper stage. signed
	/// This MUST take into account the InputFlags, the 'flagged' part of the name
	/// means when the input flags are None, then the vector is zero [Vec3::ZERO]
	pub struct FlaggedPositionNormalVectors; type = Signed<Vec3>
);

thrust_stage!(
	/// type = [f32]
	///
	/// UN FLAGGED
	///
	/// Used for *braking*
	///
	/// Vectors of maximum, used as a helper stage
	pub struct MaxAllowableVelocityMagnitudes; type = f32
);

thrust_stage!(
	/// type = [f32]
	///
	/// Semi flagged, because taking actual player velocity into account.
	///
	/// Used for UI and for *braking*.
	/// Is public, and used for the HUD.
	///
	/// Represents the factor of current velocity, broken up into separate
	/// **linearly independent** dimensions.
	pub struct RelativeVelocityMagnitudes; type = f32
);

thrust_stage!(
	/// type = [f32]
	///
	/// FLAGGED
	///
	/// UI and processing
	///
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
	pub struct FinalVectors; type = Vec3
);

thrust_stage!(
	/// Just needs to be multiplied by the [ForceFactors] before it is a final thrust type
	pub struct AlmostFinalVectors; type = Vec3
);

impl std::ops::Mul<Thrust<BasePositionNormalVectors>> for Thrust<NonBrakingInputFlags> {
	type Output = Thrust<FlaggedPositionNormalVectors>;

	fn mul(self, base: Thrust<BasePositionNormalVectors>) -> Self::Output {
		Thrust::<FlaggedPositionNormalVectors> {
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

// force factor * constant
impl std::ops::Mul<f32> for Thrust<ForceFactors> {
	type Output = Thrust<ForceFactors>;

	fn mul(self, rhs: f32) -> Self::Output {
		Thrust::<ForceFactors> {
			forward: self.forward * rhs,
			up: self.up * rhs,
			right: self.right * rhs,

			turn_left: self.turn_left * rhs,
			tilt_up: self.tilt_up * rhs,
			roll_left: self.roll_left * rhs,
			
			_stage: PhantomData,
		}
	}
}