use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    fmt::{Debug, Display},
    rc::Rc,
};

use uid::IdU64;

pub trait GarbageCollectable {
    fn get_children(&self) -> Vec<GarbageCollectionId>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GarbageCollectionId(IdU64<Self>);

/// TODO: actually implement garbage collection
/// A type for a garbage collected object
pub struct Gc<T: GarbageCollectable + ?Sized> {
    /// The data that is being garbage collected
    data: Option<Rc<RefCell<T>>>,
    /// A unique identifier of the data.
    id: GarbageCollectionId,
}

impl<T: GarbageCollectable + Display + ?Sized> Display for Gc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.borrow())
    }
}

impl<T: GarbageCollectable + Debug + ?Sized> Debug for Gc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.borrow())
    }
}

#[derive(Debug)]
/// An error for when a garbage collected object is accessed after it has been collected.
pub struct CollectedError;

#[derive(Debug)]
/// An error type to combine [`CollectedError`] and [`BorrowError`]
pub enum GarbageCollectionBorrowError {
    CollectedError(CollectedError),
    BorrowError(BorrowError),
}

impl From<CollectedError> for GarbageCollectionBorrowError {
    fn from(e: CollectedError) -> Self {
        Self::CollectedError(e)
    }
}

impl From<BorrowError> for GarbageCollectionBorrowError {
    fn from(e: BorrowError) -> Self {
        Self::BorrowError(e)
    }
}

#[derive(Debug)]
/// An error type to combine [`CollectedError`] and [`BorrowMutError`]
pub enum GarbageCollectionBorrowMutError {
    CollectedError(CollectedError),
    BorrowMutError(BorrowMutError),
}

impl From<CollectedError> for GarbageCollectionBorrowMutError {
    fn from(e: CollectedError) -> Self {
        Self::CollectedError(e)
    }
}

impl From<BorrowMutError> for GarbageCollectionBorrowMutError {
    fn from(e: BorrowMutError) -> Self {
        Self::BorrowMutError(e)
    }
}

impl<T: GarbageCollectable> Gc<T> {
    /// Creates a new `Gc<T>`, from the provided T   
    pub fn new(t: T) -> Self {
        Self {
            data: Some(Rc::new(RefCell::new(t))),
            id: GarbageCollectionId(IdU64::new()),
        }
    }
}

impl<T: GarbageCollectable + ?Sized> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            id: self.id,
        }
    }
}

impl<T: GarbageCollectable + ?Sized> Gc<T> {
    #[allow(dead_code)]
    pub(crate) fn from_rc(t: Rc<RefCell<T>>) -> Self {
        Self {
            data: Some(t),
            id: GarbageCollectionId(IdU64::new()),
        }
    }

    /// Borrows the data. Panics if the data has been collected or if the data is mutable borrowed.
    pub fn borrow(&self) -> Ref<'_, T> {
        (*self.data.as_ref().unwrap()).try_borrow().unwrap()
    }
    /// Borrows the data mutably. Panics if the data has been collected or if the data is borrowed.
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        (*self.data.as_ref().unwrap()).try_borrow_mut().unwrap()
    }
    /// Borrows the data. Panics if the data has been collected.
    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
        (*self.data.as_ref().unwrap()).try_borrow()
    }
    /// Borrows the data mutably. Panics if the data has been collected.
    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
        (*self.data.as_ref().unwrap()).try_borrow_mut()
    }

    /// Borrows the data. Panics if the data is mutable borrowed.
    pub fn borrow_if_exists(&self) -> Result<Ref<'_, T>, CollectedError> {
        Ok((*self.data.as_ref().ok_or(CollectedError)?)
            .try_borrow()
            .unwrap())
    }
    /// Borrows the data mutably. Panics if the data is borrowed.
    pub fn borrow_mut_if_exists(&self) -> Result<RefMut<'_, T>, CollectedError> {
        Ok((*self.data.as_ref().ok_or(CollectedError)?)
            .try_borrow_mut()
            .unwrap())
    }
    /// Borrows the data.
    pub fn try_borrow_if_exists(&self) -> Result<Ref<'_, T>, GarbageCollectionBorrowError> {
        Ok((*self.data.as_ref().ok_or(CollectedError)?).try_borrow()?)
    }
    /// Borrows the data mutably.
    pub fn try_borrow_mut_if_exists(
        &self,
    ) -> Result<RefMut<'_, T>, GarbageCollectionBorrowMutError> {
        Ok((*self.data.as_ref().ok_or(CollectedError)?).try_borrow_mut()?)
    }

    /// Returns whether the object has been collected.
    pub fn is_collected(&self) -> bool {
        self.data.is_none()
    }
    /// Returns whether the object still exists (whether it has *not* been garbage collected)
    pub fn exists(&self) -> bool {
        self.data.is_some()
    }

    /// Gets the uniqiue id of this object
    pub fn get_id(&self) -> GarbageCollectionId {
        self.id
    }
}
