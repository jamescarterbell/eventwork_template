use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bevy_eventwork::{ClientMessage, AppNetworkClientMessage, NetworkClientProvider};
use bevy::prelude::*;
use crate::shared::shared_client_register_network_messages;

pub fn client_register_network_messages<NCP: NetworkClientProvider>(app: &mut App){
    shared_client_register_network_messages::<NCP>(app);
    app
        .listen_for_client_message::<AllReady, NCP>();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AllReady;

impl ClientMessage for AllReady{
    const NAME: &'static str = "notify:AllReady";
}