use clap::{arg, command, Parser};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(long)]
    role: String,
}

impl Args {
    pub fn command(&self) -> Role {
        match self.role.trim() {
            "admin" => Role::Admin,
            "user" => Role::User,
            _ => Role::User,
        }
    }
}

pub enum Role {
    Admin,
    User,
}

impl Role {
    pub fn port(&self) -> u16 {
        match self {
            Role::Admin => 7878,
            Role::User => 8081,
        }
    }
}
