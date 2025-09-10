/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 9/9/25
******************************************************************************/
use crate::prelude::{Instrument, TickerData};
use crate::{impl_json_debug_pretty, impl_json_display};
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Combined option instrument data with ticker information
#[derive(Clone, Serialize, Deserialize)]
pub struct OptionInstrument {
    /// The instrument details
    pub instrument: Instrument,
    /// Real-time ticker data for the option
    pub ticker: TickerData,
}

/// A pair of option instruments representing both call and put options for the same underlying asset
///
/// This structure groups together the call and put options for a specific underlying asset,
/// allowing for easy access to both sides of an option strategy. Both options are optional,
/// meaning you can have just a call, just a put, or both.
///
#[derive(Clone, Serialize, Deserialize)]
pub struct OptionInstrumentPair {
    /// Call option instrument data, if available
    call: Option<OptionInstrument>,
    /// Put option instrument data, if available  
    put: Option<OptionInstrument>,
}

/// Spread information for bid/ask prices
#[derive(Clone, Serialize, Deserialize)]
pub struct Spread {
    /// Best bid price
    bid: Option<f64>,
    /// Best ask price
    ask: Option<f64>,
    /// Mid price (average of bid and ask)
    mid: Option<f64>,
}

/// Basic Greeks values for option pricing
#[derive(Clone, Serialize, Deserialize)]
pub struct BasicGreeks {
    /// Delta value for call option
    delta_call: Option<f64>,
    /// Delta value for put option
    delta_put: Option<f64>,
    /// Gamma value (rate of change of delta)
    gamma: Option<f64>,
}

/// Comprehensive option data structure containing all relevant pricing and risk information
#[derive(Clone, Serialize, Deserialize)]
pub struct BasicOptionData {
    /// Strike price of the option
    pub strike_price: f64,
    /// Best bid price for call option
    pub call_bid: Option<f64>,
    /// Best ask price for call option
    pub call_ask: Option<f64>,
    /// Best bid price for put option
    pub put_bid: Option<f64>,
    /// Best ask price for put option
    pub put_ask: Option<f64>,
    /// Implied volatility for call and put options (call_iv, put_iv)
    pub implied_volatility: (Option<f64>, Option<f64>),
    /// Delta value for call option
    pub delta_call: Option<f64>,
    /// Delta value for put option
    pub delta_put: Option<f64>,
    /// Gamma value (rate of change of delta)
    pub gamma: Option<f64>,
    /// Total trading volume
    pub volume: f64,
    /// Total open interest
    pub open_interest: f64,
    /// Option expiration date
    pub expiration_date: Option<DateTime<Utc>>,
    /// Current price of the underlying asset
    pub underlying_price: Option<f64>,
    /// Risk-free interest rate
    pub risk_free_rate: f64,
    /// Additional fields as JSON value
    pub extra_fields: Option<Value>,
}

#[allow(dead_code)]
impl OptionInstrumentPair {
    fn expiration(&self) -> Option<DateTime<Utc>> {
        let expiration_timestamp = match self.instrument() {
            Some(i) => i.expiration_timestamp,
            None => return None,
        };

        if let Some(expiration_timestamp) = expiration_timestamp {
            Utc.timestamp_millis_opt(expiration_timestamp).single()
        } else {
            None
        }
    }
    fn instrument(&self) -> Option<Instrument> {
        self.call
            .as_ref()
            .map(|i| i.instrument.clone())
            .or_else(|| self.put.as_ref().map(|i| i.instrument.clone()))
    }
    fn ticker(&self) -> Option<TickerData> {
        self.call
            .as_ref()
            .map(|i| i.ticker.clone())
            .or_else(|| self.put.as_ref().map(|i| i.ticker.clone()))
    }

    fn volume(&self) -> f64 {
        let mut volume: f64 = 0.0;
        if let Some(call) = &self.call {
            volume += call.ticker.stats.volume
        }
        if let Some(put) = &self.put {
            volume += put.ticker.stats.volume
        }
        volume
    }

