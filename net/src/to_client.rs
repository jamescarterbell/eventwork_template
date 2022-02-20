use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bevy_spicy_networking::{ClientMessage, NetworkMessage, AppNetworkClientMessage};
use bevy::prelude::*;
use crate::shared::shared_client_register_network_messages;

pub fn client_register_network_messages(app: &mut App){
    shared_client_register_network_messages(app);
    app
        .listen_for_client_message::<AllReady>();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AllReady;

#[typetag::serde]
impl NetworkMessage for AllReady{}

impl ClientMessage for AllReady{
    const NAME: &'static str = "notify:AllReady";
}