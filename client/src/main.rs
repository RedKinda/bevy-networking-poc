use common::bevy::prelude::*;
use common::bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin};
use common::events::*;
use common::game::{Movable, PlayerControllable};
use common::pointer::PlayerPointer;
use common::protocol::*;
use std::net::SocketAddr;

pub fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin {
            link_conditioner: None,
            message_flushing_strategy: Default::default(),
            idle_timeout_ms: None,   // Some(7000),
            auto_heartbeat_ms: None, //Some(2000),
            heartbeats_and_timeouts_timestep_in_seconds: None,
        })
        .add_plugin(common::game::GameEnginePlugin {});

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(common::bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(startup.system());
    app.add_startup_system(startup_test.system());

    app.insert_resource(common::protocol::ClientIdentification::new(0));

    app.add_system(capture_clicks.system())
        .add_system(log_connectivity.system())
        .add_system(receive_initial.system())
        .add_system(receive_server_events.system())
        .add_system(handle_movement_changes.system());

    app.run();
}

fn startup_test(mut commands: Commands, assets: Res<AssetServer>) {
    PlayerPointer::spawn(
        &mut commands,
        &0,
        &Default::default(),
        &NetworkSync { unique_id: 0 },
        assets.load("player_pointer.png"),
    );
}

fn startup(mut commands: Commands, mut net: ResMut<NetworkResource>) {
    network_setup(&mut net);

    if common::is_headless() {
        warn!("Client is running headless!")
    }

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let address = SocketAddr::new(
        common::bevy_networking_turbulence::find_my_ip_address().unwrap(),
        common::SERVER_PORT,
    );

    info!("Connecting to address {}", address);
    net.connect(address);
}

fn send_command(mut net: ResMut<NetworkResource>, command: PlayerCommand) {
    debug!(
        "Sending command {}",
        common::serde_form::to_string(&command).unwrap()
    );
    net.broadcast_message(GameEvent::PlayerCommand(command));
}

fn log_connectivity(mut reader: EventReader<NetworkEvent>) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Connected(handle) => {
                info!("Connected! Handle is {}", handle)
            }
            NetworkEvent::Disconnected(handle) => {
                warn!("Handle {} disconnected!", handle)
            }
            NetworkEvent::Packet(handle, packet) => {
                debug!(
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
    mut query: Query<(&NetworkSync, &mut Movable, &mut Transform)>,
) {
    for event in events.iter() {
        if let ServerEvent::EntityMovementChange(netsync, movable, pos) = event {
            if let Some((_, mut current_movable, mut current_transform)) = query
                .iter_mut()
                .find(|unit| unit.0.unique_id == netsync.unique_id)
            {
                current_movable.update(*movable);
                current_transform.translation.x = pos.x;
                current_transform.translation.y = pos.y;
                current_transform.translation.z = pos.z;
            } else {
                warn!(msg = "Movement changed but there is no corresponding netsync present", netsync = ?netsync);
            }
        }
    }
}

fn capture_clicks(
    net: ResMut<NetworkResource>,
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
        debug!("Click detected at {},{}", position.x, position.y);
        if let Some(pointer) = my_pointer
            .iter()
            .find(|(_, ctrl)| ctrl.owner == identity.player_id)
        {
            send_command(
                net,
                PlayerCommand::PointerMoveChange(*pointer.0, Movable::new(position)),
            )
        } else {
            send_command(net, PlayerCommand::Ping(42));
            warn!("No pointer for this player :(")
        }
    }
}
