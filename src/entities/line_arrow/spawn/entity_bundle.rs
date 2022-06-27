use std::collections::BTreeMap;

use bevy_transform::prelude::Transform;

use crate::core::{
    entity::{
        resources::SpawnData,
        spawn::{BaseEntityBundle, BaseEntitySummonable, NoData},
    },
    examinable::components::{Examinable, RichName},
};

use super::{LineArrowSummoner, LINE_ARROW_ENTITY_NAME};

pub fn get_default_transform() -> Transform {
    Transform::identity()
}

impl BaseEntitySummonable<NoData> for LineArrowSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> BaseEntityBundle {
        let template_examine_text =
            "A holographic arrow without additional data points.".to_string();
        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, template_examine_text);

        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "arrow".to_string(),
                    n: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_name: LINE_ARROW_ENTITY_NAME.to_string(),
            ..Default::default()
        }
    }
}
