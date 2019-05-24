use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    exercise(100);
}

pub fn exercise(no_threads: u32) -> u32 {
    let data = Arc::new(Mutex::new(0u32));

    let (tx, rx) = mpsc::channel();

    for _ in 0..no_threads {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(());
        });
    }

    // join
    for _ in 0..no_threads {
        rx.recv();
    }
    let x = *data.lock().unwrap();
    x
}

#[cfg(test)]
mod tests {
    use super::exercise;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn assert_total_count() {
        // test wether there have been data races
        assert_eq!(10, exercise(10));
        assert_eq!(1000, exercise(1000));
    }

    #[test]
    fn reflection_test() {
        let mut file = File::open("src/main.rs").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents.split("#[cfg(test)]").next().unwrap().matches("join").count(), 0);

        assert!(contents.split("#[cfg(test)]").next().unwrap().matches("Mutex").count() > 0);

        assert!(contents.split("#[cfg(test)]").next().unwrap().matches("Arc").count() > 0);

        assert!(contents.split("#[cfg(test)]").next().unwrap().matches("send").count() > 0);

        assert!(contents.split("#[cfg(test)]").next().unwrap().matches("recv").count() > 0);
    }

}
