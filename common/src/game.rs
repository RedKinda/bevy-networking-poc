use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::ops::{Deref, DerefMut};
use bevy::math::Vec3Swizzles;
use crate::events::{PlayerCommand, PlayerId, ServerEvent};
use crate::errors::*;
use crate::pointer::*;
use crate::graphics::*;

const POINTER_SPEED: u64 = 100;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Location(pub Vec2);

impl Location {
    pub fn to_transform(&self) -> Transform {
        Transform::from_xyz(self.x, self.y, 0.0)
    }
}

impl Deref for Location {
    type Target = Vec2;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Location {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

// Component definitions
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Movable {
    target_location: Location,
    active: bool,
    speed: u64
}

impl Movable {
    pub fn new(target: Vec2) -> Self {
        return Movable {
            target_location: Location(target),
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

        if !self.settings.headless {
            app.add_system_set(SystemSet::new()
                //.with_run_criteria(headless_condition.system())
                .with_system(add_sprites_to_graphicals.system())
                .with_system(location_to_transform.system())
            );
        }

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

fn move_movable(mut query: Query<(&mut Movable, &mut Location)>, time: Res<Time>) {
    let delta = time.delta_seconds_f64() as f32;
    for (mut mv, mut location) in query.iter_mut() {
        if mv.active {
            info!(movable = ?mv, location = ?location);
            let target_point = mv.to_dumb_vec3();
            // info!(distance = transform.translation.distance(target_point), can_travel =  delta * (mv.speed as f32));
            if location.distance(target_point.xy()) <= delta * (mv.speed as f32) {
                mv.active = false;
                location.x = target_point.x;
                location.y = target_point.y;
            } else {
                let diff = (target_point.xy() - **location).normalize();
                location.x += diff.x * delta * (mv.speed as f32);
                location.y += diff.y * delta * (mv.speed as f32);
            }
        }
    }
}

pub fn validate_player_command(player_id: PlayerId, controllable: &PlayerControllable, command: PlayerCommand) -> Result<(), PlayerCommandValidationError> {
    dbg!(command);
    if player_id != controllable.owner {
        error!("Player tried to control that, which was not controllable");
        Err(PlayerCommandValidationError::NotOwned { attempted: player_id, owner: controllable.owner })
    } else {
        Ok(())
    }
}

