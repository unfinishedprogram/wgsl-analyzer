use std::ops::{Index, IndexMut};

pub mod handle;

// There is no ability to remove, this means that a handle is always valid.
// All are must_use because the handle is the only way to access the data
pub trait Store<T> {
    #[must_use]
    fn insert(&mut self, value: T) -> handle::Handle<T>;
    #[must_use]
    fn get(&self, handle: handle::Handle<T>) -> &T;
    #[must_use]
    fn get_mut(&mut self, handle: handle::Handle<T>) -> &mut T;
}

impl<T> Index<handle::Handle<T>> for dyn Store<T> {
    type Output = T;
    fn index(&self, index: handle::Handle<T>) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<handle::Handle<T>> for dyn Store<T> {
    fn index_mut(&mut self, index: handle::Handle<T>) -> &mut Self::Output {
        self.get_mut(index)
    }
}
