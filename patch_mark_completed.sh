#!/bin/bash
sed -i 's/pub async fn mark_agent_job_completed(&self, job_id: &str) -> anyhow::Result<()> {/pub async fn mark_agent_job_completed(\&self, job_id: \&str, final_summary: Option<\&str>) -> anyhow::Result<()> {/' ontocode-rs/state/src/runtime/agent_jobs.rs
sed -i 's/SET status = ?, updated_at = ?, completed_at = ?, last_error = NULL/SET status = ?, updated_at = ?, completed_at = ?, last_error = NULL, final_summary = ?/' ontocode-rs/state/src/runtime/agent_jobs.rs
sed -i '/\.bind(AgentJobStatus::Completed\.as_str())/a \        .bind(final_summary)' ontocode-rs/state/src/runtime/agent_jobs.rs
