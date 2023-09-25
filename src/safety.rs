//! This module documents the reasoning behind this crate's safety guarantees.
//!
//! This crate uses `UnsafeCell` internally, but only has a small amount of
//! unsafe code. The goal of this crate is to ensure that at compile time, all
//! Rust borrowing invariants are guaranteed to be upheld. This means that it
//! should be impossible to gain access to an `&mut T` while a `&T` is in
//! existence.
//!
//! To prevent this, several design decisions were made about the types in this
//! crate:
//!
//! - [`SharedMode`](crate::SharedMode) does not implement `Clone`. This ensures
//!   the compiler's borrow checking rules on a `SharedMode` value can be
//!   extended to cells that are associated with this `SharedMode`.
//! - [`SharedMode::as_exclusive()`](crate::SharedMode::as_exclusive)
//!   exclusively borrows the `SharedMode` and returns an
//!   [`ExclusiveMode<'_>`](crate::ExclusiveMode). This lifetime causes the
//!   compiler to reject any attempt at using the underlying `&SharedMode` until
//!   the lifetime is no longer in use.
//! - [`ExclusiveCell::get_mut`](crate::ExclusiveCell::get_mut) requires an
//!   [`ExclusiveMode`](crate::ExclusiveMode) to return access to its value. The
//!   returned [`RefMut`](crate::RefMut) merges the cell's lifetime and the
//!   mode's lifetime, ensuring the Rust compiler will error on invalid usage.
//! - [`SharedCell::get`](crate::SharedCell::get) requires a
//!   [`&SharedMode`](crate::SharedMode).
//!   [`ExclusiveMode`](crate::ExclusiveMode) does not provide a way to access
//!   the `SharedMode` it is wrapping. These two guarantees allow the compiler
//!   to ensure that only either [`SharedCell::get`](crate::SharedCell::get) or
//!   [`ExclusiveCell::get_mut`](crate::ExclusiveCell::get_mut) can be accessed
//!   at any given line of code.
//! - [`SharedCell`](crate::SharedCell) implements `Clone`, but
//!   [`ExclusiveCell`](crate::ExclusiveCell) does not. This matches Rust's
//!   rules that only one exclusive reference should be able to exist, while it
//!   is valid to have as many shared references as desired.
//!
//! ## Examples demonstrating compile-time checking
//!
//! ### Unable to use `SharedMode` while `ExclusiveMode` exists
//!
//! #### Incorrect usage:
//!
//! `error[E0502]: cannot borrow `shared` as immutable because it is also borrowed as mutable`
//!
//! ```rust,compile_fail
//! use modalcell::SharedMode;
//! let mut shared = SharedMode::new();
//! let mut exclusive_cell = shared.new_cell(1);
//! let shared_cell = exclusive_cell.as_shared();
//! let exclusive = shared.as_exclusive();
//! //              --------------------- mutable borrow occurs here
//! *exclusive_cell.get_mut(exclusive) = 2;
//! assert_eq!(*shared_cell.get(&shared), 2);
//! //                          ^^^^^^^ immutable borrow occurs here
//!
//! *exclusive_cell.get_mut(exclusive) = 3;
//! //                      --------- mutable borrow later used here
//! ```
//!
//! #### Correct usage:
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
//! ### Unable to get an `ExclusiveMode` while a `SharedMode` is borrowed.
//!
//! #### Incorrect usage:
//!
//! `error[E0502]: cannot borrow `shared` as mutable because it is also borrowed as immutable`
//!
//! ```rust,compile_fail
//! use modalcell::SharedMode;
//! let mut shared = SharedMode::new();
//! let mut exclusive_cell = shared.new_cell(1);
//! let shared_cell = exclusive_cell.as_shared();
//! let cell_contents = shared_cell.get(&shared);
//! //                                  ------- immutable borrow occurs here
//! let exclusive = shared.as_exclusive();
//! //              ^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
//! assert_eq!(*cell_contents, 1);
//! //------------------------------ immutable borrow later used here
//!
//! *exclusive_cell.get_mut(exclusive) = 3;
//! ```
//!
//! #### Correct usage:
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
