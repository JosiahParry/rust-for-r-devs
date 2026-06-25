#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let point = Point { x: 3.0, y: 0.14 };
    println!("The point is {:?}", point);
    let Point { x, y } = point;
    println!("x is {x}, y is {y}");
}
