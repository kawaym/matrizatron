pub fn multiply_matrices(matrix1: &Vec<Vec<f32>>, matrix2: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
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

pub fn multiply_matrix_scalar(matrix: &Vec<Vec<f32>>, scalar: f32) -> Vec<Vec<f32>> {
    let mut new_matrix: Vec<Vec<f32>> = vec![vec![0.0; matrix[0].len()]; matrix.len()];

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            new_matrix[i][j] = matrix[i][j] * scalar
        }
    }

    return new_matrix;
}

pub fn add_matrix(matrix1: &Vec<Vec<f32>>, matrix2: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut new_matrix: Vec<Vec<f32>> = vec![vec![0.0; matrix1[0].len()]; matrix1.len()];
    for i in 0..matrix1.len() {
        for j in 0..matrix1[0].len() {
            new_matrix[i][j] = matrix1[i][j] + matrix2[i][j]
        }
    }
    return new_matrix;
}

pub fn tranpose_matrix(matrix: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut new_matrix: Vec<Vec<f32>> = vec![vec![0.0; matrix.len()]; matrix[0].len()];

    for i in 0..matrix[0].len() {
        for j in 0..matrix.len() {
            new_matrix[i][j] += matrix[j][i]
        }
    }

    return new_matrix;
}

pub fn compare_if_vector_is_bigger(vector1: &Vec<Vec<f32>>, vector2: &Vec<Vec<f32>>) -> bool {
    let mut result = false;
    for i in 0..vector1.len() {
        if vector1[i][0] > vector2[i][0] {
            result = true;
            break;
        }
    }

    return result;
}
