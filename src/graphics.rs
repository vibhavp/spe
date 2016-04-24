extern crate rugra;

use std::sync::mpsc::Receiver;
use vector::Vector;
use self::rugra::window::Window;
use self::rugra::circle::Circle;

pub struct RenderedSpace<'a> {
    rx: &'a Receiver<Vec<(Vector, usize)>>,
    window: &'a mut Window,
    points: Vec<Circle<'a>>,
    shift_x: f64,
    shift_y: f64,
}

impl<'a> RenderedSpace<'a> {
    pub fn new(rx: &'a Receiver<Vec<(Vector, usize)>>, window: &'a mut Window, height: u32, width: u32) -> RenderedSpace<'a>  {
        RenderedSpace{
            rx: rx,
            window: window,
            points: Vec::<rugra::circle::Circle<'a>>::new(),
            shift_x: (width/2) as f64,
            shift_y: (height/2) as f64,
        }
    }

    pub fn add_point(&mut self, color: (u8,u8,u8)) {
        let mut circle = Circle::<'a>::new();
        circle.radius(10.0);
        circle.color(color.0,color.1,color.2);
        self.points.push(circle);
    }

    pub fn render(&mut self) {
        while self.window.is_open() {
            for points in self.rx.into_iter() {
                self.window.clear(0,200,200);

                for point in points {
                    let mut vector = point.0;
                    self.shift(&mut vector);
                    let index = point.1;
                    let ref mut circle = self.points[index];

                    circle.set(((vector.x as f32), (vector.y as f32)));
                    circle.draw(self.window);
                    self.window.update();
                }
            }
        }
    }

    fn shift(&self, v: &mut Vector) {
        v.x += self.shift_x;
        v.y += self.shift_y;

    }
}
