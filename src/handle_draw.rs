use crate::cube::Cube;
use crate::state::State;
use crate::point::Point;
use crate::canvas::CairoCanvas;

use std::cell::Ref;

pub fn handle_draw(canvas: &mut CairoCanvas, cube: &mut Cube, state: &Ref<State>) {
    let result_transformation =
        multiply(
            rotate_ox
        )
}
