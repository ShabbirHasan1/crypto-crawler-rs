use crypto_market_type::MarketType;
use crypto_rest_client::fetch_l2_snapshot;
use serde_json::Value;
use std::collections::HashMap;
use test_case::test_case;

#[test_case(MarketType::Spot, "BTC_USDT")]
#[test_case(MarketType::InverseSwap, "BTC_USD")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
#[test_case(MarketType::LinearFuture, "BTC_USDT_20210924")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("gate", market_type, symbol).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    let asks = obj.get("asks").unwrap().as_array().unwrap();
    assert!(!asks.is_empty());

    let bids = obj.get("bids").unwrap().as_array().unwrap();
    assert!(!bids.is_empty());
}
