use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;
use bevy_mod_picking::{
	prelude::{DebugPickingPlugin, DefaultHighlightingPlugin},
	DefaultPickingPlugins,
};
#[cfg(feature = "dev")]
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

mod stars;
use stars::*;
// mod testparticles;

use super::camera::CameraPlugin;
use crate::utils::*;

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(Msaa::default())
			.add_systems(Startup, (setup, spawn_initial_world))
			.add_plugins(
				DefaultPickingPlugins
					.build()
					.disable::<DefaultHighlightingPlugin>()
					.disable::<DebugPickingPlugin>(),
			)
			.add_systems(Update, blink_stars);

		// app.add_systems(Update, test_activate_particles);

		#[cfg(feature = "dev")]
		app.add_plugins((
			ScreenFrameDiagnosticsPlugin,
			ScreenDiagnosticsPlugin::default(),
		));

		app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
		app.add_systems(
			Startup,
			|mut physics_config: ResMut<RapierConfiguration>| {
				physics_config.gravity = Vec3::ZERO;
			},
		);

		#[cfg(feature = "hanabi_particles")]
		app.add_plugins(HanabiPlugin);

		// #[cfg(feature = "debugging")]
		// app.add_plugins(RapierDebugRenderPlugin::default());

		#[cfg(feature = "dev")]
		#[cfg(feature = "debugging")]
		app
			.add_plugins(bevy_editor_pls::prelude::EditorPlugin::default())
			.insert_resource(editor_controls());
	}
}

pub fn setup(mut commands: Commands, mut mma: MMA) {
	// cam
	commands.spawn(CameraPlugin::default());

	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 5000000.0,
			range: 25000.,
			// shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(0., LIGHT_HEIGHT, 0.),
		..default()
	});

	// stars
	for _ in 0..5_000 {
		stars::spawn_random_star(&mut commands, &mut mma);
		// tracing::info!("Spawned star");
	}
}

#[cfg(feature = "dev")]
fn editor_controls() -> bevy_editor_pls::controls::EditorControls {
	use bevy_editor_pls::controls;
	use bevy_editor_pls::controls::EditorControls;

	let mut editor_controls = EditorControls::default_bindings();
	editor_controls.unbind(controls::Action::PlayPauseEditor);

	editor_controls.insert(
		controls::Action::PlayPauseEditor,
		controls::Binding {
			input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Backslash)),
			conditions: vec![controls::BindingCondition::ListeningForText(false)],
		},
	);

	editor_controls
}
