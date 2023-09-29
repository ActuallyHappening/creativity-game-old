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
