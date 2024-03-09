use std::marker::PhantomData;

// TODO:
// Add Spanned version of handle, which is transparent,
// but allows tracking the position of a reference in the source code
// Otherwise, only the declaration's position will be saved, making diagnostic messages awkward
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Handle<T> {
    index: usize,
    _type: PhantomData<T>,
}

impl<T> Copy for Handle<T> where T: Clone {}

impl<T> Handle<T> {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            _type: Default::default(),
        }
    }

    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }
}
