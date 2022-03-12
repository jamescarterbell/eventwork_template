use std::{time::Duration, net::{SocketAddr, IpAddr, Ipv4Addr}, str::FromStr};
use bevy::{prelude::*, app::ScheduleRunnerSettings};
use net::to_server::server_register_network_messages;
use states::add_server_states;

mod states;

fn main(){
    let mut app = App::new();
    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )));
    app.add_plugins(MinimalPlugins);
    app.add_plugin(bevy::log::LogPlugin::default());
    app.init_resource::<bevy::tasks::TaskPool>();

    // Temp
    app.insert_resource(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));

    // Before we can register the potential message types, we
    // need to add the plugin
    app.add_plugin(bevy_eventwork::ServerPlugin::<bevy_eventwork::tcp::TcpServerProvider>::default());
    app.insert_resource(bevy_eventwork::tcp::NetworkSettings{
        max_packet_length: 1024 * 1024,
        addr: SocketAddr::from_str("127.0.0.1:7777").unwrap()
    });

    server_register_network_messages::<bevy_eventwork::tcp::TcpServerProvider>(&mut app);
    add_server_states::<bevy_eventwork::tcp::TcpServerProvider, bevy::tasks::TaskPool>(&mut app);

    app.run();
}