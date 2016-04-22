extern crate spe;
extern crate sfml;

use std::thread;
use std::sync::mpsc::Receiver;

use self::sfml::window::{ContextSettings, VideoMode, window_style};
use self::sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color, Transformable, Shape};

use spe::vector::Vector;
use spe::space::Space;

struct RenderedSpace<'a> {
    rx: &'a Receiver<Vec<(Vector, usize)>>,
    window: &'a mut RenderWindow,
    points: Vec<CircleShape<'a>>,
    shift_x: f64,
    shift_y: f64,
}

impl<'a> RenderedSpace<'a> {
    fn new(rx: &'a Receiver<Vec<(Vector, usize)>>, window: &'a mut RenderWindow, height: u32, width: u32) -> RenderedSpace<'a>  {
        RenderedSpace{
            rx: rx,
            window: window,
            points: Vec::<CircleShape<'a>>::new(),
            shift_x: (height/2) as f64,
            shift_y: (width/2) as f64,
        }
    }

    fn add_point(&mut self, color: &Color) {
        let mut circle = CircleShape::<'a>::new_init(10.0, 100).unwrap();
        circle.set_fill_color(color);
        self.points.push(circle);
    }

    fn render(&mut self) {
        while self.window.is_open() {
            for points in self.rx.into_iter() {
                self.window.clear(&Color::new_rgb(0, 200, 200));

                for point in points {
                    let vector = self.shift(point.0);
                    let index = point.1;
                    
                    self.points[index].set_position2f((vector.x as f32), (vector.y as f32));
                    self.window.draw(&self.points[index]);
                    self.window.display();
                }
            }
        }
    }

    fn shift(&self, v: Vector) -> Vector {
        v + Vector::new(self.shift_x, self.shift_y, 0.0)
    }
}

fn main() {
    let t = Space::new(0.05);
    let mut space = t.0;
    let rx = t.1;

    let mut window = match RenderWindow::new(VideoMode::new_init(1024, 768, 32),
                                             "SFML Example",
                                             window_style::CLOSE,
                                             &ContextSettings::default()) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };

    let mut render = RenderedSpace::new(&rx, &mut window, 1024, 768);

    thread::spawn(move || {
        space.update();
    });

    render.render();
}
