use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use super::store::{handle::Handle, Store};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ScopeID(pub usize);

// Scopes can only ever be added to, never removed
pub struct Scopes {
    pub scopes: Vec<Scope>,
}

impl Scopes {
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    pub fn create_scope(&mut self, parent: Option<Handle<Scope>>) -> Handle<Scope> {
        let handle = Handle::new(self.scopes.len());

        self.scopes.push(Scope {
            parent,
            types: Default::default(),
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

    fn get(&self, handle: Handle<Scope>) -> &Scope {
        &self.scopes[handle.index()]
    }

    fn get_mut(&mut self, handle: Handle<Scope>) -> &mut Scope {
        &mut self.scopes[handle.index()]
    }
}

pub struct Scope {
    // Acts as a handle to the scopes struct
    parent: Option<Handle<Scope>>,
    types: HashMap<String, usize>,
    variables: HashMap<String, usize>,
    functions: HashMap<String, usize>,
}

pub struct Identifier {
    pub name: String,
    pub scope: usize,
}

impl Index<ScopeID> for Scopes {
    type Output = Scope;
    fn index(&self, index: ScopeID) -> &Self::Output {
        &self.scopes[index.0]
    }
}

impl IndexMut<ScopeID> for Scopes {
    fn index_mut(&mut self, index: ScopeID) -> &mut Self::Output {
        &mut self.scopes[index.0]
    }
}
