use crate::canvas::{CairoCanvas, Canvas};
use std::vec::Vec;
use crate::transformations::*;
use crate::point::Point;
use crate::color::Color;

struct Polygon {
    points: Vec<[f64; 4]>,
}

impl Polygon {
    fn from(pts: Vec<[f64; 4]>) -> Self {
        Self {
            points: pts,
        }
    }
}

pub trait FigureImpl {
    fn transform(&mut self, matrix: [[f64; 4]; 4]);
    fn draw(&self, canvas: &mut CairoCanvas, pos: (i32, i32));
}

pub struct Figure {
    polygons: Vec<Polygon>,
}

impl Figure {
    pub fn newCube() -> Self {
        Self {
            polygons: vec![
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [0.0, 1.0, 0.0, 1.0],
                        [1.0, 1.0, 0.0, 1.0],
                        [1.0, 0.0, 0.0, 1.0],
                    ]
                ),
                Polygon::from(
                    vec![
                        [0.0, 0.0, 1.0, 1.0],
                        [0.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 0.0, 1.0, 1.0],
                    ]
                ),
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [1.0, 0.0, 0.0, 1.0],
                        [1.0, 0.0, 1.0, 1.0],
                        [0.0, 0.0, 1.0, 1.0],
                    ]
                ),
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [0.0, 1.0, 0.0, 1.0],
                        [0.0, 1.0, 1.0, 1.0],
                        [0.0, 0.0, 1.0, 1.0],
                    ]
                ),
                Polygon::from(
                    vec![
                        [0.0, 1.0, 0.0, 1.0],
                        [1.0, 1.0, 0.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [0.0, 1.0, 1.0, 1.0],
                    ]
                ),
                Polygon::from(
                    vec![
                        [1.0, 0.0, 0.0, 1.0],
                        [1.0, 1.0, 0.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 0.0, 1.0, 1.0],
                    ]
                ),
            ],
        }
    }

    pub fn newAxes() -> Self {
        Self {
            polygons: vec![
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [1.0, 0.0, 0.0, 1.0],
                    ]
                ),
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [0.0, 1.0, 0.0, 1.0],
                    ]
                ),
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0, 1.0],
                    ]
                ),

            ],
        }
    }
}

impl FigureImpl for Figure {
    fn transform(&mut self, matrix: [[f64; 4]; 4]) {
        for polygon in &mut self.polygons {
            polygon.points = mult_matrix_on_transform(&polygon.points, matrix);
            //println!("{:?}", polygon.points);
        }
    }

    fn draw(&self, canvas: &mut CairoCanvas, pos: (i32, i32)) {
        canvas.set_draw_color(Color::new(0, 0, 0));

        for polygon in &self.polygons {
            let mut pts: Vec<Point> = Vec::new();
            for point in &polygon.points {
                println!("Point x = {}, y = {}, z = {}", point[0], point[1], point[2]);
                pts.push(Point::new(point[0] as i32 + pos.0,
                                    point[1] as i32 + pos.1));
            }
            canvas.draw_polygon(&pts);
        }
    }
}
