use std::collections::HashMap;

use crate::server::vec;
use crate::server::vec::Vec3i;

#[derive(Debug)]
pub struct Block {}

#[derive(Debug)]
pub struct Chunk {
    pub chunk_size: Vec3i,
    pub blocks: Vec<Block>,
}

impl Chunk {
    pub fn get_block_at(&self, position: &Vec3i) -> Option<&Block> {
        self.blocks.get(vec::linear_index(position, &self.chunk_size) as usize)
    }
}

#[derive(Debug)]
pub struct World {
    pub world_size: Vec3i,
    pub chunk_size: Vec3i,
    pub chunks: HashMap<Vec3i, Chunk>,
}

impl World {
    pub fn get_chunk_at(&self, position: &Vec3i) -> Option<&Chunk> {
        // TODO if chunk is None, then create one if within world_size range
        self.chunks.get(position)
    }

    pub fn get_block_at(&self, position: &Vec3i) -> Option<&Block> {
        let (world_position, chunk_position) = vec::quot_rem_vec3i(position, &self.chunk_size);
        self.get_chunk_at(&world_position)
            .and_then(|chunk| chunk.get_block_at(&chunk_position))
    }
}
