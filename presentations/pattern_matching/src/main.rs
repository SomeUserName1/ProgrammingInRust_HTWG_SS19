mod exercise;
mod live;

fn main() {
    live::color_example();
    live::pointers_example();
    exercise::check_age(exercise::Person {name: String::from("Michael"), age: 42});
}
