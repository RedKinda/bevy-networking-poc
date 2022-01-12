use bevy::prelude::*;
use std::default::Default;
use crate::game::Location;

#[derive(Component)]
pub struct Graphical {
    pub(crate) texture_id: String,
    pub material: Option<ColorMaterial>,
}

pub fn add_sprites_to_graphicals(
    mut commands: Commands,
    mut added: Query<(Entity, &Graphical, &Location), (With<Graphical>, Without<Sprite>)>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>
) {
    let win = windows.get_primary().expect("no primary window");
    for (entity, graphical, location) in added.iter_mut() {

        let mut loc = location.clone();
        loc.x -= win.width() / 2.0;
        loc.y -= win.height() / 2.0;
        let sprite = SpriteBundle {
            texture: asset_server.load(graphical.texture_id.as_str()),
            sprite: Sprite {
                color: Color::ORANGE,
                ..Default::default()
            },
            transform: Location::to_transform(&loc),
            ..Default::default()
        };

        commands.entity(entity).insert_bundle(sprite);
    }

}

pub fn location_to_transform(mut query: Query<(&Location, &mut Transform), Changed<Location>>, windows: Res<Windows>) {
    let win = windows.get_primary().expect("no primary window");
    for (location, mut transform) in query.iter_mut() {
        let mut loc = location.clone();
        loc.x -= win.width() / 2.0;
        loc.y -= win.height() / 2.0;

        transform.translation.x = loc.x;
        transform.translation.y = loc.y;
    }
}
