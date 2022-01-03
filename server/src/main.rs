mod internal_events;

use crate::internal_events::{Internal, InternalPlugin};
use common::bevy::app::ScheduleRunnerSettings;
use common::bevy::asset::AssetPlugin;
use common::bevy::log::LogPlugin;
use common::bevy::prelude::*;
use common::bevy::utils::HashMap;
use common::bevy_networking_turbulence::{
    ConnectionHandle, NetworkEvent, NetworkResource, NetworkingPlugin,
};
use common::events::*;
use common::game::{validate_player_command, GameInfo, Movable, PlayerControllable, Location};
use common::get_random;
use common::protocol::{ClientIdentification, NetworkSync};
use std::net::SocketAddr;
use std::time::Duration;

type ClientHandleMap = HashMap<ConnectionHandle, PlayerId>;
type AssociatedCommand = (PlayerId, PlayerCommand);

pub fn main() {
    let mut app = App::build();

    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )))
    .insert_resource(ClientHandleMap::default());

    app.add_event::<AssociatedCommand>();

    app.add_plugins(MinimalPlugins)
        .add_plugin(NetworkingPlugin {
            link_conditioner: None,
            message_flushing_strategy: Default::default(),
            idle_timeout_ms: None,   //Some(7000),
            auto_heartbeat_ms: None, //Some(2000),
            heartbeats_and_timeouts_timestep_in_seconds: None,
        })
        .add_plugin(LogPlugin::default())
        .add_plugin(AssetPlugin::default())
        .add_plugin(common::game::GameEnginePlugin { settings: GameInfo { is_network_authority: true, headless: true } })
        .add_plugin(InternalPlugin {});

    app.add_startup_system(startup.system());

    app.add_system(handle_clients_commands.system())
        .add_system(sync_movable.system())
        .add_system(handle_client_connections.system())
        .add_system(handle_client_move_commands.system())
        .add_system(broadcast_server_events.system());

    app.run();
}

fn startup(mut net: ResMut<NetworkResource>, game_info: ResMut<GameInfo>) {
    common::protocol::network_setup(&mut net);

    let address = common::bevy_networking_turbulence::find_my_ip_address().unwrap();

    let server_address = SocketAddr::new(address, common::SERVER_PORT);
    info!("Server listening on {}", server_address);

    if game_info.headless {
        info!("Server is headless");
    } else {
        info!("Server is NOT headless!");
    }

    net.listen(server_address, None, None);
}

fn handle_clients_commands(
    mut net: ResMut<NetworkResource>,
    mut player_command_queue: EventWriter<AssociatedCommand>,
    client_player_map: Res<ClientHandleMap>,
) {
    // info!("Handling clients...");
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(game_event) = channels.recv::<GameEvent>() {
            match game_event {
                GameEvent::PlayerCommand(cmd) => {
                    if let Some(id) = client_player_map.get(handle) {
                        player_command_queue.send((id.clone(), cmd));
                    } else {
                        warn!("An unmapped client {} sent command", handle);
                    }
                }
                GameEvent::ServerUpdate(_) => {
                    error!("Client should never send a GameEvent!")
                }
            }

            /*match network_event {
                NetworkEvent::Packet(handle, packet) => {
                    //let content: GameEvent = packet.into();
                    let content = common::serde_form::from_slice::<GameEvent>(packet.as_bytes()).expect("Failed to deserialize packet");
                }

                NetworkEvent::Connected(handle) => {
                    info!("New client connected {}", handle)
                }
                NetworkEvent::Disconnected(handle) => {}
                NetworkEvent::Error(handle, error) => {
                    let err_message = match error {
                        NetworkError::TurbulenceChannelError(e) => { e.to_string() }
                        NetworkError::IoError(e) => { e.to_string() }
                        NetworkError::MissedHeartbeat => { "Missed heartbeat".to_string() }
                        NetworkError::Disconnected => { "Errorneous disconnect".to_string() }
                    };
                    error!("Error network event from handle {}: {}", handle, err_message)
                }
            }

             */
        }
    }
}

fn broadcast_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut net: ResMut<NetworkResource>,
) {
    server_events.iter().for_each(|event| {
        info!(broadcasting = ?event);
        net.connections.iter_mut().for_each(|(_, conn)| {
            conn.channels()
                .unwrap()
                .send::<GameEvent>(GameEvent::ServerUpdate(*event));
        });
    });
}

fn handle_client_connections(
    mut reader: EventReader<NetworkEvent>,
    mut internal_events: EventWriter<Internal>,
    mut handle_map: ResMut<ClientHandleMap>,
) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Connected(handle) => {
                info!("New client! Handle is {}", handle);

                let new_id = ClientIdentification::new(get_random());
                handle_map.insert(*handle, new_id.player_id.clone());

                internal_events.send(Internal::PlayerConnected(*handle, new_id));
            }
            NetworkEvent::Disconnected(handle) => {
                info!("Client {} disconnected.", handle);
            }
            NetworkEvent::Packet(_, packet) => {
                info!(packet_received = ?packet);
            }
            NetworkEvent::Error(handle, error) => {
                info!(handle = handle, error = ?error);
            }
        }
    }
}

fn handle_client_move_commands(
    mut command_queue: EventReader<AssociatedCommand>,
    mut query: Query<(&mut Movable, &mut PlayerControllable, &NetworkSync)>,
) {
    /*let mut movables: HashMap<NetworkObjectId, (Mut<Movable>, Mut<PlayerControllable>, &NetworkSync)>;
    query.iter_mut().for_each(|unit| {
        movables.insert(unit.2.unique_id, unit);
    });

     */

    command_queue.iter().for_each(|(player_id, controllable)| {
        if let PlayerCommand::PointerMoveChange(unit_id, target_movable) = controllable {
            //info!(target_unit = unit_id, query = ?query.iter_mut().collect::<Vec<(Mut<'_, Movable>, Mut<'_, PlayerControllable>, &NetworkSync)>>());
            if let Some(mut unit) = query.iter_mut().find(|unit| unit.2.unique_id == unit_id.unique_id) {
                match validate_player_command(*player_id, &unit.1, *controllable) {
                    Ok(_) => {
                        unit.0.update(*target_movable);
                    }
                    Err(e) => {
                        warn!("{}", e);
                    }
                }
            } else {
                warn!(msg = "Player tried to move unit X which is not movable or does not exist", player = player_id, unit = ?unit_id);
            }
        }


    })
}

fn broadcast_server_event(event_writer: &mut EventWriter<ServerEvent>, event: ServerEvent) {
    // info!(sending_event = ?event);
    event_writer.send(event);
}

fn sync_movable(
    mut to_sync: Query<(&NetworkSync, &Movable, &Location), (Changed<Movable>, With<NetworkSync>)>,
    mut server_events: EventWriter<ServerEvent>,
) {
    for (netsync, &movable, &location) in to_sync.iter_mut() {
        broadcast_server_event(
            &mut server_events,
            ServerEvent::EntityMovementChange(*netsync, movable, *location),
        );
    }
}
