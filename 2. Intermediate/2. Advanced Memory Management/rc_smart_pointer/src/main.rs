use std::cell::RefCell;
use std::rc::Rc;

struct Database {
    max_connections: u32,
}

struct AuthService {
    db: Rc<RefCell<Database>>,
}

struct ContentService {
    db: Rc<RefCell<Database>>,
}

fn main() {
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100
    }));
    let auth_service = AuthService {
        db: Rc::clone(&db)
    };
    let content_service = ContentService {
        db: Rc::clone(&db)
    };

    println!("{}", Rc::strong_count(&db));

    let mut r1 = db.borrow_mut();
    let r2 = db.borrow_mut(); // Panics 
    db.borrow_mut().max_connections = 200;
}

// Rc::strong_count
// RC pointers only allow immutable data.
// Answer is RefCell
// RefCell uses interior mutablity principle.
// RefCell is used when we want shared ownership and also be able to mutate the value in it.