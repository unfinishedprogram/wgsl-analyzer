use crate::module::store::handle::Handle;

pub struct ScopeStore {
    pub scopes: Vec<Scope>,
    pub root: Handle<Scope>,
}

// A scope is the module representation of the ast Block
#[derive(Clone)]
pub struct Scope {
    pub parent: Option<Handle<Scope>>,
}

impl Default for ScopeStore {
    fn default() -> Self {
        let root = Handle::new(0);
        Self {
            scopes: vec![Scope { parent: None }],
            root,
        }
    }
}

impl ScopeStore {
    pub fn root(&self) -> Handle<Scope> {
        self.root
    }

    pub fn insert_child(&mut self, parent: Handle<Scope>) -> Handle<Scope> {
        let handle = Handle::new(self.scopes.len());
        self.scopes.push(Scope {
            parent: Some(parent),
        });
        handle
    }
}
