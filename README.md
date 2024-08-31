# Blossom-auth

Blossom-auth is a CLI program to easily generate nostr authorization events for testing/debugging blossom servers.
It lets you generate events for uploading and deleting blobs. It also has some options for faking file hashes and using
invalid kinds in the authorization event.

### Features
- Generate valid events for all types of actions
- Generate invalid event with fake event signature
- Generate invalid event with invalid kind
- Generate invalid event with fake file hash in x-tag

## Install from crates.io
```
cargo install blossom-auth
```

## Build from source
```
git clone https://github.com/0xtrr/blossom-auth.git
cd blossom-auth
cargo build --release
./target/release/blossom-auth -h
```

## Example usage

Get an overview of available options
```
blossom-auth -h
```

Generate a new upload authorization event
```
blossom-auth -a upload -d "{FILE_NAME}" -f "{PATH_TO_FILE}"
```

## Contributing

All contributions are welcome! If you have a good idea for the CLI please either make a PR or reach out in the 
issues section.

Ensure that you run `cargo fmt` and `cargo clippy` before creating the PR.