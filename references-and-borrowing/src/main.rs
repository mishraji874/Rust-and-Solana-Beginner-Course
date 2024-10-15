// Pass the value
fn length_of_string(value: String) -> usize {
    value.len()
}
fn main() {
    let s1 = String::from("Hey there!");
    let len = length_of_string(s1);
    println!("The length is {len}.");

    let mut s1 = String::from("Hey there!");
    let s2 = s1.clone();
    s1.push_str(" How are you?");
    let len = length_of_string(s1);
    println!("The length of {s2:?} is {len}.");
}

// Returning Ownership
// fn length_of_string(value: String) -> (String, usize) {
//     let len = value.len();
//     (value, len)
// }
// fn main() {
//     let s1 = String::from("Hey there!");
//     let (s1, len) = length_of_string(s1);
//     println!("The length of {s1:?} is {len}.");
// }

// Pass by reference
// fn length_of_string_reference(value: &String) -> usize {
//     value.len()
// }
// fn main() {
//     let s1 = String::from("Hey there!");
//     // v - Take a reference to `s1`.
//     let len = length_of_string(&s1);
//     println!("The length of {s1:?} is {len}.");
// }

// fn append_world(value: &mut String) {
//     value.push_str(", World!");
// }
// fn main() {
//     // vvv - Allow s1 to be mutated.
//     let mut s1 = String::from("Hello");
//     //           vvvv - Take a mutable reference.
//     append_world(&mut s1);
//     println!("The value is now {s1:?}.");
// }