//! ## Intro
//! The kelvin crate provides modules and methods used for creating simple password encryption managers. The crate was mainly built
//! while working on kelvin, a terminal password manager.
//!
//! ## Features
//! - [ ] terminal user interface
//! - [ ] clipboard communication
//! - [x] encrypt directory that saves encrypted password data with gnu gpg
//! - [ ] handle multiple decks for same domain
/// Admin module for creating a struct for superusers to passwords
pub mod admin;
/// Data module contains methods for managing data stored locally
pub mod data;

/// Deck module for creating a struct for decks(domain name and passwors to the specific domain) by and admin
pub mod deck;

/// Deckdata module for dealing with final data created after user add a a deck
pub mod deckdata;

/// password module contains methods for managing secure password generation
pub mod password;

/// prompt module contains methods for interacting with the user on the terminal
pub mod prompt;