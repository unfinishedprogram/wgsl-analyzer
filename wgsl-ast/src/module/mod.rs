use crate::{
    diagnostic::Diagnostic,
    front::{
        ast::{
            create_ast,
            statement::{declaration::Declaration, Statement},
            tokenize, AstResult,
        },
        span::{SpanAble, Spanned},
    },
};

pub mod declaration;
mod scope;
mod store;

// A WGSL Module https://www.w3.org/TR/WGSL/#wgsl-module
// Represents a complete WGSL module, but with only minimal validation
// This is the data-structure that the LSP interacts with
// Therefore, it must maintain span information, in order to provide document hints/completions

pub struct Module {
    pub source: String,
    pub ast: Vec<Spanned<Statement>>,
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
    pub fn from_ast(ast: AstResult, source: String) -> Result<Self, Diagnostic> {
        if ast.errors.is_empty() {
            Ok(Self {
                source,
                ast: ast.ast,
            })
        } else {
            let err = &ast.errors[0];
            Err(err.into())
        }
    }

    pub fn from_source(source: &str) -> Result<Self, Diagnostic> {
        let token_result = tokenize(source);
        let ast_result = create_ast(&token_result);

        Self::from_ast(ast_result, source.to_owned())
    }

    pub fn declarations(&self) -> Vec<Spanned<Declaration>> {
        self.ast
            .iter()
            .filter_map(|Spanned { inner, span }| match inner {
                Statement::Declaration(declaration) => Some(declaration.clone().with_span(*span)),
                _ => None,
            })
            .collect()
    }
}
