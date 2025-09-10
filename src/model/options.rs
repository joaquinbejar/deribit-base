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

#[derive(Clone, Serialize, Deserialize)]
pub struct Spread {
    bid: Option<f64>,
    ask: Option<f64>,
    mid: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BasicGreeks {
    delta_call: Option<f64>,
    delta_put: Option<f64>,
    gamma: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BasicOptionData {
    pub strike_price: f64,
    pub call_bid: Option<f64>,
    pub call_ask: Option<f64>,
    pub put_bid: Option<f64>,
    pub put_ask: Option<f64>,
    pub implied_volatility: (Option<f64>, Option<f64>),
    pub delta_call: Option<f64>,
    pub delta_put: Option<f64>,
    pub gamma: Option<f64>,
    pub volume: f64,
    pub open_interest: f64,
    pub expiration_date: Option<DateTime<Utc>>,
    pub underlying_price: Option<f64>,
    pub risk_free_rate: f64,
    pub extra_fields: Option<Value>,
}

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
