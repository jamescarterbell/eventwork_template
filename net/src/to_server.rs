use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bevy_eventwork::{ServerMessage, AppNetworkServerMessage, NetworkServerProvider};
use bevy::prelude::*;
use crate::shared::shared_server_register_network_messages;

pub fn server_register_network_messages<NSP: NetworkServerProvider>(app: &mut App){
    shared_server_register_network_messages::<NSP>(app);
    app
        .listen_for_server_message::<ReadyUpRequest, NSP>();
}

#[derive(Serialize, Deserialize)]
pub struct ReadyUpRequest{
    pub ready: bool,
}

impl ServerMessage for ReadyUpRequest{
    const NAME: &'static str = "ReadyUpRequest";
}