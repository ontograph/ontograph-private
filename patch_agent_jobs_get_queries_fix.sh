#!/bin/bash
sed -i 's/    final_summary,//' ontocode-rs/state/src/runtime/agent_jobs.rs
sed -i 's/    last_error,/    last_error,\n    final_summary,/' ontocode-rs/state/src/runtime/agent_jobs.rs
