use bevy::prelude::{Changed, Query};


use crate::space_core::{components::{entity_updates::EntityUpdates, gi_probe::GIProbe}, structs::network_messages::EntityUpdateData};

pub fn gi_probe_update(
    mut updated_gi_probes: Query<(&GIProbe, &mut EntityUpdates), Changed<GIProbe>>,
) {

    for (gi_probe_component, mut entity_updates_component) in updated_gi_probes.iter_mut() {

        let entity_updates = entity_updates_component.updates
        .get_mut(&".".to_string()).unwrap();

        entity_updates.insert(
            "bias".to_string(),
            EntityUpdateData::Float(gi_probe_component.bias)
        );
        entity_updates.insert(
            "compressed".to_string(),
            EntityUpdateData::Bool(gi_probe_component.compressed)
        );
        entity_updates.insert(
            "dynamic_range".to_string(),
            EntityUpdateData::Int(gi_probe_component.dynamic_range as i64)
        );
        entity_updates.insert(
            "energy".to_string(),
            EntityUpdateData::Float(gi_probe_component.energy)
        );
        entity_updates.insert(
            "interior".to_string(),
            EntityUpdateData::Bool(gi_probe_component.interior)
        );
        entity_updates.insert(
            "normal_bias".to_string(),
            EntityUpdateData::Float(gi_probe_component.normal_bias)
        );
        entity_updates.insert(
            "propagation".to_string(),
            EntityUpdateData::Float(gi_probe_component.propagation)
        );
        entity_updates.insert(
            "subdiv".to_string(),
            EntityUpdateData::Int(gi_probe_component.subdiv as i64)
        );
        entity_updates.insert(
            "extents".to_string(),
            EntityUpdateData::Vec3(gi_probe_component.extents)
        );
    }

}