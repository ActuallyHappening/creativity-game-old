use crate::utils::*;

#[derive(Component, Debug, Default)]
pub struct UiCamera<S: CamType>(PhantomData<S>);

assert_impl_all!(UiCamera<BottomLeft>: Send, Sync);

#[allow(private_bounds)]
pub trait CamType: Component + std::fmt::Debug + Send + Sync + Default + Sealed {
	fn get_cam_transform(half_width: f32, half_height: f32) -> UVec2;

	/// Just a large number to stop cameras from viewing the same area
	/// When Bevy releases a proper SubApp or Multi-World functionality, that will be used instead
	fn get_non_interfering_offset() -> Vec2;

	fn get_camera_order() -> isize;
}
trait Sealed {}

macro_rules! impl_cam_sticky {
		($(pub struct $name:ident; half_width = $w:literal%, half_height = $h:literal %, offset = ($x:literal, $y:literal), order = $order:literal)*) => {
		// ($(pub struct $name:ident;)*) => {
			$(
				#[derive(Component, Debug, Clone, Copy, Default)]
				pub struct $name;
				impl Sealed for $name {}
				impl CamType for $name {
					fn get_cam_transform(half_width: f32, half_height: f32) -> UVec2 {
						UVec2::new(
							(half_width as f32 * $w as f32 / 100.) as u32,
							(half_height as f32 * $h as f32 / 100.) as u32,
						)
					}

					fn get_non_interfering_offset() -> Vec2 {
						Vec2::new($x as f32, $y as f32)
					}

					fn get_camera_order() -> isize {
						$order
					}
				}
			)*
		};
}

impl_cam_sticky!(
	pub struct BottomLeft; half_width = 100%, half_height = 100%, offset = (0, 0), order = 1
	pub struct TopLeft; half_width = 100%, half_height = -100%, offset = (1000, 0), order = 2
	pub struct BottomRight; half_width = -100%, half_height = 100%, offset = (2000, 0), order = 3
	pub struct TopRight; half_width = -100%, half_height = -100%, offset = (3000, 0), order = 4
);

impl<T: CamType> UiCamera<T> {
	pub fn get_camera_bundle() -> impl Bundle {
		Camera2dBundle {
			camera: Camera {
				order: T::get_camera_order(),
				..default()
			},
			camera_2d: Camera2d {
				clear_color: ClearColorConfig::None,
			},
			..default()
		}
		.insert(Self::default())
		.not_pickable()
	}
}
