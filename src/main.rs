mod gradient;
mod libs;

fn main() {
    println!("{:#?}", teste())
}

fn teste() -> Vec<Vec<f32>> {
    return gradient::main(
        [
            [1.0].to_vec(),
            [1.0].to_vec(),
            [1.0].to_vec(),
            [1.0].to_vec(),
            [1.0].to_vec(),
        ]
        .to_vec(),
    );
}