    fn open_interest(&self) -> f64 {
        let mut open_interest: f64 = 0.0;
        if let Some(call) = &self.call {
            open_interest += call.ticker.open_interest.unwrap_or(0.0)
        }
        if let Some(put) = &self.put {
            open_interest += put.ticker.open_interest.unwrap_or(0.0)
        }
        open_interest
    }

    fn interest_rate(&self) -> f64 {
        let mut interest_rate: f64 = 0.0;
        if let Some(call) = &self.call {
            interest_rate += call.ticker.interest_rate.unwrap_or(0.0)
        }
        if let Some(put) = &self.put {
            interest_rate += put.ticker.interest_rate.unwrap_or(0.0)
        }
        interest_rate
    }

    fn value(&self) -> Option<Value> {
        serde_json::to_value(self).ok()
    }

    fn call_spread(&self) -> Spread {
        if let Some(call) = &self.call {
            let bid = call.ticker.best_bid_price;
            let ask = call.ticker.best_ask_price;
            let mid = match (bid, ask) {
                (Some(b), Some(a)) => Some((b + a) / 2.0),
                (Some(b), None) => Some(b),
                (None, Some(a)) => Some(a),
                (None, None) => None,
            };
            Spread { bid, ask, mid }
        } else {
            Spread {
                bid: None,
                ask: None,
                mid: None,
            }
        }
    }

    fn put_spread(&self) -> Spread {
        if let Some(put) = &self.put {
            let bid = put.ticker.best_bid_price;
            let ask = put.ticker.best_ask_price;
            let mid = match (bid, ask) {
                (Some(b), Some(a)) => Some((b + a) / 2.0),
                (Some(b), None) => Some(b),
                (None, Some(a)) => Some(a),
                (None, None) => None,
            };
            Spread { bid, ask, mid }
        } else {
            Spread {
                bid: None,
                ask: None,
                mid: None,
            }
        }
    }

    fn iv(&self) -> (Option<f64>, Option<f64>) {
        let call_iv = self.call.as_ref().and_then(|c| c.ticker.mark_iv);
        let put_iv = self.put.as_ref().and_then(|p| p.ticker.mark_iv);
        (call_iv, put_iv)
    }

    fn greeks(&self) -> BasicGreeks {
        let delta_call = self
            .call
            .as_ref()
            .and_then(|c| c.ticker.greeks.as_ref().and_then(|g| g.delta));
        let delta_put = self
            .put
            .as_ref()
            .and_then(|p| p.ticker.greeks.as_ref().and_then(|g| g.delta));
        let gamma = self
            .call
            .as_ref()
            .and_then(|c| c.ticker.greeks.as_ref().and_then(|g| g.gamma))
            .or_else(|| {
                self.put
                    .as_ref()
                    .and_then(|p| p.ticker.greeks.as_ref().and_then(|g| g.gamma))
            });
        BasicGreeks {
            delta_call,
            delta_put,
            gamma,
        }
    }

    fn data(&self) -> BasicOptionData {
        let strike_price: f64 = match self.instrument() {
            Some(i) => i.strike.unwrap_or(0.0),
            None => 0.0,
        };
        let call_spread = self.call_spread();
        let call_bid: Option<f64> = call_spread.bid;
        let call_ask: Option<f64> = call_spread.ask;
        let put_spread = self.put_spread();
        let put_bid: Option<f64> = put_spread.bid;
        let put_ask: Option<f64> = put_spread.ask;
        let implied_volatility = self.iv();
        let greeks = self.greeks();
        let delta_call: Option<f64> = greeks.delta_call;
        let delta_put: Option<f64> = greeks.delta_put;
        let gamma: Option<f64> = greeks.gamma;
        let volume = self.volume();
        let open_interest: f64 = self.open_interest();
        let expiration_date: Option<DateTime<Utc>> = self.expiration();
        let underlying_price: Option<f64> = self.ticker().and_then(|t| t.underlying_price);
        let risk_free_rate: f64 = self.interest_rate();
        let extra_fields: Option<Value> = self.value();
        BasicOptionData {
            strike_price,
            call_bid,
            call_ask,
            put_bid,
            put_ask,
            implied_volatility,
            delta_call,
            delta_put,
            gamma,
            volume,
            open_interest,
            expiration_date,
            underlying_price,
            risk_free_rate,
            extra_fields,
        }
    }
}

