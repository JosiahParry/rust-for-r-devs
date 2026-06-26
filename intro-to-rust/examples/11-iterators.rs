fn mean(x: &[f64]) -> f64 {
    let total: f64 = x.iter().sum();
    total / x.len() as f64
}

fn main() {
    // Exercise 1: mean via `.iter()`
    let nums = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    println!("Mean is: {}", mean(&nums));

    // Exercise 2: print index + value for even values only
    let nums = vec![1, 2, 3, 4, 5, 6];
    for (i, n) in nums.iter().enumerate() {
        if n % 2 == 0 {
            println!("Index {i}: {n} is even");
        }
    }
}
