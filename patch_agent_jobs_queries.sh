#!/bin/bash
sed -i 's/    last_error/    last_error,\n    final_summary/' ontocode-rs/state/src/runtime/agent_jobs.rs
sed -i 's/NULL, NULL, NULL)/NULL, NULL, NULL, NULL)/' ontocode-rs/state/src/runtime/agent_jobs.rs
