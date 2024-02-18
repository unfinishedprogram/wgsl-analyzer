use crate::{
    diagnostic::Diagnostic,
    front::ast::{create_ast, tokenize, AstResult},
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
    pub fn from_ast(ast: AstResult) -> Result<Self, Diagnostic> {
        if ast.errors.is_empty() {
            Ok(Self {})
        } else {
            let err = &ast.errors[0];
            Err(err.into())
        }
    }

    pub fn from_source(source: &str) -> Result<Self, Diagnostic> {
        let token_result = tokenize(source);
        let ast_result = create_ast(&token_result);
        Self::from_ast(ast_result)
    }
}
