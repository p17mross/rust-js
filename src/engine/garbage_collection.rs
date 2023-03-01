//! Functionality related to the [Gc] type, which implements garbage collection.
//! Currently the implementation is simple and can lead to memory leaks.

use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    fmt::{Debug, Display},
    rc::Rc,
};

use uid::IdU64;

/// A trait for items which can be garbage collected.
/// Each type implementing this trait must implement the [`get_children`][GarbageCollectable::get_children] method, which returns the [`GarbageCollectionId`]s of its children.
/// This will allow the garbage collector to detect loops.
/// If a type contains a [Gc] property but does not return its id from this method, then that property may be collected, which may lead to panics.
pub trait GarbageCollectable {
    /// Poll what references a [`GarbageCollectable`] type holds.
    /// The implementation of this method should return the [`GarbageCollectionId`] of any [`Gc`] properties of the type, including grandchildren
    ///
    /// ```
    /// # use js::{Gc, GarbageCollectable, GarbageCollectionId};
    /// /// The data for a GcTreeNode
    /// struct GcTreeData {
    ///     number: i32,
    ///     some_other_data: Gc<GcTreeNode>,
    /// }
    ///
    /// /// A type which implements the GarbageCollectable trait
    /// struct GcTreeNode {
    ///     left: Option<Gc<GcTreeNode>>,
    ///     right: Option<Gc<GcTreeNode>>,
    ///     data: GcTreeData,
    /// }
    ///
    /// impl GarbageCollectable for GcTreeNode {
    ///     fn get_children(&self) -> Vec<GarbageCollectionId> {
    ///         let mut ids = Vec::new();
    ///
    ///
    ///         // Don't recursively call get_children, just return the ids of any Gc properties
    ///         if let Some(n) = &self.left {
    ///             ids.push(n.get_id());
    ///         }
    ///
    ///         if let Some(n) = &self.right {
    ///             ids.push(n.get_id());
    ///         }
    ///     
    ///         // self.data.number can be ignored as it is not a Gc type
    ///         // Grandchild properties must be included as well
    ///         ids.push(self.data.some_other_data.get_id());
    ///
    ///         ids
    ///     }
    /// }
    /// ```
    ///
    fn get_children(&self) -> Vec<GarbageCollectionId>;
}

/// A unique identifier for a [`Gc`] object, used to construct a reference graph for garbage collection.
/// The [`GarbageCollectionId`] of a [`Gc`] object can be gotten using the [`get_id`][Gc::get_id] method.
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
    /// The data was collected
    CollectedError(CollectedError),
    /// The data was already mutably borrowed
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
    /// The data was collected
    CollectedError(CollectedError),
    /// The data was already borrowed
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
    /// Borrows the data.
    ///
    /// ### Panics
    /// * If the data has been collected
    /// * If the data is mutably borrowed.
    #[must_use]
    pub fn borrow(&self) -> Ref<'_, T> {
        let data = self.data.as_ref().unwrap();
        data.try_borrow().unwrap()
    }

    /// Borrows the data mutably.
    ///
    /// ### Panics
    /// * If the data has been collected
    /// * If the data is borrowed.
    #[must_use]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        let data = self.data.as_ref().unwrap();
        data.try_borrow_mut().unwrap()
    }

    /// Borrows the data.
    ///
    /// ### Panics
    /// * If the data has been collected.
    ///
    /// ### Errors
    /// * If the data is mutably borrowed
    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
        let data = self.data.as_ref().unwrap();
        data.try_borrow()
    }

    /// Borrows the data mutably.
    ///
    /// ### Panics
    /// * If the data has been collected.
    ///
    /// ### Errors
    /// * If the data is borrowed
    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
        (*self.data.as_ref().unwrap()).try_borrow_mut()
    }

    /// Borrows the data.
    ///
    /// ### Panics
    /// * If the data is mutably borrowed.
    ///
    /// ### Errors
    /// * If the data has been collected
    pub fn borrow_if_exists(&self) -> Result<Ref<'_, T>, CollectedError> {
        let data = self.data.as_ref().ok_or(CollectedError)?;
        Ok(data.try_borrow().unwrap())
    }

    /// Borrows the data mutably.
    ///
    /// ### Panics
    /// * If the data is borrowed.
    ///
    /// ### Errors
    /// * If the data has been collected
    pub fn borrow_mut_if_exists(&self) -> Result<RefMut<'_, T>, CollectedError> {
        let data = self.data.as_ref().ok_or(CollectedError)?;
        Ok(data.try_borrow_mut().unwrap())
    }

    /// Borrows the data.
    ///
    /// ### Errors
    /// * If the data is mutably borrowed
    /// * If the data has been collected
    pub fn try_borrow_if_exists(&self) -> Result<Ref<'_, T>, GarbageCollectionBorrowError> {
        let data = self.data.as_ref().ok_or(CollectedError)?;
        Ok(data.try_borrow()?)
    }

    /// Borrows the data mutably.
    ///
    /// ### Errors
    /// * If the data is borrowed
    /// * If the data has been collected
    pub fn try_borrow_mut_if_exists(
        &self,
    ) -> Result<RefMut<'_, T>, GarbageCollectionBorrowMutError> {
        let data = self.data.as_ref().ok_or(CollectedError)?;
        Ok(data.try_borrow_mut()?)
    }

    /// Returns whether the object has been collected.
    #[must_use]
    pub fn is_collected(&self) -> bool {
        self.data.is_none()
    }
    /// Returns whether the object still exists (whether it has *not* been garbage collected)
    #[must_use]
    pub fn exists(&self) -> bool {
        self.data.is_some()
    }

    /// Gets the unique id of this object
    #[must_use]
    pub fn get_id(&self) -> GarbageCollectionId {
        self.id
    }
}
