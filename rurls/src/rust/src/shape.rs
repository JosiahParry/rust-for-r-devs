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

    fn get_extptr(robj: Robj) -> extendr_api::Result<ExternalPtr<Self>> {
        ExternalPtr::<Self>::try_from(robj)
    }
}

#[extendr]
fn parse_shapes(shapes: Strings) -> extendr_api::error::Result<List> {
    let mut res = shapes
        .into_iter()
        .map(|vi| RShape::new(vi).unwrap())
        .collect::<List>();

    res.set_class(&["shapes", "vctrs_vctr", "list"])?;
    Ok(res)
}

#[extendr]
fn format_shapes(x: List) -> Strings {
    x.into_iter()
        .map(|(_, robj)| match RShape::get_extptr(robj) {
            Ok(v) => Rstr::from(format!("{:?}", v.0)),
            Err(_) => Rstr::na(),
        })
        .collect()
}

extendr_module! {
    mod shape;
    impl RShape;
}
