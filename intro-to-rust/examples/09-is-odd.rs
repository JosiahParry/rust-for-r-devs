fn mean(x: Vec<f64>) -> f64 {
    let mut total = 0.0;
    let n = x.len();
    for xi in x {
        total += xi;
    }
    total / n as f64
}

fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = mean(x);
    println!("Mean is: {}", result);
}
