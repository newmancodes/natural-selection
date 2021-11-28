mod world;

use rand::prelude::*;
use world::world::World;

fn main() {
    match world::initialise(128) {
        Ok(mut world) => execute_simulation(world),
        Err(e) => eprintln!("Unable to generate world. {:?}", e),
    }
}

fn execute_simulation(mut world: World) {
    println!("Executing simulation.");
}
