#![feature(box_syntax, decl_macro, macro_metavar_expr)]

mod compile;
mod parse;
mod run;
mod value;

pub use {compile::*, parse::*, run::*, value::*};

/// Run script file
pub fn run(file: &str) -> Value {
    Tree::from(file).compile().run()
}

/// Run script file, printing parse and compile steps
pub fn run_steps(file: &str) -> Value {
    println!("\n\t\x1b[36mꜰɪʟᴇ\x1b[0m\n{}", file.trim());

    println!("\n\t\x1b[36mᴘᴀʀsᴇ\x1b[0m");
    let parsed = Tree::from(file);
    println!("{:?}", parsed);

    println!("\n\t\x1b[36mᴄᴏᴍᴘɪʟᴇ\x1b[0m");
    let compiled = parsed.compile();
    println!("{:?}", compiled);

    println!("\n\t\x1b[36mʀᴜɴ\x1b[0m");
    let result = compiled.run();

    println!("\n\t\x1b[36mʀᴇsᴜʟᴛ\n>>\x1b[0m {:?}", result);
    result
}
