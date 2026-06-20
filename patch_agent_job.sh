#!/bin/bash
sed -i 's/pub last_error: Option<String>,/pub last_error: Option<String>,\n    pub final_summary: Option<String>,/' ontocode-rs/state/src/model/agent_job.rs
sed -i 's/pub(crate) last_error: Option<String>,/pub(crate) last_error: Option<String>,\n    pub(crate) final_summary: Option<String>,/' ontocode-rs/state/src/model/agent_job.rs
sed -i 's/last_error: value.last_error,/last_error: value.last_error,\n            final_summary: value.final_summary,/' ontocode-rs/state/src/model/agent_job.rs
