import re

with open('ontocode-rs/core/src/tools/handlers/agent_jobs.rs', 'r') as f:
    content = f.read()

content = content.replace('    db.mark_agent_job_completed(job_id.as_str()).await?;', '    let final_summary = job.final_summary.as_deref();\n    db.mark_agent_job_completed(job_id.as_str(), final_summary).await?;')

with open('ontocode-rs/core/src/tools/handlers/agent_jobs.rs', 'w') as f:
    f.write(content)

with open('ontocode-rs/core/src/tools/handlers/agent_jobs/spawn_agents_on_csv.rs', 'r') as f:
    content = f.read()

content = content.replace('    job_error: Option<String>,', '    job_error: Option<String>,\n    final_summary: Option<String>,')

content = content.replace('        failed_items: progress.failed_items,', '        failed_items: progress.failed_items,\n        final_summary: job.final_summary.clone(),')

with open('ontocode-rs/core/src/tools/handlers/agent_jobs/spawn_agents_on_csv.rs', 'w') as f:
    f.write(content)
