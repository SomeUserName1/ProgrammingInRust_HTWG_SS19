use std::sync::{Mutex, Arc};
use std::thread;
use std::fmt;

#[derive(Debug)]
struct List<T> {
    value : T,
    tail : Option<Box<List<T>>>
}

impl<T> List<T> {
    fn new(value: T) -> Self {
        List {    
            value,
            tail: None,
        }
    }
    
    fn insert(&mut self, value: T) {
        unimplemented!()
    }
}


impl<T> fmt::Display for List<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.tail {
            None => write!(f, "{}", self.value),
            Some(arc) => {
                // let guard = arc.lock().unwrap();
                write!(f, "{}, ", self.value)?;
                arc.fmt(f)
            }
        }
    }
}

fn main() {
    let mut list = List::new(0);
    println!("{:?}", list);

    //let mut handles = vec![];

    for i in 0..50 {
        //
        //let handle = thread::spawn(move || {
            //
            list.insert(i)
        //});
        //handles.push(handle);
    }
    //for handle in handles {
     //   handle.join().unwrap();
    //}
 
}
