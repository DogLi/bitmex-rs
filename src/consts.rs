use std::env::var;

use lazy_static::lazy_static;

// dotenv is a must run in every test otherwise the url will be mis-loaded
lazy_static! {
    pub static ref WS_URL: &'static str = {
        if var("BITMEX_TESTNET").is_ok() {
            "wss://www.bitmex.com/realtime"
        } else {
            log::warn!("Your are using BitMEX testnet Websocket");
            "wss://testnet.bitmex.com/realtime"
        }
    };
    pub static ref REST_URL: &'static str = {
        if var("BITMEX_TESTNET").is_ok() {
            "https://www.bitmex.com/api/v1"
        } else {
            log::warn!("Your are using BitMEX testnet Restful API");
            "https://testnet.bitmex.com/api/v1"
        }
    };
}
