use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, atomic, mpsc};
use std::result::Result;

use point::Point;
use vector::Vector;

mod edge;
use space::edge::Edge;

pub struct Space {
    time_step: f64,
    point_count: atomic::AtomicUsize,
    points: Arc<Mutex<Vec<Point>>>,
    edges: Arc<Mutex<Vec<Edge>>>,
    updates_in: mpsc::Sender<Vec<(Vector, usize)>>,
    height: u64,
    width: u64,
}

impl Space {
    pub fn new(step: f64, height: u64, width: u64) -> (Space, mpsc::Receiver<Vec<(Vector, usize)>>) {
        let (tx, rx) = mpsc::channel::<Vec<(Vector, usize)>>();

        (Space{
            time_step: step,
            point_count: atomic::AtomicUsize::new(0),
            points: Arc::new(Mutex::new(Vec::new())),
            edges: Arc::new(Mutex::new(Vec::new())),
            updates_in: tx,
            height: height,
            width: width,
        }, rx)
    }

    fn update_gravity(&mut self) {
        let data = self.points.clone();
        let mut points = data.lock().unwrap();

        for i in 0..points.len() {
            for j in 0..points.len() {
                if i == j {
                    continue
                }
                let point = points[j];
                points[i].update_gravity(point);
            }
        }
    }

    fn update_verlet(&mut self) {
        let data = self.points.clone();
        let mut points = data.lock().unwrap();

        for point in points.iter_mut() {
            point.update_verlet(self.time_step);
        }
    }

    fn update_edges(&mut self) {
        let data = self.points.clone();
        let points = data.lock().unwrap();

        let d2 = self.edges.clone();
        let mut edges = d2.lock().unwrap();

        for edge in edges.iter_mut() {
            let mut p1 = points[edge.point_1];
            let mut p2 = points[edge.point_2];

            edge::update_points_for_edge(edge.length, &mut p1, &mut p2)
        }
    }

    pub fn update(&mut self) {
        let ns = (self.time_step * self.time_step*(10.0 as f64).powi(9)) as u32;
        let duration = Duration::new(0, ns);

        loop {
            if self.point_count.load(atomic::Ordering::Relaxed) == 0 {
                return;
            }

            self.update_gravity();
            self.update_verlet();
            self.update_edges();

            let data = self.points.clone();
            let mut points = data.lock().unwrap();

            let mut vectors = Vec::new();
            for i in 0..points.len() {
                if self.has_collided(points[i].cur_pos) {
                    points[i].accel = points[i].accel*0.5*-1.0;

                    let temp = points[i].cur_pos;
                    points[i].cur_pos = points[i].prev_pos;
                    points[i].prev_pos = temp;
                }

                vectors.push((points[i].cur_pos.clone(), i))
            }

            match self.updates_in.send(vectors) {
                Result::Ok(val) => val,
                Result::Err(err) => panic!("{}", err),
            };

            thread::sleep(duration);
        }
    }

    fn has_collided(&self, pos: Vector) -> bool {
        pos.y.abs() >= (self.height as f64) || pos.x.abs() >= (self.width as f64)
    }

    pub fn add_point(&mut self, p: Point) -> usize {
        let data = self.points.clone();
        let mut points = data.lock().unwrap();
        self.point_count.store(self.point_count.load(atomic::Ordering::Relaxed) + 1, atomic::Ordering::Relaxed);
        points.push(p);
        points.len() - 1
    }

    pub fn remove_point(&mut self, index: usize) -> Option<&str> {
        let data = self.points.clone();
        let mut points = data.lock().unwrap();

        if index >= points.len() {
            Some("Invalid index value")
        } else {
            points.remove(index);
        self.point_count.store(self.point_count.load(atomic::Ordering::Relaxed) - 1, atomic::Ordering::Relaxed);
            None
        }
    }
}
