use super::Token;

#[derive(Debug)]
pub enum ChunkValue {
    Source(String),
    Tokens(Vec<Token>),
    Block(Branch),
}

#[derive(Debug)]
pub struct Chunk {
    value: ChunkValue,
}

impl Chunk {
    pub fn new(value: ChunkValue) -> Chunk {
        Chunk {
            value,
        }
    }

    pub fn value(&self) -> &ChunkValue {
        &self.value
    }
}

#[derive(Debug)]
pub struct Branch {
    pub value: Vec<Chunk>,
}

impl Branch {
    pub fn new(value: Vec<Chunk>) -> Branch {
        Branch {
            value,
        }
    }
}

#[derive(Debug)]
pub struct BlockTree<'a> {
    source: &'a str,
    current_line: usize,
    inside: i32,
    inside_brace: i32,
}

#[allow(dead_code)]
impl<'a> BlockTree<'a> {
    pub fn new(source: &str, current_line: usize) -> BlockTree {
        BlockTree {
            source,
            current_line,
            inside: 0,
            inside_brace: 0,
        }
    }

    pub fn indents(&mut self) -> Vec<(usize, &'a str)> {
        let mut indents = Vec::new();
        let mut lines   = self.source.lines();
        while let Some(line) = lines.next() {
            let parts: Vec<&str> = line.split("~").collect();
            let ln = parts.get(0).unwrap().trim();

            if ln.len() > 0 {
                let indent = self.indent(&line);
                indents.push((indent, ln))
            }
        }
        indents
    }

    pub fn indent(&mut self, line: &str) -> usize {
        let mut pos: usize = 0;
        for c in line.chars() {
            match c {
                '{' => self.inside_brace += 1,
                '}' => self.inside_brace -= 1,
                '(' => self.inside += 1,
                ')' => self.inside -= 1,
                _ => (),
            }
        }

        for c in line.chars() {
            match c {
                ' ' | '\t' => {
                    if self.inside > 0 || self.inside_brace > 0 {
                        continue
                    }

                    pos += 1
                }
                _ => break,
            }
        }
        
        pos
    }

    pub fn tree(&mut self, indents: &Vec<(usize, &'a str)>) -> Branch {
        let mut branch = Branch::new(Vec::new());
        let line       = indents.get(self.current_line);
        let &(base_indent, _) = match line {
            Some(i) => i,
            None    => return branch,
        };

        while self.current_line < indents.len() {
            let (indent, line) = indents[self.current_line];
            if indent == base_indent {
                branch.value.push(Chunk::new(ChunkValue::Source(line.to_owned())))
            } else if indent < base_indent {
                self.current_line -= 1;
                return branch
            } else if indent > base_indent {
                branch.value.push(Chunk::new(ChunkValue::Block(self.tree(&indents))))
            }
            self.current_line += 1
        }
        branch
    }
}
