#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic, missing_docs)]
#![no_std]

extern crate alloc;

use alloc::rc::Rc;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

pub mod safety;
#[cfg(feature = "std")]
pub mod threadsafe;

#[cfg(feature = "std")]
use crate::threadsafe::ThreadSafe;

/// A tag that allows read-only access to its associated [`ExclusiveCell`]s.
pub struct SharedMode<Behavior = SingleThreaded>
where
    Behavior: crate::Behavior,
{
    tag: Behavior::Container<()>,
}

impl<Behavior> Default for SharedMode<Behavior>
where
    Behavior: crate::Behavior,
{
    fn default() -> Self {
        Self {
            tag: Behavior::new(()),
        }
    }
}

#[cfg(feature = "std")]
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

impl<Behavior> ExclusiveMode<'_, Behavior>
where
    Behavior: crate::Behavior,
{
    /// Returns a new `ExclusiveCell` associated with this mode.
    pub fn new_cell<T>(&self, value: T) -> ExclusiveCell<T, Behavior> {
        self.0.new_cell(value)
    }
}

/// A tag that allows exclusive access to the underlying [`SharedMode`]'s
/// associated [`ExclusiveCell`]s.
pub struct ExclusiveMode<'a, Behavior = SingleThreaded>(&'a SharedMode<Behavior>)
where
    Behavior: crate::Behavior;

impl<Behavior> Clone for ExclusiveMode<'_, Behavior>
where
    Behavior: crate::Behavior,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Behavior> Copy for ExclusiveMode<'_, Behavior> where Behavior: crate::Behavior {}

impl<Behavior> SharedMode<Behavior>
where
    Behavior: crate::Behavior,
{
    /// Begin a write session, allowing mutable access to associated
    /// [`ExclusiveCell`]s.
    pub fn as_exclusive(&mut self) -> ExclusiveMode<'_, Behavior> {
        ExclusiveMode(self)
    }

    /// Returns a new [`ExclusiveCell`] associated with the underlying
    /// [`SharedMode`].
    pub fn new_cell<T>(&self, value: T) -> ExclusiveCell<T, Behavior> {
        ExclusiveCell::new(value, self)
    }
}

/// A wrapper type that guarantees memory safety by only allowing access when
/// its associated [`ExclusiveMode`] tag is provided.
pub struct ExclusiveCell<T, Behavior = SingleThreaded>
where
    Behavior: crate::Behavior,
{
    tag: Behavior::Container<()>,
    data: Behavior::Container<UnsafeCell<T>>,
}

impl<T, Behavior> ExclusiveCell<T, Behavior>
where
    Behavior: crate::Behavior,
{
    fn new(value: T, mode: &SharedMode<Behavior>) -> Self {
        Self {
            tag: mode.tag.clone(),
            data: Behavior::new(UnsafeCell::new(value)),
        }
    }

    /// Returns a [`RefMut`] providing exclusive access to the contained value.
    ///
    /// # Panics
    ///
    /// This function panics if `mode` is not the same mode that was used to
    /// create the cell.
    pub fn get_mut<'mode>(
        &'mode mut self,
        mode: ExclusiveMode<'mode, Behavior>,
    ) -> RefMut<'_, T, Behavior> {
        assert!(Behavior::ptr_eq(&mode.0.tag, &self.tag));
        RefMut {
            cell: self,
            _mode: mode,
        }
    }

    /// Returns a [`RefMut`] providing exclusive access to the contained value.
    ///
    /// # Panics
    ///
    /// This function panics when `debug_assertions` are enabled and `mode` is
    /// not the same mode that was used to create the cell.
    ///
    /// # Safety
    ///
    /// When compiled with `debug_assertions` disabled, undefined behavior may
    /// result if `mode` is not the same mode that was used to create the cell.
    pub unsafe fn get_mut_unchecked<'mode>(
        &'mode mut self,
        mode: ExclusiveMode<'mode, Behavior>,
    ) -> RefMut<'_, T, Behavior> {
        debug_assert!(Behavior::ptr_eq(&mode.0.tag, &self.tag));
        RefMut {
            cell: self,
            _mode: mode,
        }
    }

    /// Returns a clone of this cell that can only be used to read the
    /// underlying data.
    pub fn as_shared(&self) -> SharedCell<T, Behavior> {
        SharedCell {
            tag: self.tag.clone(),
            data: self.data.clone(),
        }
    }
}

