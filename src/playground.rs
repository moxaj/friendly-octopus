// Observer

use std::collections::HashMap;
use crate::server::model::{Block, Chunk, World};
use crate::server::vec::Vec3;

/*
pub struct Block<'a> {
    pub value: i32,
    pub observers: Vec<&'a Observer<i32>>,
}

impl<'a> Observable<'a, i32> for Block<'a> {
    fn subscribe_observer(&mut self, observer: &'a impl Observer<i32>) {
        self.observers.push(observer);
    }

    fn get_value(&'a self) -> &'a i32 {
        &self.value
    }

    fn get_observers(&self) -> &[&dyn Observer<i32>] {
        &self.observers
    }
}

impl<'a> Block<'a> {
    fn new(value: i32) -> Block<'a> {
        Block {
            value,
            observers: vec![],
        }
    }
}

pub struct Player {}

impl Observer<i32> for Player {
    fn notify(&self, value: &i32) {
        println!("Updated value: {}", value);
    }
}
*/

pub fn test() {
    let chunk_size = Vec3(10, 10, 10);

    let make_blocks = || {
        let mut blocks = Vec::new();
        for i in 0..1000 {
            blocks.push(Block {});
        }

        blocks
    };

    let mut chunks = HashMap::new();
    for x in -5..=5 {
        for y in -5..=5 {
            for z in -5..=5 {
                chunks.insert(Vec3(x, y, z), Chunk {
                    chunk_size,
                    blocks: make_blocks(),
                });
            }
        }
    }

    let world = World {
        world_size: Vec3(5, 5, 5),
        chunk_size,
        chunks,
    };

    let block = world.get_block_at(&Vec3(25, 25, 25));
    println!("{:?}", &block);
}