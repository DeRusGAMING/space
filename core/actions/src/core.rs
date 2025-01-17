use std::collections::HashMap;

use bevy::prelude::Resource;
use bevy::prelude::{warn, Component, Entity, EventReader, Query, Res, ResMut};

/// Resource with a list of actions being built this frame.

#[derive(Default, Resource)]
pub struct BuildingActions {
    pub list: Vec<BuildingAction>,
}
/// Resource with requests to execute actions which will go through prerequisite checking this frame.

#[derive(Default, Resource)]
pub struct ActionRequests {
    pub list: HashMap<u64, ActionRequest>,
}

/// A request to execute a request.

pub struct ActionRequest {
    /// Action identifier.
    id: String,
}

impl ActionRequest {
    /// Get action identifier.
    pub fn get_id(&self) -> &str {
        &self.id
    }
    /// Create from action identifier.
    pub fn from_id(new_id: String) -> Self {
        Self { id: new_id }
    }
    /// Set action identifier.
    pub fn set_id(&mut self, new_id: String) {
        self.id = new_id;
    }
}
/// A request to build a list of available actions.
pub struct BuildingAction {
    /// Available to-be-approved actions
    pub actions: Vec<ActionData>,
    /// Build action request identifier.
    pub incremented_i: u64,
    /// The entity which we request action data for.
    pub action_taker: Entity,
    pub action_taker_item: Option<Entity>,
    /// The entity targetted in the action.
    pub target_entity_option: Option<Entity>,
    /// The ship cell targetted in the action.
    pub target_cell_option: Option<TargetCell>,
}

/// Data related to an individual action.
pub struct ActionData {
    /// The action.
    pub data: Action,
    /// Whether the action is approved or not by a prerequisite checker.
    pub approved: Option<bool>,
}

impl ActionData {
    /// Approve action, typically performed by prerequisite checks.
    pub fn approve(&mut self) {
        match self.approved {
            Some(_) => {}
            None => {
                self.approved = Some(true);
            }
        }
    }
    /// Do not approve action, typically performed by prerequisite checks.
    pub fn do_not_approve(&mut self) {
        self.approved = Some(false);
    }
    /// Check if action is approved.
    pub fn is_approved(&self) -> bool {
        match self.approved {
            Some(_) => {
                return true;
            }
            None => {}
        }
        return false;
    }
}
use networking::server::HandleToEntity;
use resources::grid::TargetCell;

use crate::networking::NetAction;
use networking::server::OutgoingReliableServerMessage;

use bevy::prelude::EventWriter;

use crate::net::ActionsServerMessage;

/// Send lists of approved actions back to player.
pub(crate) fn list_action_data_finalizer(
    building_actions: Res<BuildingActions>,
    handle_to_entity: Res<HandleToEntity>,
    action_data_requests: Res<ListActionDataRequests>,
    mut server: EventWriter<OutgoingReliableServerMessage<ActionsServerMessage>>,
) {
    for action_data in building_actions.list.iter() {
        let action_data_request;
        match action_data_requests.list.get(&action_data.incremented_i) {
            Some(d) => {
                action_data_request = d;
            }
            None => {
                continue;
            }
        }
        let mut net_action_datas = vec![];
        let mut handle = None;

        for action in action_data.actions.iter() {
            if action.is_approved() {
                match handle_to_entity.inv_map.get(&action_data.action_taker) {
                    Some(h) => {
                        handle = Some(*h);
                        let mut cell_option = None;

                        match action_data.target_cell_option.clone() {
                            Some(c) => {
                                cell_option = Some(c);
                            }
                            None => {}
                        }

                        net_action_datas.push(action.data.into_net(
                            &action_data_request.get_id(),
                            action_data.target_entity_option,
                            cell_option,
                            action_data.action_taker,
                            action_data.action_taker_item,
                        ));
                    }
                    None => {
                        warn!("No entity handle for tab data request!");
                        continue;
                    }
                }
            }
        }

        match handle {
            Some(h) => {
                server.send(OutgoingReliableServerMessage {
                    handle: h,
                    message: ActionsServerMessage::TabData(net_action_datas),
                });
            }
            None => {}
        }
    }
}

/// Append actions found in [Actions] component of entities to their list.
pub(crate) fn list_action_data_from_actions_component(
    examinable_query: Query<&Actions>,
    mut building_actions: ResMut<BuildingActions>,
) {
    for building_action in building_actions.list.iter_mut() {
        match examinable_query.get(building_action.action_taker) {
            Ok(actions_component) => {
                for action in actions_component.actions.iter() {
                    building_action.actions.push(ActionData {
                        data: action.clone(),
                        approved: None,
                    });
                }
            }
            Err(_rr) => {}
        }
    }
}

