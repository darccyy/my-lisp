use std::fs;

use lips::{compile, parse, run};

fn main() {
    let file = fs::read_to_string("./test.lips").unwrap();

    let parsed = parse(&file);

    println!("{:?}", parsed);

    let compiled = compile(parsed);

    println!("{:?}", compiled);

    let result = run(compiled);

    println!("{:?}", result);
}
