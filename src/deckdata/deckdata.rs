use crate::admin;


use admin::admin::Admin;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use std::fs::{self, File, OpenOptions};
use std::io::{self, prelude::*, BufReader, Result};

//Data to save
#[derive(Serialize, Deserialize, Debug)]
pub struct DeckData {
    admin_data: Admin,
    domain: String,
    ciphertext: Vec<u8>,
}

impl DeckData {
    pub fn new(admin_data: Admin, domain: String, ciphertext: Vec<u8>) -> DeckData {
        DeckData {
            admin_data,
            domain,
            ciphertext,
        }
    }

    pub fn serialize_struct(&self) -> String {
        let dat_ser = to_string(&vec![self]);
        if let Err(err) = &dat_ser {
            err.to_string()
        } else {
            dat_ser.unwrap()
        }
    }

    pub fn save_to_json(&self) -> Result<()> {
        let contents = self.serialize_struct();
        let filepath = format!("./data/{}.json", self.domain);

        let mut file = File::create(filepath)?;
        writeln!(file, "{}", contents)?;

        Ok(())
    }

    pub fn read_data_from_json(filepath: String, domain: &str) -> Result<()> {
        let mut file = File::open(filepath)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;

        for line in json_data.split('\n').collect::<Vec<_>>() {
            if line.contains(domain) {
                println!("{:#?}", line);
            }
            //let deck_data: Vec<DeckData> = serde_json::from_str(&json_data)?;
        }

        Ok(())
    }
}