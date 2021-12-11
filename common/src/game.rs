use bevy::prelude::*;

const POINTER_SPEED: u64 = 20;

pub struct Movable {
    target_location: Vec2,
    active: bool
}

impl Movable {
    pub fn to_dumb_vec3(&self) -> Vec3 {
        Vec3::new(self.target_location.x, self.target_location.y, 0.0)
    }
}

pub struct GamePlugin {}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_movable.system());
    }
}

fn move_movable(mut query: Query<(&mut Movable, &mut Transform)>, time: Res<Time>) {
    let delta = time.delta_seconds_f64() as f32;
    for (mut mv, mut transform) in query.iter_mut() {
        if mv.active {
            let target_point = mv.to_dumb_vec3();
            if transform.translation.distance(target_point) < delta * (POINTER_SPEED as f32){
                mv.active = false;
                transform.translation.x = target_point.x;
                transform.translation.y = target_point.y;
            } else {
                let diff = (target_point - transform.translation).normalize() * (POINTER_SPEED as f32);
                transform.translation.x += diff.x;
                transform.translation.y += diff.y;
            }
        }
    }
}

