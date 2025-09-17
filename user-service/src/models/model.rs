use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
}

impl User {
    fn new(username: String, email: String, password: String) -> Self {
        User { username, email }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Create_User {
    id: Option<i32>,
    username: String,
    email: String,
    password: String,
    is_active: bool,
    is_admin: bool,
    nit: i32,
}

impl Create_User {
    fn new(
        username: String,
        email: String,
        password: String,
        is_active: bool,
        is_admin: bool,
        nit: i32,
    ) -> Self {
        Create_User {
            username,
            email,
            password,
            is_active,
            is_admin,
            nit,
        }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_is_active(&self) -> bool {
        self.is_active
    }

    pub fn get_is_admin(&self) -> bool {
        self.is_admin
    }

    pub fn get_nit(&self) -> i32 {
        self.nit
    }
}
