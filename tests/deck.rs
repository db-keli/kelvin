use kelvin::admin::admin::Admin;
use kelvin::deck::deck::*;
use kelvin::deckdata::deckdata::DeckData;
use kelvin::generate_password;

//#[test]
// fn test_encryption(){
//     let plaintext = generate_password(21); 
//     let deck1 = Deck::new("google.com", &plaintext);
    
//     let admin1 = Admin::new("Michael", &generate_password(12));

//     let encrypted_data = deck1.encrypt();
//     let decktemp = DeckData::new(admin1, deck1.domain, encrypted_data.0);

//     let keys = (encrypted_data.1.0, encrypted_data.1.1);
//     let test_plaintext = &decktemp.decrypt(keys)[..];

//     assert_eq!(&plaintext.as_bytes()[..], test_plaintext);
// }