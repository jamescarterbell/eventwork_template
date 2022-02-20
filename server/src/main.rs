use std::{time::Duration, net::{SocketAddr, IpAddr, Ipv4Addr}};
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

    // Temp
    app.insert_resource(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));

    // Before we can register the potential message types, we
    // need to add the plugin
    app.add_plugin(bevy_spicy_networking::ServerPlugin);

    server_register_network_messages(&mut app);
    add_server_states(&mut app);

    app.run();
}