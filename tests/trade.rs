use bitmex::models::BinSize;
use bitmex::models::{GetTradeBucketedRequest, GetTradeRequest};
use bitmex::BitMEX;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_trade() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?, true);

    let _ = rt.block_on(bm.request(GetTradeRequest {
        ..Default::default()
    }))?;
    Ok(())
}

#[test]
fn get_trade_bucketed() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;
    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?, true);

    let _ = rt.block_on(bm.request(GetTradeBucketedRequest {
        partial: Some(false),
        bin_size: Some(BinSize::D1),
        count: Some(10),
        ..Default::default()
    }))?;

    Ok(())
}
