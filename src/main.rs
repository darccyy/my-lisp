use std::fs;

use lips::run_steps;

fn main() {
    let file = fs::read_to_string("./test.lips").unwrap();

    run_steps(&file);
}
