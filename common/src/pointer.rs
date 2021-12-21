use crate::events::{PlayerId, ServerEvent};
use crate::game::{Movable, PlayerControllable};
use crate::protocol::NetworkSync;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerPointer {
    control: PlayerControllable,
    movable: Movable,
    network_sync: NetworkSync,
    #[cfg(feature = "headless")]
    location: Transform,
    #[cfg(not(feature = "headless"))]
    #[bundle]
    sprite: SpriteBundle,
}

pub fn handle_pointer_spawns(
    mut commands: Commands,
    mut reader: EventReader<ServerEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for event in reader.iter() {
        match event {
            ServerEvent::PointerSpawn(netsync, owner, location) => {
                info!("Player pointer locally spawned!");
                PlayerPointer::spawn(
                    &mut commands,
                    &mut materials,
                    owner,
                    location,
                    netsync,
                    asset_server.load("player_pointer.png"),
                );
            }
            _ => {}
        }
    }
}

impl PlayerPointer {
    pub fn spawn(
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        owner: &PlayerId,
        location: &Vec2,
        netsync: &NetworkSync,
        texture_handle: Handle<TextureAtlas>,
    ) -> Entity {
        info!("Pointer spawning!");

        commands
            .spawn_bundle(Self {
                control: PlayerControllable::new(*owner),
                movable: Movable::new(*location),
                network_sync: *netsync,
                #[cfg(feature = "headless")]
                location: Transform::from_xyz(location.x, location.y, 0.0),
                #[cfg(not(feature = "headless"))]
                sprite: SpriteBundle {
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    material: materials.add(ColorMaterial::color(Color::ORANGE)),
                    transform: Transform::from_translation(location.extend(0.0)),
                    ..Default::default()
                },
            })
            .id()

        // sprite: SpriteSheetBundle {
        //     sprite: TextureAtlasSprite {
        //         ..Default::default()
        //     },
        //     texture_atlas: texture_handle.clone(),
        //     ..Default::default()
        // }
    }
}
