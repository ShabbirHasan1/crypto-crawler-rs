use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Market type.
///
/// * In spot market, cryptocurrencies are traded for immediate delivery, see https://en.wikipedia.org/wiki/Spot_market.
/// * In futures market, delivery is set at a specified time in the future, see https://en.wikipedia.org/wiki/Futures_exchange.
/// * Swap market is a variant of futures market with no expiry date.
///
/// ## Margin
///
/// A market can have margin enabled or disabled.
///
/// * All contract markets are margin enabled, including future, swap and option.
/// * Most spot markets don't have margin enabled, only a few exchanges have spot market with margin enabled.
///
/// ## Linear VS. Inverse
///
/// A market can be inverse or linear.

/// * Linear means USDT-margined, i.e., you can use USDT as collateral
/// * Inverse means coin-margined, i.e., you can use BTC as collateral.
/// * Spot market is always linear.
///
/// **Margin and Inverse are orthogonal.**
#[derive(Copy, Clone, Serialize, Deserialize, Display, Debug, EnumString, PartialEq)]
pub enum MarketType {
    Spot,
    Future,
    Swap,
    Option,
}
