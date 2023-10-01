use std::ops::RangeInclusive;

use crate::utils::*;

/// Dividing space by radius from the origin
pub enum SpaceRegions {
	FarAway,
	VisibleNotInsidePlayer,
}

pub enum VelocityRanges {
	Slow,
}

#[extension(pub trait RangeDefinedRegion)]
impl SpaceRegions {
	fn range(&self) -> RangeInclusive<f32> {
		match self {
			Self::FarAway => 20_000. ..=400_000.,
			Self::VisibleNotInsidePlayer => 50. ..=1_000.,
		}
	}
}

impl VelocityRanges {
	/// 0: linvel length ranges, 1: angvel length ranges
	fn ranges(&self) -> (RangeInclusive<f32>, RangeInclusive<f32>) {
		(0. ..=3., 0. ..=0.05)
	}
}

#[extension(trait Vec3Ext)]
impl Vec3 {
	/// Radius in normal units
	/// Theta in radians
	/// phi in radians
	fn from_polar(radius: f32, theta: f32, phi: f32) -> Self {
		Self {
			x: radius * theta.sin() * phi.cos(),
			y: radius * theta.sin() * phi.sin(),
			z: radius * theta.cos(),
		}
	}
}

fn checked_random(range: RangeInclusive<f32>) -> Vec3 {
	let (lower, _upper) = (*range.start(), *range.end());
	assert!(lower >= 0., "Lower bound passed to random util func must be greater than 0, because radius must be positive.");
	let mut rng = rand::thread_rng();

	Vec3::from_polar(rng.gen_range(range), rng.gen_range(0. ..=TAU), rng.gen_range(0. ..=TAU))
}

pub fn random_pos(range: SpaceRegions) -> Vec3 {
	checked_random(range.range())
}

pub fn random_velocity(range: VelocityRanges) -> Velocity {
	Velocity {
		linvel: checked_random(range.ranges().0),
		angvel: checked_random(range.ranges().1),
	}
}
