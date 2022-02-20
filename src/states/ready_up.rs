use std::ops::{Deref, DerefMut};

use bevy::{prelude::*, utils::HashMap, ecs::entity::Entities};
use bevy_spicy_networking::{NetworkData, NetworkClient};
use net::{shared::{ReadyUpResponse, NewPlayerJoined}, to_server, to_client::AllReady};
use uuid::Uuid;

use crate::ClientInfo;

use super::{ClientState, ReadyState, common::PlayerMap};

pub fn add_state(app: &mut App){
    app
        .add_system_set(
            SystemSet::on_enter(ClientState::ReadyUp(ReadyState::NotReady))
                .with_system(initialize_ui)
                .with_system(enter_not_ready)
        )
        .add_system_set(
            SystemSet::on_update(ClientState::ReadyUp(ReadyState::NotReady))
                .with_system(handle_not_ready_button)
                .with_system(handle_new_players)
                .with_system(handle_ready)
                .with_system(handle_all_ready)
        )
        .add_system_set(
            SystemSet::on_exit(ClientState::ReadyUp(ReadyState::NotReady))
                .with_system(cleanup_ui)
        )
        .add_system_set(
            SystemSet::on_enter(ClientState::ReadyUp(ReadyState::Ready))
                .with_system(initialize_ui)
                .with_system(enter_ready)
        )
        .add_system_set(
            SystemSet::on_update(ClientState::ReadyUp(ReadyState::Ready))
                .with_system(handle_ready_button)
                .with_system(handle_ready)
                .with_system(handle_all_ready)
        )
        .add_system_set(
            SystemSet::on_exit(ClientState::ReadyUp(ReadyState::Ready))
                .with_system(cleanup_ui)
        );
}

#[derive(Default)]
struct ReadyMap{
    readies: HashMap<Uuid, bool>,
}

impl Deref for ReadyMap{
    type Target = HashMap<Uuid, bool>;

    fn deref(&self) -> &Self::Target {
        &self.readies
    }
}

impl DerefMut for ReadyMap{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.readies
    }
}

#[derive(Component)]
struct ReadyButton{
    clicked: bool,
}

#[derive(Component)]
struct ReadyList;

#[derive(Component)]
struct UiContainer;

fn initialize_ui(
    mut commands: Commands,
    ready_map_res: Option<Res<ReadyMap>>,
    font: Res<Handle<Font>>,
    players: Res<PlayerMap>,
){
    let mut text = Text::default();
    match ready_map_res{
        Some(map) => for (id, _) in map.iter(){
            text.sections.push(
                TextSection{
                    value: id.to_string(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.1, 0.1),
                    }
                }
            );
        },
        None => {
            let mut ready_map = ReadyMap::default();
            for id in players.iter(){
                ready_map.insert(*id, false);
            }
            for (id, _) in ready_map.iter(){
                text.sections.push(
                    TextSection{
                        value: id.to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.1, 0.1),
                        }
                    }
                );
            }
            commands.insert_resource(ready_map);
        }
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(UiContainer)
        .with_children(|parent|{
            parent
                .spawn_bundle(TextBundle{
                    text,
                    ..Default::default()
                })
                .insert(ReadyList);
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Percent(0.25)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgb(0.1, 0.9, 0.1).into(),
                    ..Default::default()
                })
                .insert(ReadyButton{clicked: false})
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Ready",
                            TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
}

fn handle_ready(
    mut readies: EventReader<NetworkData<ReadyUpResponse>>,
    mut ready_map: ResMut<ReadyMap>,
    mut ready_ui: Query<&mut Text, With<ReadyList>>,
    font: Res<Handle<Font>>,
    asset_server: Res<AssetServer>,
    player: Res<ClientInfo>,
){
    for ready in readies.iter(){
        if ready.id == player.id{
            continue;
        }
        match ready_map.entry(ready.id){
            std::collections::hash_map::Entry::Occupied(mut o) => {*o.get_mut() = ready.ready;},
            std::collections::hash_map::Entry::Vacant(v) => {v.insert(ready.ready);},
        }
    }

    let mut ready_ui = ready_ui.single_mut();
    let mut text = Vec::new();

    for (id, ready) in ready_map.iter(){
        let color = match *ready{
            true => Color::rgb(0.1, 0.9, 0.1),
            false => Color::rgb(0.9, 0.1, 0.1)
        };
        text.push(
            TextSection{
                value: id.to_string(),
                style: TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color,
                }
            }
        );
    }

    ready_ui.sections = text;
}

