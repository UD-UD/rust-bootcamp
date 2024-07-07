// Fix the code by addressing the TODO.

#[derive(Debug)]
struct User {
    name: String,
    id: u32,
}

fn fetch_user(id: u32) -> Result<User, String> {
    let database = vec![
        User { name: "Alice".to_string(), id: 1 },
        User { name: "Bob".to_string(), id: 2 },
        User { name: "Cindy".to_string(), id: 3 },
    ];
    for user in database {
        if user.id == id {
            return Ok(user);
        }
    }
    Err("User record not present".to_string())
}

fn main() {
    let user = fetch_user(2).ok();

    println!("User details: {user:?}");
}
