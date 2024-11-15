use nostr::nips::{nip06::FromMnemonic, nip19::ToBech32};
use nostr::Keys;

use nostr_sdk::{Client, EventBuilder, Filter, Kind, RelayPoolNotification, SecretKey, Timestamp};
use serde_json::json;
use tokio::sync::mpsc;

use crate::channel::RelayCommand;
use crate::models::message::{self, UpdateMessage};
use crate::models::record::CreateRecord;
use crate::models::relays;
use crate::{config::CustomConfig, server::server::SharedState};

use nostr::prelude::*;

pub async fn send_message(
    client: &Client,
    server: &SharedState,
    message: &crate::models::message::Message,
) -> anyhow::Result<()> {
    let content: &str = "invite";
    let tags = [
        Tag::identifier("invite".to_string()),
        Tag::custom(
            TagKind::SingleLetter(SingleLetterTag::lowercase(Alphabet::F)),
            [
                message.from.as_str(),
                ""
            ]
        ),
        Tag::custom(
            TagKind::SingleLetter(SingleLetterTag::lowercase(Alphabet::T)),
            [
                message.to.as_str(),
                ""
            ]
        ),
        Tag::from_standardized(TagStandard::PublishedAt(Timestamp::from(message.created_at.and_utc().timestamp() as u64))),
    ];
    let builder = EventBuilder::long_form_text_note(content, tags);

    let r = client.send_event_builder(builder).await?;
    let id = r.id();
    tracing::debug!("send message to relay with event id {}", id);
    for success in r.success.iter() {
        let fin = final_message(&server, &message, &success, id).await;
        match fin {
            Ok(_) => {
                tracing::debug!("update message record success");
            },
            Err(_) => {
                tracing::error!("update message record error");
            },
        }
    }


    for failed in r.failed.iter() {
        tracing::error!("send message to relay {} with error {:#?}", failed.0, failed.1);
    }
 
 



    Ok(())
}


pub async fn final_message(
    server: &SharedState,
    message: &crate::models::message::Message,
    url: &Url,
    event: &EventId,
) -> anyhow::Result<()> {
    let server = server.0.write().await;
    let mut conn = server.pg.get()?;
    let update_message = UpdateMessage {
        status: Some("complete".to_string()),
        ..Default::default()
    };
    crate::models::message::Message::update(&mut conn, message.id.clone(), &update_message)?;

    let uid = uuid::Uuid::new_v4().to_string();

    let record = CreateRecord {
        id: uid,
        event_id: event.to_string(),
        relay: url.to_string(),
        message_id: message.id.to_owned(),
        status: "complete".to_string(),
        info: json!({
            "event": event.to_string()
        }),
        created_at: chrono::Local::now().naive_utc(),
    };

    crate::models::record::Record::create(&mut conn, &record)?;
    Ok(())
}


pub async fn sync_message(
    server: SharedState,
    mut message_rx: mpsc::Receiver<message::Message>,
    mut relays_rx: mpsc::Receiver<RelayCommand>,
    config: CustomConfig
) -> anyhow::Result<()> {
    tracing::debug!("mnemonic {:#?}", config.mnemonic);
    let keys = Keys::from_mnemonic(config.account.unwrap().mnemonic.clone().unwrap(), None).unwrap();

    let bech32_address = keys.public_key().to_bech32().unwrap();

    let client = Client::new(&keys);
    let relays = config.nostr.clone().map(|f|  {
        f.relays
    }).unwrap();
    for relay in relays.iter() {
        client.add_relay(relay).await?;
        tracing::info!("connect relay {:#?} with {:#?}", relay, bech32_address);

    }
    client.connect().await;
    let ac = client.clone();
    let r = tokio::spawn(async move {
        while let Some(relay) = relays_rx.recv().await {
            match relay {
                RelayCommand::Add(relays) => {
                    ac.add_relay(relays.url).await;
                },
                RelayCommand::Remove(relays) => {
                    ac.remove_relay(relays.url).await;
                },
            }
        }
    });
    while let Some(m) = message_rx.recv().await {
        // let event = EventBuilder::job_request(Kind::BookmarkSet, tags)
        // client.send_msg(msg);
        let result = send_message(&client, &server, &m).await;
        match result {
            Ok(_) => {
                tracing::debug!("send message success");
            },
            Err(_) => {
                tracing::error!("send message error");
            },
        }
    }
    r.await;
    Ok(())
}
