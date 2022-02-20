use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bevy_spicy_networking::{ClientMessage, NetworkMessage, ServerMessage, AppNetworkClientMessage, AppNetworkServerMessage};
use bevy::prelude::*;

pub fn shared_client_register_network_messages(app: &mut App){
    app
        .listen_for_client_message::<NewPlayerJoined>()
        .listen_for_client_message::<ReadyUpResponse>();
}

pub fn shared_server_register_network_messages(app: &mut App){
    app
        .listen_for_server_message::<NewPlayerJoined>()
        .listen_for_server_message::<ReadyUpResponse>();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewPlayerJoined{
    pub id: Uuid,
}

#[typetag::serde]
impl NetworkMessage for NewPlayerJoined{}


impl ServerMessage for NewPlayerJoined{
    const NAME: &'static str = "request:NewPlayer";
}

impl ClientMessage for NewPlayerJoined{
    const NAME: &'static str = "response:NewPlayer";
}


#[derive(Serialize, Deserialize, Clone)]
pub struct ReadyUpResponse{
    pub id: Uuid,
    pub ready: bool,
}

#[typetag::serde]
impl NetworkMessage for ReadyUpResponse{}

impl ClientMessage for ReadyUpResponse{
    const NAME: &'static str = "ReadyUpResponse";
}

impl ServerMessage for ReadyUpResponse{
    const NAME: &'static str = "ReadyUpResponse";
}
