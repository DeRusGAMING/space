use std::collections::HashMap;

use bevy::{
    math::Vec3,
    prelude::{error, warn, Component, Entity, EventReader, Query, Res, Transform},
};

use bevy::prelude::SystemLabel;
use entity::{
    sensable::Sensable,
    senser::{to_doryen_coordinates, Senser},
};
use math::grid::world_to_cell_id;
use networking::server::EntityUpdateData;
use pawn::pawn::ShipJobsEnum;
use sfx::{proximity_message::PlaySoundProximityMessageData, radio_sound::PlaySoundRadioMessage};
use text_api::core::{
    escape_bb, BILLBOARD_DATA_SECURITY_END, BILLBOARD_DATA_SECURITY_START, BILLBOARD_SHOUT_FONT,
    BILLBOARD_SHOUT_ITALIC_FONT, FAR_BOLD_FONT, FAR_SHOUT_FONT, FURTHER_BOLD_FONT,
    FURTHER_SHOUT_FONT, JOB_CONTROL_WORD, JOB_SECURITY_WORD, NEARBY_BOLD_FONT, NEARBY_SHOUT_FONT,
    SHOUT_DATA_MACHINE_FAR_I_END, SHOUT_DATA_MACHINE_FAR_I_START, SHOUT_DATA_MACHINE_FURTHER_I_END,
    SHOUT_DATA_MACHINE_FURTHER_I_START, SHOUT_DATA_MACHINE_NEARBY_I_END,
    SHOUT_DATA_MACHINE_NEARBY_I_START, SHOUT_DATA_STANDARD_FAR_I_END,
    SHOUT_DATA_STANDARD_FAR_I_START, SHOUT_DATA_STANDARD_FURTHER_I_END,
    SHOUT_DATA_STANDARD_FURTHER_I_START, SHOUT_DATA_STANDARD_NEARBY_I_END,
    SHOUT_DATA_STANDARD_NEARBY_I_START, TALK_DATA_MACHINE_B_FAR_END, TALK_DATA_MACHINE_B_FAR_START,
    TALK_DATA_MACHINE_B_FURTHER_END, TALK_DATA_MACHINE_B_FURTHER_START,
    TALK_DATA_MACHINE_B_NEARBY_END, TALK_DATA_MACHINE_B_NEARBY_START, TALK_DATA_MACHINE_I_FAR_END,
    TALK_DATA_MACHINE_I_FAR_START, TALK_DATA_MACHINE_I_FURTHER_END,
    TALK_DATA_MACHINE_I_FURTHER_START, TALK_DATA_MACHINE_I_NEARBY_END,
    TALK_DATA_MACHINE_I_NEARBY_START, TALK_DATA_MACHINE_NORMAL_FAR_END,
    TALK_DATA_MACHINE_NORMAL_FAR_START, TALK_DATA_MACHINE_NORMAL_FURTHER_END,
    TALK_DATA_MACHINE_NORMAL_FURTHER_START, TALK_DATA_MACHINE_NORMAL_NEARBY_END,
    TALK_DATA_MACHINE_NORMAL_NEARBY_START, TALK_DATA_STANDARD_B_FAR_END,
    TALK_DATA_STANDARD_B_FAR_START, TALK_DATA_STANDARD_B_FURTHER_END,
    TALK_DATA_STANDARD_B_FURTHER_START, TALK_DATA_STANDARD_B_NEARBY_END,
    TALK_DATA_STANDARD_B_NEARBY_START, TALK_DATA_STANDARD_I_FAR_END,
    TALK_DATA_STANDARD_I_FAR_START, TALK_DATA_STANDARD_I_FURTHER_END,
    TALK_DATA_STANDARD_I_FURTHER_START, TALK_DATA_STANDARD_I_NEARBY_END,
    TALK_DATA_STANDARD_I_NEARBY_START, TALK_DATA_STANDARD_NORMAL_FAR_END,
    TALK_DATA_STANDARD_NORMAL_FAR_START, TALK_DATA_STANDARD_NORMAL_FURTHER_END,
    TALK_DATA_STANDARD_NORMAL_FURTHER_START, TALK_DATA_STANDARD_NORMAL_NEARBY_END,
    TALK_DATA_STANDARD_NORMAL_NEARBY_START, TALK_SPACE_COMMON_CHATPREFIX,
    TALK_SPACE_COMMON_MESSAGEBBEND, TALK_SPACE_COMMON_MESSAGEBBSTART,
    TALK_SPACE_COMMON_PREFIXBBEND, TALK_SPACE_COMMON_PREFIXBBSTART, TALK_SPACE_COMMON_WORD,
    TALK_SPACE_GLOBAL_CHATPREFIX, TALK_SPACE_PROXIMITY_EMOTE_CHATPREFIX,
    TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBEND, TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBSTART,
    TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBEND, TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBSTART,
    TALK_SPACE_PROXIMITY_MESSAGEBBEND, TALK_SPACE_PROXIMITY_MESSAGEBBSTART,
    TALK_SPACE_PROXIMITY_PREFIXBBEND, TALK_SPACE_PROXIMITY_PREFIXBBSTART,
    TALK_SPACE_SECURITY_CHATPREFIX, TALK_SPACE_SECURITY_MESSAGEBBEND,
    TALK_SPACE_SECURITY_MESSAGEBBSTART, TALK_SPACE_SECURITY_PREFIXBBEND,
    TALK_SPACE_SECURITY_PREFIXBBSTART, TALK_SPACE_SECURITY_WORD, TALK_SPACE_SPECIALOPS_CHATPREFIX,
    TALK_SPACE_SPECIALOPS_MESSAGEBBEND, TALK_SPACE_SPECIALOPS_MESSAGEBBSTART,
    TALK_SPACE_SPECIALOPS_PREFIXBBEND, TALK_SPACE_SPECIALOPS_PREFIXBBSTART,
    TALK_SPACE_SPECIALOPS_WORD, TALK_STYLE_MACHINE_ASKS, TALK_STYLE_MACHINE_EXCLAIMS,
    TALK_STYLE_MACHINE_SHOUTS, TALK_STYLE_MACHINE_STANDARD, TALK_STYLE_STANDARD_ASKS,
    TALK_STYLE_STANDARD_EXCLAIMS, TALK_STYLE_STANDARD_SHOUTS, TALK_STYLE_STANDARD_STANDARD,
    TALK_TYPE_MACHINE_NEARBY_END, TALK_TYPE_MACHINE_NEARBY_START, TALK_TYPE_STANDARD_NEARBY_END,
    TALK_TYPE_STANDARD_NEARBY_START,
};
use voca_rs::*;

