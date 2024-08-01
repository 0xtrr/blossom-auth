use std::ops::Add;
use std::path::Path;
use std::time::SystemTime;

use clap::Args;
use nostr_sdk::{Event, EventBuilder, Keys, Kind, SingleLetterTag, Tag, TagKind, Timestamp};

use crate::{compute_sha256_hash, get_random_sha256_hash};

#[derive(Args)]
pub struct UploadArgs {
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

pub fn generate_upload_event(keys: &Keys, args: UploadArgs) -> Event {
    // Read file
    let path = Path::new(&args.file_path);
    // Build tags
    let t_tag = Tag::hashtag("upload");

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

    let tags: Vec<Tag> = vec![t_tag, expiration_tag, file_hash_tag];

    let kind = if args.invalid_kind {
        Kind::Custom(20202)
    } else {
        Kind::Custom(24242)
    };

    EventBuilder::new(kind, args.description, tags)
        .to_event(keys)
        .unwrap()
}
