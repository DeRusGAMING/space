use bevy::prelude::{Changed, Query};
use entity::senser::{Senser, SensingAbility};
use pawn::pawn::{DataLink, DataLinkType};

/// Sensing ability allows players to obtain atmospherics data of a tile by examining it.

pub(crate) fn atmospherics_sensing_ability(
    mut data_linked: Query<(&DataLink, &mut Senser), Changed<DataLink>>,
) {
    for (data_link_component, mut senser_component) in data_linked.iter_mut() {
        if data_link_component
            .links
            .contains(&DataLinkType::FullAtmospherics)
            && senser_component
                .sensing_abilities
                .contains(&SensingAbility::AtmosphericsSensor)
                == false
        {
            senser_component
                .sensing_abilities
                .push(SensingAbility::AtmosphericsSensor);
        } else if data_link_component
            .links
            .contains(&DataLinkType::FullAtmospherics)
            == false
            && senser_component
                .sensing_abilities
                .contains(&SensingAbility::AtmosphericsSensor)
        {
            let index = senser_component
                .sensing_abilities
                .iter()
                .position(|r| r == &SensingAbility::AtmosphericsSensor)
                .unwrap();

            senser_component.sensing_abilities.remove(index);
        }
    }
}