/// Radio component for entities that can hear or speak through radios.
#[cfg(feature = "server")]
#[derive(Component)]
pub struct Radio {
    pub listen_access: Vec<RadioChannel>,
    pub speak_access: Vec<RadioChannel>,
}

/// All available chat channels.
#[cfg(feature = "server")]
#[derive(PartialEq, Debug, Clone)]
pub enum RadioChannel {
    Proximity,
    ProximityEmote,
    Global,
    Common,
    Security,
    SpecialOps,
}

use networking::server::HandleToEntity;

use networking::server::ConnectedPlayer;
use player::boarding::SoftPlayer;

/// Chat distance. Impacts font size.
#[cfg(feature = "server")]
enum Distance {
    Nearby,
    Further,
    Far,
}

/// Chat talk style variant.
#[cfg(feature = "server")]
enum TalkStyleVariant {
    Standard,
    Shouts,
    Exclaims,
    Asks,
}

/// Check if a message has a shouting intend as a function.
#[cfg(feature = "server")]
fn is_shouting(message: &str) -> bool {
    message.ends_with("!!!")
        || message.ends_with("!!?")
        || message.ends_with("!?!")
        || message.ends_with("?!!")
        || message.ends_with("??!")
        || message.ends_with("?!?")
        || message.ends_with("??!")
        || message.ends_with("!??")
        || message.ends_with("???")
}

/// Check if a message has a questioning intend as a function.
#[cfg(feature = "server")]
fn is_asking(message: &str) -> bool {
    message.ends_with("?") || message.ends_with("??") || message.ends_with("?!")
}

/// Process chat prefixes which act as flags as a function.
#[cfg(feature = "server")]
fn get_talk_space(message: String) -> (RadioChannel, String, bool, bool) {
    let radio_channel;
    let content;
    let mut exclusive_proximity = false;
    let mut is_emote = false;

    if message.starts_with(TALK_SPACE_GLOBAL_CHATPREFIX) {
        radio_channel = RadioChannel::Global;
        content = message.split(TALK_SPACE_GLOBAL_CHATPREFIX).collect();
    } else if message.starts_with(TALK_SPACE_PROXIMITY_EMOTE_CHATPREFIX) {
        radio_channel = RadioChannel::ProximityEmote;
        content = message
            .split(TALK_SPACE_PROXIMITY_EMOTE_CHATPREFIX)
            .collect();
        exclusive_proximity = true;
        is_emote = true;
    } else if message.starts_with(TALK_SPACE_COMMON_CHATPREFIX) {
        radio_channel = RadioChannel::Common;
        content = message.split(TALK_SPACE_COMMON_CHATPREFIX).collect();
    } else if message.starts_with(TALK_SPACE_SECURITY_CHATPREFIX) {
        radio_channel = RadioChannel::Security;
        content = message.split(TALK_SPACE_SECURITY_CHATPREFIX).collect();
    } else if message.starts_with(TALK_SPACE_SPECIALOPS_CHATPREFIX) {
        radio_channel = RadioChannel::SpecialOps;
        content = message.split(TALK_SPACE_SPECIALOPS_CHATPREFIX).collect();
    } else {
        radio_channel = RadioChannel::Proximity;
        content = message.to_owned();
        exclusive_proximity = true;
    }

    (radio_channel, content, exclusive_proximity, is_emote)
}
/// Parts of the chat and radio channels can and can't they access depend on it.
#[cfg(feature = "server")]
pub enum MessagingPlayerState {
    SoftConnected,
    Alive,
}

/// Event triggers new chat message.
pub struct NewChatMessage {
    pub messenger_entity_option: Option<Entity>,
    pub messenger_name_option: Option<String>,
    pub raw_message: String,
    pub exclusive_radio: bool,
    pub position_option: Option<Vec3>,
    pub send_entity_update: bool,
}
use pawn::pawn::Communicator;
use pawn::pawn::Pawn;

use entity::net::EntityServerMessage;
use networking::server::NetworkingChatServerMessage;
use networking::server::OutgoingReliableServerMessage;
use player::account::Accounts;
use sfx::net::SfxServerMessage;

