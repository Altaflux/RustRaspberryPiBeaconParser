
use std::error::Error;

use super::super::beacon::Beacon;

pub trait Publisher {
    fn publish(&mut self, message: &Beacon) -> Result<(), Box<dyn Error>>;
}