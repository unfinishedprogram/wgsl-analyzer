use crate::{
    diagnostic::Diagnostic,
    front::{
        ast::{
            create_ast,
            statement::{declaration::Declaration, Statement},
            tokenize, Ast,
        },
        span::{SpanAble, Spanned},
        token::Token,
    },
};

use self::{
    scope::{Scope, Scopes},
    store::handle::Handle,
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
    pub identifiers: Vec<Spanned<String>>,
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

            let identifiers = ast
                .tokens
                .iter()
                .filter_map(|(token, span)| match *token {
                    Token::Ident(s) => Some(s.to_owned().with_span(*span)),
                    _ => None,
                })
                .collect();

            Ok(Self {
                source,
                ast: ast.statements,
                scopes,
                type_store,
                module_scope,
                identifiers,
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

    pub fn ident_at_position(&self, offset: &usize) -> Option<&Spanned<String>> {
        let res = self
            .identifiers
            .iter()
            .find(|Spanned { span, .. }| span.into_range().contains(offset));

        log::info!("{offset} : {res:?}");

        res
    }
}
