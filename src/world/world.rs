use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

pub struct World {
    locations: Vec<WorldLocation>,
}

enum WorldLocation {
    Empty,
}

#[derive(Debug, PartialEq)]
pub struct WorldError{
    message: String,
}

impl fmt::Display for WorldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "WorldError {{ message: {} }}", self.message)
    }
}

pub fn new(size: u8) -> Result<World, WorldError> {
    if size < 32 {
        return Err(
            WorldError
            {
                message: "World size must be at least 32.".to_string(),
            });
    }

    let mut locations = Vec::with_capacity(size as usize);
    for _ in 0..size {
        locations.push(WorldLocation::Empty);
    }
    Ok(World { locations })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;

    #[test]
    fn size_must_be_at_least_thirty_two() {
        let size: u8 = 31;
        let expected = WorldError { message: "World size must be at least 32.".to_string() };
        assert_eq!(expected, new(size).err().unwrap());
    }

    #[test]
    fn world_is_created_empty() {
        let size: u8 = 32;
        let world = new(size).ok().unwrap();
        assert_eq!(size as usize, world.locations.len());
        for location in world.locations {
            assert!(matches!(location, WorldLocation::Empty));
        }
    }
}