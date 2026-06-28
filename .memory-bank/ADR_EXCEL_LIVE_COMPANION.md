# ADR: Excel Live Companion Crate Boundary

## Status
Accepted.

## Context
Phases 5 and 6 require live named-range updates and `Formula2` write operations. The Excel COM API and VertiPaq Data Model (ADO MSOLAP provider) only run on Windows with Excel locally installed. The core `ontocode-rs` workspace runs on Linux/macOS as well. 

We must not leak Windows-only `win32com` / C# Interop dependencies into the offline read-only `ext/excel` crate.

## Decision
We will establish a separate crate: `ontocode-rs/ext/excel-live`.

1. **Strict Division**:
   - `ext/excel` remains completely offline, COM-free, and cross-platform. It parses files, extracts metadata, builds dependency graphs, and generates SQL plans.
   - `ext/excel-live` is Windows-only (`#[cfg(windows)]`), handles live COM interaction, reads the ADO connection, and executes DAX queries.
2. **Sidecar Option**:
   - If a C# compilation dependency on non-Windows hosts is too disruptive for the Rust project build pipeline, `ext/excel-live` can spawn a packaged Python sidecar (reusing the local `mcp-server-excel` scripts) via a standard `Command` interface.
3. **Execution Schema**:
   - The offline planner writes out the plan artifact (JSON).
   - `ext/excel-live` reads the JSON plan and invokes the COM/ADO layers synchronously to execute the changes.

## Consequences
- The cross-platform build matrix is preserved.
- Live mutation is isolated, preventing silent calc locks in offline tools.
