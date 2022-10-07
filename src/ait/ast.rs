#[derive(Debug)]
pub enum Ty {
    Int,
    String,
    Generic(String),
}

#[derive(Debug)]
pub struct StackEffect {
    before: Vec<Ty>,
    after: Vec<Ty>,
}

impl StackEffect {
    pub fn new(before: Vec<Ty>, after: Vec<Ty>) -> Self {
        Self { before, after }
    }
}

#[derive(Debug)]
pub enum Constant {
    Int(i32),
    Str(String),
}

#[derive(Debug)]
pub enum Expr {
    Int(i32),
    Str(String),
    Constant(i32),
    FunctionCall(String)
}

#[derive(Debug)]
pub struct ConstantDefinition {
    index: i32,
    value: Constant,
}

impl ConstantDefinition {
    pub fn new(index: i32, value: Constant) -> Self {
        Self { index, value }
    }
}

#[derive(Debug)]
pub struct Instruction {
    name: String,
    expression: Option<Expr>,
}

impl Instruction {
    pub fn new(name: String, expression: Option<Expr>) -> Self {
        Self { name, expression }
    }
}

#[derive(Debug)]
pub struct FunctionDefinition {
    name: String,
    effect: StackEffect,
    instructions: Vec<Instruction>,
}

impl FunctionDefinition {
    pub fn new(name: String, effect: StackEffect, instructions: Vec<Instruction>) -> Self {
        Self {
            name,
            effect,
            instructions,
        }
    }
}

#[derive(Debug)]
pub enum Ast {
    Error,
    Constant(ConstantDefinition),
    Function(FunctionDefinition),
}
