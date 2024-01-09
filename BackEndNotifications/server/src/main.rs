#![allow(non_snake_case)]
use chrono::Utc;
use pocketbase_sdk::client::Client as DataClient;
pub mod musiums;

use futures::{stream, StreamExt};
use musiums::musiums::{get_museums, Museum};
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use tokio;
#[derive(Clone, Serialize, Deserialize, Debug, Default)]

pub struct DatabaseOutput {
    pub time: String,
    pub available: String,
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

const CONCURRENT_REQUESTS: usize = 2;

#[tokio::main]
async fn main() {


    
}



sudo fn oofda(){

    let client = Client::new();
    let authenticated_client = DataClient::new("http://localhost:8090")
        .auth_with_password(
            "users",
            "NormalUserForPocketbase",
            "X@freHk*!84oyMdb6V93ubZf6bRHAoShBsrnRaRcgr#uf*#fNmutzciMRoJF!%JteJ5V@FLd",
        )
        .expect("Could not log into database");

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
    let evetything_sent: Vec<_> = results
        .into_iter()
        .map(|x| {
            let string_to_push = x
                .data
                .available
                .into_iter()
                .map(|x| x.label)
                .collect::<Vec<_>>()
                .join("|");

            let getOneObject = &authenticated_client
                .records(Box::leak(x.musium.id.clone().into_boxed_str()))
                .list()
                .sort("-created")
                .call::<DatabaseOutput>()
                .expect("Could not get data")
                .items[0];
            match getOneObject.available == string_to_push {
                true => {
                    println!("They are the same, skipping");
                }
                false => {
                    println!("Pushed To Database {}", x.musium.title);
                    authenticated_client
                        .records(Box::leak(x.musium.id.into_boxed_str()))
                        .create(DatabaseOutput {
                            time: Utc::now().format("%Y-%m-%d %H:%M:%S%.3fZ").to_string(),
                            available: string_to_push,
                        })
                        .call()
                        .expect("Could not send to database");
                }
            }
        })
        .collect();
    println!("{:?}", evetything_sent.len());
}
