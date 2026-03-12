// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// Don't worry about the function bodies themselves, we are only interested in
// the signatures for now.

fn is_pair(num: u8) -> bool {
    num % 2 == 0
}

// TODO: Fix the function signature.
fn sale_price(price: u8) -> u8 {
    if is_pair(price) {
        return price - 10
    }
    return price - 3
}    
   

fn main() {
    let original_price: u8 = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
