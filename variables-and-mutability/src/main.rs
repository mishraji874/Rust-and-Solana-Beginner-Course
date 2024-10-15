fn main() {
    // Variables are immutable in Rust, but we can make them mutable
    let x = 2;
    // Type Ascription
    // You can inform the compiler of the type you wish the variable to have through type ascription.
    let x: i16 = 2;

    // Mutability
    // Variables are immutable by default. That means the variable cannot be mutated or modified after its initialization.
    let x = 2;
    x = 3;

    // To declare a mutable variable, you can use `let mut``
    let mut x = 2;
    x = 3;

    // Scoping
    // Rust allows you to use variables declared in outer scopes, but variables do not leak out of their scope.
    let x = 2;
    println!("{}", x); // 2
    // Create a new scope
    {
        let y = 3;
        // We can use x here
        println!("{}", x); // 2
        println!("{}", y); // 3
    }
    println!("{}", x); // 2
    // println!("{}", y); // won't compile because y is "not in scope"

    // Shadowing
    // It is allowed to redefine a variable with the same name.
    let x = 2;
    let x = 3;
    // This is allowed in a nested scope
    let x = 2;
    {
        let x = 3;
    }
    println!("{}", x); // 2

    // Patterns
    // You can provide a pattern instead of just the variable name.
    let {x, y} = {2, 3};
}
