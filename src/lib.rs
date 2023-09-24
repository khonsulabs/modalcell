#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic, missing_docs)]

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

/// A tag that allows read-only access to its associated [`ExclusiveCell`]s.
pub struct SharedMode<Mode>
where
    Mode: crate::Mode,
{
    tag: Mode::Container<()>,
}

impl<Mode> Default for SharedMode<Mode>
where
    Mode: crate::Mode,
{
    fn default() -> Self {
        Self { tag: Mode::new(()) }
    }
}

impl SharedMode<ThreadSafe> {
    /// Returns a new instance that can be used in multi-threaded code.
    #[must_use]
    pub fn new_threadsafe() -> Self {
        Self::default()
    }
}

impl SharedMode<SingleThreaded> {
    /// Returns a new instance that can only be used in single-threaded code.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Mode> ExclusiveMode<'_, Mode>
where
    Mode: crate::Mode,
{
    /// Returns a new `ExclusiveCell` associated with this mode.
    pub fn new_cell<T>(&self, value: T) -> ExclusiveCell<T, Mode> {
        self.0.new_cell(value)
    }
}

/// A tag that allows exclusive access to the underlying [`SharedMode`]'s
/// associated [`ExclusiveCell`]s.
pub struct ExclusiveMode<'a, Mode>(&'a SharedMode<Mode>)
where
    Mode: crate::Mode;

impl<Mode> Clone for ExclusiveMode<'_, Mode>
where
    Mode: crate::Mode,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Mode> Copy for ExclusiveMode<'_, Mode> where Mode: crate::Mode {}

impl<Mode> SharedMode<Mode>
where
    Mode: crate::Mode,
{
    /// Begin a write session, allowing mutable access to associated
    /// [`ExclusiveCell`]s.
    pub fn as_exclusive(&mut self) -> ExclusiveMode<'_, Mode> {
        ExclusiveMode(self)
    }

    /// Returns a new [`ExclusiveCell`] associated with the underlying
    /// [`SharedMode`].
    pub fn new_cell<T>(&self, value: T) -> ExclusiveCell<T, Mode> {
        ExclusiveCell::new(value, self)
    }
}

/// A wrapper type that guarantees memory safety by only allowing access when
/// its associated [`ExclusiveMode`] tag is provided.
pub struct ExclusiveCell<T, Mode>
where
    Mode: crate::Mode,
{
    tag: Mode::Container<()>,
    data: Mode::Container<UnsafeCell<T>>,
}

impl<T, Mode> ExclusiveCell<T, Mode>
where
    Mode: crate::Mode,
{
    fn new(value: T, mode: &SharedMode<Mode>) -> Self {
        Self {
            tag: mode.tag.clone(),
            data: Mode::new(UnsafeCell::new(value)),
        }
    }

    /// Returns a [`RefMut`] providing exclusive access to the contained value.
    ///
    /// # Panics
    ///
    /// This function panics if `mode` is not the same mode that was used to
    /// create the cell.
    pub fn get_mut<'mode>(
        &mut self,
        mode: ExclusiveMode<'mode, Mode>,
    ) -> RefMut<'_, 'mode, T, Mode> {
        assert!(Mode::ptr_eq(&mode.0.tag, &self.tag));
        RefMut {
            cell: self,
            _mode: mode,
        }
    }

    /// Returns a clone of this cell that can only be used to read the
    /// underlying data.
    pub fn as_shared(&self) -> SharedCell<T, Mode> {
        SharedCell {
            tag: self.tag.clone(),
            data: self.data.clone(),
        }
    }
}

unsafe impl<T, Mode> Send for ExclusiveCell<T, Mode>
where
    Mode: crate::Mode,
    Mode::Container<T>: Send,
{
}
unsafe impl<T, Mode> Sync for ExclusiveCell<T, Mode>
where
    Mode: crate::Mode,
    Mode::Container<T>: Sync,
{
}

/// A read-only reference to a [`ExclusiveCell`].
pub struct SharedCell<T, Mode>
where
    Mode: crate::Mode,
{
    tag: Mode::Container<()>,
    data: Mode::Container<UnsafeCell<T>>,
}

