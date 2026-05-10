use extendr_api::prelude::*;

#[derive(Debug)]
/// @export
#[extendr]
pub enum Shape {
    Triangle,
    Rectangle,
    Pentagon,
    Hexagon,
}

/// @export
#[extendr]
impl Shape {
    fn new(x: &str) -> Self {
        match x {
            "triangle" => Self::Triangle,
            "rectangle" => Self::Rectangle,
            "pentagon" => Self::Pentagon,
            "hexagon" => Self::Hexagon,
            &_ => unimplemented!(),
        }
    }

    fn n_coords(&self) -> usize {
        match &self {
            Shape::Triangle => 3,
            Shape::Rectangle => 4,
            Shape::Pentagon => 4,
            Shape::Hexagon => 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Inner(i32);

/// @export
#[extendr]
#[derive(Debug, Clone)]
pub struct Wrapper(Inner);

/// @export
#[extendr]
impl Wrapper {
    fn new(x: i32) -> Self {
        Wrapper(Inner(x))
    }

    fn value(&self) -> i32 {
        self.0 .0
    }
}

/// @export
#[extendr]
#[derive(Debug, Clone)]
pub struct Counter(i32);

/// @export
#[extendr]
impl Counter {
    /// Create a new Counter starting at zero.
    /// @return A `Counter` object.
    /// @export
    fn new() -> Self {
        Counter(0)
    }

    /// Increment the counter by one.
    /// @return Self (invisibly)
    /// @export
    fn increment(&mut self) {
        self.0 += 1;
    }

    /// Return the current count.
    /// @return An integer.
    /// @export
    fn value(&self) -> i32 {
        self.0
    }
}

extendr_module! {
    mod extendr_macro;
    impl Shape;
    impl Wrapper;
    impl Counter;
}
