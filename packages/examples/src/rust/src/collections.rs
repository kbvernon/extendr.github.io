use extendr_api::prelude::*;
use std::collections::HashMap;

/// @export
#[extendr]
fn test_hm_i32(mut x: HashMap<String, i32>) -> List {
    x.insert("inserted_value".to_string(), 314);
    List::try_from(x).unwrap()
}

/// @export
#[extendr]
fn test_hm_string(mut x: HashMap<String, Robj>) -> List {
    x.insert("inserted_value".to_string(), List::new(0).into());
    List::try_from(x).unwrap()
}

pub struct Point {
    x: f64,
    y: f64,
}

impl TryFrom<Robj> for Point {
    type Error = Error;

    fn try_from(value: Robj) -> std::result::Result<Self, Self::Error> {
        let inner_vec = Doubles::try_from(value)?;
        let x: f64 = inner_vec[0].0;
        let y: f64 = inner_vec[1].0;
        Ok(Point { x, y })
    }
}

impl From<Point> for Doubles {
    fn from(value: Point) -> Self {
        Doubles::from_values([value.x, value.y])
    }
}

impl From<Point> for Robj {
    fn from(value: Point) -> Self {
        Robj::from(Doubles::from(value))
    }
}

/// @export
#[extendr]
fn test_hm_custom_try_from(mut x: HashMap<&str, Point>) -> List {
    x.insert("inserted_value", Point { x: 3.0, y: 0.1415 });
    List::try_from(x).unwrap()
}

extendr_module! {
    mod collections;
    fn test_hm_i32;
    fn test_hm_string;
    fn test_hm_custom_try_from;
}