impl<T, Mode> SharedCell<T, Mode>
where
    Mode: crate::Mode,
{
    /// Returns a read-only reference to the contained value.
    ///
    /// # Panics
    ///
    /// This function panics if `mode` is not the same mode that was used to
    /// create the cell.
    pub fn get<'a>(&'a self, mode: &'a SharedMode<Mode>) -> &'a T {
        assert!(Mode::ptr_eq(&mode.tag, &self.tag));
        // SAFETY: Because we have a read-only reference to `mode`, we
        // know that no mutable references to the underlying unsace cell can
        // exist.
        unsafe { &*self.data.get() }
    }
}

impl<T, Mode> Clone for SharedCell<T, Mode>
where
    Mode: crate::Mode,
{
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            data: self.data.clone(),
        }
    }
}
unsafe impl<T, Mode> Send for SharedCell<T, Mode>
where
    Mode: crate::Mode,
    Mode::Container<T>: Send,
{
}
unsafe impl<T, Mode> Sync for SharedCell<T, Mode>
where
    Mode: crate::Mode,
    Mode::Container<T>: Sync,
{
}

/// A wrapper to an exclusive reference to an [`ExclusiveCell`]'s contents.
pub struct RefMut<'a, 'mode, T, Mode>
where
    Mode: crate::Mode,
{
    cell: &'a ExclusiveCell<T, Mode>,
    _mode: ExclusiveMode<'mode, Mode>,
}

impl<T, Mode> Deref for RefMut<'_, '_, T, Mode>
where
    Mode: crate::Mode,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `RefMut` can only be constructed with an exclusive
        // 'mode lifetime that comes from mut access to a SharedTag. Since
        // `ExclusiveCell` cannot be cloned, this ensures no other references
        // can exist to the underlying value.
        unsafe { &*self.cell.data.get() }
    }
}

impl<T, Mode> DerefMut for RefMut<'_, '_, T, Mode>
where
    Mode: crate::Mode,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `RefMut` can only be constructed with an exclusive
        // 'mode lifetime that comes from mut access to a SharedTag. Since
        // `ExclusiveCell` cannot be cloned, this ensures no other references
        // can exist to the underlying value.
        unsafe { &mut *self.cell.data.get() }
    }
}

/// A trait controlling how [`SharedMode`] and all associated types store
/// reference-counted values.
pub trait Mode {
    /// The container type used by this tag type.
    type Container<T>: Clone + Deref<Target = T>;

    /// Returns a new container wrapping `value`.
    fn new<T>(value: T) -> Self::Container<T>;
    /// Returns true if `a` and `b` represent the same underlying memory
    /// allocation.
    fn ptr_eq<T>(a: &Self::Container<T>, b: &Self::Container<T>) -> bool;
}

/// A [`Mode`] that uses [`Arc`] for thread-safety.
pub struct ThreadSafe;

impl Mode for ThreadSafe {
    type Container<T> = Arc<T>;

    fn new<T>(value: T) -> Self::Container<T> {
        Arc::new(value)
    }

    fn ptr_eq<T>(a: &Self::Container<T>, b: &Self::Container<T>) -> bool {
        Arc::ptr_eq(a, b)
    }
}

/// A [`Mode`] that uses [`Rc`], which is optimized for single-threaded use
/// cases.
pub struct SingleThreaded;

impl Mode for SingleThreaded {
    type Container<T> = Rc<T>;

    fn new<T>(value: T) -> Self::Container<T> {
        Rc::new(value)
    }

    fn ptr_eq<T>(a: &Self::Container<T>, b: &Self::Container<T>) -> bool {
        Rc::ptr_eq(a, b)
    }
}

#[test]
fn test_threadsafe() {
    let mut mode = SharedMode::new_threadsafe();
    let mut a = ExclusiveCell::new(1, &mode);
    let b = a.as_shared();

    let writer = mode.as_exclusive();
    let mut a_value = a.get_mut(writer);
    *a_value = 2;

    assert_eq!(*b.get(&mode), 2);

    // *a_value = 3;
}

#[test]
fn test_single_threaded() {
    let mut mode = SharedMode::new();
    let mut a = ExclusiveCell::new(1, &mode);
    let b = a.as_shared();

    let writer = mode.as_exclusive();
    let mut a_value = a.get_mut(writer);
    *a_value = 2;

    assert_eq!(*b.get(&mode), 2);

    // *a_value = 3;
}

