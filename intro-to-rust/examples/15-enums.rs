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
        ((self.x - destination.x).powi(2) + (self.y - destination.y).powi(2)).sqrt()
    }

    fn haversine_distance(&self, destination: &Self) -> f64 {
        let radius = 6_371_008.7714; // Earth's mean radius in meters
        let theta1 = self.y.to_radians();
        let theta2 = destination.y.to_radians();
        let delta_theta = (destination.y - self.y).to_radians();
        let delta_lambda = (destination.x - self.x).to_radians();

        let a = (delta_theta / 2f64).sin().powi(2)
            + theta1.cos() * theta2.cos() * (delta_lambda / 2f64).sin().powi(2);

        2f64 * a.sqrt().asin() * radius
    }

    // Exercise 2: pick the distance method based on the `Measure` variant
    fn distance(&self, destination: &Self, measure: &Measure) -> f64 {
        match measure {
            Measure::Euclidean => self.euclidean_distance(destination),
            Measure::Haversine => self.haversine_distance(destination),
        }
    }
}

// Exercise 1: an enum with a method that matches on its variants
enum Measure {
    Euclidean,
    Haversine,
}

impl Measure {
    fn ndim(&self) -> i32 {
        match self {
            Self::Euclidean => 2,
            Self::Haversine => 3,
        }
    }
}

fn main() {
    let a = Point::new(0.0, 5.0);
    let b = Point::new(-10.0, 1.5);

    println!("Euclidean is {}-dimensional", Measure::Euclidean.ndim());
    println!("Haversine is {}-dimensional", Measure::Haversine.ndim());

    println!("Euclidean distance: {}", a.distance(&b, &Measure::Euclidean));
    println!("Haversine distance: {}", a.distance(&b, &Measure::Haversine));
}
