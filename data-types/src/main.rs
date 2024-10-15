fn main() {
    println!("Hello, world!");
    // Data types are of 4 types: Integer, Float, Boolean, Characters

    // Integer: An integer represents a "whole" number. Integers are numbers that do not have a fractional part. They often come in two flavors: signed and unsigned.
    // 1. Signed:- represent negative numbers
    // 2. Unsigned:- represent only positive numbers
    // Integers also come in a few different memory sizes:- 8-bit, 16-bit, 32-bit, 64-bit, 128-bit

    // Floats: The floating point is used to represent numbers with fractional parts.
    // Rust provides the following floating point types: 32-bit, 64-bit

    // Boolean: The boolean is a type named bool which only has two values: true and false. While bool only requires 1 bit of memory to store, they take up one byte in size.

    // Character: Rust has a type called
    // 1. char:- which can store a unicode scalar value. This type lends itself well for storing a single character, but not for storing a piece of text because it is 4 bytes in size. To store text, use the
    // 2. str:- type, which represents a UTF-8 encoded string stored as a sequence of bytes.

    // Compound Types: Compound types are types that are composed of one or more other types. Rust's compound types can be subdivided into anonymous and named compound types.
    // It is of two types: Arrays, Tuples

    // Arrays: An array represents an ordered sequence of array items, where each array item is of the same type.
    // The array type is written as [T; N] where T is the type and N is the number of array items
    let array: [u32; 3] = [1, 2, 3];
    let array = [1, 2, 3];
    println!("{}", array[0]);
    println!("{}", array[1]);
    println!("{}", array[2]);

    // Slices:An array is of a known length. Often, however, you would like to be able to write code that can handle arrays of any length. This is where slices come in. The slice type is written as [T] where T is the type. Notice that the length is not specified. 👉 The slice is known as an unsized type or a dynamically sized type (DST). It is not possible to store an unsized type on the stack.
    // let a: [u8]; -> not compile
    // let a: &[u8] = &[1, 2, 3]; -> store a reference to an unsized type
    // let s: &str = "Hello World!";

    // Tuples: A tuple represents an ordered sequence of tuple items, where each tuple item may be of a different type. The tuple type is written as (T1, T2, ...) where T1 T2 and are the types of the tuple items.
    let tuple: (bool, u32, f64) = (true, 2, 3.0);
    let tuple = (true, 2, 3.0);
    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);

    // Tuple Structs: It can be useful to name tuples and can be done with tuple structs.
    // Tuple structs can be declared as follows:
    struct MyTuple(bool, u32, f64);
    // A value of type MyTuple can be created as follows:
    let tuple = MyTuple(false, 2, 3.0);
    // Another way of naming a tuple using type alias:
    type MyTupleAlias = (bool, u32, f64);

    // Structs: While tuple structs could be used to represent complex objects, giving a name to each tuple field is often useful.
    struct MyStruct {
        should_do_groceries: bool,
        birth_year: u32,
        height_in_meters: f64,
    }
    let my_struct = MyStruct {
        should_do_groceries: true,
        birth_year: 1992,
        height_in_meters: 1.79,
    };

    // Enumerations: In programming, you often need to represent a fixed set of possible values known at compile time.
    enum CardinalDirections {
        North,
        East,
        South,
        West,
    }
    let d = CardinalDirections::East;
    if let CardinalDirections::East = d {
        println!("We are going east!");
    } else {
        println!("We are not going east but in some other direction!");
    }

    // Tagged Unions: Attaching data to enum variants can be useful when you want to put different data types into a single collection, and the different types of data are known at compile time. This is sometimes referred to as tagged unions in other programming languages.
    enum Shape {
        Square { side: f64 },
        Rectangle { width: f64, height: f64 },
        Circle { radius: f64 },
    }
    let s = Shape::Rectangle {
        width: 800.0,
        height: 60.0,
    };
    match s {
        Shape::Square { side } => {
            println!("A {}x{} square!", side, side);
        }
        Shape::Rectangle { width, height } => {
            println!("A {}x{} rectangle!", width, height);
        }
        Shape::Circle { radius } => {
            println!(
                "A circle of radius {} and diameter {}!",
                radius,
                radius * 2.0
            );
        }
    }
}
