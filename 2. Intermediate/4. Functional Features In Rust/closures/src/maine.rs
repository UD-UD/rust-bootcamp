struct Credential<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}

impl<T> Credential<T>
where
    T: Fn(&str, &str) -> bool,
{
    fn is_valid(&self) -> bool {
        (self.validator)(self.username.as_str(), self.password.as_str())
    }
}

fn main() {
    let validator1 = |username: &str, password: &str| !username.is_empty() && !password.is_empty();

    let week_password = "password".to_string();
    let validator2 = |username: &str, password: &str| {
        !username.is_empty() &&
            !password.is_empty() &&
            password.len() > 8 &&
            password.contains(&['!', '@', '#', '$', '%', '^', '&', '*'][..]) &&
            password != week_password // Closure captures week_password from scope
    };

    let user = Credential {
        username: "admin".to_string(),
        password: "password".to_string(),
        validator: validator1,
    };

    let admin_user = Credential {
        username: "admin2".to_string(),
        password: "password".to_string(),
        validator: validator2,
    };

    println!("User {} \n", user.is_valid());

    println!("User2 {}", admin_user.is_valid());
    /*
        1. Fn: Immutably borrow variables in environment
        2. FnMut: Mutually borrow variables in environment. Can change environment
        3. FnOnce: Take ownership of variables in environment. Can only be called once
        4. move: Move variables from environment into closure, a.k.a take ownership
                For eg: let x = 5;
                        let closure = move || x;
                        println!("{}", x); // Error: x moved into closure
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}