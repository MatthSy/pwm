use std::io::{stdin, stdout, Write};
#[allow(unused)]
use crate::memory::{mem_get, mem_put};

pub(crate) fn no_cmd_app() {
    let mut stdout = stdout();
    println!("\n\n------------------PassWord Manager------------------\n");
    println!("\tWelcome to pwm, type a command to execute :");
    println!("\t  - put : save a password in memory");
    println!("\t  - get : get a password from memory\n");


    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer).unwrap();

    match buffer.trim().to_lowercase().as_str() {
        "get" => {
            mem_get(None);
            print!("Enter the site name to get its saved password :\n   ");
            stdout.flush().unwrap();
            let mut buffer = String::new();
            stdin.read_line(&mut buffer).unwrap();
            mem_get(Some(buffer.trim().parse().unwrap()));
        }
        "put" => {
            println!("Enter the site name you want to save password for :\n   ");
            let mut buffer = String::new();
            stdin.read_line(&mut buffer).unwrap();

            mem_put(String::from(buffer.trim()));
        }
        "exit" | "quit" | "cancel" | "clear" | "" => println!("\nEnd of task"),
        _ => println!("Unknown command"),
    }
}