impl_json_debug_pretty!(
    OptionInstrument,
    OptionInstrumentPair,
    Spread,
    BasicGreeks,
    BasicOptionData
);
impl_json_display!(
    OptionInstrument,
    OptionInstrumentPair,
    Spread,
    BasicGreeks,
    BasicOptionData
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ticker::{Greeks, TickerStats};
    use serde_json;

    fn create_test_instrument(name: &str, strike: f64, option_type: &str) -> Instrument {
        use crate::model::instrument::{InstrumentKind, InstrumentType, OptionType};

        Instrument {
            instrument_name: name.to_string(),
            strike: Some(strike),
            option_type: Some(match option_type {
                "call" => OptionType::Call,
                "put" => OptionType::Put,
                _ => OptionType::Call,
            }),
            expiration_timestamp: Some(1757491200000),
            kind: Some(InstrumentKind::Option),
            instrument_type: Some(InstrumentType::Reversed),
            currency: Some("BTC".to_string()),
            is_active: Some(true),
            contract_size: Some(1.0),
            tick_size: Some(0.0001),
            min_trade_amount: Some(0.1),
            settlement_currency: Some("BTC".to_string()),
            base_currency: Some("BTC".to_string()),
            counter_currency: Some("USD".to_string()),
            quote_currency: Some("BTC".to_string()),
            price_index: None,
            maker_commission: None,
            taker_commission: None,
            instrument_id: None,
            creation_timestamp: None,
            settlement_period: None,
            max_leverage: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_test_ticker(
        instrument_name: &str,
        last_price: f64,
        mark_price: f64,
        bid_price: Option<f64>,
        ask_price: Option<f64>,
        bid_amount: f64,
        ask_amount: f64,
        volume: f64,
        open_interest: f64,
        delta: Option<f64>,
        gamma: Option<f64>,
        mark_iv: Option<f64>,
    ) -> TickerData {
        TickerData {
            instrument_name: instrument_name.to_string(),
            last_price: Some(last_price),
            mark_price,
            best_bid_price: bid_price,
            best_ask_price: ask_price,
            best_bid_amount: bid_amount,
            best_ask_amount: ask_amount,
            timestamp: 1757476246684,
            state: "open".to_string(),
            stats: TickerStats {
                volume,
                volume_usd: Some(volume * 1000.0),
                high: Some(0.1),
                low: Some(0.01),
                price_change: Some(5.0),
            },
            greeks: Some(Greeks {
                delta,
                gamma,
                vega: Some(0.02544),
                theta: Some(-0.84746),
                rho: Some(0.50202),
            }),
            open_interest: Some(open_interest),
            mark_iv,
            underlying_price: Some(111421.0915),
            interest_rate: Some(0.0),
            volume: None,
            volume_usd: None,
            high: None,
            low: None,
            price_change: None,
            price_change_percentage: None,
            bid_iv: None,
            ask_iv: None,
            settlement_price: None,
            index_price: None,
            min_price: None,
            max_price: None,
            underlying_index: None,
            estimated_delivery_price: None,
        }
    }

    #[test]
    fn test_option_instrument_creation() {
        let instrument = create_test_instrument("BTC-10SEP25-106000-C", 106000.0, "call");
        let ticker = create_test_ticker(
            "BTC-10SEP25-106000-C",
            0.047,
            0.0487,
            Some(0.0001),
            Some(0.05),
            10.0,
            30.8,
            104.2,
            49.6,
            Some(0.99972),
            Some(0.0),
            Some(66.62),
        );

        let option_instrument = OptionInstrument { instrument, ticker };

        assert_eq!(
            option_instrument.instrument.instrument_name,
            "BTC-10SEP25-106000-C"
        );
        assert_eq!(option_instrument.instrument.strike, Some(106000.0));
        assert_eq!(option_instrument.ticker.last_price, Some(0.047));
        assert_eq!(option_instrument.ticker.mark_price, 0.0487);
    }

    #[test]
    fn test_option_instrument_pair_serialization() {
        let json_str = r#"{
            "call": {
                "instrument": {
                    "instrument_name": "BTC-10SEP25-106000-C",
                    "strike": 106000.0,
                    "option_type": "call",
                    "expiration_timestamp": 1757491200000
                },
                "ticker": {
                    "instrument_name": "BTC-10SEP25-106000-C",
                    "timestamp": 1757476246684,
                    "state": "open",
                    "last_price": 0.047,
                    "mark_price": 0.0487,
                    "best_bid_price": 0.0001,
                    "best_ask_price": 0.05,
                    "best_bid_amount": 10.0,
                    "best_ask_amount": 30.8,
                    "stats": {
                        "volume": 104.2,
                        "volume_usd": 666962.69,
                        "high": 0.0635,
                        "low": 0.0245,
                        "price_change": -21.0084
                    }
                }
            },
            "put": null
        }"#;

        let pair: OptionInstrumentPair =
            serde_json::from_str(json_str).expect("Failed to deserialize OptionInstrumentPair");

        assert!(pair.call.is_some());
        assert!(pair.put.is_none());

        let call = pair.call.as_ref().unwrap();
        assert_eq!(call.instrument.instrument_name, "BTC-10SEP25-106000-C");
        assert_eq!(call.instrument.strike, Some(106000.0));
    }

    #[test]
    fn test_spread_creation() {
        let spread = Spread {
            bid: Some(0.045),
            ask: Some(0.055),
            mid: Some(0.05),
        };

        assert_eq!(spread.bid, Some(0.045));
        assert_eq!(spread.ask, Some(0.055));
        assert_eq!(spread.mid, Some(0.05));
    }

    #[test]
    fn test_basic_greeks_creation() {
        let greeks = BasicGreeks {
            delta_call: Some(0.99972),
            delta_put: Some(-0.00077),
            gamma: Some(0.0),
        };

        assert_eq!(greeks.delta_call, Some(0.99972));
        assert_eq!(greeks.delta_put, Some(-0.00077));
        assert_eq!(greeks.gamma, Some(0.0));
    }

    #[test]
    fn test_basic_option_data_creation() {
        let expiration = Utc.timestamp_millis_opt(1757491200000).single();
        let option_data = BasicOptionData {
            strike_price: 106000.0,
            call_bid: Some(0.0001),
            call_ask: Some(0.05),
            put_bid: Some(0.0),
            put_ask: Some(0.019),
            implied_volatility: (Some(66.62), Some(107.51)),
            delta_call: Some(0.99972),
            delta_put: Some(-0.00077),
            gamma: Some(0.0),
            volume: 196.1,
            open_interest: 75.2,
            expiration_date: expiration,
            underlying_price: Some(111421.0915),
            risk_free_rate: 0.0,
            extra_fields: None,
        };

        assert_eq!(option_data.strike_price, 106000.0);
        assert_eq!(option_data.call_bid, Some(0.0001));
        assert_eq!(option_data.volume, 196.1);
        assert_eq!(option_data.open_interest, 75.2);
    }

    fn create_test_option_pair() -> OptionInstrumentPair {
        let call_instrument = create_test_instrument("BTC-10SEP25-106000-C", 106000.0, "call");
        let call_ticker = create_test_ticker(
            "BTC-10SEP25-106000-C",
            0.047,
            0.0487,
            Some(0.0001),
            Some(0.05),
            10.0,
            30.8,
            104.2,
            49.6,
            Some(0.99972),
            Some(0.0),
            Some(66.62),
        );

        let put_instrument = create_test_instrument("BTC-10SEP25-106000-P", 106000.0, "put");
        let put_ticker = create_test_ticker(
            "BTC-10SEP25-106000-P",
            0.0002,
            0.0,
            Some(0.0),
            Some(0.019),
            0.0,
            10.0,
            91.9,
            25.6,
            Some(-0.00077),
            Some(0.0),
            Some(107.51),
        );

        OptionInstrumentPair {
            call: Some(OptionInstrument {
                instrument: call_instrument,
                ticker: call_ticker,
            }),
            put: Some(OptionInstrument {
                instrument: put_instrument,
                ticker: put_ticker,
            }),
        }
    }

    #[test]
    fn test_option_pair_expiration() {
        let pair = create_test_option_pair();
        let expiration = pair.expiration();

        assert!(expiration.is_some());
        let exp_date = expiration.unwrap();
        assert_eq!(exp_date.timestamp_millis(), 1757491200000);
    }

    #[test]
    fn test_option_pair_instrument() {
        let pair = create_test_option_pair();
        let instrument = pair.instrument();

        assert!(instrument.is_some());
        let inst = instrument.unwrap();
        assert_eq!(inst.instrument_name, "BTC-10SEP25-106000-C");
        assert_eq!(inst.strike, Some(106000.0));
    }

    #[test]
    fn test_option_pair_ticker() {
        let pair = create_test_option_pair();
        let ticker = pair.ticker();

        assert!(ticker.is_some());
        let tick = ticker.unwrap();
        assert_eq!(tick.instrument_name, "BTC-10SEP25-106000-C");
        assert_eq!(tick.last_price, Some(0.047));
    }

    #[test]
    fn test_option_pair_volume() {
        let pair = create_test_option_pair();
        let volume = pair.volume();

        // Should be sum of call (104.2) + put (91.9) = 196.1
        assert!((volume - 196.1).abs() < 1e-10);
    }

    #[test]
    fn test_option_pair_open_interest() {
        let pair = create_test_option_pair();
        let open_interest = pair.open_interest();

        // Should be sum of call (49.6) + put (25.6) = 75.2
        assert_eq!(open_interest, 75.2);
    }

    #[test]
    fn test_option_pair_interest_rate() {
        let pair = create_test_option_pair();
        let interest_rate = pair.interest_rate();

        // Both call and put have 0.0 interest rate
        assert_eq!(interest_rate, 0.0);
    }

    #[test]
    fn test_option_pair_value() {
        let pair = create_test_option_pair();
        let value = pair.value();

        assert!(value.is_some());
        // Should be able to serialize to JSON
        let json_value = value.unwrap();
        assert!(json_value.is_object());
    }

    #[test]
    fn test_option_pair_call_spread() {
        let pair = create_test_option_pair();
        let call_spread = pair.call_spread();

        assert_eq!(call_spread.bid, Some(0.0001));
        assert_eq!(call_spread.ask, Some(0.05));
        assert_eq!(call_spread.mid, Some((0.0001 + 0.05) / 2.0));
    }

    #[test]
    fn test_option_pair_put_spread() {
        let pair = create_test_option_pair();
        let put_spread = pair.put_spread();

        assert_eq!(put_spread.bid, Some(0.0));
        assert_eq!(put_spread.ask, Some(0.019));
        assert_eq!(put_spread.mid, Some((0.0 + 0.019) / 2.0));
    }

    #[test]
    fn test_option_pair_iv() {
        let pair = create_test_option_pair();
        let (call_iv, put_iv) = pair.iv();

        assert_eq!(call_iv, Some(66.62));
        assert_eq!(put_iv, Some(107.51));
    }

    #[test]
    fn test_option_pair_greeks() {
        let pair = create_test_option_pair();
        let greeks = pair.greeks();

        assert_eq!(greeks.delta_call, Some(0.99972));
        assert_eq!(greeks.delta_put, Some(-0.00077));
        assert_eq!(greeks.gamma, Some(0.0));
    }

    #[test]
    fn test_option_pair_data() {
        let pair = create_test_option_pair();
        let data = pair.data();

        assert_eq!(data.strike_price, 106000.0);
        assert_eq!(data.call_bid, Some(0.0001));
        assert_eq!(data.call_ask, Some(0.05));
        assert_eq!(data.put_bid, Some(0.0));
        assert_eq!(data.put_ask, Some(0.019));
        assert_eq!(data.implied_volatility, (Some(66.62), Some(107.51)));
        assert_eq!(data.delta_call, Some(0.99972));
        assert_eq!(data.delta_put, Some(-0.00077));
        assert_eq!(data.gamma, Some(0.0));
        assert!((data.volume - 196.1).abs() < 1e-10);
        assert_eq!(data.open_interest, 75.2);
        assert_eq!(data.underlying_price, Some(111421.0915));
        assert_eq!(data.risk_free_rate, 0.0);
    }

    #[test]
    fn test_option_pair_with_only_call() {
        let call_instrument = create_test_instrument("BTC-10SEP25-106000-C", 106000.0, "call");
        let call_ticker = create_test_ticker(
            "BTC-10SEP25-106000-C",
            0.047,
            0.0487,
            Some(0.0001),
            Some(0.05),
            10.0,
            30.8,
            104.2,
            49.6,
            Some(0.99972),
            Some(0.0),
            Some(66.62),
        );

        let pair = OptionInstrumentPair {
            call: Some(OptionInstrument {
                instrument: call_instrument,
                ticker: call_ticker,
            }),
            put: None,
        };

        assert_eq!(pair.volume(), 104.2);
        assert_eq!(pair.open_interest(), 49.6);

        let put_spread = pair.put_spread();
        assert_eq!(put_spread.bid, None);
        assert_eq!(put_spread.ask, None);
        assert_eq!(put_spread.mid, None);

        let (call_iv, put_iv) = pair.iv();
        assert_eq!(call_iv, Some(66.62));
        assert_eq!(put_iv, None);
    }

    #[test]
    fn test_option_pair_with_only_put() {
        let put_instrument = create_test_instrument("BTC-10SEP25-106000-P", 106000.0, "put");
        let put_ticker = create_test_ticker(
            "BTC-10SEP25-106000-P",
            0.0002,
            0.0,
            Some(0.0),
            Some(0.019),
            0.0,
            10.0,
            91.9,
            25.6,
            Some(-0.00077),
            Some(0.0),
            Some(107.51),
        );

        let pair = OptionInstrumentPair {
            call: None,
            put: Some(OptionInstrument {
                instrument: put_instrument,
                ticker: put_ticker,
            }),
        };

        assert_eq!(pair.volume(), 91.9);
        assert_eq!(pair.open_interest(), 25.6);

        let call_spread = pair.call_spread();
        assert_eq!(call_spread.bid, None);
        assert_eq!(call_spread.ask, None);
        assert_eq!(call_spread.mid, None);

        let (call_iv, put_iv) = pair.iv();
        assert_eq!(call_iv, None);
        assert_eq!(put_iv, Some(107.51));
    }

    #[test]
    fn test_empty_option_pair() {
        let pair = OptionInstrumentPair {
            call: None,
            put: None,
        };

        assert_eq!(pair.volume(), 0.0);
        assert_eq!(pair.open_interest(), 0.0);
        assert_eq!(pair.interest_rate(), 0.0);
        assert!(pair.instrument().is_none());
        assert!(pair.ticker().is_none());
        assert!(pair.expiration().is_none());

        let (call_iv, put_iv) = pair.iv();
        assert_eq!(call_iv, None);
        assert_eq!(put_iv, None);

        let greeks = pair.greeks();
        assert_eq!(greeks.delta_call, None);
        assert_eq!(greeks.delta_put, None);
        assert_eq!(greeks.gamma, None);
    }
}
