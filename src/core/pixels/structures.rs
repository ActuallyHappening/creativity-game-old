use super::Pixel;
use crate::utils::*;

pub struct Structure {
	parts: Vec<StructurePart>,
}

pub enum StructurePart {
	Pixel(Pixel),
	Thruster(Thruster),
}

pub struct Thruster {
	facing: Vec3,
}
