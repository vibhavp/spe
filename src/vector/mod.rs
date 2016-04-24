use std::ops;
use std::fmt;

pub struct Vector {
    pub x: f64,
    pub y: f64,
//    pub z: f64,
}

impl Clone for Vector {
    fn clone(&self) -> Vector {
        Vector{x: self.x,
               y: self.y,
               //z: self.z
        }
    }
}

impl Copy for Vector {}

impl Default for Vector {
    fn default() -> Vector {Vector{
        x:0.0,
        y:0.0,
        //z:0.0
    }}
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector{x: self.x - other.x, y: self.y - other.y,
               //z: self.z - other.z
        }
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector{x: self.x + other.x, y: self.y + other.y,
               //z: self.z + other.z
        }
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, f: f64) -> Vector {
        Vector{x: self.x*f,y: self.y*f,
               //z: self.z*f
        }
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, f: f64) -> Vector {
        Vector{x: self.x/f,y: self.y/f,
               //z: self.z/f
        }
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector{x: x,y: y,
               //z: z
        }
    }
    
    pub fn norm(self) -> f64 {
        (self.x*self.x+self.y*self.y).sqrt()
    }

    pub fn normalized(self) -> Vector {
         self/self.norm()
    }
}

pub fn unit(v1: Vector, v2: Vector) -> Vector {
    (v1 - v2).normalized()
}

impl fmt::Display for Vector{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
