use api::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};
use networking_macros::NetMessage;
#[derive(NetMessage)]
pub(crate) struct NetAtmosphericsNotices {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetMapHoverAtmospherics {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetMapDisplayAtmospherics {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
