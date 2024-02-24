use std::collections::HashMap;

use super::store::{handle::Handle, Store};

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

pub struct Scope {
    // Acts as a handle to the scopes struct
    pub parent: Option<Handle<Scope>>,
    pub variables: HashMap<String, usize>,
    pub functions: HashMap<String, usize>,
}
