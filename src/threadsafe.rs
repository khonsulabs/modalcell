//! Types for using this crate with multi-threaded code.
extern crate std;

use std::sync::Arc;

use crate::Behavior;

/// A type alias for [`SharedMode`] using the [`ThreadSafe`] behavior.
pub type SharedMode = super::SharedMode<ThreadSafe>;
/// A type alias for [`ExclusiveMode`] using the [`ThreadSafe`] behavior.
pub type ExclusiveMode<'a> = super::ExclusiveMode<'a, ThreadSafe>;
/// A type alias for [`SharedCell`] using the [`ThreadSafe`] behavior.
pub type SharedCell<T> = super::SharedCell<T, ThreadSafe>;
/// A type alias for [`ExclusiveCell`] using the [`ThreadSafe`] behavior.
pub type ExclusiveCell<T> = super::ExclusiveCell<T, ThreadSafe>;
/// A type alias for [`RefMut`] using the [`ThreadSafe`] behavior.
pub type RefMut<'a, T> = super::RefMut<'a, T, ThreadSafe>;

/// A [`Behavior`] that uses [`Arc`] for thread-safety.
pub enum ThreadSafe {}

unsafe impl Behavior for ThreadSafe {
    type Container<T> = Arc<T>;

    fn new<T>(value: T) -> Self::Container<T> {
        Arc::new(value)
    }

    fn ptr_eq<T>(a: &Self::Container<T>, b: &Self::Container<T>) -> bool {
        Arc::ptr_eq(a, b)
    }
}