/// Documentation and doctests attempting to prove the safety of this crate.
pub mod safety {
    //! This code uses `UnsafeCell` internally, but only has a small amount of
    //! unsafe code. The goal of this crate is to ensure that at compile time,
    //! all Rust borrowing invariants are guaranteed to be upheld. This means
    //! that it should be impossible to gain access to an `&T` while an `&mut T`
    //! is in existence.
    //!
    //! To prevent this, several design decisions were made about the types in
    //! this crate:
    //!
    //! - `SharedMode` does not implement `Clone`. This ensures the compiler's
    //!   borrow checking rules on a `SharedMode` value can be extended to cells
    //!   that are associated with this `SharedMode`.
    //! - `SharedMode::as_exclusive()` exclusively borrows the `SharedMode` and
    //!   returns an `ExclusiveMode<'_>`. This lifetime causes the compiler to
    //!   reject any attempt at using the underlying `&SharedMode` until the
    //!   lifetime is no longer in use.
    //! - `ExclusiveCell::get_mut` requires an `ExclusiveMode` to return access
    //!   to its value. The returned `RefMut` encompasses both the cell's
    //!   lifetime and the mode's lifetime, causing compilation errors to arise
    //!   from invalid usage.
    //! - `SharedCell::get` requires an `&SharedMode`. `ExclusiveMode` does not
    //!   provide a way to access the `SharedMode` it is wrapping, ensuring that
    //!   the compiler enforces that only either `SharedCell::get` or
    //!   `ExclusiveCell::get_mut` can be accessed at any given line of code.
    //! - `SharedCell` implements `Clone`, but `ExclusiveCell` does not. This
    //!   upholds Rust's guarantees that only one exclusive reference should be
    //!   able to exist, while it is valid to have as many shared references as
    //!   desired.
    //!
    //! ## Unable to use `SharedMode` while `ExclusiveMode` exists
    //!
    //! Incorrect usage:
    //!
    //! ```rust,compile_fail
    //! use modalcell::SharedMode;
    //! let mut shared = SharedMode::new();
    //! let mut exclusive_cell = shared.new_cell(1);
    //! let shared_cell = exclusive_cell.as_shared();
    //! let exclusive = shared.as_exclusive();
    //! *exclusive_cell.get_mut(exclusive) = 2;
    //! assert_eq!(*shared_cell.get(&shared), 2);
    //!
    //! *exclusive_cell.get_mut(exclusive) = 3;
    //! ```
    //!
    //! Correct usage:
    //!
    //! ```rust
    //! use modalcell::SharedMode;
    //!
    //! let mut shared = SharedMode::new();
    //! let mut exclusive_cell = shared.new_cell(1);
    //! let shared_cell = exclusive_cell.as_shared();
    //! let exclusive = shared.as_exclusive();
    //! *exclusive_cell.get_mut(exclusive) = 2;
    //! assert_eq!(*shared_cell.get(&shared), 2);
    //!
    //! // By creating a new exclusive session, the compiler ends the
    //! // previous `exclusive` borrow at its last usage.
    //! let exclusive = shared.as_exclusive();
    //! *exclusive_cell.get_mut(exclusive) = 3;
    //! ```
    //!
    //! ## Unable to get an `ExclusiveMode` while a `SharedMode` is borrowed.
    //!
    //! Incorrect usage:
    //!
    //! ```rust,compile_fail
    //! use modalcell::SharedMode;
    //! let mut shared = SharedMode::new();
    //! let mut exclusive_cell = shared.new_cell(1);
    //! let shared_cell = exclusive_cell.as_shared();
    //! let cell_contents = shared_cell.get(&shared);
    //! let exclusive = shared.as_exclusive();
    //! assert_eq!(*cell_contents, 1);
    //!
    //! *exclusive_cell.get_mut(exclusive) = 3;
    //! ```
    //!
    //! Correct usage:
    //!
    //! ```rust
    //! use modalcell::SharedMode;
    //! let mut shared = SharedMode::new();
    //! let mut exclusive_cell = shared.new_cell(1);
    //! let shared_cell = exclusive_cell.as_shared();
    //! let cell_contents = shared_cell.get(&shared);
    //! assert_eq!(*cell_contents, 1);
    //!
    //! let exclusive = shared.as_exclusive();
    //! *exclusive_cell.get_mut(exclusive) = 3;
    //! ```
}
