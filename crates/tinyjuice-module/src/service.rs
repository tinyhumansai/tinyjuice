use std::path::PathBuf;
use std::sync::Arc;

use tinybus::{Connection, Result as BusResult};
use tinyjuice::cache::store::RangeUnit;
use tinyjuice::types::{AgentTokenjuiceCompression, CompressedOutput, ContentHint};

// The interface's vocabulary is the contract crate's, not this adapter's.
// These five used to be private structs in this file, which meant a host had
// no way to reach them and re-declared its own — the drift a shared contract
// exists to remove.
use tinyjuice::tool_integration::ToolOutputCall;
use tinyjuice_bus::names::ml_host;
pub use tinyjuice_bus::names::{BUS_NAME, ML_HOST_NAME, ML_HOST_PATH, OBJECT_PATH};
use tinyjuice_bus::wire::{
    CacheStats, CompactRequest, CompactResponse, GenerateRequest, InstallRequest,
    RangeUnit as WireRangeUnit, RetrieveRange,
};

/// Deadline for one host model call. A summary of a large page takes tens of
/// seconds, well past the bus default; this bounds a hung host, not a slow one.
const GENERATE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(300);

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
        Ok(compact_request(CompactRequest {
            content,
            tool_name,
            enabled,
            profile,
            arguments: None,
            focus: None,
            context_token: None,
            scope: None,
        })
        .await)
    }

    async fn compact_with(&self, request: CompactRequest) -> BusResult<CompactResponse> {
        Ok(compact_request(request).await)
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

/// `Compact` and `CompactWith` share one body; the positional form is
/// `CompactWith` with no arguments, focus or context. `enabled = false` turns
/// the content router off but not the summary stage, which is gated by its
/// own option and a context token.
async fn compact_request(request: CompactRequest) -> CompactResponse {
    let CompactRequest {
        content,
        tool_name,
        enabled,
        profile,
        arguments,
        focus,
        context_token,
        scope,
    } = request;
    let original_tokens = tinyjuice::tokens::estimate_tokens(&content);
    let report = tinyjuice::tool_integration::compact_tool_output(ToolOutputCall {
        tool_name: &tool_name,
        arguments: arguments.as_ref(),
        output: &content,
        exit_code: None,
        profile,
        compaction_enabled: enabled,
        focus: focus.as_deref(),
        context_token: context_token.as_deref(),
        scope: scope.as_deref(),
    })
    .await;
    let stats = report.stats;
    let compacted_tokens = tinyjuice::tokens::estimate_tokens(&report.text);
    let (compressor, content_kind) = stats.rule_id.strip_prefix("none/").map_or_else(
        || (stats.rule_id.clone(), "plain_text".to_string()),
        |kind| ("none".to_string(), kind.to_string()),
    );
    CompactResponse {
        text: report.text,
        original_bytes: stats.original_bytes,
        compacted_bytes: stats.compacted_bytes,
        rule_id: stats.rule_id,
        applied: stats.applied,
        content_kind,
        compressor,
        original_tokens,
        compacted_tokens,
        notice: report.notice.map(str::to_string),
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
                    ml_host::COMPRESS,
                    (
                        text,
                        serde_json::to_value(options).map_err(|error| error.to_string())?,
                    ),
                )
                .await
                .map_err(|error| error.to_string())
        })
    })));

    // The summary stage's model call goes to the same host object: the module
    // writes the prompt, the host owns the model and the turn it runs under.
    let llm_connection = connection.clone();
    tinyjuice::llm::configure_callback(Some(Arc::new(move |request: GenerateRequest| {
        let connection = llm_connection.clone();
        Box::pin(async move {
            let proxy = connection
                .proxy(ML_HOST_NAME, ML_HOST_PATH, ML_HOST_NAME)
                .map_err(|error| error.to_string())?
                .with_timeout(GENERATE_TIMEOUT);
            proxy
                .call(ml_host::GENERATE, (request,))
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
    tinybus_module::module_export_optional_static! {
        setup = super::setup,
        worker_threads = 2,
        provides = ["ai.tinyhumans.tinyjuice.Compression"],
        methods = [
            "Install",
            "Detect",
            "Compress",
            "Compact",
            "CompactWith",
            "Retrieve",
            "CacheStats"
        ],
        signals = [],
        requires = [],
        optional = ["ai.tinyhumans.tinyjuice.MlHost"],
        lazy = false,
    }
}

#[cfg(feature = "static-link")]
pub use exports::linked_module;

#[cfg(test)]
mod tests {
    use super::*;
    use tinyjuice::types::CompressOptions;

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
