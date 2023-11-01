use super::*;

/// Used on the thruster to show when it should be displaying particles
/// relative to player movement inputs
#[derive(Debug, Clone, Component, Default, Builder, PartialEq, Serialize, Deserialize)]
#[builder(setter(into, strip_option,))]
pub struct ThrusterFlags {
	#[builder(default)]
	pub forward_back: Option<bool>,
	#[builder(default)]
	pub up_down: Option<bool>,
	#[builder(default)]
	pub right_left: Option<bool>,

	#[builder(default)]
	pub turn_right: Option<bool>,
	#[builder(default)]
	pub tilt_up: Option<bool>,
	#[builder(default)]
	pub roll_right: Option<bool>,
}

impl ThrusterFlags {
	pub fn builder() -> ThrusterFlagsBuilder {
		ThrusterFlagsBuilder::default()
	}
}

impl Reflection for ThrusterFlags {
	fn reflect_horizontally(mut self) -> Self {
		self.right_left = self.right_left.map(|v| !v);
		self.turn_right = self.turn_right.map(|v| !v);
		self.roll_right = self.roll_right.map(|v| !v);
		self
	}

	fn reflect_vertically(mut self) -> Self {
		self.up_down = self.up_down.map(|v| !v);
		self.tilt_up = self.tilt_up.map(|v| !v);
		self.roll_right = self.roll_right.map(|v| !v);
		self
	}
}