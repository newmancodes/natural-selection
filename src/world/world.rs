use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
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
    if size <= 32 {
        return Err(
            WorldError
            {
                message: "World size must be at least 32.".to_string(),
            });
    }

    Ok(World { locations })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_must_be_at_least_thirty_two() {
        let size: u8 = 31;
        let expected = WorldError { message: "World size must be at least 32.".to_string() };
        assert_eq!(expected, new(size).err().unwrap());
    }

    #[test]
    fn world_is_created_empty() {
        let size: u8 = 32;
        let world = new(size).unwrap();
    }
}