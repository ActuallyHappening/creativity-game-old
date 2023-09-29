use super::*;

#[derive(Debug, Component, Default, Clone)]
pub struct BrakingInfo(pub bool, pub Thrust<NonBrakingInputFlags>);
