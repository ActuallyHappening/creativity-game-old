use std::ops::RangeInclusive;

use bevy::prelude::Color;
use static_assertions::assert_obj_safe;

/// Macro for implementing `IsResource` for a type
macro_rules! impl_pixel_type {
	($type:ty {}) => {};

	($type:ty {
		col = $col:expr,

		$($tail:tt)*
	}) => {
		impl $crate::bevy::world::resources::traits::IsPixel for $type {
			fn get_primary_colour(&self) -> bevy::prelude::Color {
				$col
			}
		}

		impl_pixel_type!($type { $($tail)* });
	};

	($type:ty {
		res = { cap = { range = $range:expr }},

		$($tail:tt)*
	}) => {
		impl $crate::bevy::world::resources::traits::IsResource for $type {
			fn capacity_range() -> std::ops::RangeInclusive<u32> {
				$range
			}

			fn get_capacity(&self) -> u32 {
				self.capacity
			}
		}

		impl_pixel_type!($type { $($tail)* });
	};

	($type:ty {
		natural = { freq = $freq:expr, random = $random:expr },

		$($tail:tt)*
	}) => {
		impl $crate::bevy::world::resources::traits::NaturallyOccurring for $type {
			const FREQUENCY: u8 = $freq;

			fn new_random() -> Self {
				$random
			}
		}

		impl_pixel_type!($type { $($tail)* });
	};
}

pub(crate) use impl_pixel_type;

pub trait IsPixel {
	fn get_primary_colour(&self) -> Color;
}
assert_obj_safe!(IsPixel);

/// Whether the pixel can be collected and / or used to build things
pub trait IsResource: IsPixel {
	fn capacity_range() -> RangeInclusive<u32>;
	fn get_capacity(&self) -> u32;
}
// assert_obj_safe!(IsResource);

pub trait NaturallyOccurring: IsPixel {
	/// Weighting used to calculate the frequency of this pixel spawning naturally
	const FREQUENCY: u8;

	fn new_random() -> Self;
}
// assert_obj_safe!(NaturallyOccurring);
