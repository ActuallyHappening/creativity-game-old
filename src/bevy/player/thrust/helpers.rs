use crate::utils::*;

#[extension(pub trait Vec3Ext)]
impl Vec3 {
	/// Returns a number between [0, 1] where 0 is no correlation and 1 is perfect correlation
	fn factor_towards(&self, aimed: &Vec3) -> f32 {
		self.normalize().dot(aimed.normalize()).add(1.).div(2.)
	}

	/// Returns a vector which is the projection of self onto aimed.
	/// Amount of `self` in `aimed`
	fn vector_project(&self, aimed: &Vec3) -> Signed<Vec3> {
		let projected = *aimed * self.dot(*aimed) / aimed.length_squared();
		if self.dot(*aimed) > 0. {
			Signed::Positive(projected)
		} else {
			Signed::Negative(projected)
		}
	}
}

#[extension(pub trait OptionExt)]
impl Option<bool> {
	fn wrap_signed(self, wrapped: Vec3) -> Signed<Vec3> {
		match self {
			Some(true) => Signed::Positive(wrapped),
			Some(false) => Signed::Negative(wrapped),
			None => Signed::Zero,
		}
	}
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
	pub fn is_zero(&self) -> bool {
		matches!(self, Signed::Zero)
	}

	pub fn is_positive(&self) -> bool {
		matches!(self, Signed::Positive(_))
	}

	/// Access the underlying `T`, panic-ing of [Signed::Zero]
	// pub fn unwrap(self) -> T {
	// 	match self {
	// 		Signed::Positive(v) => v,
	// 		Signed::Negative(v) => v,
	// 		Signed::Zero => panic!(
	// 			"Unwrapped a Signed<{:?}> which was Signed::Zero",
	// 			any::type_name::<T>()
	// 		),
	// 	}
	// }

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
