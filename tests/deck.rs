use kelvin::deck::deck::*;
use kelvin::generate_password;
use rsa::Pkcs1v15Encrypt;

#[test]
fn test_encryption(){
    let plaintext = generate_password(21); 
    let deck1 = Deck::new("google.com", &plaintext);
    
    deck1.encrypt();

    let test_plaintext = &deck1.decrypt()[..];

    assert_eq!(&plaintext.as_bytes()[..], test_plaintext);
}