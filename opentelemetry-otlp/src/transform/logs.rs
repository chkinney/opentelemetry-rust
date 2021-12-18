#[cfg(feature = "tonic")]
mod tonic {
    use crate::{
        proto::{
            logs::v1::{InstrumentationLibraryLogs, LogRecord, ResourceLogs, SeverityNumber},
            resource::v1::Resource,
        },
        transform::common::{to_nanos, tonic::Attributes},
    };
    use opentelemetry::{
        trace::{SpanId, TraceId},
        Value,
    };

    impl From<opentelemetry::sdk::export::logs::LogData> for ResourceLogs {
        fn from(record: opentelemetry::sdk::export::logs::LogData) -> Self {
            ResourceLogs {
                resource: record.resource.as_ref().map(|resource| Resource {
                    attributes: Attributes::from(
                        resource
                            .iter()
                            .map(|(k, v)| opentelemetry::KeyValue::new(k.clone(), v.clone()))
                            .collect::<Vec<_>>(),
                    )
                    .0,
                    dropped_attributes_count: 0,
                }),
                instrumentation_library_logs: vec![InstrumentationLibraryLogs {
                    instrumentation_library: None,
                    logs: vec![LogRecord {
                        time_unix_nano: to_nanos(record.timestamp),
                        severity_number: SeverityNumber::from(record.severity_number).into(),
                        severity_text: record.severity_text.into(),
                        name: record.name.into(),
                        body: Some(Value::String(record.body.clone()).into()),
                        dropped_attributes_count: record.attributes.dropped_count(),
                        attributes: Attributes::from(record.attributes).0,
                        flags: record.trace_flags.to_u8().into(),
                        trace_id: record
                            .trace_id
                            .unwrap_or(TraceId::invalid())
                            .to_u128()
                            .to_be_bytes()
                            .to_vec(),
                        span_id: record
                            .span_id
                            .unwrap_or(SpanId::invalid())
                            .to_u64()
                            .to_be_bytes()
                            .to_vec(),
                    }],
                }],
            }
        }
    }

    impl From<opentelemetry::logs::SeverityNumber> for SeverityNumber {
        fn from(severity: opentelemetry::logs::SeverityNumber) -> Self {
            match severity {
                opentelemetry::logs::SeverityNumber::Trace => SeverityNumber::Trace,
                opentelemetry::logs::SeverityNumber::Trace2 => SeverityNumber::Trace2,
                opentelemetry::logs::SeverityNumber::Trace3 => SeverityNumber::Trace3,
                opentelemetry::logs::SeverityNumber::Trace4 => SeverityNumber::Trace4,
                opentelemetry::logs::SeverityNumber::Debug => SeverityNumber::Debug,
                opentelemetry::logs::SeverityNumber::Debug2 => SeverityNumber::Debug2,
                opentelemetry::logs::SeverityNumber::Debug3 => SeverityNumber::Debug3,
                opentelemetry::logs::SeverityNumber::Debug4 => SeverityNumber::Debug4,
                opentelemetry::logs::SeverityNumber::Info => SeverityNumber::Info,
                opentelemetry::logs::SeverityNumber::Info2 => SeverityNumber::Info2,
                opentelemetry::logs::SeverityNumber::Info3 => SeverityNumber::Info3,
                opentelemetry::logs::SeverityNumber::Info4 => SeverityNumber::Info4,
                opentelemetry::logs::SeverityNumber::Warn => SeverityNumber::Warn,
                opentelemetry::logs::SeverityNumber::Warn2 => SeverityNumber::Warn2,
                opentelemetry::logs::SeverityNumber::Warn3 => SeverityNumber::Warn3,
                opentelemetry::logs::SeverityNumber::Warn4 => SeverityNumber::Warn4,
                opentelemetry::logs::SeverityNumber::Error => SeverityNumber::Error,
                opentelemetry::logs::SeverityNumber::Error2 => SeverityNumber::Error2,
                opentelemetry::logs::SeverityNumber::Error3 => SeverityNumber::Error3,
                opentelemetry::logs::SeverityNumber::Error4 => SeverityNumber::Error4,
                opentelemetry::logs::SeverityNumber::Fatal => SeverityNumber::Fatal,
                opentelemetry::logs::SeverityNumber::Fatal2 => SeverityNumber::Fatal2,
                opentelemetry::logs::SeverityNumber::Fatal3 => SeverityNumber::Fatal3,
                opentelemetry::logs::SeverityNumber::Fatal4 => SeverityNumber::Fatal4,
            }
        }
    }
}
