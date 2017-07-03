mod plof;

use plof::syntax;
use syntax::lexer::{BlockTree, process_branch};

fn main() {
    let test = r#"
str (str name) greet =
  say "yes hello, " ++ name
    "#;

    let mut blocks = BlockTree::new(test, 0);
    let indents    = blocks.indents();

    let root = blocks.tree(&indents);
    let done = process_branch(&root);

    println!("{:#?}", done)
}
