#!/usr/bin/env python3

from pathlib import Path
import json
import stat
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from codex_package.layout import build_package_dir
from codex_package.layout import validate_package_dir
from codex_package.targets import PACKAGE_VARIANTS
from codex_package.targets import PackageInputs
from codex_package.targets import TARGET_SPECS


class PackageLayoutTest(unittest.TestCase):
    def test_primary_package_can_include_lean_ctx_backend_resource(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            package_dir = root / "package"
            package_dir.mkdir()

            inputs = PackageInputs(
                entrypoint_bin=executable(root / "codex"),
                rg_bin=executable(root / "rg"),
                zsh_bin=None,
                bwrap_bin=None,
                codex_command_runner_bin=None,
                codex_windows_sandbox_setup_bin=None,
                lean_ctx_bin=executable(root / "lean-ctx"),
            )
            executable(root / "ontocode")

            build_package_dir(
                package_dir,
                "1.2.3",
                PACKAGE_VARIANTS["codex"],
                TARGET_SPECS["x86_64-apple-darwin"],
                inputs,
            )
            validate_package_dir(
                package_dir,
                PACKAGE_VARIANTS["codex"],
                TARGET_SPECS["x86_64-apple-darwin"],
                include_zsh=False,
            )

            metadata = json.loads((package_dir / "codex-package.json").read_text())
            self.assertEqual(
                metadata["leanCtxBackend"],
                "codex-resources/lean-ctx",
            )
            self.assertTrue((package_dir / "codex-resources" / "lean-ctx").is_file())


def executable(path: Path) -> Path:
    path.write_text("#!/bin/sh\n", encoding="utf-8")
    path.chmod(path.stat().st_mode | stat.S_IXUSR)
    return path.resolve()


if __name__ == "__main__":
    unittest.main()
