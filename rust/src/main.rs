use rand::Rng;
use std::time::Instant;

fn main() {
    let n = 1024;

    // Generate matrices a and b with random values
    let mut rng = rand::thread_rng();
    let a: Vec<Vec<f64>> = (0..n)
        .map(|_| (0..n).map(|_| rng.gen::<f64>()).collect())
        .collect();

    let b: Vec<Vec<f64>> = (0..n)
        .map(|_| (0..n).map(|_| rng.gen::<f64>()).collect())
        .collect();

    // Initialize matrix c with zeros
    let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

    let start = Instant::now();

    // Perform matrix multiplication
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
