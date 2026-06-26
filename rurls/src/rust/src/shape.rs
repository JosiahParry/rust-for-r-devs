use extendr_api::prelude::*;

#[derive(Debug, Clone)]
enum Shape {
    Triangle,
    Rectangle,
    Pentagon,
    Hexagon,
}

#[derive(Debug, Clone)]
#[extendr]
struct RShape(Shape);

#[extendr]
impl RShape {
    fn new(x: &str) -> anyhow::Result<Self> {
        match x {
            "triangle" => Ok(Self(Shape::Triangle)),
            "rectangle" => Ok(Self(Shape::Rectangle)),
            "pentagon" => Ok(Self(Shape::Pentagon)),
            "hexagon" => Ok(Self(Shape::Hexagon)),
            _ => Err(anyhow::anyhow!("unknown shape: {x}")),
        }
    }

    fn n_coords(&self) -> usize {
        match self.0 {
            Shape::Triangle => 3,
            Shape::Rectangle => 4,
            Shape::Pentagon => 4,
            Shape::Hexagon => 5,
        }
    }
}

extendr_module! {
    mod shape;
    impl RShape;
}
