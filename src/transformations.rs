// affine transformations

struct TransformMatrix ([[i32; 4]; 4]);

fn stretch(x: i32, y: i32, z: i32) ->  {
    return [
        [x, 0, 0, 0],
        [0, y, 0, 0],
        [0, 0, z, 0],
        [0, 0, 0, 1],
    ]
}

fn rotate_ox(angle: f64) -> [[f64; 4]; 4] {
    return [
        [1,            0,           0, 0],
        [0,  angle.cos(), angle.sin(), 0],
        [0, -angle.sin(), angle.cos(), 0],
        [0,            0,           0, 1],
    ]
}

fn rotate_oy(angle: f64) -> [[f64; 4]; 4] {
    return [
        [angle.cos(), 0, -angle.sin(), 0],
        [0,           1,            0, 0],
        [angle.sin(), 0,  angle.cos(), 0],
        [0,           0,            0, 1],
    ]
}

fn rotate_oz(angle: f64) -> [[f64; 4]; 4] {
    return [
        [ angle.cos(), angle.sin(), 0, 0],
        [-angle.sin(), angle.cos(), 0, 0],
        [0,            0,           1, 0],
        [0,            0,           0, 1],
    ]
}

fn rotate_unit_vector(angle: f64, [l, m, n, _]: [f64; 4]) ->  [[f64; 4]; 4] {
    return [
        [
            l * l * (1 - angle.cos()) + angle.cos(),
            m * l * (1 - angle.cos()) - n * angle.sin(),
            n * l * (1 - angle.cos()) + m * angle.sin(),
            0
        ],
        [
            l * m * (1 - angle.cos()) + n * angle.sin(),
            m * m * (1 - angle.cos()) + angle.cos(),
            n * m * (1 - angle.cos()) - l * angle.sin(),
            0
        ],
        [
            l * n * (1 - angle.cos()) - m * angle.sin(),
            m * n * (1 - angle.cos()) + l * angle.sin(),
            n * n * (1 - angle.cos()) + angle.cos(),
            0
        ],
        [0, 0, 0, 1],
    ]
}

fn move_by_vector([l, m, n, _]: [f64; 4]) -> [[f64; 4]; 4] {
    return [
        [1, 0, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 1, 0],
        [l, m, n, 1],
    ]
}
