use bevy::{utils::{HashMap, HashSet}, prelude::ResMut};
use bevy_eventwork::ConnectionId;
use uuid::Uuid;

pub mod shared;
pub mod to_server;
pub mod to_client;

#[derive(Default)]
pub struct ConnectionMap{
    conn_map: HashMap<ConnectionId, Uuid>,
    id_map: HashMap<Uuid, ConnectionId>,
    pending: HashSet<ConnectionId>,
}

impl ConnectionMap{
    pub fn add(&mut self, connection_id: ConnectionId, id: Uuid) -> Result<(), ()>{
        if let Some(old_id) = self.conn_map.insert(connection_id, id){
            self.conn_map.insert(connection_id, old_id);
            return Err(());
        }
        self.id_map.insert(id, connection_id);
        Ok(())
    }

    pub fn remove_by_connection(&mut self, connection_id: &ConnectionId) -> Result<(), ()>{
        let id =  match self.conn_map.remove(connection_id){
            Some(id) => id,
            None => return Err(()),
        };
        self.id_map.remove(&id);
        Ok(())
    }

    pub fn remove_by_id(&mut self, id: &Uuid) -> Result<(), ()>{
        let connection_id =  match self.id_map.remove(id){
            Some(id) => id,
            None => return Err(()),
        };
        self.conn_map.remove(&connection_id);
        Ok(())
    }

    pub fn by_connection(&self, connection_id: &ConnectionId) -> Option<Uuid>{
        self.conn_map.get(connection_id).map(|id| id.clone())
    }

    pub fn by_id(&self, id: &Uuid) -> Option<ConnectionId>{
        self.id_map.get(id).map(|conn| conn.clone())
    }

    pub fn add_pending(&mut self, connection_id: ConnectionId) -> Result<(), ()>{
        if self.pending.insert(connection_id){
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn remove_pending(&mut self, connection_id: &ConnectionId) -> Result<(), ()>{
        if self.pending.remove(&connection_id){
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn upgrade(&mut self, connection_id: ConnectionId, id: Uuid) -> Result<(), ()>{
        if self.pending.remove(&connection_id){
            self.add(connection_id, id)?;
            Ok(())
        } else {
            Err(())
        }
    }
}