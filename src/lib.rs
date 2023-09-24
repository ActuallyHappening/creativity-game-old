#![feature(const_trait_impl)]

mod tools;
pub use tools::init_debug_tools;

mod bevy;
pub use bevy::MainPlugin;