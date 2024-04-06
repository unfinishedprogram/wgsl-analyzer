use std::collections::HashMap;

use crate::{
    diagnostic::Diagnostic,
    front::{ast::statement::declaration, span::Spanned},
};

use super::{declaration::function::Function, type_store::TypeStore};

// The only scope without a parent scope, is the module scope
// Functions can only be declared in the module's scope
#[derive(Default)]
pub struct ModuleScope {
    // Acts as a handle to the scopes struct
    pub functions: HashMap<String, Function>,
    pub variables: HashMap<String, usize>,
}

impl ModuleScope {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ModuleScope {
    pub fn insert_pre_declared_functions(
        &mut self,
        type_store: &mut TypeStore,
    ) -> Result<(), Vec<Diagnostic>> {
        let functions = Function::get_all_builtin_functions(type_store)?;
        for function in functions {
            match function {
                Function::Builtin(function) => {
                    self.functions
                        .insert(function.ident.to_owned(), Function::Builtin(function));
                },
                _ => unreachable!("User defined functions should never be returned form get_all_builtin_functions")
            }
        }
        Ok(())
    }

    // Two phase
    // First insert all function "headers"
    // THEN check the validity of the bodies
    // Otherwise, the order that functions are declared in the source code matters
    pub fn insert_function_declarations(
        &mut self,
        type_store: &mut TypeStore,
        functions: &[Spanned<declaration::Function>],
    ) -> Result<(), Vec<Diagnostic>> {
        for function in functions {
            let res = Function::unprocessed_from_ast(type_store, function.clone())?;
            self.functions.insert(function.ident.inner.clone(), res);
        }

        Ok(())
    }
}
