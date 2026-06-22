#!/usr/bin/env bash
set -euo pipefail
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
( CARGO_BUILD_JOBS=8 just test -p ontocode-core spawn_agent_tool_v2_requires_task_name_and_lists_visible_models > /tmp/ontocode_spec_v2.log 2>&1; echo $? > /tmp/ontocode_spec_v2.exit ) &
echo $! > /tmp/ontocode_spec_v2.pid
