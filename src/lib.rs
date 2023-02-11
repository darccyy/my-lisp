#![feature(box_syntax, decl_macro, macro_metavar_expr)]

mod compile;
mod parse;
mod run;

pub use {compile::*, parse::*, run::*};

/// Run script file
pub fn run(file: &str) -> Value {
    Parsed::from(file).compile().run()
}

/// Run script file, printing parse and compile steps
pub fn run_steps(file: &str) -> Value {
    let parsed = Parsed::from(file);
    println!("\t\x1b[36mᴘᴀʀsᴇ\x1b[0m\n{:?}", parsed);

    let compiled = parsed.compile();
    println!("\t\x1b[36mᴄᴏᴍᴘɪʟᴇ\x1b[0m\n{:?}", compiled);

    let result = compiled.run();
    println!("\t\x1b[36mʀᴜɴ\x1b[0m\n{:?}", result);

    result
}
