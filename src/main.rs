mod plof;

use plof::syntax;
use syntax::lexer::{BlockTree, process_branch};
use syntax::parser::{Traveler, Parser};

fn main() {
    let test = r#"
add1 ((1 + 2) * 2) 10
    "#;

    let mut blocks = BlockTree::new(test, 0);
    let indents    = blocks.indents();

    let root = blocks.tree(&indents);
    let done = process_branch(&root);

    println!("#{:#?}\n------{}\n------", done, test);

    let mut parser = Parser::new(Traveler::new(done.clone()));

    match parser.parse() {
        Err(why)  => println!("error: {}", why),
        Ok(stuff) => {
            println!("{:#?}", stuff);
        },
    }
}
