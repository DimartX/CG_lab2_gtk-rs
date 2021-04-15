use crate::cube::{FigureImpl, Figure};
use crate::state::State;
use crate::canvas::{CairoCanvas, Canvas};
use crate::transformations::TransformMatrix;
use std::cmp::min;

use std::cell::Ref;

use std::f64::consts::PI;
fn to_radians(angle: f64) -> f64 {
    angle / 180.0 * PI
}

pub fn handle_draw(canvas: &mut CairoCanvas, state: &Ref<State>) {
    let width = canvas.width();
    let height = canvas.height();

    let coefficient = min(width, height) as f64 / 600.0;

    let mut cube: Figure = Figure::newCube();

    let mut cube_transformation =
        TransformMatrix::new().
        move_by_vector([-0.5, -0.5, -0.5, 1.0]).
        stretch(state.stretchOx, state.stretchOy, state.stretchOz).
        rotate_ox(to_radians(state.rotateOx));
    let after_ox_rotated = TransformMatrix.
        rotate_oy(to_radians(state.rotateOy)).
        rotate_oz(to_radians(state.rotateOz));

    cube_transformation = cube_transformation
        .move_by_vector([state.moveOx, state.moveOy, state.moveOz, 1.0])
        .zoom(state.zoom / 50.0 * coefficient);

    println!("{:?}", cube_transformation.mtx);

    cube.transform(cube_transformation.mtx);
    cube.draw(canvas, ((width / 2), (height / 2)),
              state.hide_lines, state.carcass, state.filling, state.view);

    let mut axes: Figure = Figure::newAxes();

    let axes_transformation =
        TransformMatrix::new().
        rotate_ox(to_radians(state.rotateOx)).
        rotate_oy(to_radians(state.rotateOy)).
        rotate_oz(to_radians(state.rotateOz)).
        zoom(40.0 * coefficient);

    axes.transform(axes_transformation.mtx);
    axes.drawAxes(canvas, ((width as f64 * 0.9) as i32,
                       (height as f64 * 0.9) as i32));
}
