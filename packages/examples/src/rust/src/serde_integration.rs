use extendr_api::deserializer::from_robj;
use extendr_api::error::*;
use extendr_api::prelude::*;
use extendr_api::serializer::to_robj;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    x: f64,
    y: f64,
}

/// @export
#[extendr]
fn point_from_r(x: Robj) {
    let point = from_robj::<Point>(&x);
    rprintln!("{point:?}");
}

/// @export
#[extendr]
fn round_trip(x: Robj) -> Result<Robj> {
    let point = from_robj::<Point>(&x)?;
    to_robj(&point)
}

/// @export
#[extendr]
fn replicate_point(x: Robj, n: i32) -> Result<Robj> {
    let point = from_robj::<Point>(&x)?;
    let points = vec![point; n as usize];
    to_robj(&points)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiPoint {
    x: Vec<f64>,
    y: Vec<f64>,
}

/// @export
#[extendr]
fn make_multipoint(x: Robj) -> Result<()> {
    let mpoint = from_robj::<MultiPoint>(&x)?;
    rprintln!("{mpoint:#?}");
    Ok(())
}

pub struct MPoint(Vec<Point>);

impl TryFrom<&Robj> for MPoint {
    type Error = Error;
    fn try_from(value: &Robj) -> std::result::Result<Self, Self::Error> {
        let point_df = List::try_from(value)?;
        let x_vec = Doubles::try_from(point_df.dollar("x")?)?;
        let y_vec = Doubles::try_from(point_df.dollar("y")?)?;
        let inner = x_vec
            .into_iter()
            .zip(y_vec.into_iter())
            .map(|(x, y)| Point { x: x.0, y: y.0 })
            .collect::<Vec<_>>();
        Ok(MPoint(inner))
    }
}

/// @export
#[extendr]
fn centroid(x: MPoint) -> Result<Robj> {
    let total = x.0.into_iter().fold((0.0, 0.0, 0.0), |mut acc, next| {
        acc.0 += next.x;
        acc.1 += next.y;
        acc.2 += 1.0;
        acc
    });
    let centroid = Point {
        x: total.0 / total.2,
        y: total.1 / total.2,
    };
    to_robj(&centroid)
}

extendr_module! {
    mod serde_integration;
    fn point_from_r;
    fn round_trip;
    fn replicate_point;
    fn make_multipoint;
    fn centroid;
}
