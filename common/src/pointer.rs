use crate::events::{PlayerId, ServerEvent};
use crate::game::{Movable, PlayerControllable};
use crate::protocol::NetworkSync;
use bevy::prelude::*;
use crate::game::Location;
use crate::graphics::Graphical;


#[derive(Bundle)]
pub struct PlayerPointer {
    control: PlayerControllable,
    movable: Movable,
    network_sync: NetworkSync,
    location: Location,
    graphical: Graphical
}

pub fn handle_pointer_spawns(
    mut commands: Commands,
    mut reader: EventReader<ServerEvent>,
) {
    for event in reader.iter() {
        match event {
            ServerEvent::PointerSpawn(netsync, owner, location) => {
                info!("Player pointer locally spawned!");
                PlayerPointer::spawn(
                    &mut commands,
                    owner,
                    location,
                    netsync,
                );
            }
            _ => {}
        }
    }
}

impl PlayerPointer {
    pub fn spawn(
        commands: &mut Commands,
        // materials: &mut ResMut<Assets<ColorMaterial>>,
        owner: &PlayerId,
        location: &Vec2,
        netsync: &NetworkSync,
    ) -> Entity {
        info!("Pointer spawning!");

        commands
            .spawn_bundle(Self {
                control: PlayerControllable::new(*owner),
                movable: Movable::new(*location),
                network_sync: *netsync,
                location: Location(*location),
                graphical: Graphical {
                    texture_id: "player_pointer.png".to_string(),
                    material: None
                }
            })
            .id()
    }
}
