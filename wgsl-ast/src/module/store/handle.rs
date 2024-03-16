use std::marker::PhantomData;

// TODO:
// Add Spanned version of handle, which is transparent,
// but allows tracking the position of a reference in the source code
// Otherwise, only the declaration's position will be saved, making diagnostic messages awkward
#[derive(PartialEq, Eq, Debug)]
pub struct Handle<T> {
    index: usize,
    _type: PhantomData<T>,
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            _type: Default::default(),
        }
    }
}

impl<T: Clone> Copy for Handle<T> {}

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
