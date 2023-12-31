use anyhow::Result;
use std::time::Duration;

use orderbook_agg::book_summary::Summary;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        duration: Duration,
        counter_sleep: u32,
        counter_tick: u64,
        summary: Summary,
        datapoints_bid: Vec<(f64, f64)>,
        datapoints_ask: Vec<(f64, f64)>,
        datapoints_spread: Vec<(f64, f64)>,
    },
}

impl AppState {
    pub async fn initialized() -> Result<Self> {
        let duration = Duration::from_secs(1);
        let counter_sleep = 0;
        let counter_tick = 0;
        let summary = Summary::default();
        let datapoints_bid = Vec::new();
        let datapoints_ask = Vec::new();
        let datapoints_spread = Vec::new();

        Ok(Self::Initialized {
            duration,
            counter_sleep,
            counter_tick,
            summary,
            datapoints_bid,
            datapoints_ask,
            datapoints_spread,
        })
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_sleep(&mut self) {
        if let Self::Initialized { counter_sleep, .. } = self {
            *counter_sleep += 1;
        }
    }

    pub fn incr_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    pub fn count_sleep(&self) -> Option<u32> {
        if let Self::Initialized { counter_sleep, .. } = self {
            Some(*counter_sleep)
        } else {
            None
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }

    pub fn get_summary(&self) -> Option<&Summary> {
        if let Self::Initialized { summary, .. } = self {
            Some(&summary)
        } else {
            None
        }
    }

    pub fn get_datapoints(&self) -> Option<[&Vec<(f64, f64)>; 3]> {
        if let Self::Initialized {
            datapoints_bid,
            datapoints_ask,
            datapoints_spread,
            ..
        } = self
        {
            Some([datapoints_bid, datapoints_ask, datapoints_spread])
        } else {
            None
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized { duration, .. } = self {
            Some(duration)
        } else {
            None
        }
    }

    pub fn increment_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() + 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }

    pub fn decrement_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() - 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }

    pub fn update_summary(&mut self, summary_new: Summary) {
        if let Self::Initialized {
            summary,
            datapoints_bid,
            datapoints_ask,
            datapoints_spread,
            ..
        } = self
        {
            datapoints_bid.push((summary_new.timestamp as f64, summary_new.bids[0].price));
            datapoints_ask.push((summary_new.timestamp as f64, summary_new.asks[0].price));
            datapoints_spread.push((summary_new.timestamp as f64, summary_new.spread));

            *summary = summary_new;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
