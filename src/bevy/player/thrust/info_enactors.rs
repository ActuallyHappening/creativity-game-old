use super::*;

pub fn apply_thrust(
	In(thrust): In<Thrust<FinalVectors>>,
	mut player_physics: Query<&mut ExternalForce, With<MainPlayer>>,
	time: Res<Time>,
) -> Thrust<FinalVectors> {
	let mut player = player_physics.single_mut();
	let delta = time.delta_seconds_f64() as f32;

	impl MainPlayer {
		const MAX_TOTAL_ANGULAR_FORCE: f32 = 10_000_000.;
		const MAX_TOTAL_LINEAR_FORCE: f32 = 10_000_000.;
	}

	player.force = (thrust.forward + thrust.up + thrust.right)
		.clamp_length(0., MainPlayer::MAX_TOTAL_LINEAR_FORCE);
	player.force *= delta;

	player.torque = (thrust.turn_left + thrust.tilt_up + thrust.roll_left)
		.clamp_length(0., MainPlayer::MAX_TOTAL_ANGULAR_FORCE);
	player.torque *= delta;

	// info!("Thrust: (ang len = {})");

	thrust
}

pub fn trigger_player_thruster_particles(
	player: Query<&MainPlayer>,
	mut particles: Query<(&mut EffectSpawner, &Thruster)>,
) {
	let MainPlayer {
		relative_strength: thrust,
		..
	} = player.single();

	impl ThrusterFlags {
		/// If flags match, add relative strength, else add nothing
		fn degree_of_match(&self, actual: &Thrust<RelativeStrength>) -> f32 {
			let flags = self;
			let mut counter = 0.;

			let forward = Signed::from(actual.forward);
			if flags
				.forward_back
				.is_some_and(|f| f == forward.is_positive())
			{
				counter += forward.into_unit().abs();
			}

			let up = Signed::from(actual.up);
			if flags.up_down.is_some_and(|f| f == up.is_positive()) {
				counter += up.into_unit().abs();
			}

			let right = Signed::from(actual.right);
			if flags.right_left.is_some_and(|f| f == right.is_positive()) {
				counter += right.into_unit().abs();
			}

			let turn_left = Signed::from(actual.turn_left);
			if flags
				.turn_left
				.is_some_and(|f| f == turn_left.is_positive())
			{
				counter += turn_left.into_unit().abs();
			}

			let tilt_up = Signed::from(actual.tilt_up);
			if flags.tilt_up.is_some_and(|f| f == tilt_up.is_positive()) {
				counter += tilt_up.into_unit().abs();
			}

			let roll_left = Signed::from(actual.roll_left);
			if flags
				.roll_left
				.is_some_and(|f| f == roll_left.is_positive())
			{
				counter += roll_left.into_unit().abs();
			}

			counter
		}
	}

	for (mut spawner, Thruster { flags, .. }) in particles.iter_mut() {
		// todo: show gradient of particles, change acceleration / lifetime?

		let degree = flags.degree_of_match(thrust);

		if degree > 0. {
			spawner.set_active(true);
		} else {
			spawner.set_active(false);
		}
	}
}
