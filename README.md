<img src="kelvin-logo.png" width="120" alt="kelvin logo">

A terminal password manager. Generates strong passwords to your clipboard and stores them encrypted locally — no cloud, no external tools, one master password.

Vault is a single `~/.vault.enc` file locked with AES-256-GCM. Key derived from your master password via Argon2id. One prompt per session.

---

## Installation

### Option 1 — pre-built binary (macOS x86_64)

Download from the [latest release](https://github.com/db-keli/kelvin/releases/latest):

```sh
curl -L https://github.com/db-keli/kelvin/releases/latest/download/kelvin-macos-x86_64 -o kelvin
chmod +x kelvin
mv kelvin /usr/local/bin/kelvin
```

### Option 2 — build from source

Requires the [Rust toolchain](https://rustup.rs).

```sh
git clone https://github.com/db-keli/kelvin.git
cd kelvin
cargo install --path .
```

---

## Usage

```sh
kelvin [COMMAND]
```

| Command        | Description                                                               |
| -------------- | ------------------------------------------------------------------------- |
| `generate`     | Generate a random password and copy to clipboard. Use `-l` to set length. |
| `create-admin` | Set up your vault and admin account.                                      |
| `deck`         | Add a password entry. Prompts for domain, password, and optional notes.   |
| `list`         | List all stored domains with notes.                                       |
| `open-sesame`  | Retrieve a password to clipboard. Pass `--stdout` to print instead.       |
| `env`          | Output a password as `export VAR=value` for shell scripting.              |
| `update`       | Change the password for a stored entry.                                   |
| `delete`       | Remove an entry by domain name.                                           |
| `reset`        | Wipe the vault.                                                           |

### Example

```sh
# first run — creates the vault
kelvin create-admin

# add a server credential
kelvin deck
# Enter domain: prod/db
# Enter password: ••••••
# Enter notes (optional): host: db.prod.example.com, port: 5432

# list everything
kelvin list

# retrieve to clipboard
kelvin open-sesame

# use in a script
eval $(kelvin env)
echo $DB_PASSWORD
```

---

## Roadmap

- [x] Clipboard with auto-clear (30s)
- [x] AES-256-GCM encrypted vault
- [x] Namespace support (`prod/db`, `staging/api`)
- [x] Notes per entry
- [x] List, update, delete entries
- [x] Script-friendly output (`--stdout`, `env`)
- [ ] Terminal UI (TUI)
- [ ] Daemonize with keybinding

---

## Contributing

Contributions welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.
