use std::time::Duration;

use crate::{
    proto::collector::logs::v1::{
        logs_service_client::LogsServiceClient, ExportLogsServiceRequest,
    },
    ExportConfig,
};
use async_trait::async_trait;
use opentelemetry::sdk::export::{logs::LogData, trace::ExportResult};

#[derive(Clone, Debug)]
pub enum LogRecorder {
    #[cfg(feature = "tonic")]
    Tonic {
        timeout: Duration,
        metadata: Option<tonic::metadata::MetadataMap>,
        exporter: LogsServiceClient<tonic::transport::Channel>,
    },
}

#[cfg(feature = "tonic")]
impl LogRecorder {
    pub fn new_tonic(
        config: ExportConfig,
        tonic_config: crate::exporter::tonic::TonicConfig,
    ) -> Result<Self, crate::Error> {
        use tonic::transport::channel::Channel;

        let endpoint = Channel::from_shared(config.endpoint.clone())?;

        #[cfg(feature = "tls")]
        let channel = match tonic_config.tls_config.as_ref() {
            Some(tls_config) => endpoint.tls_config(tls_config.clone())?,
            None => endpoint,
        }
        .timeout(config.timeout)
        .connect_lazy()?;

        #[cfg(not(feature = "tls"))]
        let channel = endpoint.timeout(config.timeout).connect_lazy()?;

        Self::from_tonic_channel(config, tonic_config, channel)
    }

    pub fn from_tonic_channel(
        config: ExportConfig,
        tonic_config: crate::exporter::tonic::TonicConfig,
        channel: tonic::transport::Channel,
    ) -> Result<Self, crate::Error> {
        Ok(LogRecorder::Tonic {
            timeout: config.timeout,
            metadata: tonic_config.metadata,
            exporter: LogsServiceClient::new(channel),
        })
    }
}

#[async_trait]
impl opentelemetry::sdk::export::logs::LogRecorder for LogRecorder {
    async fn record(&mut self, batch: Vec<LogData>) -> ExportResult {
        match self {
            #[cfg(feature = "tonic")]
            LogRecorder::Tonic {
                metadata, exporter, ..
            } => {
                use tonic::{metadata::KeyAndValueRef, Request};

                let mut request = Request::new(ExportLogsServiceRequest {
                    resource_logs: batch.into_iter().map(Into::into).collect(),
                });

                if let Some(metadata) = metadata {
                    for key_and_value in metadata.iter() {
                        match key_and_value {
                            KeyAndValueRef::Ascii(key, value) => {
                                request.metadata_mut().append(key, value.to_owned())
                            }
                            KeyAndValueRef::Binary(key, value) => {
                                request.metadata_mut().append_bin(key, value.to_owned())
                            }
                        };
                    }
                }

                exporter
                    .export(request)
                    .await
                    .map_err::<crate::Error, _>(Into::into)?;
            }
        }

        Ok(())
    }
}
