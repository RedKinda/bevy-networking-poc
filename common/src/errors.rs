use thiserror::Error;
use crate::events::*;

#[derive(Error, Debug)]
pub enum PlayerCommandValidationError {
    #[error("Player {attempted:?} does not own this unit owned by {owner:?}")]
    NotOwned{
        attempted: PlayerId,
        owner: PlayerId
    }
}