use anyhow::Result;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // `.parse::<f64>()` returns a `Result` whose error type belongs to the
    // standard library. With `anyhow::Result`, `?` accepts (almost) any error
    // type, so we never have to name it ourselves.
    fn from_strings(x: &str, y: &str) -> Result<Self> {
        let x = x.parse::<f64>()?;
        let y = y.parse::<f64>()?;
        Ok(Point { x, y })
    }
}

// `anyhow::Result<()>` lets `?` bubble up any error from inside `main`.
fn main() -> Result<()> {
    let point = Point::from_strings("3.0", "0.14")?;
    println!("Parsed {point:?}");

    // Try changing "0.14" to "abc" above: the parse fails, `?` returns the
    // error, and the program prints it and exits non-zero.

    Ok(())
}
