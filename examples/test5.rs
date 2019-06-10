
use std::thread;
use std::time::Duration;
fn main() {
    let mut x = 0;
    let mut y = 0;
    loop {
            x = x + 1;
            println!("looping x: {:?}", x);
            thread::sleep(Duration::from_millis(800));

            y = y + 1;
            println!("looping y: {:?}", y);
            thread::sleep(Duration::from_millis(800));
    }
    println!("looping END x: {:?}", x);
    println!("looping END y: {:?}", y);
}
