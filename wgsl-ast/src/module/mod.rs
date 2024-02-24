use crate::{
    diagnostic::Diagnostic,
    front::{
        ast::{
            create_ast,
            statement::{declaration::Declaration, Statement},
            tokenize, Ast,
        },
        span::{SpanAble, Spanned},
    },
};

use self::{
    scope::{Scope, Scopes},
    store::{handle::Handle, Store},
    type_store::TypeStore,
};

pub mod declaration;
mod scope;
mod store;
mod type_store;

// A WGSL Module https://www.w3.org/TR/WGSL/#wgsl-module
// Represents a complete WGSL module, but with only minimal validation
// This is the data-structure that the LSP interacts with
// Therefore, it must maintain span information, in order to provide document hints/completions

pub struct Module {
    pub source: String,
    pub ast: Vec<Spanned<Statement>>,
    pub scopes: Scopes,
    pub module_scope: Handle<Scope>,
    pub type_store: TypeStore,
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
    pub fn from_ast(ast: Ast, source: String) -> Result<Self, Vec<Diagnostic>> {
        if ast.errors.is_empty() {
            let mut scopes = Scopes::default();
            let module_scope = scopes.create_scope(None);

            let declarations = ast.top_level_declarations().collect();

            let mut type_store = type_store::TypeStore::new();
            type_store.insert_declarations(declarations)?;

            Ok(Self {
                source,
                ast: ast.statements,
                scopes,
                type_store,
                module_scope,
            })
        } else {
            let errors = ast.errors.iter().map(Diagnostic::from).collect();
            Err(errors)
        }
    }

    pub fn from_source(source: &str) -> Result<Self, Vec<Diagnostic>> {
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
