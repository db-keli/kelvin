use crate::admin;


use admin::admin::Admin;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use std::fs::{self, File, OpenOptions};
use std::io::{self, prelude::*, BufReader, Result};

//Data to save
#[derive(Serialize, Deserialize, Debug)]
pub struct DeckData {
    pub domain: String,
    pub ciphertext: Vec<u8>,
    pub admin_data: Admin,
}



impl DeckData {
    pub fn new(admin_data: Admin, domain: String, ciphertext: Vec<u8>) -> DeckData {
        DeckData {
            domain,
            ciphertext,
            admin_data,
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
        file.flush()?;

        Ok(())
    }

    pub fn read_data_from_json(&self) -> Result<DeckData>{
        let filepath = format!("./data/{}.json", self.domain);
        let mut file = File::open(filepath)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        file.flush()?;

        println!("{}", json_data);
        let deck_data_vec: Vec<DeckData> = serde_json::from_str(&json_data)?;

        // Extract the first item from the vector (assuming it contains only one item)
        let deck_data = deck_data_vec.into_iter().next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "No data found in JSON")
        })?;
        
        Ok(deck_data)
    }
}