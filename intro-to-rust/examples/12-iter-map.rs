fn mean(x: &[f64]) -> f64 {
    let total: f64 = x.iter().sum();
    total / x.len() as f64
}

// Exercise 1: variance via `.map()` + `.sum()`
fn variance(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    let avg = mean(x);
    let sq_diffs: f64 = x.iter().map(|xi| (xi - avg).powi(2)).sum();
    sq_diffs / (n - 1.0)
}

// Exercise 2: z-score standardization via `.map()` + `.collect()`
fn standardize(x: &[f64]) -> Vec<f64> {
    let avg = mean(x);
    let std_dev = variance(x).sqrt();
    x.iter().map(|xi| (xi - avg) / std_dev).collect()
}

fn main() {
    let x = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    println!("Variance is: {:.2}", variance(&x));

    let standardized = standardize(&x);
    println!("Standardized: {:?}", standardized);
}
