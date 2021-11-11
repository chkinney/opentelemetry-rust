use crate::{
    sdk::{trace::EvictedHashMap, Resource},
    trace::{SpanId, TraceFlags, TraceId},
};
use std::{borrow::Cow, sync::Arc, time::SystemTime};

#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct LogData {
    pub timestamp: SystemTime,
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub trace_flags: TraceFlags,
    pub severity_text: Cow<'static, str>,
    pub severity_number: SeverityNumber,
    pub name: Cow<'static, str>,
    pub body: Cow<'static, str>,
    pub resource: Option<Arc<Resource>>,
    pub attributes: EvictedHashMap,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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
