use crate::libs::{calcs, matrices};

pub fn main(
    initial_point: &Vec<Vec<f32>>,
    y: f32,
    n: f32,
    descent_direction: &Vec<Vec<f32>>,
) -> f32 {
    let mut t: f32 = 1.0;
    let current_point = initial_point;

    while calcs::objetive_function(&matrices::add_matrix(
        &current_point,
        &matrices::multiply_matrix_scalar(&descent_direction, t),
    )) > (calcs::objetive_function(&current_point)
        + n * t
            * matrices::multiply_matrices(
                &matrices::tranpose_matrix(&calcs::gradient(&current_point)),
                &descent_direction,
            )[0][0])
    {
        t = y * t;
    }

    return t;
}
