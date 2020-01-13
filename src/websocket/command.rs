use super::topic::Topic;
use crate::consts::get_ws_url;
use crate::BitMEX;
use failure::Fallible;
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
    pub fn authenticate(bm: &BitMEX, expires: i64) -> Fallible<Command> {
        let (key, sig) = bm.signature(Method::GET, expires, &Url::parse(&get_ws_url(bm.is_testnet))?, "")?;
        Ok(Command::Authenticate(key.to_string(), expires, sig))
    }
}
