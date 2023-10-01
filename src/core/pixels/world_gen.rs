use crate::utils::*;

#[derive(Debug, EnumIs, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(name(WorldObjectTypes))]
pub enum WorldObjectType {
	Asteroid { approx_radius: NonZeroU8 },
}
