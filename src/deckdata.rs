use crate::admin;
use crate::data::{decrypt_directory, encrypt_directory};
use admin::Admin;
use crate::prompt::vault_path;
use rsa::{
    pkcs1::{self, DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use serde::{Deserialize, Serialize};

use serde_json::to_string;

use rsa::pkcs1::EncodeRsaPublicKey;
use std::fs::File;

use std::io::{self, prelude::*, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeckData {
    pub domain: String,
    pub ciphertext: Vec<u8>,
    pub admin_data: Admin,
    pub rsa_public_key: String,
    pub rsa_private_key: String,
}

impl DeckData {
    pub fn new(
        admin_data: Admin,
        domain: String,
        ciphertext: Vec<u8>,
        rsa_public_key: RsaPublicKey,
        rsa_private_key: RsaPrivateKey,
    ) -> DeckData {
        let pem_pub = rsa_public_key.to_pkcs1_pem(pkcs1::LineEnding::LF).unwrap();
        let pem_prv = rsa_private_key
            .to_pkcs1_pem(pkcs1::LineEnding::LF)
            .unwrap()
            .to_string();
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
        let filepath = format!("{}/{}.json",vault_path(), self.domain);
        let mut file = File::create(filepath)?;
        writeln!(file, "{}", contents)?;
        file.flush()?;
        let _ = encrypt_directory();
        Ok(())
    }
    #[allow(dead_code)]
    pub fn read_data_from_json(&self) -> Result<DeckData> {
        let filepath = format!("{}/{}.json",vault_path(), self.domain);
        let _ = decrypt_directory();
        let mut file = File::open(filepath)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        file.flush()?;
        let _ = encrypt_directory();
        let deck_data_vec: Vec<DeckData> = serde_json::from_str(&json_data)?;

        let deck_data = deck_data_vec
            .into_iter()
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No data found in JSON"))?;

        Ok(deck_data)
    }

    #[allow(dead_code)]
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
