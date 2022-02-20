use std::{net::SocketAddr, collections::hash_map::Entry};

use bevy::{prelude::*, utils::HashMap};
use bevy_spicy_networking::{NetworkServer, NetworkData};
use net::{ConnectionMap, to_server::ReadyUpRequest, shared::{NewPlayerJoined, self, ReadyUpResponse}, to_client::{AllReady, self}};
use uuid::Uuid;

use super::ServerState;


pub fn add_state(app: &mut App){
    app
        .add_system_set(
            SystemSet::on_enter(ServerState::Lobby)
                .with_system(initialize_server)
                .with_system(initialize_readies)
        )
        .add_system_set(
            SystemSet::on_update(ServerState::Lobby)
                .label("handle_new_players")
                .with_system(handle_new_players)
        )
        .add_system_set(
            SystemSet::on_update(ServerState::Lobby)
                .label("handle_readies")
                .after("handle_new_players")
                .with_system(handle_readies)
        );
}

struct ReadyMap{
    readies: HashMap<Uuid, bool>,
}

/// System to start listening for clients
pub fn initialize_server(
    mut net: ResMut<NetworkServer>,
    listen_addr: Res<SocketAddr>,
){
    net.listen(listen_addr.clone()).unwrap();
}

/// System to start listening for clients
pub fn initialize_readies(
    mut commands: Commands
){
    commands
        .insert_resource(
            ReadyMap{
                readies: HashMap::default()
            }
        );
}

/// System to handle a new player joining
/// this will probably set the player as a spectator by default
/// then let them change to a player.
fn handle_new_players(
    mut new_conns: EventReader<NetworkData<NewPlayerJoined>>,
    mut conn_map: ResMut<ConnectionMap>,
    mut net: ResMut<NetworkServer>,
    mut ready_map: ResMut<ReadyMap>,
){
    for new_conn in new_conns.iter(){
        conn_map.upgrade(new_conn.source(), new_conn.id);
        info!("New player: {} connected!", new_conn.source());
        for (id, ready) in ready_map.readies.iter(){
            info!("Sending {}, ready state of {}", new_conn.source(), id);
            net.send_message(new_conn.source(), shared::ReadyUpResponse{id: id.clone(), ready: *ready});
        }
        ready_map.readies.insert(new_conn.id, false);
        net.broadcast(NewPlayerJoined{id: new_conn.id});
    }
}

/// System to handle readys and forward readies to other players
fn handle_readies(
    mut readies: EventReader<NetworkData<ReadyUpRequest>>,
    conn_map: ResMut<ConnectionMap>,
    net: Res<NetworkServer>,
    mut ready_map: ResMut<ReadyMap>,
    mut state: ResMut<State<ServerState>>
){
    let mut changed = HashMap::default();
    for ready in readies.iter(){
        match changed.entry(conn_map.by_connection(&ready.source()).unwrap()){
            Entry::Occupied(mut e) => *e.get_mut() = ready.ready,
            Entry::Vacant(e) => {e.insert(ready.ready);},
        }
    }

    for (id, change) in changed.iter(){
        ready_map.readies.insert(*id, *change);
    }

    if !ready_map
        .readies
        .iter()
        .map(|(id, ready)| ready)
        .any(|r| !*r)&&
        ready_map.readies.len() > 0{
        net.broadcast(AllReady); 
        state.set(ServerState::Launching);
        return;
    }

    for (id, change) in changed{
        net.broadcast(ReadyUpResponse{id, ready: change});
    }
}