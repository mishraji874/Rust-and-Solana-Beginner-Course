use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Ingredient {
    Avocado,
    Bacon,
    Egg,
}

#[derive(Debug, Eq, PartialEq)]
struct BaconAndEggs;

#[derive(Debug, Eq, PartialEq)]
enum PreparationError {
    NotEnoughOfIngredient {
        ingredient: Ingredient,
        required: u32,
        available: u32,
    },
    // Other possible errors that could occur would go here.
}

/// Helper function to check if stock contains enough of an ingredient.
fn ensure_available(
    stock: &HashMap<Ingredient, u32>,
    ingredient: Ingredient,
    required: u32,
) -> Result<(), PreparationError> {
    let available = stock.get(&ingredient).copied().unwrap_or_default();

    if available >= required {
        Ok(())
    } else {
        Err(PreparationError::NotEnoughOfIngredient {
            ingredient,
            required,
            available,
        })
    }
}

/// Attempts to prepare bacon and eggs with ingredients that are in stock. The
/// stock is updated depending on the result.
fn prepare_bacon_and_eggs(
    stock: &mut HashMap<Ingredient, u32>,
) -> Result<BaconAndEggs, PreparationError> {
    const REQUIRED: [(Ingredient, u32); 2] = [(Ingredient::Bacon, 1), (Ingredient::Egg, 2)];

    // Ensure everything is available.
    for (ingredient, required) in REQUIRED {
        // Notice the use of the error propagation operator here. You return from
        // `prepare_bacon_and_eggs` early if `ensure_available` returns an
        // error.
        ensure_available(stock, ingredient, required)?;
    }

    // Update the stock.
    for (ingredient, required) in REQUIRED {
        // Since you verified that the hashmap contains all ingredients above you
        // can unwrap here.
        *stock.get_mut(&ingredient).unwrap() -= required;
    }

    Ok(BaconAndEggs)
}

fn main() {
    // Create a map from ingredient to a count that represents availability.
    let mut stock: HashMap<Ingredient, u32> = [
        (Ingredient::Avocado, 3),
        (Ingredient::Bacon, 2),
        (Ingredient::Egg, 1),
    ]
    .into_iter()
    .collect();

    // You should expect an error because bacon and eggs requires 2 eggs but you only have 1 in stock.
    assert_eq!(
        prepare_bacon_and_eggs(&mut stock),
        Err(PreparationError::NotEnoughOfIngredient {
            ingredient: Ingredient::Egg,
            required: 2,
            available: 1,
        })
    );

    // Add half a dozen of eggs to the stock.
    *stock.get_mut(&Ingredient::Egg).unwrap() += 6;

    // Verify that you now get bacon and eggs.
    assert_eq!(prepare_bacon_and_eggs(&mut stock), Ok(BaconAndEggs));

    // Make sure that the stock has been updated.
    assert_eq!(
        stock,
        [
            (Ingredient::Avocado, 3),
            (Ingredient::Bacon, 1),
            (Ingredient::Egg, 5),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>()
    )
}
