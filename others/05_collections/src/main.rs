use std::io;

fn main() {
    let reader = io::stdin();
    let mut input = String::new();

    loop {
        println!("\nWort(e) eingeben:");
        match reader.read_line(&mut input) {
            Ok(_) => {
            	// Für jedes eingegebene Wort parse_input() aufrufen
                input = parse_input(&mut input);
                println!("{}", input);
                input = String::new();

            },
            Err(error) => println!("error: {}", error),
        }
    }
}

fn parse_input(s: &mut String) -> String {
    // Hier String in Pig Latin String übersetzen
    match &s[..1] {
        a|| e || i || o || u => {
            // just append -ay
            s.push("-ay")
        }
        _ => {
            // iterate over the string till non-consonant is hit
            for c in s.chars() {
                match c {
                    
                }
                s
            }
        }
    }
    s
}
