#!/usr/bin/env python3
"""Redact credential-like Claude OAuth JSON before sharing schema samples."""

import argparse
import json
import re
import sys
from collections.abc import Mapping
from pathlib import Path
from typing import Any


SECRET_KEY_RE = re.compile(
    r"(access|refresh|id|bearer|session|credential|secret|token|cookie)",
    re.IGNORECASE,
)
ACCOUNT_KEY_RE = re.compile(
    r"(account|user|workspace|organization|org|tenant)", re.IGNORECASE
)
EMAIL_RE = re.compile(r"^[^@\s]+@[^@\s]+\.[^@\s]+$")
JWT_RE = re.compile(r"^[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+$")
LONG_SECRET_RE = re.compile(r"^[A-Za-z0-9_./+=:-]{32,}$")

SECRET_PLACEHOLDERS = {
    "access": "REDACTED_ACCESS_TOKEN",
    "refresh": "REDACTED_REFRESH_TOKEN",
    "id": "REDACTED_ID_TOKEN",
    "bearer": "REDACTED_BEARER_TOKEN",
    "session": "REDACTED_SESSION_TOKEN",
    "cookie": "REDACTED_COOKIE",
}

STRUCTURAL_ID_KEYS = {"client_id"}


def placeholder_for_key(key: str) -> str:
    lowered = key.lower()
    for prefix, placeholder in SECRET_PLACEHOLDERS.items():
        if prefix in lowered:
            return placeholder
    return "REDACTED_SECRET"


def redact_value(value: Any, path: tuple[str, ...] = ()) -> Any:
    if isinstance(value, Mapping):
        return {
            key: redact_field(str(key), child, (*path, str(key)))
            for key, child in value.items()
        }
    if isinstance(value, list):
        return [redact_value(child, (*path, "[]")) for child in value]
    if isinstance(value, str):
        return redact_string(value, path[-1] if path else "")
    return value


def redact_field(key: str, value: Any, path: tuple[str, ...]) -> Any:
    if isinstance(value, str):
        if key.lower() in STRUCTURAL_ID_KEYS:
            return value
        if ACCOUNT_KEY_RE.search(key) and looks_identifier_like(value):
            return f"{key.lower()}_redacted_1"
        if SECRET_KEY_RE.search(key):
            return placeholder_for_key(key)
    return redact_value(value, path)


def redact_string(value: str, key: str) -> str:
    if EMAIL_RE.match(value):
        return "user@example.invalid"
    if JWT_RE.match(value) or (SECRET_KEY_RE.search(key) and value):
        return placeholder_for_key(key)
    if LONG_SECRET_RE.match(value) and not looks_url_like(value):
        return "REDACTED_SECRET"
    return value


def looks_identifier_like(value: str) -> bool:
    return bool(value) and not looks_url_like(value) and not EMAIL_RE.match(value)


def looks_url_like(value: str) -> bool:
    return value.startswith(("http://", "https://"))


def summarize_shape(value: Any, path: str = "$") -> list[str]:
    if isinstance(value, Mapping):
        lines = [f"{path}: object keys={list(value.keys())}"]
        for key, child in value.items():
            lines.extend(summarize_shape(child, f"{path}.{key}"))
        return lines
    if isinstance(value, list):
        lines = [f"{path}: array len={len(value)}"]
        if value:
            lines.extend(summarize_shape(value[0], f"{path}[0]"))
        return lines
    return [f"{path}: {type(value).__name__}"]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Redact a Claude credential-like JSON sample for OAuth import validation."
    )
    parser.add_argument("input", type=Path, help="Path to the raw JSON file to redact.")
    parser.add_argument(
        "--output",
        type=Path,
        help="Where to write redacted JSON. Defaults to stdout after the summary.",
    )
    parser.add_argument(
        "--summary-only",
        action="store_true",
        help="Print only the structural summary and do not emit redacted JSON.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        raw = json.loads(args.input.read_text())
    except OSError as error:
        print(f"failed to read input: {error}", file=sys.stderr)
        return 1
    except json.JSONDecodeError as error:
        print(f"input is not valid JSON: {error}", file=sys.stderr)
        return 1

    redacted = redact_value(raw)
    summary = "\n".join(summarize_shape(redacted))
    print(summary, file=sys.stderr)

    if args.summary_only:
        return 0

    payload = json.dumps(redacted, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.write_text(payload)
    else:
        print(payload, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
