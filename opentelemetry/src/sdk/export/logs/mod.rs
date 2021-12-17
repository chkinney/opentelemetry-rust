use crate::logs::{LogData, LogError};
use async_trait::async_trait;
use std::fmt::Debug;

/// Describes the result of an export.
pub type ExportResult = Result<(), LogError>;

/// Records log messages and prepares them to be exported.
#[async_trait]
pub trait LogRecorder: Send + Debug {
    async fn record(&mut self, batch: Vec<LogData>) -> ExportResult;
}

/// A log message that is being exported.
#[cfg_attr(feature = "serialize", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct LogData {
    pub timestamp: SystemTime,
    pub trace_id: Option<TraceId>,
    pub span_id: Option<SpanId>,
    pub trace_flags: TraceFlags,
    pub severity_text: Cow<'static, str>,
    pub severity_number: SeverityNumber,
    pub name: Cow<'static, str>,
    pub body: Cow<'static, str>,
    pub resource: Option<Arc<Resource>>,
    pub attributes: EvictedHashMap,
}
