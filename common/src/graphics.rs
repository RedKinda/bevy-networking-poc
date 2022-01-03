use bevy::prelude::*;

pub struct Graphical {
    pub(crate) texture_id: String,
    pub material: Option<ColorMaterial>,
}

pub fn add_sprites_to_graphicals(
    mut commands: Commands,
    mut added: Query<(Entity, &Graphical), (With<Graphical>, Without<Sprite>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    for (entity, graphical) in added.iter_mut() {
        let sprite = SpriteBundle {
            material: materials.add(
                ColorMaterial{
                    color: Color::ORANGE,
                    texture: Some(asset_server.load(graphical.texture_id.as_str()))
                }),
            ..Default::default()
        };

        commands.entity(entity).insert_bundle(sprite);
    }

}
