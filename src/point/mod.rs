use vector::Vector;
use std::process::exit;
use vector::unit;

const GRAV_CONST: f64 = 0.00000000006674;

pub struct Point {
    mass: f64,
    pub cur_pos: Vector,
    pub prev_pos: Vector,
    pub accel: Vector,
    grav_accel: Vector,
}

impl Point {
    pub fn new(mass: f64, cur_pos: Vector, prev_pos: Vector, init_accel: Vector) -> Point {
        Point{
            mass: mass,
            cur_pos: cur_pos,
            prev_pos: prev_pos,
            accel: init_accel,
            grav_accel: Vector::default(),
        }
    }

    pub fn update_verlet(&mut self, step: f64) {
        let temp = self.cur_pos;
        let total_accel = self.accel + self.grav_accel;

        //println!("diff: {}",(self.cur_pos-self.prev_pos).norm());
        self.cur_pos = self.cur_pos + (self.cur_pos - self.prev_pos) + total_accel*(step.powi(2));
        self.prev_pos = temp;
        //println!("prev: {}, cur: {}", temp, self.cur_pos)
    }

    pub fn update_gravity(&mut self, point: Point) {
        let diff = point.cur_pos - self.cur_pos;
        let dist = diff.norm();
        if dist == 0.0 {
            exit(1);
        }
        let u = unit(point.cur_pos, self.cur_pos);
        
        //println!("{}", self.grav_accel.mul_scalar(GRAV_CONST * point.mass/(dist*dist)));
        self.grav_accel = self.grav_accel + u*(GRAV_CONST * (point.mass/dist.powi(2)));
        //println!("dist: {} grav: {}", dist, self.grav_accel);
    }

    pub fn current_position(self) -> Vector {
        self.cur_pos.clone()
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point{mass: self.mass,
              cur_pos: self.cur_pos.clone(),
              prev_pos: self.prev_pos.clone(),
              accel: self.accel.clone(),
              grav_accel: self.grav_accel.clone()}
    }
}

impl Copy for Point {}
