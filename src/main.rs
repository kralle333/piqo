mod commands;
mod data_storage;
mod models;
mod printing;
mod utils;

fn main() {
    commands::parse().unwrap();
}
