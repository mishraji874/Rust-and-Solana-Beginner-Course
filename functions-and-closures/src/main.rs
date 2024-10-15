fn main() {
    // In Rust, a function can be declared as follows:
    // fn fn_name(input1: InputType1, input2, InputType2) -> OutputType {
        // body
    //}
    // A function can have zero or more arguments, and it can optionally have an output.
    fn u32_add(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn u32_add_another(a: u32, b: u32) -> u32 {
        a + b
    }

    // It is allowed to define functions inside another function:
    fn f(n: u32) -> u32 {
        fn g(n: u32) -> u32 {
            n + 1
        }
        g(n * 2)
    }
    println!("{}", f(3));

    // Main function -> A Rust executable actually needs a function that defines the entry point of the program.

    struct X(&'static str);
    // An implementation block for the type `X`.
    impl X {
        // An associated function.
        fn associated_fn() -> &'static str {
            "I am always the same!"
        }
        // A method.
        fn method(self: &Self) -> &'static str {
            self.0
        }
    }
    // Call a function associated to the type `X`.
    println!("{}", X::associated_fn());
    // Create an instance of X and call a method on the instance.
    let instance = X("My value depends on an instance of `X`!");
    println!("{}", instance.method());

    // Closures -> Closures are very similar to functions, except they have the ability to "capture their environment".
    let c = |x| {
        x * 2
    };
    println!("{}", c(6));

    // if the closure body consist of the single expression, the curly braces can be omitted
    let c = |x| x * 2;
    println!("{}", c(6));

    // we can also use the above one using fn
    fn c(x: i32) -> i32 {
        x * 2
    }
    println!("{}", c(6));
    
    // Closures are most commonly used when iterating over collections of values.
}
