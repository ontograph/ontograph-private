CREATE TABLE operational_evidence_records (
    id TEXT PRIMARY KEY,
    evidence_domain TEXT NOT NULL,
    source_tool TEXT NOT NULL,
    source_version TEXT,
    schema_version INTEGER NOT NULL,
    source_ref TEXT,
    repo TEXT,
    task_key TEXT,
    thread_id TEXT,
    parent_thread_id TEXT,
    child_thread_id TEXT,
    symbol_uid TEXT,
    symbol_name TEXT,
    file_path TEXT,
    process_label TEXT,
    gate_name TEXT,
    risk TEXT,
    status TEXT NOT NULL,
    summary TEXT NOT NULL CHECK (length(summary) <= 8192),
    source_links_json TEXT NOT NULL DEFAULT '[]' CHECK (length(source_links_json) <= 16384),
    metadata_json TEXT NOT NULL DEFAULT '{}' CHECK (length(metadata_json) <= 16384),
    provenance_hash TEXT NOT NULL UNIQUE,
    redaction_status TEXT NOT NULL,
    target_head TEXT,
    graph_index_id TEXT,
    plan_hash TEXT,
    tracking_hash TEXT,
    created_at INTEGER NOT NULL,
    expires_at INTEGER
);

CREATE INDEX idx_operational_evidence_task_status
    ON operational_evidence_records (task_key, status, created_at DESC);

CREATE INDEX idx_operational_evidence_thread
    ON operational_evidence_records (thread_id, created_at DESC);

CREATE INDEX idx_operational_evidence_symbol
    ON operational_evidence_records (symbol_uid, created_at DESC);

CREATE INDEX idx_operational_evidence_file
    ON operational_evidence_records (file_path, created_at DESC);

CREATE INDEX idx_operational_evidence_domain_risk
    ON operational_evidence_records (evidence_domain, risk, created_at DESC);

CREATE INDEX idx_operational_evidence_target_head
    ON operational_evidence_records (target_head, created_at DESC);

CREATE INDEX idx_operational_evidence_expires_at
    ON operational_evidence_records (expires_at);
