# ModalCell

[![crate version](https://img.shields.io/crates/v/modalcell.svg)](https://crates.io/crates/modalcell)
[![Live Build Status](https://img.shields.io/github/actions/workflow/status/khonsulabs/modalcell/rust.yml?branch=main)](https://github.com/khonsulabs/modalcell/actions?query=workflow:Tests)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/modalcell/main/modalcell/)

ModalCell provides an approach for using Rust's borrow checker to enforce access
to a value via an associated mode.

Consider this example:

```rust
use modalcell::{ExclusiveCell, ExclusiveMode, RefMut, SharedCell, SharedMode};

// Create our shared mode.
let mut shared = SharedMode::new();
// Obtain exclusvie access to the mode.
let exclusive: ExclusiveMode<'_, _> = shared.as_exclusive();
// Create a new cell with the initial value of `1`. This cell can be
// accessed `mut`-ably using an `ExclusiveMode`.
let mut exclusive_cell: ExclusiveCell<usize, _> = exclusive.new_cell(1);
// Obtain a SharedCell, which can be converted to an `&T` using a
// `SharedMode`.
let shared_cell: SharedCell<usize, _> = exclusive_cell.as_shared();

// The Rust compiler now guarantees safe access to the data.
// shared_cell.get(&shared); // Because `exclusive` borrowed from
//                           // `shared`, this is a compilation error.

let cell_contents: &usize = shared_cell.get(&shared);
assert_eq!(*cell_contents, 1);

// To change the value, we must obtain exclusive access again.
let exclusive: ExclusiveMode<'_, _> = shared.as_exclusive();

// `&T` references prevent entering exclusive mode. This can be tested by
// uncommenting the following line:
// assert_eq!(*cell_contents, 1);

// Each RefMut tracks the lifetime of the &mut ExclusiveCell<T> as well as
// the lifetime of the `ExclusiveMode`. This ensures that no `&mut T` can be
// created without exclusive access to `shared`.
let mut cell_contents: RefMut<'_, '_, usize, _> = exclusive_cell.get_mut(exclusive);
*cell_contents = 2;

assert_eq!(*shared_cell.get(&shared), 2);

// Accessing `shared_cell` can only be done safely if `cell_contents` isn't
// used again. Uncommenting this line will produce a compiler error.
// *cell_contents = 3;
```

The approach taken by this crate gives similar properties to an
`Rc<RefCell<T>>`, except that with an `Rc<RefCell<T>>` the contents must be
accessed through `std::cell::Ref`/`std::cell::RefMut`. This crate allows direct
access to shared references without an intermediate type. With this type, the
Rust compiler enforces correctness, while `RefCell` requires runtime checks.

This crate still performs one check at runtime: is the mode being passed the
same one that was used to create the cell. Because this is a logic bug, passing
an incorrect mode will result in a panic.

This crate also provides an implementation that allows using this mode of data
access in multi-threaded code:

```rust
use std::sync::mpsc::{Receiver, SyncSender};

use modalcell::{ExclusiveCell, SharedMode, ThreadSafe};

fn main() {
    let mut shared = SharedMode::new_threadsafe();
    let cell = shared.new_cell(0);
    let shared_cell = cell.as_shared();

    let (counting_sender, counting_receiver) = std::sync::mpsc::sync_channel(1);
    let (printing_sender, printing_receiver) = std::sync::mpsc::sync_channel(1);

    // Spawn a thread that updates the value.
    std::thread::spawn(|| counting_thread(cell, counting_receiver, printing_sender));

    loop {
        // Send the mode to the thread, allowing it to gain mutable access.
        counting_sender.send(shared).unwrap();
        // Wait for mode to be returned to us.
        shared = printing_receiver.recv().unwrap();
        // Use it to gain access a reference of the value.
        let value: &usize = shared_cell.get(&shared);
        println!("New Count: {value}");
        // Stop after 10.
        if *value == 10 {
            break;
        }
    }
}

fn counting_thread(
    mut cell: ExclusiveCell<usize, ThreadSafe>,
    receiver: Receiver<SharedMode<ThreadSafe>>,
    sender: SyncSender<SharedMode<ThreadSafe>>,
) {
    while let Ok(mut shared) = receiver.recv() {
        // Enter exclusive mode. This is a borrow-checker only operation and has
        // no runtime overhead.
        let exclusive = shared.as_exclusive();
        // Gain access to the cell's contents using our exclusive marker.
        let mut contents = cell.get_mut(exclusive);
        // Update the value through DerefMut.
        *contents += 1;
        // Return the `SharedMode` back to the other thread.
        sender.send(shared).unwrap();

        // Attempting to use `exclusive` now will result in a compiler error.
        // let _error = cell.get_mut(exclusive);
        // let _error = *contents;
    }
}
```
