use bevy::{
    prelude::*,
    ui::{Style, Val, JustifyContent, AlignItems}, math::{Size, Rect}, text::{Text, TextStyle}
};

use super::{ClientState, ConnectingState};

pub fn add_state(app: &mut App){
    app
        .add_system_set(
            SystemSet::on_enter(ClientState::NotConnected)
                .with_system(initialize_ui)
        )
        .add_system_set(
            SystemSet::on_update(ClientState::NotConnected)
                .with_system(handle_connect_ui)
        )
        .add_system_set(
            SystemSet::on_exit(ClientState::NotConnected)
                .with_system(cleanup_connect_ui)
        );
}

#[derive(Component)]
struct ConnectButton{
    clicked: bool,
}

/// Creates the button to connect to the server with
fn initialize_ui(
    mut commands: Commands,
    font: Res<Handle<Font>>,
){
    
    commands
        .spawn_bundle(ButtonBundle {
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
            color: Color::rgb(0.2, 0.9, 0.2).into(),
            ..Default::default()
        })
        .insert(ConnectButton{clicked: false})
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Connect",
                    TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.1, 0.1, 0.1),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

/// Handle the clicking on, and connection to the server
fn handle_connect_ui(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &mut ConnectButton),
        Changed<Interaction>,
    >,
    mut state: ResMut<State<ClientState>>
){
    for (interaction, mut color, mut button_state) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.1, 0.6, 0.1).into();
                button_state.clicked = true;
            }
            Interaction::Hovered if !button_state.clicked  => {
                *color = Color::rgb(0.3, 1.0, 0.3).into();
            }
            Interaction::None if !button_state.clicked  => {
                *color = Color::rgb(0.2, 0.9, 0.2).into();
            },
            _ =>{
                state.set(ClientState::Connecting(ConnectingState::NotConnected)).unwrap();
            }
        }
    }
}

/// Cleanup the UI 
fn cleanup_connect_ui(
    mut commands: Commands,
    connect_button: Query<(Entity, &Children), With<ConnectButton>>,
){
    let (button, children) = connect_button.single();
    commands
        .entity(button)
        .despawn();
    for child in children.iter(){
        commands
            .entity(*child)
            .despawn();
    }
}