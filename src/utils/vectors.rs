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
			Self::VisibleNotInsidePlayer => 50. ..=5_000.,
		}
	}
}

impl VelocityRanges {
	/// 0: linvel length ranges, 1: angvel length ranges
	fn ranges(&self) -> (RangeInclusive<f32>, RangeInclusive<f32>) {
		(0. ..=3., 0. ..=0.05)
	}
}

fn unchecked_random(upper: f32) -> Vec3 {
	let mut rng = rand::thread_rng();
	let range = -upper..upper;
	Vec3::new(
		rng.gen_range(range.clone()),
		rng.gen_range(range.clone()),
		rng.gen_range(range.clone()),
	)
}

fn checked_random(range: RangeInclusive<f32>) -> Vec3 {
	let (lower, upper) = (*range.start(), *range.end());
	assert!(lower > 0., "Lower bound passed to random util func must be greater than 0, because radius must be positive.");

	let mut p = unchecked_random(upper);
	while p.length() < lower {
		p = unchecked_random(upper);
	}
	p
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
