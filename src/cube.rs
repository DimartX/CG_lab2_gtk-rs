use std::f64::consts::PI;
use crate::canvas::{CairoCanvas, Canvas};

trait Figure {
    fn transform(&mut self, matrix: &[[i32; 4]; 4]);
    fn draw(&self, canvas: &mut CairoCanvas);
}

pub struct Cube {
    points: [[i32; 4]; 8],
    center: [i32; 4],
    transformation: [[i32; 4]; 4],
}

impl Cube {
    pub fn new() -> Self {
        Self {
            points:
            [
                [0, 0, 0, 0],
                [1, 0, 0, 0],
                [1, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 1, 0],
                [1, 0, 1, 0],
                [1, 1, 1, 0],
                [0, 1, 1, 0],
            ],
            center: [0, 0, 0, 1],
            transformation:
            [
                [1, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 1],
            ],
        }
    }
}

impl Figure for Cube {
    fn transform(&mut self, matrix: &[[i32; 4]; 4]) {

    }

    fn draw(&self, canvas: &mut CairoCanvas) {

    }
}
