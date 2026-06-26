use crate::StateDbHandle;
use ontocode_state::EvidenceStatus;
use ontocode_state::OperationalEvidenceQuery;
use ontocode_state::OperationalEvidenceSummary;
use ontocode_state::RedactionStatus;
use tracing::warn;

use super::ContextualUserFragment;

const CONTEXT_START_MARKER: &str = "<operational_evidence_context>";
const CONTEXT_END_MARKER: &str = "</operational_evidence_context>";
const MAX_QUERY_RECORDS: usize = 3;
const MAX_QUERY_BYTES: usize = 2_048;
const MAX_RENDERED_BYTES: usize = 4_096;
const MAX_SUMMARY_CHARS: usize = 240;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OperationalEvidenceContextFragment {
    thread_id: String,
    records: Vec<OperationalEvidenceSummary>,
}

impl OperationalEvidenceContextFragment {
    pub(crate) async fn from_state_db(
        state_db: Option<&StateDbHandle>,
        thread_id: &str,
    ) -> Option<Self> {
        let state_db = state_db?;

        let records = match state_db
            .query_operational_evidence(OperationalEvidenceQuery {
                thread_id: Some(thread_id.to_string()),
                limit: Some(MAX_QUERY_RECORDS),
                byte_limit: Some(MAX_QUERY_BYTES),
                ..Default::default()
            })
            .await
        {
            Ok(records) => records,
            Err(err) => {
                warn!(
                    thread_id = %thread_id,
                    error = %err,
                    "failed to query operational evidence for model context"
                );
                return None;
            }
        };

        let records = records
            .into_iter()
            .filter(|record| matches_context_record_status(record.status))
            .filter(|record| !matches!(record.redaction_status, RedactionStatus::Rejected))
            .collect::<Vec<_>>();

        if records.is_empty() {
            return None;
        }

        Some(Self {
            thread_id: thread_id.to_string(),
            records,
        })
    }

    fn render_record(record: &OperationalEvidenceSummary) -> String {
        let mut rendered = String::from("  <record");
        push_attr(
            &mut rendered,
            "evidence_domain",
            record.evidence_domain.as_ref(),
        );
        push_attr(&mut rendered, "status", record.status.as_ref());
        if let Some(risk) = record.risk {
            push_attr(&mut rendered, "risk", risk.as_ref());
        }
        push_attr(&mut rendered, "source_tool", record.source_tool.as_str());
        push_optional_attr(&mut rendered, "source_ref", record.source_ref.as_deref());
        push_optional_attr(&mut rendered, "task_key", record.task_key.as_deref());
        push_optional_attr(&mut rendered, "gate_name", record.gate_name.as_deref());
        push_optional_attr(&mut rendered, "symbol_name", record.symbol_name.as_deref());
        push_optional_attr(&mut rendered, "file_path", record.file_path.as_deref());
        push_optional_attr(&mut rendered, "target_head", record.target_head.as_deref());
        push_optional_attr(
            &mut rendered,
            "graph_index_id",
            record.graph_index_id.as_deref(),
        );
        push_attr(
            &mut rendered,
            "provenance_hash",
            record.provenance_hash.as_str(),
        );
        rendered.push_str(">\n");
        rendered.push_str("    <summary>");
        push_xml_escaped_text(
            &mut rendered,
            &truncate_summary(record.summary.as_str(), MAX_SUMMARY_CHARS),
        );
        rendered.push_str("</summary>\n");
        rendered.push_str("  </record>");
        rendered
    }
}

impl ContextualUserFragment for OperationalEvidenceContextFragment {
    fn role(&self) -> &'static str {
        "user"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        (CONTEXT_START_MARKER, CONTEXT_END_MARKER)
    }

    fn body(&self) -> String {
        let mut rendered = String::new();
        rendered.push('\n');
        rendered.push_str("  <thread_id>");
        push_xml_escaped_text(&mut rendered, self.thread_id.as_str());
        rendered.push_str("</thread_id>\n");

        for record in &self.records {
            let record_text = Self::render_record(record);
            if rendered
                .len()
                .saturating_add(record_text.len())
                .saturating_add(1)
                > MAX_RENDERED_BYTES
            {
                let truncated = "  <truncated reason=\"byte_cap\" />\n";
                if rendered
                    .len()
                    .saturating_add(truncated.len())
                    .saturating_add(1)
                    <= MAX_RENDERED_BYTES
                {
                    rendered.push_str(truncated);
                }
                break;
            }

            rendered.push_str(&record_text);
            rendered.push('\n');
        }
        rendered
    }
}

fn matches_context_record_status(status: EvidenceStatus) -> bool {
    matches!(
        status,
        EvidenceStatus::Implemented | EvidenceStatus::Verified | EvidenceStatus::Done
    )
}

fn truncate_summary(summary: &str, max_chars: usize) -> String {
    let mut chars = summary.chars();
    let mut truncated = chars.by_ref().take(max_chars).collect::<String>();
    if chars.next().is_some() {
        truncated.push_str("...");
    }
    truncated
}

fn push_optional_attr(rendered: &mut String, name: &str, value: Option<&str>) {
    let Some(value) = value else {
        return;
    };
    push_attr(rendered, name, value);
}

fn push_attr(rendered: &mut String, name: &str, value: &str) {
    rendered.push(' ');
    rendered.push_str(name);
    rendered.push_str("=\"");
    push_xml_escaped_text(rendered, value);
    rendered.push('"');
}

fn push_xml_escaped_text(rendered: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '&' => rendered.push_str("&amp;"),
            '<' => rendered.push_str("&lt;"),
            '>' => rendered.push_str("&gt;"),
            '"' => rendered.push_str("&quot;"),
            '\'' => rendered.push_str("&apos;"),
            _ => rendered.push(ch),
        }
    }
}

#[cfg(test)]
#[path = "operational_evidence_context_tests.rs"]
mod tests;
