use std::io::{stdin};
use std::str::from_utf8;
use crate::my_hasher::hash;

pub(crate) fn get_mdp_input() -> String {
    println!("Enter password : ");
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer).unwrap();

    // Ligne dégueu:
    from_utf8(&*hash(&buffer).to_vec()).unwrap().to_string() // hash le mdp et le passe en String
}