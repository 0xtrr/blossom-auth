use std::borrow::Cow;
use std::ops::Add;
use std::process::exit;
use std::time::SystemTime;

use clap::Args;
use nostr_sdk::{Event, EventBuilder, Keys, Kind, SingleLetterTag, Tag, TagKind, Timestamp};

use crate::get_random_sha256_hash;

#[derive(Args)]
pub struct GetArgs {
    /// Description (put into content field in event)
    #[arg(short, long)]
    description: String,
    /// Path to file that is to be uploaded
    #[arg(short, long)]
    server_url: Option<String>,
    /// SHA256 hash of the file that should be fetched
    #[arg(short, long)]
    file_hash: Option<String>,
    /// Puts a random generated sha256 hash in the x tag of the event
    #[arg(long, default_value = "false")]
    fake_file_hash: bool,
    /// Sets incorrect kind in the authorization event
    #[arg(long, default_value = "false")]
    invalid_kind: bool,
}

pub fn generate_get_event(keys: &Keys, args: GetArgs) -> Event {
    // Define kind
    let kind = if args.invalid_kind {
        Kind::Custom(20202)
    } else {
        Kind::Custom(24242)
    };

    // Build tags
    let t_tag = Tag::hashtag("get");

    let file_sha_or_server_tag = if let Some(file_hash) = args.file_hash {
        let filehash: String = if args.fake_file_hash {
            get_random_sha256_hash()
        } else {
            file_hash
        };
        let file_hash_tag = Tag::custom(
            TagKind::SingleLetter(SingleLetterTag::from_char('x').unwrap()),
            vec![filehash],
        );
        file_hash_tag
    } else if let Some(server_url) = args.server_url {
        // TODO Add URL parsing & validation
        let server_tag = Tag::custom(TagKind::Custom(Cow::from("server")), vec![server_url]);
        server_tag
    } else {
        eprintln!("File hash or server URL is required but was not provided");
        exit(1)
    };

    let timestamp = SystemTime::now()
        .add(core::time::Duration::new(3600, 0))
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expiration_tag = Tag::expiration(Timestamp::from(timestamp));

    let tags: Vec<Tag> = vec![t_tag, expiration_tag, file_sha_or_server_tag];

    EventBuilder::new(kind, args.description, tags)
        .to_event(keys)
        .unwrap()
}
