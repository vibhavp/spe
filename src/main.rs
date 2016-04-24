extern crate spe;
extern crate rugra;

use std::thread;

use self::rugra::window::Window;

use spe::space::Space;
use spe::vector::Vector;
use spe::point::Point;
use spe::graphics::RenderedSpace;

fn main() {
    let t = Space::new(0.05, 1024/2, 768/2);
    let mut space = t.0;
    let rx = t.1;

    let mut window = Window::new("SPE");
    window.height(1024).width(768);

    let mut render = RenderedSpace::new(&rx, &mut window, 1024, 768);

    space.add_point(Point::new(1.0, Vector::new(0.0,20.0,0.0), Vector::new(0.0,0.0,0.0), Vector::new(0.0,0.0,0.0)));
    render.add_point((0,0,0));
    space.add_point(Point::new(99999999.0, Vector::new(10.0,0.0,0.0), Vector::new(0.0,0.0,0.0), Vector::new(0.0,0.0,0.0)));
    render.add_point((0,0,0));

    
    thread::spawn(move || {
        space.update();
    });

    render.render();
}
