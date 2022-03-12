use bevy::prelude::App;
use bevy_eventwork::{NetworkClientProvider, Runtime};

mod common;
mod not_connected;
mod connecting;
mod ready_up;

pub fn add_client_states<NCP: NetworkClientProvider, RT: Runtime>(app: &mut App){
    app.add_state(ClientState::NotConnected);
    not_connected::add_state(app);
    connecting::add_state::<NCP, RT>(app);
    ready_up::add_state::<NCP>(app);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ClientState{
    NotConnected,
    Connecting(ConnectingState),
    ReadyUp(ReadyState),
    Launching,
    GameRunning
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ConnectingState{
    NotConnected,
    Validating
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ReadyState{
    NotReady,
    Ready
}