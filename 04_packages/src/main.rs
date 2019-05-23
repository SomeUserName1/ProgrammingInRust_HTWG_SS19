mod animal;

use crate::animal::bird::duck::Duck;
use crate::animal::bird::duck::DuckNum;

fn main() {
    let default_duck = Duck::default();

    println!("This duck goes {}", default_duck.quack());
    println!("and its quack sounds like {}", default_duck.quack);

    // The next line won't compile if we uncomment it:
//    println!("The amount is {}", default_duck.amount);

    let quacker_duck = DuckNum::Quacker;
    //let quacker_duck = duck::DuckNum::Quacker !NOT IDIOMATIC!
}
