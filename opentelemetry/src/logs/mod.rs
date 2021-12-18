use crate::{
    sdk::{export::ExportError, trace::EvictedHashMap, Resource},
    trace::{SpanId, TraceFlags, TraceId},
};
use futures::channel::{mpsc::TrySendError, oneshot::Canceled};
use std::{borrow::Cow, sync::Arc, time::SystemTime};
use thiserror::Error;

/// Errors returned from the logs API.
#[derive(Error, Debug)]
pub enum LogError {
    /// Export failed with the error returned by the exporter
    #[error("Exporter {} encountered the following error(s): {0}", .0.exporter_name())]
    ExportFailed(Box<dyn ExportError>),

    /// Export failed to finish after a certain period and the processor stopped the export
    #[error("Exporting timed out after {} seconds", .0.as_secs())]
    ExportTimedOut(std::time::Duration),

    /// Other errors propagated from logs SDK that weren't covered
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl<T> From<T> for LogError
where
    T: ExportError,
{
    fn from(err: T) -> Self {
        LogError::ExportFailed(Box::new(err))
    }
}

impl<T> From<TrySendError<T>> for LogError {
    fn from(err: TrySendError<T>) -> Self {
        LogError::Other(Box::new(err.into_send_error()))
    }
}

impl From<Canceled> for LogError {
    fn from(err: Canceled) -> Self {
        LogError::Other(Box::new(err))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize, serde::Serialize))]
#[repr(i32)]
pub enum SeverityNumber {
    Trace,
    Trace2,
    Trace3,
    Trace4,
    Debug,
    Debug2,
    Debug3,
    Debug4,
    Info,
    Info2,
    Info3,
    Info4,
    Warn,
    Warn2,
    Warn3,
    Warn4,
    Error,
    Error2,
    Error3,
    Error4,
    Fatal,
    Fatal2,
    Fatal3,
    Fatal4,
}
