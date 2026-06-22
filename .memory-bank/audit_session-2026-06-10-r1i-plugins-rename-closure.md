# R1I Plugins Internal Crate Rename Closure

Date: 2026-06-10

Scope:
- Renamed `codex-utils-plugins` to `ontocode-utils-plugins`.
- Updated Cargo package identity, Rust lib crate identity, Bazel crate name, workspace dependency names, dependent manifests, and Rust import paths.
- Preserved plugin discovery/install behavior, plugin mention syntax, MCP connector filtering, plugin namespace mapping, and skill root semantics.

Risk:
- OntoIndex impact for `find_plugin_manifest_path`: CRITICAL, affecting plugin discovery/install, app-server plugin processors, CLI plugin commands, and core plugin loaders.
- OntoIndex impact for `plugin_namespace_for_skill_path`: LOW.
- OntoIndex impact for `is_connector_id_allowed`: LOW after disambiguation.
- OntoIndex impact for `sanitize_name`: LOW after disambiguation.
- OntoIndex impact for `PluginSkillRoot`: LOW.

Verification:
- `CARGO_BUILD_JOBS=8 cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 cargo generate-lockfile`
- `CARGO_BUILD_JOBS=8 cargo update -p allocative --precise 0.3.4`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-plugins`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-plugins`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-skills`
- `CARGO_BUILD_JOBS=8 just test -p codex-plugin`
- `CARGO_BUILD_JOBS=8 just test -p codex-mcp`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli plugin_marketplace`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server plugins`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH just bazel-lock-check`
- Stale-reference search found zero `codex-utils-plugins` or `codex_utils_plugins` matches under `ontocode-rs`.
- `Cargo.lock` contains `ontocode-utils-plugins`; old plugin crate name is absent.
- `allocative` remains pinned at `0.3.4`; `allocative_derive` remains `0.3.3`.
- `git diff --check`
- Scoped OntoIndex verification passed.

Notes:
- `just test -p codex-cli plugin_cli` compiled the CLI but matched zero tests; the accepted focused CLI coverage is `plugin_marketplace`, which ran six plugin marketplace tests.
- Two `codex-core-skills` tests initially failed from default tempdir ancestry contamination, not the rename. The tests now create tempdirs beside the repository root to avoid inherited parent `.git` or `.codex` directories.
- Next rename work is blocked until the remaining utility candidates are freshly inventoried, risk-reviewed, and explicitly approved as exact identity-only slices.
