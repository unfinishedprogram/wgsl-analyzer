use super::{scope::Scope, FunctionBody, UserDefined};
use crate::{
    diagnostic::Diagnostic,
    front::{
        ast::statement::{declaration::Declaration, Statement},
        span::{SpanAble, Spanned, WithSpan},
    },
    module::{
        declaration::function::scope::ScopeStore, module_scope::ModuleScope, store::handle::Handle,
        type_store::TypeStore,
    },
};

pub struct ValidationContext<'a> {
    scope_store: &'a mut ScopeStore,
    scope: Handle<Scope>,
    type_store: &'a TypeStore,
    module_scope: &'a ModuleScope,
}

impl<'a> ValidationContext<'a> {
    pub fn new(
        scope_store: &'a mut ScopeStore,
        scope: Handle<Scope>,
        type_store: &'a TypeStore,
        module_scope: &'a ModuleScope,
    ) -> Self {
        Self {
            scope_store,
            scope,
            type_store,
            module_scope,
        }
    }

    pub fn validate_function_stmt(&mut self, stmt: &Spanned<Statement>) -> Result<(), Diagnostic> {
        match stmt.as_inner() {
            Statement::Trivia => Ok(()),
            Statement::Compound(statements) => {
                let mut inner_ctx = self.create_inner_scope();
                for stmt in statements {
                    inner_ctx.validate_function_stmt(stmt)?
                }
                Ok(())
            }
            Statement::Assignment(_, _, _) => Ok(()),
            Statement::Increment(_) => Ok(()),
            Statement::Decrement(_) => Ok(()),
            Statement::Return(_) => Ok(()),
            Statement::Continue => Ok(()),
            Statement::Continuing(_, _) => Ok(()),
            Statement::Break => Ok(()),
            Statement::BreakIf(_) => Ok(()),
            Statement::If {
                if_block,
                else_if_blocks,
                else_block,
            } => Ok(()),
            Statement::Declaration(decl) => {
                self.validate_declaration_in_function(decl.with_span(stmt.span))
            }
            Statement::FuncCall(_) => Ok(()),
            Statement::Discard => Ok(()),
            Statement::Loop {
                loop_attributes,
                body_attributes,
                body,
            } => Ok(()),
            Statement::For {
                attributes,
                init,
                expression,
                update,
                body,
            } => Ok(()),
            Statement::While {
                attributes,
                expression,
                body,
            } => Ok(()),
            Statement::Switch {
                attributes,
                expression,
                body,
            } => Ok(()),
        }
    }

    pub fn validate_user_defined_function(
        &mut self,
        function: &UserDefined,
    ) -> Result<Handle<Scope>, Diagnostic> {
        let mut inner_ctx = self.create_inner_scope();
        let FunctionBody::Unprocessed(statements) = &function.body else {
            unreachable!("validate should only be called on unprocessed functions")
        };

        for stmt in statements {
            inner_ctx.validate_function_stmt(stmt)?;
        }

        Ok(inner_ctx.scope)
    }

    pub fn validate_declaration_in_function(
        &mut self,
        decl: Spanned<&Declaration>,
    ) -> Result<(), Diagnostic> {
        match decl.inner {
            Declaration::Variable(_) => Ok(()),
            Declaration::LocalConstant(_) => Ok(()),

            Declaration::ModuleConstant(_) => Err(Diagnostic::error(
                "Module constants can only appear in module scope",
            )
            .span(decl.span)),
            Declaration::TypeAlias(_) => Err(Diagnostic::error(
                "Type aliases can only appear in module scope",
            )
            .span(decl.span)),
            Declaration::Struct(_) => Err(Diagnostic::error(
                "Struct declarations can only appear in module scope",
            )
            .span(decl.span)),
            Declaration::Function(_) => Err(Diagnostic::error(
                "Functions declarations cannot be nested within function bodies",
            )
            .span(decl.span)),
        }
    }

    pub fn create_inner_scope(&mut self) -> ValidationContext {
        let inner_scope = self.scope_store.insert_child(self.scope);

        ValidationContext::new(
            self.scope_store,
            inner_scope,
            self.type_store,
            self.module_scope,
        )
    }
}
