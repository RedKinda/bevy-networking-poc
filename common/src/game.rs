use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::events::{PlayerCommand, PlayerId, ServerEvent};
use crate::errors::*;
use crate::pointer::*;
use crate::graphics::*;

const POINTER_SPEED: u64 = 100;


// Component definitions
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Movable {
    target_location: Vec2,
    active: bool,
    speed: u64
}

impl Movable {
    pub fn new(target: Vec2) -> Self {
        return Movable {
            target_location: target,
            active: true,
            speed: POINTER_SPEED
        }
    }

    pub fn to_dumb_vec3(&self) -> Vec3 {
        Vec3::new(self.target_location.x, self.target_location.y, 0.0)
    }

    pub fn update(&mut self, new: Movable) {
        self.speed = new.speed;
        self.active = new.active;
        self.target_location = new.target_location;
    }
}

#[derive(Debug)]
pub struct PlayerControllable {
    pub owner: PlayerId
}

impl PlayerControllable {
    pub fn new(owner: PlayerId) -> Self {
        return PlayerControllable {owner};
    }
}

#[derive(Clone)]
pub struct GameInfo {
    pub is_network_authority: bool,
    pub headless: bool
}

pub struct GameEnginePlugin {
    pub settings: GameInfo
}

impl Default for GameEnginePlugin {
    fn default() -> Self {
        GameEnginePlugin{
            settings: GameInfo{ is_network_authority: false, headless: false}
        }
    }
}

impl Plugin for GameEnginePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_movable.system())
            .add_system(handle_pointer_spawns.system());

        app.add_system_set(SystemSet::new()
            .with_run_criteria(headless_condition.system())
            .with_system(add_sprites_to_graphicals.system())
        );

        app.add_event::<ServerEvent>();

        app.insert_resource::<GameInfo>(self.settings.clone());
        // app.add_asset::<ColorMaterial>();
        info!("Included game engine plugin!")
    }
}

fn headless_condition(settings: Res<GameInfo>) -> ShouldRun {
    match settings.headless {
        true => ShouldRun::YesAndCheckAgain,
        false => ShouldRun::NoAndCheckAgain
    }
}

fn move_movable(mut query: Query<(&mut Movable, &mut Transform)>, time: Res<Time>) {
    let delta = time.delta_seconds_f64() as f32;
    for (mut mv, mut transform) in query.iter_mut() {
        if mv.active {
            info!(movable = ?mv, transform = ?transform);
            let target_point = mv.to_dumb_vec3();
            // info!(distance = transform.translation.distance(target_point), can_travel =  delta * (mv.speed as f32));
            if transform.translation.distance(target_point) <= delta * (mv.speed as f32) {
                mv.active = false;
                transform.translation.x = target_point.x;
                transform.translation.y = target_point.y;
            } else {
                let diff = (target_point - transform.translation).normalize();
                transform.translation.x += diff.x * delta * (mv.speed as f32);
                transform.translation.y += diff.y * delta * (mv.speed as f32);
            }
        }
    }
}

pub fn validate_player_command(player_id: PlayerId, controllable: &PlayerControllable, command: PlayerCommand) -> Result<(), PlayerCommandValidationError> {
    dbg!(command);
    if player_id != controllable.owner {
        Err(PlayerCommandValidationError::NotOwned { attempted: player_id, owner: controllable.owner })
    } else {
        Ok(())
    }
}

