use bevy::prelude::*;
use bevy_spicy_networking::ServerNetworkEvent;
use net::ConnectionMap;


pub fn add_common(app: &mut App){
    app.init_resource::<ConnectionMap>();
    app.add_system_to_stage(CoreStage::PreUpdate, handle_new_connections);
}

/// Handle incoming connections
fn handle_new_connections(
    mut network_events: EventReader<ServerNetworkEvent>,
    mut connection_map: ResMut<ConnectionMap>,
){
    for event in network_events.iter(){
        match event{
            ServerNetworkEvent::Connected(conn_id) => {
                connection_map.add_pending(*conn_id);
            },
            ServerNetworkEvent::Disconnected(conn_id) => {
                if let Err(_) = connection_map.remove_by_connection(conn_id){
                    connection_map.remove_pending(conn_id);
                }
            },
            _ => (),
        }
    }
}