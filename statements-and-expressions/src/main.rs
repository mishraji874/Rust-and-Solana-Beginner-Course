fn main() {
    // ternary if -> The if statement is an expression that in other languages is sometimes called the ternary conditional operator
    let brownies_eaten = 2;
    if brownies_eaten == 0 {
        println!("i had no brownies today");
    } else {
        println!("i had at least one brownie today");
    }

    // we can reduce the duplication in the above code using this:
    let quantifier;
    if brownies_eaten == 0 {
        quantifier = "no brownies";
    } else {
        quantifier = "at least one brownie";
    }
    println!("i had {quantifier} today");

    // we can simplify the above code by using if can be used as an expression
    let quantifier = if brownies_eaten == 0 {
        "no brownies"
    } else {
        "at least one brownie"
    };
    println!("I had {quantifier} today.");

    // if match is also an expression
    let quantifier = match brownies_eaten {
        0 => "no brownies",
        1 => "a brownie",
        _ => "multiple brownies",
    };
    println!("I had {quantifier} today.");

    let mut i = 0;
    let x = loop {
        if i > 7 {
            break i;
        }
        i += i*2 + 1;
    };
    println!("{x}");
}
