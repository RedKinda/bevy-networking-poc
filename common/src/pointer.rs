use crate::events::{PlayerId, ServerEvent};
use crate::game::{Movable, PlayerControllable};
use crate::protocol::NetworkSync;
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct Location(Vec2);

#[derive(Bundle)]
pub struct PlayerPointer {
    control: PlayerControllable,
    movable: Movable,
    network_sync: NetworkSync,
    location: Location,
    #[cfg(not(feature = "headless"))]
    #[bundle]
    sprite: SpriteBundle,
}

pub fn handle_pointer_spawns(
    mut commands: Commands,
    mut reader: EventReader<ServerEvent>,
    #[cfg(not(feature = "headless"))] mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in reader.iter() {
        match event {
            ServerEvent::PointerSpawn(netsync, owner, location) => {
                info!("Player pointer locally spawned!");
                PlayerPointer::spawn(
                    &mut commands,
                    #[cfg(not(feature = "headless"))]
                    &mut materials,
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
        #[cfg(not(feature = "headless"))] materials: &mut ResMut<Assets<ColorMaterial>>,
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
                #[cfg(not(feature = "headless"))]
                sprite: SpriteBundle {
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    material: materials.add(ColorMaterial::color(Color::ORANGE)),
                    transform: Transform::from_translation(location.extend(0.0)),
                    ..Default::default()
                },
            })
            .id()
    }
}
