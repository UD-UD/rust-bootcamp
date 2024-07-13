use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Database {
    connections: Vec<u32>,
}

impl Database {
    fn new() -> Database {
        Database {
            connections: vec![]
        }
    }

    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}

fn main() {
    // Get mutex
    let db = Arc::new(Mutex::new(Database::new()));

    let mut handles = vec![];

    for i in 0..10 {
        let db_ref = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let mut db_lock = db_ref.lock().unwrap();
            db_lock.connect(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", db);
    // lock is dropped automatically when it goes out of scope
    let db_lock = db.lock().unwrap();

    println!("{:?}", db_lock);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main()
    }
}