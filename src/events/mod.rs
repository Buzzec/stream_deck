use serde::{Deserialize, Serialize};

pub mod received;
pub mod sent;

/// Coordinates of a key/encoder on the Stream Deck.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Copy)]
pub struct Coordinates {
    pub column: u32,
    pub row: u32,
}
