pub fn objetive_function(x: &Vec<Vec<f32>>) -> f32 {
    let result: f32 = (x[0][0].powi(2)) * (x[1][0].sin()).powi(2)
        + 2.0 * (x[2][0].powi(2))
        + (x[3][0] * x[4][0]).powi(2);

    return result;
}

// [x, y, z, s, t]

pub fn gradient(x: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let result: Vec<Vec<f32>> = [
        [2.0 * x[0][0] * (x[1][0].sin()).powi(2)].to_vec(),
        [x[0][0].powi(2) * (2.0 * x[1][0]).sin()].to_vec(),
        [4.0 * x[2][0]].to_vec(),
        [2.0 * x[3][0] * x[4][0].powi(2)].to_vec(),
        [2.0 * x[3][0].powi(2) * x[4][0]].to_vec(),
    ]
    .to_vec();

    return result;
}

pub fn hessian(x: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let result: Vec<Vec<f32>> = [
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
