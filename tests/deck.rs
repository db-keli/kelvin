use kelvin::admin::*;
use kelvin::deck::*;
use kelvin::deckdata::*;
use kelvin::password::generate_password;
use kelvin::prompt::initialize_vault;

#[test]
fn test_encryption() {
    let plaintext = generate_password(21);
    let deck1 = Deck::new("google.com", &plaintext);

    let admin1 = Admin::new("Michael", &generate_password(12));

    let encrypted_data = deck1.encrypt();
    let pem_pub = encrypted_data.1 .1;
    let pem_prv = encrypted_data.1 .0;
    let decktemp = DeckData::new(admin1, deck1.domain, encrypted_data.0, pem_pub, pem_prv);

    let test_plaintext = &decktemp.decrypt()[..];

    assert_eq!(&plaintext.as_bytes()[..], test_plaintext);
}

#[test]
fn test_save_read_to_json() {
    let _ = initialize_vault();

    let plaintext = generate_password(12);
    let deck = Deck::new("facebook.com", &plaintext);

    let admin = Admin::new("Michael", &generate_password(12));
    let encrypted_data = deck.encrypt();

    let decktemp = DeckData::new(
        admin,
        deck.domain,
        encrypted_data.0,
        encrypted_data.1 .1,
        encrypted_data.1 .0,
    );

    let _ = decktemp.test_save_to_json();
    let read_deckdata = decktemp.test_read_data_from_json().unwrap();

    assert_eq!(read_deckdata, decktemp);
}
