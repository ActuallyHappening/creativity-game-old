use crate::utils::*;

#[allow(private_bounds)]
#[derive(Component, Debug, Default)]
pub struct UiCamera<S: CamSticky>(PhantomData<S>);

assert_impl_all!(UiCamera<BottomLeft>: Send, Sync);

trait CamSticky: Component + std::fmt::Debug + Send + Sync + Default {}

macro_rules! impl_cam_sticky {
		($(pub struct $name:ident;)*) => {
			$(
				#[derive(Component, Debug, Clone, Copy, Default)]
				pub struct $name;
				impl CamSticky for $name {}
			)*
		};
}

impl_cam_sticky!(
	pub struct BottomLeft;
	pub struct TopLeft;
	pub struct TopRight;
	pub struct BottomRight;
);
