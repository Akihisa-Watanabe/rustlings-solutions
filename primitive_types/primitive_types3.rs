// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!


fn main() {
    let a:[i32;100]= [1; 100]; //=Default::default(); は要素数が32以下の時のみ使用可能．
    //配列を初期化するときは全ての値を初期化するか，それぞれの要素の値を個別に指定するかのどちらか．
    //部分的な初期化は安全にはできない．
    

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
