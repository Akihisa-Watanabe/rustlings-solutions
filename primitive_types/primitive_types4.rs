// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];//&[i32]型.最初の要素への参照と長さを保持する．

    assert_eq!([2, 3, 4], nice_slice)
}
