// #![allow(non_snake_case)]

// use reqwest;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct ApiResponse {
//     totalPages: u32,
//     page: u32,
//     totalResults: u32,
//     data: Vec<DataEntry>,
//     language: Option<String>,
//     seriesTitle: Option<String>,
//     seriesTitleSearch: Option<String>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct DataEntry {
//     id: String,
//     customerId: String,
//     entityType: String,
//     coverUrl: CoverUrl,
//     title: String,
//     materialTabs: Vec<MaterialTab>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct CoverUrl {
//     small: String,
//     medium: String,
//     large: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct MaterialTab {
//     name: String,
//     #[serde(rename = "type")]
//     type_: String,
//     callNumber: Option<String>,
//     itemLibrary: Option<String>,
//     availability: Availability,
//     recordIds: Vec<String>,
//     materialTypes: Vec<String>,
//     locations: Vec<Location>,
//     hasMoreLocations: bool,
//     locationsTotalResults: u32,
//     editions: Vec<Edition>,
//     checkAvailabilityFromOtherSources: bool,
//     multimediaLinks: Vec<MultimediaLink>,
//     itemEditionRecordId: Option<String>,
//     translationKey: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Availability {
//     isGetItSuppressed: bool,
//     status: Status,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Status {
//     general: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Location {
//     code: String,
//     label: String,
//     availabilityStatus: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Edition {
//     id: String,
//     availabilityStatus: String,
//     callNumber: Option<String>,
//     hasItems: bool,
//     recordId: String,
//     publicationDate: String,
//     #[serde(rename = "type")]
//     type_: String,
//     isGetItSuppressed: bool,
// }

// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// struct MultimediaLink {
//     url: String,
//     label: String,
// }

// #[derive(Serialize, Deserialize, Debug, Default)]
// struct Museum {
//     id: String,
//     title: String,
//     url: String,
// }

// fn main() -> Result<(), reqwest::Error> {
//     let client = reqwest::blocking::Client::new();
//     let url = "https://na.iiivega.com/api/search-result/search/format-groups";

//     let response = client
//         .post(url)
//         .header(
//             "User-Agent",
//             "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0",
//         )
//         .header("Accept", "application/json, text/plain, */*")
//         .header("Accept-Language", "en-US,en;q=0.5")
//         .header("Accept-Encoding", "gzip, deflate, br")
//         .header("api-version", "2")
//         .header("iii-customer-domain", "mdpls.na.iiivega.com")
//         .header("iii-host-domain", "mdpls.na.iiivega.com")
//         .header("Anonymous-User-Id", "eda0ad8a-f8d4-4b0a-a98f-643fed73d916")
//         .header("Content-Type", "application/json")
//         .header("Origin", "https://mdpls.na.iiivega.com")
//         .header("Connection", "keep-alive")
//         .header("Referer", "https://mdpls.na.iiivega.com/")
//         .header("Sec-Fetch-Dest", "empty")
//         .header("Sec-Fetch-Mode", "cors")
//         .header("Sec-Fetch-Site", "same-site")
//         .header("Pragma", "no-cache")
//         .header("Cache-Control", "no-cache")
//         .header("TE", "trailers")
//         .json(&serde_json::json!({
//             "searchText": "\"Museum pass program\"",
//             "sorting": "relevance",
//             "sortOrder": "asc",
//             "searchType": "series",
//             "pageNum": 0,
//             "pageSize": 40,
//             "resourceType": "FormatGroup"
//         }))
//         .send()?;

//     if response.status().is_success() {
//         let response_data: ApiResponse = response.json()?;

//         let x = response_data
//             .data
//             .into_iter()
//             .map(|x| x)
//             .map(|mut x| {
//                 x.title = x.title.split_once('-').unwrap().0.trim().to_owned();
//                 x
//             })
//             .map(|x| {
//                 let clean_url: String = x
//                     .materialTabs
//                     .first()
//                     .unwrap()
//                     .multimediaLinks
//                     .clone()
//                     .into_iter()
//                     .find(|x| !x.label.contains("Cover image"))
//                     .unwrap_or_default()
//                     .url;

//                 Museum {
//                     id: x.id,
//                     title: x.title,
//                     url: clean_url,
//                 }
//             })
//             .collect::<Vec<_>>();

//         println!("Response Data: {:#?}", x);
//     } else {
//         println!("Failed to get a successful response");
//     }

//     Ok(())
// }
fn main() {}
