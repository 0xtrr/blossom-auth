use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use nostr_sdk::base64::engine::general_purpose;
use nostr_sdk::base64::Engine;
use nostr_sdk::{Event, JsonUtil, Keys, SecretKey, ToBech32};
use rand::Rng;
use sha2::{Digest, Sha256};

mod sub_commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Nostr private key (hex)
    #[arg(short, long)]
    private_key: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate upload auth event
    Upload(sub_commands::upload::UploadArgs),
    /// Generate list auth event
    List(sub_commands::list::ListArgs),
    /// Generate get auth event
    Get(sub_commands::get::GetArgs),
    /// Generate delete auth event
    Delete(sub_commands::delete::DeleteArgs),
    /// Generate mirror auth event
    Mirror(sub_commands::mirror::MirrorArgs),
}

fn main() {
    let cli = Cli::parse();

    let keys: Keys = match cli.private_key.clone() {
        None => generate_new_keys(),
        Some(private_key) => parse_private_key(private_key),
    };
    println!("=== Private key ===");
    println!("{}", keys.secret_key().unwrap().to_bech32().unwrap());
    println!("{}", keys.secret_key().unwrap().display_secret());
    println!("=== Public key ===");
    println!("{}", keys.public_key().to_bech32().unwrap());
    println!("{}", keys.public_key());

    let event: Event = match cli.command {
        Commands::Upload(sub_command_args) => {
            sub_commands::upload::generate_upload_event(&keys, sub_command_args)
        }
        Commands::List(sub_command_args) => {
            sub_commands::list::generate_list_event(&keys, sub_command_args)
        }
        Commands::Get(sub_command_args) => {
            sub_commands::get::generate_get_event(&keys, sub_command_args)
        }
        Commands::Delete(sub_command_args) => {
            sub_commands::delete::generate_delete_event(&keys, sub_command_args)
        }
        Commands::Mirror(sub_command_args) => {
            sub_commands::mirror::generate_mirror_event(&keys, sub_command_args)
        }
    };

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

fn compute_sha256_hash(path: &Path) -> io::Result<String> {
    let mut file = File::open(path).unwrap_or_else(|e| {
        panic!("Failed to open file {}: {}", path.display(), e);
    });
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
