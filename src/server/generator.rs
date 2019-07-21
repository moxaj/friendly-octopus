use crate::server::model::World;
use crate::server::seed::Seed;
use crate::server::vec::Vec3i;

#[derive(Debug, Clone, Copy)]
pub struct WorldDefinition {
    pub seed: Seed,
    pub world_size: Vec3i,
    pub chunk_size: Vec3i,
}

pub trait WorldGenerator {
    fn generate(&self, definition: WorldDefinition) -> World;
}

pub struct DefaultWorldGenerator {}

impl WorldGenerator for DefaultWorldGenerator {
    fn generate(&self, definition: WorldDefinition) -> World {
        unimplemented!()
    }
}
