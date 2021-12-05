// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!


fn main() {
    let cat = ("Furry McFurson", 3.5);//&str型とf64型にはCopyトレイトが実装されているためこのタプルはコピーが可能
    let name = cat.0; //nameはcat.0の文字列のコピーに束縛されて所有権を保持．

    println!("{} is {} years old.", name, cat.1);
}
