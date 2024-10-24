# kelvin

### Intro
The kelvin crate provides modules and methods used for creating simple password encryption managers. The crate was mainly built
while working on kelvin, a terminal password manager.

`terminal password manager`

![kelvin](kelvin.png)

### about

kelvin is a password manager like 1password but for the terminal. kelvin generates strong passwords from the terminal to your clipboard. generated passwords could be encrypted and saved with the domain names or what they correspond to, locally. all encrypted passwords are saved locally for later decryption by the authorized user. kelvin saves the data locally and encrypts the hidden directory with the gnu gpg.

<p>
    in future kelvin is to be a terminal app with tui, daemonized so it could be started with a keybinding when user wants to get their password or add a password to the vault.
</p>

<p>still building and welcoming contributions</p>

### features to handle

- [ ] terminal user interface
- [x] clipboard communication
- [x] encrypt directory that saves encrypted password data with gnu gpg
- [x] handle multiple decks for same domain
- [x] reset vault
- [ ] daemonize

## Installation

To install Kelvin, follow these steps:

1. **Clone the repository:**

   ```sh
   git clone https://github.com/db-keli/kelvin.git
   cd kelvin
   ```

2. **Build the project using Cargo:**

   ```sh
   cargo build --release
   ```

3. **Install the tool:**
   ```sh
   cargo install --path .
   ```

## Usage

Kelvin offers several commands for different operations related to password management and user account administration:

### Commands Overview

- `generate`
- `create-admin`
- `deck`
- `reset`
- `open-sesame`
- `help`

**Usage:**

```sh
kelvin [COMMAND]
```

### contributing

Welcoming contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for more details on how to contribute.

join [discord server](https://discord.gg/kMb55jNV9T) for community.
