/******************************************************************************
Closure

*******************************************************************************/
fn main() {
    // Ferris has a recipe and ingredients
    let recipe = String::from("Apple pie");
    let ingredients = String::from("Apple, A Pie");

    // Ferris puts the recipe and ingredients in the box
    let ferris_recipe = move || {
        // This is the box where Ferris stores the recipe and ingredients
        println!("The recipe of {} is as follows:", recipe);
        println!("-- {}", ingredients);
    };

    // A few hundred years later, Ferris's descendants use the recipe
    ferris_recipe();
}

