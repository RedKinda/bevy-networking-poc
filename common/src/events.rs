use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::game::Movable;
use crate::protocol::NetworkSync;

pub type PlayerId = u32;


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    PlayerCommand(PlayerCommand),
    ServerUpdate(ServerEvent)
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PlayerCommand {
    PointerMoveChange(NetworkSync, Movable),
    Ping(u32)
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ServerEvent {
    PointerSpawn(NetworkSync, PlayerId, Vec2),
    EntityMovementChange(NetworkSync, Movable, Vec2),
}