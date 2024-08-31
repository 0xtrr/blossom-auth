use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use nostr_sdk::base64::engine::general_purpose;
use nostr_sdk::base64::Engine;
use nostr_sdk::{serde_json, Event, JsonUtil, Keys, SecretKey, Tag, ToBech32};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

mod sub_commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Nostr private key (hex)
    #[arg(short, long)]
    private_key: Option<String>,
    /// Provides a fake signature to test signature verification
    #[arg(short, long, default_value = "false")]
    fake_sig: bool,
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

    let event_json = if cli.fake_sig {
        let mut custom_event = CustomEvent::from(event);
        custom_event = CustomEvent::fake_event_signature(custom_event);
        custom_event.as_json().to_string()
    } else {
        event.as_json().to_string()
    };

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

/// Custom event type used for faking signature
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEvent {
    /// EventId
    pub id: String,
    /// Author
    pub pubkey: String,
    /// Timestamp (seconds)
    pub created_at: u64,
    /// Kind
    pub kind: u64,
    /// Vector of [`Tag`]
    pub tags: Vec<Tag>,
    /// Content
    pub content: String,
    /// Signature
    pub sig: String,
}

impl From<Event> for CustomEvent {
    fn from(event: Event) -> Self {
        CustomEvent {
            id: event.id.to_hex(),
            pubkey: event.pubkey.to_hex(),
            kind: event.kind.as_u64(),
            content: event.content.clone(),
            created_at: event.created_at.as_u64(),
            tags: event.tags.clone(),
            sig: event.sig.to_string(),
        }
    }
}

impl CustomEvent {
    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Sets a static invalid signature
    pub fn fake_event_signature(event: Self) -> Self {
        CustomEvent {
            sig: String::from("16a9b833a060b06cf45578be129e5cd2d1b2c5f0ff3c28e97152c755832c24b3a5099ac8450ffaec2e7c337af41b4f49def5d0024c2630413b5992b41f6c5bf6"),
            ..event
        }
    }
}
