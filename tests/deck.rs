use kelvin::deck::deck::*;
use kelvin::generate_password;
use rsa::Pkcs1v15Encrypt;

#[test]
fn test_encryption(){
    let plaintext = generate_password(21); 
    let deck1 = Deck::new("google.com", &plaintext);
    let mut keys = get_keys();
    
    let encrypted_data = deck1.encrypt();
    let encrypted_data2 = keys.1.encrypt(&mut keys.2, Pkcs1v15Encrypt, &plaintext.as_bytes()[..]).expect("Failed to encrypt");    

    assert_eq!(encrypted_data, encrypted_data2);
}