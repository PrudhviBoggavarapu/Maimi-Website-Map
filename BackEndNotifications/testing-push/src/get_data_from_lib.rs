#![allow(non_snake_case)]
use std::time::Duration;

use crate::museum::museum::{get_museums, Museum};
use crate::NotificationData;
use chrono::Utc;
use futures::{stream, StreamExt};
use pocketbase_sdk::client::Client as DataClient;
use rayon::prelude::*;
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use tokio;
use tokio::time::sleep;
#[derive(Clone, Serialize, Deserialize, Debug, Default)]

pub struct DatabaseOutput {
    pub time: String,
    pub available: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]

pub struct SendToPushNotification {
    pub available: Vec<LocationItem>,
    pub mus: Museum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationItem {
    pub code: String,
    pub label: String,
    pub is_patron_home: Option<bool>,
    pub item_location_label: Option<String>,
    pub call_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreData {
    pub available: Vec<LocationItem>,
    pub unavailable: Vec<LocationItem>,
}

pub struct MuseumsResponce {
    pub musium: Museum,
    pub data: StoreData,
}
#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let pocketbase_client = DataClient::new("http://localhost:8090")
        .auth_with_password(
            "users",
            "NormalUserForPocketbase",
            "X@freHk*!84oyMdb6V93ubZf6bRHAoShBsrnRaRcgr#uf*#fNmutzciMRoJF!%JteJ5V@FLd",
        )
        .expect("Could not log into database");

    let gotten_data = get_data_from_miami(10).await;
    let output = check_data_and_push_to_database(gotten_data, &pocketbase_client).await;
    println!("{:?}", output);
}

pub async fn get_data_from_miami(CONCURRENT_REQUESTS: usize) -> Vec<MuseumsResponce> {
    let client = Client::new();
    let MusiumVec = get_museums();
    let bodies = stream::iter(MusiumVec)
        .map(|mut MusObj| {
            let client = &client;
            async move {
                let resp = client
                    .get(MusObj.into_url())
                    .header(
                        "User-Agent",
                        "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0",
                    )
                    .header("Accept", "application/json, text/plain, */*")
                    .header("Accept-Language", "en-US,en;q=0.5")
                    .header("Accept-Encoding", "gzip, deflate, br")
                    .header("api-version", "2")
                    .header("iii-customer-domain", "mdpls.na.iiivega.com")
                    .header("iii-host-domain", "mdpls.na.iiivega.com")
                    .header("Anonymous-User-Id", "eda0ad8a-f8d4-4b0a-a98f-643fed73d916")
                    .header("Content-Type", "application/json")
                    .header("Origin", "https://mdpls.na.iiivega.com")
                    .header("Connection", "keep-alive")
                    .header("Referer", "https://mdpls.na.iiivega.com/")
                    .header("Sec-Fetch-Dest", "empty")
                    .header("Sec-Fetch-Mode", "cors")
                    .header("Sec-Fetch-Site", "same-site")
                    .header("Pragma", "no-cache")
                    .header("Cache-Control", "no-cache")
                    .header("TE", "trailers")
                    .send()
                    .await
                    .expect("Could not get api");
                MusObj.id = MusObj.id.replace("-", "_");
                MuseumsResponce {
                    musium: MusObj,
                    data: resp.json::<StoreData>().await.expect("could parse api"),
                }
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);
    let results: Vec<MuseumsResponce> = bodies.collect().await;
    results
}

pub async fn check_data_and_push_to_database(
    results: Vec<MuseumsResponce>,
    pocketbase_client: &pocketbase_sdk::client::Client<pocketbase_sdk::client::Auth>,
) -> Vec<std::option::Option<SendToPushNotification>> {
    let evetything_sent: Vec<_> = results
        .into_par_iter()
        .map(|x| {
            let list_available_for_museum_item = x.data.available.into_iter().collect::<Vec<_>>();

            let list_available_for_museum = list_available_for_museum_item
                .iter()
                .map(|x| x.label.clone())
                .collect::<Vec<_>>()
                .join("|");

            let getOneObject = &pocketbase_client
                .records(Box::leak(x.musium.id.clone().into_boxed_str()))
                .list()
                .sort("-created")
                .call::<DatabaseOutput>()
                .expect("Could not get data")
                .items[0];
            match getOneObject.available == list_available_for_museum {
                true => None,
                false => {
                    println!("Pushed To Database {}", x.musium.title);
                    pocketbase_client
                        .records(Box::leak(x.musium.id.clone().into_boxed_str()))
                        .create(DatabaseOutput {
                            time: Utc::now().format("%Y-%m-%d %H:%M:%S%.3fZ").to_string(),
                            available: list_available_for_museum.clone(),
                        })
                        .call()
                        .expect("Could not send to database");
                    Some(SendToPushNotification {
                        available: list_available_for_museum_item,
                        mus: x.musium,
                    })
                }
            }
        })
        .collect();
    evetything_sent
}

pub async fn get_and_check_data(tx: tokio::sync::broadcast::Sender<Vec<NotificationData>>) {
    let pocketbase_client = DataClient::new("http://localhost:8090")
        .auth_with_password(
            "users",
            "NormalUserForPocketbase",
            "X@freHk*!84oyMdb6V93ubZf6bRHAoShBsrnRaRcgr#uf*#fNmutzciMRoJF!%JteJ5V@FLd",
        )
        .expect("Could not log into database");

    loop {
        let gotten_data = get_data_from_miami(10).await;
        let output = check_data_and_push_to_database(gotten_data, &pocketbase_client).await;

        let shouldPrint = output.iter().flatten().count();
        match shouldPrint {
            0 => {}
            _ => {
                println!("{:?}", shouldPrint);
                cleaned_data_to_send(output, tx.clone()).await;
            }
        }
        sleep(Duration::from_secs(10)).await
    }
}

pub async fn cleaned_data_to_send(
    input: Vec<Option<SendToPushNotification>>,
    tx: tokio::sync::broadcast::Sender<Vec<NotificationData>>,
) {
    let output = input
        .into_iter()
        .flatten()
        .map(|x| {
            x.available.into_iter().map(move |y| NotificationData {
                title: format!("{} Has Pass {}", y.label, x.mus.title),
                body: format!("{} Has Pass {}", y.label, x.mus.title),
                icon: "/appicon.png".to_string(),
                badge: "/appicon.png".to_string(),
                link: "/appicon.png".to_string(),
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    if output.len() != 0 {
        tx.send(output).unwrap();
    }
}
