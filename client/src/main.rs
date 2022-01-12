use common::bevy::prelude::*;
use common::bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin};
use common::events::*;
use common::game::{GameInfo, Location, Movable, PlayerControllable};
use common::protocol::*;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use common::bevy::log::{Level, LogSettings};

pub fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin {
            link_conditioner: None,
            message_flushing_strategy: Default::default(),
            idle_timeout_ms: None,   // Some(7000),
            auto_heartbeat_ms: None, //Some(2000),
            heartbeats_and_timeouts_timestep_in_seconds: None,
        })
        .add_plugin(common::game::GameEnginePlugin::default());

    app.add_startup_system(startup.system());

    app.insert_resource(common::protocol::ClientIdentification::new(0));
    app.insert_resource(LogSettings{ filter: "".to_string(), level: Level::DEBUG });

    app.add_system(capture_clicks.system())
        .add_system(log_connectivity.system())
        .add_system(receive_initial.system())
        .add_system(receive_server_events.system())
        .add_system(handle_movement_changes.system());

    app.run();
}

fn startup(mut commands: Commands, mut net: ResMut<NetworkResource>, info: Res<GameInfo>) {
    network_setup(&mut net);

    if info.headless {
        warn!("Client is running headless!")
    }

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let address = SocketAddr::new(
        //common::bevy_networking_turbulence::find_my_ip_address().unwrap(),
        IpAddr::from_str("80.112.130.36").unwrap(),
        common::SERVER_PORT,
    );

    info!("Connecting to address {}", address);
    net.connect(address);
}

fn send_command(net: &mut ResMut<NetworkResource>, command: PlayerCommand) {
    info!(
        "Sending command {}",
        common::serde_form::to_string(&command).unwrap()
    );
    net.broadcast_message(GameEvent::PlayerCommand(command));
}

fn log_connectivity(mut reader: EventReader<NetworkEvent>, mut net: ResMut<NetworkResource>) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Connected(handle) => {
                info!("Connected! Handle is {}", handle);
                send_command(&mut net, PlayerCommand::Ping(42));
            }
            NetworkEvent::Disconnected(handle) => {
                warn!("Handle {} disconnected!", handle)
            }
            NetworkEvent::Packet(handle, packet) => {
                info!(
                    "Got a packet: {} fron handle {}",
                    String::from_utf8_lossy(packet),
                    handle
                )
            }
            NetworkEvent::Error(handle, error) => {
                error!(handle = handle, error = ?error)
            }
        }
    }
}

fn receive_initial(mut net: ResMut<NetworkResource>, mut identity: ResMut<ClientIdentification>) {
    for (_, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(info) = channels.recv::<MetaInformation>() {
            match info {
                MetaInformation::ClientIdentificationMessage(id) => {
                    identity.update(id);
                }
                MetaInformation::DisconnectReason(reason) => {
                    error!("Was disconnected! {}", reason);
                }
            }
        }
    }
}

fn receive_server_events(mut net: ResMut<NetworkResource>, mut writer: EventWriter<ServerEvent>) {
    for (_, conn) in net.connections.iter_mut() {
        let channels = conn.channels().unwrap();
        while let Some(event) = channels.recv::<GameEvent>() {
            match event {
                GameEvent::ServerUpdate(e) => {
                    writer.send(e);
                }
                _ => {}
            }
        }
    }
}

fn handle_movement_changes(
    mut events: EventReader<ServerEvent>,
    mut query: Query<(&NetworkSync, &mut Movable, &mut Location)>,
) {
    for event in events.iter() {
        if let ServerEvent::EntityMovementChange(netsync, movable, pos) = event {
            if let Some((_, mut current_movable, mut current_transform)) = query
                .iter_mut()
                .find(|unit| unit.0.unique_id == netsync.unique_id)
            {
                current_movable.update(*movable);
                current_transform.x = pos.x;
                current_transform.y = pos.y;
            } else {
                warn!(msg = "Movement changed but there is no corresponding netsync present", netsync = ?netsync);
            }
        }
    }
}

fn capture_clicks(
    mut net: ResMut<NetworkResource>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    identity: Res<ClientIdentification>,
    my_pointer: Query<(&NetworkSync, &PlayerControllable), With<Movable>>,
) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) {
        let position = win
            .cursor_position()
            .expect("Mouse was clicked, cursor should have position");
        info!("Click detected at {},{}", position.x, position.y);
        if let Some(pointer) = my_pointer
            .iter()
            .find(|(_, ctrl)| ctrl.owner == identity.player_id)
        {
            send_command(
                &mut net,
                PlayerCommand::PointerMoveChange(*pointer.0, Movable::new(position)),
            )
        } else {
            send_command(&mut net, PlayerCommand::Ping(42));
            warn!("No pointer for this player :(")
        }
    }
}
