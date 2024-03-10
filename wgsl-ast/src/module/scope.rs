use std::collections::HashMap;

use crate::{
    diagnostic::Diagnostic,
    front::{
        ast::statement::declaration::{self, Declaration},
        span::Spanned,
    },
};

use super::{
    store::{handle::Handle, Store},
    type_store::TypeStore,
};

// Scopes can only ever be added to, never removed
#[derive(Default)]
pub struct Scopes {
    pub scopes: Vec<Scope>,
}

impl Scopes {
    pub fn create_scope(&mut self, parent: Option<Handle<Scope>>) -> Handle<Scope> {
        let handle = Handle::new(self.scopes.len());

        self.scopes.push(Scope {
            parent,
            variables: Default::default(),
            functions: Default::default(),
        });

        handle
    }
}

// TODO make a macro for this
impl Store<Scope> for Scopes {
    fn insert(&mut self, value: Scope) -> Handle<Scope> {
        let handle = Handle::new(self.scopes.len());
        self.scopes.push(value);
        handle
    }

    fn get(&self, handle: &Handle<Scope>) -> &Scope {
        &self.scopes[handle.index()]
    }

    fn get_mut(&mut self, handle: &Handle<Scope>) -> &mut Scope {
        &mut self.scopes[handle.index()]
    }
}

// The only scope without a parent scope, is the module scope
// Functions can only be declared in the module's scope
pub struct Scope {
    // Acts as a handle to the scopes struct
    pub parent: Option<Handle<Scope>>,
    pub variables: HashMap<String, usize>,
    pub functions: HashMap<String, usize>,
}

impl Scope {
    pub fn insert_pre_declared_functions(
        &mut self,
        type_store: &mut TypeStore,
    ) -> Result<(), Diagnostic> {
        Ok(())
    }

    pub fn insert_function_declarations(
        &mut self,
        type_store: &mut TypeStore,
        functions: &[Spanned<declaration::Function>],
    ) -> Result<(), Diagnostic> {
        for function in functions {
            todo!()
        }

        Ok(())
    }
}
