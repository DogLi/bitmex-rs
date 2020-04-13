use super::topic::Topic;
use crate::consts::get_ws_url;
use crate::BitMEX;
use fehler::throws;
use hyper::Method;
use serde_derive::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "args")]
#[serde(rename_all = "camelCase")]
pub enum Command {
    Subscribe(Vec<Topic>),
    Unsubscribe(Vec<Topic>),
    #[serde(rename = "authKeyExpires")]
    Authenticate(String, i64, String), // ApiKey, Expires, Signature
    CancelAllAfter(i64),
    Ping,
}

impl Command {
    #[throws(failure::Error)]
    pub fn authenticate(bm: &BitMEX, expires: i64, is_testnet: bool) -> Command {
        let ws_url = get_ws_url(is_testnet);
        let (key, sig) = bm.signature(Method::GET, expires, &Url::parse(&ws_url)?, "")?;
        Command::Authenticate(key.to_string(), expires, sig)
    }
}
