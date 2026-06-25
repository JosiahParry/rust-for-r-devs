// Final form of `mean()` from this chapter: accepts a slice (`&[f64]`),
// which works for both `Vec<f64>` and arrays without moving them.
fn mean(x: &[f64]) -> f64 {
    let mut total = 0.0;
    for xi in x {
        total += xi;
    }
    total / x.len() as f64
}

fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [0.0, 9.5, 3.3, 11.78, 3.14159];
    println!(
        "The mean of x is {}.\nThe mean of y is {}",
        mean(&x),
        mean(&y)
    );
}
