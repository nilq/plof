mod plof;

use plof::syntax;
use syntax::lexer::{BlockTree, process_branch};
use syntax::parser::{Traveler, Parser, Type};
use syntax::{SymTab, Env};

use std::rc::Rc;

use std::io::prelude::*;
use std::error::Error;

use std::fs;
use std::fs::File;
use std::env;
use std::path::Path;

use std::fs::metadata;

fn add_global(sym: &SymTab, env: &Env, name: &str, t: Type) {
    let i = sym.add_name(name);

    if i >= env.size() {
        env.grow();
    }
    
    env.set_type(i, 0, t);
}

fn add_lua_standard(sym: &SymTab, env: &Env) {
    add_global(sym, env, "print", Type::Lambda(Rc::new(vec![Type::Nil, Type::Any])));
    add_global(sym, env, "tostring", Type::Str);
}

fn do_path(path: &str) {
    let meta = metadata(path).unwrap();
    
    if meta.is_file() {
        file(path)
    } else {
        let paths = fs::read_dir(path).unwrap();
        
        for path in paths {
            let path = format!("{}", path.unwrap().path().display());
            let split: Vec<&str> = path.split(".").collect();

            match split.get(1) {
                Some(n) if *n == "plof" => (),
                _ => continue,
            }

            do_path(&format!("{}", path))
        }
    }
}

fn file(path: &str) {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why.description()),
        Ok(file) => file,
    };    

    let mut s = String::new();
        
    match file.read_to_string(&mut s) {
        Err(why) => panic!("failed to read {}: {}", display,  why.description()),
        Ok(_)    => {
            println!("building: {}", display);
            
            let mut blocks = BlockTree::new(&s, 0);
            let indents    = blocks.indents();

            let root = blocks.tree(&indents);
            let done = process_branch(&root);

            let mut parser = Parser::new(Traveler::new(done.clone()));

            let symtab = Rc::new(syntax::SymTab::new_global());
            let env    = Rc::new(syntax::Env::new_global());

            add_lua_standard(&symtab, &env);

            match parser.parse() {
                Err(why)  => println!("error: {}", why),
                Ok(stuff) => {                    
                    for s in stuff.iter() {
                        match s.visit(&symtab, &env) {
                            Ok(()) => (),
                            Err(e) => {
                                println!("{}", e);
                                return
                            },
                        }
                    }
                    
                    let mut output = String::new();
                    
                    for s in stuff.iter() {
                        output.push_str(&format!("{}", s))
                    }
                    
                    let split_name = path.file_name().unwrap().to_str().unwrap().split(".");
                    let split: Vec<&str> = split_name.collect();
                    
                    let parent_path = match path.parent() {
                        Some(p) => match p.file_name() {
                            Some(path) => path.to_str().unwrap(),
                            None       => ".",
                        },
                        None    => ".",
                    };

                    let output_name = format!("{}/{}.lua", parent_path, split.get(0).unwrap());

                    let mut output_file = File::create(output_name).unwrap();
                    match output_file.write_all(output.as_bytes()) {
                        Ok(_)    => (),
                        Err(why) => println!("{}", why.description())
                    }
                },
            }
        }
    }
}

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(a) => do_path(&a),
        None    => println!(r"
the plof language

usage:
  plof <file> or <folder>
        "),
    }
}
