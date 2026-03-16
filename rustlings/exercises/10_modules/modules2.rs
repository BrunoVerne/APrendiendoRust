// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

   mod fruits {
       pub const PEAR: &str = "Pear";
        const APPLE: &str = "Apple";
    }

    mod veggies {
       pub const CUCUMBER: &str = "Cucumber";
        const CARROT: &str = "Carrot";
    }

    pub use  self::fruits::PEAR as my_pear;
    pub use self::veggies::CUCUMBER as my_cucumber;
}


use delicious_snacks::my_pear as my_pear_main;
use delicious_snacks::my_cucumber as my_cucumber_main;
fn main() {
    println!(
        "favorite snacks: {} and {}",
        my_pear_main,
        my_cucumber_main
        
    );
}
