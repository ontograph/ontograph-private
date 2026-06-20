use pretty_assertions::assert_eq;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use uuid::Uuid;

use super::*;

#[tokio::test]
async fn sqlite_sink_uses_default_persistent_filter() {
    let codex_home =
        std::env::temp_dir().join(format!("codex-state-log-db-filter-{}", Uuid::new_v4()));
    let runtime = StateRuntime::init(codex_home.clone(), "test-provider".to_string())
        .await
        .expect("initialize runtime");
    let layer = start(runtime.clone());

    let guard = tracing_subscriber::registry()
        .with(layer.clone().with_filter(default_filter()))
        .set_default();

    tracing::trace!(target: "opentelemetry_sdk", "dropped-trace");
    tracing::info!(target: "opentelemetry_sdk", "dropped-info");
    tracing::trace!(target: "log", "dropped-log-trace");
    tracing::info!(target: "codex_otel.log_only", "dropped-otel-info");
    tracing::info!(target: "codex_state", "retained-codex-info");
    tracing::warn!(target: "log", "retained-log-warn");

    layer.flush().await;
    drop(guard);

    let logs = runtime
        .query_logs(&crate::LogQuery::default())
        .await
        .expect("query logs after flush");
    assert_eq!(
        logs.iter()
            .map(|row| (
                row.level.as_str(),
                row.target.as_str(),
                row.message.as_deref()
            ))
            .collect::<Vec<_>>(),
        vec![
            ("INFO", "codex_state", Some("retained-codex-info")),
            ("WARN", "log", Some("retained-log-warn")),
        ]
    );

    let _ = tokio::fs::remove_dir_all(codex_home).await;
}
