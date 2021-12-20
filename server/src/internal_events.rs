use common::bevy::app::{AppBuilder, Plugin};
use common::bevy::math::Vec2;
use common::bevy::prelude::{IntoSystem, ResMut};
use common::bevy_networking_turbulence::NetworkResource;
use common::events::{GameEvent, ServerEvent};
use common::events::ServerEvent::PointerSpawn;
use common::game::{Movable, PlayerControllable};
use common::protocol::{ClientIdentification, MetaInformation, NetworkSync};
use crate::{broadcast_server_event, ConnectionHandle, EventReader, EventWriter, Query, Transform};

pub enum Internal {
    PlayerConnected(ConnectionHandle, ClientIdentification)
}

pub struct InternalPlugin {}

impl Plugin for InternalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<Internal>();
        app.add_system(handle_new_player_connections.system())
            .add_system(spawn_point_on_player_connect.system())
            .add_system(sync_pointers_on_connect.system());
    }
}

fn handle_new_player_connections(
    mut reader: EventReader<Internal>,
    mut net: ResMut<NetworkResource>)
{
    for event in reader.iter() {
        if let Internal::PlayerConnected(handle, id) = event {
            let to_send = MetaInformation::ClientIdentificationMessage(id.clone());
            net.connections.get_mut(&handle).unwrap().channels().unwrap().send::<MetaInformation>(to_send);
        }
    }
}

fn spawn_point_on_player_connect(
    mut reader: EventReader<Internal>,
    mut server_events: EventWriter<ServerEvent>
) {
    for event in reader.iter() {
        if let Internal::PlayerConnected(_, id) = event {
            broadcast_server_event(&mut server_events, PointerSpawn(
                NetworkSync::new(),
                id.player_id.clone(),
                Vec2::new(50.0, 50.0)
            ));
        }
    }
}

fn sync_pointers_on_connect(
    mut reader: EventReader<Internal>,
    pointers: Query<(&NetworkSync, &Movable, &Transform, &PlayerControllable)>,
    mut net: ResMut<NetworkResource>
) {
    for event in reader.iter() {
        if let Internal::PlayerConnected(handle, _id) = event {
            for (nsync, movable, transform, player) in pointers.iter() {
                net.connections.get_mut(&handle).unwrap().channels().unwrap().send::<GameEvent>(GameEvent::ServerUpdate(ServerEvent::PointerSpawn(
                    nsync.clone(), player.owner, Vec2::from(transform.translation)
                )));
                net.connections.get_mut(&handle).unwrap().channels().unwrap().send::<GameEvent>(GameEvent::ServerUpdate(ServerEvent::EntityMovementChange(
                    nsync.clone(), *movable, transform.translation
                )));
            }
        }
    }
}










