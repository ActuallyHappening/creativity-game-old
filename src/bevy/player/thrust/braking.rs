use super::*;

#[derive(Debug, Component, Default)]
pub struct BrakingInfo(pub bool, pub Thrust<NonBrakingInputFlags>);

