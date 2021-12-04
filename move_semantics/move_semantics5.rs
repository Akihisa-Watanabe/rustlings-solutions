// move_semantics5.rs
// Make me compile only be reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)



fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; //参照の所有者xの値を設定.xの値は200になる.  
    //2つ以上のポインタが同じデータに同時にアクセスすることはできない．

    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
