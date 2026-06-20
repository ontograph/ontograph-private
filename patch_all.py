import re

# Patch model/agent_job.rs
with open('ontocode-rs/state/src/model/agent_job.rs', 'r') as f:
    content = f.read()

content = content.replace('    pub last_error: Option<String>,', '    pub last_error: Option<String>,\n    pub final_summary: Option<String>,', 1)
content = content.replace('    pub(crate) last_error: Option<String>,', '    pub(crate) last_error: Option<String>,\n    pub(crate) final_summary: Option<String>,', 1)
content = content.replace('            last_error: value.last_error,', '            last_error: value.last_error,\n            final_summary: value.final_summary,', 1)

with open('ontocode-rs/state/src/model/agent_job.rs', 'w') as f:
    f.write(content)

# Patch runtime/agent_jobs.rs
with open('ontocode-rs/state/src/runtime/agent_jobs.rs', 'r') as f:
    content = f.read()

content = content.replace('''    completed_at,
    last_error
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, NULL)''', '''    completed_at,
    last_error,
    final_summary
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, NULL, NULL)''', 1)

content = content.replace('''    completed_at,
    last_error
FROM agent_jobs''', '''    completed_at,
    last_error,
    final_summary
FROM agent_jobs''', 1)

new_mark_completed = r'''    pub async fn mark_agent_job_completed(&self, job_id: &str, final_summary: Option<&str>) -> anyhow::Result<()> {
        let now = Utc::now().timestamp();
        sqlx::query(
            r#"
UPDATE agent_jobs
SET status = ?, updated_at = ?, completed_at = ?, last_error = NULL, final_summary = COALESCE(?, final_summary)
WHERE id = ?
 AND status NOT IN (?, ?, ?)
            "#,
        )
        .bind(AgentJobStatus::Completed.as_str())
        .bind(now)
        .bind(now)
        .bind(final_summary)
        .bind(job_id)
        .bind(AgentJobStatus::Completed.as_str())
        .bind(AgentJobStatus::Failed.as_str())
        .bind(AgentJobStatus::Cancelled.as_str())
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }'''

content = re.sub(r'    pub async fn mark_agent_job_completed.*?Ok\(\(\)\)\n    \}', new_mark_completed, content, flags=re.DOTALL)

with open('ontocode-rs/state/src/runtime/agent_jobs.rs', 'w') as f:
    f.write(content)
