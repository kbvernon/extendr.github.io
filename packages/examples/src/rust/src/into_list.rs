use extendr_api::prelude::*;

#[derive(IntoList)]
pub struct Person {
    name: String,
    age: i32,
}

/// @export
#[extendr]
fn create_person() -> Person {
    Person {
        name: String::from("Alice"),
        age: 30,
    }
}

#[derive(IntoList)]
pub struct Analysis {
    id: i32,
    values: Vec<f64>,
    passed: bool,
}

/// @export
#[extendr]
fn run_analysis() -> Analysis {
    Analysis {
        id: 123,
        values: vec![1.5, 2.7, 3.9],
        passed: true,
    }
}

#[derive(IntoList)]
pub struct Config {
    setting: String,
    value: i32,
    #[into_list(ignore)]
    internal_cache: Vec<u8>,
}

/// @export
#[extendr]
fn get_config() -> Config {
    Config {
        setting: String::from("timeout"),
        value: 30,
        internal_cache: vec![1, 2, 3],
    }
}

/// @export
#[extendr]
pub struct StatefulCounter {
    count: i32,
}

/// @export
#[extendr]
impl StatefulCounter {
    fn new() -> Self {
        StatefulCounter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn get(&self) -> i32 {
        self.count
    }
}

extendr_module! {
    mod into_list;
    fn create_person;
    fn run_analysis;
    fn get_config;
    impl StatefulCounter;
}
