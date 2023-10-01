use std::ops::RangeInclusive;

use crate::utils::*;

/// Dividing space by radius from the origin
pub enum SpaceRegions {
	FarAway,
	VisibleNotInsidePlayer,
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

#[extension(trait VecExt)]
impl Vec3 {
	fn unchecked_random(range: &impl RangeDefinedRegion) -> Vec3 {
		let mut rng = rand::thread_rng();
		let max = *range.range().end();
		let range = -max..max;
		Vec3::new(
			rng.gen_range(range.clone()),
			rng.gen_range(range.clone()),
			rng.gen_range(range.clone()),
		)
	}
}

pub fn random_pos(range: impl RangeDefinedRegion) -> Vec3 {
	let mut p = Vec3::unchecked_random(&range);
	while p.length() < *range.range().start() {
		p = Vec3::unchecked_random(&range);
	}
	p
}