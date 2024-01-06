use musiums::musiums::get_museums;
use pocketbase_sdk::client::Client as DataClient;
use serde::{Deserialize, Serialize};

mod musiums;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Lib {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]

pub struct DatabaseOutput {
    pub time: String,
    pub available: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]

pub struct DatabaseOutputClean {
    pub time: String,
    pub available: Vec<Lib>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let authenticated_client = DataClient::new("http://localhost:8090").auth_with_password(
        "users",
        "NormalUserForPocketbase",
        "X@freHk*!84oyMdb6V93ubZf6bRHAoShBsrnRaRcgr#uf*#fNmutzciMRoJF!%JteJ5V@FLd",
    )?;

    let _musium = get_museums();

    let output2 = authenticated_client
        .records("8c8127e9_7ff5_4bb1_8127_e97ff5abb1ef")
        .list()
        .call::<DatabaseOutput>()?;
    let clean_database: Vec<_> = output2.items.into_iter().collect();
    println!("{:#?}", clean_database);
    println!("{}", clean_database.len());
    Ok(())
}
