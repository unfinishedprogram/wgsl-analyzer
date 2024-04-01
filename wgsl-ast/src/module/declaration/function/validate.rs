use super::{Function, FunctionBody, UserDefined};
use crate::{
    diagnostic::Diagnostic,
    front::ast::statement::{declaration::Declaration, Statement},
    module::{
        declaration::function::scope::ScopeStore, module_scope::ModuleScope, type_store::TypeStore,
    },
};

impl Function {
    pub fn validate_body(
        &self,
        module_scope: &ModuleScope,
        type_store: &TypeStore,
    ) -> Result<FunctionBody, Vec<Diagnostic>> {
        match self {
            Function::UserDefined(user_defined) => user_defined.validate(module_scope, type_store),
            Function::Builtin(_) => unreachable!("Builtin functions should never be validated"),
        }
    }
}

impl UserDefined {
    pub fn validate(
        &self,
        module_scope: &ModuleScope,
        type_store: &TypeStore,
    ) -> Result<FunctionBody, Vec<Diagnostic>> {
        let mut diagnostics: Vec<Diagnostic> = vec![];

        let FunctionBody::Unprocessed(body) = &self.body else {
            unreachable!("Function::validate should only be called on unprocessed functions")
        };

        let function_scope = ScopeStore::default();

        for statement in body {
            match statement.inner {
                Statement::Declaration(Declaration::Function(_)) => diagnostics.push(
                    Diagnostic::error("function definitions can only appear at module scope")
                        .span(statement.span),
                ),
                Statement::Declaration(Declaration::TypeAlias(_)) => diagnostics.push(
                    Diagnostic::error("type alias definitions can only appear at module scope")
                        .span(statement.span),
                ),
                Statement::Declaration(Declaration::Struct(_)) => diagnostics.push(
                    Diagnostic::error("struct definitions can only appear at module scope")
                        .span(statement.span),
                ),
                _ => {}
            }
        }

        if diagnostics.is_empty() {
            Ok(FunctionBody::Validated(function_scope))
        } else {
            Err(diagnostics)
        }
    }
}
