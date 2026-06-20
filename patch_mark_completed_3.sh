#!/bin/bash
git checkout ontocode-rs/state/src/runtime/agent_jobs.rs
bash patch_agent_jobs_queries.sh

# More precise patch
python3 -c "
with open('ontocode-rs/state/src/runtime/agent_jobs.rs', 'r') as f:
    content = f.read()

import re

new_func = r'''    pub async fn mark_agent_job_completed(&self, job_id: &str, final_summary: Option<&str>) -> anyhow::Result<()> {
        let now = Utc::now().timestamp();
        sqlx::query(
            r#\"
UPDATE agent_jobs
SET status = ?, updated_at = ?, completed_at = ?, last_error = NULL, final_summary = COALESCE(?, final_summary)
WHERE id = ?
 AND status NOT IN (?, ?, ?)
            \"#,
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

content = re.sub(r'    pub async fn mark_agent_job_completed.*?Ok\(\(\)\)\n    \}', new_func, content, flags=re.DOTALL)

with open('ontocode-rs/state/src/runtime/agent_jobs.rs', 'w') as f:
    f.write(content)
"