/// A resource storing the current uniquely iterated identifier of action requests.

#[derive(Default, Resource)]
pub(crate) struct ActionIncremented {
    i: u64,
}

impl ActionIncremented {
    /// Get i with iteration.
    pub fn get_i_it(&mut self) -> u64 {
        let i = self.i.clone();
        self.i += 1;
        i
    }
    /// Get i without iterating.
    pub fn get_i(&self) -> u64 {
        if self.i > 0 {
            return self.i - 1;
        }
        self.i
    }
}

/// Resource with a request list of available actions for entity with prerequisite checking of this frame.

#[derive(Default, Resource)]
pub struct ListActionDataRequests {
    pub list: HashMap<u64, ActionRequest>,
}

/// Initialize listing action requests from input events.

pub(crate) fn init_action_data_listing(
    mut entity_events: EventReader<InputListActions>,
    mut building_action: ResMut<BuildingActions>,
    mut action_data_i: ResMut<ActionIncremented>,
    mut action_data_requests: ResMut<ListActionDataRequests>,
) {
    for event in entity_events.iter() {
        building_action.list.push(BuildingAction {
            incremented_i: action_data_i.get_i_it(),
            actions: vec![],
            action_taker: event.action_taker,
            target_entity_option: event.targetted_entity,
            target_cell_option: event.targetted_cell.clone(),
            action_taker_item: event.action_taker_item,
        });
        action_data_requests.list.insert(
            action_data_i.get_i(),
            ActionRequest::from_id("".to_string()),
        );
    }
}

/// An individual action.

#[derive(Clone)]
pub struct Action {
    /// Action identifier.
    pub id: String,
    /// Action text.
    pub text: String,
    /// Decides the order in which the actions show up.
    pub tab_list_priority: u8,
}

/// A component for entities with a list of actions available to them.

#[derive(Component, Default)]
pub struct Actions {
    pub actions: Vec<Action>,
}

impl Action {
    /// Convert action into a new struct with data suitable to be sent over the net.
    pub fn into_net(
        &self,
        item_name: &str,
        target_entity_option: Option<Entity>,
        target_cell_option: Option<TargetCell>,
        action_taker: Entity,
        action_taker_item: Option<Entity>,
    ) -> NetAction {
        NetAction {
            id: self.id.clone(),
            text: self.text.clone(),
            tab_list_priority: self.tab_list_priority,
            action_taker: action_taker,
            action_taker_item: action_taker_item,

            target_cell_option,
            item_name: item_name.to_string(),
            target_entity_option: target_entity_option,
        }
    }
}

/// Clears all actions for the next tick.

pub(crate) fn clear_action_building(
    mut action_data_requests: ResMut<ListActionDataRequests>,
    mut action_requests: ResMut<ActionRequests>,
    mut building_action: ResMut<BuildingActions>,
) {
    action_data_requests.list.clear();
    action_requests.list.clear();
    building_action.list.clear();
}

/// Initialize action (list) requests.

pub(crate) fn init_action_request_building(
    mut building_actions: ResMut<BuildingActions>,
    mut action_events: EventReader<InputAction>,
    mut action_data_i: ResMut<ActionIncremented>,
    mut actions_requests: ResMut<ActionRequests>,
) {
    for event in action_events.iter() {
        let mut examined_cell = None;

        match event.target_cell_option.clone() {
            Some(ya) => {
                examined_cell = Some(ya);
            }
            None => {}
        }

        building_actions.list.push(BuildingAction {
            actions: vec![],
            incremented_i: action_data_i.get_i_it(),
            action_taker: event.action_taker,
            target_entity_option: event.target_entity_option,
            target_cell_option: examined_cell,
            action_taker_item: event.action_taker_item,
        });
        actions_requests.list.insert(
            action_data_i.get_i(),
            ActionRequest::from_id(event.fired_action_id.clone()),
        );
    }
}

/// Client input list actions entity event.
#[derive(Clone)]
pub struct InputListActions {
    pub action_taker: Entity,
    pub action_taker_item: Option<Entity>,
    /// Targetted entity.
    pub targetted_entity: Option<Entity>,
    pub targetted_cell: Option<TargetCell>,
    /// Whether UI should be displayed to the requested by entity.
    pub with_ui: bool,
}

/// Client initiates execution of an action event.

pub struct InputAction {
    /// Action ID.
    pub fired_action_id: String,
    pub action_taker: Entity,
    pub action_taker_item: Option<Entity>,

    pub target_entity_option: Option<Entity>,
    pub target_cell_option: Option<TargetCell>,
}
