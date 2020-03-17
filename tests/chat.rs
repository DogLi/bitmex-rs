use bitmex::models::{
    GetChatChannelsRequest, GetChatConnectedRequest, GetChatRequest, PostChatRequest,
};
use bitmex::BitMEX;
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_chat() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.request(GetChatRequest {
        count: Some(1),
        channel_id: Some(1.),
        ..Default::default()
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

#[test]
#[ignore] // My test account was banned from chatting on testnet
fn post_chat() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?, true);
    let fut = bm.request(PostChatRequest {
        message: "Hey there".into(),
        channel_id: Some(1.),
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

#[test]
fn get_chat_channels() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.request(GetChatChannelsRequest {});

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

#[test]
fn get_chat_connected() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::new();
    let fut = bm.request(GetChatConnectedRequest {});

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}
