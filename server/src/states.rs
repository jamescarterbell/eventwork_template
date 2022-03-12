use bevy::prelude::App;
use bevy_eventwork::{NetworkServerProvider, Runtime};

mod common;
mod lobby;

pub fn add_server_states<NSP: NetworkServerProvider, RT: Runtime>(app: &mut App){
    app.add_state(ServerState::Lobby);
    common::add_common(app);
    lobby::add_state::<NSP, RT>(app);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ServerState{
    Lobby,
    Launching,
    GameRunning
}