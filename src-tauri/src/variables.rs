use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VARIABLES: Mutex<HashMap<String, f64>> = Mutex::new(initial_variables());
}

pub fn initial_variables() -> HashMap<String, f64> {
    HashMap::from([
        ("e".to_string(), std::f64::consts::E),
        ("pi".to_string(), std::f64::consts::PI),
        ("phi".to_string(), (1.0 + 5.0_f64.sqrt()) / 2.0),
    ])
}

pub fn reset_variables() {
    let mut variables = VARIABLES.lock().unwrap();
    *variables = initial_variables();
}