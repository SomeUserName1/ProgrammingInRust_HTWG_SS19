use rand::{Rng};
use std::io::{self, BufRead, Write};


enum Fruit {
	Apfel(Color),
	Banane(Color),
	Kirsche(Color),
	Kiwi(Color),
}

impl Fruit {
	fn generate(tried_color: Color) -> Self {
		
	}

}

enum Color {
	Red,
	Blue,
	Green,
	Black,
	JeyJo,
}

fn main() {
    println!("Fruchtlotterie! Es wird gespielt bis
        alles verzockt ist! Startgeld ist 100");
    
    let mut balance = 100.0;
    
    while balance > 0.0 {
        let einsatz = get_einsatz();
        balance -= einsatz;
        
        match roll(einsatz) {
            Some(value) => {
                balance += value; 
                println!("{} gewonnen :) Neuer Kontostand: {}", value, balance)
            },
            None => println!("Leider verloren :( Neuer Kontostand: {}", balance),
        }
    }
    println!("Alles verzockt!");
}

//Fragt vom Benutzer einen Zahlenwert ab. Achtung Falscheingaben sind hier nicht behandelt!
fn get_einsatz() -> f32 {
    print!("Einsatz: ");
    
    io::stdout().flush().unwrap();
    
    let stdin = io::stdin();
    let mut line = String::new();
    
    stdin.lock().read_line(&mut line).unwrap();
    line = line.replace("\n", "").replace("\r", "");
    line.parse::<f32>().unwrap()
}

fn roll(einsatz: f32) -> Option<f32> {
    let fruit = Fruit::generate();

}
