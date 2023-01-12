mod ap;

use std::env;
use std::time::Duration;
use twilight_http::Client;
use twilight_model::channel::message::{Embed, embed::EmbedField};
use twilight_model::id::{marker::ChannelMarker, Id};

use tokio;

async fn announce_new_firmware(client: &Client, fw_info: ap::Firmware, channel_id: Id<ChannelMarker>) -> Result<twilight_http::Response<twilight_model::channel::Message>, twilight_http::Error> {
    let embed = Embed {
        author: None,
        color: Some(123),
        description: None,
        fields: vec![
            EmbedField { name: "Id".to_owned(), value: fw_info.id, inline: false },
            EmbedField { name: "Version".to_owned(), value: fw_info.version, inline: false },
            EmbedField { name: "Publish date".to_owned(), value: fw_info.published_at, inline: false },
            EmbedField { name: "Filename".to_owned(), value: fw_info.details.filename, inline: false },
            EmbedField { name: "Filesize".to_owned(), value: fw_info.details.filesize, inline: false },
            EmbedField { name: "MD5".to_owned(), value: fw_info.details.md5_hash, inline: false },
            EmbedField { name: "URL".to_owned(), value: fw_info.details.url, inline: false },
        ],
        footer: None,
        image: None,
        kind: "rich".to_owned(),
        provider: None,
        thumbnail: None,
        timestamp: None,
        title: Some("New AP firmware available".to_owned()),
        url: Some(ap::BASE_URL.to_owned()),
        video: None,
    };

    // We can use ChannelId directly to send a message to a specific channel; in this case, the
    // message would be sent to the #testing channel on the discord server.
    client
        .create_message(channel_id)
        .embeds(&[
            embed
        ])
        .unwrap()
        .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");
    let channel_id: Id<ChannelMarker> = {
        let id = env::var("CHANNEL_ID")
        .expect("Expected a channel id in the environment")
        .parse::<u64>()
        .expect("Invalid channel id");

        Id::new(id)
    };

    // Fetch initial firmware info
    let info = ap::fetch_firmware_info()
        .expect("Failed to gather initial firmware info!");
    let mut current_firmware_version = info.props.page_props.firmware.version;

    let client = Client::new(token);
    let me = client.current_user().await?.model().await?;
    println!("Current user: {}#{}", me.name, me.discriminator);

    loop {
        match ap::fetch_firmware_info() {
            Ok(info) => {
                let fw_node = info.props.page_props.firmware;
                let latest_fw_version = fw_node.version.clone();
                if latest_fw_version != current_firmware_version {
                    println!("New firmware info from HTTP: {:?}", fw_node);
                    match announce_new_firmware(&client, fw_node, channel_id).await {
                        Ok(_) => {
                            println!("New firmware announced to channel!");
                            println!("Setting new known fw to: {}", latest_fw_version);
                            current_firmware_version = latest_fw_version;
                        },
                        Err(err) => {
                            eprintln!("Failed announcing firmware to channel, err={}", err);
                        }
                    }
                }
            },
            Err(err) => {
                eprintln!("Failed to fetch firmware version, err={:?}", err);
            }
        }
        tokio::time::sleep(Duration::from_secs(120)).await;
    }
}