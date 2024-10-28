fn main() {
    let x: String = String::from("hello");
    let y = x;
    // Error: println!("{x}");
    println!("{y}");
}
