fn main() {
    let mut hello = String::from("Hello, ");

    hello.push('w');
    hello.push_str("orld!");

    println!("{hello}");

    println!("{:?}", "a".as_bytes()); // [97]
    println!("{:?}", "😊".as_bytes()); // [240, 159, 152, 138]
}
