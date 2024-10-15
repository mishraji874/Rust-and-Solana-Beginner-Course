fn main() {
    // If statement
    let should_print = true;
    if should_print {
        println!("Printing!");
    }

    // If-else statement
    let value = 10;
    if value > -10 && value < 10 {
        println!("Single digit");
    } else {
        println!("Multiple digits!");
    }

    // else-if statement
    let value = 10;
    if value == 0 {
        println!("zero");
    } else if value > -10 && value < 10 {
        println!("single digit");
    } else {
        println!("multiple digit");
    }

    // Looping - Three kinds of loop are there in rust
    // 1. loop -> loops indefinitely
    // 2. while
    // 3. for

    // loop
    // loop {
    //     println!("i can't stop!");
    // }
    // we can break the loop using `break` statement
    let mut i = 10;
    loop {
        if i == 0 {
            break;
        }
        println!("{i}...");
        i -= 1;
    }
    println!("launch!");

    // while
    let mut i = 10;
    while i != 0 {
        println!("{i}...");
        i -= 1;
    }
    println!("launch!");

    // for
    for i in (1..10).rev() {
        println!("{i}...");
    }
    println!("launch!");

    // use of continue
    for i in (1..10).rev() {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}...");
    }
    println!("launch!");
}
