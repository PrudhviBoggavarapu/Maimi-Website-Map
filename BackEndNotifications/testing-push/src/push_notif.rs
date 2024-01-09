use crate::nof::nof::NotificationData;
use std::fs::File;
use tokio::sync::broadcast::Receiver;
use web_push::*;
#[tokio::main]
#[allow(dead_code, unused_variables)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let subs_info = std::fs::read_to_string("mussage.json")?;
    let payload = std::fs::read_to_string("payload.json")?;
    // handle_push_notifications(subs_info, payload).await?;
    Ok(())
}

pub async fn handle_push_notifications(
    subscription_info: String,
    mut web_server_force: Receiver<NotificationData>,
    mut change_in_musium: Receiver<Vec<NotificationData>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = IsahcWebPushClient::new()?;

    loop {
        let data_to_push = tokio::select! {
            result = web_server_force.recv() => {
                result.expect("Channel closed")
            },
            result = change_in_musium.recv() => {
                result.expect("Channel closed")[0].clone()
            },
        };

        let clean_push_data = serde_json::to_string(&data_to_push)?;
        let file = File::open("private_key.pem")?;

        let subscription_info: SubscriptionInfo = serde_json::from_str(&subscription_info)?;
        let mut builder = WebPushMessageBuilder::new(&subscription_info);
        let sig_builder = VapidSignatureBuilder::from_pem(file, &subscription_info)?;

        let signature = sig_builder.build()?;

        builder.set_vapid_signature(signature);
        builder.set_payload(ContentEncoding::Aes128Gcm, clean_push_data.as_bytes());

        let response = client.send(builder.build()?).await?;
        println!("Sent: {:?}", response);
    }
}
