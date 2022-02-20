use std::ops::{Deref, DerefMut};

use bevy::{utils::HashSet, prelude::{App, EventReader, CoreStage, ResMut}};
use bevy_spicy_networking::NetworkData;
use net::{ConnectionMap, shared::NewPlayerJoined};
use uuid::Uuid;

pub fn add_common(app: &mut App){
    app.init_resource::<ConnectionMap>();
    app.add_system_to_stage(CoreStage::PreUpdate, handle_new_connections);
}

#[derive(Default)]
pub struct PlayerMap{
    players: HashSet<Uuid>,
}

impl Deref for PlayerMap{
    type Target = HashSet<Uuid>;

    fn deref(&self) -> &Self::Target {
        &self.players
    }
}

impl DerefMut for PlayerMap{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.players
    }
}

/// Handle incoming connections
fn handle_new_connections(
    mut network_events: EventReader<NetworkData<NewPlayerJoined>>,
    mut connection_map: ResMut<PlayerMap>,
){
    for event in network_events.iter(){
        connection_map.insert(event.id);
    }
}