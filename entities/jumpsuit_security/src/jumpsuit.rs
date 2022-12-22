use bevy::prelude::Component;

/// The component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Jumpsuit;
#[cfg(feature = "server")]
pub const JUMPSUIT_SECURITY_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "jumpsuitSecurity");
use const_format::concatcp;
use entity::meta::SF_CONTENT_PREFIX;
