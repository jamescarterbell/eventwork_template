use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bevy_spicy_networking::{ServerMessage, NetworkMessage, AppNetworkServerMessage};
use bevy::prelude::*;
use crate::shared::shared_server_register_network_messages;

pub fn server_register_network_messages(app: &mut App){
    shared_server_register_network_messages(app);
    app
        .listen_for_server_message::<ReadyUpRequest>();
}

#[derive(Serialize, Deserialize)]
pub struct ReadyUpRequest{
    pub ready: bool,
}

#[typetag::serde]
impl NetworkMessage for ReadyUpRequest{}


impl ServerMessage for ReadyUpRequest{
    const NAME: &'static str = "ReadyUpRequest";
}