unsafe impl<T, Behavior> Send for ExclusiveCell<T, Behavior>
where
    Behavior: crate::Behavior,
    Behavior::Container<T>: Send,
{
}
unsafe impl<T, Behavior> Sync for ExclusiveCell<T, Behavior>
where
    Behavior: crate::Behavior,
    Behavior::Container<T>: Sync,
{
}

/// A read-only reference to a [`ExclusiveCell`].
pub struct SharedCell<T, Behavior = SingleThreaded>
where
    Behavior: crate::Behavior,
{
    tag: Behavior::Container<()>,
    data: Behavior::Container<UnsafeCell<T>>,
}

impl<T, Behavior> SharedCell<T, Behavior>
where
    Behavior: crate::Behavior,
{
    /// Returns a read-only reference to the contained value.
    ///
    /// # Panics
    ///
    /// This function panics if `mode` is not the same mode that was used to
    /// create the cell.
    pub fn get<'a>(&'a self, mode: &'a SharedMode<Behavior>) -> &'a T {
        assert!(Behavior::ptr_eq(&mode.tag, &self.tag));
        // SAFETY: Because we have a read-only reference to `mode`, we
        // know that no mutable references to the underlying unsace cell can
        // exist.
        unsafe { &*self.data.get() }
    }

    /// Returns a read-only reference to the contained value.
    ///
    /// # Panics
    ///
    /// This function panics when `debug_assertions` are enabled and `mode` is
    /// not the same mode that was used to create the cell.
    ///
    /// # Safety
    ///
    /// When compiled with `debug_assertions` disabled, undefined behavior may
    /// result if `mode` is not the same mode that was used to create the cell.
    pub unsafe fn get_unchecked<'a>(&'a self, mode: &'a SharedMode<Behavior>) -> &'a T {
        debug_assert!(Behavior::ptr_eq(&mode.tag, &self.tag));
        // SAFETY: Because we have a read-only reference to `mode`, we
        // know that no mutable references to the underlying unsace cell can
        // exist.
        unsafe { &*self.data.get() }
    }
}

impl<T, Behavior> Clone for SharedCell<T, Behavior>
where
    Behavior: crate::Behavior,
{
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            data: self.data.clone(),
        }
    }
}
unsafe impl<T, Behavior> Send for SharedCell<T, Behavior>
where
    Behavior: crate::Behavior,
    Behavior::Container<T>: Send,
{
}
unsafe impl<T, Behavior> Sync for SharedCell<T, Behavior>
where
    Behavior: crate::Behavior,
    Behavior::Container<T>: Sync,
{
}

/// A wrapper to an exclusive reference to an [`ExclusiveCell`]'s contents.
pub struct RefMut<'a, T, Behavior = SingleThreaded>
where
    Behavior: crate::Behavior,
{
    cell: &'a ExclusiveCell<T, Behavior>,
    _mode: ExclusiveMode<'a, Behavior>,
}

impl<T, Behavior> Deref for RefMut<'_, T, Behavior>
where
    Behavior: crate::Behavior,
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

impl<T, Behavior> DerefMut for RefMut<'_, T, Behavior>
where
    Behavior: crate::Behavior,
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
///
/// # Safety
///
/// When `ptr_eq` is provided two `Container`s from separate calls to `new()`,
/// it always returns false. When `ptr_eq` is provided the same `Container` in
/// both `a` and `b`, it always returns true.
///
/// Breaking this guarantee is undefined behavior.
pub unsafe trait Behavior {
    /// The container type used by this tag type.
    type Container<T>: Clone + Deref<Target = T>;

    /// Returns a new container wrapping `value`.
    fn new<T>(value: T) -> Self::Container<T>;
    /// Returns true if `a` and `b` represent the same underlying memory
    /// allocation.
    fn ptr_eq<T>(a: &Self::Container<T>, b: &Self::Container<T>) -> bool;
}

/// A [`Behavior`] that uses [`Rc`], which is optimized for single-threaded use
/// cases.
pub enum SingleThreaded {}

unsafe impl Behavior for SingleThreaded {
    type Container<T> = Rc<T>;

    fn new<T>(value: T) -> Self::Container<T> {
        Rc::new(value)
    }

    fn ptr_eq<T>(a: &Self::Container<T>, b: &Self::Container<T>) -> bool {
        Rc::ptr_eq(a, b)
    }
}

#[test]
#[cfg(feature = "std")]
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
