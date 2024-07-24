# Blossom-auth

Blossom-auth is a CLI program for testing and debugging Blossom server instances.


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
blossom-auth -a upload -d "8uv27z.jpg" -f "{PATH_TO_FILE}"
```
