pub struct Person {
    pub name: String,
    pub age: u8,
}

pub fn check_age(person: Person) {
    match person.age {
        a @ 0..=6 => println!("{} is 6 years old or younger, namely {} years", person.name, a),
        b @ 7..=12 => println!("{} is between 7 and 12 years old, namely {} years", person.name, b),
        c @ 13..=18 => println!("{} is between 13 and 18 years old, namely {} years", person.name, c),
        d @ _ => println!("{} is older than 18 years, namely {} years", person.name, d)
    }
}
