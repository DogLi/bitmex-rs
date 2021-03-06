use bitmex::models::GetOrderBookL2Request;
use bitmex::BitMEX;
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_order_book_l2() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?, true);
    let fut = bm.request(GetOrderBookL2Request {
        symbol: "XBTUSD".into(),
        depth: Some(1),
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}
