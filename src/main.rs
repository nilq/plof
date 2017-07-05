mod plof;

use plof::syntax;
use syntax::lexer::{BlockTree, process_branch};
use syntax::parser::{Traveler, Parser};

use std::rc::Rc;

fn main() {
    let test = r#"
str a  = "hey, " ++ "world"
bool b = 10
num c  = 1 + 1
any d  = a ++ c

a
b
c
d
    "#;

    let mut blocks = BlockTree::new(test, 0);
    let indents    = blocks.indents();

    let root = blocks.tree(&indents);
    let done = process_branch(&root);

    println!("{:#?}\n------{}\n------", done, test);

    let mut parser = Parser::new(Traveler::new(done.clone()));

    let mut symtab = Rc::new(syntax::SymTab::new_global());
    let mut env    = Rc::new(syntax::Env::new_global());

    match parser.parse() {
        Err(why)  => println!("error: {}", why),
        Ok(stuff) => {
            println!("{:#?}\n------", stuff);

            for s in stuff.iter() {
                s.visit(&symtab, &env);
            }

            println!("------\n{:?}", env);
            println!("------\n{:?}", symtab);
        },
    }
}
