use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, atomic, mpsc};

use point::Point;
use vector::Vector;

pub struct Space {
    time_step: f64,
    point_count: atomic::AtomicUsize,
    points: Arc<Mutex<Vec<Point>>>,
    updates_in: mpsc::Sender<Vec<(Vector, usize)>>,
}

impl Space {
    pub fn new(step: f64) -> (Space, mpsc::Receiver<Vec<(Vector, usize)>>) {
        let (tx, rx) = mpsc::channel::<Vec<(Vector, usize)>>();
        
        (Space{
            time_step: step,
            point_count: atomic::AtomicUsize::new(0),
            points: Arc::new(Mutex::new(Vec::new())),        
            updates_in: tx,
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

    pub fn update(&mut self) {
        let ns = (self.time_step * self.time_step*(10.0 as f64).powi(9)) as u32;
        let duration = Duration::new(0, ns);
        
        loop {
            if self.point_count.load(atomic::Ordering::Relaxed) == 0 {
                return;
            }
            
            self.update_gravity();
            self.update_verlet();

            let data = self.points.clone();
            let points = data.lock().unwrap();

            let mut vectors = Vec::new();
            for i in 0..points.len() {
                vectors.push((points[i].current_position(), i))
            }

            self.updates_in.send(vectors);
            thread::sleep(duration);            
        }
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
