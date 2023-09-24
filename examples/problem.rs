use modalcell::{ExclusiveCell, ExclusiveMode, Mode, RefMut, SharedCell, SharedMode};

struct MyMode;

impl Mode for MyMode {
    type Container<T> = std::rc::Rc<T>;

    fn new<T>(value: T) -> Self::Container<T> {
        std::rc::Rc::new(value)
    }

    fn ptr_eq<T>(_: &Self::Container<T>, _: &Self::Container<T>) -> bool {
        true
    }
}

fn main() {
    let mut shared = SharedMode::<MyMode>::default();

    let exclusive: ExclusiveMode<'_, _> = shared.as_exclusive();
    let mut exclusive_cell: ExclusiveCell<usize, _> = exclusive.new_cell(1);
    let shared_cell: SharedCell<usize, _> = exclusive_cell.as_shared();

    let second_shared = SharedMode::default();
    let cell_contents: &usize = shared_cell.get(&second_shared);
    assert_eq!(*cell_contents, 1);

    let exclusive: ExclusiveMode<'_, _> = shared.as_exclusive();

    assert_eq!(*cell_contents, 1);
    println!("UB has been triggered");

    let mut cell_contents: RefMut<'_, '_, usize, _> = exclusive_cell.get_mut(exclusive);
    *cell_contents = 2;

    assert_eq!(*shared_cell.get(&shared), 2);
}

#[test]
fn runs() {
    main();
}
