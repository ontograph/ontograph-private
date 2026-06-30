#!/usr/bin/env python3

from pathlib import Path
import contextlib
import io
import json
import stat
import sys
import tempfile
import unittest
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from codex_package.cli import main


class PackageCliTest(unittest.TestCase):
    def test_windows_package_uses_prebuilt_core_binaries(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            package_dir = root / "package"
            executable(root / "codex.exe")
            executable(root / "ontocode.exe")
            executable(root / "rg.exe")
            executable(root / "ontocode-command-runner.exe")
            executable(root / "ontocode-windows-sandbox-setup.exe")

            argv = [
                "build_codex_package.py",
                "--target",
                "x86_64-pc-windows-msvc",
                "--variant",
                "codex",
                "--entrypoint-bin",
                str(root / "codex.exe"),
                "--rg-bin",
                str(root / "rg.exe"),
                "--ontocode-command-runner-bin",
                str(root / "ontocode-command-runner.exe"),
                "--ontocode-windows-sandbox-setup-bin",
                str(root / "ontocode-windows-sandbox-setup.exe"),
                "--package-dir",
                str(package_dir),
                "--force",
            ]

            with mock.patch.object(sys, "argv", argv):
                with contextlib.redirect_stdout(io.StringIO()):
                    self.assertEqual(main(), 0)

            metadata = json.loads((package_dir / "codex-package.json").read_text())
            self.assertEqual(metadata["entrypoint"], "bin/codex.exe")
            self.assertNotIn("leanCtxBackend", metadata)


def executable(path: Path) -> Path:
    path.write_text("fake\n", encoding="utf-8")
    path.chmod(path.stat().st_mode | stat.S_IXUSR)
    return path.resolve()


if __name__ == "__main__":
    unittest.main()
