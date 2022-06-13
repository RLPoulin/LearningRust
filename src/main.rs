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

    println!("--> And printing debug info:");
    println!("{user:#?}");

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

    println!("\n5. Create a user by cloning the last one:");
    let user2 = user.clone();
    user2.greet();

    println!("--> And check for equality:");
    let is_equal = user == user2;
    println!("{is_equal:#?}");
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct User {
    name: String,
}

impl User {
    fn greet(&self) {
        println!("Hello, {self}!")
    }

    fn new(name: String) -> Self {
        let name = name.trim();
        if name.chars().count() > 0 {
            return Self { name: name.into() };
        }
        Self::default()
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
        stdin().read_line(&mut input).expect("That's not a name!");
        Self::new(input)
    }
}

impl Default for User {
    fn default() -> Self {
        Self::new("World".into())
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&str> for User {
    fn from(name: &str) -> Self {
        Self::new(name.into())
    }
}

impl From<String> for User {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}
