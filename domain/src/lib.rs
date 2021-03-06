use serde::{Deserialize, Serialize};
use tdn_did::Proof;
use tdn_types::{group::GroupId, primitive::PeerId};

// Same ID can has many name !.

/// Group chat app(service) default TDN GROUP ID.
#[rustfmt::skip]
pub const DOMAIN_ID: GroupId = GroupId([
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 4,
]);

/// ESSE domain service layer Event.
#[derive(Serialize, Deserialize)]
pub struct LayerServerEvent(pub ServerEvent, pub Proof);

/// ESSE domain service layer Event.
#[derive(Serialize, Deserialize)]
pub struct LayerPeerEvent(pub PeerEvent, pub Proof);

/// ESSE domain service to peer layer Event.
#[derive(Serialize, Deserialize)]
pub enum ServerEvent {
    /// check result status.
    /// params: provider name, is support request proxy.
    Status(String, bool),
    /// register result.
    /// params: name, is_ok.
    Result(String, bool),
    /// a identity info.
    /// params: user_name, user_ID, user_address, user_bio, user_avatar.
    Info(String, GroupId, PeerId, String, Vec<u8>),
    /// not found a user by name.
    None(String),
    /// current name is active.
    /// params: name, is_actived
    Actived(String, bool),
    /// current name is deleted.
    /// params: name.
    Deleted(String),
    /// response the make friend.
    /// params: remote_ID, name, is_ok.
    Response(GroupId, String, bool),
}

/// ESSE domain peer to service layer Event.
#[derive(Serialize, Deserialize)]
pub enum PeerEvent {
    /// check service status is ok.
    Check,
    /// register new unique identity to service.
    /// params: name, bio, avatar.
    Register(String, String, Vec<u8>),
    /// update user info.
    /// params: name, bio, avatar.
    Update(String, String, Vec<u8>),
    /// search a identity info.
    /// params: name.
    Search(String),
    /// make a friend request,
    /// params: remote_name, my_name, request_remark.
    Request(String, String, String),
    /// suspend the name.
    /// params: name.
    Suspend(String),
    /// active the name.
    /// params: name.
    Active(String),
    /// delete the name.
    /// params: name.
    Delete(String),
}
