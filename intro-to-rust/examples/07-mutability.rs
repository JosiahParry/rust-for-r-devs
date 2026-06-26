fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0];

    let n = x.len() as f64;
    let mut total = 0.0;

    for xi in x {
        total += xi;
    }

    println!("The mean is: {}", total / n);
}
