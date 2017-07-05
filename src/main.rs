mod plof;

use plof::syntax;
use syntax::lexer::{BlockTree, process_branch};
use syntax::parser::{Traveler, Parser};

use std::rc::Rc;

fn main() {
    let test = r#"
str (str a, str b) concat = a ++ b
str (a, b) add = a + b
str (a) add1 = a ++ " hello"
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
                match s.visit(&symtab, &env) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("{}", e);
                        return
                    },
                }
            }

            println!("------\n{:?}", env);
            println!("------\n{:?}", symtab);
        },
    }
}
