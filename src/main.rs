use crate::server::definition::{Vec3, WorldDefinition};
use crate::server::generator::DefaultWorldGenerator;
use crate::server::generator::WorldGenerator;

mod client;
mod common;
mod playground;
mod server;

const SEED: [u8; 32] = [13; 32];

fn main() {
    let world_generator = DefaultWorldGenerator {};
    let world = world_generator.generate(WorldDefinition {
        size: Vec3(10, 10, 10),
        seed: SEED,
    });
    println!("{:?}", world);
}
