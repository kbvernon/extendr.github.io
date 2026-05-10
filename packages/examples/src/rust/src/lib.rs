use extendr_api::prelude::*;

mod collections;
mod default_args;
mod extendr_macro;
mod into_list;
mod serde_integration;

extendr_module! {
    mod examples;
    use collections;
    use default_args;
    use extendr_macro;
    use into_list;
    use serde_integration;
}
