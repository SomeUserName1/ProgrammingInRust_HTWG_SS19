use std::sync::{Mutex, Arc};
use std::thread;
use std::fmt;

#[derive(Debug)]
struct List<T> {
	value: T,
	tail: Option<Arc<Mutex<List<T>>>>,
}

impl<T> List<T> {
	fn new(value: T) -> Self {
        List {
            value,
            tail: None,
        }
    }

    fn insert(&mut self, value: T) {

       match &self.tail {
            None => self.tail = Some(Arc::new(Mutex::new(Self::new(value)))),
            Some(list) => {
                let mut guard = list.lock().unwrap();
                guard.insert(value);
            }
       }
    }
}

impl<T> fmt::Display for List<T> where T: std::fmt::Display {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.tail {
            None => write!(f, "{}", self.value),
            Some(arc) => {
                let guard = arc.lock().unwrap();
                write!(f, "{}, ", self.value)?;
                guard.fmt(f)
            }
        }
    }
}

fn main() {
	let list = Arc::new(Mutex::new(List::new(0)));
	println!("{}", list.lock().unwrap());

    let mut handles = vec![];

    for i in 0..10 {
        let l = Arc::clone(&list);
        let handle = thread::spawn(move || {
            let mut s = l.lock().unwrap();
            s.insert(i + 100);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{}", list.lock().unwrap());
}

