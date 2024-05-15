#[allow(unused)]
use std::fmt::{Display, Octal};
use serde;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{File};
use std::io::{Read, Write};
use crate::encryption::{decrypt_mdp, encrypt_mdp};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct EncryptedData {
    pub(crate) mdp: [u8; 32],
    pub(crate) site: String,
    pub(crate) tag: [u8; 16],
}

pub(crate) fn mem_put(input_password:String, site: String) {
    let mut file = File::open("./data/data.json").expect("File opening error");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut data_vec: Vec<EncryptedData> = serde_json::from_str(&buffer).unwrap_or(vec![]);

    let input_encrypted = encrypt_mdp(input_password, site, data_vec.len() as u32);
    data_vec.push(input_encrypted);

    let serialized_data = serde_json::to_string_pretty(&data_vec).unwrap();

    let mut file = File::create("./data/data.json").expect("File opening error");
    file.write_all(serialized_data.as_ref()).unwrap();
}

pub(crate) fn mem_get(site: Option<String>) {
    let mut file = File::open("./data/data.json").expect("File opening error");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let data: Vec<EncryptedData> = serde_json::from_str(&buffer).unwrap();

    match site {
        Some(site) => {
            for i in 0..data.len() {
                if data[i].site == site {
                    println!("{}", decrypt_mdp(data[i].clone(), i as u32));
                }
            }
        }

        None => {
            println!("{:?}", data);
        }
    }
}

pub(crate) trait ToMdp {
    fn to_mdp(&self) -> [u8; 32];
}

impl ToMdp for String {
    fn to_mdp(&self) -> [u8; 32] {
        if self.len() > 32 { panic!("String too long in to_mdp fn from ToMdp trait, len is {} but must be 32 or shorter", self.len()); }
        let mut result: [u8; 32] = [0; 32];
        for i in 0..self.len() {
            result[i] = self.as_bytes()[i];
        }

        result
    }
}