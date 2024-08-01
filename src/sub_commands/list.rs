use std::ops::Add;
use std::time::SystemTime;

use clap::Args;
use nostr_sdk::{Event, EventBuilder, Keys, Kind, Tag, Timestamp};

#[derive(Args)]
pub struct ListArgs {
    /// Description (put into content field in event)
    #[arg(short, long)]
    description: String,
    /// Sets incorrect kind in the authorization event
    #[arg(long, default_value = "false")]
    invalid_kind: bool,
}

pub fn generate_list_event(keys: &Keys, args: ListArgs) -> Event {
    // Define kind
    let kind = if args.invalid_kind {
        Kind::Custom(20202)
    } else {
        Kind::Custom(24242)
    };

    // Build tags
    let t_tag = Tag::hashtag("list");

    let timestamp = SystemTime::now()
        .add(core::time::Duration::new(3600, 0))
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expiration_tag = Tag::expiration(Timestamp::from(timestamp));

    let tags: Vec<Tag> = vec![t_tag, expiration_tag];

    EventBuilder::new(kind, args.description, tags)
        .to_event(keys)
        .unwrap()
}
