use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::{
    core::rigid_body::spawn::RigidbodyBundle, entities::computers::spawn::STANDARD_BODY_FRICTION,
};

pub fn rigidbody_bundle() -> RigidbodyBundle {
    let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
    friction.combine_rule = CoefficientCombineRule::Multiply;

    RigidbodyBundle {
        collider: Collider::cuboid(0.11 * 1.5, 0.1 * 1.5, 0.13 * 1.5),
        collider_transform: Transform::from_translation(Vec3::new(0., 0.087, 0.)),
        collider_friction: friction,
    }
}
