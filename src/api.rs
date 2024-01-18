use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;

pub async fn call_api() -> Vec<Item> {
    let mut items: Vec<Item> = vec![];

    let client = reqwest::Client::new();

    let mut body = HashMap::new();
    body.insert("rid", "32423542");
    body.insert("token", "");
    body.insert("shares", "VIX,ITA,HAG");

    let response = client
        .post("https://mktapi1.mbs.com.vn/pbResfulMarkets/securities/list")
        .json(&body)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => match response.text().await {
            Ok(txt) => {
                let res = typed_example(&txt);
                match res {
                    Ok(data) => {
                        items = data.data;
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        },
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        }
    };
    items
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    msgid: String,
    status: String,
    data: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub sym: String,
    pub mlc: String,
    pub fp: String,
    pub rp: String,
    pub cp: String,
    #[serde(rename = "openP")]
    pub open_p: String,
    #[serde(rename = "closedP")]
    pub closed_p: String,
    #[serde(rename = "openC")]
    pub open_c: String,
    #[serde(rename = "closeC")]
    pub close_c: String,
    pub bbc1: String,
    pub bac2: String,
    pub bbc2: String,
    pub bac3: String,
    pub bac1: String,
    pub fsr: String,
    #[serde(rename = "highestC")]
    pub highest_c: String,
    pub tvtraded: String,
    pub bbv2: String,
    pub bav3: String,
    pub bbv3: String,
    #[serde(rename = "avgP")]
    pub avg_p: String,
    pub bav1: String,
    pub fcr: f32,
    pub bbv1: String,
    pub bav2: String,
    #[serde(rename = "lowestC")]
    pub lowest_c: String,
    pub mchv: String,
    #[serde(rename = "lowestP")]
    pub lowest_p: String,
    pub trdses: String,
    #[serde(rename = "avgC")]
    pub avg_c: String,
    pub bbc3: String,
    pub side: String,
    pub mp: String,
    pub tstraded: String,
    pub ftr: f32,
    pub trss: String,
    pub msgid: String,
    pub mv: String,
    pub bbp2: String,
    pub bap3: String,
    pub mchp: f32,
    pub bbp3: String,
    pub bap1: String,
    pub bbp1: String,
    pub bap2: String,
    pub fbr: String,
    #[serde(rename = "highestP")]
    pub highest_p: String,
    pub trbs: String,
}

fn typed_example(data: &str) -> serde_json::Result<Data> {
    let p: Data = from_str(data)?;
    Ok(p)
}
