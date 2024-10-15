use std::collections::HashSet;
fn main() {
    // Vector -> a continuous growable array type with heap-allocated contents.
    let statically_sized_array: [u8; 4] = [0, 1, 2, 3];
    let dynamically_sized_array: Vec<u8> = vec![0, 1, 2, 3];
    let dynamically_sized_array = {
        let mut a = Vec::new();
        a.push(0);
        a.push(1);
        a.push(2);
        a.push(3);
        a
    };
    let mut a = vec![1, 2];
    println!("{:?}", a.pop()); // Some(2)
    let a = vec![1, 2, 3];
    println!("{}", a[2]); // 3

    // Mpas ->  A map allows associating one value with another value
    struct UserProfile {
        name: String,
        age: i32,
    }

    let users = vec![
        UserProfile {
            name: "Mick".to_string(),
            age: 30,
        },
        UserProfile {
            name: "Jenny".to_string(),
            age: 28,
        },
    ];
    // Iterate through the array to find "Mick" and print his age.
    println!(
        "{:?}",
        users
            .iter()
            .find(|profile| profile.name == "Mick")
            .unwrap()
            .age
    ); // 30

    // Sets -> A set holds a collection of unique values

    let mut cool_numbers = HashSet::from([2, 10, 8]);

    // Inserting an element that is already present will have no effect.
    cool_numbers.insert(8);

    // Sets implement basic set operations like subtraction.
    println!("{:?}", &cool_numbers - &HashSet::from([2])); // {10, 8}
}
