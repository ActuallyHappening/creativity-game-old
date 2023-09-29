use crate::{bevy::camera::MainCamera, utils::*};

pub struct UiPlugin;
impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, setup_ui)
			.add_systems(Update, update_ui);
	}
}

mod camtype;
pub use camtype::*;

fn setup_ui(mut commands: Commands, mut mma: MM2) {
	commands.spawn(
		Camera2dBundle {
			camera: Camera {
				order: 1,
				..default()
			},
			camera_2d: Camera2d {
				clear_color: ClearColorConfig::None,
			},
			..default()
		}
		.insert(UiCamera::<BottomLeft>::default())
		.not_pickable(),
	);

	// Circle
	commands.spawn(MaterialMesh2dBundle {
		mesh: mma.meshs.add(shape::Circle::new(50.).into()).into(),
		material: mma.mats.add(ColorMaterial::from(Color::PURPLE)),
		transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
		..default()
	});
}

fn update_ui() {}

// struct ItemPreview;
// impl ItemPreview {
// 	const WIDTH_PERCENT: u32 = 20;
// 	const MARGIN: u32 = 20;
// 	const EDGE_OFFSET: u32 = 0;
// }

// fn update_ui(
// 	windows: Query<&Window>,
// 	mut resize_events: EventReader<WindowResized>,
// 	mut cam: Query<&mut Camera, (With<UiCamera>, Without<MainCamera>)>,
// ) {
// 	for resize_event in resize_events.iter() {
// 		let window = windows.get(resize_event.window).unwrap();
// 		let mut preview_cam = cam.single_mut();

// 		let width = window.resolution.physical_width();
// 		let _height = window.resolution.physical_height();
// 		let preview_width = ((ItemPreview::WIDTH_PERCENT as f32 / 100.) * width as f32).round() as u32
// 			- ItemPreview::MARGIN * 2;
// 		let top_left_x = width - preview_width - ItemPreview::MARGIN - ItemPreview::EDGE_OFFSET;
// 		let top_left_y = ItemPreview::MARGIN + ItemPreview::EDGE_OFFSET;

// 		preview_cam.viewport = Some(Viewport {
// 			physical_position: UVec2::new(top_left_x, top_left_y),
// 			physical_size: UVec2::new(preview_width, preview_width),
// 			..default()
// 		});
// 	}
// }
