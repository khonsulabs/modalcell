use modalcell::{ExclusiveCell, ExclusiveMode, RefMut, SharedCell, SharedMode};

fn main() {
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
    let mut cell_contents: RefMut<'_, usize, _> = exclusive_cell.get_mut(exclusive);
    *cell_contents = 2;

    assert_eq!(*shared_cell.get(&shared), 2);

    // Accessing `shared_cell` can only be done safely if `cell_contents` isn't
    // used again. Uncommenting this line will produce a compiler error.
    // *cell_contents = 3;
}

#[test]
fn runs() {
    main();
}
