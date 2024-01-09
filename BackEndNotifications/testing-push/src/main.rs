use std::{sync::Arc, time::Duration};

use crate::nof::nof::NotificationData;
use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use get_data_from_lib::get_and_check_data;
use serde_json::json;
use tokio::{
    sync::broadcast::{self, Sender},
    time::sleep,
};
use web_push::SubscriptionInfo;
mod get_data_from_lib;
mod museum;
mod nof;
mod push_notif;

// Handler for /create-user
async fn handle_nof(input: Json<SubscriptionInfo>) -> impl IntoResponse {
    let response_body = json!(input.0);
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from(serde_json::to_string(&response_body).unwrap()))
        .unwrap()
}

async fn send_notficiaction(
    tx: Extension<Arc<broadcast::Sender<NotificationData>>>,
) -> impl IntoResponse {
    tx.0.send(NotificationData {
        title: "Hay This is a test".to_string(),
        body: "Do not be alarmed.".to_string(),
        icon: "/appicon.png".to_string(),
        badge: "/appicon.png".to_string(),
        link: "https://example.com".to_string(),
    })
    .unwrap();
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("Ok"))
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subs_info = std::fs::read_to_string("mussage.json")?;

    let (send_notif_manually, rec_notif_manually) = broadcast::channel::<NotificationData>(16);
    let (send_notif_normal, rec_notif_normal) = broadcast::channel::<Vec<NotificationData>>(16);

    let _ = tokio::join!(
        api_server(send_notif_manually),
        get_and_check_data(send_notif_normal),
        push_notif::handle_push_notifications(subs_info, rec_notif_manually, rec_notif_normal)
    );
    Ok(())
}

async fn api_server(tx: Sender<NotificationData>) {
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/create-user", post(handle_nof))
        .route("/send", get(send_notficiaction))
        .layer(Extension(Arc::new(tx)));
    println!("Running on http://localhost:3000");
    // Start Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
#[allow(dead_code)]
async fn dummy_musium(tx: Sender<NotificationData>) {
    loop {
        tx.send(NotificationData {
            title: "DUMMY".to_string(),
            body: "DUMMY".to_string(),
            icon: "DUMMY".to_string(),
            badge: "DUMMY".to_string(),
            link: "https://example.com".to_string(),
        })
        .unwrap();
        sleep(Duration::from_secs(2)).await;
    }
}
