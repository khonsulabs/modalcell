//! A cell type using a shared tag to control mutable access.
#![warn(clippy::pedantic, missing_docs)]

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

/// A tag that allows read-only access to its associated [`TagCell`]s.
pub struct ReadTag<Tag>
where
    Tag: TagType,
{
    tag: Tag::Container<()>,
}

impl<Tag> Default for ReadTag<Tag>
where
    Tag: TagType,
{
    fn default() -> Self {
        Self { tag: Tag::new(()) }
    }
}

/// A tag that allows mutable access to its associated [`TagCell`]s.
#[derive(Clone, Copy)]
pub struct WriteTag<'a, Tag>(&'a ReadTag<Tag>)
where
    Tag: TagType;

impl<Tag> ReadTag<Tag>
where
    Tag: TagType,
{
    /// Begin a write session, allowing mutable access to associated
    /// [`TagCell`]s.
    pub fn write(&mut self) -> WriteTag<'_, Tag> {
        WriteTag(self)
    }
}

/// A wrapper type that guarantees memory safety using an associated tag.
pub struct MutableCell<T, Tag>
where
    Tag: TagType,
{
    tag: Tag::Container<()>,
    data: Tag::Container<UnsafeCell<T>>,
}

impl<T, Tag> MutableCell<T, Tag>
where
    Tag: TagType,
{
    /// Allocates a new cell containing `value`. The returned cell can only be
    /// accessed by providing `tag`.
    pub fn new(value: T, tag: &ReadTag<Tag>) -> Self {
        Self {
            tag: tag.tag.clone(),
            data: Tag::new(UnsafeCell::new(value)),
        }
    }

    /// Returns a [`RefMut`] providing mutable access to the contained value.
    ///
    /// # Panics
    ///
    /// This function panics if `tag` is not the same tag that was provided in
    /// [`TagCell::new`].
    pub fn write<'collection>(
        &mut self,
        tag: WriteTag<'collection, Tag>,
    ) -> RefMut<'_, 'collection, T, Tag> {
        assert!(Tag::ptr_eq(&tag.0.tag, &self.tag));
        RefMut {
            cell: self,
            _tag: tag,
        }
    }

    /// Returns a clone of this cell that can only be used to read the
    /// underlying data.
    pub fn as_read_only(&self) -> ReadOnlyCell<T, Tag> {
        ReadOnlyCell {
            tag: self.tag.clone(),
            data: self.data.clone(),
        }
    }
}

/// A read-only reference to a [`TagCell`].
pub struct ReadOnlyCell<T, Tag>
where
    Tag: TagType,
{
    tag: Tag::Container<()>,
    data: Tag::Container<UnsafeCell<T>>,
}

impl<T, Tag> ReadOnlyCell<T, Tag>
where
    Tag: TagType,
{
    /// Returns a read-only reference to the contained value.
    ///
    /// # Panics
    ///
    /// This function panics if `tag` is not the same tag that was provided in
    /// [`TagCell::new`].
    pub fn get<'a>(&'a self, collection: &'a ReadTag<Tag>) -> &'a T {
        assert!(Tag::ptr_eq(&collection.tag, &self.tag));
        // SAFETY: Because we have a read-only reference to `collection`, we
        // know that no mutable references to the underlying unsace cell can
        // exist.
        unsafe { &*self.data.get() }
    }
}

impl<T, Tag> Clone for ReadOnlyCell<T, Tag>
where
    Tag: TagType,
{
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            data: self.data.clone(),
        }
    }
}

/// A wrapper to an exclusive reference to a [`MutableCell`]'s contents.
pub struct RefMut<'a, 'collection, T, Tag>
where
    Tag: TagType,
{
    cell: &'a MutableCell<T, Tag>,
    _tag: WriteTag<'collection, Tag>,
}

impl<T, Tag> Deref for RefMut<'_, '_, T, Tag>
where
    Tag: TagType,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `RefMut` can only be constructed with an exclusive
        // 'collection lifetime that comes from mut access to a ReadTag. Since
        // TagCell cannot be cloned, this ensures no other references can exist
        // to the underlying value.
        unsafe { &*self.cell.data.get() }
    }
}

impl<T, Tag> DerefMut for RefMut<'_, '_, T, Tag>
where
    Tag: TagType,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `RefMut` can only be constructed with an exclusive
        // 'collection lifetime that comes from mut access to a ReadTag. Since
        // TagCell cannot be cloned, this ensures no other references can exist
        // to the underlying value.
        unsafe { &mut *self.cell.data.get() }
    }
}

/// A trait controlling how [`ReadTag`] and all associated types store
/// reference-counted values.
pub trait TagType {
    /// The container type used by this tag type.
    type Container<T>: Clone + Deref<Target = T>;

    /// Returns a new container wrapping `value`.
    fn new<T>(value: T) -> Self::Container<T>;
    /// Returns true if `a` and `b` represent the same underlying memory
    /// allocation.
    fn ptr_eq<T>(a: &Self::Container<T>, b: &Self::Container<T>) -> bool;
}

/// A [`TagType`] that uses [`Arc`] for thread-safety.
pub struct ThreadSafe;

impl TagType for ThreadSafe {
    type Container<T> = Arc<T>;

    fn new<T>(value: T) -> Self::Container<T> {
        Arc::new(value)
    }

    fn ptr_eq<T>(a: &Self::Container<T>, b: &Self::Container<T>) -> bool {
        Arc::ptr_eq(a, b)
    }
}

/// A [`TagType`] that uses [`Rc`], which is optimized for single-threaded use
/// cases.
pub struct SingleThreaded;

impl TagType for SingleThreaded {
    type Container<T> = Rc<T>;

    fn new<T>(value: T) -> Self::Container<T> {
        Rc::new(value)
    }

    fn ptr_eq<T>(a: &Self::Container<T>, b: &Self::Container<T>) -> bool {
        Rc::ptr_eq(a, b)
    }
}

/// A tag that utilizes [`Arc`] for thread safety.
pub type ThreadsafeTag = ReadTag<ThreadSafe>;
/// A tag that uses [`Rc`], which is optimized for single-threaded use cases.
pub type SingleThreadedTag = ReadTag<SingleThreaded>;

#[test]
fn test_threadsafe() {
    let mut collection = ThreadsafeTag::default();
    let mut a = MutableCell::new(1, &collection);
    let b = a.as_read_only();

    let writer = collection.write();
    let mut a_value = a.write(writer);
    *a_value = 2;

    assert_eq!(*b.get(&collection), 2);

    // *a_value = 3;
}

#[test]
fn test_single_threaded() {
    let mut collection = SingleThreadedTag::default();
    let mut a = MutableCell::new(1, &collection);
    let b = a.as_read_only();

    let writer = collection.write();
    let mut a_value = a.write(writer);
    *a_value = 2;

    assert_eq!(*b.get(&collection), 2);

    // *a_value = 3;
}
