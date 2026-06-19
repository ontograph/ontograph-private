#!/usr/bin/env python3
"""Repository-only, read-only, standard-library-only memory-bank helpers."""

import argparse
import re
from dataclasses import dataclass
from pathlib import Path


MEMORY_BANK_DIRNAME = ".memory-bank"
LINK_PATTERN = re.compile(r"(?<!\!)\[[^\]]+\]\(([^)]+)\)")
MAX_BROKEN_LINKS = 25
TRACKING_MARKERS = {
    "done": ("[x]", "done", "completed", "accepted"),
    "pending": ("[ ]", "pending", "todo"),
    "blocked": ("blocked",),
    "in progress": ("in progress", "in-progress"),
}


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def truncate(text: str, max_len: int = 160) -> str:
    cleaned = " ".join(text.split())
    if len(cleaned) <= max_len:
        return cleaned
    if max_len <= 3:
        return cleaned[:max_len]
    return f"{cleaned[: max_len - 3]}..."


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def memory_bank_path(root: Path, name: str) -> Path:
    return root / MEMORY_BANK_DIRNAME / name


def status_digest(root: Path) -> int:
    memory_path = memory_bank_path(root, "MEMORY.md")
    plan_path = memory_bank_path(root, "project_plan-current.md")
    pending_path = memory_bank_path(root, "project_pending-tasks.md")

    required = [memory_path, plan_path, pending_path]
    missing = [path for path in required if not path.exists()]
    if missing:
        print("ERROR: missing required memory-bank file(s):")
        for path in missing:
            print(f"- {path.relative_to(root)}")
        return 1

    memory_lines = read_text(memory_path).splitlines()
    title_count = sum(1 for line in memory_lines if line.startswith("- ["))

    plan_lines = read_text(plan_path).splitlines()
    status_line = next(
        (
            line.strip()
            for line in plan_lines
            if line.strip().lower().startswith("status:")
        ),
        "status: unknown",
    )

    pending_lines = [
        line.strip()
        for line in read_text(pending_path).splitlines()
        if line.lstrip().startswith(("- ", "- ["))
    ]
    print("status-digest")
    print(f"- memory titles: {title_count}")
    print(f"- plan {status_line}")
    print(f"- pending items: {len(pending_lines)}")
    for index, line in enumerate(pending_lines[:3], start=1):
        print(f"- pending {index}: {truncate(line)}")
    return 0


def count_markers(lines: list[str]) -> dict[str, int]:
    counts = {key: 0 for key in TRACKING_MARKERS}
    for line in lines:
        stripped = line.lstrip()
        if not stripped.startswith(("|", "-", "*")):
            continue
        lowered = stripped.lower()
        for label, markers in TRACKING_MARKERS.items():
            if any(marker in lowered for marker in markers):
                counts[label] += 1
    return counts


def count_left(root: Path) -> int:
    paths = [
        memory_bank_path(root, "project_pending-tasks.md"),
        memory_bank_path(root, "CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md"),
    ]

    print("count-left (heuristic)")
    print("file | done | pending | blocked | in progress")
    print("--- | ---: | ---: | ---: | ---:")
    for path in paths:
        if not path.exists():
            print(f"{path.relative_to(root)} | missing | missing | missing | missing")
            continue
        counts = count_markers(read_text(path).splitlines())
        print(
            f"{path.relative_to(root)} | {counts['done']} | {counts['pending']} | "
            f"{counts['blocked']} | {counts['in progress']}"
        )
    return 0


def normalize_doc_link_target(raw_target: str) -> str:
    target = raw_target.strip().split("#", 1)[0]
    if target.startswith(("http://", "https://", "mailto:")):
        return target
    if target.startswith("/"):
        head, sep, tail = target.rpartition(":")
        if sep and tail.isdigit() and Path(head).suffix:
            return head
    return target


def resolve_link_source(base_dir: Path, target: str) -> Path:
    if target.startswith("/"):
        return Path(target)
    return base_dir / target


@dataclass(frozen=True)
class BrokenLink:
    file_path: Path
    target: str


def collect_broken_links(root: Path) -> list[BrokenLink]:
    broken: list[BrokenLink] = []
    memory_bank_dir = root / MEMORY_BANK_DIRNAME
    for md_path in sorted(memory_bank_dir.glob("*.md")):
        if not md_path.is_file():
            continue
        text = read_text(md_path)
        for match in LINK_PATTERN.finditer(text):
            raw_target = match.group(1).strip()
            if not raw_target or raw_target.startswith(
                ("#", "http://", "https://", "mailto:")
            ):
                continue
            normalized = normalize_doc_link_target(raw_target)
            resolved = resolve_link_source(md_path.parent, normalized)
            if not resolved.exists():
                broken.append(BrokenLink(md_path.relative_to(root), raw_target))
    return broken


def doc_link_check(root: Path) -> int:
    broken = collect_broken_links(root)
    if broken:
        print("broken links")
        for item in broken[:MAX_BROKEN_LINKS]:
            print(f"- {item.file_path}: {item.target}")
        remaining = len(broken) - MAX_BROKEN_LINKS
        if remaining > 0:
            print(f"- ... and {remaining} more")
        return 1
    print("broken links: none")
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="onto_memory_tools",
        description="Repository-only read-only memory-bank helper commands.",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser(
        "status-digest", help="print a bounded memory-bank status digest"
    )
    subparsers.add_parser("count-left", help="print heuristic task-status counts")
    subparsers.add_parser(
        "doc-link-check", help="check local markdown links in .memory-bank"
    )
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    root = repo_root()

    match args.command:
        case "status-digest":
            return status_digest(root)
        case "count-left":
            return count_left(root)
        case "doc-link-check":
            return doc_link_check(root)
        case _:
            parser.error(f"unknown command: {args.command}")
            return 2


if __name__ == "__main__":
    raise SystemExit(main())
