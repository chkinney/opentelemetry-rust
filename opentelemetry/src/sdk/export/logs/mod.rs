use std::fmt::Debug;
use async_trait::async_trait;
use crate::{logs::LogData, sdk::export::trace::ExportResult};

#[async_trait]
pub trait LogRecorder : Send + Debug {
    async fn record(&mut self, batch: Vec<LogData>) -> ExportResult;
}
