fn main() {
    // Pattern Matching - Pattern matching allows you to deconstruct values of complex types into their parts. The pattern does not have to include all possibilities when conditionally matching. This is called a refutable pattern.
    // Syntax -> let <pattern>: <type> = <expression>;
    struct Plant {
        flowering: bool,
        mass: f64,
    }
    let Plant { flowering, mass };
    let plant = Plant {
        flowering: true,
        mass: 10.0,
    };
    let Plant { flowering, mass } = plant;

    enum Meal {
        FishAndChips { chip_proportion: f64 },
        Hamburger { vegetarian: bool },
    }
    let meal = Meal::Hamburger {
        vegetarian: true,
    };
    let Meal::Hamburger { vegetarian } = meal; // this will show the error

    // use of if-let
    let meal = Meal::Hamburger {
        vegetarian: true,
    };
    
    if let Meal::Hamburger { vegetarian } = meal {
        println!("I had a hamburger!");
    }

    // use of match -> allows you to specify multiple patterns to be matched against
    for n in 0..=5 {
        match n {
            1 => println!("It was one!"),
            2 => println!("It was two!"),
            // or-pattern
            3 | 4 => println!("It was a bit more than two!"),
            // match guard
            high if high >= 5 => println!("It was a high number!"),
            // a pattern consisting only of the name `other`
            other => println!("It was {other}!"),
        }
    }

    match meal {
        Meal::FishAndChips { chip_proportion } => {
            if chip_proportion > 0.5 {
                println!("I had some fish and plenty of chips!");
            } else if chip_proportion < 0.5 {
                println!("I had plenty of fish and some chips!");
            } else {
                println!("I had fish and chips!");
            }
        }
        Meal::Hamburger { vegetarian } => {
            if vegetarian {
                println!("I had a vegetarian hamburger!");
            } else {
                println!("I had a meaty hamburger!");
            }
        }
    }

    match meal {
        Meal::FishAndChips { chip_proportion } if chip_proportion > 0.5 => {
            println!("I had some fish and plenty of chips!");
        }
        Meal::FishAndChips { chip_proportion } if chip_proportion < 0.5 => {
            println!("I had plenty of fish and some chips!");
        }
        Meal::FishAndChips { chip_proportion } => {
            println!("I had fish and chips!");
        }
        Meal::Hamburger { vegetarian: true } => {
            println!("I had a vegetarian hamburger!");
        }
        Meal::Hamburger { vegetarian: false } => {
            println!("I had a meaty hamburger!");
        }
    }

    // use of while-let -> loop until pattern does not match
    let mut meal = Meal::FishAndChips {
        chip_proportion: 0.6,
    };
    while let Meal::FishAndChips { chip_proportion } = meal {
        println!("Having fish and chips with chip proportion {:.2}...", chip_proportion);
        if chip_proportion > 0.3 {
            // Order a meal with less chips.
            meal = Meal::FishAndChips {
                chip_proportion: chip_proportion - 0.2,
            }
        } else {
            // Too fishy, let's get a hamburger next.
            meal = Meal::Hamburger { vegetarian: true }
        }
    }
    println!("I'm so done with fish and chips!");

    // use of for -> accepts an irrefutable pattern
    let tuples: [(usize, &'static str); 3] = [
        (1, "red"),
        (2, "white"),
        (3, "blue"),
    ];

    for (numbering, color) in tuples {
        println!("Color #{numbering} is {color}!");
    }

    let colors = [
        "red",
        "white",
        "blue",
    ];

    for (index, color) in colors.into_iter().enumerate() {
        let numbering = index + 1;
        println!("Color #{numbering} is {color}!");
    }
}
