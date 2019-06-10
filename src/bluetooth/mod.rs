

mod blurz;

use super::beacon::Beacon;
pub use self::blurz::BlurzListener;

pub type EventHandler = Box<Fn(Beacon) + Send>;
