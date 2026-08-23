use std::path::PathBuf;
use std::sync::Arc;

use tinybus::{Connection, Result as BusResult};
use tinyjuice::cache::store::RangeUnit;
use tinyjuice::types::{AgentTokenjuiceCompression, CompressedOutput, ContentHint};

// The interface's vocabulary is the contract crate's, not this adapter's.
// These five used to be private structs in this file, which meant a host had
// no way to reach them and re-declared its own — the drift a shared contract
// exists to remove.
pub use tinyjuice_bus::names::{BUS_NAME, ML_HOST_NAME, ML_HOST_PATH, OBJECT_PATH};
use tinyjuice_bus::wire::{
    CacheStats, CompactResponse, InstallRequest, RangeUnit as WireRangeUnit, RetrieveRange,
};

#[derive(Clone)]
struct Compression;

#[tinybus::interface(name = "ai.tinyhumans.tinyjuice.Compression")]
impl Compression {
    async fn install(&self, request: InstallRequest) -> BusResult<()> {
        tinyjuice::tool_integration::install_config(
            request.options,
            request.max_cache_entries,
            request.max_cache_bytes,
            request.ccr_ttl_secs,
            request.disk_tier_root.map(PathBuf::from),
        );
        Ok(())
    }

    async fn detect(&self, content: String, hint: ContentHint) -> BusResult<String> {
        Ok(tinyjuice::detect_content_kind(&content, &hint)
            .as_str()
            .to_string())
    }

    async fn compress(&self, content: String, hint: ContentHint) -> BusResult<CompressedOutput> {
        let options = tinyjuice::tool_integration::current_options();
        Ok(tinyjuice::compress_content(&content, Some(hint), &options).await)
    }

    async fn compact(
        &self,
        content: String,
        tool_name: String,
        enabled: bool,
        profile: AgentTokenjuiceCompression,
    ) -> BusResult<CompactResponse> {
        if !enabled {
            let bytes = content.len();
            let tokens = tinyjuice::tokens::estimate_tokens(&content);
            return Ok(CompactResponse {
                text: content,
                original_bytes: bytes,
                compacted_bytes: bytes,
                rule_id: "none/disabled".to_string(),
                applied: false,
                content_kind: "plain_text".to_string(),
                compressor: "none".to_string(),
                original_tokens: tokens,
                compacted_tokens: tokens,
            });
        }
        let original_tokens = tinyjuice::tokens::estimate_tokens(&content);
        let (text, stats) = tinyjuice::tool_integration::compact_tool_output_with_policy(
            &tool_name, None, &content, None, profile,
        )
        .await;
        let compacted_tokens = tinyjuice::tokens::estimate_tokens(&text);
        let (compressor, content_kind) = stats.rule_id.strip_prefix("none/").map_or_else(
            || (stats.rule_id.clone(), "plain_text".to_string()),
            |kind| ("none".to_string(), kind.to_string()),
        );
        Ok(CompactResponse {
            text,
            original_bytes: stats.original_bytes,
            compacted_bytes: stats.compacted_bytes,
            rule_id: stats.rule_id,
            applied: stats.applied,
            content_kind,
            compressor,
            original_tokens,
            compacted_tokens,
        })
    }

    async fn retrieve(
        &self,
        token: String,
        range: Option<RetrieveRange>,
    ) -> BusResult<Option<String>> {
        Ok(match range {
            Some(range) => tinyjuice::cache::retrieve_range(
                &token,
                range.start,
                range.end,
                match range.unit {
                    WireRangeUnit::Bytes => RangeUnit::Bytes,
                    WireRangeUnit::Lines => RangeUnit::Lines,
                },
            ),
            None => tinyjuice::cache::retrieve(&token),
        })
    }

    async fn cache_stats(&self) -> BusResult<CacheStats> {
        let (entries, bytes) = tinyjuice::cache::stats();
        Ok(CacheStats { entries, bytes })
    }
}

async fn setup(connection: Connection) -> BusResult<()> {
    let ml_connection = connection.clone();
    tinyjuice::ml::configure_callback(Some(Arc::new(move |text, options| {
        let connection = ml_connection.clone();
        Box::pin(async move {
            let proxy = connection
                .proxy(ML_HOST_NAME, ML_HOST_PATH, ML_HOST_NAME)
                .map_err(|error| error.to_string())?;
            proxy
                .call(
                    "Compress",
                    (
                        text,
                        serde_json::to_value(options).map_err(|error| error.to_string())?,
                    ),
                )
                .await
                .map_err(|error| error.to_string())
        })
    })));

    connection
        .serve_at(OBJECT_PATH.try_into()?, Compression)
        .await?;
    connection.request_name(BUS_NAME).await?;
    Ok(())
}

#[allow(missing_docs, unreachable_pub)]
mod exports {
    tinybus_module::module_export! {
        setup = super::setup,
        worker_threads = 2,
        provides = ["ai.tinyhumans.tinyjuice.Compression"],
        methods = ["Install", "Detect", "Compress", "Compact", "Retrieve", "CacheStats"],
        signals = [],
        requires = [],
        optional = ["ai.tinyhumans.tinyjuice.MlHost"],
        lazy = false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn service_compresses_and_retrieves_a_large_log() {
        let service = Compression;
        service
            .install(InstallRequest {
                options: CompressOptions {
                    min_bytes_to_compress: 64,
                    ccr_min_tokens: 16,
                    ..CompressOptions::default()
                },
                max_cache_entries: 8,
                max_cache_bytes: 1024 * 1024,
                ccr_ttl_secs: None,
                disk_tier_root: None,
            })
            .await
            .expect("install should succeed");

        let content = (0..200)
            .map(|index| {
                if index == 137 {
                    format!("ERROR request failed id={index}\n")
                } else {
                    format!("INFO request completed id={index}\n")
                }
            })
            .collect::<String>();
        let output = service
            .compress(
                content.clone(),
                ContentHint {
                    explicit: Some(tinyjuice::types::ContentKind::Log),
                    ..ContentHint::default()
                },
            )
            .await
            .expect("compression should succeed");
        assert!(output.applied);
        let token = output
            .ccr_token
            .expect("lossy compression should be recoverable");
        assert_eq!(
            service
                .retrieve(token, None)
                .await
                .expect("retrieve should succeed"),
            Some(content)
        );
    }
}
