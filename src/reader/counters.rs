use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

pub(crate) struct AppCounters {
    pub(crate) df_count: BTreeMap<u32, i32>,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) cleanup_count: u32,
}

impl AppCounters {
    pub(crate) fn from_update_interval(update: i64) -> Self {
        AppCounters {
            df_count: BTreeMap::new(),
            timestamp: chrono::Utc::now() + chrono::Duration::seconds(update),
            cleanup_count: 0u32,
        }
    }

    pub(crate) fn update_count(&mut self, df: u32) {
        *self.df_count.entry(df).or_insert(1) += 1;
    }

    pub(crate) fn reset_cleanup_count(&mut self) {
        self.cleanup_count = 0;
    }

    pub(crate) fn increment_cleanup_count(&mut self) {
        self.cleanup_count += 1;
    }

    pub(crate) fn reset_timestamp(&mut self, now: DateTime<Utc>) {
        self.timestamp = now
    }

    pub(crate) fn print_df_count_line(&self) {
        println!(
            "{}",
            self.df_count
                .iter()
                .fold(String::new(), |acc, (df, count)| {
                    acc + &format!("DF{}:{} ", df, count)
                })
        );
    }

    pub(crate) fn is_time_to_refresh(&self, now: &DateTime<Utc>, update: i64) -> bool {
        now.signed_duration_since(self.timestamp).num_seconds() > update
    }
}
