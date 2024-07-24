use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;
use std::time::SystemTime;

use clap::Parser;
use nostr_sdk::base64::engine::general_purpose;
use nostr_sdk::base64::Engine;
use nostr_sdk::{
    JsonUtil, Keys, Kind, SecretKey, SingleLetterTag, Tag, TagKind, Timestamp, ToBech32,
};
use rand::Rng;
use sha2::{Digest, Sha256};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Nostr private key (hex)
    #[arg(short, long)]
    private_key: Option<String>,
    /// Blossom action (get, upload, list or delete)
    #[arg(short, long)]
    action: String,
    /// Description (put into content field in event)
    #[arg(short, long)]
    description: String,
    /// Path to file that is to be uploaded
    #[arg(short, long)]
    file_path: String,
    /// Puts a random generated sha256 hash in the x tag of the event
    #[arg(long, default_value = "false")]
    fake_file_hash: bool,
    /// Sets incorrect kind in the authorization event
    #[arg(long, default_value = "false")]
    invalid_kind: bool,
}

fn main() {
    let args = Cli::parse();

    let keys: Keys = match args.private_key.clone() {
        None => generate_new_keys(),
        Some(private_key) => parse_private_key(private_key),
    };
    println!("=== Private key ===");
    println!("{}", keys.secret_key().unwrap().to_bech32().unwrap());
    println!("{}", keys.secret_key().unwrap().display_secret());
    println!("=== Public key ===");
    println!("{}", keys.public_key().to_bech32().unwrap());
    println!("{}", keys.public_key());

    // Read file
    let path = Path::new(&args.file_path);
    let file = File::open(path).unwrap();

    // Get filename and filesize
    let metadata = file.metadata().unwrap();
    let filesize = metadata.len(); // size of the file in bytes
    println!("{}", filesize);

    // Build tags
    let t_tag = Tag::hashtag(args.action.clone());

    let size_tag = Tag::custom(TagKind::Size, vec![filesize.to_string()]);

    let filehash: String = if args.fake_file_hash {
        get_random_sha256_hash()
    } else {
        compute_sha256_hash(path).unwrap()
    };
    let file_hash_tag = Tag::custom(
        TagKind::SingleLetter(SingleLetterTag::from_char('x').unwrap()),
        vec![filehash],
    );

    let timestamp = SystemTime::now()
        .add(core::time::Duration::new(3600, 0))
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expiration_tag = Tag::expiration(Timestamp::from(timestamp));

    let tags: Vec<Tag> = vec![t_tag, size_tag, expiration_tag, file_hash_tag];

    let kind = if args.invalid_kind {
        Kind::Custom(20202)
    } else {
        Kind::Custom(24242)
    };

    let event = nostr_sdk::EventBuilder::new(kind, args.description.clone(), tags)
        .to_event(&keys)
        .unwrap();
    let event_json = event.as_json();
    println!();
    println!("=== Event JSON: ===");
    println!("{:?}", event_json.clone());

    let token = generate_auth_token(event_json);
    println!();
    println!("Nostr {}", token);
}

fn generate_auth_token(event_json: String) -> String {
    general_purpose::STANDARD.encode(event_json)
}

fn parse_private_key(private_key: String) -> Keys {
    let secret_key = SecretKey::from_str(private_key.as_str()).unwrap();
    Keys::new(secret_key)
}

fn generate_new_keys() -> Keys {
    println!("Generating new keypair");
    Keys::generate()
}

fn compute_sha256_hash<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn get_random_sha256_hash() -> String {
    // Create a random byte array
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 32];
    rng.fill(&mut random_bytes);

    // Compute the SHA-256 hash of the random byte array
    let mut hasher = Sha256::new();
    hasher.update(random_bytes);
    let result = hasher.finalize();

    // Convert the hash result to a hexadecimal string
    hex::encode(result)
}
