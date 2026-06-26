// `Result<T, E>` is an enum: `Ok(T)` on success, `Err(E)` on failure.
// T is the type returned when things go well, E the type returned on error.
// You can think of it as:  enum Result { Ok(MyReturnType), Err(MyErrorType) }
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Because `main` itself can now return an error, we can use `?`.
// `?` unwraps `Ok(T)` to its value, OR returns the `Err(E)` early
// and stops the computation.
fn main() -> Result<(), String> {
    // Recoverable: `match` lets us decide how to behave on an error.
    // The error is just data, so we can handle it however we like.
    match divide(10.0, 0.0) {
        Ok(value) => println!("10 / 0 = {value}"),
        Err(msg) => println!("Error: {msg}"),
    }

    // Propagating: `?` bubbles the error up instead of recovering.
    let result = divide(10.0, 2.0)?;
    println!("10 / 2 = {result}");

    Ok(())
}
