use bevy_internal::prelude::{Component, Transform};


#[derive(Copy, Clone, Component)]
pub struct StaticTransform {
    pub transform: Transform,
}
