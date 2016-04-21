extern crate physics;

use physics::vector::Vector;
use physics::point::Point;
use physics::space::Space;
use std::thread;

fn main() {
    let mut s = Space::new(1.0);

    s.add_point(Point::new(2000000000.0, Vector::new(0.0,0.0,0.0), Vector::new(0.0,0.0,0.0), Vector::new(0.0,0.0,0.0)));
    s.add_point(Point::new(1.0, Vector::new(0.0,10.0,0.0), Vector::new(0.0,10.0,0.0), Vector::new(0.0,-0.0,0.0)));

    let child = thread::spawn(move || {
        s.update();
    });

    child.join();  
}