fn handle_new_players(
    mut player_joined: EventReader<NetworkData<NewPlayerJoined>>,
    mut ready_map: ResMut<ReadyMap>,
){
    for player in player_joined.iter(){
        ready_map.readies.insert(player.id, false);
    }
}

fn handle_not_ready_button(
    mut state: ResMut<State<ClientState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children, &mut ReadyButton),
        Changed<Interaction>,
    >,
    mut ready_map: ResMut<ReadyMap>,
    info: Res<ClientInfo>,
){
    for (interaction, mut color, children, mut button_state) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.1, 0.6, 0.1).into();
                button_state.clicked = true;
            }
            Interaction::Hovered if !button_state.clicked => {
                *color = Color::rgb(0.3, 1.0, 0.3).into();
            }
            Interaction::None if !button_state.clicked => {
                *color = Color::rgb(0.1, 0.9, 0.1).into();
            },
            _ =>{
                button_state.clicked = false;
                state.set(ClientState::ReadyUp(ReadyState::Ready)).unwrap();
                *ready_map.get_mut(&info.id).unwrap() = true;
            }
        }
    }
}


fn handle_ready_button(
    mut state: ResMut<State<ClientState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children, &mut ReadyButton),
        Changed<Interaction>,
    >,
    mut ready_map: ResMut<ReadyMap>,
    info: Res<ClientInfo>,
){
    for (interaction, mut color, children, mut button_state) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.6, 0.1, 0.1).into();
                button_state.clicked = true;
            }
            Interaction::Hovered if !button_state.clicked => {
                *color = Color::rgb(1.0, 0.3, 0.3).into();
            }
            Interaction::None if !button_state.clicked  => {
                *color = Color::rgb(0.9, 0.1, 0.1).into();
            },
            _ =>{
                button_state.clicked = false;
                state.set(ClientState::ReadyUp(ReadyState::NotReady)).unwrap();
                *ready_map.get_mut(&info.id).unwrap() = false;
            }
        }
    }
}

fn enter_ready(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children, &mut ReadyButton),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    info: Res<ClientInfo>,
    net: Res<NetworkClient>
){
    for (interaction, mut color, children, mut button_state) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        text.sections[0].value = "Unready".into();
        *color = Color::rgb(0.9, 0.1, 0.1).into();
    }
    net.send_message(to_server::ReadyUpRequest{
        ready: true,
    }).unwrap();
}

fn enter_not_ready(
    mut interaction_query: Query<
        (&mut UiColor, &Children),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    info: Res<ClientInfo>,
    net: Res<NetworkClient>
){
    for (mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        text.sections[0].value = "Ready".into();
        *color = Color::rgb(0.1, 0.9, 0.1).into();
    }
    net.send_message(to_server::ReadyUpRequest{
        ready: false,
    }).unwrap();
}

fn handle_all_ready(
    mut ready: EventReader<NetworkData<AllReady>>,
    mut state: ResMut<State<ClientState>>,
){
    if let Some(_) = ready.iter().next(){
        state.set(ClientState::Launching);
    }
}

fn cleanup_ui(
    mut commands: Commands,
    state: Res<State<ClientState>>,
    entities: Query<(Entity, Option<&Children>), Or<(With<ReadyButton>, With<ReadyList>, With<UiContainer>)>>,
){
    for (entity, children) in entities.iter(){
        commands
            .entity(entity)
            .despawn();
        if let Some(children) = children{
            for child in children.iter(){
                commands
                    .entity(*child)
                    .despawn();
            }
        }
    }
}