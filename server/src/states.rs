use bevy::prelude::App;

mod common;
mod lobby;

pub fn add_server_states(app: &mut App){
    app.add_state(ServerState::Lobby);
    common::add_common(app);
    lobby::add_state(app);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ServerState{
    Lobby,
    Launching,
    GameRunning
}