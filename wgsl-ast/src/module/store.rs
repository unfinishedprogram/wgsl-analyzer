use std::ops::{Index, IndexMut};

pub mod handle;

// There is no ability to remove, this means that a handle is always valid.
// All are must_use because the handle is the only way to access the data
pub trait Store<T> {
    #[must_use]
    fn insert(&mut self, value: T) -> handle::Handle<T>;
    #[must_use]
    fn get(&self, handle: &handle::Handle<T>) -> &T;
    #[must_use]
    fn get_mut(&mut self, handle: &handle::Handle<T>) -> &mut T;
}

impl<T> Index<handle::Handle<T>> for dyn Store<T> {
    type Output = T;
    fn index(&self, index: handle::Handle<T>) -> &Self::Output {
        self.get(&index)
    }
}

impl<T> IndexMut<handle::Handle<T>> for dyn Store<T> {
    fn index_mut(&mut self, index: handle::Handle<T>) -> &mut Self::Output {
        self.get_mut(&index)
    }
}

pub struct Arena<T> {
    pub data: Vec<T>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl<T> Store<T> for Arena<T> {
    fn insert(&mut self, value: T) -> handle::Handle<T> {
        let index = self.data.len();
        self.data.push(value);
        handle::Handle::new(index)
    }

    fn get(&self, handle: &handle::Handle<T>) -> &T {
        &self.data[handle.index()]
    }

    fn get_mut(&mut self, handle: &handle::Handle<T>) -> &mut T {
        &mut self.data[handle.index()]
    }
}
