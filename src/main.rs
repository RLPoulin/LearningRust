use std::default::Default;
use std::env;
use std::fmt;
use std::io::{stdin, stdout, Write};

fn main() {
    demo();
}

fn demo() {
    println!("\n0. Create a user directly:");
    let user = User {
        name: "Alice".to_string(),
    };
    user.greet();

    println!("\n1. Create a user with from:");
    let user = User::from("Bob");
    user.greet();

    println!("\n2. Create a user from the default:");
    let user = User::default();
    user.greet();

    println!("\n3. Create a user from the environment:");
    let user = User::from_env();
    user.greet();

    println!("\n4. Create a user from console input:");
    let user = User::from_stdin();
    user.greet();
}

struct User {
    name: String,
}

impl User {
    fn greet(&self) {
        println!("Hello, {}!", self)
    }

    fn from_env() -> Self {
        #[cfg(target_family = "windows")]
        const USER_VAR: &str = "USERNAME";

        #[cfg(target_family = "unix")]
        const USER_VAR: &str = "NAME";

        match env::var(USER_VAR) {
            Ok(name) => Self::from(name),
            Err(_) => Self::default(),
        }
    }

    fn from_stdin() -> Self {
        print!("What is your name? ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("I want a name!");

        let input = input.trim();
        if input.chars().count() > 0 {
            return Self::from(input);
        }
        Self::default()
    }
}

impl Default for User {
    fn default() -> Self {
        Self::from("World")
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&str> for User {
    fn from(s: &str) -> Self {
        Self {
            name: s.to_string(),
        }
    }
}

impl From<String> for User {
    fn from(name: String) -> Self {
        Self { name }
    }
}
