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

#[test]
fn runs() {
    main();
}
