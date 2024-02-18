use crate::{
    diagnostic::Diagnostic,
    front::{ast::statement::Statement, span::Spanned},
};

pub mod declaration;
mod scope;
mod store;

// A WGSL Module https://www.w3.org/TR/WGSL/#wgsl-module
// Represents a complete WGSL module, but with only minimal validation
// This is the data-structure that the LSP interacts with
// Therefore, it must maintain span information, in order to provide document hints/completions

pub struct Module {
    // directives: Vec<Directive>, TODO
    // functions: Vec<Function>,
    // types: Vec<Type>,
    // variables: Vec<Variable>,
    // constants: Vec<Constant>,
    // statements: Vec<Statement>,
    // attributes: Vec<Attribute>,
    // entry_points: Vec<EntryPoint>,
    // diagnostics: Vec<Diagnostic>,
}

// Validation must be done in multiple passes:
impl Module {
    pub fn from_ast(ast: Vec<Spanned<Statement>>) -> Result<Self, Diagnostic> {
        Ok(Self {})
    }
}
