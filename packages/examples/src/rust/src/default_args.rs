use extendr_api::prelude::*;

/// @export
#[extendr]
fn check_default(#[extendr(default = "NULL")] x: Robj) -> bool {
    x.is_null()
}

/// @export
#[extendr]
fn greet(name: &str, #[extendr(default = "FALSE")] loud: bool) -> String {
    let greeting = format!("Hello, {}", name);
    if loud {
        greeting.to_uppercase()
    } else {
        greeting
    }
}

/// @export
#[extendr]
fn multiply(
    x: f64,
    #[extendr(default = "1.0")] multiplier: f64,
    #[extendr(default = "FALSE")] round_result: bool,
) -> f64 {
    let result = x * multiplier;
    if round_result {
        result.round()
    } else {
        result
    }
}

extendr_module! {
    mod default_args;
    fn check_default;
    fn greet;
    fn multiply;
}
