#![allow(dead_code, unused_variables)]
//

mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        Status::Connected
    }

    pub fn get_user() {}
}

/// A saving account
mod auth_utils {
    pub fn login(credentials: models::Credentials) {
        // auth
        crate::database::get_user();
    }

    fn logout() {}

    pub mod models {
        /// A struct representing a user's credentials `username` and `password`
        /// ```
        /// let cred = Credentials {
        ///    username: "user".to_string(),
        ///   password: "password".to_string(),
        /// };
        /// assert_eq!(cred.username, "user");
        /// assert_eq!(cred.password, "password");
        /// ```
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}


pub fn authenticate(cred: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login(cred);
    }
}

// cargo-modules : Help us to visualise module tree