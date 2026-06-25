#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn euclidean_distance(&self, destination: &Self) -> f64 {
        let x_diff = (self.x - destination.x).powi(2);
        let y_diff = (self.y - destination.y).powi(2);
        (x_diff + y_diff).sqrt()
    }

    // Bonus: distance on a sphere via the Haversine formula
    fn haversine_distance(&self, destination: &Self) -> f64 {
        let radius = 6_371_008.7714; // Earth's mean radius in meters
        let theta1 = self.y.to_radians(); // Latitude of point 1
        let theta2 = destination.y.to_radians(); // Latitude of point 2
        let delta_theta = (destination.y - self.y).to_radians(); // Delta Latitude
        let delta_lambda = (destination.x - self.x).to_radians(); // Delta Longitude

        let a = (delta_theta / 2f64).sin().powi(2)
            + theta1.cos() * theta2.cos() * (delta_lambda / 2f64).sin().powi(2);

        2f64 * a.sqrt().asin() * radius
    }
}

fn main() {
    let a = Point::new(0.0, 5.0);
    let b = Point::new(-10.0, 1.5);
    let distance = a.euclidean_distance(&b);
    println!("Distance between a and b is {distance}");

    let haversine = a.haversine_distance(&b);
    println!("Haversine distance between a and b is {haversine}");
}
