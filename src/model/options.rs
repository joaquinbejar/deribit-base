/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 9/9/25
******************************************************************************/
use crate::prelude::{Instrument, TickerData};
use serde::{Deserialize, Serialize};

/// Combined option instrument data with ticker information
#[derive(Clone, Serialize, Deserialize)]
pub struct OptionInstrument {
    /// The instrument details
    pub instrument: Instrument,
    /// Real-time ticker data for the option
    pub ticker: TickerData,
}
