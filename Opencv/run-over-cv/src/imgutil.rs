use opencv::prelude::*;

pub fn shape(mat: &Mat) -> (i32, i32, i32) {
    (mat.rows(), mat.cols(), mat.channels())
}
