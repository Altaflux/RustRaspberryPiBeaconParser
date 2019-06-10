

mod blurz;

use super::beacon::Beacon;
use self::blurz::BlurzListener;

pub type EventHandler = Box<Fn(Beacon) + Send>;
