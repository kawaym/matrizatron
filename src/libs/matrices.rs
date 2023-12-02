pub fn objetive_function(x: &Vec<Vec<f64>>) -> f64 {
    let result: f64 = (x[0][0].powi(2)) * (x[1][0].sin()).powi(2)
        + 2.0 * (x[2][0].powi(2))
        + (x[3][0] * x[4][0]).powi(2);

    return result;
}

// [x, y, z, s, t]

pub fn gradient(x: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let result: Vec<Vec<f64>> = [
        [2.0 * x[0][0] * (x[1][0].sin()).powi(2)].to_vec(),
        [x[0][0].powi(2) * (2.0 * x[1][0]).sin()].to_vec(),
        [4.0 * x[2][0]].to_vec(),
        [2.0 * x[3][0] * x[4][0].powi(2)].to_vec(),
        [2.0 * x[3][0].powi(2) * x[4][0]].to_vec(),
    ]
    .to_vec();

    return result;
}

pub fn hessian(x: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let result: Vec<Vec<f64>> = [
        [
            2.0 * (x[1][0].sin()).powi(1),
            4.0 * x[0][0] * x[1][0].cos() * x[1][0].sin(),
            0.0,
            0.0,
            0.0,
        ]
        .to_vec(),
        [
            4.0 * x[0][0] * x[1][0].cos() * x[1][0].sin(),
            2.0 * x[0][0].powi(2) * ((x[1][0].cos()).powi(2) - (x[1][0].sin()).powi(2)),
            0.0,
            0.0,
            0.0,
        ]
        .to_vec(),
        [0.0, 0.0, 4.0, 0.0, 0.0].to_vec(),
        [
            0.0,
            0.0,
            0.0,
            2.0 * x[4][0].powi(2),
            4.0 * x[3][0] * x[4][0],
        ]
        .to_vec(),
        [
            0.0,
            0.0,
            0.0,
            4.0 * x[3][0] * x[4][0],
            2.0 * x[3][0].powi(2),
        ]
        .to_vec(),
    ]
    .to_vec();

    return result;
}

pub fn multiply_matrices(matrix1: &Vec<Vec<f64>>, matrix2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let number_of_lines_matrix1 = matrix1.len();
    let number_of_columns_matrix1 = matrix1[0].len();
    let number_of_columns_matrix2 = matrix2[0].len();

    let mut matrix = vec![vec![0.0; number_of_columns_matrix2]; number_of_lines_matrix1];

    for i in 0..number_of_lines_matrix1 {
        for j in 0..number_of_columns_matrix2 {
            for k in 0..number_of_columns_matrix1 {
                matrix[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }
    return matrix;
}

pub fn multiply_matrix_scalar(matrix: &Vec<Vec<f64>>, scalar: f64) -> Vec<Vec<f64>> {
    let mut new_matrix: Vec<Vec<f64>> = vec![vec![0.0; matrix[0].len()]; matrix.len()];

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            new_matrix[i][j] = matrix[i][j] * scalar
        }
    }

    return new_matrix;
}

pub fn add_matrix(matrix1: &Vec<Vec<f64>>, matrix2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut new_matrix: Vec<Vec<f64>> = vec![vec![0.0; matrix1[0].len()]; matrix1.len()];
    for i in 0..matrix1.len() {
        for j in 0..matrix1[0].len() {
            new_matrix[i][j] = matrix1[i][j] + matrix2[i][j]
        }
    }
    return new_matrix;
}

pub fn tranpose_matrix(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut new_matrix: Vec<Vec<f64>> = vec![vec![0.0; matrix.len()]; matrix[0].len()];

    for i in 0..matrix[0].len() {
        for j in 0..matrix.len() {
            new_matrix[i][j] += matrix[j][i]
        }
    }

    return new_matrix;
}
