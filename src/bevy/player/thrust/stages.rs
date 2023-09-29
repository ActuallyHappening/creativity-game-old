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
	pub struct BasePositionNormalVectors; type = Vec3
);

thrust_stage!(
	/// FLAGGED
	/// Vectors of length 1, used as a helper stage. signed
	/// This MUST take into account the InputFlags, the 'flagged' part of the name
	/// means when the input flags are None, then the vector is zero [Vec3::ZERO]
	pub struct FlaggedNormalVectors; type = Signed<Vec3>
);

thrust_stage!(
	/// type = [f32]
	/// UN FLAGGED
	/// Vectors of maximum, used as a helper stage
	pub struct MaxVelocityMagnitudes; type = f32
);

thrust_stage!(
	/// type = [f32]
	/// Semi flagged, because taking actual player velocity into account.
	/// Is public, and used for the HUD
	pub struct RelativeVelocityMagnitudes; type = f32
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
