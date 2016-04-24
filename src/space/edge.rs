pub struct Edge {
    pub point_1: usize,
    pub point_2: usize,
    pub length: f64, 
}

use point::Point;

pub fn update_points_for_edge(orig_length: f64, p1: &mut Point, p2: &mut Point) {
    let vec_diff = p1.current_position() - p2.current_position();
    let edge_length_diff = vec_diff.norm() - orig_length;
    let vec_diff_unit = vec_diff.normalized();

    p1.cur_pos = p1.cur_pos + vec_diff_unit*edge_length_diff*0.5;
    p2.cur_pos = p2.cur_pos - vec_diff_unit*edge_length_diff*0.5;
}
