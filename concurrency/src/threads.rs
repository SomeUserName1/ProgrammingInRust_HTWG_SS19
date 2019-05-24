use std::thread;
use std::env::current_exe;
use std::cmp::min;

const BASE: u32 = 10;

fn main() {
    // number of characters in data: 287
    // sum should be 1275
    let data = "50864610078694052971775003929462155208093\
                      77064340425148538054538711850053702484659\
                      09642292853684872701496113871733802184854\
                      56909504073733513702607946560272618850856\
                      88992228101727465889633479915769825783860\
                      37443413087022608585008790236124865516461\
                      32427277631970591716922477548550122690640";

    let mut children = Vec::new();
    let chunked_data = chunk_data(data, 40);

    for chunk in chunked_data {
        children.push(thread::spawn(move || calc_sum(chunk)));
    }

    let mut sum = 0;

    for child in children {
        let res = child.join().unwrap();
        sum += res;
    }

    println!("sum: {}", sum);
}

fn calc_sum(data: &str) -> u32 {
    let mut sum: u32 = 0;

    for c in data.chars() {
        match c.to_digit(BASE) {
            Some(digit) => sum += digit,
            None => println!("Could not process '{}': should be a digit!", c)
        };
    }

    sum
}

fn chunk_data(data: &str, chunk_size: usize) -> Vec<&str> {
    let mut current = data;
    let mut chunked_data = Vec::new();

    while !current.is_empty() {
        let (chunk, rest) = current.split_at(min(chunk_size, current.len()));
        chunked_data.push(chunk);
        current = rest;
    }

    chunked_data
}