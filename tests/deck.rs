use kelvin_rs::admin::*;
use kelvin_rs::data::list_decks;
use kelvin_rs::deck::*;
use kelvin_rs::deckdata::{test_delete_deck as deck_delete, test_update_deck as deck_update, DeckData};
use kelvin_rs::password::generate_password;
use kelvin_rs::prompt::initialize_vault;

#[test]
fn test_encryption() {
    let plaintext = generate_password(21);
    let deck1 = Deck::new("google.com", &plaintext);

    let admin1 = Admin::new("Michael", &generate_password(12));

    let encrypted_data = deck1.encrypt();
    let pem_pub = encrypted_data.1 .1;
    let pem_prv = encrypted_data.1 .0;
    let decktemp = DeckData::new(admin1, deck1.domain, encrypted_data.0, pem_pub, pem_prv, None);

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
        None,
    );

    let _ = decktemp.test_save_to_json();
    let read_deckdata = decktemp.test_read_data_from_json().unwrap();

    assert_eq!(read_deckdata, decktemp);
}

#[test]
fn test_deck_with_notes() {
    let _ = initialize_vault();

    let plaintext = generate_password(12);
    let deck = Deck::new("notes-test.com", &plaintext);
    let notes = Some("host: prod.example.com, port: 5432, user: admin".to_string());

    let admin = Admin::new("Michael", &generate_password(12));
    let encrypted_data = deck.encrypt();

    let decktemp = DeckData::new(
        admin,
        deck.domain,
        encrypted_data.0,
        encrypted_data.1 .1,
        encrypted_data.1 .0,
        notes.clone(),
    );

    let _ = decktemp.test_save_to_json();
    let read_deckdata = decktemp.test_read_data_from_json().unwrap();

    assert_eq!(read_deckdata.notes, notes);
}

#[test]
fn test_delete_deck() {
    let _ = initialize_vault();

    let plaintext = generate_password(12);
    let deck = Deck::new("delete-test.com", &plaintext);
    let domain = deck.domain.clone();

    let admin = Admin::new("Michael", &generate_password(12));
    let encrypted_data = deck.encrypt();

    let decktemp = DeckData::new(
        admin,
        domain.clone(),
        encrypted_data.0,
        encrypted_data.1 .1,
        encrypted_data.1 .0,
        None,
    );

    decktemp.test_save_to_json().unwrap();
    deck_delete(&domain).unwrap();

    let result = decktemp.test_read_data_from_json();
    assert!(result.is_err());
}

#[test]
fn test_update_deck() {
    let _ = initialize_vault();

    let plaintext = generate_password(12);
    let deck = Deck::new("update-test.com", &plaintext);
    let domain = deck.domain.clone();

    let admin = Admin::new("Michael", &generate_password(12));
    let encrypted_data = deck.encrypt();

    let decktemp = DeckData::new(
        admin.clone(),
        domain.clone(),
        encrypted_data.0,
        encrypted_data.1 .1,
        encrypted_data.1 .0,
        None,
    );

    decktemp.test_save_to_json().unwrap();

    let new_password = generate_password(12);
    deck_update(&domain, &new_password, admin).unwrap();

    let updated = decktemp.test_read_data_from_json().unwrap();
    let decrypted = String::from_utf8(updated.decrypt()).unwrap();
    assert_eq!(decrypted, new_password);
}

#[test]
fn test_update_preserves_notes() {
    let _ = initialize_vault();

    let plaintext = generate_password(12);
    let deck = Deck::new("update-notes-test.com", &plaintext);
    let domain = deck.domain.clone();
    let notes = Some("host: db.example.com, port: 5432".to_string());

    let admin = Admin::new("Michael", &generate_password(12));
    let encrypted_data = deck.encrypt();

    let decktemp = DeckData::new(
        admin.clone(),
        domain.clone(),
        encrypted_data.0,
        encrypted_data.1 .1,
        encrypted_data.1 .0,
        notes.clone(),
    );

    decktemp.test_save_to_json().unwrap();

    let new_password = generate_password(12);
    deck_update(&domain, &new_password, admin).unwrap();

    let updated = decktemp.test_read_data_from_json().unwrap();
    assert_eq!(updated.notes, notes);
}

#[test]
fn test_list_decks() {
    let _ = initialize_vault();

    let admin = Admin::new("Michael", &generate_password(12));
    let plaintext = generate_password(12);
    let deck = Deck::new("list-test.com", &plaintext);
    let domain = deck.domain.clone();
    let encrypted_data = deck.encrypt();

    let decktemp = DeckData::new(
        admin,
        domain.clone(),
        encrypted_data.0,
        encrypted_data.1 .1,
        encrypted_data.1 .0,
        None,
    );

    decktemp.test_save_to_json().unwrap();

    let domains: Vec<String> = list_decks().into_iter().map(|(d, _)| d).collect();
    assert!(domains.contains(&domain));
}
