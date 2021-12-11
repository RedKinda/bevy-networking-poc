use bevy::prelude::*;
use bevy_networking_turbulence::{NetworkResource};
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    MovePoint(Vec2)
}

pub fn broadcast_game_event(event: GameEvent, network_resource: &mut NetworkResource) {
    network_resource.broadcast_message(event)
}