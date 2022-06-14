use std::default::Default;
use std::env;
use std::error;
use std::fmt;
use std::io::{self, stdin, stdout, Write};

// Demo for the User class
pub fn demo() {
    println!("\n0. Create a user directly:");
    let user = User {
        name: "Alice".to_string(),
    };
    user.greet();

    println!("\n1. Create a user from a &str:");
    let user = User::from("Bob");
    user.greet();

    println!("\n2. Create a user from the default:");
    let user = User::default();
    user.greet();

    println!("\n3. Create a user from the environment:");
    let user = User::from_env().unwrap_or_default();
    user.greet();

    println!("\n4. Create a user from console input:");
    let user = User::from_stdin().unwrap_or_default();
    user.greet();
}

// The User class
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct User {
    name: String,
}

impl User {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn greet(&self) {
        println!("Hello, {self}!")
    }

    pub fn new(name: String) -> Result<Self, NewUserError> {
        let name = name.trim();
        if name.chars().count() > 0 {
            return Ok(Self { name: name.into() });
        }
        Err(NewUserError::InvalidName("name too short".into()))
    }

    pub fn from_env() -> Result<Self, NewUserError> {
        #[cfg(target_family = "windows")]
        const USER_VAR: &str = "USERNAME";

        #[cfg(target_family = "unix")]
        const USER_VAR: &str = "USER";

        let name = env::var(USER_VAR)?;
        Self::new(name)
    }

    pub fn from_stdin() -> Result<Self, NewUserError> {
        print!("What is your name? ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        Self::new(input)
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "World".into(),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&str> for User {
    fn from(s: &str) -> Self {
        Self { name: s.into() }
    }
}

impl From<String> for User {
    fn from(name: String) -> Self {
        Self { name }
    }
}

// Custom Result and Error for creating a new user.
#[derive(Debug)]
pub enum NewUserError {
    InvalidName(String),
    Io(io::Error),
    Var(env::VarError),
}

impl From<io::Error> for NewUserError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<env::VarError> for NewUserError {
    fn from(e: env::VarError) -> Self {
        Self::Var(e)
    }
}

impl error::Error for NewUserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use NewUserError::*;
        match *self {
            InvalidName(_) => None,
            Io(ref e) => Some(e),
            Var(ref e) => Some(e),
        }
    }
}

impl fmt::Display for NewUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NewUserError::*;
        match *self {
            InvalidName(ref msg) => write!(f, "invalid name: {msg}"),
            Io(ref e) => write!(f, "io error: {e}"),
            Var(ref e) => write!(f, "environment error: {e}"),
        }
    }
}
