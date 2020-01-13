pub fn get_ws_url(is_testnet: bool) -> String {
    if is_testnet {
        log::warn!("Your are using BitMEX testnet Websocket");
        "wss://testnet.bitmex.com/realtime".into()
    } else {
        "wss://www.bitmex.com/realtime".into()
    }
}

pub fn get_rest_url(is_testnet: bool) -> String {
    if is_testnet {
        log::warn!("Your are using BitMEX testnet Restful API");
        "https://testnet.bitmex.com/api/v1".into()
    } else {
        "https://www.bitmex.com/api/v1".into()
    }
}
