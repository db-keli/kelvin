use kelvin_rs::admin::Admin;
use kelvin_rs::data::{read_vault, write_vault};
use kelvin_rs::deck::Deck;
use kelvin_rs::deckdata::DeckData;
use kelvin_rs::password::generate_password;
use kelvin_rs::prompt::initialize_vault;
use rand::thread_rng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

fn vault_delete(domain: &str) {
    let mut vault = read_vault();
    vault.decks.retain(|d| d.domain != domain);
    write_vault(&vault).unwrap();
}

fn vault_update(domain: &str, new_password: &str, admin: Admin) {
    let mut vault = read_vault();
    if let Some(pos) = vault.decks.iter().position(|d| d.domain == domain) {
        let notes = vault.decks[pos].notes.clone();
        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let public_key = RsaPublicKey::from(&private_key);
        let ciphertext = public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, new_password.as_bytes())
            .unwrap();
        vault.decks[pos] = DeckData::new(
            admin,
            domain.to_string(),
            ciphertext,
            public_key,
            private_key,
            notes,
        );
    }
    write_vault(&vault).unwrap();
}

#[test]
fn test_encryption() {
    let plaintext = generate_password(21);
    let deck1 = Deck::new("google.com", &plaintext);
    let admin1 = Admin::new("Michael", &generate_password(12));
    let encrypted_data = deck1.encrypt();
    let pem_pub = encrypted_data.1 .1;
    let pem_prv = encrypted_data.1 .0;
    let decktemp = DeckData::new(admin1, deck1.domain, encrypted_data.0, pem_pub, pem_prv, None);

    assert_eq!(&plaintext.as_bytes()[..], &decktemp.decrypt()[..]);
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
    vault_delete(&domain);

    assert!(decktemp.test_read_data_from_json().is_err());
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
    vault_update(&domain, &new_password, admin);

    let updated = decktemp.test_read_data_from_json().unwrap();
    assert_eq!(String::from_utf8(updated.decrypt()).unwrap(), new_password);
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
    vault_update(&domain, &generate_password(12), admin);

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

    let domains: Vec<String> = read_vault().decks.into_iter().map(|d| d.domain).collect();
    assert!(domains.contains(&domain));
}
