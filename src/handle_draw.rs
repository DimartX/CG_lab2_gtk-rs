use crate::cube::{FigureImpl, Figure};
use crate::state::State;
use crate::canvas::{CairoCanvas, Canvas};
use crate::transformations::TransformMatrix;

use std::cell::Ref;

use std::f64::consts::PI;
fn to_radians(angle: f64) -> f64 {
    angle / 180.0 * PI
}

pub fn handle_draw(canvas: &mut CairoCanvas, state: &Ref<State>) {
    let width = canvas.width();
    let height = canvas.height();

    let mut cube: Figure = Figure::newCube();

    let cube_transformation =
        TransformMatrix::new().
        move_by_vector([-0.5, -0.5, -0.5, 1.0]).
        stretch(state.stretchOx, state.stretchOy, state.stretchOz).
        rotate_ox(to_radians(state.rotateOx)).
        rotate_oy(to_radians(state.rotateOy)).
        rotate_oz(to_radians(state.rotateOz)).
        move_by_vector([state.moveOx, state.moveOy, state.moveOz, 1.0]).
        zoom(state.zoom / 50.0);

    cube.transform(cube_transformation.mtx);
    cube.draw(canvas, ((width / 2), (height / 2)));

    let mut axes: Figure = Figure::newAxes();

    let axes_transformation =
        TransformMatrix::new().
        rotate_ox(to_radians(state.rotateOx)).
        rotate_oy(to_radians(state.rotateOy)).
        rotate_oz(to_radians(state.rotateOz)).
        zoom(30.0);

    axes.transform(axes_transformation.mtx);
    axes.draw(canvas, ((width as f64 * 0.9) as i32, (height as f64 * 0.9) as i32));
}
