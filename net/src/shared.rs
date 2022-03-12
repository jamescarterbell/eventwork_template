use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bevy_eventwork::{ClientMessage, ServerMessage, AppNetworkClientMessage, AppNetworkServerMessage, NetworkClientProvider, NetworkServerProvider};
use bevy::prelude::*;

pub fn shared_client_register_network_messages<NCP: NetworkClientProvider>(app: &mut App){
    app
        .listen_for_client_message::<NewPlayerJoined, NCP>()
        .listen_for_client_message::<ReadyUpResponse, NCP>();
}

pub fn shared_server_register_network_messages<NSP: NetworkServerProvider>(app: &mut App){
    app
        .listen_for_server_message::<NewPlayerJoined, NSP>()
        .listen_for_server_message::<ReadyUpResponse, NSP>();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewPlayerJoined{
    pub id: Uuid,
}

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

impl ClientMessage for ReadyUpResponse{
    const NAME: &'static str = "ReadyUpResponse";
}

impl ServerMessage for ReadyUpResponse{
    const NAME: &'static str = "ReadyUpResponse";
}
