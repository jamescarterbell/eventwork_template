use std::{time::Duration, net::{SocketAddr, IpAddr, Ipv4Addr}, str::FromStr};
use bevy::{prelude::*, app::ScheduleRunnerSettings, log::{Level, LogSettings}};
use net::to_client::client_register_network_messages;
use states::add_client_states;
use uuid::Uuid;

mod states;

fn main(){
    let mut args = std::env::args();
    args.next();
    let name = args.next().unwrap();

    let mut app = App::new();
    app
        .insert_resource(LogSettings {
            level: Level::INFO,
            filter: "wgpu=error".to_string(),
        });
    app.add_plugins(DefaultPlugins);
    app.init_resource::<bevy::tasks::TaskPool>();

    // Before we can register the potential message types, we
    // need to add the plugin
    app.add_plugin(bevy_eventwork::ClientPlugin::<bevy_eventwork::tcp::TcpClientProvider>::default());
    app.add_startup_system(general_setup);

    // Temps
    app.insert_resource(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));
    app.insert_resource(ClientInfo{
        id: Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes()),
        name,
    });
    app.insert_resource(bevy_eventwork::tcp::NetworkSettings{
        max_packet_length: 1024 * 1024,
        addr: SocketAddr::from_str("127.0.0.1:7777").unwrap()
    });

    client_register_network_messages::<bevy_eventwork::tcp::TcpClientProvider>(&mut app);
    add_client_states::<bevy_eventwork::tcp::TcpClientProvider, bevy::tasks::TaskPool>(&mut app);

    app.run();
}

fn general_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(PerspectiveCameraBundle::default());
    commands.insert_resource(asset_server.load::<Font, &str>("fonts/RobotoCondensed-Regular.ttf"));
}

pub struct ClientInfo{
    pub id: Uuid,
    pub name: String
}