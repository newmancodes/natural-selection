pub mod world;

use world::World;
use world::WorldError;

pub fn initialise(size: u8) -> Result<World, WorldError> {
    world::new(size)
}