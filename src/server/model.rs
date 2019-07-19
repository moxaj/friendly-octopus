use std::collections::HashMap;

use crate::server::definition::{Position, Size};

#[derive(Debug)]
pub struct Tile {}

#[derive(Debug)]
pub struct World {
    pub size: Size,
    pub tiles: HashMap<Position, Tile>,
}
