use crate::libs::matrices;

pub fn main(initial_point: Vec<Vec<f64>>, y: f64, n: f64, descent_direction: Vec<Vec<f64>>) -> f64 {
    let mut t: f64 = 1.0;
    let current_point = initial_point;

    while matrices::objetive_function(&matrices::add_matrix(
        &current_point,
        &matrices::multiply_matrix_scalar(&descent_direction, t),
    )) > (matrices::objetive_function(&current_point)
        + n * t
            * matrices::multiply_matrices(
                &matrices::tranpose_matrix(&matrices::gradient(&current_point)),
                &descent_direction,
            )[0][0])
    {
        println!(
            "lado esquerdo: {:#?}",
            matrices::objetive_function(&matrices::add_matrix(
                &current_point,
                &matrices::multiply_matrix_scalar(&descent_direction, t)
            ))
        );
        println!(
            "lado direito: {:#?}",
            matrices::multiply_matrices(
                &matrices::tranpose_matrix(&matrices::gradient(&current_point)),
                &descent_direction,
            )[0][0]
                * n
                * t
                + matrices::objetive_function(&current_point)
        );
        t = y * t;
    }

    return t;
}