use bevy::prelude::EventWriter;
/// It is huge, not-modular and just overall not nice. This will get modularized and rewritten for the Bevy client when it is ready.
#[cfg(feature = "server")]
pub(crate) fn chat_message(
    mut new_chat_messages: EventReader<NewChatMessage>,
    soft_player_query: Query<&SoftPlayer>,
    global_listeners: Query<&ConnectedPlayer>,
    mut server: EventWriter<OutgoingReliableServerMessage<NetworkingChatServerMessage>>,
    mut server1: EventWriter<OutgoingReliableServerMessage<EntityServerMessage>>,
    mut server2: EventWriter<OutgoingReliableServerMessage<SfxServerMessage>>,
    radio_pawns: Query<(Entity, &Radio, &Transform, &Pawn)>,
    handle_to_entity: Res<HandleToEntity>,
    player_pawns: Query<(&Pawn, &Transform, &Sensable)>,
    accounts: Res<Accounts>,
) {
    use entity::entity_data::EntityWorldType;

    for new_message in new_chat_messages.iter() {
        let mut messaging_player_state = &MessagingPlayerState::Alive;

        let position;

        match new_message.position_option {
            Some(p) => {
                position = p;
            }
            None => match new_message.messenger_entity_option {
                Some(e) => match player_pawns.get(e) {
                    Ok((_, transform, _)) => {
                        position = transform.translation;
                    }
                    Err(_) => {
                        warn!("Couldnt find messenger position.");
                        position = Vec3::ZERO;
                    }
                },
                None => {
                    position = Vec3::ZERO;
                }
            },
        }

        let communicator;
        let job;
        let mut messenger_name = "".to_string();

        let mut set_name = false;

        match &new_message.messenger_name_option {
            Some(n) => {
                messenger_name = n.clone();
                set_name = true;
            }
            None => {}
        }

        match new_message.messenger_entity_option {
            Some(ent) => match player_pawns.get(ent) {
                Ok((pawn, _, _)) => {
                    communicator = pawn.communicator.clone();
                    job = pawn.job;
                    if !set_name {
                        messenger_name = pawn.character_name.clone();
                    }
                }
                Err(_) => {
                    warn!("Couldnt find pawn chat data.");

                    communicator = Communicator::Machine;
                    job = ShipJobsEnum::Control;
                }
            },
            None => {
                communicator = Communicator::Machine;
                job = ShipJobsEnum::Control;
            }
        }

        match new_message.messenger_entity_option {
            Some(ent) => match soft_player_query.get(ent) {
                Ok(_) => {
                    messaging_player_state = &MessagingPlayerState::SoftConnected;
                }
                Err(_) => {}
            },
            None => {}
        }
        let mut message = new_message.raw_message.clone();

        if message.len() > 500 {
            message = message[..500].to_string();
        }

        message = escape_bb(message, false, false);

        let mut radio_channel;
        let mut exclusive_proximity;
        let mut is_emote;

        let result = get_talk_space(message.clone());
        radio_channel = result.0;
        message = result.1;
        exclusive_proximity = result.2;
        is_emote = result.3;

        message = escape_bb(message, false, false);

        let mut prev_was_proximity;

        if matches!(radio_channel, RadioChannel::Proximity) {
            prev_was_proximity = true;
        } else {
            prev_was_proximity = false;
        }

        let mut proximity_emote_included;

        if matches!(radio_channel, RadioChannel::ProximityEmote) {
            proximity_emote_included = true;
        } else {
            proximity_emote_included = false;
        }

        let mut radio_command_included;
        let mut included_radio_channel;

        if !matches!(radio_channel, RadioChannel::Proximity)
            && !matches!(radio_channel, RadioChannel::ProximityEmote)
        {
            radio_command_included = true;
            included_radio_channel = Some(radio_channel.clone());
        } else {
            radio_command_included = false;
            included_radio_channel = None;
        }

        while !prev_was_proximity {
            let result = get_talk_space(message.clone());

            if matches!(result.0, RadioChannel::ProximityEmote) {
                proximity_emote_included = true;
            }

            if !matches!(radio_channel, RadioChannel::Proximity)
                && !matches!(radio_channel, RadioChannel::ProximityEmote)
            {
                radio_command_included = true;
                included_radio_channel = Some(radio_channel.clone());
            }

            if matches!(result.0, RadioChannel::Proximity) {
                prev_was_proximity = true;
            } else {
                prev_was_proximity = false;
                radio_channel = result.0;
                message = result.1;
                exclusive_proximity = result.2;
                is_emote = result.3;

                message = escape_bb(message, false, false);
            }
        }

        if matches!(messaging_player_state, &MessagingPlayerState::SoftConnected) {
            radio_channel = RadioChannel::Global;
        }

        // Emote over Radio channel for the memes.
        if !matches!(radio_channel, RadioChannel::ProximityEmote) {
            if proximity_emote_included {
                is_emote = true;
                exclusive_proximity = false;
            } else {
                message = case::capitalize(&message, false);
            }
        } else {
            if radio_command_included {
                is_emote = true;
                exclusive_proximity = false;
                radio_channel = included_radio_channel.unwrap();

                if matches!(radio_channel, RadioChannel::Global) {
                    message = case::capitalize(&message, false);
                }
            }
        }

        if matches!(radio_channel, RadioChannel::Global) {
            match global_listeners.get(new_message.messenger_entity_option.unwrap()) {
                Ok(_connected) => {
                    for connected_player_component in global_listeners.iter() {
                        let account_name;
                        match accounts.list.get(&connected_player_component.handle) {
                            Some(a) => {
                                account_name = a.clone();
                            }
                            None => {
                                warn!(
                                    "Could not find account with handle {}",
                                    connected_player_component.handle
                                );
                                continue;
                            }
                        }

                        let message =
                            account_name + "[b][color=#322bff](Global)[/color][/b]: " + &message;

                        if connected_player_component.connected == false {
                            continue;
                        }

                        server.send(OutgoingReliableServerMessage {
                            handle: connected_player_component.handle,
                            message: NetworkingChatServerMessage::ChatMessage(message.clone()),
                        });
                    }
                }
                Err(_rr) => {
                    warn!("Couldnt find components of global messenger.");
                }
            }

            return;
        }

        if new_message.exclusive_radio == true {
            exclusive_proximity = false;
        }

        if message.len() == 0 {
            return;
        }

        let mut talk_style_variation = TalkStyleVariant::Standard;

        if is_emote == false {
            if is_shouting(&message) {
                talk_style_variation = TalkStyleVariant::Shouts;
            } else if message.ends_with("!") {
                talk_style_variation = TalkStyleVariant::Exclaims;
            } else if is_asking(&message) {
                talk_style_variation = TalkStyleVariant::Asks;
            }
        }

        let mut radio_message: String = "".to_string();

        if exclusive_proximity == false {
            // Radio chat message.

            let talk_space_prefix_bb_start;
            let talk_space_prefix_bb_end;
            let talk_space_message_bb_start;
            let talk_space_message_bb_end;
            let mut talk_space_word = "";
            match radio_channel {
                RadioChannel::Proximity => {
                    talk_space_prefix_bb_start = TALK_SPACE_PROXIMITY_PREFIXBBSTART;
                    talk_space_prefix_bb_end = TALK_SPACE_PROXIMITY_PREFIXBBEND;
                    talk_space_message_bb_start = TALK_SPACE_PROXIMITY_MESSAGEBBSTART;
                    talk_space_message_bb_end = TALK_SPACE_PROXIMITY_MESSAGEBBEND;
                }
                RadioChannel::ProximityEmote => {
                    talk_space_prefix_bb_start = TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBSTART;
                    talk_space_prefix_bb_end = TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBEND;
                    talk_space_message_bb_start = TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBSTART;
                    talk_space_message_bb_end = TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBEND;
                }
                RadioChannel::Common => {
                    talk_space_prefix_bb_start = TALK_SPACE_COMMON_PREFIXBBSTART;
                    talk_space_word = TALK_SPACE_COMMON_WORD;
                    talk_space_prefix_bb_end = TALK_SPACE_COMMON_PREFIXBBEND;
                    talk_space_message_bb_start = TALK_SPACE_COMMON_MESSAGEBBSTART;
                    talk_space_message_bb_end = TALK_SPACE_COMMON_MESSAGEBBEND;
                }
                RadioChannel::Security => {
                    talk_space_prefix_bb_start = TALK_SPACE_SECURITY_PREFIXBBSTART;
                    talk_space_word = TALK_SPACE_SECURITY_WORD;
                    talk_space_prefix_bb_end = TALK_SPACE_SECURITY_PREFIXBBEND;
                    talk_space_message_bb_start = TALK_SPACE_SECURITY_MESSAGEBBSTART;
                    talk_space_message_bb_end = TALK_SPACE_SECURITY_MESSAGEBBEND;
                }
                RadioChannel::SpecialOps => {
                    talk_space_prefix_bb_start = TALK_SPACE_SPECIALOPS_PREFIXBBSTART;
                    talk_space_word = TALK_SPACE_SPECIALOPS_WORD;
                    talk_space_prefix_bb_end = TALK_SPACE_SPECIALOPS_PREFIXBBEND;
                    talk_space_message_bb_start = TALK_SPACE_SPECIALOPS_MESSAGEBBSTART;
                    talk_space_message_bb_end = TALK_SPACE_SPECIALOPS_MESSAGEBBEND;
                }
                RadioChannel::Global => {
                    warn!("Processing global chat while we shouldn't?");
                    return;
                }
            }

            let talk_font_nearby_start;
            let talk_font_nearby_end;

            let talk_font_nearby_start_1;
            let talk_font_nearby_end_1;
            let talk_style_variation_word;
            match communicator {
                Communicator::Standard => {
                    talk_font_nearby_start = TALK_DATA_STANDARD_B_NEARBY_START;
                    talk_font_nearby_end = TALK_DATA_STANDARD_B_NEARBY_END;

                    talk_font_nearby_start_1 = TALK_TYPE_STANDARD_NEARBY_START;
                    talk_font_nearby_end_1 = TALK_TYPE_STANDARD_NEARBY_END;

                    match talk_style_variation {
                        TalkStyleVariant::Standard => {
                            talk_style_variation_word = TALK_STYLE_STANDARD_STANDARD;
                        }
                        TalkStyleVariant::Shouts => {
                            talk_style_variation_word = TALK_STYLE_STANDARD_SHOUTS;
                        }
                        TalkStyleVariant::Exclaims => {
                            talk_style_variation_word = TALK_STYLE_STANDARD_EXCLAIMS;
                        }
                        TalkStyleVariant::Asks => {
                            talk_style_variation_word = TALK_STYLE_STANDARD_ASKS;
                        }
                    }
                }
                Communicator::Machine => {
                    talk_font_nearby_start = TALK_DATA_MACHINE_B_NEARBY_START;
                    talk_font_nearby_end = TALK_DATA_MACHINE_B_NEARBY_END;

                    talk_font_nearby_start_1 = TALK_TYPE_MACHINE_NEARBY_START;
                    talk_font_nearby_end_1 = TALK_TYPE_MACHINE_NEARBY_END;
                    match talk_style_variation {
                        TalkStyleVariant::Standard => {
                            talk_style_variation_word = TALK_STYLE_MACHINE_STANDARD;
                        }
                        TalkStyleVariant::Shouts => {
                            talk_style_variation_word = TALK_STYLE_MACHINE_SHOUTS;
                        }
                        TalkStyleVariant::Exclaims => {
                            talk_style_variation_word = TALK_STYLE_MACHINE_EXCLAIMS;
                        }
                        TalkStyleVariant::Asks => {
                            talk_style_variation_word = TALK_STYLE_MACHINE_ASKS;
                        }
                    }
                }
            }

            let rank_word;
            match job {
                ShipJobsEnum::Security => {
                    rank_word = JOB_SECURITY_WORD;
                }
                ShipJobsEnum::Control => {
                    rank_word = JOB_CONTROL_WORD;
                }
            }

            if is_emote {
                radio_message = radio_message + talk_space_prefix_bb_start;
                radio_message = radio_message + talk_font_nearby_start;
                radio_message = radio_message
                    + &messenger_name
                    + " ["
                    + talk_space_word
                    + "]["
                    + rank_word
                    + "] ";
                radio_message = radio_message + talk_font_nearby_end + talk_space_prefix_bb_end;
                radio_message = radio_message + talk_space_message_bb_start;

                radio_message =
                    radio_message + talk_font_nearby_start_1 + &message + talk_font_nearby_end_1;
                radio_message = radio_message + talk_space_message_bb_end;
            } else {
                radio_message = radio_message + talk_space_prefix_bb_start;
                radio_message = radio_message + talk_font_nearby_start;
                radio_message = radio_message
                    + &messenger_name
                    + " ["
                    + talk_space_word
                    + "]["
                    + rank_word
                    + "] ";
                radio_message = radio_message + talk_font_nearby_end + talk_space_prefix_bb_end;
                radio_message = radio_message + talk_space_message_bb_start;

                radio_message = radio_message + talk_style_variation_word + ",\n";

                if matches!(talk_style_variation, TalkStyleVariant::Shouts) {
                    radio_message = radio_message
                        + talk_font_nearby_start_1
                        + "[font="
                        + NEARBY_SHOUT_FONT
                        + "]\""
                        + &message
                        + "\"[/font]"
                        + talk_font_nearby_end_1;
                } else {
                    radio_message = radio_message
                        + talk_font_nearby_start_1
                        + "\""
                        + &message
                        + "\""
                        + talk_font_nearby_end_1;
                }

                radio_message = radio_message + talk_space_message_bb_end;
            }
        }

        // Build proximity message.
        // For 3 different distances.

        let mut proximity_message_nearby = "".to_string();
        let mut proximity_message_further = "".to_string();
        let mut proximity_message_far = "".to_string();

        if new_message.exclusive_radio == false {
            proximity_message_nearby = proximity_message_nearby
                + "[font="
                + NEARBY_BOLD_FONT
                + "]"
                + TALK_SPACE_PROXIMITY_PREFIXBBSTART;
            proximity_message_further = proximity_message_further
                + "[font="
                + FURTHER_BOLD_FONT
                + "]"
                + TALK_SPACE_PROXIMITY_PREFIXBBSTART;
            proximity_message_far = proximity_message_far
                + "[font="
                + FAR_BOLD_FONT
                + "]"
                + TALK_SPACE_PROXIMITY_PREFIXBBSTART;

            let nearby_talk_data_b_end;
            let further_talk_data_b_end;
            let far_talk_data_b_end;

            match communicator {
                Communicator::Standard => {
                    proximity_message_nearby =
                        proximity_message_nearby + TALK_DATA_STANDARD_B_NEARBY_START;
                    proximity_message_further =
                        proximity_message_further + TALK_DATA_STANDARD_B_FURTHER_START;
                    proximity_message_far = proximity_message_far + TALK_DATA_STANDARD_B_FAR_START;

                    nearby_talk_data_b_end = TALK_DATA_STANDARD_B_NEARBY_END;
                    further_talk_data_b_end = TALK_DATA_STANDARD_B_FURTHER_END;
                    far_talk_data_b_end = TALK_DATA_STANDARD_B_FAR_END;
                }
                Communicator::Machine => {
                    proximity_message_nearby =
                        proximity_message_nearby + TALK_DATA_MACHINE_B_NEARBY_START;
                    proximity_message_further =
                        proximity_message_further + TALK_DATA_MACHINE_B_FURTHER_START;
                    proximity_message_far = proximity_message_far + TALK_DATA_MACHINE_B_FAR_START;

                    nearby_talk_data_b_end = TALK_DATA_MACHINE_B_NEARBY_END;
                    further_talk_data_b_end = TALK_DATA_MACHINE_B_FURTHER_END;
                    far_talk_data_b_end = TALK_DATA_MACHINE_B_FAR_END;
                }
            }

            proximity_message_nearby =
                proximity_message_nearby + &messenger_name + nearby_talk_data_b_end + " ";
            proximity_message_further =
                proximity_message_further + &messenger_name + further_talk_data_b_end + " ";
            proximity_message_far =
                proximity_message_far + &messenger_name + far_talk_data_b_end + " ";

            let rank_word;
            match job {
                ShipJobsEnum::Security => {
                    rank_word = JOB_SECURITY_WORD;
                }
                ShipJobsEnum::Control => {
                    rank_word = JOB_CONTROL_WORD;
                }
            }

            if is_emote == false {
                proximity_message_nearby = proximity_message_nearby + "[" + rank_word + "]";
                proximity_message_further = proximity_message_further + "[" + rank_word + "]";
                proximity_message_far = proximity_message_far + "[" + rank_word + "]";
            }

            proximity_message_nearby = proximity_message_nearby
                + TALK_SPACE_PROXIMITY_PREFIXBBEND
                + "[/font]"
                + TALK_SPACE_PROXIMITY_MESSAGEBBSTART;
            proximity_message_further = proximity_message_further
                + TALK_SPACE_PROXIMITY_PREFIXBBEND
                + "[/font]"
                + TALK_SPACE_PROXIMITY_MESSAGEBBSTART;
            proximity_message_far = proximity_message_far
                + TALK_SPACE_PROXIMITY_PREFIXBBEND
                + "[/font]"
                + TALK_SPACE_PROXIMITY_MESSAGEBBSTART;

            if is_emote == false {
                let talk_style_variation_word;
                match communicator {
                    Communicator::Standard => {
                        proximity_message_nearby =
                            proximity_message_nearby + TALK_DATA_STANDARD_NORMAL_NEARBY_START;
                        proximity_message_further =
                            proximity_message_further + TALK_DATA_STANDARD_NORMAL_FURTHER_START;
                        proximity_message_far =
                            proximity_message_far + TALK_DATA_STANDARD_NORMAL_FAR_START;

                        match talk_style_variation {
                            TalkStyleVariant::Standard => {
                                talk_style_variation_word = TALK_STYLE_STANDARD_STANDARD;
                            }
                            TalkStyleVariant::Shouts => {
                                talk_style_variation_word = TALK_STYLE_STANDARD_SHOUTS;
                            }
                            TalkStyleVariant::Exclaims => {
                                talk_style_variation_word = TALK_STYLE_STANDARD_EXCLAIMS;
                            }
                            TalkStyleVariant::Asks => {
                                talk_style_variation_word = TALK_STYLE_STANDARD_ASKS;
                            }
                        }
                    }
                    Communicator::Machine => {
                        proximity_message_nearby =
                            proximity_message_nearby + TALK_DATA_MACHINE_NORMAL_NEARBY_START;
                        proximity_message_further =
                            proximity_message_further + TALK_DATA_MACHINE_NORMAL_FURTHER_START;
                        proximity_message_far =
                            proximity_message_far + TALK_DATA_MACHINE_NORMAL_FAR_START;

                        match talk_style_variation {
                            TalkStyleVariant::Standard => {
                                talk_style_variation_word = TALK_STYLE_MACHINE_STANDARD;
                            }
                            TalkStyleVariant::Shouts => {
                                talk_style_variation_word = TALK_STYLE_MACHINE_SHOUTS;
                            }
                            TalkStyleVariant::Exclaims => {
                                talk_style_variation_word = TALK_STYLE_MACHINE_EXCLAIMS;
                            }
                            TalkStyleVariant::Asks => {
                                talk_style_variation_word = TALK_STYLE_MACHINE_ASKS;
                            }
                        }
                    }
                }

                proximity_message_nearby =
                    proximity_message_nearby + talk_style_variation_word + ",\n";
                proximity_message_further =
                    proximity_message_further + talk_style_variation_word + ",\n";
                proximity_message_far = proximity_message_far + talk_style_variation_word + ",\n";

                let nearby_talk_data_i_start;
                let further_talk_data_i_start;
                let far_talk_data_i_start;

                let nearby_talk_data_i_end;
                let further_talk_data_i_end;
                let far_talk_data_i_end;

                let nearby_talk_data_normal_start;
                let further_talk_data_normal_start;
                let far_talk_data_normal_start;

                let nearby_talk_data_normal_end;
                let further_talk_data_normal_end;
                let far_talk_data_normal_end;

                let nearby_talk_data_b_start;
                let further_talk_data_b_start;
                let far_talk_data_b_start;

                let nearby_shout_data_i_start;
                let further_shout_data_i_start;
                let far_shout_data_i_start;

                let nearby_shout_data_i_end;
                let further_shout_data_i_end;
                let far_shout_data_i_end;

                let nearby_talk_data_start;
                let further_talk_data_start;
                let far_talk_data_start;

                let nearby_talk_data_end;
                let further_talk_data_end;
                let far_talk_data_end;

                match communicator {
                    Communicator::Standard => {
                        proximity_message_nearby =
                            proximity_message_nearby + TALK_DATA_STANDARD_NORMAL_NEARBY_END;
                        proximity_message_further =
                            proximity_message_further + TALK_DATA_STANDARD_NORMAL_FURTHER_END;
                        proximity_message_far =
                            proximity_message_far + TALK_DATA_STANDARD_NORMAL_FAR_END;

                        nearby_talk_data_i_start = TALK_DATA_STANDARD_I_NEARBY_START;
                        further_talk_data_i_start = TALK_DATA_STANDARD_I_FURTHER_START;
                        far_talk_data_i_start = TALK_DATA_STANDARD_I_FAR_START;

                        nearby_talk_data_i_end = TALK_DATA_STANDARD_I_NEARBY_END;
                        further_talk_data_i_end = TALK_DATA_STANDARD_I_FURTHER_END;
                        far_talk_data_i_end = TALK_DATA_STANDARD_I_FAR_END;

                        nearby_talk_data_normal_start = TALK_DATA_STANDARD_NORMAL_NEARBY_START;
                        further_talk_data_normal_start = TALK_DATA_STANDARD_NORMAL_FURTHER_START;
                        far_talk_data_normal_start = TALK_DATA_STANDARD_NORMAL_FAR_START;

                        nearby_talk_data_normal_end = TALK_DATA_STANDARD_NORMAL_NEARBY_END;
                        further_talk_data_normal_end = TALK_DATA_STANDARD_NORMAL_FURTHER_END;
                        far_talk_data_normal_end = TALK_DATA_STANDARD_NORMAL_FAR_END;

                        nearby_talk_data_b_start = TALK_DATA_STANDARD_B_NEARBY_START;
                        further_talk_data_b_start = TALK_DATA_STANDARD_B_FURTHER_START;
                        far_talk_data_b_start = TALK_DATA_STANDARD_B_FAR_START;

                        nearby_shout_data_i_start = SHOUT_DATA_STANDARD_NEARBY_I_START;
                        further_shout_data_i_start = SHOUT_DATA_STANDARD_FURTHER_I_START;
                        far_shout_data_i_start = SHOUT_DATA_STANDARD_FAR_I_START;

                        nearby_shout_data_i_end = SHOUT_DATA_STANDARD_NEARBY_I_END;
                        further_shout_data_i_end = SHOUT_DATA_STANDARD_FURTHER_I_END;
                        far_shout_data_i_end = SHOUT_DATA_STANDARD_FAR_I_END;

                        nearby_talk_data_start = TALK_TYPE_STANDARD_NEARBY_START;
                        further_talk_data_start = TALK_TYPE_STANDARD_NEARBY_START;
                        far_talk_data_start = TALK_TYPE_STANDARD_NEARBY_START;

                        nearby_talk_data_end = TALK_TYPE_STANDARD_NEARBY_END;
                        further_talk_data_end = TALK_TYPE_STANDARD_NEARBY_END;
                        far_talk_data_end = TALK_TYPE_STANDARD_NEARBY_END;
                    }
                    Communicator::Machine => {
                        proximity_message_nearby =
                            proximity_message_nearby + TALK_DATA_MACHINE_NORMAL_NEARBY_END;
                        proximity_message_further =
                            proximity_message_further + TALK_DATA_MACHINE_NORMAL_FURTHER_END;
                        proximity_message_far =
                            proximity_message_far + TALK_DATA_MACHINE_NORMAL_FAR_END;

                        nearby_talk_data_i_start = TALK_DATA_MACHINE_I_NEARBY_START;
                        further_talk_data_i_start = TALK_DATA_MACHINE_I_FURTHER_START;
                        far_talk_data_i_start = TALK_DATA_MACHINE_I_FAR_START;

                        nearby_talk_data_i_end = TALK_DATA_MACHINE_I_NEARBY_END;
                        further_talk_data_i_end = TALK_DATA_MACHINE_I_FURTHER_END;
                        far_talk_data_i_end = TALK_DATA_MACHINE_I_FAR_END;

                        nearby_talk_data_normal_start = TALK_DATA_MACHINE_NORMAL_NEARBY_START;
                        further_talk_data_normal_start = TALK_DATA_MACHINE_NORMAL_FURTHER_START;
                        far_talk_data_normal_start = TALK_DATA_MACHINE_NORMAL_FAR_START;

                        nearby_talk_data_normal_end = TALK_DATA_MACHINE_NORMAL_NEARBY_END;
                        further_talk_data_normal_end = TALK_DATA_MACHINE_NORMAL_FURTHER_END;
                        far_talk_data_normal_end = TALK_DATA_MACHINE_NORMAL_FAR_END;

                        nearby_talk_data_b_start = TALK_DATA_MACHINE_B_NEARBY_START;
                        further_talk_data_b_start = TALK_DATA_MACHINE_B_FURTHER_START;
                        far_talk_data_b_start = TALK_DATA_MACHINE_B_FAR_START;

                        nearby_shout_data_i_start = SHOUT_DATA_MACHINE_NEARBY_I_START;
                        further_shout_data_i_start = SHOUT_DATA_MACHINE_FURTHER_I_START;
                        far_shout_data_i_start = SHOUT_DATA_MACHINE_FAR_I_START;

                        nearby_shout_data_i_end = SHOUT_DATA_MACHINE_NEARBY_I_END;
                        further_shout_data_i_end = SHOUT_DATA_MACHINE_FURTHER_I_END;
                        far_shout_data_i_end = SHOUT_DATA_MACHINE_FAR_I_END;

                        nearby_talk_data_start = TALK_TYPE_MACHINE_NEARBY_START;
                        further_talk_data_start = TALK_TYPE_MACHINE_NEARBY_START;
                        far_talk_data_start = TALK_TYPE_MACHINE_NEARBY_START;

                        nearby_talk_data_end = TALK_TYPE_MACHINE_NEARBY_END;
                        further_talk_data_end = TALK_TYPE_MACHINE_NEARBY_END;
                        far_talk_data_end = TALK_TYPE_MACHINE_NEARBY_END;
                    }
                }

                if exclusive_proximity == false {
                    proximity_message_nearby = proximity_message_nearby + nearby_talk_data_i_start;
                    proximity_message_further =
                        proximity_message_further + further_talk_data_i_start;
                    proximity_message_far = proximity_message_far + far_talk_data_i_start;
                } else {
                    proximity_message_nearby =
                        proximity_message_nearby + nearby_talk_data_normal_start;
                    proximity_message_further =
                        proximity_message_further + further_talk_data_normal_start;
                    proximity_message_far = proximity_message_far + far_talk_data_normal_start;
                }

                if matches!(talk_style_variation, TalkStyleVariant::Shouts) {
                    if exclusive_proximity == false {
                        proximity_message_nearby = proximity_message_nearby
                            + nearby_shout_data_i_start
                            + &message
                            + nearby_shout_data_i_end;
                        proximity_message_further = proximity_message_further
                            + further_shout_data_i_start
                            + &message
                            + further_shout_data_i_end;
                        proximity_message_far = proximity_message_far
                            + far_shout_data_i_start
                            + &message
                            + far_shout_data_i_end;
                    } else {
                        proximity_message_nearby = proximity_message_nearby
                            + nearby_talk_data_b_start
                            + "[font="
                            + NEARBY_SHOUT_FONT
                            + "]\""
                            + &message
                            + "\"[/font]"
                            + nearby_talk_data_b_end;
                        proximity_message_further = proximity_message_further
                            + further_talk_data_b_start
                            + "[font="
                            + FURTHER_SHOUT_FONT
                            + "]\""
                            + &message
                            + "\"[/font]"
                            + further_talk_data_b_end;
                        proximity_message_far = proximity_message_far
                            + far_talk_data_b_start
                            + "[font="
                            + FAR_SHOUT_FONT
                            + "]\""
                            + &message
                            + "\"[/font]"
                            + far_talk_data_b_end;
                    }
                } else {
                    proximity_message_nearby = proximity_message_nearby
                        + nearby_talk_data_start
                        + "\""
                        + &message
                        + "\""
                        + nearby_talk_data_end;
                    proximity_message_further = proximity_message_further
                        + further_talk_data_start
                        + "\""
                        + &message
                        + "\""
                        + further_talk_data_end;
                    proximity_message_far = proximity_message_far
                        + far_talk_data_start
                        + "\""
                        + &message
                        + "\""
                        + far_talk_data_end;
                }

                if exclusive_proximity == false {
                    proximity_message_nearby = proximity_message_nearby + nearby_talk_data_i_end;
                    proximity_message_further = proximity_message_further + further_talk_data_i_end;
                    proximity_message_far = proximity_message_far + far_talk_data_i_end;
                } else {
                    proximity_message_nearby =
                        proximity_message_nearby + nearby_talk_data_normal_end;
                    proximity_message_further =
                        proximity_message_further + further_talk_data_normal_end;
                    proximity_message_far = proximity_message_far + far_talk_data_normal_end;
                }
            } else {
                proximity_message_nearby = proximity_message_nearby + &message;
                proximity_message_further = proximity_message_further + &message;
                proximity_message_far = proximity_message_far + &message;
            }

            proximity_message_nearby = proximity_message_nearby + TALK_SPACE_PROXIMITY_MESSAGEBBEND;
            proximity_message_further =
                proximity_message_further + TALK_SPACE_PROXIMITY_MESSAGEBBEND;
            proximity_message_far = proximity_message_far + TALK_SPACE_PROXIMITY_MESSAGEBBEND;
        }

        // Todo...
        // Create & send proximity billboard message.

        let mut billboard_message = "".to_string();

        if new_message.exclusive_radio == false {
            billboard_message = billboard_message + BILLBOARD_DATA_SECURITY_START;

            let nearby_talk_data_i_start;
            let nearby_talk_data_i_end;
            let nearby_talk_data_b_start;
            let nearby_talk_data_b_end;

            match communicator {
                Communicator::Standard => {
                    nearby_talk_data_i_start = TALK_DATA_STANDARD_I_NEARBY_START;
                    nearby_talk_data_i_end = TALK_DATA_STANDARD_I_NEARBY_END;

                    nearby_talk_data_b_start = TALK_DATA_STANDARD_B_NEARBY_START;
                    nearby_talk_data_b_end = TALK_DATA_STANDARD_B_NEARBY_END;
                }
                Communicator::Machine => {
                    nearby_talk_data_i_start = TALK_DATA_MACHINE_I_NEARBY_START;
                    nearby_talk_data_i_end = TALK_DATA_STANDARD_I_NEARBY_END;

                    nearby_talk_data_b_start = TALK_DATA_STANDARD_B_NEARBY_START;
                    nearby_talk_data_b_end = TALK_DATA_STANDARD_B_NEARBY_END;
                }
            }

            if exclusive_proximity == false {
                billboard_message = billboard_message + nearby_talk_data_i_start;
            }

            if matches!(talk_style_variation, TalkStyleVariant::Shouts) {
                billboard_message = billboard_message + nearby_talk_data_b_start;

                if exclusive_proximity == false {
                    billboard_message = billboard_message
                        + "[font="
                        + BILLBOARD_SHOUT_FONT
                        + "]"
                        + &message
                        + "[/font]";
                } else {
                    billboard_message = billboard_message
                        + "[font="
                        + BILLBOARD_SHOUT_ITALIC_FONT
                        + "]"
                        + &message
                        + "[/font]";
                }

                billboard_message = billboard_message + nearby_talk_data_b_end;
            } else {
                if is_emote {
                    billboard_message = billboard_message + nearby_talk_data_i_start;
                    billboard_message = billboard_message + "*" + &message + "*";
                    billboard_message = billboard_message + nearby_talk_data_i_end;
                } else {
                    billboard_message = billboard_message + &message;
                }
            }

            if exclusive_proximity == false {
                billboard_message = billboard_message + nearby_talk_data_i_end;
            }

            billboard_message = billboard_message + BILLBOARD_DATA_SECURITY_END;
        }

        // Send radio message to all radio_pawns who can listen to that channel.

        let mut handles_direct_proximity: Vec<u64> = vec![];
        let mut handles_radio: Vec<u64> = vec![];

        if exclusive_proximity == false {
            let mut has_radio_permission = false;

            match new_message.messenger_entity_option {
                Some(entity) => {
                    let messenger_components_result = radio_pawns.get(entity);

                    match messenger_components_result {
                        Ok((
                            _entity,
                            radio_component,
                            _rigid_body_handle_component,
                            _persistent_player_data_component,
                        )) => {
                            if radio_component.speak_access.contains(&radio_channel) {
                                has_radio_permission = true;
                            }
                        }
                        Err(_rr) => {
                            return;
                        }
                    }
                }
                None => {
                    has_radio_permission = true;
                }
            }

            if has_radio_permission {
                for (
                    entity,
                    radio_component,
                    _rigid_body_handle_component,
                    _persistent_player_data_component,
                ) in radio_pawns.iter()
                {
                    if radio_component.listen_access.contains(&radio_channel) {
                        let listener_handle_result = handle_to_entity.inv_map.get(&entity);
                        match listener_handle_result {
                            Some(listener_handle) => {
                                server.send(OutgoingReliableServerMessage {
                                    handle: *listener_handle,
                                    message: NetworkingChatServerMessage::ChatMessage(
                                        radio_message.clone(),
                                    ),
                                });
                                handles_radio.push(*listener_handle);
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        if new_message.exclusive_radio == false {
            // Proximity messages to listeners based on distance and shouting.
            let mut sensed_by_list: Vec<Entity>;

            if matches!(talk_style_variation, TalkStyleVariant::Shouts) {
                sensed_by_list = vec![];

                match new_message.messenger_entity_option {
                    Some(entity) => match player_pawns.get(entity) {
                        Ok((_, _transform, sensable)) => {
                            for s in sensable.sensed_by.iter() {
                                sensed_by_list.push(*s);
                            }
                            for s in sensable.sensed_by_cached.iter() {
                                sensed_by_list.push(*s);
                            }
                        }
                        Err(_) => {
                            warn!("chat couldnt find player pawn.");
                            continue;
                        }
                    },
                    None => {
                        warn!("Couldnt create proximity message as no messenger was passed.");
                        continue;
                    }
                }
            } else {
                match new_message.messenger_entity_option {
                    Some(entity) => match player_pawns.get(entity) {
                        Ok((_, _transform, sensable)) => {
                            sensed_by_list = sensable.sensed_by.to_vec();
                        }
                        Err(_) => {
                            warn!("chat couldnt find player pawn.");
                            continue;
                        }
                    },
                    None => {
                        warn!("No messenger entity option passed!");
                        continue;
                    }
                }
            }

            // Build billboard entity_update
            let mut billboard_entity_update = HashMap::new();

            let mut parameters_entity_update = HashMap::new();

            parameters_entity_update.insert(
                "billboardMessage".to_string(),
                EntityUpdateData::String(billboard_message.clone()),
            );

            billboard_entity_update.insert(
                "Smoothing/pawn/humanMale/textViewPortChat0/ViewPort/chatText".to_string(),
                parameters_entity_update,
            );

            for entity in sensed_by_list {
                let sensed_by_entity_components_result = radio_pawns.get(entity);

                match sensed_by_entity_components_result {
                    Ok((
                        entity,
                        _radio_component,
                        rigid_body_position,
                        _persistent_player_data_component,
                    )) => {
                        let listener_handle_result = handle_to_entity.inv_map.get(&entity);

                        match listener_handle_result {
                            Some(listener_handle) => {
                                let listener_rigid_body_transform = rigid_body_position.translation;
                                let listener_position = Vec3::new(
                                    listener_rigid_body_transform.x,
                                    listener_rigid_body_transform.y,
                                    listener_rigid_body_transform.z,
                                );

                                let listener_distance = position.distance(listener_position);

                                let distance;

                                if listener_distance > 24. {
                                    distance = Distance::Far;
                                } else if listener_distance > 14. {
                                    distance = Distance::Further;
                                } else {
                                    distance = Distance::Nearby;
                                }

                                match distance {
                                    Distance::Nearby => {
                                        server.send(OutgoingReliableServerMessage {
                                            handle: *listener_handle,
                                            message: NetworkingChatServerMessage::ChatMessage(
                                                proximity_message_nearby.clone(),
                                            ),
                                        });
                                    }
                                    Distance::Further => {
                                        server.send(OutgoingReliableServerMessage {
                                            handle: *listener_handle,
                                            message: NetworkingChatServerMessage::ChatMessage(
                                                proximity_message_further.clone(),
                                            ),
                                        });
                                    }
                                    Distance::Far => {
                                        server.send(OutgoingReliableServerMessage {
                                            handle: *listener_handle,
                                            message: NetworkingChatServerMessage::ChatMessage(
                                                proximity_message_far.clone(),
                                            ),
                                        });
                                    }
                                }

                                match new_message.send_entity_update {
                                    true => match new_message.messenger_entity_option {
                                        Some(ent) => {
                                            server1.send(OutgoingReliableServerMessage {
                                                handle: *listener_handle,
                                                message: EntityServerMessage::EntityUpdate(
                                                    ent.to_bits(),
                                                    billboard_entity_update.clone(),
                                                    false,
                                                    EntityWorldType::Main,
                                                ),
                                            });
                                            handles_direct_proximity.push(*listener_handle);
                                        }
                                        None => {
                                            warn!("Couldnt find messenger entity.!");
                                        }
                                    },
                                    false => {
                                        error!("Cannot send proximity message without providing EventWriter<NetSendEntityUpdates>.");
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                    Err(_rr) => {}
                }
            }
        }

        for player_handle in handles_direct_proximity.iter() {
            server2.send(OutgoingReliableServerMessage {
                handle: *player_handle,
                message: PlaySoundProximityMessageData::get_message(position),
            });
        }

        for player_handle in handles_radio.iter() {
            if !handles_direct_proximity.contains(player_handle) {
                server2.send(OutgoingReliableServerMessage {
                    handle: *player_handle,
                    message: PlaySoundRadioMessage::get_message(),
                });
            }
        }
    }
}

/// Requested proximity message event.
#[cfg(feature = "server")]
pub struct EntityProximityMessage {
    pub entities: Vec<Entity>,
    pub message: String,
}

/// Requested entity proximity messages systems ordering label.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "server")]
pub enum EntityProximityMessages {
    Send,
}

/// Send entity proximity messages to receivers.
#[cfg(feature = "server")]
pub(crate) fn send_entity_proximity_messages(
    mut entity_proximity_messages: EventReader<EntityProximityMessage>,
    sensers: Query<(Entity, &Senser)>,
    positions: Query<&Transform>,
    handle_to_entity: Res<HandleToEntity>,
    mut server: EventWriter<OutgoingReliableServerMessage<NetworkingChatServerMessage>>,
) {
    for entity_proximity_message in entity_proximity_messages.iter() {
        for proximity_entity in entity_proximity_message.entities.iter() {
            let entity_transform;

            match positions.get(*proximity_entity) {
                Ok(t) => {
                    entity_transform = t;
                }
                Err(_rr) => {
                    warn!("Couldnt find transform of entity");
                    continue;
                }
            }

            let entity_gridmap_coords = world_to_cell_id(entity_transform.translation);
            let entity_cell_id_doryen =
                to_doryen_coordinates(entity_gridmap_coords.x, entity_gridmap_coords.z);

            for (entity, senser_component) in sensers.iter() {
                if senser_component.fov.is_in_fov(
                    entity_cell_id_doryen.0 as usize,
                    entity_cell_id_doryen.1 as usize,
                ) {
                    match handle_to_entity.inv_map.get(&entity) {
                        Some(handle) => {
                            server.send(OutgoingReliableServerMessage {
                                handle: *handle,
                                message: NetworkingChatServerMessage::ChatMessage(
                                    entity_proximity_message.message.clone(),
                                ),
                            });
                        }
                        None => {}
                    }
                }
            }
        }
    }
}
