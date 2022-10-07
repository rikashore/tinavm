use super::ast::Ast;

#[derive(Debug)]
pub struct Quarry {
    pub name: String,
    pub ast: Vec<Ast>,
}

impl Quarry {
    pub fn new(name: String, ast: Vec<Ast>) -> Self {
        Self { name, ast }
    }
}
