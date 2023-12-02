use crate::libs::{armijo, calcs, matrices};

pub fn main(initial_point: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut current_point = initial_point;
    let epsilon: f32 = 0.0001;
    let mut k = 0;

    while matrices::compare_if_vector_is_bigger(
        &calcs::gradient(&current_point),
        &[
            [epsilon].to_vec(),
            [epsilon].to_vec(),
            [epsilon].to_vec(),
            [epsilon].to_vec(),
            [epsilon].to_vec(),
        ]
        .to_vec(),
    ) {
        let descent_direction =
            matrices::multiply_matrix_scalar(&calcs::gradient(&current_point), -1.0);
        let t = armijo::main(&current_point, 0.8, 0.25, &descent_direction);

        current_point = matrices::add_matrix(
            &current_point,
            &matrices::multiply_matrix_scalar(&descent_direction, t),
        );
        k += 1;
        println!("{:#?}", current_point);
        println!("{}", k);
        if (k > 9999) {
            break;
        }
    }
    return current_point;
}
