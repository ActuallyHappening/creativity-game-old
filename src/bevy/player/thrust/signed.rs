use crate::utils::*;

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
	pub fn is_zero(&self) -> bool {
		matches!(self, Signed::Zero)
	}

	pub fn is_positive(&self) -> bool {
		matches!(self, Signed::Positive(_))
	}

	/// Access the underlying `T`, panic-ing of [Signed::Zero]
	pub fn unwrap(self) -> T {
		match self {
			Signed::Positive(v) => v,
			Signed::Negative(v) => v,
			Signed::Zero => panic!(
				"Unwrapped a Signed<{:?}> which was Signed::Zero",
				any::type_name::<T>()
			),
		}
	}

	pub fn into_unit(self) -> f32 {
		match self {
			Signed::Positive(_) => 1.,
			Signed::Negative(_) => -1.,
			Signed::Zero => 0.,
		}
	}
}

impl Signed<Vec3> {
	pub fn factor_in(self) -> Vec3 {
		match self {
			Signed::Positive(v) => v,
			Signed::Negative(v) => -v,
			Signed::Zero => Vec3::ZERO,
		}
	}

	pub fn signed_length(self) -> f32 {
		match self {
			Signed::Positive(v) => v.length(),
			Signed::Negative(v) => -v.length(),
			Signed::Zero => 0.,
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
