use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    count(100);
}

pub fn count(no_threads: u32) -> u32 {
    let data = Arc::new(Mutex::new(0u32));

    let (tx, rx) = mpsc::channel();

    for _ in 0..no_threads {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(()).unwrap();
         });
    } 

    for _ in 0..no_threads {
        rx.recv().unwrap();
    }
    let x = *data.lock().unwrap();
    x
}

#[cfg(test)]
mod tests {
    use super::count;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn assert_total_count() {
        // test wether there have been data races
        assert_eq!(10, count(10));
        assert_eq!(1000, count(1000));
    }

    #[test]
    fn reflection_test() {
        let mut file = File::open("src/exercise.rs").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let code = contents.split("#[cfg(test)]").next().unwrap();
        assert_eq!(code.matches("join").count(), 0);
        println!("{}", code);
        assert!(code.matches("Mutex").count() > 0);

        assert!(code.matches("Arc").count() > 0);

        assert!(code.matches("send").count() > 0);

        assert!(code.matches("recv").count() > 0);
    }

}
