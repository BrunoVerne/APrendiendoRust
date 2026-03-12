// Booleans (`bool`)

fn main() {
    let is_morning: bool = true;
    let mut is_evening: bool = true;
    if is_morning {
        println!("Good morning!");
        is_evening = false
    }

    // TODO: Define a boolean variable with the name `is_evening` before the `if` statement below.
    // The value of the variable should be the negation (opposite) of `is_morning`.
    
    if is_evening {
        println!("Good evening!");
    }
}
