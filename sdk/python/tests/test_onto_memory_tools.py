import importlib.util
import io
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SCRIPT_PATH = ROOT / "scripts" / "onto_memory_tools.py"


def load_script_module():
    spec = importlib.util.spec_from_file_location("onto_memory_tools", SCRIPT_PATH)
    if spec is None or spec.loader is None:
        raise AssertionError(f"Failed to load script module: {SCRIPT_PATH}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


class OntoMemoryToolsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.script = load_script_module()

    def test_parser_help_lists_only_the_three_planned_subcommands(self):
        parser = self.script.build_parser()
        subparsers_action = next(
            action for action in parser._actions if getattr(action, "choices", None) is not None
        )

        self.assertEqual(
            list(subparsers_action.choices),
            ["status-digest", "count-left", "doc-link-check"],
        )
        help_text = parser.format_help()
        for command in ("status-digest", "count-left", "doc-link-check"):
            self.assertIn(command, help_text)

    def test_doc_link_check_passes_for_valid_local_memory_bank_links(self):
        with tempfile.TemporaryDirectory() as tmp_dir:
            root = Path(tmp_dir)
            memory_bank = root / ".memory-bank"
            memory_bank.mkdir()

            local_target = memory_bank / "local-target.md"
            absolute_target = memory_bank / "absolute-target.md"
            local_target.write_text("local target\n", encoding="utf-8")
            absolute_target.write_text("absolute target\n", encoding="utf-8")

            (memory_bank / "index.md").write_text(
                "\n".join(
                    [
                        "[relative](./local-target.md)",
                        f"[absolute]({absolute_target}:12#section)",
                        "[anchor](#section-only)",
                        "[external](https://example.com/docs)",
                        "[mail](mailto:person@example.com)",
                    ]
                )
                + "\n",
                encoding="utf-8",
            )

            with redirect_stdout(io.StringIO()):
                self.assertEqual(self.script.doc_link_check(root), 0)

    def test_doc_link_check_fails_for_broken_local_markdown_link(self):
        with tempfile.TemporaryDirectory() as tmp_dir:
            root = Path(tmp_dir)
            memory_bank = root / ".memory-bank"
            memory_bank.mkdir()

            (memory_bank / "index.md").write_text(
                "[broken](./missing-target.md)\n",
                encoding="utf-8",
            )

            with redirect_stdout(io.StringIO()):
                self.assertEqual(self.script.doc_link_check(root), 1)

    def test_count_markers_handles_done_pending_blocked_and_in_progress(self):
        counts = self.script.count_markers(
            [
                "| done |",
                "- [x] accepted",
                "* pending",
                "- [ ] todo",
                "- blocked",
                "| in progress |",
            ]
        )

        self.assertEqual(
            counts,
            {
                "done": 2,
                "pending": 2,
                "blocked": 1,
                "in progress": 1,
            },
        )


if __name__ == "__main__":
    unittest.main()
