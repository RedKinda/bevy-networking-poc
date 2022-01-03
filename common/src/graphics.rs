use bevy::prelude::*;
use crate::game::Location;

pub struct Graphical {
    pub(crate) texture_id: String,
    pub material: Option<ColorMaterial>,
}

pub fn add_sprites_to_graphicals(
    mut commands: Commands,
    mut added: Query<(Entity, &Graphical, &Location), (With<Graphical>, Without<Sprite>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut windows: Res<Windows>
) {
    let win = windows.get_primary().expect("no primary window");
    for (entity, graphical, location) in added.iter_mut() {

        let mut loc = location.clone();
        loc.x -= win.width() / 2.0;
        loc.y -= win.height() / 2.0;
        let sprite = SpriteBundle {
            material: materials.add(
                ColorMaterial{
                    color: Color::ORANGE,
                    texture: Some(asset_server.load(graphical.texture_id.as_str()))
                }),
            transform: Location::to_transform(&loc),
            ..Default::default()
        };

        commands.entity(entity).insert_bundle(sprite);
    }

}

pub fn location_to_transform(mut query: Query<(&Location, &mut Transform), Changed<Location>>, mut windows: Res<Windows>) {
    let win = windows.get_primary().expect("no primary window");
    for (location, mut transform) in query.iter_mut() {
        let mut loc = location.clone();
        loc.x -= win.width() / 2.0;
        loc.y -= win.height() / 2.0;

        transform.translation.x = loc.x;
        transform.translation.y = loc.y;
    }
}
