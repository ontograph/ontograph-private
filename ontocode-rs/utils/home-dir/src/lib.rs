use dirs::home_dir;
use ontocode_utils_absolute_path::AbsolutePathBuf;
use std::path::PathBuf;

/// Returns the path to the Codex configuration directory, which can be
/// specified by the `ONTOCODE_HOME` or `CODEX_HOME` environment variables. If
/// neither is set, this defaults to `~/.ontocode`.
///
/// - If `ONTOCODE_HOME` or `CODEX_HOME` is set, the selected value must exist
///   and be a directory. The value will be canonicalized and this function
///   will Err otherwise.
/// - If neither environment variable is set, this function does not verify
///   that the selected default directory exists.
pub fn find_codex_home() -> std::io::Result<AbsolutePathBuf> {
    let ontocode_home_env = std::env::var("ONTOCODE_HOME")
        .ok()
        .filter(|val| !val.is_empty());
    let codex_home_env = std::env::var("CODEX_HOME")
        .ok()
        .filter(|val| !val.is_empty());
    find_codex_home_with_overrides(
        ontocode_home_env.as_deref(),
        codex_home_env.as_deref(),
        home_dir(),
    )
}

fn find_codex_home_with_overrides(
    ontocode_home_env: Option<&str>,
    codex_home_env: Option<&str>,
    home_dir_override: Option<PathBuf>,
) -> std::io::Result<AbsolutePathBuf> {
    match resolve_home_override(ontocode_home_env, codex_home_env) {
        Some((source, val)) => {
            let path = PathBuf::from(val);
            let metadata = std::fs::metadata(&path).map_err(|err| match err.kind() {
                std::io::ErrorKind::NotFound => std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("{source} points to {val:?}, but that path does not exist"),
                ),
                _ => std::io::Error::new(
                    err.kind(),
                    format!("failed to read {source} {val:?}: {err}"),
                ),
            })?;

            if !metadata.is_dir() {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("{source} points to {val:?}, but that path is not a directory"),
                ))
            } else {
                let canonical = path.canonicalize().map_err(|err| {
                    std::io::Error::new(
                        err.kind(),
                        format!("failed to canonicalize {source} {val:?}: {err}"),
                    )
                })?;
                AbsolutePathBuf::from_absolute_path(canonical)
            }
        }
        None => {
            let mut p = home_dir_override.ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not find home directory",
                )
            })?;
            p.push(".ontocode");
            AbsolutePathBuf::from_absolute_path(p)
        }
    }
}

fn resolve_home_override<'a>(
    ontocode_home_env: Option<&'a str>,
    codex_home_env: Option<&'a str>,
) -> Option<(&'static str, &'a str)> {
    ontocode_home_env
        .map(|val| ("ONTOCODE_HOME", val))
        .or_else(|| codex_home_env.map(|val| ("CODEX_HOME", val)))
}

#[cfg(test)]
mod tests {
    use super::find_codex_home_with_overrides;
    use ontocode_utils_absolute_path::AbsolutePathBuf;
    use pretty_assertions::assert_eq;
    use std::fs;
    use std::io::ErrorKind;
    use std::path::Path;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn find_codex_home_for_test(
        ontocode_home_env: Option<&str>,
        codex_home_env: Option<&str>,
        home_dir: &Path,
    ) -> std::io::Result<AbsolutePathBuf> {
        find_codex_home_with_overrides(
            ontocode_home_env,
            codex_home_env,
            Some(home_dir.to_path_buf()),
        )
    }

    #[test]
    fn find_codex_home_ontocode_home_missing_path_is_fatal() {
        let temp_home = TempDir::new().expect("temp home");
        let missing = temp_home.path().join("missing-ontocode-home");
        let missing_str = missing
            .to_str()
            .expect("missing ontocode home path should be valid utf-8");

        let err = find_codex_home_for_test(Some(missing_str), None, temp_home.path())
            .expect_err("missing ONTOCODE_HOME");
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert!(
            err.to_string().contains("ONTOCODE_HOME"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn find_codex_home_codex_home_file_path_is_fatal() {
        let temp_home = TempDir::new().expect("temp home");
        let file_path = temp_home.path().join("codex-home.txt");
        fs::write(&file_path, "not a directory").expect("write temp file");
        let file_str = file_path
            .to_str()
            .expect("file codex home path should be valid utf-8");

        let err = find_codex_home_for_test(None, Some(file_str), temp_home.path())
            .expect_err("file CODEX_HOME");
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert!(
            err.to_string().contains("not a directory"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn find_codex_home_ontocode_home_wins_over_codex_home() {
        let temp_home = TempDir::new().expect("temp home");
        let ontocode_home = temp_home.path().join("ontocode-home");
        fs::create_dir_all(&ontocode_home).expect("create ontocode home");
        let codex_home = temp_home.path().join("codex-home");
        fs::create_dir_all(&codex_home).expect("create codex home");

        let resolved = find_codex_home_for_test(
            Some(
                ontocode_home
                    .to_str()
                    .expect("ontocode home path should be valid utf-8"),
            ),
            Some(
                codex_home
                    .to_str()
                    .expect("codex home path should be valid utf-8"),
            ),
            temp_home.path(),
        )
        .expect("valid ONTOCODE_HOME");
        let expected = ontocode_home
            .canonicalize()
            .expect("canonicalize ontocode home");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn find_codex_home_ontocode_home_invalid_is_fatal_even_when_codex_home_is_valid() {
        let temp_home = TempDir::new().expect("temp home");
        let missing_ontocode_home = temp_home.path().join("missing-ontocode-home");
        let codex_home = temp_home.path().join("codex-home");
        fs::create_dir_all(&codex_home).expect("create codex home");

        let err = find_codex_home_for_test(
            Some(
                missing_ontocode_home
                    .to_str()
                    .expect("missing ontocode home path should be valid utf-8"),
            ),
            Some(
                codex_home
                    .to_str()
                    .expect("codex home path should be valid utf-8"),
            ),
            temp_home.path(),
        )
        .expect_err("invalid ONTOCODE_HOME should win");
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert!(
            err.to_string().contains("ONTOCODE_HOME"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn find_codex_home_codex_home_valid_directory_canonicalizes() {
        let temp_home = TempDir::new().expect("temp home");
        let temp_str = temp_home
            .path()
            .to_str()
            .expect("temp codex home path should be valid utf-8");

        let resolved = find_codex_home_for_test(None, Some(temp_str), temp_home.path())
            .expect("valid CODEX_HOME");
        let expected = temp_home
            .path()
            .canonicalize()
            .expect("canonicalize temp home");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn find_codex_home_without_env_uses_ontocode_home_even_when_codex_home_exists() {
        let temp_home = TempDir::new().expect("temp home");
        let ontocode_home = temp_home.path().join(".ontocode");
        let codex_home = temp_home.path().join(".codex");
        fs::create_dir_all(&ontocode_home).expect("create ontocode home");
        fs::create_dir_all(&codex_home).expect("create codex home");

        let resolved = find_codex_home_for_test(None, None, temp_home.path())
            .expect("default should prefer existing ontocode home");
        let expected = AbsolutePathBuf::from_absolute_path(ontocode_home).expect("absolute home");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn find_codex_home_without_env_defaults_to_ontocode_home_path() {
        let temp_home = TempDir::new().expect("temp home");

        let resolved = find_codex_home_for_test(None, None, temp_home.path())
            .expect("default ONTOCODE_HOME path");
        let expected = PathBuf::from(temp_home.path()).join(".ontocode");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }
}
