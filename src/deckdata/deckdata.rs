use crate::admin;

use admin::admin::Admin;

use rsa::{pkcs1::{self, DecodeRsaPublicKey, EncodeRsaPrivateKey, DecodeRsaPrivateKey}, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use std::fs::File;
use rsa::pkcs1::EncodeRsaPublicKey;
use std::io::{self, prelude::*, Result};

//Data to save
#[derive(Serialize, Deserialize, Debug)]
pub struct DeckData {
    pub domain: String,
    pub ciphertext: Vec<u8>,
    pub admin_data: Admin,
    pub rsa_public_key: String,
    pub rsa_private_key: String,
}

impl DeckData {
    pub fn new(admin_data: Admin, domain: String, ciphertext: Vec<u8>, rsa_public_key: RsaPublicKey, rsa_private_key: RsaPrivateKey) -> DeckData {
        let pem_pub = rsa_public_key.to_pkcs1_pem(pkcs1::LineEnding::LF).unwrap();
        let pem_prv = rsa_private_key.to_pkcs1_pem(pkcs1::LineEnding::LF).unwrap().to_string();
        DeckData {
            domain,
            ciphertext,
            admin_data,
            rsa_public_key: pem_pub,
            rsa_private_key: pem_prv,
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

    pub fn read_data_from_json(&self) -> Result<DeckData> {
        let filepath = format!("./data/{}.json", self.domain);
        let mut file = File::open(filepath)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        file.flush()?;

        println!("{}", json_data);
        let deck_data_vec: Vec<DeckData> = serde_json::from_str(&json_data)?;

        // Extract the first item from the vector (assuming it contains only one item)
        let deck_data = deck_data_vec
            .into_iter()
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No data found in JSON"))?;

        Ok(deck_data)
    }

    // pub fn string_to_keys(&self) -> (RsaPrivateKey, RsaPublicKey) {
    //     let private_key = RsaPrivateKey::from_pkcs1_pem(&self.rsa_private_key).unwrap();
    //     let public_key = RsaPublicKey::from_pkcs1_pem(&self.rsa_public_key).unwrap();

    //     (private_key, public_key)        
    // }

    

    pub fn decrypt(&self) -> Vec<u8> {
        let private_key = RsaPrivateKey::from_pkcs1_pem(&self.rsa_private_key).unwrap();
        let public_key = RsaPublicKey::from_pkcs1_pem(&self.rsa_public_key).unwrap();

        let keys = (private_key, public_key);     
        let decrypted_data = keys
            .0
            .decrypt(Pkcs1v15Encrypt, &self.ciphertext)
            .expect("Failed to decrypt");

        decrypted_data
    }
}
