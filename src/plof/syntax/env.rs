use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use super::{RunResult, RunError};

use super::parser::Type;

pub struct Env {
    parent: Option<Rc<Env>>,
    types: RefCell<Vec<Type>>,
}

impl Env {
    pub fn new(parent: Rc<Env>, types: &Vec<Type>) -> Env {
        Env {
            parent: Some(parent),
            types: RefCell::new(types.clone()),
        }
    }

    pub fn new_global() -> Env {
        Env {
            parent: None,
            types: RefCell::new(Vec::new()),
        }
    }

    pub fn new_partial(parent: Rc<Env>, types: &[Type], size: usize) -> Env {
        let mut stack = types.to_vec();
        for _ in 0 .. size - types.len() {
            stack.push(Type::Undefined)
        }

        Env {
            parent: Some(parent),
            types: RefCell::new(stack),
        }
    }

    pub fn set_type(&self, index: usize, env_index: usize, t: Type) -> RunResult<()> {
        if env_index == 0 {
            let mut types = self.types.borrow_mut();
            match types.get_mut(index) {
                Some(v) => {
                    *v = t;
                    Ok(())
                },
                None => Err(RunError::new(&format!("can't set type of invalid type index: {}", index))),
            }
        } else {
            match self.parent {
                Some(ref p) => p.set_type(index, env_index - 1, t),
                None => Err(RunError::new(&format!("can't set type with invalid env index: {}", env_index))),
            }
        }
    }

    pub fn get_type(&self, index: usize, env_index: usize) -> RunResult<Type> {
        if env_index == 0 {
            match self.types.borrow().get(index) {
                Some(v) => Ok(v.clone()),
                None    => Err(RunError::new(&format!("can't get type of invalid type index: {}", index))),
            }
        } else {
            match self.parent {
                Some(ref p) => p.get_type(index, env_index - 1),
                None => Err(RunError::new(&format!("can't get type with invalid env index: {}", index))),
            }
        }
    }

    pub fn visualize(&self, env_index: usize) {
        if env_index > 0 {
            if let Some(ref p) = self.parent {
                p.visualize(env_index - 1);
                println!("------------------------------");
            }
        }

        for (i, v) in self.types.borrow().iter().enumerate() {
            println!("({} : {}) = {:?}", i, env_index, v)
        }
    }

    fn dump(&self, f: &mut fmt::Formatter, env_index: usize) -> fmt::Result {
        if env_index > 0 {
            if let Some(ref p) = self.parent {
                try!(p.dump(f, env_index - 1));
                try!(writeln!(f, "------------------------------"));
            }
        }

        for (i, v) in self.types.borrow().iter().enumerate() {
            try!(writeln!(f, "({} : {}) = {:?}", i, env_index, v))
        }

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.types.borrow().len()
    }

    pub fn grow(&self) {
        self.types.borrow_mut().push(Type::Undefined)
    }
}

impl fmt::Debug for Env {
    fn fmt(&self, f : &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(self.dump(f, 0));
        Ok(())
    }
}
