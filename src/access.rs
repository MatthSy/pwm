use std::str::from_utf8;
use rpassword::read_password;
use crate::my_hasher::hash;

pub(crate) fn get_hashed_password() -> String {
    println!("Enter password : ");

    let password = read_password().expect("Failed to read Password");
    let password = password.trim();

    // Hash password and parse it back to string
    from_utf8(&*hash(&password).to_vec()).unwrap().to_string()
}

pub(crate) fn get_password() -> String {
    println!("Enter the password you want to save : ");
    String::from(
        read_password().expect("Failed to read Password").trim()
    )
}