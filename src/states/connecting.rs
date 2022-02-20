
use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_spicy_networking::{NetworkClient, NetworkSettings, ClientNetworkEvent, NetworkData};
use net::shared::NewPlayerJoined;
use uuid::Uuid;

use crate::ClientInfo;

use super::{ClientState, ConnectingState, common::PlayerMap, ReadyState};

pub fn add_state(app: &mut App){
    app
        .add_system_set(
            SystemSet::on_enter(ClientState::Connecting(ConnectingState::NotConnected))
                .with_system(initialize_connecting)
                .with_system(initialize_ui)
        )
        .add_system_set(
            SystemSet::on_update(ClientState::Connecting(ConnectingState::NotConnected))
                .with_system(handle_not_connected_ui)
        )
        .add_system_set(
            SystemSet::on_enter(ClientState::Connecting(ConnectingState::Validating))
                .with_system(initialize_validating)
        )
        .add_system_set(
            SystemSet::on_update(ClientState::Connecting(ConnectingState::Validating))
                .with_system(handle_validation)
        )
        .add_system_set(
            SystemSet::on_exit(ClientState::Connecting(ConnectingState::Validating))
                .with_system(cleanup_connecting)
        );
}

#[derive(Component)]
struct ConnectingText;

fn initialize_connecting(
    mut net: ResMut<NetworkClient>,
    socket_addr: Res<SocketAddr>,
    network_settings: Res<NetworkSettings>,
){
    net.connect(
        *socket_addr,
        network_settings.clone()
    );
}

fn initialize_ui(
    mut commands: Commands,
    font: Res<Handle<Font>>,
){
    commands
        .spawn_bundle(
            TextBundle {
                text: Text::with_section(
                    "Connecting...",
                    TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: Rect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .insert(
            ConnectingText
        );
}

fn handle_not_connected_ui(
    mut network_events: EventReader<ClientNetworkEvent>,
    mut state: ResMut<State<ClientState>>,
){
    for event in network_events.iter(){
        match event{
            ClientNetworkEvent::Connected => state.set(ClientState::Connecting(ConnectingState::Validating)).unwrap(),
            _ => (),
        }
    }
}

fn initialize_validating(
    mut commands: Commands,
    mut connecting_text: Query<&mut Text, With<ConnectingText>>,
    font: Res<Handle<Font>>,
    net: Res<NetworkClient>,
    player_id: Res<ClientInfo>,
){
    let mut text = connecting_text.single_mut();

    text.sections[0].value = "Connected!".into();
    text.sections.push(
        TextSection{
            value: "Validating".into(),
            style: TextStyle {
                font: font.clone(),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        }
    );
    net.send_message(NewPlayerJoined{
        id: player_id.id.clone()
    }).unwrap();

    commands.insert_resource(PlayerMap::default());
}

fn handle_validation(
    mut state: ResMut<State<ClientState>>,
    mut new_players: EventReader<NetworkData<NewPlayerJoined>>,
    mut players: ResMut<PlayerMap>,
){
    let mut change = false;
    for new_player in new_players.iter(){
        players.insert(new_player.id);
        change = true;
    }

    if change{
        state.set(ClientState::ReadyUp(ReadyState::NotReady)).unwrap();
    }
}

fn cleanup_connecting(
    mut commands: Commands,
    connecting_text: Query<Entity, With<ConnectingText>>,
){
    let text = connecting_text.single();
    commands
        .entity(text)
        .despawn();
}