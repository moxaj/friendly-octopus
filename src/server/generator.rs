use std::collections::HashMap;

use base64;
use rand::RngCore;
use rand::SeedableRng;

use crate::server::definition::WorldDefinition;
use crate::server::model::World;

pub trait WorldGenerator {
    fn generate(&self, definition: WorldDefinition) -> World;
}

pub struct DefaultWorldGenerator {}

impl WorldGenerator for DefaultWorldGenerator {
    fn generate(&self, definition: WorldDefinition) -> World {
        let mut rand = rand::rngs::StdRng::from_seed([0; 32]);
        println!("{}", rand.next_u32());

        let s = base64::encode(&[2; 32]);
        println!("{} {:?}", s, base64::decode(&s).unwrap());

        World {
            size: definition.size,
            tiles: HashMap::new(),
        }
    }
}
