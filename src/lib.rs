#![feature(augmented_assignments)]

pub fn test_vec_add(){
    let v1 = vector::Vector::new(1.0,1.0,1.0);
    let v2 = vector::Vector::new(2.0,2.0,2.0);
    let v3 = v1 + v2;
    assert!( v3.x == 3.0 && v3.y == 3.0 && v3.z == 3.0);    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        test_vec_add();
    }
}

pub mod point;
pub mod vector;
pub mod space;
