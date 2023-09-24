use bevy::prelude::*;
use bevy_mod_picking::{
	prelude::{RaycastPickCamera, RaycastPickTarget, DefaultHighlightingPlugin, DebugPickingPlugin},
	PickableBundle, DefaultPickingPlugins,
};

use super::{camera::{CameraPlugin, MainCamera}, utils::LIGHT_HEIGHT};

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Msaa::default()).add_systems(Startup, setup).add_plugins(
			DefaultPickingPlugins
				.build()
				.disable::<DefaultHighlightingPlugin>()
				.disable::<DebugPickingPlugin>(),
		);
	}
}

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// cam
	commands.spawn(
		CameraPlugin::default(),
	);

	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 500000.0,
			range: 25000.,
			// shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(0., LIGHT_HEIGHT, 0.),
		..default()
	});

	// ground plane
	commands.spawn((
		PbrBundle {
			mesh: meshes.add(shape::Plane::from_size(50000.0).into()),
			material: materials.add(Color::SILVER.into()),
			// transform to be behind, xy plane
			transform: Transform::from_xyz(0., 0., 0.),
			..default()
		},
		PickableBundle::default(),    // Makes the entity pickable
		RaycastPickTarget::default(), // Marker for the `bevy_picking_raycast` backend
	));
}
