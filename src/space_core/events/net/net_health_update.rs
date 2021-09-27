use crate::space_core::resources::network_messages::ReliableServerMessage;

pub struct NetHealthUpdate {
    pub handle : u32,
    pub message : ReliableServerMessage
}
