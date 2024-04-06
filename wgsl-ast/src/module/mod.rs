use std::collections::HashMap;

use crate::{
    diagnostic::{Diagnostic, DiagnosticSource},
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
    declaration::function::{
        scope::{Scope, ScopeStore},
        validate::ValidationContext,
        Function, FunctionBody,
    },
    module_scope::ModuleScope,
    store::handle::Handle,
    type_store::TypeStore,
};

pub mod declaration;
mod module_scope;
mod store;
mod type_store;

// A WGSL Module https://www.w3.org/TR/WGSL/#wgsl-module
// Represents a complete WGSL module, but with only minimal validation
// This is the data-structure that the LSP interacts with
// Therefore, it must maintain span information, in order to provide document hints/completions

pub struct Module {
    pub source: String,
    pub ast: Vec<Spanned<Statement>>,
    pub module_scope: ModuleScope,
    pub type_store: TypeStore,
    pub scope_store: ScopeStore,
    pub identifiers: Vec<Spanned<String>>,
    pub diagnostics: Vec<Diagnostic>,
}

// Validation must be done in multiple passes:
impl Module {
    pub fn from_ast(ast: Ast, source: String) -> Result<Self, Vec<Diagnostic>> {
        let mut diagnostics: Vec<Diagnostic> = vec![];
        let mut scope_store: ScopeStore = ScopeStore::default();

        if !ast.errors.is_empty() {
            let errors = ast.errors.iter().map(Diagnostic::from).collect();
            return Err(errors);
        }

        let mut module_scope = ModuleScope::new();

        let declarations: Vec<Spanned<Declaration>> = ast.top_level_declarations().collect();

        let mut type_store = type_store::TypeStore::default();

        // Inserts types,
        // All types must be declared at module scope
        type_store
            .insert_type_declarations(&declarations)
            .extend(&mut diagnostics);

        module_scope
            .insert_pre_declared_functions(&mut type_store)
            .extend(&mut diagnostics);

        // Inserts user-declared functions
        // All user-defined functions must be defined at module scope
        let functions: Vec<_> = ast.function_declarations().collect();
        module_scope
            .insert_function_declarations(&mut type_store, &functions)
            .extend(&mut diagnostics);

        {
            let mut validated_functions: HashMap<String, Handle<Scope>> = Default::default();

            for (key, function) in module_scope.functions.iter() {
                if let Function::UserDefined(function) = function {
                    let function_scope = scope_store.insert_child(scope_store.root());
                    let res = {
                        let mut ctx = ValidationContext::new(
                            &mut scope_store,
                            function_scope,
                            &type_store,
                            &module_scope,
                        );
                        ctx.validate_user_defined_function(function)
                    };

                    match res {
                        Ok(scope) => {
                            validated_functions.insert(key.clone(), scope);
                        }
                        Err(diagnostic) => diagnostics.push(diagnostic),
                    };
                }
            }

            for (key, scope) in validated_functions {
                if let Some(Function::UserDefined(function)) = module_scope.functions.get_mut(&key)
                {
                    function.inner.body = FunctionBody::Validated(scope);
                }
            }
        }

        // We use this list of identifiers, to enable Ident picking/autocompletion in IDE
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
            module_scope,
            type_store,
            identifiers,
            diagnostics,
            scope_store,
        })
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
