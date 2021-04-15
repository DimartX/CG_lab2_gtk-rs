use crate::canvas::{CairoCanvas, Canvas};
use std::vec::Vec;
use crate::transformations::*;
use crate::point::Point;
use crate::color::Color;
use crate::view::View;

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
    fn draw(&self, canvas: &mut CairoCanvas, pos: (i32, i32),
            hide_lines: bool, carcass: bool, filling: bool, projection: View);
}

pub struct Figure {
    polygons: Vec<Polygon>,
}

impl Figure {
    pub fn newCube() -> Self {
        Self {
            polygons: vec![
                // front
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [1.0, 0.0, 0.0, 1.0],
                        [1.0, 1.0, 0.0, 1.0],
                        [0.0, 1.0, 0.0, 1.0],
                    ]
                ),
                // back
                Polygon::from(
                    vec![
                        [0.0, 0.0, 1.0, 1.0],
                        [0.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 0.0, 1.0, 1.0],
                    ]
                ),
                // above
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0, 1.0],
                        [1.0, 0.0, 1.0, 1.0],
                        [1.0, 0.0, 0.0, 1.0],
                    ]
                ),
                // bottom
                Polygon::from(
                    vec![
                        [0.0, 1.0, 0.0, 1.0],
                        [1.0, 1.0, 0.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [0.0, 1.0, 1.0, 1.0],
                    ]
                ),
                // right
                Polygon::from(
                    vec![
                        [1.0, 0.0, 0.0, 1.0],
                        [1.0, 0.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 0.0, 1.0],
                    ]
                ),
                // left
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [0.0, 1.0, 0.0, 1.0],
                        [0.0, 1.0, 1.0, 1.0],
                        [0.0, 0.0, 1.0, 1.0],
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

    pub fn drawAxes(&self, canvas: &mut CairoCanvas, pos: (i32, i32)) {
        {
            let mut pts: Vec<Point> = Vec::new();
            canvas.set_draw_color(Color::red());
            for point in &self.polygons[0].points {
                //println!("Point x = {}, y = {}, z = {}", point[0], point[1], point[2]);
                pts.push(Point::new(point[0] as i32 + pos.0,
                                    point[1] as i32 + pos.1));
            }
            canvas.draw_polygon(&pts);
        }

        {
            let mut pts: Vec<Point> = Vec::new();
            canvas.set_draw_color(Color::green());
            for point in &self.polygons[1].points {
                pts.push(Point::new(point[0] as i32 + pos.0,
                                    point[1] as i32 + pos.1));
            }
            canvas.draw_polygon(&pts);
        }

        {
            let mut pts: Vec<Point> = Vec::new();
            canvas.set_draw_color(Color::blue());
            for point in &self.polygons[2].points {
                pts.push(Point::new(point[0] as i32 + pos.0,
                                    point[1] as i32 + pos.1));
            }
            canvas.draw_polygon(&pts);
        }
    }
}

fn cross_product(a: [f64; 4], b: [f64; 4], c: [f64; 4]) -> [f64; 4] {
    [
        (a[1] - c[1]) * (b[2] - c[0]) - (a[2] - c[2]) * (b[1] - c[1]),
        (a[2] - c[2]) * (b[0] - c[0]) - (a[0] - c[0]) * (b[2] - c[2]),
        (a[0] - c[0]) * (b[1] - c[1]) - (a[1] - c[1]) * (b[0] - c[0]),
        1.0,
    ]
}

fn dot_product(lhs: [f64; 4], rhs: [f64; 4]) -> f64 {
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

fn angle_vectors(lhs: [f64; 4], rhs: [f64; 4]) -> f64 {
    dot_product(lhs, rhs) /
        (dot_product(lhs, lhs).sqrt() * dot_product(rhs, rhs).sqrt())
}

fn norm(vector: [f64; 4]) -> [f64; 4] {
    let norma = dot_product(vector, vector).sqrt();
    [
        vector[0] / norma,
        vector[1] / norma,
        vector[2] / norma,
        1.0,
    ]
}

fn draw_tile(canvas: &mut CairoCanvas, pos: (i32, i32), polygon: &Polygon,
             hide_lines: bool, carcass: bool, filling: bool) {
    let mut pts: Vec<Point> = Vec::new();

    let normal = cross_product(
        norm(polygon.points[1]),
        norm(polygon.points[0]),
        norm(polygon.points[2]),
    );

    println!("Normal {:?}", normal);

    if hide_lines && normal[2] > 0.000001 {
        return;
    }

    for point in &polygon.points {
        //println!("Point x = {}, y = {}, z = {}", point[0], point[1], point[2]);
        pts.push(Point::new(point[0] as i32 + pos.0,
                            point[1] as i32 + pos.1));
    }

    if filling {
        let alpha = angle_vectors(normal, [0.0, 0.0, -1.0, 0.0]) * 2.0 / 3.0;
        println!("Alpha {}", alpha);
        canvas.set_draw_color_alpha(Color::dark_green(), 1.0 - alpha);
        canvas.fill_polygon(&pts);
    }

    if carcass {
        canvas.set_draw_color(Color::black());
        canvas.draw_polygon(&pts);
    }
}


impl FigureImpl for Figure {
    fn transform(&mut self, matrix: [[f64; 4]; 4]) {
        for polygon in &mut self.polygons {
            polygon.points = mult_matrix_on_transform(&polygon.points, matrix);
            //println!("{:?}", polygon.points);
        }
    }

    fn draw(&self, canvas: &mut CairoCanvas, pos: (i32, i32),
            hide_lines: bool, carcass: bool, filling: bool, projection: View) {
        canvas.set_draw_color(Color::new(0, 0, 0));

        match projection {
            View::Isometric => {
                for polygon in &self.polygons {
                    draw_tile(canvas, pos, &polygon, hide_lines, carcass, filling);
                }
            },
            View::Front => {
                draw_tile(canvas, pos, &self.polygons[0], hide_lines, carcass, filling);
            },
            View::Above => {
                draw_tile(canvas, pos, &self.polygons[2], hide_lines, carcass, filling);
            },
            View::Side => {
                draw_tile(canvas, pos, &self.polygons[4], hide_lines, carcass, filling);
            }
        }


    }
}
